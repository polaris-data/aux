use crate::{Authentication, Error, Kalshi, Operation};
use aux_rs_core::{HttpMethod, HttpRequest};
use base64::{Engine, engine::general_purpose::STANDARD};
use ed25519_dalek::Signer as _;
use rsa::{
    pkcs1::DecodeRsaPrivateKey,
    pkcs8::DecodePrivateKey,
    signature::{RandomizedSigner, SignatureEncoding},
};
use std::{
    future::Future,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use zeroize::Zeroizing;
/// Native algorithm-specific signing; external custody receives the final pre-sign bytes.
pub trait RequestSigner: Send + Sync + 'static {
    fn sign(&self, message: &[u8]) -> impl Future<Output = Result<Vec<u8>, Error>> + Send;
}
impl<S: RequestSigner> RequestSigner for Arc<S> {
    async fn sign(&self, m: &[u8]) -> Result<Vec<u8>, Error> {
        (**self).sign(m).await
    }
}
pub enum LocalSigner {
    Rsa(Box<rsa::RsaPrivateKey>),
    Ed25519(Box<ed25519_dalek::SigningKey>),
}
impl std::fmt::Debug for LocalSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("LocalSigner([redacted])")
    }
}
impl LocalSigner {
    pub fn from_pem(pem: Zeroizing<String>) -> Result<Self, Error> {
        if let Ok(k) = ed25519_dalek::SigningKey::from_pkcs8_pem(&pem) {
            return Ok(Self::Ed25519(Box::new(k)));
        }
        let k = rsa::RsaPrivateKey::from_pkcs8_pem(&pem)
            .or_else(|_| rsa::RsaPrivateKey::from_pkcs1_pem(&pem))
            .map_err(|_| Error::Signing("invalid RSA or Ed25519 PEM key"))?;
        if rsa::traits::PublicKeyParts::size(&k) < 256 {
            return Err(Error::Signing("RSA key must be at least 2048 bits"));
        }
        Ok(Self::Rsa(Box::new(k)))
    }
    pub fn ed25519(seed: Zeroizing<[u8; 32]>) -> Self {
        Self::Ed25519(Box::new(ed25519_dalek::SigningKey::from_bytes(&seed)))
    }
}
impl RequestSigner for LocalSigner {
    async fn sign(&self, m: &[u8]) -> Result<Vec<u8>, Error> {
        match self {
            Self::Ed25519(k) => Ok(k.sign(m).to_bytes().to_vec()),
            Self::Rsa(k) => {
                let signer =
                    rsa::pss::SigningKey::<sha2::Sha256>::new_with_salt_len((**k).clone(), 32);
                Ok(signer
                    .try_sign_with_rng(&mut rand::rngs::OsRng, m)
                    .map_err(|_| Error::Signing("RSA-PSS signing failed"))?
                    .to_vec())
            }
        }
    }
}
#[derive(Clone)]
pub struct KalshiAuth<S = Arc<LocalSigner>> {
    key_id: String,
    signer: S,
}
impl<S> std::fmt::Debug for KalshiAuth<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("KalshiAuth([redacted])")
    }
}
impl<S: RequestSigner> KalshiAuth<S> {
    pub fn new(key_id: impl Into<String>, signer: S) -> Result<Self, Error> {
        let key_id = key_id.into();
        if key_id.is_empty()
            || !key_id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"-_".contains(&b))
        {
            return Err(Error::Authentication("invalid API key ID"));
        }
        Ok(Self { key_id, signer })
    }
    /// Deterministic pre-sign preparation; RSA-PSS signatures remain randomized.
    pub async fn headers_at(
        &self,
        timestamp_ms: u64,
        method: &str,
        path: &str,
    ) -> Result<Vec<(String, String)>, Error> {
        if !matches!(method, "GET" | "POST" | "PUT" | "PATCH" | "DELETE")
            || !path.starts_with('/')
            || path.contains('#')
        {
            return Err(Error::InvalidRequest("invalid signing method/path"));
        }
        let path = path.split('?').next().unwrap_or(path);
        let ts = timestamp_ms.to_string();
        let message = format!("{ts}{method}{path}");
        Ok(vec![
            ("KALSHI-ACCESS-KEY".into(), self.key_id.clone()),
            ("KALSHI-ACCESS-TIMESTAMP".into(), ts),
            (
                "KALSHI-ACCESS-SIGNATURE".into(),
                STANDARD.encode(self.signer.sign(message.as_bytes()).await?),
            ),
        ])
    }
    pub async fn websocket_headers(&self, url: &str) -> Result<Vec<(String, String)>, Error> {
        let u = url::Url::parse(url).map_err(|_| Error::InvalidRequest("invalid WebSocket URL"))?;
        self.headers_at(unix_millis()?, "GET", u.path()).await
    }
}
impl<S: RequestSigner + Clone> Authentication for KalshiAuth<S> {
    fn identity(&self) -> String {
        format!("kalshi:{}", self.key_id)
    }
    async fn prepare(&self, _: &Kalshi, op: &Operation, r: &mut HttpRequest) -> Result<(), Error> {
        if op.auth == "bearer" {
            return Err(Error::Authentication(
                "clearing API requires a bearer token",
            ));
        }
        let u =
            url::Url::parse(&r.url).map_err(|_| Error::InvalidRequest("invalid request URL"))?;
        let m = match r.method {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Delete => "DELETE",
        };
        r.headers
            .extend(self.headers_at(unix_millis()?, m, u.path()).await?);
        Ok(())
    }
}
#[derive(Clone)]
pub struct KalshiBearerAuth(Arc<Zeroizing<String>>);
impl std::fmt::Debug for KalshiBearerAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("KalshiBearerAuth([redacted])")
    }
}
impl KalshiBearerAuth {
    pub fn new(token: Zeroizing<String>) -> Result<Self, Error> {
        if token.is_empty() || token.contains(['\r', '\n']) {
            return Err(Error::Authentication("invalid bearer token"));
        }
        Ok(Self(Arc::new(token)))
    }
}
impl Authentication for KalshiBearerAuth {
    fn identity(&self) -> String {
        use sha2::Digest;
        format!(
            "kalshi-bearer:{}",
            aux_rs_core::hex::encode(sha2::Sha256::digest(self.0.as_bytes()))
        )
    }
    async fn prepare(&self, _: &Kalshi, op: &Operation, r: &mut HttpRequest) -> Result<(), Error> {
        if op.service != "clearing"
            && !matches!(
                op.path.as_str(),
                "/portfolio/fills" | "/portfolio/positions" | "/historical/fills"
            )
        {
            return Err(Error::Authentication(
                "bearer credentials do not authorize this endpoint",
            ));
        }
        r.headers.push((
            "Authorization".into(),
            format!("Bearer {}", self.0.as_str()),
        ));
        Ok(())
    }
}
pub fn unix_millis() -> Result<u64, Error> {
    u64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| Error::Signing("clock before Unix epoch"))?
            .as_millis(),
    )
    .map_err(|_| Error::Signing("timestamp overflow"))
}
impl Kalshi {
    pub fn demo() -> Self {
        let mut e = Self::mainnet();
        e.network = "demo".into();
        e.endpoints.insert(
            "predictions".into(),
            "https://external-api.demo.kalshi.co/trade-api/v2".into(),
        );
        e.endpoints.insert(
            "perps".into(),
            "https://external-api.demo.kalshi.co/trade-api/v2".into(),
        );
        e.endpoints.insert(
            "clearing".into(),
            "https://demo-api.kalshi.co/klear-api/v1".into(),
        );
        for u in e.websocket.values_mut() {
            *u = u.replace(".kalshi.com", ".demo.kalshi.co");
        }
        e
    }
    pub fn testnet() -> Self {
        Self::demo()
    }
}
#[cfg(any(feature = "http", feature = "ws"))]
impl crate::KalshiClient<aux_rs_core::StandardTransport> {
    pub fn testnet() -> Result<Self, aux_rs_core::TransportError> {
        Self::with_config(Kalshi::demo(), Default::default())
    }
}
#[cfg(any(feature = "http", feature = "ws"))]
impl crate::KalshiPerpsClient<aux_rs_core::StandardTransport> {
    pub fn testnet() -> Result<Self, aux_rs_core::TransportError> {
        Self::with_config(Kalshi::demo(), Default::default())
    }
}
/// Verify a remote signer's native bytes before attaching the signature to a request.
pub fn verify_request_signature(
    public_pem: &str,
    message: &[u8],
    signature: &[u8],
) -> Result<(), Error> {
    use rsa::{pkcs8::DecodePublicKey, signature::Verifier};
    if let Ok(key) = ed25519_dalek::VerifyingKey::from_public_key_pem(public_pem) {
        let sig = ed25519_dalek::Signature::from_slice(signature)
            .map_err(|_| Error::Signing("invalid Ed25519 signature"))?;
        return key
            .verify_strict(message, &sig)
            .map_err(|_| Error::Signing("remote Ed25519 signature does not match public key"));
    }
    let key = rsa::RsaPublicKey::from_public_key_pem(public_pem)
        .map_err(|_| Error::Signing("invalid RSA or Ed25519 public PEM"))?;
    let verifier = rsa::pss::VerifyingKey::<sha2::Sha256>::new_with_salt_len(key, 32);
    let sig = rsa::pss::Signature::try_from(signature)
        .map_err(|_| Error::Signing("invalid RSA-PSS signature"))?;
    verifier
        .verify(message, &sig)
        .map_err(|_| Error::Signing("remote RSA signature does not match public key"))
}
impl<S> KalshiAuth<S> {
    pub fn signer(&self) -> &S {
        &self.signer
    }
    pub fn with_signer<B>(self, signer: B) -> KalshiAuth<B> {
        KalshiAuth {
            key_id: self.key_id,
            signer,
        }
    }
}

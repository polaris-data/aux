use crate::*;
use aux_rs_core::{Endpoint, Error as ClientError, HttpMethod, HttpRequest, HttpTransport, hex};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sha3::Keccak256;
use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    task::{Context, Poll, Waker},
    time::{SystemTime, UNIX_EPOCH},
};
use zeroize::Zeroizing;

pub trait SessionSigner: Send + Sync {
    fn public_key(&self) -> [u8; 32];
    fn sign(&self, message: &[u8]) -> impl Future<Output = Result<[u8; 64], Error>> + Send;
}
pub trait WalletSigner: Send + Sync {
    fn address(&self) -> String;
    /// Sign the complete challenge with EIP-191 personal_sign (or the wallet's native contract authorization).
    fn sign_message(&self, message: &str) -> impl Future<Output = Result<String, Error>> + Send;
}
impl<S: SessionSigner> SessionSigner for Arc<S> {
    fn public_key(&self) -> [u8; 32] {
        (**self).public_key()
    }
    async fn sign(&self, m: &[u8]) -> Result<[u8; 64], Error> {
        (**self).sign(m).await
    }
}
impl<S: WalletSigner> WalletSigner for Arc<S> {
    fn address(&self) -> String {
        (**self).address()
    }
    async fn sign_message(&self, m: &str) -> Result<String, Error> {
        (**self).sign_message(m).await
    }
}
/// Ed25519 seed storage is zeroized by ed25519-dalek; no automatic persistence.
pub struct LocalSessionSigner(ed25519_dalek::SigningKey);
impl std::fmt::Debug for LocalSessionSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("LocalSessionSigner(..)")
    }
}
impl LocalSessionSigner {
    pub fn generate() -> Result<Self, Error> {
        let mut seed = Zeroizing::new([0u8; 32]);
        getrandom::getrandom(seed.as_mut())
            .map_err(|_| Error::Signing("secure randomness unavailable"))?;
        Ok(Self::from_bytes(seed))
    }
    pub fn from_bytes(seed: Zeroizing<[u8; 32]>) -> Self {
        Self(ed25519_dalek::SigningKey::from_bytes(&seed))
    }
    pub fn from_hex(seed: Zeroizing<String>) -> Result<Self, Error> {
        let mut bytes = Zeroizing::new([0; 32]);
        hex::decode_to_slice(seed.strip_prefix("0x").unwrap_or(&seed), &mut *bytes)
            .map_err(|_| Error::Signing("expected a 32-byte session seed"))?;
        Ok(Self::from_bytes(bytes))
    }
    /// Explicit secret export for caller-owned secure persistence. Never serialized by the client.
    pub fn seed(&self) -> Zeroizing<[u8; 32]> {
        Zeroizing::new(self.0.to_bytes())
    }
}
impl SessionSigner for LocalSessionSigner {
    fn public_key(&self) -> [u8; 32] {
        self.0.verifying_key().to_bytes()
    }
    async fn sign(&self, m: &[u8]) -> Result<[u8; 64], Error> {
        use ed25519_dalek::Signer as _;
        Ok(self.0.sign(m).to_bytes())
    }
}
pub struct LocalWalletSigner {
    key: k256::ecdsa::SigningKey,
    address: String,
}
impl std::fmt::Debug for LocalWalletSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("LocalWalletSigner(..)")
    }
}
impl LocalWalletSigner {
    pub fn from_hex(key: Zeroizing<String>) -> Result<Self, Error> {
        let mut bytes = Zeroizing::new([0; 32]);
        hex::decode_to_slice(key.strip_prefix("0x").unwrap_or(&key), &mut *bytes)
            .map_err(|_| Error::Signing("expected a 32-byte wallet key"))?;
        let key = k256::ecdsa::SigningKey::from_slice(&*bytes)
            .map_err(|_| Error::Signing("invalid wallet key"))?;
        let hash = Keccak256::digest(&key.verifying_key().to_encoded_point(false).as_bytes()[1..]);
        Ok(Self {
            key,
            address: format!("0x{}", hex::encode(&hash[12..])),
        })
    }
}
impl WalletSigner for LocalWalletSigner {
    fn address(&self) -> String {
        self.address.clone()
    }
    async fn sign_message(&self, m: &str) -> Result<String, Error> {
        let digest = wallet_message_digest(m);
        let (sig, recovery) = self
            .key
            .sign_prehash_recoverable(&digest)
            .map_err(|_| Error::Signing("wallet unavailable"))?;
        if recovery.to_byte() > 1 {
            return Err(Error::Signing("invalid recovery ID"));
        }
        let mut bytes = sig.to_bytes().to_vec();
        bytes.push(recovery.to_byte() + 27);
        Ok(format!("0x{}", hex::encode(&bytes)))
    }
}
/// EIP-191 hashes the UTF-8 byte length, not the number of Unicode characters.
pub fn wallet_message_digest(message: &str) -> [u8; 32] {
    let mut h = Keccak256::new();
    h.update(format!("\x19Ethereum Signed Message:\n{}", message.len()));
    h.update(message.as_bytes());
    h.finalize().into()
}
pub fn wallet_address(s: &str) -> Result<String, Error> {
    if !s.starts_with("0x") || s.len() != 42 || !s.as_bytes()[2..].iter().all(u8::is_ascii_hexdigit)
    {
        return Err(Error::InvalidRequest("expected an Ethereum address"));
    }
    Ok(s.to_ascii_lowercase())
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RequestSigning {
    #[default]
    Legacy,
    Rfc9421,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LoginOptions {
    pub invite_code: Option<String>,
    pub referral_code: Option<String>,
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub request_signing: RequestSigning,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SessionInfo {
    pub address: String,
    pub client_id: String,
    pub session_public_key: String,
    pub expires_at: u64,
    pub chain_id: u64,
}
static NEXT_SESSION: AtomicU64 = AtomicU64::new(1);
struct SessionState {
    id: u64,
    failure: Mutex<Option<serde_json::Value>>,
    expires: AtomicU64,
    revoked: AtomicBool,
    revision: AtomicU64,
    next_waiter: AtomicU64,
    waiters: Mutex<HashMap<u64, Waker>>,
}
impl SessionState {
    fn notify(&self) {
        self.revision.fetch_add(1, Ordering::SeqCst);
        let waiters = std::mem::take(&mut *self.waiters.lock().unwrap_or_else(|e| e.into_inner()));
        for w in waiters.into_values() {
            w.wake();
        }
    }
}
struct Changed {
    state: Arc<SessionState>,
    revision: u64,
    id: u64,
}
impl Future for Changed {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut waiters = self.state.waiters.lock().unwrap_or_else(|e| e.into_inner());
        if self.state.revision.load(Ordering::SeqCst) != self.revision {
            waiters.remove(&self.id);
            Poll::Ready(())
        } else {
            waiters.insert(self.id, cx.waker().clone());
            Poll::Pending
        }
    }
}
impl Drop for Changed {
    fn drop(&mut self) {
        self.state
            .waiters
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(&self.id);
    }
}
pub struct MonacoAuth<S> {
    pub(crate) signer: Arc<S>,
    exchange: Monaco,
    address: String,
    client_id: String,
    signing: RequestSigning,
    state: Arc<SessionState>,
}
impl<S> Clone for MonacoAuth<S> {
    fn clone(&self) -> Self {
        Self {
            signer: self.signer.clone(),
            exchange: self.exchange.clone(),
            address: self.address.clone(),
            client_id: self.client_id.clone(),
            signing: self.signing,
            state: self.state.clone(),
        }
    }
}
impl<S> std::fmt::Debug for MonacoAuth<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("MonacoAuth(..)")
    }
}
pub fn unix_millis() -> Result<u64, Error> {
    u64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| Error::Signing("clock before epoch"))?
            .as_millis(),
    )
    .map_err(|_| Error::Signing("clock overflow"))
}
impl<S: SessionSigner> MonacoAuth<S> {
    /// Restore a registered session for exactly these endpoints/chain and identity.
    pub fn restore(
        exchange: Monaco,
        signer: S,
        info: SessionInfo,
        signing: RequestSigning,
    ) -> Result<Self, Error> {
        if UNIX_EPOCH
            .checked_add(std::time::Duration::from_secs(info.expires_at))
            .is_none()
        {
            return Err(Error::InvalidRequest("session expiry out of range"));
        }
        if info.chain_id != exchange.chain_id()
            || info.session_public_key != hex::encode(signer.public_key())
        {
            return Err(Error::Authentication("session context/key mismatch"));
        }
        ed25519_dalek::VerifyingKey::from_bytes(&signer.public_key())
            .map_err(|_| Error::Signing("invalid session public key"))?;
        nonempty(&info.client_id)?;
        let auth = Self {
            signer: Arc::new(signer),
            exchange,
            address: wallet_address(&info.address)?,
            client_id: info.client_id,
            signing,
            state: Arc::new(SessionState {
                id: NEXT_SESSION.fetch_add(1, Ordering::Relaxed),
                failure: Mutex::new(None),
                expires: AtomicU64::new(info.expires_at),
                revoked: AtomicBool::new(false),
                revision: AtomicU64::new(0),
                next_waiter: AtomicU64::new(0),
                waiters: Mutex::default(),
            }),
        };
        auth.check(&auth.exchange)?;
        Ok(auth)
    }
    /// Rebind an equivalent signer (for example, a caller's async execution context)
    /// while retaining shared session expiry and revocation state.
    pub fn with_signer<B: SessionSigner>(&self, signer: B) -> Result<MonacoAuth<B>, Error> {
        if signer.public_key() != self.signer.public_key() {
            return Err(Error::Authentication("replacement signer key mismatch"));
        }
        Ok(MonacoAuth {
            signer: Arc::new(signer),
            exchange: self.exchange.clone(),
            address: self.address.clone(),
            client_id: self.client_id.clone(),
            signing: self.signing,
            state: self.state.clone(),
        })
    }
    pub fn signer(&self) -> &S {
        &self.signer
    }
    pub fn session_info(&self) -> SessionInfo {
        SessionInfo {
            address: self.address.clone(),
            client_id: self.client_id.clone(),
            session_public_key: hex::encode(self.signer.public_key()),
            expires_at: self.state.expires.load(Ordering::SeqCst),
            chain_id: self.exchange.chain_id(),
        }
    }
    pub fn is_active(&self) -> bool {
        !self.state.revoked.load(Ordering::SeqCst)
            && unix_millis().is_ok_and(|now| now / 1000 < self.state.expires.load(Ordering::SeqCst))
    }
    pub(crate) fn session_id(&self) -> u64 {
        self.state.id
    }
    pub(crate) fn failure(&self) -> Option<serde_json::Value> {
        self.state
            .failure
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }
    pub(crate) fn reject(&self, body: serde_json::Value) {
        *self.state.failure.lock().unwrap_or_else(|e| e.into_inner()) = Some(body);
        self.invalidate();
    }
    pub fn invalidate(&self) {
        self.state.revoked.store(true, Ordering::SeqCst);
        self.state.notify();
    }
    pub(crate) fn check(&self, exchange: &Monaco) -> Result<(), Error> {
        if exchange != &self.exchange {
            return Err(Error::Authentication("endpoint or chain mismatch"));
        }
        if !self.is_active() {
            if let Some(body) = self.failure() {
                return Err(Error::Api { status: None, body });
            }
            return Err(Error::Authentication("session expired or revoked"));
        }
        Ok(())
    }
    pub(crate) fn changed(&self) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
        Box::pin(Changed {
            state: self.state.clone(),
            revision: self.state.revision.load(Ordering::SeqCst),
            id: self.state.next_waiter.fetch_add(1, Ordering::SeqCst),
        })
    }
    pub(crate) fn expires_at(&self) -> SystemTime {
        UNIX_EPOCH + std::time::Duration::from_secs(self.state.expires.load(Ordering::SeqCst))
    }
    pub async fn sign_checked(&self, message: &[u8]) -> Result<[u8; 64], Error> {
        self.check(&self.exchange)?;
        let signature = self.signer.sign(message).await?;
        ed25519_dalek::VerifyingKey::from_bytes(&self.signer.public_key())
            .map_err(|_| Error::Signing("invalid session public key"))?
            .verify_strict(message, &ed25519_dalek::Signature::from_bytes(&signature))
            .map_err(|_| Error::Signing("session signature did not verify"))?;
        self.check(&self.exchange)?;
        Ok(signature)
    }
    pub async fn sign_request_at(
        &self,
        request: &mut HttpRequest,
        timestamp_ms: u64,
    ) -> Result<(), Error> {
        self.check(&self.exchange)?;
        let url = url::Url::parse(&request.url)
            .map_err(|_| Error::InvalidRequest("invalid request URL"))?;
        // Reject cross-origin/prefix requests before invoking an external signer.
        let prefix = format!(
            "{}/api/v1/",
            aux_rs_core::Exchange::http_url(&self.exchange)
        );
        if !request.url.starts_with(&prefix)
            || url.fragment().is_some()
            || !url.username().is_empty()
            || url.password().is_some()
        {
            return Err(Error::Authentication("request outside session endpoint"));
        }
        let target = match url.query() {
            Some(q) if !q.is_empty() => format!("{}?{q}", url.path()),
            _ => url.path().into(),
        };
        let method = match request.method {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Delete => "DELETE",
        };
        let public = hex::encode(self.signer.public_key());
        let hash = Sha256::digest(&request.body);
        if request.headers.iter().any(|(k, _)| {
            k.to_ascii_lowercase().starts_with("x-monaco-")
                || ["signature", "signature-input", "content-digest"]
                    .contains(&k.to_ascii_lowercase().as_str())
        }) {
            return Err(Error::InvalidRequest(
                "authentication headers cannot be overridden",
            ));
        }
        match self.signing {
            RequestSigning::Legacy => {
                let message = format!("{method}\n{target}\n{timestamp_ms}\n{}", hex::encode(hash));
                let sig = self.sign_checked(message.as_bytes()).await?;
                request.headers.extend([
                    ("X-Monaco-PublicKey".into(), public),
                    ("X-Monaco-Timestamp".into(), timestamp_ms.to_string()),
                    ("X-Monaco-Signature".into(), hex::encode(sig)),
                ]);
            }
            RequestSigning::Rfc9421 => {
                let digest = format!("sha-256=:{}:", STANDARD.encode(hash));
                let params = format!(
                    "(\"@method\" \"@path\" \"@query\" \"content-digest\");created={};keyid=\"{public}\";alg=\"ed25519\"",
                    timestamp_ms / 1000
                );
                let query = url
                    .query()
                    .map(|q| format!("?{q}"))
                    .unwrap_or_else(|| "?".into());
                let message = format!(
                    "\"@method\": {method}\n\"@path\": {}\n\"@query\": {query}\n\"content-digest\": {digest}\n\"@signature-params\": {params}",
                    url.path()
                );
                let sig = self.sign_checked(message.as_bytes()).await?;
                request.headers.extend([
                    ("Content-Digest".into(), digest),
                    ("Signature-Input".into(), format!("monaco={params}")),
                    (
                        "Signature".into(),
                        format!("monaco=:{}:", STANDARD.encode(sig)),
                    ),
                ]);
            }
        }
        Ok(())
    }
}
impl<T: HttpTransport, A: Sync> MonacoClient<T, A> {
    pub async fn create_challenge(
        &self,
        r: ChallengeRequest,
    ) -> Result<ChallengeResponse, ClientError<Error>> {
        self.transport.ensure_open()?;
        wallet_address(&r.address).map_err(ClientError::Exchange)?;
        let mut key = [0; 32];
        hex::decode_to_slice(&r.session_public_key, &mut key).map_err(|_| {
            ClientError::Exchange(Error::InvalidRequest("invalid session public key"))
        })?;
        let w = wire(
            &self.exchange,
            HttpMethod::Post,
            "/api/v1/auth/challenge",
            &[],
            Some(
                &serde_json::to_value(r)
                    .map_err(Error::from)
                    .map_err(ClientError::Exchange)?,
            ),
        )
        .map_err(ClientError::Exchange)?;
        decode(self.transport.execute(w).await?).map_err(ClientError::Exchange)
    }
    pub async fn verify_signature(
        &self,
        r: VerifyRequest,
    ) -> Result<VerifyResponse, ClientError<Error>> {
        self.transport.ensure_open()?;
        wallet_address(&r.address).map_err(ClientError::Exchange)?;
        nonempty(&r.nonce).map_err(ClientError::Exchange)?;
        let signature = r.signature.strip_prefix("0x").unwrap_or(&r.signature);
        if signature.is_empty()
            || signature.len() > 16384
            || !signature.len().is_multiple_of(2)
            || !signature.bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err(ClientError::Exchange(Error::InvalidRequest(
                "invalid wallet signature encoding",
            )));
        }
        let w = wire(
            &self.exchange,
            HttpMethod::Post,
            "/api/v1/auth/verify",
            &[],
            Some(
                &serde_json::to_value(r)
                    .map_err(Error::from)
                    .map_err(ClientError::Exchange)?,
            ),
        )
        .map_err(ClientError::Exchange)?;
        decode(self.transport.execute(w).await?).map_err(ClientError::Exchange)
    }
    pub async fn login<W: WalletSigner, S: SessionSigner>(
        &self,
        wallet: &W,
        client_id: impl Into<String>,
        session: S,
        options: LoginOptions,
    ) -> Result<MonacoAuth<S>, ClientError<Error>> {
        let client_id = client_id.into();
        nonempty(&client_id).map_err(ClientError::Exchange)?;
        let address = wallet_address(&wallet.address()).map_err(ClientError::Exchange)?;
        let public = hex::encode(session.public_key());
        let challenge = self
            .create_challenge(ChallengeRequest {
                address: address.clone(),
                client_id: Some(client_id.clone()),
                chain_id: Some(self.exchange.chain_id().to_string()),
                session_public_key: public.clone(),
            })
            .await?;
        let expires = challenge
            .expires_at
            .and_then(|v| u64::try_from(v).ok())
            .ok_or(ClientError::Exchange(Error::Authentication(
                "missing challenge expiry",
            )))?;
        if unix_millis().map_err(ClientError::Exchange)? / 1000 >= expires {
            return Err(ClientError::Exchange(Error::Authentication(
                "challenge expired",
            )));
        }
        let message = challenge
            .message
            .ok_or(ClientError::Exchange(Error::Authentication(
                "missing challenge message",
            )))?;
        // Do not authorize a challenge that fails to bind the requested session key.
        if !message.contains(&public) {
            return Err(ClientError::Exchange(Error::Authentication(
                "challenge does not bind session key",
            )));
        }
        let signature = wallet
            .sign_message(&message)
            .await
            .map_err(ClientError::Exchange)?;
        if unix_millis().map_err(ClientError::Exchange)? / 1000 >= expires {
            return Err(ClientError::Exchange(Error::Authentication(
                "challenge expired while signing",
            )));
        }
        let response = self
            .verify_signature(VerifyRequest {
                address: address.clone(),
                client_id: Some(client_id.clone()),
                chain_id: Some(self.exchange.chain_id().to_string()),
                nonce: challenge
                    .nonce
                    .ok_or(ClientError::Exchange(Error::Authentication(
                        "missing nonce",
                    )))?,
                signature,
                session_public_key: public.clone(),
                invite_code: options.invite_code,
                referral_code: options.referral_code,
            })
            .await?;
        if response
            .user
            .as_ref()
            .and_then(|u| u.address.as_deref())
            .map(wallet_address)
            .transpose()
            .map_err(ClientError::Exchange)?
            .as_ref()
            .is_some_and(|a| a != &address)
        {
            return Err(ClientError::Exchange(Error::Authentication(
                "verified wallet mismatch",
            )));
        }
        let expires_at = response
            .expires_at
            .and_then(|v| u64::try_from(v).ok())
            .ok_or(ClientError::Exchange(Error::Authentication(
                "missing session expiry",
            )))?;
        MonacoAuth::restore(
            self.exchange.clone(),
            session,
            SessionInfo {
                address,
                client_id,
                session_public_key: public,
                expires_at,
                chain_id: self.exchange.chain_id(),
            },
            options.request_signing,
        )
        .map_err(ClientError::Exchange)
    }
}
impl<T: HttpTransport, S: SessionSigner> MonacoClient<T, MonacoAuth<S>> {
    pub(crate) async fn signed_request<R: Endpoint<Monaco, Access = SessionRequired>>(
        &self,
        r: R,
    ) -> Result<R::Response, ClientError<Error>> {
        self.transport.ensure_open()?;
        self.auth
            .check(&self.exchange)
            .map_err(ClientError::Exchange)?;
        let mut w = r.request(&self.exchange).map_err(ClientError::Exchange)?;
        self.auth
            .sign_request_at(&mut w, unix_millis().map_err(ClientError::Exchange)?)
            .await
            .map_err(ClientError::Exchange)?;
        // Sign once and execute once, including reads: transport retries could reuse an expired signature.
        R::decode(self.transport.execute(w).await?).map_err(ClientError::Exchange)
    }
    async fn session_operation<R: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<R, ClientError<Error>> {
        self.transport.ensure_open()?;
        self.auth
            .check(&self.exchange)
            .map_err(ClientError::Exchange)?;
        let mut w = wire(
            &self.exchange,
            HttpMethod::Post,
            path,
            &[],
            Some(&serde_json::json!({})),
        )
        .map_err(ClientError::Exchange)?;
        self.auth
            .sign_request_at(&mut w, unix_millis().map_err(ClientError::Exchange)?)
            .await
            .map_err(ClientError::Exchange)?;
        decode(self.transport.execute(w).await?).map_err(ClientError::Exchange)
    }
    pub async fn refresh_session(&self) -> Result<RefreshResponse, ClientError<Error>> {
        let r: RefreshResponse = self.session_operation("/api/v1/auth/refresh").await?;
        let expiry = r
            .expires_at
            .and_then(|v| u64::try_from(v).ok())
            .filter(|v| *v > unix_millis().unwrap_or(u64::MAX) / 1000)
            .ok_or(ClientError::Exchange(Error::Authentication(
                "invalid refreshed expiry",
            )))?;
        self.auth.state.expires.store(expiry, Ordering::SeqCst);
        self.auth.state.notify();
        Ok(r)
    }
    /// Invalidates every local clone and private stream even if the server response is lost.
    pub async fn revoke_session(&self) -> Result<RevokeResponse, ClientError<Error>> {
        struct InvalidateOnDrop<'a, S: SessionSigner>(&'a MonacoAuth<S>);
        impl<S: SessionSigner> Drop for InvalidateOnDrop<'_, S> {
            fn drop(&mut self) {
                self.0.invalidate();
            }
        }
        let _invalidate = InvalidateOnDrop(&self.auth);
        self.session_operation("/api/v1/auth/revoke").await
    }
}

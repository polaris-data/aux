use crate::{Error, Qfex};
use aux_rs_core::hex;
use hmac::{Hmac, Mac};
use serde_json::{Value, json};
use sha2::Sha256;
use std::{
    future::Future,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use zeroize::Zeroizing;
/// Replaceable native authentication for REST and the trade WebSocket.
pub trait AuthHandler: Send + Sync {
    fn headers(&self) -> Result<Vec<(String, String)>, Error>;
    fn trade_url(&self, exchange: &Qfex) -> Result<String, Error>;
    fn authenticate(&self) -> impl Future<Output = Result<Value, Error>> + Send;
}
#[derive(Clone)]
pub struct QfexAuth {
    public_key: String,
    secret: Arc<Zeroizing<String>>,
    account_id: Option<String>,
    builder_code: Option<String>,
}
impl std::fmt::Debug for QfexAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QfexAuth").finish_non_exhaustive()
    }
}
impl QfexAuth {
    pub fn new(public_key: impl Into<String>, secret: Zeroizing<String>) -> Result<Self, Error> {
        let public_key = public_key.into();
        if public_key.is_empty()
            || !public_key
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
            || secret.is_empty()
        {
            return Err(Error::InvalidRequest("invalid QFEX API credentials"));
        }
        Ok(Self {
            public_key,
            secret: Arc::new(secret),
            account_id: None,
            builder_code: None,
        })
    }
    pub fn with_account(mut self, account_id: impl Into<String>) -> Result<Self, Error> {
        let s = account_id.into();
        validate_uuid(&s)?;
        self.account_id = Some(s);
        Ok(self)
    }
    pub fn with_builder(mut self, builder_code: impl Into<String>) -> Result<Self, Error> {
        let s = builder_code.into();
        validate_uuid(&s)?;
        self.builder_code = Some(s);
        Ok(self)
    }
    /// Deterministic native signature primitive; nonce uniqueness is the caller's responsibility.
    pub fn signature(&self, nonce: &str, unix_ts: u64) -> Result<String, Error> {
        if nonce.is_empty() || nonce.len() > 100 || !nonce.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(Error::InvalidRequest(
                "nonce must be 1 to 100 hex characters",
            ));
        }
        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret.as_bytes())
            .map_err(|_| Error::Signing("invalid HMAC key"))?;
        mac.update(format!("{nonce}:{unix_ts}").as_bytes());
        Ok(hex::encode(mac.finalize().into_bytes()))
    }
    fn fresh(&self) -> Result<(String, u64, String), Error> {
        let mut bytes = [0u8; 16];
        getrandom::getrandom(&mut bytes)
            .map_err(|_| Error::Signing("OS randomness unavailable"))?;
        let nonce = hex::encode(bytes);
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| Error::Signing("clock before Unix epoch"))?
            .as_secs();
        let signature = self.signature(&nonce, ts)?;
        Ok((nonce, ts, signature))
    }
}
fn validate_uuid(s: &str) -> Result<(), Error> {
    if s.len() != 36
        || !s.bytes().enumerate().all(|(i, b)| {
            if [8, 13, 18, 23].contains(&i) {
                b == b'-'
            } else {
                b.is_ascii_hexdigit()
            }
        })
    {
        Err(Error::InvalidRequest("expected a UUID"))
    } else {
        Ok(())
    }
}
impl AuthHandler for QfexAuth {
    fn headers(&self) -> Result<Vec<(String, String)>, Error> {
        let (nonce, ts, sig) = self.fresh()?;
        let mut h = vec![
            ("x-qfex-public-key".into(), self.public_key.clone()),
            ("x-qfex-hmac-signature".into(), sig),
            ("x-qfex-nonce".into(), nonce),
            ("x-qfex-timestamp".into(), ts.to_string()),
        ];
        if let Some(id) = &self.account_id {
            h.push(("x-qfex-requested-account-id".into(), id.clone()));
        }
        Ok(h)
    }
    fn trade_url(&self, e: &Qfex) -> Result<String, Error> {
        if e.trade_url().contains(['?', '#']) {
            return Err(Error::InvalidRequest(
                "trade URL must not contain query or fragment",
            ));
        }
        Ok(format!(
            "{}?api_key={}",
            e.trade_url(),
            aux_rs_core::url_component(&self.public_key)
        ))
    }
    async fn authenticate(&self) -> Result<Value, Error> {
        let (nonce, ts, sig) = self.fresh()?;
        let mut p = json!({"hmac":{"public_key":self.public_key,"nonce":nonce,"unix_ts":ts,"signature":sig}});
        if let Some(a) = &self.account_id {
            p["account_id"] = json!(a);
        }
        if let Some(b) = &self.builder_code {
            p["builder_code"] = json!(b);
        }
        Ok(json!({"type":"auth","params":p}))
    }
}

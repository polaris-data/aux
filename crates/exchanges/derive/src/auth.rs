use crate::{
    ActionPayload, Derive, Error, Signer, SigningPayload, SigningScheme, address, personal_hash,
    verify_signature,
};
use aux_rs_core::hex;
use std::{
    collections::HashMap,
    future::Future,
    sync::{Arc, Mutex, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};
pub(crate) fn now_ms() -> Result<u64, Error> {
    u64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| Error::InvalidRequest("clock precedes Unix epoch"))?
            .as_millis(),
    )
    .map_err(|_| Error::InvalidRequest("clock overflow"))
}
type Nonces = Mutex<HashMap<(u64, [u8; 20]), Arc<Mutex<u64>>>>;
static NONCES: OnceLock<Nonces> = OnceLock::new();
/// HTTP login and WebSocket login use the same EIP-191 timestamp proof.
pub trait Authentication: Send + Sync {
    fn wallet(&self) -> &str;
    fn signer_address(&self) -> [u8; 20];
    fn nonce(&self) -> Result<String, Error>;
    fn login(
        &self,
        exchange: &Derive,
    ) -> impl Future<Output = Result<serde_json::Value, Error>> + Send;
    fn sign_action(
        &self,
        exchange: &Derive,
        action: &ActionPayload,
    ) -> impl Future<Output = Result<String, Error>> + Send;
}
#[derive(Clone)]
pub struct DeriveAuth<S> {
    owner: String,
    signer: S,
    chain_id: u64,
    matching: String,
    nonce: Arc<Mutex<u64>>,
}
impl<S> std::fmt::Debug for DeriveAuth<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeriveAuth").finish_non_exhaustive()
    }
}
impl<S: Signer> DeriveAuth<S> {
    pub fn new(exchange: &Derive, owner: &str, signer: S) -> Result<Self, Error> {
        let owner = format!("0x{}", hex::encode(address(owner)?));
        let nonce = NONCES
            .get_or_init(Default::default)
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .entry((exchange.chain_id, signer.address()))
            .or_default()
            .clone();
        Ok(Self {
            owner,
            signer,
            chain_id: exchange.chain_id,
            matching: exchange.matching.clone(),
            nonce,
        })
    }
    pub fn signer(&self) -> &S {
        &self.signer
    }
    fn check(&self, e: &Derive) -> Result<(), Error> {
        if self.chain_id != e.chain_id || self.matching != e.matching {
            return Err(Error::Authentication("signing network mismatch"));
        }
        Ok(())
    }
}
impl<S: Signer> Authentication for DeriveAuth<S> {
    fn wallet(&self) -> &str {
        &self.owner
    }
    fn signer_address(&self) -> [u8; 20] {
        self.signer.address()
    }
    fn nonce(&self) -> Result<String, Error> {
        let mut value = self.nonce.lock().unwrap_or_else(|e| e.into_inner());
        *value = now_ms()?
            .checked_mul(1_000_000)
            .and_then(|n| value.checked_add(1).map(|previous| n.max(previous)))
            .ok_or(Error::InvalidRequest("nonce overflow"))?;
        Ok(value.to_string())
    }
    async fn login(&self, e: &Derive) -> Result<serde_json::Value, Error> {
        self.check(e)?;
        let timestamp = now_ms()?.to_string();
        let digest = personal_hash(timestamp.as_bytes());
        let signature = self
            .signer
            .sign(&SigningPayload {
                digest,
                chain_id: e.chain_id,
                scheme: SigningScheme::Login,
            })
            .await?;
        verify_signature(self.signer.address(), &digest, &signature)?;
        Ok(
            serde_json::json!({"wallet":self.owner,"timestamp":timestamp,"signature":format!("0x{}",hex::encode(signature))}),
        )
    }
    async fn sign_action(&self, e: &Derive, action: &ActionPayload) -> Result<String, Error> {
        self.check(e)?;
        if address(&action.owner)? != address(&self.owner)?
            || address(&action.signer)? != self.signer.address()
        {
            return Err(Error::Signing("action owner or signer mismatch"));
        }
        let digest = action.digest(e.chain_id, &e.matching)?;
        let signature = self
            .signer
            .sign(&SigningPayload {
                digest,
                chain_id: e.chain_id,
                scheme: SigningScheme::Action,
            })
            .await?;
        verify_signature(self.signer.address(), &digest, &signature)?;
        Ok(format!("0x{}", hex::encode(signature)))
    }
}

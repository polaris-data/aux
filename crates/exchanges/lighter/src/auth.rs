use crate::{
    Error, Lighter, Signer, SigningPayload, signing::SigningPurpose, trading::validate_identity,
};
use aux_rs_core::hex;
use futures_util::lock::{Mutex as AsyncMutex, MutexGuard};
use std::{
    collections::BTreeMap,
    fmt,
    sync::{Arc, Mutex, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};

pub(crate) fn unix_millis() -> Result<i64, Error> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|d| i64::try_from(d.as_millis()).ok())
        .ok_or(Error::InvalidRequest("clock outside Unix timestamp range"))
}
pub(crate) fn unix_seconds() -> Result<i64, Error> {
    Ok(unix_millis()? / 1000)
}

/// Sequential state is shared across independent clients and auth handlers in
/// this process by (chain, account, API key). Other processes need coordination.
#[derive(Clone)]
pub struct NonceSequence {
    identity: (u32, i64, u8),
    state: Arc<AsyncMutex<NonceState>>,
}
#[derive(Default)]
pub(crate) struct NonceState {
    next: Option<i64>,
    uncertain: bool,
}
impl NonceState {
    pub(crate) fn next(&self) -> Option<i64> {
        self.next
    }
    pub(crate) fn uncertain(&self) -> bool {
        self.uncertain
    }
    pub(crate) fn set(&mut self, next: i64) {
        self.next = Some(next);
        self.uncertain = false;
    }
    pub(crate) fn mark_uncertain(&mut self) {
        self.uncertain = true;
    }
}
type Registry = BTreeMap<(u32, i64, u8), Arc<AsyncMutex<NonceState>>>;
impl NonceSequence {
    pub fn for_identity(chain_id: u32, account: i64, key: u8) -> Result<Self, Error> {
        validate_identity(account, key)?;
        static STATES: OnceLock<Mutex<Registry>> = OnceLock::new();
        let identity = (chain_id, account, key);
        let mut states = STATES
            .get_or_init(Mutex::default)
            .lock()
            .map_err(|_| Error::Signing("nonce registry poisoned"))?;
        let state = states.entry(identity).or_default().clone();
        Ok(Self { identity, state })
    }
    pub(crate) async fn lock(&self) -> MutexGuard<'_, NonceState> {
        self.state.lock().await
    }
}
impl fmt::Debug for NonceSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NonceSequence")
            .field("identity", &self.identity)
            .finish_non_exhaustive()
    }
}

/// Applications may replace credential handling while retaining the shared
/// sequential nonce coordinator and exchange-native signer contract.
pub trait AuthHandler: Send + Sync {
    type Signer: Signer;
    fn signer(&self) -> &Self::Signer;
    fn account_index(&self) -> i64;
    fn api_key_index(&self) -> u8;
    fn chain_id(&self) -> u32;
    fn nonce_sequence(&self) -> &NonceSequence;
    fn check_exchange(&self, exchange: &Lighter) -> Result<(), Error> {
        if exchange.chain_id() != self.chain_id()
            || self.nonce_sequence().identity
                != (self.chain_id(), self.account_index(), self.api_key_index())
        {
            return Err(Error::NetworkMismatch);
        }
        validate_identity(self.account_index(), self.api_key_index())
    }
}
#[derive(Clone)]
pub struct LighterAuth<S> {
    signer: S,
    nonce: NonceSequence,
}
impl<S: Signer> LighterAuth<S> {
    pub fn new(
        signer: S,
        chain_id: u32,
        account_index: i64,
        api_key_index: u8,
    ) -> Result<Self, Error> {
        Ok(Self {
            signer,
            nonce: NonceSequence::for_identity(chain_id, account_index, api_key_index)?,
        })
    }
    pub async fn token(&self, deadline: i64) -> Result<AuthToken, Error> {
        sign_auth_token(self, deadline).await
    }
}
impl<S: Signer> AuthHandler for LighterAuth<S> {
    type Signer = S;
    fn signer(&self) -> &S {
        &self.signer
    }
    fn account_index(&self) -> i64 {
        self.nonce.identity.1
    }
    fn api_key_index(&self) -> u8 {
        self.nonce.identity.2
    }
    fn chain_id(&self) -> u32 {
        self.nonce.identity.0
    }
    fn nonce_sequence(&self) -> &NonceSequence {
        &self.nonce
    }
}
impl<S> fmt::Debug for LighterAuth<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LighterAuth")
            .field("nonce", &self.nonce)
            .finish_non_exhaustive()
    }
}

/// The native token signs deadline:account:key. Chain ID is deliberately absent
/// from that native hash; the SDK binds the credential to its client context.
pub fn auth_signing_payload(
    chain_id: u32,
    account_index: i64,
    api_key_index: u8,
    deadline: i64,
) -> Result<SigningPayload, Error> {
    validate_identity(account_index, api_key_index)?;
    if deadline <= 0 {
        return Err(Error::InvalidRequest(
            "auth deadline must be a positive Unix timestamp in seconds",
        ));
    }
    let message = format!("{deadline}:{account_index}:{api_key_index}");
    let words: Vec<_> = message
        .as_bytes()
        .chunks(8)
        .map(|chunk| {
            let mut bytes = [0; 8];
            bytes[..chunk.len()].copy_from_slice(chunk);
            poseidon_hash::Goldilocks::from_canonical_u64(u64::from_le_bytes(bytes))
        })
        .collect();
    let digest = crate::signing::encode_fp5(poseidon_hash::hash_to_quintic_extension(&words));
    Ok(SigningPayload {
        digest,
        chain_id,
        account_index,
        api_key_index,
        purpose: SigningPurpose::AuthToken { deadline },
    })
}
pub async fn sign_auth_token<A: AuthHandler>(auth: &A, deadline: i64) -> Result<AuthToken, Error> {
    let now = unix_seconds()?;
    if deadline <= now || deadline > now + 8 * 60 * 60 {
        return Err(Error::InvalidRequest(
            "auth deadline must be in the next eight hours (seconds)",
        ));
    }
    let payload = auth_signing_payload(
        auth.chain_id(),
        auth.account_index(),
        auth.api_key_index(),
        deadline,
    )?;
    let sig = auth.signer().sign(&payload).await?;
    sig.verify(&payload, auth.signer().public_key())?;
    Ok(AuthToken {
        value: zeroize::Zeroizing::new(format!(
            "{deadline}:{}:{}:{}",
            auth.account_index(),
            auth.api_key_index(),
            hex::encode(sig.as_bytes())
        )),
        chain_id: auth.chain_id(),
        account_index: auth.account_index(),
        api_key_index: auth.api_key_index(),
        deadline,
    })
}
/// Bearer credential. Formatting always redacts it; as_str explicitly opts in.
#[derive(Clone)]
pub struct AuthToken {
    value: zeroize::Zeroizing<String>,
    chain_id: u32,
    account_index: i64,
    api_key_index: u8,
    deadline: i64,
}
impl AuthToken {
    pub fn as_str(&self) -> &str {
        &self.value
    }
    pub fn deadline(&self) -> i64 {
        self.deadline
    }
    pub(crate) fn scope(&self) -> String {
        format!(
            "lighter:{}:{}:{}:{}",
            self.chain_id, self.account_index, self.api_key_index, self.deadline
        )
    }
    pub(crate) fn check(&self, exchange: &Lighter, account: i64) -> Result<(), Error> {
        if exchange.chain_id() != self.chain_id || account != self.account_index {
            return Err(Error::NetworkMismatch);
        }
        if unix_seconds()? >= self.deadline {
            return Err(Error::InvalidRequest("auth token expired"));
        }
        Ok(())
    }
}
impl fmt::Debug for AuthToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AuthToken")
            .field("deadline", &self.deadline)
            .finish_non_exhaustive()
    }
}

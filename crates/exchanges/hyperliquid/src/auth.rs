use crate::{
    Action, Address, Error, ExecutionOptions, Network, Signature, Signer, SignerError,
    signing::l1_signing_payload,
};
use serde::Serialize;
use std::{
    collections::HashMap,
    future::Future,
    sync::{
        Arc, Mutex, OnceLock,
        atomic::{AtomicU64, Ordering},
    },
    time::{SystemTime, UNIX_EPOCH},
};

type Registry = Mutex<HashMap<Address, Arc<AtomicU64>>>;
static NONCES: OnceLock<Registry> = OnceLock::new();

/// Native authentication/preparation hook. Implement this to replace nonce
/// coordination or signing while retaining the native trading extension API.
pub trait AuthHandler: Send + Sync {
    fn account_address(&self) -> Address;
    fn signer_address(&self) -> Address;
    fn prepare(
        &self,
        action: Action,
        network: Network,
        options: ExecutionOptions,
    ) -> impl Future<Output = Result<SignedRequest, Error>> + Send;
}

/// Monotonic in-process nonces, shared by signing identity rather than account.
#[derive(Clone, Debug)]
pub struct NonceManager(Arc<AtomicU64>);
impl NonceManager {
    pub fn shared_for(signer: Address) -> Self {
        let mut registry = NONCES
            .get_or_init(Mutex::default)
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        // Retain counters for the process lifetime: dropping and reconstructing
        // every client within one millisecond must not reuse a previous nonce.
        Self(
            registry
                .entry(signer)
                .or_insert_with(|| Arc::new(AtomicU64::new(0)))
                .clone(),
        )
    }

    pub fn next_at(&self, unix_milliseconds: u64) -> Result<u64, Error> {
        let mut current = self.0.load(Ordering::Relaxed);
        loop {
            let next = unix_milliseconds.max(current.checked_add(1).ok_or(Error::NonceExhausted)?);
            match self
                .0
                .compare_exchange_weak(current, next, Ordering::Relaxed, Ordering::Relaxed)
            {
                Ok(_) => return Ok(next),
                Err(updated) => current = updated,
            }
        }
    }

    pub fn next(&self) -> Result<u64, Error> {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| Error::Clock)?;
        self.next_at(time.as_millis().try_into().map_err(|_| Error::Clock)?)
    }
}

/// Agent keys sign for an account; a vault changes the target, never the signer.
pub struct HyperliquidAuth<S> {
    signer: Arc<S>,
    signer_address: Address,
    account: Address,
    vault_address: Option<Address>,
    nonces: NonceManager,
}
impl<S> Clone for HyperliquidAuth<S> {
    fn clone(&self) -> Self {
        Self {
            signer: self.signer.clone(),
            signer_address: self.signer_address,
            account: self.account,
            vault_address: self.vault_address,
            nonces: self.nonces.clone(),
        }
    }
}
impl<S> std::fmt::Debug for HyperliquidAuth<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HyperliquidAuth")
            .field("signer", &self.signer_address)
            .field("account", &self.account)
            .field("vault_address", &self.vault_address)
            .finish_non_exhaustive()
    }
}
impl<S: Signer> HyperliquidAuth<S> {
    pub fn direct(signer: S) -> Self {
        let account = signer.address();
        Self::agent(signer, account)
    }
    pub fn agent(signer: S, account: Address) -> Self {
        let signer_address = signer.address();
        Self {
            signer: Arc::new(signer),
            signer_address,
            account,
            vault_address: None,
            nonces: NonceManager::shared_for(signer_address),
        }
    }
    pub fn with_vault(mut self, vault: Address) -> Self {
        self.vault_address = Some(vault);
        self.account = vault;
        self
    }
    pub fn account_address(&self) -> Address {
        self.account
    }
    pub fn signer_address(&self) -> Address {
        self.signer_address
    }
    pub fn vault_address(&self) -> Option<Address> {
        self.vault_address
    }
    pub fn nonce_manager(&self) -> &NonceManager {
        &self.nonces
    }

    pub async fn prepare(
        &self,
        action: Action,
        network: Network,
        options: ExecutionOptions,
    ) -> Result<SignedRequest, Error> {
        action.validate()?;
        if self.signer.address() != self.signer_address {
            return Err(Error::Signing(SignerError::AddressMismatch));
        }
        let nonce = match options.nonce {
            Some(nonce) => nonce,
            None => self.nonces.next()?,
        };
        if options.expires_after.is_some_and(|expiry| expiry <= nonce) {
            return Err(Error::InvalidRequest(
                "expiresAfter must be greater than the nonce timestamp",
            ));
        }
        let payload = l1_signing_payload(
            &action,
            self.vault_address,
            nonce,
            options.expires_after,
            network,
        )?;
        let signature = self
            .signer
            .sign(payload.clone())
            .await
            .map_err(Error::Signing)?;
        SignedRequest::from_signature(
            action,
            nonce,
            signature,
            self.vault_address,
            options.expires_after,
            network,
            self.signer_address,
        )
    }
}

impl<S: Signer> AuthHandler for HyperliquidAuth<S> {
    fn account_address(&self) -> Address {
        self.account
    }
    fn signer_address(&self) -> Address {
        self.signer_address
    }
    async fn prepare(
        &self,
        action: Action,
        network: Network,
        options: ExecutionOptions,
    ) -> Result<SignedRequest, Error> {
        HyperliquidAuth::prepare(self, action, network, options).await
    }
}

/// Prepared fields cannot be mutated after signing. A caller can inspect or
/// serialize this envelope, but cannot replace the action or signing context.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedRequest {
    #[serde(skip)]
    network: Network,
    #[serde(skip)]
    signer_address: Address,
    action: Action,
    nonce: u64,
    signature: Signature,
    #[serde(skip_serializing_if = "Option::is_none")]
    vault_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_after: Option<u64>,
}
impl SignedRequest {
    /// Build an immutable envelope from a custom handler's signature. Validate
    /// the action and recover its signer before it can reach the transport.
    pub fn from_signature(
        action: Action,
        nonce: u64,
        signature: Signature,
        vault_address: Option<Address>,
        expires_after: Option<u64>,
        network: Network,
        expected_signer: Address,
    ) -> Result<Self, Error> {
        action.validate()?;
        if expires_after.is_some_and(|expiry| expiry <= nonce) {
            return Err(Error::InvalidRequest(
                "expiresAfter must be greater than the nonce timestamp",
            ));
        }
        let payload = l1_signing_payload(&action, vault_address, nonce, expires_after, network)?;
        if signature
            .recover_address(&payload)
            .map_err(Error::Signing)?
            != expected_signer
        {
            return Err(Error::Signing(SignerError::AddressMismatch));
        }
        Ok(Self {
            network,
            signer_address: expected_signer,
            action,
            nonce,
            signature,
            vault_address,
            expires_after,
        })
    }
    pub fn action(&self) -> &Action {
        &self.action
    }
    pub fn network(&self) -> Network {
        self.network
    }
    pub fn signer_address(&self) -> Address {
        self.signer_address
    }
    pub fn nonce(&self) -> u64 {
        self.nonce
    }
    pub fn signature(&self) -> &Signature {
        &self.signature
    }
    pub fn vault_address(&self) -> Option<Address> {
        self.vault_address
    }
    pub fn expires_after(&self) -> Option<u64> {
        self.expires_after
    }
}
impl std::fmt::Debug for SignedRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SignedRequest")
            .field("nonce", &self.nonce)
            .field("vault_address", &self.vault_address)
            .field("expires_after", &self.expires_after)
            .finish_non_exhaustive()
    }
}

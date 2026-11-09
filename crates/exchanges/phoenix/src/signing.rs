use crate::Error;
use ed25519_dalek::Signer as _;
use std::{future::Future, sync::Arc};
use zeroize::Zeroizing;
/// Signs exact Solana message or login-challenge bytes. Callers inspect before signing.
pub trait Signer: Send + Sync {
    fn public_key(&self) -> [u8; 32];
    fn sign(&self, message: &[u8]) -> impl Future<Output = Result<[u8; 64], Error>> + Send;
}
#[derive(Clone)]
pub struct LocalSigner {
    key: Arc<Zeroizing<[u8; 32]>>,
}
impl std::fmt::Debug for LocalSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalSigner").finish_non_exhaustive()
    }
}
impl LocalSigner {
    pub fn from_hex(value: Zeroizing<String>) -> Result<Self, Error> {
        let mut key = Zeroizing::new([0; 32]);
        aux_rs_core::hex::decode_to_slice(value.strip_prefix("0x").unwrap_or(&value), &mut *key)
            .map_err(|_| Error::Signing("32-byte Ed25519 seed required"))?;
        Ok(Self { key: Arc::new(key) })
    }
}
impl Signer for LocalSigner {
    fn public_key(&self) -> [u8; 32] {
        ed25519_dalek::SigningKey::from_bytes(&self.key)
            .verifying_key()
            .to_bytes()
    }
    async fn sign(&self, message: &[u8]) -> Result<[u8; 64], Error> {
        Ok(ed25519_dalek::SigningKey::from_bytes(&self.key)
            .sign(message)
            .to_bytes())
    }
}
pub fn public_key(value: &str) -> Result<[u8; 32], Error> {
    let key: [u8; 32] = bs58::decode(value)
        .into_vec()
        .map_err(|_| Error::InvalidRequest("invalid Solana public key"))?
        .try_into()
        .map_err(|_| Error::InvalidRequest("32-byte public key required"))?;
    Ok(key)
}
pub fn verify_signature(key: &[u8; 32], message: &[u8], sig: &[u8; 64]) -> Result<(), Error> {
    ed25519_dalek::VerifyingKey::from_bytes(key)
        .map_err(|_| Error::Signing("invalid public key"))?
        .verify_strict(message, &ed25519_dalek::Signature::from_bytes(sig))
        .map_err(|_| Error::Signing("signature does not match signer/message"))
}
/// Native trader PDA; indexes remain explicit and distinct from the wallet authority.
pub fn trader_address(
    authority: &str,
    pda_index: u8,
    subaccount_index: u8,
) -> Result<String, Error> {
    let authority = solana_pubkey::Pubkey::new_from_array(public_key(authority)?);
    let (pda, _) = solana_pubkey::Pubkey::find_program_address(
        &[
            b"trader",
            authority.as_ref(),
            &[pda_index, subaccount_index],
        ],
        &instructions_program(),
    );
    Ok(pda.to_string())
}
fn instructions_program() -> solana_pubkey::Pubkey {
    crate::instructions::constants::PROD_PHOENIX_PROGRAM_ID
}

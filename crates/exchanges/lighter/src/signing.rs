//! ECgFp5/Schnorr contracts, independent of HTTP and the async runtime.
use crate::Error;
use goldilocks_crypto::{Point, ScalarField};
use std::{fmt, future::Future};
use zeroize::Zeroizing;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PublicKey([u8; 40]);
impl PublicKey {
    pub fn from_bytes(bytes: [u8; 40]) -> Result<Self, Error> {
        canonical_digest(&bytes)?;
        goldilocks_crypto::validate_public_key(&bytes)
            .map_err(|_| Error::Signing("invalid public key"))?;
        Ok(Self(bytes))
    }
    pub fn as_bytes(&self) -> &[u8; 40] {
        &self.0
    }
}
impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PublicKey")
            .field(&hex::encode(self.0))
            .finish()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Signature([u8; 80]);
impl Signature {
    pub fn from_bytes(bytes: [u8; 80]) -> Result<Self, Error> {
        if !canonical_scalar(&bytes[..40]) || !canonical_scalar(&bytes[40..]) {
            return Err(Error::InvalidSignature);
        }
        Ok(Self(bytes))
    }
    pub fn as_bytes(&self) -> &[u8; 80] {
        &self.0
    }
    pub fn verify(&self, payload: &SigningPayload, key: PublicKey) -> Result<(), Error> {
        // Use the primitive operations directly: the upstream verifier prints
        // signature material on failure in debug builds, which we must not do.
        let scalar =
            |bytes: &[u8]| ScalarField::from_bytes_le(bytes).map_err(|_| Error::InvalidSignature);
        let point = goldilocks_crypto::Fp5Element::from_bytes_le(&key.0)
            .ok()
            .and_then(|p| Point::decode(&p))
            .ok_or(Error::InvalidSignature)?;
        let s = scalar(&self.0[..40])?;
        let e = scalar(&self.0[40..])?;
        let r = Point::generator().mul(&s).add(&point.mul(&e)).encode();
        let message = goldilocks_crypto::Fp5Element::from_bytes_le(&payload.digest)
            .map_err(|_| Error::InvalidSignature)?;
        let mut preimage = [poseidon_hash::Goldilocks::zero(); 10];
        preimage[..5].copy_from_slice(&r.0);
        preimage[5..].copy_from_slice(&message.0);
        let expected =
            ScalarField::from_fp5_element(&poseidon_hash::hash_to_quintic_extension(&preimage));
        if e == expected {
            Ok(())
        } else {
            Err(Error::InvalidSignature)
        }
    }
}
impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Signature").finish_non_exhaustive()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SigningPurpose {
    Transaction { tx_type: u8, nonce: i64 },
    AuthToken { deadline: i64 },
}

/// Immutable native digest and its context for local or remote policy checks.
#[derive(Clone, Debug)]
pub struct SigningPayload {
    pub(crate) digest: [u8; 40],
    pub(crate) chain_id: u32,
    pub(crate) account_index: i64,
    pub(crate) api_key_index: u8,
    pub(crate) purpose: SigningPurpose,
}
impl SigningPayload {
    pub fn digest(&self) -> &[u8; 40] {
        &self.digest
    }
    pub fn chain_id(&self) -> u32 {
        self.chain_id
    }
    pub fn account_index(&self) -> i64 {
        self.account_index
    }
    pub fn api_key_index(&self) -> u8 {
        self.api_key_index
    }
    pub fn purpose(&self) -> SigningPurpose {
        self.purpose
    }
}
pub trait Signer: Send + Sync {
    fn public_key(&self) -> PublicKey;
    fn sign(
        &self,
        payload: &SigningPayload,
    ) -> impl Future<Output = Result<Signature, Error>> + Send;
}
impl<S: Signer> Signer for std::sync::Arc<S> {
    fn public_key(&self) -> PublicKey {
        (**self).public_key()
    }
    async fn sign(&self, payload: &SigningPayload) -> Result<Signature, Error> {
        (**self).sign(payload).await
    }
}

/// Raw native 40-byte little-endian scalar, not an Ethereum private key or seed.
/// Cryptographic dependencies are unaudited; see SIGNING.md before deployment.
pub struct LocalSigner {
    key: Zeroizing<[u8; 40]>,
    public_key: PublicKey,
}
impl fmt::Debug for LocalSigner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LocalSigner")
            .field("public_key", &self.public_key)
            .finish_non_exhaustive()
    }
}
impl LocalSigner {
    pub fn from_bytes(key: Zeroizing<[u8; 40]>) -> Result<Self, Error> {
        if key.iter().all(|v| *v == 0) || !canonical_scalar(key.as_ref()) {
            return Err(Error::Signing(
                "private key must be a nonzero canonical 40-byte scalar",
            ));
        }
        let scalar = Zeroizing::new(
            ScalarField::from_bytes_le(key.as_ref())
                .map_err(|_| Error::Signing("invalid private key"))?,
        );
        let public_key =
            PublicKey::from_bytes(encode_fp5(Point::generator().mul(&scalar).encode()))?;
        Ok(Self { key, public_key })
    }
    pub fn from_hex(key: Zeroizing<String>) -> Result<Self, Error> {
        let mut bytes = Zeroizing::new([0; 40]);
        hex::decode_to_slice(key.strip_prefix("0x").unwrap_or(&key), bytes.as_mut())
            .map_err(|_| Error::Signing("private key must encode 40 bytes"))?;
        Self::from_bytes(bytes)
    }
}
impl Signer for LocalSigner {
    fn public_key(&self) -> PublicKey {
        self.public_key
    }
    async fn sign(&self, payload: &SigningPayload) -> Result<Signature, Error> {
        canonical_digest(&payload.digest)?;
        // OS-backed CSPRNG. Explicit Schnorr nonces are never exposed in the SDK.
        let mut nonce_bytes = Zeroizing::new([0; 40]);
        loop {
            getrandom::getrandom(nonce_bytes.as_mut())
                .map_err(|_| Error::Signing("operating-system randomness unavailable"))?;
            if nonce_bytes.iter().any(|v| *v != 0) && canonical_scalar(nonce_bytes.as_ref()) {
                break;
            }
        }
        let bytes = goldilocks_crypto::sign_hashed_message(
            self.key.as_ref(),
            &payload.digest,
            nonce_bytes.as_ref(),
        )
        .map_err(|_| Error::Signing("Schnorr operation failed"))?;
        let sig = Signature::from_bytes(bytes.try_into().map_err(|_| Error::InvalidSignature)?)?;
        sig.verify(payload, self.public_key)?;
        Ok(sig)
    }
}
fn canonical_scalar(bytes: &[u8]) -> bool {
    bytes.len() == 40
        && bytes
            .iter()
            .rev()
            .cmp(ScalarField::N.to_bytes_le().iter().rev())
            .is_lt()
}
pub(crate) fn canonical_digest(bytes: &[u8; 40]) -> Result<(), Error> {
    if bytes.chunks_exact(8).any(|c| {
        u64::from_le_bytes(c.try_into().expect("eight bytes")) >= poseidon_hash::Goldilocks::MODULUS
    }) {
        return Err(Error::Signing("noncanonical digest or public key"));
    }
    Ok(())
}
pub(crate) fn encode_fp5(value: poseidon_hash::Fp5Element) -> [u8; 40] {
    // Fp5Element::to_bytes_le in pinned 0.1.4 writes raw limbs; its arithmetic
    // permits noncanonical representations. Wire encodings must be canonical.
    let mut bytes = [0; 40];
    for (chunk, limb) in bytes.chunks_exact_mut(8).zip(value.0) {
        chunk.copy_from_slice(&limb.to_canonical_u64().to_le_bytes());
    }
    bytes
}

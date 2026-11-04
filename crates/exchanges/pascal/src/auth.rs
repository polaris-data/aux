use crate::{Authentication, Error, Operation, Pascal};
use aux_rs_core::{HttpRequest, hex};
use ed25519_dalek::Signer as _;
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    future::Future,
    sync::{Arc, Mutex, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};
use zeroize::Zeroizing;
pub trait Signer: Send + Sync + 'static {
    fn public_key(&self) -> [u8; 32];
    fn sign(&self, message: &[u8]) -> impl Future<Output = Result<[u8; 64], Error>> + Send;
}
impl<S: Signer> Signer for Arc<S> {
    fn public_key(&self) -> [u8; 32] {
        (**self).public_key()
    }
    async fn sign(&self, m: &[u8]) -> Result<[u8; 64], Error> {
        (**self).sign(m).await
    }
}
pub struct LocalSigner(ed25519_dalek::SigningKey);
impl std::fmt::Debug for LocalSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("LocalSigner([redacted])")
    }
}
impl LocalSigner {
    pub fn new(seed: Zeroizing<[u8; 32]>) -> Self {
        Self(ed25519_dalek::SigningKey::from_bytes(&seed))
    }
    pub fn from_hex(secret: Zeroizing<String>) -> Result<Self, Error> {
        let b = Zeroizing::new(
            hex::decode(&secret).map_err(|_| Error::Signing("invalid Ed25519 seed"))?,
        );
        let seed = <[u8; 32]>::try_from(&b[..])
            .map_err(|_| Error::Signing("expected 32-byte Ed25519 seed"))?;
        Ok(Self::new(Zeroizing::new(seed)))
    }
}
impl Signer for LocalSigner {
    fn public_key(&self) -> [u8; 32] {
        self.0.verifying_key().to_bytes()
    }
    async fn sign(&self, m: &[u8]) -> Result<[u8; 64], Error> {
        Ok(self.0.sign(m).to_bytes())
    }
}
pub fn public_key(s: &str) -> Result<[u8; 32], Error> {
    bs58::decode(s)
        .into_vec()
        .map_err(|_| Error::InvalidRequest("invalid base58 public key"))?
        .try_into()
        .map_err(|_| Error::InvalidRequest("expected 32-byte public key"))
}
pub fn integer(v: &Value) -> Result<u64, Error> {
    match v {
        Value::String(s) => s
            .parse()
            .map_err(|_| Error::InvalidRequest("invalid u64 decimal string")),
        Value::Number(n) => n
            .as_u64()
            .ok_or(Error::InvalidRequest("integer overflows u64")),
        _ => Err(Error::InvalidRequest("u64 string or integer required")),
    }
}
pub fn micro_usd(s: &str) -> Result<u64, Error> {
    if s.starts_with('-') || s.contains(['e', 'E', '+']) {
        return Err(Error::InvalidRequest("plain unsigned decimal required"));
    }
    let (whole, frac) = s.split_once('.').unwrap_or((s, ""));
    if whole.is_empty()
        || frac.len() > 6
        || !whole.bytes().all(|b| b.is_ascii_digit())
        || !frac.bytes().all(|b| b.is_ascii_digit())
    {
        return Err(Error::InvalidRequest("at most six decimal places required"));
    }
    let w: u64 = whole
        .parse()
        .map_err(|_| Error::InvalidRequest("decimal overflow"))?;
    let f: u64 = if frac.is_empty() {
        0
    } else {
        frac.parse()
            .map_err(|_| Error::InvalidRequest("invalid decimal"))?
    };
    w.checked_mul(1_000_000)
        .and_then(|x| x.checked_add(f * 10u64.pow(6 - frac.len() as u32)))
        .ok_or(Error::InvalidRequest("decimal overflow"))
}
fn text<'a>(v: &'a Value, k: &str) -> Result<&'a str, Error> {
    v[k].as_str()
        .ok_or(Error::InvalidRequest("missing native string field"))
}
fn fixed(s: &str, n: usize) -> Result<Vec<u8>, Error> {
    if !s.is_ascii() || s.len() > n || s.contains('\0') {
        return Err(Error::InvalidRequest("invalid fixed ASCII field"));
    }
    let mut b = vec![0; n];
    b[..s.len()].copy_from_slice(s.as_bytes());
    Ok(b)
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct PermitContext {
    pub deployment_id: u8,
    pub owner: String,
    pub signer: String,
    pub client_ts_ms: u64,
    pub recv_window_ms: u32,
}
impl PermitContext {
    fn validate(&self) -> Result<(), Error> {
        public_key(&self.owner)?;
        public_key(&self.signer)?;
        if self.recv_window_ms > 60_000 {
            return Err(Error::InvalidRequest("receive window exceeds 60000ms"));
        }
        Ok(())
    }
    fn header(&self, cmd: u8) -> Result<Vec<u8>, Error> {
        self.validate()?;
        let mut b = vec![1, cmd, self.deployment_id, 0];
        b.extend(self.recv_window_ms.to_le_bytes());
        b.extend(self.client_ts_ms.to_le_bytes());
        b.extend(public_key(&self.owner)?);
        b.extend(public_key(&self.signer)?);
        Ok(b)
    }
}
/// Final native bytes. Trading permits are binary; wallet/profile permits are compact UTF-8 JSON.
pub fn permit_message(kind: &str, request: &Value, c: &PermitContext) -> Result<Vec<u8>, Error> {
    c.validate()?;
    match kind {
        "place_orders" => {
            let mut b = c.header(1)?;
            let size = integer(&request["size"])?;
            if size == 0 {
                return Err(Error::InvalidRequest("positive size required"));
            }
            let px = micro_usd(text(request, "price")?)?;
            if px > 1_000_000 {
                return Err(Error::InvalidRequest("price exceeds one dollar"));
            }
            let id = integer(&request["client_order_id"])?;
            if id == u64::MAX {
                return Err(Error::InvalidRequest("reserved client order ID"));
            }
            let replace = request
                .get("replace_client_order_id")
                .filter(|x| !x.is_null())
                .map(integer)
                .transpose()?
                .unwrap_or(u64::MAX);
            let expiry = request
                .get("expires_ts_ms")
                .filter(|x| !x.is_null())
                .map(integer)
                .transpose()?
                .unwrap_or(0);
            let tif = match request.get("tif").and_then(Value::as_str).unwrap_or("GTC") {
                "GTC" => 0,
                "GTT" => 1,
                "IOC" => 2,
                _ => return Err(Error::InvalidRequest("invalid time in force")),
            };
            if (tif == 1) != (expiry != 0) {
                return Err(Error::InvalidRequest(
                    "GTT requires expiration; other policies forbid it",
                ));
            }
            let side = match text(request, "side")? {
                "BID" => 0,
                "ASK" => 1,
                _ => return Err(Error::InvalidRequest("invalid order side")),
            };
            let ty = match request
                .get("type")
                .or_else(|| request.get("order_type"))
                .and_then(Value::as_str)
                .unwrap_or("LIMIT")
            {
                "LIMIT" => 0,
                "MARKET" => 1,
                _ => return Err(Error::InvalidRequest("invalid order type")),
            };
            let mut flags = 0;
            for (i, k) in [
                "post_only",
                "reduce_only",
                "allow_missing_replace",
                "replace_with_open_size",
            ]
            .iter()
            .enumerate()
            {
                if request.get(*k).and_then(Value::as_bool).unwrap_or(false) {
                    flags |= 1 << i;
                }
            }
            if flags & 12 != 0 && replace == u64::MAX || flags & 12 == 12 {
                return Err(Error::InvalidRequest("invalid replacement flags"));
            }
            for v in [size, px, id, replace, expiry] {
                b.extend(v.to_le_bytes());
            }
            b.extend([0; 32]);
            b.extend(fixed(text(request, "symbol")?, 32)?);
            b.extend([0; 4]);
            b.extend(
                hex::decode("dcaf53738f1d5f06164660498a75888e7f0c78ed5f1bc17be1dd1cb3ccf53ce3")
                    .expect("constant hash"),
            );
            b.extend([0, side, ty, tif, flags]);
            b.extend([0; 7]);
            Ok(b)
        }
        "cancel_orders" => {
            let mut b = c.header(2)?;
            let (client, exchange) = (request.get("client_order_id"), request.get("order_id"));
            let (by, id) = match (client, exchange) {
                (Some(id), None) => (0, integer(id)?),
                (None, Some(id)) => (1, integer(id)?),
                _ => return Err(Error::InvalidRequest("one cancel identifier required")),
            };
            b.extend([by, 0, 0, 0, 0, 0, 0, 0]);
            b.extend(id.to_le_bytes());
            Ok(b)
        }
        "mass_cancel" => {
            let mut b = c.header(10)?;
            b.extend([0; 8]);
            b.extend(fixed(text(request, "symbol")?, 32)?);
            Ok(b)
        }
        _ => {
            let owner_only = matches!(
                kind,
                "create_trading_key"
                    | "revoke_trading_key"
                    | "withdrawal"
                    | "register_deposit_address"
            );
            if owner_only && c.owner != c.signer {
                return Err(Error::Authentication(
                    "wallet operation requires the owner signer",
                ));
            }
            let mut a = vec![json!(kind)];
            match kind {
                "create_trading_key" => {
                    let name = text(request, "name")?;
                    if name.len() > 16 {
                        return Err(Error::InvalidRequest(
                            "trading key name exceeds 16 UTF-8 bytes",
                        ));
                    }
                    public_key(text(request, "trading_key")?)?;
                    a.extend([
                        json!(name),
                        request["trading_key"].clone(),
                        json!(integer(&request["expiration_ts_ms"])?),
                    ])
                }
                "revoke_trading_key" => {
                    public_key(text(request, "trading_key")?)?;
                    a.push(request["trading_key"].clone())
                }
                "withdrawal" => {
                    public_key(text(request, "destination_authority")?)?;
                    public_key(text(request, "destination_token_account")?)?;
                    a.extend([
                        json!(micro_usd(text(request, "amount")?)?),
                        request["destination_authority"].clone(),
                        request["destination_token_account"].clone(),
                    ])
                }
                "register_deposit_address" => a.push(json!(c.owner)),
                "generate_referral_code" => {}
                "claim_custom_referral_code" | "apply_referral_code" => {
                    a.push(json!(text(request, "referral_code")?))
                }
                "set_username" => a.push(json!(text(request, "username")?)),
                "set_account_bio" => a.push(json!(text(request, "account_bio")?)),
                _ => return Err(Error::InvalidRequest("unknown permit kind")),
            }
            a.extend([
                json!(1),
                json!(c.deployment_id),
                json!(c.recv_window_ms),
                json!(c.client_ts_ms),
                json!(c.owner),
                json!(c.signer),
            ]);
            if kind == "register_deposit_address"
                && let Some(code) = request.get("referral_code").filter(|v| !v.is_null())
            {
                a.extend([json!("referral_code"), code.clone()]);
            }
            Ok(serde_json::to_vec(&a)?)
        }
    }
}
#[derive(Default)]
pub(crate) struct WalletState {
    pub busy: bool,
    pub pending: bool,
    last: u64,
}
type Registry = Mutex<HashMap<(u8, [u8; 32]), Arc<Mutex<WalletState>>>>;
fn wallet_state(deployment: u8, owner: [u8; 32]) -> Arc<Mutex<WalletState>> {
    static R: OnceLock<Registry> = OnceLock::new();
    R.get_or_init(Default::default)
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .entry((deployment, owner))
        .or_default()
        .clone()
}
pub struct Submission {
    state: Option<Arc<Mutex<WalletState>>>,
}
impl Submission {
    pub(crate) fn none() -> Self {
        Self { state: None }
    }
    pub(crate) fn sent(&mut self) {
        if let Some(s) = &self.state {
            s.lock().unwrap_or_else(|e| e.into_inner()).pending = true;
        }
    }
    pub(crate) fn finish(&mut self) {
        if let Some(s) = &self.state {
            s.lock().unwrap_or_else(|e| e.into_inner()).pending = false;
        }
    }
}
impl Drop for Submission {
    fn drop(&mut self) {
        if let Some(s) = &self.state {
            s.lock().unwrap_or_else(|e| e.into_inner()).busy = false;
        }
    }
}
#[derive(Clone)]
pub struct PascalAuth<S = Arc<LocalSigner>> {
    owner: [u8; 32],
    signer: S,
    pub recv_window_ms: u32,
}
impl<S> std::fmt::Debug for PascalAuth<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PascalAuth([redacted])")
    }
}
impl<S: Signer> PascalAuth<S> {
    pub fn new(owner: &str, signer: S) -> Result<Self, Error> {
        Ok(Self {
            owner: public_key(owner)?,
            signer,
            recv_window_ms: 5000,
        })
    }
    pub fn pending(&self, deployment: u8) -> bool {
        wallet_state(deployment, self.owner)
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .pending
    }
    /// Explicit caller attestation after reconciling a wallet write through native reads.
    pub fn reconcile_wallet(&self, deployment: u8) -> Result<(), Error> {
        let s = wallet_state(deployment, self.owner);
        let mut s = s.lock().unwrap_or_else(|e| e.into_inner());
        if s.busy {
            return Err(Error::PendingOutcome);
        }
        s.pending = false;
        Ok(())
    }
    pub async fn sign_request_at(
        &self,
        deployment: u8,
        kind: &str,
        request: &mut Value,
        timestamp: u64,
    ) -> Result<(), Error> {
        let c = PermitContext {
            deployment_id: deployment,
            owner: bs58::encode(self.owner).into_string(),
            signer: bs58::encode(self.signer.public_key()).into_string(),
            client_ts_ms: timestamp,
            recv_window_ms: self.recv_window_ms,
        };
        let m = permit_message(kind, request, &c)?;
        let signature = self.signer.sign(&m).await?;
        ed25519_dalek::VerifyingKey::from_bytes(&self.signer.public_key())
            .map_err(|_| Error::Signing("invalid public key"))?
            .verify_strict(&m, &ed25519_dalek::Signature::from_bytes(&signature))
            .map_err(|_| Error::Signing("external signature verification failed"))?;
        request["auth"] = json!({"client_ts_ms":timestamp.to_string(),"recv_window_ms":self.recv_window_ms.to_string(),"owner":c.owner,"signer":c.signer,"signature":bs58::encode(signature).into_string()});
        Ok(())
    }
}
pub(crate) fn operation_kind(op: &Operation) -> Result<&'static str, Error> {
    Ok(match op.path.as_str() {
        "/api/v1/orders" => "place_orders",
        "/api/v1/cancels" => "cancel_orders",
        "/api/v1/mass-cancels" => "mass_cancel",
        "/api/v1/trading-keys" => "create_trading_key",
        "/api/v1/trading-key-revocations" => "revoke_trading_key",
        "/api/v1/withdrawals" => "withdrawal",
        "/api/v1/deposit-addresses" => "register_deposit_address",
        "/api/v1/referral-codes" => "generate_referral_code",
        "/api/v1/referral-codes/custom" => "claim_custom_referral_code",
        "/api/v1/referral-codes/apply" => "apply_referral_code",
        "/api/v1/usernames" => "set_username",
        "/api/v1/account-bios" => "set_account_bio",
        _ => return Err(Error::InvalidRequest("unknown signed route")),
    })
}
impl<S: Signer + Clone> Authentication for PascalAuth<S> {
    fn identity(&self) -> String {
        format!(
            "{}:{}",
            bs58::encode(self.owner).into_string(),
            bs58::encode(self.signer.public_key()).into_string()
        )
    }
    fn begin(&self, e: &Pascal, op: &Operation) -> Result<Submission, Error> {
        if !matches!(
            operation_kind(op)?,
            "create_trading_key" | "revoke_trading_key" | "withdrawal" | "register_deposit_address"
        ) {
            return Ok(Submission::none());
        }
        let state = wallet_state(e.deployment_id, self.owner);
        {
            let mut s = state.lock().unwrap_or_else(|e| e.into_inner());
            if s.busy || s.pending {
                return Err(Error::PendingOutcome);
            }
            s.busy = true;
        }
        Ok(Submission { state: Some(state) })
    }
    async fn prepare(&self, e: &Pascal, op: &Operation, r: &mut HttpRequest) -> Result<(), Error> {
        let kind = operation_kind(op)?;
        let mut body: Value = serde_json::from_slice(&r.body)?;
        let time = u64::try_from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|_| Error::Signing("clock before Unix epoch"))?
                .as_millis(),
        )
        .map_err(|_| Error::Signing("timestamp overflow"))?;
        let timestamp = if self.owner == self.signer.public_key() {
            let s = wallet_state(e.deployment_id, self.owner);
            let mut s = s.lock().unwrap_or_else(|e| e.into_inner());
            s.last = time.max(
                s.last
                    .checked_add(1)
                    .ok_or(Error::Signing("timestamp exhausted"))?,
            );
            s.last
        } else {
            time
        };
        if let Value::Array(items) = &mut body {
            let max = if kind == "mass_cancel" { 10 } else { 50 };
            if items.is_empty() || items.len() > max {
                return Err(Error::InvalidRequest("invalid signed batch size"));
            }
            for item in items {
                self.sign_request_at(e.deployment_id, kind, item, timestamp)
                    .await?;
            }
        } else {
            self.sign_request_at(e.deployment_id, kind, &mut body, timestamp)
                .await?;
        }
        r.body = serde_json::to_vec(&body)?;
        Ok(())
    }
}
impl Pascal {
    pub fn canary() -> Self {
        let mut e = Self::mainnet();
        e.network = "canary".into();
        e.deployment_id = 2;
        for u in e.endpoints.values_mut().chain(e.websocket.values_mut()) {
            *u = u.replace(".pascal.trade", ".canary.pascal.trade");
        }
        e
    }
}
impl<S> PascalAuth<S> {
    pub fn signer(&self) -> &S {
        &self.signer
    }
    pub fn with_signer<B>(self, signer: B) -> PascalAuth<B> {
        PascalAuth {
            owner: self.owner,
            signer,
            recv_window_ms: self.recv_window_ms,
        }
    }
}

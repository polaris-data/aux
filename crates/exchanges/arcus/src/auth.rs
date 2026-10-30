use crate::*;
use aux_rs_core::{Error as ClientError, Exchange, HttpMethod, HttpRequest, HttpTransport};
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    future::Future,
    sync::{
        Arc, Mutex, OnceLock,
        atomic::{AtomicU64, Ordering},
    },
    time::{SystemTime, UNIX_EPOCH},
};
use zeroize::Zeroizing;
pub trait Signer: Send + Sync {
    fn public_key(&self) -> [u8; 32];
    fn sign(&self, message: &[u8]) -> impl Future<Output = Result<[u8; 64], Error>> + Send;
}
pub struct LocalSigner(ed25519_dalek::SigningKey);
impl LocalSigner {
    pub fn from_bytes(seed: Zeroizing<[u8; 32]>) -> Self {
        Self(ed25519_dalek::SigningKey::from_bytes(&seed))
    }
    pub fn from_hex(seed: Zeroizing<String>) -> Result<Self, Error> {
        let mut bytes = Zeroizing::new([0; 32]);
        hex::decode_to_slice(seed.strip_prefix("0x").unwrap_or(&seed), &mut *bytes)
            .map_err(|_| Error::InvalidRequest("expected a 32-byte Ed25519 seed"))?;
        Ok(Self::from_bytes(bytes))
    }
}
impl std::fmt::Debug for LocalSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalSigner").finish_non_exhaustive()
    }
}
impl Signer for LocalSigner {
    fn public_key(&self) -> [u8; 32] {
        self.0.verifying_key().to_bytes()
    }
    async fn sign(&self, message: &[u8]) -> Result<[u8; 64], Error> {
        use ed25519_dalek::Signer as _;
        Ok(self.0.sign(message).to_bytes())
    }
}
impl<S: Signer> Signer for Arc<S> {
    fn public_key(&self) -> [u8; 32] {
        (**self).public_key()
    }
    async fn sign(&self, message: &[u8]) -> Result<[u8; 64], Error> {
        (**self).sign(message).await
    }
}
pub struct ArcusAuth<S> {
    pub(crate) signer: Arc<S>,
    pub(crate) address: String,
    pub(crate) account_index: u8,
    clock: Arc<AtomicU64>,
}
impl<S> Clone for ArcusAuth<S> {
    fn clone(&self) -> Self {
        Self {
            signer: self.signer.clone(),
            address: self.address.clone(),
            account_index: self.account_index,
            clock: self.clock.clone(),
        }
    }
}
impl<S> std::fmt::Debug for ArcusAuth<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArcusAuth").finish_non_exhaustive()
    }
}
type Clocks = Mutex<HashMap<[u8; 32], Arc<AtomicU64>>>;
impl<S: Signer> ArcusAuth<S> {
    pub fn new(signer: S, value: impl AsRef<str>, account_index: u8) -> Result<Self, Error> {
        let account = AccountRequest::new(value, account_index)?;
        static CLOCKS: OnceLock<Clocks> = OnceLock::new();
        let public = signer.public_key();
        ed25519_dalek::VerifyingKey::from_bytes(&public)
            .map_err(|_| Error::Signing("invalid signer public key"))?;
        let clock = CLOCKS
            .get_or_init(Mutex::default)
            .lock()
            .map_err(|_| Error::Signing("timestamp registry poisoned"))?
            .entry(public)
            .or_default()
            .clone();
        Ok(Self {
            signer: Arc::new(signer),
            address: account.address,
            account_index,
            clock,
        })
    }
    pub fn api_key(&self) -> String {
        hex::encode(self.signer.public_key())
    }
    pub fn address(&self) -> &str {
        &self.address
    }
    pub fn account_index(&self) -> u8 {
        self.account_index
    }
    fn timestamp(&self) -> Result<u64, Error> {
        let now = u64::try_from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|_| Error::Signing("clock before epoch"))?
                .as_nanos(),
        )
        .map_err(|_| Error::Signing("timestamp overflow"))?;
        self.clock
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |old| {
                old.checked_add(1).map(|next| now.max(next))
            })
            .map(|old| now.max(old + 1))
            .map_err(|_| Error::Signing("timestamp exhausted"))
    }
    pub fn order_payload(
        &self,
        market: &MarketInfo,
        request: &OrderRequest,
        timestamp: u64,
    ) -> Result<Vec<u8>, Error> {
        if market.market_id != request.market_id {
            return Err(Error::InvalidRequest(
                "order market does not match supplied metadata",
            ));
        }
        let price = market.price_ticks(&request.price)?;
        let size = market.size_quantums(&request.quantity)?;
        let expiry = request
            .good_til_time
            .parse::<u64>()
            .map_err(|_| Error::InvalidRequest("goodTilTime must be Unix microseconds"))?
            .checked_mul(1000)
            .ok_or(Error::InvalidRequest("goodTilTime overflow"))?;
        if expiry <= timestamp {
            return Err(Error::InvalidRequest("goodTilTime must be in the future"));
        }
        // The signed denominator is always top-level tickSize. Tier rules constrain
        // acceptable prices but must never change the signed number of ticks.
        let mut payload = json!({"ad":self.address,"ai":self.account_index,"ct":timestamp,"g":expiry,"m":request.market_id,"op":1,"p":price,"q":size,"r":u8::from(request.reduce_only),"s":request.order_side.native(),"t":request.time_in_force.native(),"v":1});
        if let Some(c) = request.client_id.as_ref().filter(|s| !s.is_empty()) {
            payload["c"] = json!(c);
        }
        payload
            .as_object_mut()
            .expect("typed payload is an object")
            .sort_keys();
        Ok(serde_json::to_vec(&payload)?)
    }
    pub fn cancel_payload(&self, r: &CancelOrderRequest, timestamp: u64) -> Result<Vec<u8>, Error> {
        let id = r.order_id.as_ref().filter(|s| !s.is_empty());
        let c = r.client_id.as_ref().filter(|s| !s.is_empty());
        if id.is_some() == c.is_some() {
            return Err(Error::InvalidRequest(
                "cancel requires exactly one orderId or clientId",
            ));
        }
        let mut p = json!({"ad":self.address,"ai":self.account_index,"ct":timestamp,"m":r.market_id,"op":2,"v":1});
        if let Some(id) = id {
            p["id"] = json!(id);
        }
        if let Some(c) = c {
            p["c"] = json!(c);
        }
        p.as_object_mut()
            .expect("typed payload is an object")
            .sort_keys();
        Ok(serde_json::to_vec(&p)?)
    }
    async fn headers(
        &self,
        payload: &[u8],
        timestamp: u64,
    ) -> Result<Vec<(String, String)>, Error> {
        let signature = self.signer.sign(payload).await?;
        let key = ed25519_dalek::VerifyingKey::from_bytes(&self.signer.public_key())
            .map_err(|_| Error::Signing("invalid public key"))?;
        key.verify_strict(payload, &ed25519_dalek::Signature::from_bytes(&signature))
            .map_err(|_| Error::Signing("signer returned an invalid signature"))?;
        Ok(vec![
            ("X-API-Key".into(), self.api_key()),
            ("X-Timestamp".into(), timestamp.to_string()),
            ("X-Signature".into(), hex::encode(signature)),
            ("content-type".into(), "application/json".into()),
        ])
    }
}
impl<T: HttpTransport, S: Signer> ArcusClient<T, ArcusAuth<S>> {
    async fn submit(
        &self,
        path: &str,
        mut body: Value,
        payload: Vec<u8>,
        timestamp: u64,
    ) -> Result<OrderResponse, ClientError<Error>> {
        self.transport.ensure_open()?;
        body["address"] = json!(self.auth.address);
        body["accountIndex"] = json!(self.auth.account_index);
        body["timestamp"] = json!(timestamp);
        let headers = self
            .auth
            .headers(&payload, timestamp)
            .await
            .map_err(ClientError::Exchange)?;
        let request = HttpRequest {
            method: HttpMethod::Post,
            url: format!("{}{path}", self.exchange.http_url()),
            headers,
            body: serde_json::to_vec(&body).map_err(|e| ClientError::Exchange(Error::Decode(e)))?,
        };
        self.transport.ensure_open()?;
        crate::endpoints::decode(self.transport.execute(request).await?)
            .map_err(ClientError::Exchange)
    }
    pub async fn place_order(
        &self,
        market: &MarketInfo,
        r: OrderRequest,
    ) -> Result<OrderResponse, ClientError<Error>> {
        self.transport.ensure_open()?;
        let ts = self.auth.timestamp().map_err(ClientError::Exchange)?;
        let p = self
            .auth
            .order_payload(market, &r, ts)
            .map_err(ClientError::Exchange)?;
        let body = serde_json::to_value(r).map_err(|e| ClientError::Exchange(Error::Decode(e)))?;
        self.submit("/v1/placeOrder", body, p, ts).await
    }
    pub async fn cancel_order(
        &self,
        r: CancelOrderRequest,
    ) -> Result<OrderResponse, ClientError<Error>> {
        self.transport.ensure_open()?;
        let ts = self.auth.timestamp().map_err(ClientError::Exchange)?;
        let p = self
            .auth
            .cancel_payload(&r, ts)
            .map_err(ClientError::Exchange)?;
        let body = serde_json::to_value(r).map_err(|e| ClientError::Exchange(Error::Decode(e)))?;
        self.submit("/v1/cancelOrder", body, p, ts).await
    }
}

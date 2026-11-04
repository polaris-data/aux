use crate::{Authentication, Error, Operation, Polymarket};
use alloy_dyn_abi::eip712::TypedData;
use alloy_primitives::{Address, B256, Signature};
use aux_rs_core::{HttpMethod, HttpRequest};
pub use aux_rs_evm::{EvmSigner, LocalEvmSigner};
use base64::{
    Engine,
    engine::general_purpose::{STANDARD, URL_SAFE, URL_SAFE_NO_PAD},
};
use hmac::{Hmac, Mac};
use serde_json::{Value, json};
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use zeroize::Zeroizing;
const ORDER_TYPE: &str = "Order(uint256 salt,address maker,address signer,uint256 tokenId,uint256 makerAmount,uint256 takerAmount,uint8 side,uint8 signatureType,uint256 timestamp,bytes32 metadata,bytes32 builder)";
const ZERO: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";
#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum PositionSystem {
    Ctf,
    V2,
}
#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct OrderDomain {
    pub chain_id: u64,
    pub exchange: Address,
    pub version: u8,
}
impl OrderDomain {
    pub fn polygon(system: PositionSystem, negative_risk: bool) -> Self {
        let (version, address) = match (system, negative_risk) {
            (PositionSystem::V2, _) => (3, "0xe3333700cA9d93003F00f0F71f8515005F6c00Aa"),
            (PositionSystem::Ctf, false) => (2, "0xE111180000d2663C0091e4f400237545B87B996B"),
            (PositionSystem::Ctf, true) => (2, "0xe2222d279d744050d28e00520010520000310F59"),
        };
        Self {
            chain_id: 137,
            exchange: address.parse().expect("pinned address"),
            version,
        }
    }
}
pub fn typed_data_hash(payload: &Value) -> Result<B256, Error> {
    let td: TypedData = serde_json::from_value(payload.clone())
        .map_err(|_| Error::Signing("invalid EIP-712 typed data"))?;
    td.eip712_signing_hash()
        .map_err(|_| Error::Signing("EIP-712 hashing failed"))
}
fn types(fields: &[(&str, &str)]) -> Value {
    Value::Array(
        fields
            .iter()
            .map(|(name, ty)| json!({"name":name,"type":ty}))
            .collect(),
    )
}
fn domain_types(contract: bool) -> Value {
    let mut f = vec![
        ("name", "string"),
        ("version", "string"),
        ("chainId", "uint256"),
    ];
    if contract {
        f.push(("verifyingContract", "address"));
    }
    types(&f)
}
pub fn wallet_auth_typed_data(chain: u64, address: Address, timestamp: u64, nonce: u64) -> Value {
    json!({"domain":{"name":"ClobAuthDomain","version":"1","chainId":chain},"primaryType":"ClobAuth","types":{"EIP712Domain":domain_types(false),"ClobAuth":types(&[("address","address"),("timestamp","string"),("nonce","uint256"),("message","string")])},"message":{"address":address,"timestamp":timestamp.to_string(),"nonce":nonce,"message":"This message attests that I control the given wallet"}})
}
pub fn order_typed_data(order: &Value, domain: OrderDomain) -> Result<Value, Error> {
    if !matches!(domain.version, 2 | 3) || domain.chain_id == 0 {
        return Err(Error::InvalidRequest("unsupported order domain"));
    }
    let mut message = json!({});
    for k in [
        "salt",
        "maker",
        "signer",
        "tokenId",
        "makerAmount",
        "takerAmount",
        "signatureType",
        "timestamp",
    ] {
        let v = order
            .get(k)
            .filter(|v| !v.is_null())
            .ok_or(Error::InvalidRequest("missing signed order field"))?;
        message[k] = v.clone();
    }
    message["side"] = match order["side"].as_str() {
        Some("BUY") => json!(0),
        Some("SELL") => json!(1),
        _ => match order["side"].as_u64() {
            Some(n @ 0..=1) => json!(n),
            _ => return Err(Error::InvalidRequest("invalid order side")),
        },
    };
    message["metadata"] = order
        .get("metadata")
        .filter(|v| v.as_str() != Some(""))
        .cloned()
        .unwrap_or(json!(ZERO));
    message["builder"] = order
        .get("builder")
        .filter(|v| v.as_str() != Some(""))
        .cloned()
        .unwrap_or(json!(ZERO));
    let order_fields = types(&[
        ("salt", "uint256"),
        ("maker", "address"),
        ("signer", "address"),
        ("tokenId", "uint256"),
        ("makerAmount", "uint256"),
        ("takerAmount", "uint256"),
        ("side", "uint8"),
        ("signatureType", "uint8"),
        ("timestamp", "uint256"),
        ("metadata", "bytes32"),
        ("builder", "bytes32"),
    ]);
    let mut td = json!({"domain":{"name":"Polymarket CTF Exchange","version":domain.version.to_string(),"chainId":domain.chain_id,"verifyingContract":domain.exchange},"primaryType":"Order","types":{"EIP712Domain":domain_types(true),"Order":order_fields},"message":message});
    let signature_type = integer(&order["signatureType"])?;
    if signature_type > 3 {
        return Err(Error::InvalidRequest("unsupported wallet signature type"));
    }
    if signature_type == 3 {
        td["primaryType"] = json!("TypedDataSign");
        td["types"]["TypedDataSign"] = types(&[
            ("contents", "Order"),
            ("name", "string"),
            ("version", "string"),
            ("chainId", "uint256"),
            ("verifyingContract", "address"),
            ("salt", "bytes32"),
        ]);
        td["message"] = json!({"contents":message,"name":"DepositWallet","version":"1","chainId":domain.chain_id,"verifyingContract":order["signer"],"salt":ZERO});
    }
    typed_data_hash(&td)?;
    Ok(td)
}
pub fn integer(v: &Value) -> Result<u64, Error> {
    match v {
        Value::Number(n) => n
            .as_u64()
            .ok_or(Error::InvalidRequest("u64 integer required")),
        Value::String(s) => s
            .parse()
            .map_err(|_| Error::InvalidRequest("u64 integer required")),
        _ => Err(Error::InvalidRequest("integer required")),
    }
}
pub async fn sign_typed_data<S: EvmSigner>(
    signer: &S,
    payload: &Value,
) -> Result<Signature, Error> {
    let h = typed_data_hash(payload)?;
    let sig = signer
        .sign_digest(h)
        .await
        .map_err(|_| Error::Signing("external EVM signer failed"))?;
    if sig.normalize_s().is_some()
        || sig
            .recover_address_from_prehash(&h)
            .map_err(|_| Error::Signing("invalid EVM signature"))?
            != signer.address()
    {
        return Err(Error::Signing("signature does not match signer"));
    }
    Ok(sig)
}
pub async fn sign_order<S: EvmSigner>(
    signer: &S,
    order: &Value,
    domain: OrderDomain,
) -> Result<String, Error> {
    let td = order_typed_data(order, domain)?;
    let sig = sign_typed_data(signer, &td).await?;
    let mut result = sig.to_string();
    if integer(&order["signatureType"])? == 3 {
        let mut inner = td.clone();
        inner["primaryType"] = json!("Order");
        inner["message"] = td["message"]["contents"].clone();
        let typed: TypedData =
            serde_json::from_value(inner).map_err(|_| Error::Signing("invalid order data"))?;
        let contents = typed
            .hash_struct()
            .map_err(|_| Error::Signing("order hashing failed"))?;
        result.push_str(&aux_rs_core::hex::encode(typed.domain.separator()));
        result.push_str(&aux_rs_core::hex::encode(contents));
        result.push_str(&aux_rs_core::hex::encode(ORDER_TYPE));
        result.push_str(&format!("{:04x}", ORDER_TYPE.len()));
    }
    Ok(result)
}
#[derive(Clone)]
pub struct ApiCredentials {
    pub key: String,
    secret: Arc<Zeroizing<Vec<u8>>>,
    passphrase: Arc<Zeroizing<String>>,
}
impl std::fmt::Debug for ApiCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ApiCredentials([redacted])")
    }
}
impl ApiCredentials {
    pub fn new(
        key: String,
        secret: Zeroizing<String>,
        passphrase: Zeroizing<String>,
    ) -> Result<Self, Error> {
        if key.is_empty() || key.contains(['\r', '\n']) || passphrase.contains(['\r', '\n']) {
            return Err(Error::Authentication("invalid API credentials"));
        }
        let bytes = URL_SAFE
            .decode(secret.as_bytes())
            .or_else(|_| URL_SAFE_NO_PAD.decode(secret.as_bytes()))
            .or_else(|_| STANDARD.decode(secret.as_bytes()))
            .map_err(|_| Error::Authentication("invalid base64 API secret"))?;
        if bytes.is_empty() {
            return Err(Error::Authentication("empty API secret"));
        }
        Ok(Self {
            key,
            secret: Arc::new(Zeroizing::new(bytes)),
            passphrase: Arc::new(passphrase),
        })
    }
    pub fn signature(
        &self,
        timestamp: u64,
        method: &str,
        path: &str,
        body: &[u8],
    ) -> Result<String, Error> {
        let mut h = Hmac::<sha2::Sha256>::new_from_slice(self.secret.as_slice())
            .map_err(|_| Error::Signing("invalid HMAC key"))?;
        h.update(format!("{timestamp}{method}{path}").as_bytes());
        h.update(body);
        Ok(URL_SAFE.encode(h.finalize().into_bytes()))
    }
    pub fn websocket_auth(&self) -> Value {
        json!({"apiKey":self.key,"secret":URL_SAFE.encode(self.secret.as_slice()),"passphrase":self.passphrase.as_str()})
    }
}
#[derive(Clone)]
pub struct PolymarketAuth<S = Arc<LocalEvmSigner>> {
    pub address: Address,
    pub wallet: Address,
    pub signer: Option<S>,
    pub api: Option<ApiCredentials>,
    pub order_domain: Option<OrderDomain>,
    pub nonce: u64,
    pub builder: Option<ApiCredentials>,
    pub relayer: Option<RelayerKey>,
    pub session_signer: bool,
    pub rfq_signature_type: u8,
}
impl<S> std::fmt::Debug for PolymarketAuth<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PolymarketAuth([redacted])")
    }
}
impl<S: EvmSigner> PolymarketAuth<S> {
    pub fn new(signer: S, wallet: Address) -> Self {
        let rfq_signature_type = if signer.address() == wallet { 0 } else { 3 };
        Self {
            address: signer.address(),
            wallet,
            signer: Some(signer),
            api: None,
            order_domain: None,
            nonce: 0,
            builder: None,
            relayer: None,
            session_signer: false,
            rfq_signature_type,
        }
    }
}
impl<S> PolymarketAuth<S> {
    pub fn with_rfq_signature_type(mut self, signature_type: u8) -> Result<Self, Error> {
        if signature_type > 3 || signature_type == 0 && self.address != self.wallet {
            return Err(Error::Authentication("invalid RFQ wallet signature mode"));
        }
        self.rfq_signature_type = signature_type;
        Ok(self)
    }

    pub fn with_api(mut self, api: ApiCredentials) -> Self {
        self.api = Some(api);
        self
    }
    pub fn with_order_domain(mut self, domain: OrderDomain) -> Self {
        self.order_domain = Some(domain);
        self
    }
}
impl<S: EvmSigner + Clone + 'static> Authentication for PolymarketAuth<S> {
    fn identity(&self) -> String {
        format!(
            "clob:{}:{}:{}",
            self.address,
            self.wallet,
            self.api
                .as_ref()
                .map(|c| c.key.as_str())
                .unwrap_or("wallet")
        )
    }
    async fn prepare(
        &self,
        e: &Polymarket,
        op: &Operation,
        r: &mut HttpRequest,
    ) -> Result<(), Error> {
        let timestamp = unix_seconds()?;
        let u = url::Url::parse(&r.url).map_err(|_| Error::InvalidRequest("invalid URL"))?;
        let wallet_auth =
            op.auth == "wallet" || op.auth == "wallet_or_native" && self.api.is_none();
        let nonce = if wallet_auth {
            r.headers
                .iter()
                .find(|(k, _)| k.eq_ignore_ascii_case("POLY_NONCE"))
                .map(|(_, v)| {
                    v.parse::<u64>()
                        .map_err(|_| Error::InvalidRequest("invalid wallet authentication nonce"))
                })
                .transpose()?
                .unwrap_or(self.nonce)
        } else {
            self.nonce
        };
        // Recompute owned authentication fields; never send stale duplicates from schema params.
        r.headers.retain(|(k, _)| {
            !matches!(
                k.to_ascii_uppercase().as_str(),
                "POLY_ADDRESS"
                    | "POLY_SIGNATURE"
                    | "POLY_TIMESTAMP"
                    | "POLY_NONCE"
                    | "POLY_API_KEY"
                    | "POLY_PASSPHRASE"
                    | "POLY_BUILDER_API_KEY"
                    | "POLY_BUILDER_SIGNATURE"
                    | "POLY_BUILDER_TIMESTAMP"
                    | "POLY_BUILDER_PASSPHRASE"
                    | "RELAYER_API_KEY"
                    | "RELAYER_API_KEY_ADDRESS"
            )
        });
        if wallet_auth {
            let signer = self
                .signer
                .as_ref()
                .ok_or(Error::Authentication("wallet signer required"))?;
            let signature = sign_typed_data(
                signer,
                &wallet_auth_typed_data(e.chain_id, self.address, timestamp, nonce),
            )
            .await?;
            r.headers.extend([
                ("POLY_ADDRESS".into(), self.address.to_string()),
                ("POLY_SIGNATURE".into(), signature.to_string()),
                ("POLY_TIMESTAMP".into(), timestamp.to_string()),
                ("POLY_NONCE".into(), nonce.to_string()),
            ]);
            return Ok(());
        }
        if op.service == "clob"
            && r.method == HttpMethod::Post
            && matches!(op.path.as_str(), "/order" | "/orders")
        {
            let mut b: Value = serde_json::from_slice(&r.body)?;
            let mut orders = if let Value::Array(ref mut rows) = b {
                rows.iter_mut().collect::<Vec<_>>()
            } else {
                vec![&mut b]
            };
            for row in &mut orders {
                let order = &mut row["order"];
                if order
                    .get("signature")
                    .and_then(Value::as_str)
                    .is_none_or(str::is_empty)
                {
                    let signer = self
                        .signer
                        .as_ref()
                        .ok_or(Error::Authentication("order signer required"))?;
                    let maker: Address = order["maker"]
                        .as_str()
                        .ok_or(Error::InvalidRequest("maker address required"))?
                        .parse()
                        .map_err(|_| Error::InvalidRequest("invalid maker address"))?;
                    let declared_signer: Address = order["signer"]
                        .as_str()
                        .ok_or(Error::InvalidRequest("signer address required"))?
                        .parse()
                        .map_err(|_| Error::InvalidRequest("invalid signer address"))?;
                    let expected = if integer(&order["signatureType"])? == 3 {
                        self.wallet
                    } else {
                        self.address
                    };
                    if maker != self.wallet || declared_signer != expected {
                        return Err(Error::Authentication(
                            "order identity differs from the configured wallet/signer",
                        ));
                    }
                    let d = self.order_domain.ok_or(Error::InvalidRequest(
                        "explicit CTF/V2 order signing domain required",
                    ))?;
                    let signature = sign_order(signer, order, d).await?;
                    order["signature"] = if self.session_signer {
                        if integer(&order["signatureType"])? != 3 {
                            return Err(Error::Authentication(
                                "session signer requires a Deposit Wallet order",
                            ));
                        }
                        json!(crate::wrap_session_signature(
                            self.address,
                            &alloy_primitives::hex::decode(&signature)
                                .map_err(|_| Error::Signing("invalid order signature"))?
                        )?)
                    } else {
                        json!(signature)
                    };
                }
            }
            r.body = serde_json::to_vec(&b)?;
        }
        let method = match r.method {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Delete => "DELETE",
        };
        if matches!(op.service.as_str(), "relayer" | "collateral_return") {
            if let Some(key) = &self.relayer {
                r.headers.extend(key.headers());
                return Ok(());
            }
            let builder = self.builder.as_ref().ok_or(Error::Authentication(
                "relayer API key or builder credentials required",
            ))?;
            r.headers.extend([
                ("POLY_BUILDER_API_KEY".into(), builder.key.clone()),
                (
                    "POLY_BUILDER_SIGNATURE".into(),
                    builder.signature(timestamp, method, u.path(), &r.body)?,
                ),
                ("POLY_BUILDER_TIMESTAMP".into(), timestamp.to_string()),
                (
                    "POLY_BUILDER_PASSPHRASE".into(),
                    builder.passphrase.as_str().to_owned(),
                ),
            ]);
            return Ok(());
        }
        if op.service == "builder_gateway"
            && method != "GET"
            && let Some(builder) = &self.builder
        {
            r.headers.extend([
                ("POLY_BUILDER_API_KEY".into(), builder.key.clone()),
                (
                    "POLY_BUILDER_SIGNATURE".into(),
                    builder.signature(timestamp, method, u.path(), &r.body)?,
                ),
                ("POLY_BUILDER_TIMESTAMP".into(), timestamp.to_string()),
                (
                    "POLY_BUILDER_PASSPHRASE".into(),
                    builder.passphrase.as_str().to_owned(),
                ),
            ]);
        }
        let api = self
            .api
            .as_ref()
            .ok_or(Error::Authentication("API credentials required"))?;
        let (method, prefix) = match r.method {
            HttpMethod::Get => ("GET", "POLY"),
            HttpMethod::Post => ("POST", "POLY"),
            HttpMethod::Put => ("PUT", "POLY"),
            HttpMethod::Patch => ("PATCH", "POLY"),
            HttpMethod::Delete => ("DELETE", "POLY"),
        };
        let signature = api.signature(timestamp, method, u.path(), &r.body)?;
        r.headers.extend([
            (format!("{prefix}_API_KEY"), api.key.clone()),
            (format!("{prefix}_SIGNATURE"), signature),
            (format!("{prefix}_TIMESTAMP"), timestamp.to_string()),
            (
                format!("{prefix}_PASSPHRASE"),
                api.passphrase.as_str().to_owned(),
            ),
            ("POLY_ADDRESS".into(), self.address.to_string()),
        ]);
        Ok(())
    }
}
pub fn unix_seconds() -> Result<u64, Error> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| Error::Signing("clock before Unix epoch"))?
        .as_secs())
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
#[derive(Clone)]
pub struct RelayerKey {
    key: Arc<Zeroizing<String>>,
    pub address: Address,
}
impl std::fmt::Debug for RelayerKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RelayerKey([redacted])")
    }
}
impl RelayerKey {
    pub fn new(key: Zeroizing<String>, address: Address) -> Result<Self, Error> {
        if key.is_empty() || key.contains(['\r', '\n']) {
            return Err(Error::Authentication("invalid relayer API key"));
        }
        Ok(Self {
            key: Arc::new(key),
            address,
        })
    }
    fn headers(&self) -> Vec<(String, String)> {
        vec![
            ("RELAYER_API_KEY".into(), self.key.as_str().to_owned()),
            ("RELAYER_API_KEY_ADDRESS".into(), self.address.to_string()),
        ]
    }
}
impl<S> PolymarketAuth<S> {
    pub fn with_builder_credentials(mut self, credentials: ApiCredentials) -> Self {
        self.builder = Some(credentials);
        self
    }
    pub fn with_relayer_key(mut self, key: RelayerKey) -> Self {
        self.relayer = Some(key);
        self
    }
    pub fn with_session_signer(mut self, enabled: bool) -> Self {
        self.session_signer = enabled;
        self
    }
}

//! Named callers for the fixture-backed exchange HTTP operations.
use crate::{Result, http};
use serde_json::Value;

/// Credentials for CLOB L2 requests. The secret is base64 encoded.
#[derive(Clone, serde::Serialize)]
pub struct ClobCredentials {
    pub address: String,
    pub key: String,
    pub secret: String,
    pub passphrase: String,
}

type SignCallback = std::sync::Arc<dyn Fn(&str) -> Result<Value> + Send + Sync>;

fn sign_pascal_body(
    op: &str,
    body: &Value,
    auth: &Value,
    sign: &SignCallback,
    timestamp: Option<u64>,
) -> Result<Value> {
    let mut input =
        serde_json::json!({"operation":op,"step":"prepare_external","auth":auth,"body":body});
    if let Some(ts) = timestamp {
        input["timestamp"] = ts.into();
    }
    let proof: Value = http::call(32, input)?;
    let rows = body
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or(std::slice::from_ref(body));
    let payloads = proof["messages"]
        .as_array()
        .filter(|p| p.len() == rows.len())
        .ok_or(crate::Error::Decode("invalid Pascal permit messages"))?;
    let mut signed = Vec::with_capacity(rows.len());
    for (row, payload) in rows.iter().zip(payloads) {
        let signature = sign(
            payload
                .as_str()
                .ok_or(crate::Error::Decode("invalid permit message"))?,
        )?;
        let mut identity = auth.clone();
        identity["signature"] = signature;
        signed.push(http::call(32, serde_json::json!({"operation":op,"auth":identity,"timestamp":proof["timestamp"],"body":row}))?);
    }
    if body.is_array() {
        Ok(Value::Array(signed))
    } else {
        signed
            .pop()
            .ok_or(crate::Error::Decode("missing signed permit"))
    }
}

fn apply_external_signature(
    venue: &str,
    operation: &str,
    args: &mut Value,
    sign: &SignCallback,
) -> Result<()> {
    if venue == "pascal" && operation.starts_with("write.") {
        args["params"]["body"] = sign_pascal_body(
            operation,
            &args["params"]["body"],
            &args["auth"],
            sign,
            None,
        )?;
        return Ok(());
    }
    let (code, field) = match (venue, operation) {
        ("kalshi", name) if !name.starts_with("clearing.") => (53, "payload"),
        ("arcus", "place_order" | "cancel_order") => (43, "payload"),
        ("hyperliquid", "order" | "cancel" | "cancel_by_cloid") => (45, "digest"),
        _ => return Ok(()),
    };
    let proof: Value = http::call(code, args.clone())?;
    let payload = proof[field]
        .as_str()
        .ok_or(crate::Error::Decode("missing signing payload"))?;
    args["auth"]["signature"] = sign(payload)?;
    if code == 53 {
        args["auth"]["payload"] = proof["payload"].clone();
        args["auth"]["timestamp"] = proof["timestamp"].clone();
    } else if code == 43 {
        args["auth"]["timestamp"] = proof["timestamp"].clone();
    } else {
        args["params"]["options"]["nonce"] = proof["nonce"].clone();
    }
    Ok(())
}
macro_rules! client {
    ($client:ident, $venue:literal, $($method:ident => $operation:literal,)*) => {
        #[derive(Default, Clone)]
        pub struct $client {
            transport: Option<http::Policy>,
            origin: Option<String>,
            api_key: Option<String>,
            services: Option<std::sync::Arc<ServiceCredentials>>,
            session_expiry: Option<std::sync::Arc<std::sync::atomic::AtomicU64>>,
            external_signer: Option<SignCallback>,
            venue_auth: Option<std::sync::Arc<Value>>,
            signing_chain_id: Option<u64>,
            clob: Option<std::sync::Arc<ClobCredentials>>,
            api_secret: Option<String>,
        }

        impl $client {
            pub fn new() -> Self { Self::default() }
            /// Delegate unary I/O to an application-owned HTTP/TLS client.
            pub fn with_transport(mut self, transport: impl http::Transport + 'static) -> Self {
                self.transport = Some(http::Policy::Application(std::sync::Arc::new(transport)));
                self
            }
            /// Apply the application's rustls verifier and client identity to HTTP and WebSockets.
            #[cfg(feature = "rustls")]
            pub fn with_tls(mut self, config:http::rustls::ClientConfig)->Self {
                self.transport=Some(http::Policy::tls(config)); self
            }
            pub fn with_origin(mut self, origin: impl Into<String>) -> Self {
                self.origin = Some(origin.into());
                self
            }
            pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
                self.api_key = Some(key.into());
                self
            }
            pub fn with_credentials(mut self, key:impl Into<String>, secret:impl Into<String>) -> Self {
                self.api_key = Some(key.into());
                self.api_secret = Some(secret.into());
                self
            }
            fn execute_named(&self,operation:&str,params:&Value)->Result<Value>{
                    let mut args = http::config($venue, operation, params, self.origin.as_deref(),
                                            self.api_key.as_deref(), self.api_secret.as_deref());
                    if let Some(creds)=&self.services {args["services"]=serde_json::json!(**creds);}
                    if let Some(auth)=&self.venue_auth {args["auth"]=(**auth).clone();}
                    if let Some(creds)=&self.clob {args["clob"]=serde_json::json!(**creds);}
                    if let Some(id)=self.signing_chain_id {args["chain_id"]=id.into();}
                    if $venue == "monaco" && let Some(state)=&self.session_expiry {
                        use std::sync::atomic::Ordering::SeqCst;
                        let active=if operation=="revoke_session"{state.swap(0,SeqCst)}else{state.load(SeqCst)};
                        args["auth"]["expires_at"]=active.into();
                    }
                    if let Some(sign)=&self.external_signer {apply_external_signature($venue,operation,&mut args,sign)?;}
                    let result=match &self.transport {
                        Some(transport) => transport.execute(args),
                        None => http::call(12, args),
                    }?;
                    if operation=="refresh_session" && let Some(state)=&self.session_expiry {
                        use std::sync::atomic::Ordering::SeqCst;
                        let expiry=result["expiresAt"].as_u64().ok_or(crate::Error::AuthenticationRequired)?;
                        let _=state.fetch_update(SeqCst,SeqCst,|old| (old!=0).then_some(expiry));
                    }
                    Ok(result)
                }
            $(
                pub fn $method(&self, params: &Value) -> Result<Value> {
                    self.execute_named($operation,params)
                }
            )*
        }
        #[cfg(feature = "async")]
        impl $client {
            pub fn into_async(self) -> crate::AsyncClient<Self> { crate::AsyncClient::new(self) }
        }
        #[cfg(feature = "async")]
        impl crate::AsyncClient<$client> {
            async fn execute_async(&self, name: &'static str, params: Value) -> Result<Value> {
                self.run(move |client| client.execute_named(name,&params)).await
            }
            $(
                pub fn $method(&self, params: &Value) -> impl std::future::Future<Output=Result<Value>> + '_ {
                    self.execute_async($operation,params.clone())
                }
            )*
        }
    };
}

include!("clients_generated.rs");

#[derive(Clone, serde::Serialize)]
pub struct PascalPermit {
    pub owner: String,
    pub seed: String,
    pub deployment_id: u8,
    pub recv_window_ms: u32,
}

impl PascalClient {
    pub fn with_permit(mut self, permit: PascalPermit) -> Self {
        self.external_signer = None;
        self.venue_auth = Some(std::sync::Arc::new(serde_json::json!(permit)));
        self
    }
    /// The callback signs the hex-encoded permit bytes and returns a base58 Ed25519 signature.
    pub fn with_external_permit_signer(
        mut self,
        owner: &str,
        public_key: &str,
        deployment_id: u8,
        recv_window_ms: u32,
        sign: impl Fn(&str) -> Result<String> + Send + Sync + 'static,
    ) -> Self {
        self.venue_auth = Some(std::sync::Arc::new(
            serde_json::json!({ "owner":owner,"public_key":public_key,"deployment_id":deployment_id,"recv_window_ms":recv_window_ms }),
        ));
        self.external_signer = Some(std::sync::Arc::new(move |payload| {
            sign(payload).map(Value::String)
        }));
        self
    }
    pub fn wallet_outcome_pending(&self) -> Result<bool> {
        http::call(58, serde_json::json!({"auth":self.venue_auth.as_deref()}))
    }
    pub fn reconcile_wallet(&self) -> Result<()> {
        let auth = self
            .venue_auth
            .as_ref()
            .ok_or(crate::Error::AuthenticationRequired)?;
        http::call(35, (**auth).clone())
    }
    pub fn sign_permit_at(&self, operation: &str, body: &Value, timestamp: u64) -> Result<Value> {
        let auth = self
            .venue_auth
            .as_ref()
            .ok_or(crate::Error::AuthenticationRequired)?;
        if let Some(sign) = &self.external_signer {
            return sign_pascal_body(operation, body, auth, sign, Some(timestamp));
        }
        http::call(
            32,
            serde_json::json!({"operation":operation,"body":body,"auth":**auth,"timestamp":timestamp}),
        )
    }
}
macro_rules! venue_header_credentials {
    ($client:ident) => {
        impl $client {
            pub fn with_header_credentials(mut self, api_key: Option<String>, access_token: Option<String>, expires_at: Option<u64>) -> Self {
                self.api_key = api_key;
                self.venue_auth = Some(std::sync::Arc::new(serde_json::json!({ "access_token":access_token, "expires_at":expires_at })));
                self
            }
        }
    };
}
venue_header_credentials!(ExtendedClient);
venue_header_credentials!(PhoenixClient);

impl ExtendedClient {
    fn registration<T: serde::de::DeserializeOwned>(
        &self,
        operation: &str,
        params: Value,
    ) -> Result<T> {
        http::call(
            49,
            serde_json::json!({"origin":self.origin,"operation":operation,"params":params}),
        )
    }
    pub fn onboarding_payload(&self, registration: &Value) -> Result<Value> {
        self.registration("onboarding_payload", registration.clone())
    }
    pub fn key_derivation_payload(&self, registration: &Value) -> Result<Value> {
        self.registration("key_derivation_payload", registration.clone())
    }
    pub fn api_request_payload(&self, path: &str, timestamp: &str) -> Result<Value> {
        self.registration(
            "api_request_payload",
            serde_json::json!({"path":path,"timestamp":timestamp}),
        )
    }
    pub fn onboarding_l2_hash(&self, wallet: &str, public_key: &str) -> Result<String> {
        self.registration(
            "onboarding_l2_hash",
            serde_json::json!({"wallet":wallet,"public_key":public_key}),
        )
    }
    pub fn scale_decimal(&self, value: &str, decimals: u8) -> Result<i64> {
        self.registration(
            "scale_decimal",
            serde_json::json!({"value":value,"decimals":decimals}),
        )
    }
}
pub struct ExtendedSigner {
    signature: String,
    public_key: String,
}
impl ExtendedSigner {
    pub fn from_wallet_signature(signature: String) -> Result<Self> {
        let public_key = http::call(
            49,
            serde_json::json!({"operation":"derived_public_key","params":{"signature":signature}}),
        )?;
        Ok(Self {
            signature,
            public_key,
        })
    }
    pub fn public_key(&self) -> &str {
        &self.public_key
    }
    pub fn sign_digest(&self, digest: &str) -> Result<[String; 2]> {
        http::call(
            49,
            serde_json::json!({"operation":"derived_sign","params":{"signature":self.signature,"digest":digest}}),
        )
    }
}
impl ParadexClient {
    fn authentication<T: serde::de::DeserializeOwned>(&self, op: &str, params: Value) -> Result<T> {
        http::call(
            50,
            serde_json::json!({"operation":op,"origin":self.origin,"auth":self.venue_auth.as_deref(),"params":params}),
        )
    }
    pub fn block_signature(
        &self,
        nonce: &str,
        expiration: u64,
        trades: &Value,
        parent: Option<&str>,
    ) -> Result<Value> {
        self.authentication("block_signature", serde_json::json!({"nonce":nonce,"expiration":expiration,"trades":trades,"parent":parent}))
    }
    pub fn onboarding_typed_data(&self) -> Result<Value> {
        self.authentication("onboarding_typed_data", serde_json::json!({}))
    }
    pub fn evm_account_address(&self, address: &str, class_hash: &str) -> Result<String> {
        self.authentication(
            "evm_account_address",
            serde_json::json!({"address":address,"class_hash":class_hash}),
        )
    }
    pub fn sign_siwe(
        &self,
        statement: &str,
        nonce: &str,
        issued_at: &str,
        expiration: Option<&str>,
    ) -> Result<Value> {
        self.authentication("sign_siwe",serde_json::json!({"statement":statement,"nonce":nonce,"issued_at":issued_at,"expiration":expiration}))
    }
    pub fn set_token(&mut self, token: &str, read_only: bool) -> Result<()> {
        let token: String =
            self.authentication("validate_token", serde_json::json!({"token":token}))?;
        let auth = self.auth_mut();
        auth["token"] = token.into();
        auth["read_only"] = read_only.into();
        Ok(())
    }
    pub fn authenticate(&mut self) -> Result<()> {
        let req: Value = self.authentication("prepare_auth", serde_json::json!({}))?;
        let response = match req["operation"].as_str() {
            Some("get_jwt") => self.get_jwt(&req["params"]),
            Some("get_subkey_jwt") => self.get_subkey_jwt(&req["params"]),
            Some("evm_auth") => self.evm_auth(&req["params"]),
            _ => Err(crate::Error::AuthenticationRequired),
        }?;
        self.set_token(
            response["jwt_token"]
                .as_str()
                .ok_or(crate::Error::AuthenticationRequired)?,
            false,
        )
    }
    pub fn with_subkey(mut self, public_key: impl Into<String>) -> Self {
        self.auth_mut()["subkey"] = public_key.into().into();
        self
    }
    pub fn with_siwe_domain(mut self, domain: &str, uri: &str, chain_id: u64) -> Self {
        let auth = self.auth_mut();
        auth["siwe_domain"] = domain.into();
        auth["siwe_uri"] = uri.into();
        auth["evm_chain_id"] = chain_id.into();
        self
    }
    fn auth_mut(&mut self) -> &mut Value {
        std::sync::Arc::make_mut(
            self.venue_auth
                .get_or_insert_with(|| std::sync::Arc::new(serde_json::json!({}))),
        )
    }
    pub fn with_account_bearer(
        mut self,
        account: impl Into<String>,
        token: impl Into<String>,
        read_only: bool,
    ) -> Self {
        self.auth_mut()["account"] = account.into().into();
        self.with_bearer_token(token, read_only)
    }
    pub fn with_bearer_token(mut self, token: impl Into<String>, read_only: bool) -> Self {
        let auth = self.auth_mut();
        auth["token"] = token.into().into();
        auth["read_only"] = read_only.into();
        self
    }
    pub fn with_signer(
        mut self,
        account: impl Into<String>,
        network: impl Into<String>,
        signer: ParadexSigner,
    ) -> Self {
        let auth = self.auth_mut();
        let map = auth
            .as_object_mut()
            .expect("client auth configuration is an object");
        map.remove("stark_key");
        map.remove("evm_key");
        match signer {
            ParadexSigner::Stark(key) => auth["stark_key"] = key.into(),
            ParadexSigner::Evm(key) => auth["evm_key"] = key.into(),
        }
        auth["account"] = account.into().into();
        auth["network"] = network.into().into();
        self
    }
    pub fn sign_typed_data(&self, typed_data: &Value) -> Result<Value> {
        let auth = self
            .venue_auth
            .as_ref()
            .ok_or(crate::Error::AuthenticationRequired)?;
        http::call(
            36,
            serde_json::json!({"auth":**auth,"typed_data":typed_data}),
        )
    }
}
pub enum ParadexSigner {
    Stark(String),
    Evm(String),
}
#[derive(Clone, Default, serde::Serialize)]
pub struct PerpsAuth {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdrawal_contract: Option<String>,
}

impl PolymarketPerpsClient {
    pub fn with_perps_auth(mut self, auth: PerpsAuth) -> Self {
        self.venue_auth = Some(std::sync::Arc::new(serde_json::json!(auth)));
        self
    }
}

#[derive(Clone, serde::Serialize)]
pub struct OrderDomain {
    pub chain_id: u64,
    pub version: u8,
    pub exchange: String,
}

#[derive(Clone, serde::Serialize)]
pub struct ClobWallet {
    pub key: String,
    /// Funding wallet. Defaults to the address derived from the key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    pub nonce: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_domain: Option<OrderDomain>,
    pub session_signer: bool,
}

#[derive(Clone, serde::Serialize)]
pub struct RelayerKey {
    pub key: String,
    pub address: String,
}
#[derive(Clone, serde::Serialize)]
pub struct BuilderCredentials {
    pub key: String,
    pub secret: String,
    pub passphrase: String,
}
#[derive(Clone, Default, serde::Serialize)]
pub struct ServiceCredentials {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relayer: Option<RelayerKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder: Option<BuilderCredentials>,
}
impl PolymarketClient {
    pub fn with_service_credentials(mut self, credentials: ServiceCredentials) -> Self {
        self.services = Some(std::sync::Arc::new(credentials));
        self
    }
    pub fn with_clob_wallet(mut self, wallet: ClobWallet) -> Self {
        self.venue_auth = Some(std::sync::Arc::new(serde_json::json!(wallet)));
        self
    }

    pub fn with_clob_credentials(mut self, creds: ClobCredentials) -> Self {
        self.clob = Some(std::sync::Arc::new(creds));
        self
    }
}
impl DeriveClient {
    fn offline_action<T: serde::de::DeserializeOwned>(&self, mut action: Value) -> Result<T> {
        action["api_key"] = serde_json::json!(self.api_key);
        action["api_secret"] = serde_json::json!(self.api_secret);
        if let Some(origin) = &self.origin {
            action["origin"] = origin.clone().into();
        }
        if let Some(chain_id) = self.signing_chain_id {
            action["chain_id"] = chain_id.into();
        }
        http::call(57, action)
    }
    pub fn sign_action(&self, action: &crate::derive::ActionPayload) -> Result<String> {
        self.offline_action(serde_json::json!({"action":action}))
    }
    pub fn prepare_signed(
        &self,
        request: &Value,
        module_data: &crate::derive::ActionData,
    ) -> Result<Value> {
        self.offline_action(serde_json::json!({"request":request,"module_data":module_data}))
    }
    /// Select the EIP-712 domain when sending writes through a custom origin.
    pub fn with_signing_chain_id(mut self, chain_id: u64) -> Self {
        self.signing_chain_id = Some(chain_id);
        self
    }
}
impl PythHermesClient {
    pub fn stream_price_updates(&self, params: &Value) -> Result<crate::PriceStream> {
        let cfg = http::config(
            "pyth_hermes",
            "stream_price_updates",
            params,
            self.origin.as_deref(),
            self.api_key.as_deref(),
            None,
        );
        crate::PriceStream::open_with_transport(
            &cfg.to_string(),
            self.transport.as_ref().map(http::Policy::connection),
        )
    }
}

impl ChainlinkCandlestickClient {
    pub fn stream_prices(&self, params: &Value) -> Result<crate::EventStream> {
        let mut config = serde_json::json!({"venue":"chainlink_candles","operation":"stream_prices","params":params});
        if let Some(origin) = &self.origin {
            config["origin"] = origin.as_str().into();
        }
        if let Some(key) = &self.api_key {
            config["api_key"] = key.as_str().into();
        }
        crate::EventStream::open_with_transport(
            &config.to_string(),
            self.transport.as_ref().map(http::Policy::connection),
        )
    }
}

#[cfg(feature = "async")]
macro_rules! async_event_stream {
    ($client:ident, $method:ident) => {
        impl crate::AsyncClient<$client> {
            pub async fn $method(&self, params: &Value) -> Result<crate::AsyncEventStream> {
                let params = params.clone();
                self.run(move |client| client.$method(&params).map(crate::EventStream::into_async))
                    .await
            }
        }
    };
}
#[cfg(feature = "async")]
async_event_stream!(PythHermesClient, stream_price_updates);
#[cfg(feature = "async")]
async_event_stream!(ChainlinkCandlestickClient, stream_prices);

macro_rules! data_stream {
    ($client:ident, $venue:literal, $($name:ident),+) => {
        impl $client {
            $(pub fn $name(&self, params: &Value) -> Result<crate::databento::DataStream> {
                let mut config=serde_json::json!({"venue":$venue,"operation":stringify!($name),"params":params});
                if let Some(url)=&self.origin {config["origin"]=url.as_str().into();}
                if let Some(key)=&self.api_key {config["api_key"]=key.as_str().into();}
                Ok(crate::databento::DataStream(crate::EventStream::open_with_transport(&config.to_string(),self.transport.as_ref().map(http::Policy::connection))?))
            })+
        }
    }
}
impl HistoricalClient {
    pub fn timeseries(&self, params: &Value) -> Result<crate::databento::ByteStream> {
        self.open_bytes("timeseries", params)
    }
    pub fn download(&self, url: &str) -> Result<crate::databento::ByteStream> {
        self.open_bytes("download", &serde_json::json!({"url":url}))
    }
    pub fn download_to(&self, url: &str, path: impl AsRef<std::path::Path>) -> Result<u64> {
        let path = path
            .as_ref()
            .to_str()
            .ok_or(crate::Error::InvalidRequest("path requires UTF-8"))?;
        let raw = self
            .download(url)?
            .0
            .command_json(&serde_json::json!({"operation":"save","path":path}).to_string())?;
        serde_json::from_str(&raw).map_err(|_| crate::Error::Decode("invalid file size"))
    }
    fn open_bytes(&self, operation: &str, params: &Value) -> Result<crate::databento::ByteStream> {
        let mut config =
            serde_json::json!({"venue":"databento","operation":operation,"params":params});
        if let Some(url) = &self.origin {
            config["origin"] = url.as_str().into();
        }
        if let Some(key) = &self.api_key {
            config["api_key"] = key.as_str().into();
        }
        Ok(crate::databento::ByteStream(
            crate::EventStream::open_with_transport(
                &config.to_string(),
                self.transport.as_ref().map(http::Policy::connection),
            )?,
        ))
    }
}
data_stream!(HistoricalClient, "databento", get_range);
data_stream!(
    ReferenceClient,
    "databento",
    security_master_get_range,
    security_master_get_last,
    corporate_actions_get_range,
    adjustment_factors_get_range
);

fn is_native_exchange(venue: &str) -> bool {
    matches!(venue, "aevo" | "apex_omni" | "backpack" | "binance_spot" | "bybit" | "deribit" | "dydx_v4" | "hibachi" | "kraken_spot" | "kuru" | "okx" | "orderly" | "reya")
}

macro_rules! socket_subscription {
    ($client:ident, $venue:literal) => {
        impl $client {
            /// Open a subscription on the default socket or a caller-selected endpoint.
            pub fn subscribe(&self, params: &Value, endpoint: Option<&str>) -> Result<crate::EventStream> {
                let mut cfg=serde_json::json!({"venue":$venue,"operation":"subscribe","params":params});
                if let Some(url)=endpoint.or(self.origin.as_deref().filter(|url| !is_native_exchange($venue) || url.starts_with("ws"))) {cfg["origin"]=url.into();}
                if let Some(key)=self.api_key.as_deref() {cfg["api_key"]=key.into();}
                if let Some(secret)=self.api_secret.as_deref() {cfg["api_secret"]=secret.into();}
                if let Some(auth)=&self.venue_auth {cfg["auth"]=(**auth).clone();}
                if let Some(state)=&self.session_expiry {cfg["auth"]["expires_at"]=state.load(std::sync::atomic::Ordering::SeqCst).into();}
                if let Some(creds)=&self.clob {cfg["clob"]=serde_json::json!(**creds);}
                crate::EventStream::open_with_transport(&cfg.to_string(),self.transport.as_ref().map(http::Policy::connection))
            }
        }
        #[cfg(feature = "async")]
        impl crate::AsyncClient<$client> {
            pub async fn subscribe(&self, params: &Value, endpoint: Option<&str>) -> Result<crate::AsyncEventStream> {
                let params = params.clone();
                let endpoint = endpoint.map(str::to_owned);
                self.run(move |client| client.subscribe(&params, endpoint.as_deref()).map(crate::EventStream::into_async)).await
            }
        }
    }
}
socket_subscription!(AevoClient, "aevo");
socket_subscription!(ApexOmniClient, "apex_omni");
socket_subscription!(BackpackClient, "backpack");
socket_subscription!(BinanceSpotClient, "binance_spot");
socket_subscription!(BybitClient, "bybit");
socket_subscription!(DeribitClient, "deribit");
socket_subscription!(DydxV4Client, "dydx_v4");
socket_subscription!(HibachiClient, "hibachi");
socket_subscription!(KrakenSpotClient, "kraken_spot");
socket_subscription!(KuruClient, "kuru");
socket_subscription!(OkxClient, "okx");
socket_subscription!(OrderlyClient, "orderly");
socket_subscription!(ReyaClient, "reya");

struct RpcPermit {
    auth: std::sync::Arc<Value>,
    sign: SignCallback,
}

macro_rules! rpc_session {
    ($session:ident, $($method:ident),+) => {
        pub struct $session(crate::EventStream, Option<Box<RpcPermit>>);
        impl $session {
            fn send(&self, operation: &str, params: &Value) -> Result<Value> {
                let signed;
                let params = if let Some(permit) = &self.1 && matches!(operation, "place_orders" | "cancel_orders" | "mass_cancel") {
                    signed = sign_pascal_body(operation, params, &permit.auth, &permit.sign, None)?;
                    &signed
                } else { params };
                let input = serde_json::json!({"operation":operation,"params":params});
                let reply = self.0.command_json(&input.to_string())?;
                serde_json::from_str(&reply).map_err(|_| crate::Error::Decode("invalid command response"))
            }
            pub fn pending(&self) -> Result<Option<Value>> {
                let reply = self.send("pending", &Value::Null)?;
                Ok((!reply.is_null()).then_some(reply))
            }
            pub fn reconcile(&self) -> Result<Value> { self.send("reconcile", &Value::Null) }
            pub fn next_event(&self) -> Result<Option<Value>> { self.0.next_event() }
            pub fn close(&self) { self.0.close() }
            #[cfg(feature = "async")]
            pub fn into_async(self) -> crate::AsyncSession<Self> {
                let events = self.0.clone();
                crate::AsyncSession::new(self, events)
            }
            $(pub fn $method(&self, params: &Value) -> Result<Value> {
                self.send(stringify!($method), params)
            })+
        }
        #[cfg(feature = "async")]
        impl crate::AsyncSession<$session> {
            pub async fn pending(&self) -> Result<Option<Value>> {
                self.commands.run($session::pending).await
            }
            pub async fn reconcile(&self) -> Result<Value> {
                self.commands.run($session::reconcile).await
            }
            $(pub async fn $method(&self, params: &Value) -> Result<Value> {
                let params = params.clone();
                self.commands.run(move |session| session.$method(&params)).await
            })+
        }
    };
}
include!("rpc_generated.rs");

rpc_session!(
    KalshiRpcSession,
    subscribe,
    unsubscribe,
    update_subscription,
    list_subscriptions
);
rpc_session!(
    ParadexRpcSession,
    subscribe_channel,
    unsubscribe_channel,
    modify_subscription,
    create_order
);
impl ParadexRpcSession {
    pub fn create_batch(&self, requests: &[Value]) -> Result<Value> {
        self.send("create_batch", &serde_json::json!(requests))
    }
    pub fn modify_order(&self, id: &str, request: &Value) -> Result<Value> {
        self.send("modify_order", &serde_json::json!({"id":id,"body":request}))
    }
    pub fn cancel_order(&self, id: &str) -> Result<Value> {
        self.send("cancel_order", &serde_json::json!({"id":id}))
    }
    pub fn cancel_by_client_id(&self, client_id: &str, market: &str) -> Result<Value> {
        self.send(
            "cancel_by_client_id",
            &serde_json::json!({"client_id":client_id,"market":market}),
        )
    }
    pub fn cancel_batch(&self, ids: &[String]) -> Result<Value> {
        self.send("cancel_batch", &serde_json::json!({"order_ids":ids}))
    }
    pub fn cancel_all(&self, market: Option<&str>) -> Result<Value> {
        self.send(
            "cancel_all",
            &market
                .map(|m| serde_json::json!({"market":m}))
                .unwrap_or(serde_json::json!({})),
        )
    }
    pub fn set_cancel_on_disconnect(&self, enabled: bool) -> Result<Value> {
        self.send(
            "set_cancel_on_disconnect",
            &serde_json::json!({"enabled":enabled}),
        )
    }
}
rpc_session!(PascalRpcSession, place_orders, cancel_orders, mass_cancel);
impl PascalRpcSession {
    pub fn ping(&self) -> Result<Value> {
        self.send("ping", &Value::Null)
    }
}
macro_rules! open_rpc {
    ($client:ident, $session:ident, $venue:literal) => {
        impl $client {
            pub fn open_rpc(&self, endpoint: Option<&str>, options: &Value) -> Result<$session> {
                let mut cfg = serde_json::json!({"venue":$venue, "operation":"open_rpc", "params":options});
                if let Some(url) = endpoint.or(self.origin.as_deref().filter(|url| !is_native_exchange($venue) || url.starts_with("ws"))) {cfg["origin"]=url.into();}
                if let Some(key) = self.api_key.as_deref() {cfg["api_key"]=key.into();}
                if let Some(secret) = self.api_secret.as_deref() {cfg["api_secret"]=secret.into();}
                if let Some(auth) = &self.venue_auth {cfg["auth"]=(**auth).clone();}
                if let Some(clob) = &self.clob {cfg["clob"]=serde_json::json!(**clob);}
                if let Some(chain) = self.signing_chain_id {cfg["params"]["chain_id"]=chain.into();}
                let permit = if $venue == "pascal" {
                    self.venue_auth.as_ref().zip(self.external_signer.as_ref())
                        .map(|(auth,sign)| Box::new(RpcPermit{auth:auth.clone(),sign:sign.clone()}))
                } else { None };
                crate::EventStream::open_with_transport(&cfg.to_string(),self.transport.as_ref().map(http::Policy::connection)).map(|stream| $session(stream, permit))
            }
        }
        #[cfg(feature = "async")]
        impl crate::AsyncClient<$client> {
            pub async fn open_rpc(&self, endpoint: Option<&str>, options: &Value) -> Result<crate::AsyncSession<$session>> {
                let endpoint = endpoint.map(str::to_owned);
                let options = options.clone();
                self.run(move |client| client.open_rpc(endpoint.as_deref(), &options).map($session::into_async)).await
            }
        }
    };
}
macro_rules! kalshi_request_signer {
    ($client:ident) => {
        impl $client {
            /// Delegate unary request signing to an Ed25519 or RSA-PSS signer.
            pub fn with_request_signer(
                mut self,
                key_id: impl Into<String>,
                sign: impl Fn(&[u8]) -> Result<Vec<u8>> + Send + Sync + 'static,
            ) -> Self {
                self.api_key = Some(key_id.into());
                self.api_secret = None;
                self.external_signer = Some(std::sync::Arc::new(move |message| {
                    use std::fmt::Write;
                    let bytes = sign(message.as_bytes())?;
                    let mut hex = String::with_capacity(bytes.len() * 2);
                    for byte in bytes {
                        write!(hex, "{byte:02x}").unwrap();
                    }
                    Ok(Value::String(hex))
                }));
                self.venue_auth = Some(std::sync::Arc::new(serde_json::json!({})));
                self
            }
        }
    };
}
kalshi_request_signer!(KalshiClient);
kalshi_request_signer!(KalshiPerpsClient);
open_rpc!(AevoClient, AevoRpcSession, "aevo");
open_rpc!(BinanceSpotClient, BinanceSpotRpcSession, "binance_spot");
open_rpc!(BybitClient, BybitRpcSession, "bybit");
open_rpc!(DeribitClient, DeribitRpcSession, "deribit");
open_rpc!(HibachiClient, HibachiRpcSession, "hibachi");
open_rpc!(KrakenSpotClient, KrakenSpotRpcSession, "kraken_spot");
open_rpc!(OkxClient, OkxRpcSession, "okx");
open_rpc!(KalshiClient, KalshiRpcSession, "kalshi");
open_rpc!(KalshiPerpsClient, KalshiRpcSession, "kalshi_perps");
open_rpc!(ParadexClient, ParadexRpcSession, "paradex");
open_rpc!(DeriveClient, DeriveRpcSession, "derive");
open_rpc!(PolymarketClient, PolymarketRfqRpcSession, "polymarket");
open_rpc!(
    PolymarketPerpsClient,
    PolymarketPerpsRpcSession,
    "polymarket_perps"
);
impl DeriveClient {
    pub fn open_public_rpc(
        &self,
        endpoint: Option<&str>,
        options: &Value,
    ) -> Result<DeriveRpcSession> {
        let mut public = self.clone();
        public.api_key = None;
        public.api_secret = None;
        public.open_rpc(endpoint, options)
    }
}
open_rpc!(PascalClient, PascalRpcSession, "pascal");

pub struct HyperdashStream(crate::EventStream);
impl HyperdashStream {
    fn command(&self, params: Value) -> Result<()> {
        self.0.command_json(&params.to_string()).map(drop)
    }
    pub fn subscribe(&self, subscription: &Value) -> Result<()> {
        self.command(serde_json::json!({"operation":"subscribe","subscription":subscription}))
    }
    pub fn ack(&self, sub: &Value, batch: &Value) -> Result<()> {
        self.command(serde_json::json!({"operation":"ack","sub":sub,"batch":batch}))
    }
    pub fn unsubscribe(&self, sub: &Value) -> Result<()> {
        self.command(serde_json::json!({"operation":"unsubscribe","sub":sub}))
    }
    pub fn next_event(&self) -> Result<Option<Value>> {
        self.0.next_event()
    }
    pub fn close(&self) {
        self.0.close()
    }
}
impl HyperdashClient {
    pub fn open_stream(&self, endpoint: Option<&str>, options: &Value) -> Result<HyperdashStream> {
        let mut cfg =
            serde_json::json!({"venue":"hyperdash","operation":"open_stream","params":options});
        if let Some(url) = endpoint {
            cfg["origin"] = url.into();
        }
        if let Some(key) = &self.api_key {
            cfg["api_key"] = key.as_str().into();
        }
        crate::EventStream::open_with_transport(
            &cfg.to_string(),
            self.transport.as_ref().map(http::Policy::connection),
        )
        .map(HyperdashStream)
    }
}
impl ArcusClient {
    pub fn with_external_signer(
        mut self,
        address: &str,
        account_index: u8,
        public_key: &str,
        sign: impl Fn(&str) -> Result<String> + Send + Sync + 'static,
    ) -> Self {
        self.venue_auth = Some(std::sync::Arc::new(
            serde_json::json!({"address":address,"account_index":account_index,"public_key":public_key}),
        ));
        self.external_signer = Some(std::sync::Arc::new(move |payload| {
            sign(payload).map(Value::String)
        }));
        self
    }

    pub fn with_signer(mut self, address: &str, account_index: u8, seed: &str) -> Self {
        self.external_signer = None;
        self.venue_auth = Some(std::sync::Arc::new(
            serde_json::json!({"address":address,"account_index":account_index,"seed":seed}),
        ));
        self
    }
    pub fn signing_payload(
        address: &str,
        account_index: u8,
        operation: &str,
        params: &Value,
    ) -> Result<Value> {
        http::call(
            43,
            serde_json::json!({"operation":operation,"params":params,"auth":{"address":address,"account_index":account_index}}),
        )
    }
    pub fn with_order_signature(
        mut self,
        address: &str,
        account_index: u8,
        timestamp: u64,
        public_key: &str,
        signature: &str,
    ) -> Self {
        self.external_signer = None;
        self.venue_auth = Some(std::sync::Arc::new(
            serde_json::json!({"address":address,"account_index":account_index,"timestamp":timestamp,"public_key":public_key,"signature":signature}),
        ));
        self
    }
}
socket_subscription!(ArcusClient, "arcus");
socket_subscription!(ChainlinkDataStreamsClient, "chainlink_streams");

impl ChainlinkDataStreamsClient {
    pub fn subscribe_reports(
        &self,
        params: &Value,
        endpoint: Option<&str>,
    ) -> Result<crate::EventStream> {
        self.subscribe(params, endpoint)
    }
}

socket_subscription!(ParadexClient, "paradex");
socket_subscription!(DeriveClient, "derive");
socket_subscription!(ExtendedClient, "extended");
socket_subscription!(PolymarketClient, "polymarket");
socket_subscription!(PolymarketPerpsClient, "polymarket_perps");
socket_subscription!(MonacoClient, "monaco");
socket_subscription!(HyperliquidClient, "hyperliquid");

macro_rules! perps_client {
    ($client:ident, $venue:literal) => {
        socket_subscription!($client, $venue);
        impl $client {
            /// Set venue-specific credentials and network options.
            pub fn with_auth(mut self, auth: Value) -> Self {
                self.venue_auth = Some(std::sync::Arc::new(auth));
                self
            }
        }
    };
}
perps_client!(PerplClient, "perpl");
perps_client!(RisexClient, "risex");
perps_client!(DecibelClient, "decibel");
perps_client!(EdgeXClient, "edgex");
perps_client!(GrvtClient, "grvt");
perps_client!(KatanaClient, "katana");
perps_client!(AsterClient, "aster");
perps_client!(InjectiveClient, "injective");
perps_client!(NadoClient, "nado");
perps_client!(OndoPerpsClient, "ondo_perps");
perps_client!(PacificaClient, "pacifica");
perps_client!(ParagonClient, "paragon");
perps_client!(TradeXyzClient, "trade_xyz");
socket_subscription!(LighterClient, "lighter");
impl LighterClient {
    pub fn subscribe_account_orders(&self, endpoint: Option<&str>) -> Result<crate::EventStream> {
        self.subscribe(
            &serde_json::json!({"channel":"account_all_orders"}),
            endpoint,
        )
    }
}
socket_subscription!(QfexClient, "qfex");
socket_subscription!(PhoenixClient, "phoenix");

socket_subscription!(PascalClient, "pascal");
socket_subscription!(KalshiClient, "kalshi");
socket_subscription!(KalshiPerpsClient, "kalshi");

impl HyperliquidClient {
    pub fn with_external_signer(
        mut self,
        address: &str,
        network: &str,
        vault_address: Option<&str>,
        sign: impl Fn(&str) -> Result<Value> + Send + Sync + 'static,
    ) -> Self {
        self.venue_auth = Some(std::sync::Arc::new(
            serde_json::json!({"address":address,"network":network,"vault_address":vault_address}),
        ));
        self.external_signer = Some(std::sync::Arc::new(sign));
        self
    }

    pub fn user_signing_payload(
        action: &Value,
        network: &str,
        signature_chain_id: u64,
    ) -> Result<String> {
        http::call(
            41,
            serde_json::json!({"action":action,"network":network,"signature_chain_id":signature_chain_id}),
        )
    }
    pub fn sign_user_action(&self, action: &Value, signature_chain_id: u64) -> Result<Value> {
        let auth = self
            .venue_auth
            .as_ref()
            .ok_or(crate::Error::AuthenticationRequired)?;
        let mut cfg = serde_json::json!({"action":action,"network":auth["network"],"signature_chain_id":signature_chain_id});
        let mut identity = (**auth).clone();
        if let Some(sign) = &self.external_signer {
            let digest: String = http::call(41, cfg.clone())?;
            identity["signature"] = sign(&digest)?;
        }
        cfg["auth"] = identity;
        http::call(41, cfg)
    }
    pub fn with_signer(mut self, key: &str, network: &str, vault_address: Option<&str>) -> Self {
        self.external_signer = None;
        self.venue_auth = Some(std::sync::Arc::new(
            serde_json::json!({"key":key,"network":network,"vault_address":vault_address}),
        ));
        self
    }
}

impl MonacoClient {
    /// Bind an imported session to the private WebSocket endpoint of a custom deployment.
    pub fn with_session_ws_origin(mut self, endpoint: &str) -> Self {
        if let Some(auth) = &mut self.venue_auth {
            std::sync::Arc::make_mut(auth)["ws_origin"] = endpoint.into();
        }
        self
    }
    /// Register an Ed25519 session. The callback signs the validated challenge with the wallet.
    pub fn login(
        &self,
        options: &Value,
        sign: impl FnOnce(&str) -> Result<String>,
    ) -> Result<Self> {
        let origin = self.origin.as_deref().unwrap_or("https://apimonaco.xyz");
        let mut context = serde_json::json!({"origin":origin,"options":options,"step":"challenge"});
        context["challenge"] = self.create_challenge(&http::call(42, context.clone())?)?;
        context["step"] = "message".into();
        let message: String = http::call(42, context.clone())?;
        context["signature"] = sign(&message)?.into();
        context["step"] = "verify".into();
        context["response"] = self.verify_signature(&http::call(42, context.clone())?)?;
        context["step"] = "finish".into();
        let session: Value = http::call(42, context)?;
        let client = self.clone().with_session(
            origin,
            session["seed"]
                .as_str()
                .ok_or(crate::Error::Decode("missing session key"))?,
            session["expires_at"]
                .as_u64()
                .ok_or(crate::Error::AuthenticationRequired)?,
            session["signing"]
                .as_str()
                .ok_or(crate::Error::Decode("missing signing mode"))?,
        );
        Ok(if let Some(ws) = session["ws_origin"].as_str() {
            client.with_session_ws_origin(ws)
        } else {
            client
        })
    }
    /// Bind a registered Ed25519 session to its API origin until its Unix expiry.
    pub fn with_session(
        mut self,
        origin: &str,
        seed: &str,
        expires_at: u64,
        signing: &str,
    ) -> Self {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        let id = NEXT
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            .to_string();
        self.origin = Some(origin.to_owned());
        self.session_expiry = Some(std::sync::Arc::new(std::sync::atomic::AtomicU64::new(
            expires_at,
        )));
        self.venue_auth = Some(std::sync::Arc::new(
            serde_json::json!({"origin":origin,"seed":seed,"expires_at":expires_at,"signing":signing,"instance":id}),
        ));
        self
    }
}

pub struct QfexTradingSession(crate::EventStream);

impl QfexTradingSession {
    fn command(&self, op: &str, params: &Value) -> Result<Value> {
        let raw = self
            .0
            .command_json(&serde_json::json!({"operation":op,"params":params}).to_string())?;
        serde_json::from_str(&raw).map_err(|_| crate::Error::Decode("invalid QFEX response"))
    }
    pub fn subscribe_account(&self) -> Result<()> {
        self.command("subscribe_account", &Value::Null).map(|_| ())
    }
    pub fn add_order(&self, order: &Value) -> Result<Value> {
        self.command("add_order", order)
    }
    pub fn cancel_order(&self, order: &Value) -> Result<Value> {
        self.command("cancel_order", order)
    }
    pub fn user_orders(&self, request: &Value) -> Result<Value> {
        self.command("user_orders", request)
    }
    pub fn next_event(&self) -> Result<Option<Value>> {
        self.0.next_event()
    }
    pub fn close(&self) {
        self.0.close();
    }
}
impl QfexClient {
    pub fn trading_session(
        &self,
        options: &Value,
        endpoint: Option<&str>,
    ) -> Result<QfexTradingSession> {
        let mut cfg = serde_json::json!({"venue":"qfex","operation":"trading_session","params":options,
            "api_key":self.api_key.as_ref().ok_or(crate::Error::AuthenticationRequired)?,
            "api_secret":self.api_secret.as_ref().ok_or(crate::Error::AuthenticationRequired)?});
        if let Some(url) = endpoint.or(self.origin.as_deref()) {
            cfg["origin"] = url.into();
        }
        crate::EventStream::open_with_transport(
            &cfg.to_string(),
            self.transport.as_ref().map(http::Policy::connection),
        )
        .map(QfexTradingSession)
    }
    pub fn add_order(&self, order: &Value) -> Result<Value> {
        self.trading_session(&serde_json::json!({}), None)?
            .add_order(order)
    }
    pub fn cancel_order(&self, order: &Value) -> Result<Value> {
        self.trading_session(&serde_json::json!({}), None)?
            .cancel_order(order)
    }
    pub fn user_orders(&self, request: &Value) -> Result<Value> {
        self.trading_session(&serde_json::json!({}), None)?
            .user_orders(request)
    }
}

data_stream!(TardisClient, "tardis", data_feeds, replay);

impl TardisClient {
    pub fn dataset(&self, params: &Value) -> Result<crate::databento::ByteStream> {
        let cfg = http::config(
            "tardis",
            "dataset",
            params,
            self.origin.as_deref(),
            self.api_key.as_deref(),
            None,
        );
        Ok(crate::databento::ByteStream(
            crate::EventStream::open_with_transport(
                &cfg.to_string(),
                self.transport.as_ref().map(http::Policy::connection),
            )?,
        ))
    }
    pub fn download_dataset(
        &self,
        params: &Value,
        path: impl AsRef<std::path::Path>,
    ) -> Result<u64> {
        let path = path
            .as_ref()
            .to_str()
            .ok_or(crate::Error::InvalidRequest("path requires UTF-8"))?;
        let result = self
            .dataset(params)?
            .0
            .command_json(&serde_json::json!({"operation":"save","path":path}).to_string())?;
        serde_json::from_str(&result).map_err(|_| crate::Error::Decode("invalid byte count"))
    }
}
pub struct TardisMachineStream(crate::databento::DataStream);
impl std::ops::Deref for TardisMachineStream {
    type Target = crate::databento::DataStream;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl TardisMachineStream {
    pub fn send(&self, message: &Value) -> Result<()> {
        self.0
            .0
            .command_json(&serde_json::json!({"operation":"send","message":message}).to_string())
            .map(|_| ())
    }
}
#[derive(Default, Clone)]
pub struct TardisMachineClient {
    transport: Option<http::Policy>,
    origin: Option<String>,
}
impl TardisMachineClient {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn with_transport(mut self, transport: impl http::Transport + 'static) -> Self {
        self.transport = Some(http::Policy::Application(std::sync::Arc::new(transport)));
        self
    }
    #[cfg(feature = "rustls")]
    pub fn with_tls(mut self, config: http::rustls::ClientConfig) -> Self {
        self.transport = Some(http::Policy::tls(config));
        self
    }
    fn open_machine(&self, op: &str, params: &Value) -> Result<crate::databento::DataStream> {
        let cfg = http::config("tardis", op, params, self.origin.as_deref(), None, None);
        Ok(crate::databento::DataStream(
            crate::EventStream::open_with_transport(
                &cfg.to_string(),
                self.transport.as_ref().map(http::Policy::connection),
            )?,
        ))
    }
    pub fn replay(&self, options: &Value) -> Result<crate::databento::DataStream> {
        self.open_machine("machine_replay", &serde_json::json!({"options":options}))
    }
    pub fn replay_normalized(&self, options: &Value) -> Result<crate::databento::DataStream> {
        self.open_machine(
            "machine_replay_normalized",
            &serde_json::json!({"options":options}),
        )
    }
    pub fn ws_replay(&self, params: &Value) -> Result<TardisMachineStream> {
        self.open_machine("ws_replay", params)
            .map(TardisMachineStream)
    }
    pub fn ws_replay_normalized(&self, options: &Value) -> Result<TardisMachineStream> {
        self.open_machine(
            "ws_replay_normalized",
            &serde_json::json!({"options":options}),
        )
        .map(TardisMachineStream)
    }
    pub fn stream_normalized(&self, options: &Value) -> Result<TardisMachineStream> {
        self.open_machine("stream_normalized", &serde_json::json!({"options":options}))
            .map(TardisMachineStream)
    }
}

impl PythProClient {
    pub fn subscribe(
        &self,
        params: &Value,
        endpoints: Option<&[String]>,
    ) -> Result<crate::EventStream> {
        self.pro_stream("subscribe", params, endpoints)
    }
    pub fn stream_merkle_roots(&self, endpoints: Option<&[String]>) -> Result<crate::EventStream> {
        self.pro_stream("stream_merkle_roots", &serde_json::json!({}), endpoints)
    }
    fn pro_stream(
        &self,
        op: &str,
        params: &Value,
        endpoints: Option<&[String]>,
    ) -> Result<crate::EventStream> {
        let mut cfg = http::config("pyth_pro", op, params, None, self.api_key.as_deref(), None);
        if let Some(urls) = endpoints {
            cfg["endpoints"] = serde_json::json!(urls);
        }
        crate::EventStream::open_with_transport(
            &cfg.to_string(),
            self.transport.as_ref().map(http::Policy::connection),
        )
    }
}

#[cfg(feature = "async")]
impl crate::AsyncClient<PythProClient> {
    pub async fn subscribe(
        &self,
        params: &Value,
        endpoints: Option<&[String]>,
    ) -> Result<crate::AsyncEventStream> {
        let params = params.clone();
        let endpoints = endpoints.map(<[String]>::to_vec);
        self.run(move |client| {
            client
                .subscribe(&params, endpoints.as_deref())
                .map(crate::EventStream::into_async)
        })
        .await
    }
    pub async fn stream_merkle_roots(
        &self,
        endpoints: Option<&[String]>,
    ) -> Result<crate::AsyncEventStream> {
        let endpoints = endpoints.map(<[String]>::to_vec);
        self.run(move |client| {
            client
                .stream_merkle_roots(endpoints.as_deref())
                .map(crate::EventStream::into_async)
        })
        .await
    }
}

impl LighterClient {
    pub fn with_signer(
        mut self,
        chain_id: u32,
        account_index: u64,
        api_key_index: u8,
        key: &str,
    ) -> Self {
        self.venue_auth = Some(std::sync::Arc::new(
            serde_json::json!({ "chain_id":chain_id,"account_index":account_index,"api_key_index":api_key_index,"key":key}),
        ));
        self
    }
    fn trading(&self, op: &str, params: Value) -> Result<Value> {
        let mut cfg = http::config("lighter", op, &params, self.origin.as_deref(), None, None);
        cfg["auth"] = (**self
            .venue_auth
            .as_ref()
            .ok_or(crate::Error::AuthenticationRequired)?)
        .clone();
        match &self.transport {
            Some(transport) => crate::native::execute_with_transport(&cfg, transport.connection()),
            None => http::call(12, cfg),
        }
    }
    pub fn create_order(&self, order: &Value, options: &Value) -> Result<Value> {
        self.trading(
            "create_order",
            serde_json::json!({"order":order,"options":options}),
        )
    }
    pub fn cancel_order(&self, order: &Value, options: &Value) -> Result<Value> {
        self.trading(
            "cancel_order",
            serde_json::json!({"order":order,"options":options}),
        )
    }
    pub fn resync_nonce(&self) -> Result<u64> {
        self.trading("resync_nonce", serde_json::json!({}))?
            .as_u64()
            .ok_or(crate::Error::Decode("invalid nonce"))
    }
    pub fn auth_token(&self, deadline: u64) -> Result<String> {
        self.trading("auth_token", serde_json::json!({"deadline":deadline}))?
            .as_str()
            .map(str::to_owned)
            .ok_or(crate::Error::Decode("invalid token"))
    }
    pub fn prepare_transaction(
        &self,
        kind: &str,
        order: &Value,
        options: &Value,
        nonce: u64,
    ) -> Result<Value> {
        self.trading(
            "prepare_transaction",
            serde_json::json!({"kind":kind,"order":order,"options":options,"nonce":nonce}),
        )
    }
}

#[cfg(feature = "async")]
impl crate::AsyncClient<DeriveClient> {
    pub async fn open_public_rpc(
        &self,
        endpoint: Option<&str>,
        options: &Value,
    ) -> Result<crate::AsyncSession<DeriveRpcSession>> {
        let endpoint = endpoint.map(str::to_owned);
        let options = options.clone();
        self.run(move |client| {
            client
                .open_public_rpc(endpoint.as_deref(), &options)
                .map(DeriveRpcSession::into_async)
        })
        .await
    }
}

#[cfg(feature = "async")]
impl crate::AsyncSession<ParadexRpcSession> {
    pub async fn create_batch(&self, requests: &[Value]) -> Result<Value> {
        let requests = requests.to_vec();
        self.commands
            .run(move |session| session.create_batch(&requests))
            .await
    }
    pub async fn modify_order(&self, id: &str, request: &Value) -> Result<Value> {
        let (id, request) = (id.to_owned(), request.clone());
        self.commands
            .run(move |s| s.modify_order(&id, &request))
            .await
    }
    pub async fn cancel_order(&self, id: &str) -> Result<Value> {
        let id = id.to_owned();
        self.commands.run(move |s| s.cancel_order(&id)).await
    }
    pub async fn cancel_by_client_id(&self, client_id: &str, market: &str) -> Result<Value> {
        let (id, market) = (client_id.to_owned(), market.to_owned());
        self.commands
            .run(move |s| s.cancel_by_client_id(&id, &market))
            .await
    }
    pub async fn cancel_batch(&self, ids: &[String]) -> Result<Value> {
        let ids = ids.to_vec();
        self.commands.run(move |s| s.cancel_batch(&ids)).await
    }
    pub async fn cancel_all(&self, market: Option<&str>) -> Result<Value> {
        let market = market.map(str::to_owned);
        self.commands
            .run(move |s| s.cancel_all(market.as_deref()))
            .await
    }
    pub async fn set_cancel_on_disconnect(&self, enabled: bool) -> Result<Value> {
        self.commands
            .run(move |s| s.set_cancel_on_disconnect(enabled))
            .await
    }
}

#[cfg(feature = "async")]
impl crate::AsyncSession<PolymarketPerpsRpcSession> {
    pub async fn ping(&self) -> Result<Value> {
        self.commands.run(PolymarketPerpsRpcSession::ping).await
    }
}

#[cfg(feature = "async")]
impl crate::AsyncSession<PascalRpcSession> {
    pub async fn ping(&self) -> Result<Value> {
        self.commands.run(PascalRpcSession::ping).await
    }
}

#[cfg(feature = "async")]
impl crate::AsyncClient<QfexClient> {
    pub async fn trading_session(
        &self,
        options: &Value,
        endpoint: Option<&str>,
    ) -> Result<crate::AsyncSession<QfexTradingSession>> {
        let (options, endpoint) = (options.clone(), endpoint.map(str::to_owned));
        self.run(move |client| {
            let session = client.trading_session(&options, endpoint.as_deref())?;
            let events = session.0.clone();
            Ok(crate::AsyncSession::new(session, events))
        })
        .await
    }
}

#[cfg(feature = "async")]
impl crate::AsyncSession<QfexTradingSession> {
    pub async fn subscribe_account(&self) -> Result<()> {
        self.commands
            .run(QfexTradingSession::subscribe_account)
            .await
    }
    pub async fn add_order(&self, order: &Value) -> Result<Value> {
        let order = order.clone();
        self.commands.run(move |s| s.add_order(&order)).await
    }
    pub async fn cancel_order(&self, order: &Value) -> Result<Value> {
        let order = order.clone();
        self.commands.run(move |s| s.cancel_order(&order)).await
    }
    pub async fn user_orders(&self, request: &Value) -> Result<Value> {
        let request = request.clone();
        self.commands.run(move |s| s.user_orders(&request)).await
    }
}

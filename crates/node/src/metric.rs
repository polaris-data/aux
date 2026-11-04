use crate::prelude::*;
use crate::{
    convert, errors,
    state::{self, ClientState},
};
use aux_rs::{StandardTransport, metric as m};
use m::EvmSigner as _;
use std::sync::Arc;
use zeroize::Zeroizing;

fn local(e: m::Error) -> crate::errors::Error {
    errors::metric(aux_rs::Error::Exchange(e))
}
fn transaction_request(request: &Input) -> Result<m::PreparedTransaction> {
    let mut value = convert::from_input(request)?;
    if let Some(amount @ serde_json::Value::Number(_)) = value.get_mut("value") {
        *amount = serde_json::Value::String(amount.to_string());
    }
    serde_json::from_value(value).map_err(|_| {
        errors::ValidationError::new_err(
            "request does not match the native Metric transaction schema",
        )
    })
}
// Alloy's U256 visitor accepts strings and u64 values. Preserve larger
// wide input integers as exact decimal strings before deserializing helpers.
fn wide_integers(value: serde_json::Value) -> serde_json::Value {
    use serde_json::Value;
    match value {
        Value::Number(ref n) if n.as_u64().is_none() && n.as_i64().is_none() => {
            Value::String(n.to_string())
        }
        Value::Array(values) => Value::Array(values.into_iter().map(wide_integers).collect()),
        Value::Object(values) => Value::Object(
            values
                .into_iter()
                .map(|(k, v)| (k, wide_integers(v)))
                .collect(),
        ),
        other => other,
    }
}

enum Kind {
    Local(m::LocalEvmSigner),
    Remote {
        address: m::Address,
        callback: Callback,
    },
}
#[derive(Clone)]
struct Signer {
    inner: Arc<Kind>,
}
impl m::EvmSigner for Signer {
    fn address(&self) -> m::Address {
        match &*self.inner {
            Kind::Local(s) => s.address(),
            Kind::Remote { address, .. } => *address,
        }
    }
    async fn sign_digest(&self, digest: m::B256) -> Result<m::Signature, m::Error> {
        match &*self.inner {
            Kind::Local(s) => s.sign_digest(digest).await,
            Kind::Remote { callback, .. } => {
                let payload=convert::output(serde_json::json!({"digest":digest,"address":self.address(),"scheme":"evm_digest"})).map_err(|_|m::Error::Signing("callback payload conversion failed"))?;
                let result = crate::signers::callback(callback, payload)
                    .await
                    .map_err(|_| m::Error::Signing("signer callback failed"))?;
                context::evaluate(|| {
                    let text = convert::parse::<String>(&result).map_err(|_| {
                        m::Error::Signing("callback must return a signature hex string")
                    })?;
                    text.parse()
                        .map_err(|_| m::Error::Signing("invalid callback signature"))
                })
            }
        }
    }
}
#[derive(Clone)]
pub struct MetricSigner {
    inner: Signer,
}
impl MetricSigner {
    pub(crate) fn new(private_key: String) -> Result<Self> {
        Ok(Self {
            inner: Signer {
                inner: Arc::new(Kind::Local(
                    m::LocalEvmSigner::from_hex(Zeroizing::new(private_key)).map_err(local)?,
                )),
            },
        })
    }
    pub(crate) fn remote(address: &str, callback: Callback) -> Result<Self> {
        Ok(Self {
            inner: Signer {
                inner: Arc::new(Kind::Remote {
                    address: address.parse().map_err(|_| {
                        errors::ValidationError::new_err("invalid EVM signer address")
                    })?,
                    callback,
                }),
            },
        })
    }
    pub(crate) fn address(&self) -> String {
        self.inner.address().to_string()
    }
    pub(crate) fn __repr__(&self) -> &'static str {
        "MetricSigner()"
    }
    pub(crate) async fn sign_digest(&self, digest: &str) -> Result<Output> {
        let digest = parse(digest)?;
        let signer = self.inner.clone();
        convert::future_output(async move {
            let signature = signer.sign_digest(digest).await.map_err(local)?;
            if signature
                .recover_address_from_prehash(&digest)
                .map_err(|_| errors::SigningError::new_err("invalid external signature"))?
                != signer.address()
            {
                return Err(errors::SigningError::new_err(
                    "external signature belongs to another signer",
                ));
            }
            Ok(signature.to_string())
        })
        .await
    }
    pub(crate) async fn sign_personal_digest(&self, digest: &str) -> Result<Output> {
        let digest = parse(digest)?;
        let signer = self.inner.clone();
        convert::future_output(async move {
            Ok(m::oracle::sign_personal_digest(&signer, digest)
                .await
                .map_err(local)?
                .to_string())
        })
        .await
    }

    pub(crate) async fn sign_transaction(&self, request: Input) -> Result<Output> {
        let request = transaction_request(&request)?;
        let signer = self.inner.clone();
        convert::future_output(async move {
            convert::output(request.sign(&signer).await.map_err(local)?)
        })
        .await
    }
}
#[derive(Clone)]
struct Clients {
    public: m::MetricClient<StandardTransport>,
    private: Option<m::MetricClient<StandardTransport, m::TradingKey>>,
}
impl state::ClientResources for Clients {
    fn transport(&self) -> &StandardTransport {
        self.public.transport()
    }
}
pub struct MetricClient {
    state: Arc<ClientState<Clients>>,
}
fn input<T: serde::de::DeserializeOwned>(r: Option<Input>) -> Result<T> {
    let v = r
        .as_ref()
        .map(convert::from_input)
        .transpose()?
        .unwrap_or(serde_json::json!({}));
    serde_json::from_value(v).map_err(|_| {
        errors::ValidationError::new_err("request does not match the native Metric schema")
    })
}
impl MetricClient {
    pub(crate) fn new(
        chain_id: u64,
        api_url: Option<String>,
        trading_api_key: Option<String>,
        timeout_ms: u64,
    ) -> Result<Self> {
        let e = m::Metric::custom(
            chain_id,
            api_url.unwrap_or_else(|| "https://api.metric.xyz".into()),
        )
        .map_err(local)?;
        let config = aux_rs::TransportConfig {
            timeout: std::time::Duration::from_millis(timeout_ms),
            ..Default::default()
        };
        let public = state::build(|| m::MetricClient::with_config(e, config))?;
        let private = trading_api_key
            .map(|s| m::TradingKey::new(Zeroizing::new(s)).map(|key| public.clone().with_auth(key)))
            .transpose()
            .map_err(local)?;
        Ok(Self {
            state: ClientState::new(Clients { public, private }),
        })
    }
    pub(crate) fn closed(&self) -> bool {
        self.state.is_closed()
    }
    pub(crate) async fn chains(&self) -> Result<Output> {
        let c = self.state.client()?.public;
        self.state
            .run(async move { c.chains().await.map_err(errors::metric) })
            .await
    }
    pub(crate) async fn factory_settings(&self) -> Result<Output> {
        let c = self.state.client()?.public;
        self.state
            .run(async move { c.factory_settings().await.map_err(errors::metric) })
            .await
    }
    pub(crate) async fn metadata(&self, request: Option<Input>) -> Result<Output> {
        let r = input(request)?;
        let c = self.state.client()?.public;
        self.state
            .run(async move { c.metadata(r).await.map_err(errors::metric) })
            .await
    }
    pub(crate) async fn cross_chain_metadata(&self, request: Option<Input>) -> Result<Output> {
        let r = input(request)?;
        let c = self.state.client()?.public;
        self.state
            .run(async move { c.cross_chain_metadata(r).await.map_err(errors::metric) })
            .await
    }
    pub(crate) async fn bid_ask(&self, pool: String, depth: Option<bool>) -> Result<Output> {
        let c = self.state.client()?.public;
        self.state
            .run(async move { c.bid_ask(&pool, depth).await.map_err(errors::metric) })
            .await
    }
    pub(crate) async fn batch_bid_ask(&self, request: Input) -> Result<Output> {
        let r = convert::parse(&request)?;
        let c = self.state.client()?.public;
        self.state
            .run(async move { c.batch_bid_ask(r).await.map_err(errors::metric) })
            .await
    }
    pub(crate) async fn route(&self, request: Input) -> Result<Output> {
        let r = convert::parse(&request)?;
        let c = self.state.client()?.public;
        self.state
            .run(async move { c.route(r).await.map_err(errors::metric) })
            .await
    }
    pub(crate) async fn pamm_quote(&self, request: Input) -> Result<Output> {
        let r = convert::parse(&request)?;
        let c = self.state.client()?.private.ok_or_else(|| {
            errors::AuthenticationError::new_err("Metric Trading API key required")
        })?;
        self.state
            .run(async move { c.pamm_quote(r).await.map_err(errors::metric) })
            .await
    }
    pub(crate) async fn aclose(&self) -> Result<()> {
        self.state.close().await
    }
}
impl Drop for MetricClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
#[derive(Clone)]
struct ChainClients {
    client: m::MetricChainClient<StandardTransport>,
    signer: Option<Signer>,
}
impl state::ClientResources for ChainClients {
    fn transport(&self) -> &StandardTransport {
        self.client.transport()
    }
}
pub struct MetricChainClient {
    state: Arc<ClientState<ChainClients>>,
}
fn parse<T: std::str::FromStr>(s: &str) -> Result<T> {
    s.parse()
        .map_err(|_| errors::ValidationError::new_err("invalid native EVM identifier"))
}
impl MetricChainClient {
    pub(crate) fn new(
        chain_id: u64,
        read_rpc: String,
        write_rpc: Option<String>,
        signer: Option<&MetricSigner>,
        timeout_ms: u64,
        code_hashes: Option<std::collections::BTreeMap<String, String>>,
    ) -> Result<Self> {
        let mut config =
            m::ChainConfig::new(chain_id, read_rpc.clone(), write_rpc.unwrap_or(read_rpc))
                .map_err(local)?;
        if let Some(hashes) = code_hashes {
            for (address, hash) in hashes {
                config = config.with_code_hash(parse(&address)?, parse(&hash)?);
            }
        }
        let transport_config = aux_rs::TransportConfig {
            timeout: std::time::Duration::from_millis(timeout_ms),
            ..Default::default()
        };
        let client = state::build(|| m::MetricChainClient::with_config(config, transport_config))?;
        Ok(Self {
            state: ClientState::new(ChainClients {
                client,
                signer: signer.map(|s| s.inner.clone()),
            }),
        })
    }
    pub(crate) fn closed(&self) -> bool {
        self.state.is_closed()
    }
    pub(crate) async fn validate_chain(&self) -> Result<Output> {
        let c = self.state.client()?.client;
        self.state
            .run(async move { c.validate_chain().await.map_err(errors::metric) })
            .await
    }
    pub(crate) async fn block_number(&self) -> Result<Output> {
        let c = self.state.client()?.client;
        self.state
            .run(async move { c.block_number().await.map_err(errors::metric) })
            .await
    }
    pub(crate) async fn block(&self, number: &str) -> Result<Output> {
        let number = number.to_string();
        let c = self.state.client()?.client;
        self.state
            .run(async move { c.block(&number).await.map_err(errors::metric) })
            .await
    }
    pub(crate) async fn native_balance(&self, address: &str, block: &str) -> Result<Output> {
        let address = parse(address)?;
        let block = block.to_string();
        let c = self.state.client()?.client;
        self.state
            .run(async move {
                c.native_balance(address, &block)
                    .await
                    .map_err(errors::metric)
            })
            .await
    }
    pub(crate) async fn code(&self, address: &str, block: &str) -> Result<Output> {
        let address = parse(address)?;
        let block = block.to_string();
        let c = self.state.client()?.client;
        self.state
            .run(async move { c.code(address, &block).await.map_err(errors::metric) })
            .await
    }
    pub(crate) fn encode_call(
        &self,
        contract: &str,
        function: &str,
        args: Input,
    ) -> Result<String> {
        self.state.client()?;
        Ok(
            m::encode_contract_call(contract, function, &convert::from_input(&args)?)
                .map_err(local)?
                .to_string(),
        )
    }
    pub(crate) fn prepare_call(
        &self,
        contract: &str,
        function: &str,
        to: &str,
        args: Input,
        value: &str,
    ) -> Result<Output> {
        self.state.client()?;
        let data = m::encode_contract_call(contract, function, &convert::from_input(&args)?)
            .map_err(local)?;
        convert::output(m::PreparedCall {
            to: parse(to)?,
            data,
            value: parse(value)?,
        })
    }
    pub(crate) async fn liquidity_snapshot(
        &self,
        pool: &str,
        lens: &str,
        bins: Vec<i16>,
        block: &str,
    ) -> Result<Output> {
        let pool = parse(pool)?;
        let lens = parse(lens)?;
        let block = block.to_string();
        let c = self.state.client()?.client;
        self.state
            .run(async move {
                c.liquidity_snapshot(pool, lens, &bins, &block)
                    .await
                    .map_err(errors::metric)
            })
            .await
    }
    pub(crate) async fn oracle_stop_loss_status(&self, request: Input) -> Result<Output> {
        let request = serde_json::from_value(wide_integers(convert::from_input(&request)?))
            .map_err(|_| errors::ValidationError::new_err("invalid stop-loss status request"))?;
        let c = self.state.client()?.client;
        self.state
            .run(async move {
                c.oracle_stop_loss_status(request)
                    .await
                    .map_err(errors::metric)
            })
            .await
    }
    pub(crate) async fn prepare_route(&self, request: Input) -> Result<Output> {
        let r = convert::parse(&request)?;
        let c = self.state.client()?.client;
        self.state
            .run(async move { c.prepare_route(r).await.map_err(errors::metric) })
            .await
    }
    pub(crate) async fn gas_price(&self) -> Result<Output> {
        let c = self.state.client()?.client;
        self.state
            .run(async move { c.gas_price().await.map_err(errors::metric) })
            .await
    }
    pub(crate) async fn estimate_gas(&self, request: Input) -> Result<Output> {
        let r = convert::from_input(&request)?;
        let c = self.state.client()?.client;
        self.state
            .run(async move { c.estimate_gas(r).await.map_err(errors::metric) })
            .await
    }
    pub(crate) async fn resync_nonce(&self, address: &str, expected_hash: &str) -> Result<Output> {
        let address = parse(address)?;
        let hash = parse(expected_hash)?;
        let c = self.state.client()?.client;
        self.state
            .run(async move { c.resync_nonce(address, hash).await.map_err(errors::metric) })
            .await
    }
    pub(crate) fn decode_input(
        &self,
        contract: &str,
        function: &str,
        data: &str,
    ) -> Result<Output> {
        self.state.client()?;
        let b: m::Bytes = parse(data)?;
        convert::output(m::decode_contract_input(contract, function, &b).map_err(local)?)
    }
    pub(crate) fn decode_output(
        &self,
        contract: &str,
        function: &str,
        data: &str,
    ) -> Result<Output> {
        self.state.client()?;
        let b: m::Bytes = parse(data)?;
        convert::output(m::decode_contract_output(contract, function, &b).map_err(local)?)
    }
    pub(crate) fn decode_event(
        &self,
        contract: &str,
        event: &str,
        topics: Vec<String>,
        data: &str,
    ) -> Result<Output> {
        self.state.client()?;
        let topics: Vec<m::B256> = topics.iter().map(|s| parse(s)).collect::<Result<_>>()?;
        let b: m::Bytes = parse(data)?;
        convert::output(m::decode_contract_event(contract, event, &topics, &b).map_err(local)?)
    }
    pub(crate) fn decode_error(&self, contract: &str, data: &str) -> Result<Output> {
        self.state.client()?;
        let b: m::Bytes = parse(data)?;
        convert::output(m::decode_contract_error(contract, &b).map_err(local)?)
    }
    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn call(
        &self,
        contract: String,
        function: String,
        to: String,
        args: Input,
        block: &str,
        from_address: Option<String>,
        value: &str,
    ) -> Result<Output> {
        let data = m::encode_contract_call(&contract, &function, &convert::from_input(&args)?)
            .map_err(local)?;
        let to = parse(&to)?;
        let from = from_address.as_ref().map(|s| parse(s)).transpose()?;
        let value = parse(value)?;
        let block = block.to_string();
        let c = self.state.client()?.client;
        self.state
            .run(async move {
                let b = c
                    .eth_call(to, data, from, value, &block)
                    .await
                    .map_err(errors::metric)?;
                m::decode_contract_output(&contract, &function, &b).map_err(local)
            })
            .await
    }
    pub(crate) async fn send_transaction(&self, request: Input) -> Result<Output> {
        let r = transaction_request(&request)?;
        let c = self.state.client()?;
        let s = c
            .signer
            .ok_or_else(|| errors::AuthenticationError::new_err("MetricSigner required"))?
            .clone();
        self.state
            .run(async move {
                c.client
                    .send_transaction(r, &s)
                    .await
                    .map_err(errors::metric)
            })
            .await
    }
    pub(crate) async fn submit_raw(&self, data: &str) -> Result<Output> {
        let data = parse(data)?;
        let c = self.state.client()?.client;
        self.state
            .run(async move { c.submit_raw(data).await.map_err(errors::metric) })
            .await
    }
    pub(crate) async fn receipt(&self, hash: &str) -> Result<Output> {
        let h = parse(hash)?;
        let c = self.state.client()?.client;
        self.state
            .run(async move { c.receipt(h).await.map_err(errors::metric) })
            .await
    }
    pub(crate) async fn transaction(&self, hash: &str) -> Result<Output> {
        let h = parse(hash)?;
        let c = self.state.client()?.client;
        self.state
            .run(async move { c.transaction(h).await.map_err(errors::metric) })
            .await
    }
    pub(crate) async fn logs(&self, filter: Input) -> Result<Output> {
        let r = convert::from_input(&filter)?;
        let c = self.state.client()?.client;
        self.state
            .run(async move { c.logs(r).await.map_err(errors::metric) })
            .await
    }
    pub(crate) fn pending(&self, address: &str) -> Result<Output> {
        convert::output(self.state.client()?.client.pending(parse(address)?))
    }
    pub(crate) async fn reconcile(&self, address: &str) -> Result<Output> {
        let address = parse(address)?;
        let c = self.state.client()?.client;
        self.state
            .run(async move { c.reconcile(address).await.map_err(errors::metric) })
            .await
    }
    pub(crate) async fn transaction_count(&self, address: &str) -> Result<Output> {
        let address = parse(address)?;
        let c = self.state.client()?.client;
        self.state
            .run(async move { c.transaction_count(address).await.map_err(errors::metric) })
            .await
    }
    pub(crate) async fn aclose(&self) -> Result<()> {
        self.state.close().await
    }
}
impl Drop for MetricChainClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}

pub fn metric_tool(name: &str, request: Input) -> Result<Output> {
    let v = wide_integers(convert::from_input(&request)?);
    let uint = |field: &str| -> Result<m::U256> {
        let raw = &v[field];
        let text = raw
            .as_str()
            .map(str::to_string)
            .unwrap_or_else(|| raw.to_string());
        text.parse()
            .map_err(|_| errors::ValidationError::new_err("exact unsigned integer required"))
    };
    let output_uint = |n: m::U256| -> Result<Output> {
        convert::output(
            serde_json::from_str::<serde_json::Value>(&n.to_string()).expect("integer JSON"),
        )
    };
    let string = |field: &str| -> Result<&str> {
        v[field]
            .as_str()
            .ok_or_else(|| errors::ValidationError::new_err("native string field required"))
    };
    let number = |field: &str| -> Result<u64> {
        v[field]
            .as_u64()
            .ok_or_else(|| errors::ValidationError::new_err("unsigned integer field required"))
    };
    let rounding = match v["rounding"].as_str().unwrap_or("exact") {
        "exact" => m::math::Rounding::Exact,
        "down" => m::math::Rounding::Down,
        "up" => m::math::Rounding::Up,
        _ => {
            return Err(errors::ValidationError::new_err(
                "rounding must be exact, down or up",
            ));
        }
    };
    match name {
        "addresses" => convert::output(m::Addresses::current(number("chainId")?).map_err(local)?),
        "pack_bins" => {
            let bins: Vec<m::math::BinData> = serde_json::from_value(v["bins"].clone())
                .map_err(|_| errors::ValidationError::new_err("invalid native bins"))?;
            convert::output(
                m::math::pack_bin_data_array(&bins)
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>(),
            )
        }
        "unpack_bins" => {
            let words = v["words"]
                .as_array()
                .ok_or_else(|| errors::ValidationError::new_err("words array required"))?
                .iter()
                .map(|x| {
                    x.as_str()
                        .map(str::to_owned)
                        .unwrap_or_else(|| x.to_string())
                        .parse()
                        .map_err(|_| {
                            errors::ValidationError::new_err("exact packed integer required")
                        })
                })
                .collect::<Result<Vec<_>>>()?;
            convert::output(m::math::unpack_bin_data_array(&words))
        }
        "decimal_to_raw" => output_uint(
            m::math::decimal_to_raw(
                string("value")?,
                u8::try_from(number("decimals")?)
                    .map_err(|_| errors::ValidationError::new_err("invalid decimal scale"))?,
            )
            .map_err(local)?,
        ),
        "mul_div" => output_uint(
            m::math::mul_div(uint("a")?, uint("b")?, uint("denominator")?, rounding)
                .map_err(local)?,
        ),
        "price_to_q64" => output_uint(
            m::math::price_to_q64(uint("numerator")?, uint("denominator")?, rounding)
                .map_err(local)?,
        ),
        "price_x64_to_e18" => {
            output_uint(m::math::price_x64_to_e18(uint("price")?).map_err(local)?)
        }
        "slippage_limit" => output_uint(
            m::math::slippage_limit(
                uint("amount")?,
                u16::try_from(number("bps")?)
                    .map_err(|_| errors::ValidationError::new_err("invalid slippage bps"))?,
                v["exactOutput"].as_bool().unwrap_or(false),
            )
            .map_err(local)?,
        ),
        "position_bin_key" => {
            let bin = v["bin"]
                .as_i64()
                .and_then(|n| i16::try_from(n).ok())
                .ok_or_else(|| errors::ValidationError::new_err("int16 bin required"))?;
            convert::output(
                m::math::position_bin_key(parse(string("owner")?)?, uint("salt")?, bin)
                    .map_err(local)?,
            )
        }
        "encode_bin_state" => {
            let state = serde_json::from_value(v)
                .map_err(|_| errors::ValidationError::new_err("invalid bin state"))?;
            output_uint(m::math::encode_bin_state(&state).map_err(local)?)
        }
        "decode_bin_state" => convert::output(m::math::decode_bin_state(uint("word")?)),
        "unpack_swap_details" => convert::output(m::math::unpack_swap_details(uint("word")?)),
        "unpack_two_uint128" => convert::output(m::math::unpack_two_uint128(uint("word")?)),
        "unpack_two_int128" => convert::output(m::math::unpack_two_int128(uint("word")?)),
        "direction_bitmap" => {
            let values: Vec<bool> = serde_json::from_value(v["directions"].clone())
                .map_err(|_| errors::ValidationError::new_err("boolean directions required"))?;
            output_uint(m::math::direction_bitmap(&values).map_err(local)?)
        }
        "liquidity_cursor_bounds" => {
            let r = serde_json::from_value(v)
                .map_err(|_| errors::ValidationError::new_err("invalid cursor request"))?;
            convert::output(m::liquidity::liquidity_cursor_bounds(&r).map_err(local)?)
        }
        "minimum_operational_liquidity" => output_uint(
            m::liquidity::minimum_operational_liquidity(uint("amount0")?, uint("amount1")?)
                .map_err(local)?,
        ),
        "recommended_initial_scaled_amounts" => {
            let (a, b) = m::liquidity::recommended_initial_scaled_amounts(uint("priceE18")?)
                .map_err(local)?;
            convert::output(
                serde_json::json!({"initialScaledAmount0PerE18Shares":a.to_string(),"initialScaledAmount1PerE18Shares":b.to_string()}),
            )
        }
        "extension_order" => {
            let slots: Vec<u8> = serde_json::from_value(v["slots"].clone())
                .map_err(|_| errors::ValidationError::new_err("extension slots required"))?;
            output_uint(m::liquidity::extension_order(&slots).map_err(local)?)
        }
        "percentage_removal" => output_uint(
            m::liquidity::percentage_removal(
                uint("shares")?,
                u16::try_from(number("bps")?)
                    .map_err(|_| errors::ValidationError::new_err("invalid percentage"))?,
            )
            .map_err(local)?,
        ),
        "scale_weighted_deltas" => {
            let weights = serde_json::from_value(v["weights"].clone())
                .map_err(|_| errors::ValidationError::new_err("invalid native weight deltas"))?;
            convert::output(
                m::liquidity::scale_weighted_deltas(
                    &weights,
                    uint("probeNeed0")?,
                    uint("probeNeed1")?,
                    uint("max0")?,
                    uint("max1")?,
                )
                .map_err(local)?,
            )
        }
        "stop_loss_metrics" => {
            let r = serde_json::from_value(v)
                .map_err(|_| errors::ValidationError::new_err("invalid stop-loss inputs"))?;
            let (a, b) = m::liquidity::stop_loss_metrics(&r).map_err(local)?;
            convert::output(
                serde_json::json!({"metricToken0":a.to_string(),"metricToken1":b.to_string()}),
            )
        }
        "decayed_high_watermark" => output_uint(
            m::liquidity::decayed_high_watermark(uint("hwm")?, uint("rate")?, number("seconds")?)
                .map_err(local)?,
        ),
        "stop_loss_threshold" => output_uint(
            m::liquidity::stop_loss_threshold(
                uint("hwm")?,
                u32::try_from(number("drawdownE6")?)
                    .map_err(|_| errors::ValidationError::new_err("invalid drawdown"))?,
            )
            .map_err(local)?,
        ),
        "stop_loss_breached" => convert::output(
            m::liquidity::stop_loss_breached(
                uint("metric")?,
                uint("hwm")?,
                u32::try_from(number("drawdownE6")?)
                    .map_err(|_| errors::ValidationError::new_err("invalid drawdown"))?,
            )
            .map_err(local)?,
        ),
        "seconds_until_stop_loss_clears" => convert::output(
            m::liquidity::seconds_until_stop_loss_clears(
                uint("metric")?,
                uint("hwm")?,
                u32::try_from(number("drawdownE6")?)
                    .map_err(|_| errors::ValidationError::new_err("invalid drawdown"))?,
                uint("rate")?,
            )
            .map_err(local)?,
        ),
        "encode_u64x32" => {
            convert::output(m::oracle::encode_u64x32(number("value")?, rounding).map_err(local)?)
        }
        "decode_u64x32" => convert::output(m::oracle::decode_u64x32(
            u32::try_from(number("word")?)
                .map_err(|_| errors::ValidationError::new_err("invalid compressed price"))?,
        )),
        "encode_spread" => convert::output(
            m::oracle::encode_spread(
                u16::try_from(number("units")?)
                    .map_err(|_| errors::ValidationError::new_err("invalid native spread units"))?,
            )
            .map_err(local)?,
        ),
        "codebook" => convert::output(m::oracle::codebook()),
        "pack_oracle_slot" => {
            let lanes = serde_json::from_value(v["lanes"].clone())
                .map_err(|_| errors::ValidationError::new_err("four native lanes required"))?;
            output_uint(
                m::oracle::pack_oracle_slot(
                    &lanes,
                    number("timestampMs")?,
                    u8::try_from(number("slotId")?)
                        .map_err(|_| errors::ValidationError::new_err("invalid slot"))?,
                    rounding,
                )
                .map_err(local)?,
            )
        }
        "oracle_push_calldata" => {
            let words: Vec<m::U256> = serde_json::from_value(v["words"].clone())
                .map_err(|_| errors::ValidationError::new_err("native words required"))?;
            convert::output(
                m::oracle::oracle_push_calldata(&words)
                    .map_err(local)?
                    .to_string(),
            )
        }
        "feed_id" => convert::output(
            m::oracle::feed_id(
                parse(string("creator")?)?,
                uint("chainId")?,
                u8::try_from(number("slot")?)
                    .map_err(|_| errors::ValidationError::new_err("invalid slot"))?,
                u8::try_from(number("position")?)
                    .map_err(|_| errors::ValidationError::new_err("invalid position"))?,
            )
            .map_err(local)?,
        ),
        "slot_update_digest" => convert::output(m::oracle::slot_update_digest(
            uint("chainId")?,
            parse(string("oracle")?)?,
            parse(string("creator")?)?,
            uint("word")?,
        )),
        "pusher_consent_digest" => convert::output(m::oracle::pusher_consent_digest(
            uint("chainId")?,
            parse(string("oracle")?)?,
            uint("deadline")?,
            parse(string("pusher")?)?,
            parse(string("creator")?)?,
        )),
        "native_payment" => {
            let call = serde_json::from_value(v["call"].clone())
                .map_err(|_| errors::ValidationError::new_err("invalid prepared call"))?;
            convert::output(
                m::native_payment(&call, uint("value")?, parse(string("wrappedNative")?)?)
                    .map_err(local)?,
            )
        }
        "addition_plan" => {
            let request = serde_json::from_value(v).map_err(|_| {
                errors::ValidationError::new_err("invalid liquidity addition request")
            })?;
            convert::output(m::planning::addition_plan(&request).map_err(local)?)
        }
        "prepare_factory_create_pool_params" => {
            let request = serde_json::from_value(v).map_err(|_| {
                errors::ValidationError::new_err("invalid pool creation parameters")
            })?;
            convert::output(
                m::creation::prepare_factory_create_pool_params(&request).map_err(local)?,
            )
        }
        "prepare_create_pool" => {
            let request = serde_json::from_value(v)
                .map_err(|_| errors::ValidationError::new_err("invalid pool creation request"))?;
            convert::output(m::creation::prepare_create_pool(&request).map_err(local)?)
        }
        "create_uniform_bins" => {
            let request = serde_json::from_value(v)
                .map_err(|_| errors::ValidationError::new_err("invalid uniform bin request"))?;
            convert::output(m::creation::create_uniform_bins(&request).map_err(local)?)
        }
        "prepare_liquidity_addition" => {
            let request = serde_json::from_value(v).map_err(|_| {
                errors::ValidationError::new_err("invalid liquidity addition workflow")
            })?;
            convert::output(m::workflows::prepare_liquidity_addition(&request).map_err(local)?)
        }
        "liquidity_native_payment" => {
            let call = serde_json::from_value(v["call"].clone())
                .map_err(|_| errors::ValidationError::new_err("invalid prepared liquidity call"))?;
            convert::output(
                m::workflows::liquidity_native_payment(&call, uint("value")?).map_err(local)?,
            )
        }
        "prepare_external_swap" => {
            let request = serde_json::from_value(v)
                .map_err(|_| errors::ValidationError::new_err("invalid external swap workflow"))?;
            convert::output(m::workflows::prepare_external_swap(&request).map_err(local)?)
        }
        "prepare_swap_with_fallback" => {
            let request = serde_json::from_value(v)
                .map_err(|_| errors::ValidationError::new_err("invalid swap fallback workflow"))?;
            convert::output(m::workflows::prepare_swap_with_fallback(&request).map_err(local)?)
        }
        "evaluate_oracle_stop_loss" => {
            let request = serde_json::from_value(v)
                .map_err(|_| errors::ValidationError::new_err("invalid stop-loss evaluation"))?;
            convert::output(m::stop_loss::evaluate_oracle_stop_loss(&request).map_err(local)?)
        }
        "weighted_plan" => {
            let request = serde_json::from_value(v).map_err(|_| {
                errors::ValidationError::new_err("invalid weighted liquidity request")
            })?;
            convert::output(m::planning::weighted_plan(&request).map_err(local)?)
        }
        "removal_plan" => {
            let positions: Vec<m::planning::PositionShares> =
                serde_json::from_value(v["positions"].clone())
                    .map_err(|_| errors::ValidationError::new_err("invalid position shares"))?;
            let specs: Vec<m::planning::RemovalSpec> =
                serde_json::from_value(v["specs"].clone())
                    .map_err(|_| errors::ValidationError::new_err("invalid removal specs"))?;
            convert::output(m::planning::removal_plan(&positions, &specs).map_err(local)?)
        }
        "unwrap_output" => {
            let call = serde_json::from_value(v["call"].clone())
                .map_err(|_| errors::ValidationError::new_err("invalid prepared call"))?;
            convert::output(
                m::unwrap_output(
                    &call,
                    parse(string("wrappedNative")?)?,
                    parse(string("recipient")?)?,
                )
                .map_err(local)?,
            )
        }
        _ => Err(errors::ValidationError::new_err("unknown Metric helper")),
    }
}

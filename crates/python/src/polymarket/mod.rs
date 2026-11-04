use crate::{
    convert, errors,
    prediction_signers::*,
    state::{self, ClientResources, ClientState},
    subscription::NativeHandle,
};
use aux_rs::{StandardTransport, polymarket as v};
use pyo3::{prelude::*, types::PyAny};
use serde_json::Value;
use std::sync::Arc;
use v::{Authentication, StreamAuthentication};
use zeroize::Zeroizing;
fn native(error: aux_rs::Error<v::Error>) -> PyErr {
    errors::polymarket(error)
}
fn local(error: v::Error) -> PyErr {
    native(aux_rs::Error::Exchange(error))
}
fn input<T: serde::de::DeserializeOwned>(value: Option<Bound<'_, PyAny>>) -> PyResult<T> {
    let v = value
        .as_ref()
        .map(convert::from_python)
        .transpose()?
        .unwrap_or(serde_json::json!({}));
    serde_json::from_value(v)
        .map_err(|_| errors::ValidationError::new_err("request does not match the native schema"))
}
#[derive(Clone)]
pub(crate) enum Auth {
    None,
    Predictions(v::PolymarketAuth<crate::metric::Signer>),
    Perps(v::PolymarketPerpsAuth<crate::metric::Signer>),
}
impl Auth {
    fn scoped(&self, py: Python<'_>) -> PyResult<Self> {
        Ok(match self {
            Self::None => Self::None,
            Self::Predictions(a) => {
                let mut a = a.clone();
                a.signer = a.signer.as_ref().map(|s| s.scoped(py)).transpose()?;
                Self::Predictions(a)
            }
            Self::Perps(a) => {
                let mut a = a.clone();
                a.owner = a.owner.as_ref().map(|s| s.scoped(py)).transpose()?;
                a.proxy = a.proxy.as_ref().map(|s| s.scoped(py)).transpose()?;
                Self::Perps(a)
            }
        })
    }
}
impl Authentication for Auth {
    fn identity(&self) -> String {
        match self {
            Self::None => "public".into(),
            Self::Predictions(a) => a.identity(),
            Self::Perps(a) => a.identity(),
        }
    }
    async fn prepare(
        &self,
        e: &v::Polymarket,
        o: &v::Operation,
        r: &mut aux_rs::HttpRequest,
    ) -> Result<(), v::Error> {
        match self {
            Self::Predictions(a) => a.prepare(e, o, r).await,
            Self::Perps(a) => a.prepare(e, o, r).await,
            Self::None => Err(v::Error::Authentication(
                "API credentials or signer required",
            )),
        }
    }
}
impl StreamAuthentication for Auth {
    fn stream_expires_at(&self) -> Option<std::time::SystemTime> {
        match self {
            Self::Perps(a) => a.stream_expires_at(),
            _ => None,
        }
    }
    fn stream_identity(&self) -> String {
        self.identity()
    }
    fn stream_credentials(&self, s: &str) -> Result<Option<Value>, v::Error> {
        match self {
            Self::Predictions(a) => a.stream_credentials(s),
            Self::Perps(a) => a.stream_credentials(s),
            Self::None => Ok(None),
        }
    }
}

#[derive(Clone)]
struct PolymarketClientResources {
    client: v::PolymarketClient<StandardTransport, Auth>,
    auth: Auth,
}
impl ClientResources for PolymarketClientResources {
    fn transport(&self) -> &StandardTransport {
        self.client.transport()
    }
}
impl PolymarketClientResources {
    fn scoped(&self, py: Python<'_>) -> PyResult<v::PolymarketClient<StandardTransport, Auth>> {
        Ok(self.client.clone().with_auth(self.auth.scoped(py)?))
    }
}
#[pyclass(module = "aux_py._native")]
pub struct PolymarketClient {
    state: Arc<ClientState<PolymarketClientResources>>,
}
#[pymethods]
impl PolymarketClient {
    #[new]
    #[pyo3(signature=(*,network="mainnet",endpoints=None,websocket=None,signer=None,wallet=None,api_key=None,api_secret=None,api_passphrase=None,order_version=None,negative_risk=false,builder_key=None,builder_secret=None,builder_passphrase=None,relayer_key=None,relayer_key_address=None,session_signer=false,rfq_signature_type=None,timeout_ms=10000,subscription_capacity=128))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        py: Python<'_>,
        network: &str,
        endpoints: Option<Bound<'_, PyAny>>,
        websocket: Option<Bound<'_, PyAny>>,
        signer: Option<PyRef<'_, PolymarketSigner>>,
        wallet: Option<String>,
        api_key: Option<String>,
        api_secret: Option<String>,
        api_passphrase: Option<String>,
        order_version: Option<&str>,
        negative_risk: bool,
        builder_key: Option<String>,
        builder_secret: Option<String>,
        builder_passphrase: Option<String>,
        relayer_key: Option<String>,
        relayer_key_address: Option<String>,
        session_signer: bool,
        rfq_signature_type: Option<u8>,
        timeout_ms: u64,
        subscription_capacity: usize,
    ) -> PyResult<Self> {
        let mut exchange = match network {
            "mainnet" | "production" => v::Polymarket::mainnet(),
            _ => {
                return Err(errors::ValidationError::new_err(
                    "Polymarket requires mainnet or explicit endpoint overrides",
                ));
            }
        };
        if let Some(e) = endpoints {
            let values: std::collections::BTreeMap<String, String> = convert::parse(&e)?;
            for (k, u) in values {
                exchange = exchange.custom(&k, &u, None).map_err(local)?;
            }
        }
        if let Some(w) = websocket {
            let values: std::collections::BTreeMap<String, String> = convert::parse(&w)?;
            for (k, u) in values {
                v::validate_url(&u, true).map_err(local)?;
                if !exchange.websocket.contains_key(&k) {
                    return Err(errors::ValidationError::new_err(
                        "unknown WebSocket service",
                    ));
                }
                exchange.websocket.insert(k, u);
            }
        }
        let owner = signer.as_ref().map(|s| s.inner.clone());
        let auth =
            {
                if owner.is_none()
                    && wallet.is_none()
                    && api_key.is_none()
                    && builder_key.is_none()
                    && relayer_key.is_none()
                {
                    Auth::None
                } else {
                    let address = owner
                        .as_ref()
                        .map(v::EvmSigner::address)
                        .or_else(|| wallet.as_ref().and_then(|s| s.parse().ok()))
                        .ok_or_else(|| {
                            errors::ValidationError::new_err("wallet or signer required")
                        })?;
                    let wallet = wallet
                        .map(|s| {
                            s.parse().map_err(|_| {
                                errors::ValidationError::new_err("invalid wallet address")
                            })
                        })
                        .transpose()?
                        .unwrap_or(address);
                    let builder = match (builder_key, builder_secret, builder_passphrase) {
                        (Some(k), Some(s), Some(p)) => Some(
                            v::ApiCredentials::new(k, Zeroizing::new(s), Zeroizing::new(p))
                                .map_err(local)?,
                        ),
                        (None, None, None) => None,
                        _ => {
                            return Err(errors::ValidationError::new_err(
                                "all builder credential fields are required",
                            ));
                        }
                    };
                    let relayer = match (relayer_key, relayer_key_address) {
                        (Some(k), Some(a)) => Some(
                            v::RelayerKey::new(
                                Zeroizing::new(k),
                                a.parse().map_err(|_| {
                                    errors::ValidationError::new_err("invalid relayer key address")
                                })?,
                            )
                            .map_err(local)?,
                        ),
                        (None, None) => None,
                        _ => {
                            return Err(errors::ValidationError::new_err(
                                "relayer_key and relayer_key_address are required together",
                            ));
                        }
                    };
                    let api = match (api_key, api_secret, api_passphrase) {
                        (Some(k), Some(s), Some(p)) => Some(
                            v::ApiCredentials::new(k, Zeroizing::new(s), Zeroizing::new(p))
                                .map_err(local)?,
                        ),
                        (None, None, None) => None,
                        _ => {
                            return Err(errors::ValidationError::new_err(
                                "all three API credential fields are required",
                            ));
                        }
                    };
                    let order_domain = order_version
                        .map(|v| match v {
                            "v1" | "ctf" => Ok(v::OrderDomain::polygon(
                                v::PositionSystem::Ctf,
                                negative_risk,
                            )),
                            "v2" => Ok(v::OrderDomain::polygon(
                                v::PositionSystem::V2,
                                negative_risk,
                            )),
                            _ => Err(errors::ValidationError::new_err(
                                "order_version must be ctf or v2",
                            )),
                        })
                        .transpose()?;
                    Auth::Predictions(
                        v::PolymarketAuth {
                            address,
                            wallet,
                            signer: owner,
                            api,
                            order_domain,
                            nonce: 0,
                            builder,
                            relayer,
                            session_signer,
                            rfq_signature_type: rfq_signature_type
                                .unwrap_or(if address == wallet { 0 } else { 3 }),
                        }
                        .with_rfq_signature_type(
                            rfq_signature_type.unwrap_or(if address == wallet { 0 } else { 3 }),
                        )
                        .map_err(local)?,
                    )
                }
            };
        let config = aux_rs::TransportConfig {
            timeout: std::time::Duration::from_millis(timeout_ms),
            subscription_capacity,
            ..Default::default()
        };
        let client = state::build(py, || v::PolymarketClient::with_config(exchange, config))?
            .with_auth(auth.clone());
        Ok(Self {
            state: ClientState::new(PolymarketClientResources { client, auth }),
        })
    }
    #[getter]
    fn closed(&self) -> bool {
        self.state.is_closed()
    }
    #[getter]
    fn authenticated(&self) -> PyResult<bool> {
        Ok(!matches!(self.state.client()?.auth, Auth::None))
    }
    fn subscribe<'py>(
        &self,
        py: Python<'py>,
        specification: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let specification: v::Subscription = convert::parse(&specification)?;
        let client = self.state.client()?.scoped(py)?;
        self.state.stream(py, async move {
            client
                .subscribe(specification)
                .await
                .map(NativeHandle::Polymarket)
                .map_err(native)
        })
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.close(py)
    }
    fn __repr__(&self) -> &'static str {
        "PolymarketClient()"
    }
}
impl Drop for PolymarketClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
mod polymarket_client_operations;
#[derive(Clone)]
struct PolymarketPerpsClientResources {
    client: v::PolymarketPerpsClient<StandardTransport, Auth>,
    auth: Auth,
}
impl ClientResources for PolymarketPerpsClientResources {
    fn transport(&self) -> &StandardTransport {
        self.client.transport()
    }
}
impl PolymarketPerpsClientResources {
    fn scoped(
        &self,
        py: Python<'_>,
    ) -> PyResult<v::PolymarketPerpsClient<StandardTransport, Auth>> {
        Ok(self.client.clone().with_auth(self.auth.scoped(py)?))
    }
}
#[pyclass(module = "aux_py._native")]
pub struct PolymarketPerpsClient {
    state: Arc<ClientState<PolymarketPerpsClientResources>>,
}
#[pymethods]
impl PolymarketPerpsClient {
    #[new]
    #[pyo3(signature=(*,network="mainnet",endpoints=None,websocket=None,signer=None,proxy_signer=None,proxy=None,secret=None,expires_at_ms=None,withdrawal_contract=None,timeout_ms=10000,subscription_capacity=128))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        py: Python<'_>,
        network: &str,
        endpoints: Option<Bound<'_, PyAny>>,
        websocket: Option<Bound<'_, PyAny>>,
        signer: Option<PyRef<'_, PolymarketSigner>>,
        proxy_signer: Option<PyRef<'_, PolymarketSigner>>,
        proxy: Option<String>,
        secret: Option<String>,
        expires_at_ms: Option<u64>,
        withdrawal_contract: Option<String>,
        timeout_ms: u64,
        subscription_capacity: usize,
    ) -> PyResult<Self> {
        let mut exchange = match network {
            "mainnet" | "production" => v::Polymarket::mainnet(),
            _ => {
                return Err(errors::ValidationError::new_err(
                    "Polymarket requires mainnet or explicit endpoint overrides",
                ));
            }
        };
        if let Some(e) = endpoints {
            let values: std::collections::BTreeMap<String, String> = convert::parse(&e)?;
            for (k, u) in values {
                exchange = exchange.custom(&k, &u, None).map_err(local)?;
            }
        }
        if let Some(w) = websocket {
            let values: std::collections::BTreeMap<String, String> = convert::parse(&w)?;
            for (k, u) in values {
                v::validate_url(&u, true).map_err(local)?;
                if !exchange.websocket.contains_key(&k) {
                    return Err(errors::ValidationError::new_err(
                        "unknown WebSocket service",
                    ));
                }
                exchange.websocket.insert(k, u);
            }
        }
        let owner = signer.as_ref().map(|s| s.inner.clone());
        let auth = {
            let credentials = match (proxy, secret) {
                (Some(p), Some(s)) => Some(
                    v::PerpsCredentials::new(
                        p.parse().map_err(|_| {
                            errors::ValidationError::new_err("invalid proxy address")
                        })?,
                        Zeroizing::new(s),
                        expires_at_ms,
                    )
                    .map_err(local)?,
                ),
                (None, None) => None,
                _ => {
                    return Err(errors::ValidationError::new_err(
                        "proxy and secret are required together",
                    ));
                }
            };
            let auth = v::PolymarketPerpsAuth {
                owner,
                proxy: proxy_signer.map(|s| s.inner.clone()),
                credentials,
                withdrawal_contract: withdrawal_contract
                    .map(|s| {
                        s.parse().map_err(|_| {
                            errors::ValidationError::new_err("invalid withdrawal contract")
                        })
                    })
                    .transpose()?,
            };
            if auth.owner.is_none() && auth.proxy.is_none() && auth.credentials.is_none() {
                Auth::None
            } else {
                Auth::Perps(auth)
            }
        };
        let config = aux_rs::TransportConfig {
            timeout: std::time::Duration::from_millis(timeout_ms),
            subscription_capacity,
            ..Default::default()
        };
        let client = state::build(py, || {
            v::PolymarketPerpsClient::with_config(exchange, config)
        })?
        .with_auth(auth.clone());
        Ok(Self {
            state: ClientState::new(PolymarketPerpsClientResources { client, auth }),
        })
    }
    #[getter]
    fn closed(&self) -> bool {
        self.state.is_closed()
    }
    #[getter]
    fn authenticated(&self) -> PyResult<bool> {
        Ok(!matches!(self.state.client()?.auth, Auth::None))
    }
    fn subscribe<'py>(
        &self,
        py: Python<'py>,
        specification: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let specification: v::Subscription = convert::parse(&specification)?;
        let client = self.state.client()?.scoped(py)?;
        self.state.stream(py, async move {
            client
                .subscribe(specification)
                .await
                .map(NativeHandle::Polymarket)
                .map_err(native)
        })
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.close(py)
    }
    fn __repr__(&self) -> &'static str {
        "PolymarketPerpsClient()"
    }
}
impl Drop for PolymarketPerpsClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
mod polymarket_perps_client_operations;

mod chain;
pub use chain::PolymarketChainClient;

mod rpc;
pub use rpc::PolymarketRpcSession;
#[pyfunction]
pub fn polymarket_encode_call(
    contract: &str,
    function: &str,
    args: Bound<'_, PyAny>,
) -> PyResult<String> {
    v::encode_contract_call(contract, function, &convert::from_python(&args)?)
        .map(|bytes| bytes.to_string())
        .map_err(local)
}
#[pyfunction]
pub fn polymarket_helper(operation: &str, payload: Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    let p = convert::from_python(&payload)?;
    let required = |k: &str| {
        p[k].as_str()
            .ok_or_else(|| errors::ValidationError::new_err("missing helper string parameter"))
    };
    let v = match operation {
        "market_asset_id" => serde_json::json!(
            v::market_asset_id(&p["market"], required("outcome")?).map_err(local)?
        ),
        "market_order_domain" => {
            serde_json::to_value(v::market_order_domain(&p["market"]).map_err(local)?)
                .map_err(|_| errors::ValidationError::new_err("invalid domain"))?
        }
        "wallet_auth_typed_data" => v::wallet_auth_typed_data(
            v::integer(&p["chain_id"]).map_err(local)?,
            required("address")?
                .parse()
                .map_err(|_| errors::ValidationError::new_err("invalid address"))?,
            v::integer(&p["timestamp"]).map_err(local)?,
            v::integer(&p["nonce"]).map_err(local)?,
        ),
        "perps_op_typed_data" => v::perps_op_typed_data(
            v::integer(&p["chain_id"]).map_err(local)?,
            &p["operation"],
            v::integer(&p["salt"]).map_err(local)?,
            v::integer(&p["timestamp"]).map_err(local)?,
        )
        .map_err(local)?,
        "deployments" => v::deployments(),
        "deposit_wallet_batch_typed_data" => {
            let chain = v::integer(&p["chain_id"]).map_err(local)?;
            let wallet = required("wallet")?
                .parse()
                .map_err(|_| errors::ValidationError::new_err("invalid wallet"))?;
            let integer = |key: &str| {
                p[key]
                    .as_str()
                    .map(str::to_owned)
                    .unwrap_or_else(|| p[key].to_string())
                    .parse::<v::U256>()
                    .map_err(|_| errors::ValidationError::new_err("invalid exact integer"))
            };
            let calls: Vec<v::ContractCall> =
                serde_json::from_value(crate::metric::wide_integers(p["calls"].clone()))
                    .map_err(|_| errors::ValidationError::new_err("invalid contract calls"))?;
            v::deposit_wallet_batch_typed_data(
                chain,
                wallet,
                integer("nonce")?,
                integer("deadline")?,
                &calls,
            )
            .map_err(local)?
        }
        _ => {
            return Err(errors::ValidationError::new_err(
                "unknown Polymarket helper",
            ));
        }
    };
    convert::output(v)
}

//! Read an EVM chain over JSON-RPC, optionally with an application-owned TLS policy.
use crate::{
    Result,
    http::{self, OutboundRequest, Transport},
};
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::{Number, Value, json};
use std::{collections::BTreeMap, sync::Arc};

#[derive(Clone)]
pub struct EvmReader {
    endpoint: String,
    transport: Option<Arc<dyn Transport>>,
}

#[derive(Deserialize)]
struct Prepared {
    id: String,
    url: String,
    body: String,
}

impl EvmReader {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            transport: None,
        }
    }
    pub fn with_transport(mut self, transport: impl Transport + 'static) -> Self {
        self.transport = Some(Arc::new(transport));
        self
    }
    fn read<T: DeserializeOwned>(&self, operation: &str, params: Value) -> Result<T> {
        let args = json!({"operation":operation,"endpoint":self.endpoint,"params":params});
        if let Some(transport) = &self.transport {
            let req: Prepared = http::call(25, args)?;
            let response = transport.send(OutboundRequest {
                method: "POST".into(),
                url: req.url,
                headers: BTreeMap::from([("content-type".into(), "application/json".into())]),
                body: Some(req.body),
            })?;
            http::call(
                26,
                json!({"operation":operation,"id":req.id,"status":response.status,"body":response.body}),
            )
        } else {
            http::call(27, args)
        }
    }
    pub fn chain_id(&self) -> Result<u64> {
        self.read("chain_id", json!({}))
    }
    pub fn code(&self, address: &str, block: &str) -> Result<String> {
        self.read("code", json!({"address": address, "block": block}))
    }
    pub fn eth_call(
        &self,
        to: &str,
        data: &str,
        block: &str,
        sender: Option<&str>,
        value: &str,
    ) -> Result<String> {
        self.read(
            "eth_call",
            json!({"to": to, "data": data, "block": block, "from": sender, "value": value}),
        )
    }
    pub fn logs(&self, filter: &Value) -> Result<Vec<Value>> {
        self.read("logs", json!({"filter": filter}))
    }
    pub fn transaction_count(&self, address: &str) -> Result<u64> {
        self.read("transaction_count", json!({"address": address}))
    }
    pub fn receipt(&self, hash: &str) -> Result<Option<Value>> {
        self.read("receipt", json!({"hash": hash}))
    }
    pub fn transaction(&self, hash: &str) -> Result<Option<Value>> {
        self.read("transaction", json!({"hash": hash}))
    }
    pub fn estimate_gas(&self, request: &Value) -> Result<u64> {
        self.read("estimate_gas", json!({"request": request}))
    }
    pub fn block_number(&self) -> Result<u64> {
        self.read("block_number", json!({}))
    }
    pub fn block(&self, block: &str) -> Result<Option<Value>> {
        self.read("block", json!({"block": block}))
    }
    pub fn native_balance(&self, address: &str, block: &str) -> Result<Number> {
        self.read(
            "native_balance",
            json!({"address": address, "block": block}),
        )
    }
    pub fn gas_price(&self) -> Result<Number> {
        self.read("gas_price", json!({}))
    }
}

/// Coordinate writes by chain and sender across clients in this process.
#[derive(Clone)]
pub struct EvmChainClient {
    chain_id: u64,
    read_rpc: String,
    write_rpc: String,
    code_hashes: BTreeMap<String, String>,
}
impl EvmChainClient {
    pub fn new(chain_id: u64, read_rpc: impl Into<String>, write_rpc: impl Into<String>) -> Self {
        Self {
            chain_id,
            read_rpc: read_rpc.into(),
            write_rpc: write_rpc.into(),
            code_hashes: Default::default(),
        }
    }
    pub fn with_code_hash(mut self, address: impl Into<String>, hash: impl Into<String>) -> Self {
        self.code_hashes.insert(address.into(), hash.into());
        self
    }
    pub fn reader(&self) -> EvmReader {
        EvmReader::new(&self.read_rpc)
    }
    fn call<T: DeserializeOwned>(&self, operation: &str, params: Value) -> Result<T> {
        http::call(
            38,
            json!({"operation":operation,"chain_id":self.chain_id,"read_rpc":self.read_rpc,"write_rpc":self.write_rpc,"code_hashes":self.code_hashes,"params":params}),
        )
    }
    pub fn validate_chain(&self) -> Result<()> {
        self.call("validate_chain", json!({}))
    }
    pub fn send_transaction(&self, prepared: &Value, key: &str) -> Result<String> {
        self.call("send_transaction", json!({"prepared":prepared,"key":key}))
    }
    /// Signs outside the native nonce coordinator, then rechecks the chain, contract and nonce before sending.
    pub fn send_transaction_with_signer(
        &self,
        prepared: &Value,
        address: &str,
        sign: impl FnOnce(&str) -> Result<String>,
    ) -> Result<String> {
        let mut args = json!({"prepared":prepared, "address":address});
        let digest: String = self.call("prepare_external", args.clone())?;
        args["signature"] = sign(&digest)?.into();
        self.call("send_external", args)
    }
    pub fn submit_raw(&self, signed: &str) -> Result<String> {
        self.call("submit_raw", json!({"signed":signed}))
    }
    pub fn pending(&self, address: &str) -> Result<Option<PendingTransaction>> {
        self.call("pending", json!({"address":address}))
    }
    pub fn reconcile(&self, address: &str) -> Result<bool> {
        self.call("reconcile", json!({"address":address}))
    }
    pub fn resync_nonce(&self, address: &str, expected_pending_hash: &str) -> Result<u64> {
        self.call(
            "resync_nonce",
            json!({"address":address,"hash":expected_pending_hash}),
        )
    }
}
#[derive(Clone, Deserialize)]
pub struct PendingTransaction {
    pub hash: String,
    pub nonce: u64,
}

pub struct PolymarketChainClient(EvmChainClient);
impl std::ops::Deref for PolymarketChainClient {
    type Target = EvmChainClient;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl PolymarketChainClient {
    pub fn new(chain_id: u64, read_rpc: impl Into<String>, write_rpc: impl Into<String>) -> Self {
        Self(EvmChainClient::new(chain_id, read_rpc, write_rpc))
    }
    pub fn with_code_hash(self, address: impl Into<String>, hash: impl Into<String>) -> Self {
        Self(self.0.with_code_hash(address, hash))
    }
    pub fn prepare_call(
        &self,
        contract: &str,
        function: &str,
        to: &str,
        args: &Value,
        value: &str,
    ) -> Result<Value> {
        http::call(
            54,
            json!({"contract":contract,"function":function,"to":to,"args":args,"value":value}),
        )
    }
    pub fn call(
        &self,
        contract: &str,
        function: &str,
        to: &str,
        args: &Value,
        block: &str,
        sender: Option<&str>,
        value: &str,
    ) -> Result<Value> {
        use crate::contracts::{Venue, decode_contract_output, encode_contract_call};
        let data = encode_contract_call(Venue::Polymarket, contract, function, args)?;
        let result = self.reader().eth_call(to, &data, block, sender, value)?;
        decode_contract_output(Venue::Polymarket, contract, function, &result)
    }
}

pub struct MetricChainClient(EvmChainClient);
impl std::ops::Deref for MetricChainClient {
    type Target = EvmChainClient;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl MetricChainClient {
    pub fn new(chain_id: u64, read_rpc: impl Into<String>, write_rpc: impl Into<String>) -> Self {
        Self(EvmChainClient::new(chain_id, read_rpc, write_rpc))
    }
    pub fn with_code_hash(self, address: impl Into<String>, hash: impl Into<String>) -> Self {
        Self(self.0.with_code_hash(address, hash))
    }
    pub fn prepare_call(
        &self,
        contract: &str,
        function: &str,
        to: &str,
        args: &Value,
        value: &str,
    ) -> Result<Value> {
        http::call(
            52,
            json!({"contract":contract,"function":function,"to":to,"args":args,"value":value}),
        )
    }
    pub fn call(
        &self,
        contract: &str,
        function: &str,
        to: &str,
        args: &Value,
        block: &str,
        sender: Option<&str>,
        value: &str,
    ) -> Result<Value> {
        use crate::contracts::{Venue, decode_contract_output, encode_contract_call};
        let data = encode_contract_call(Venue::Metric, contract, function, args)?;
        let result = self.reader().eth_call(to, &data, block, sender, value)?;
        decode_contract_output(Venue::Metric, contract, function, &result)
    }
    fn metric_read(&self, op: &str, params: &Value) -> Result<Value> {
        crate::http::call(
            51,
            serde_json::json!({"operation":op,"endpoint":self.read_rpc,"params":params}),
        )
    }
    pub fn liquidity_snapshot(
        &self,
        pool: &str,
        lens: &str,
        bins: &[i16],
        block: &str,
    ) -> Result<Value> {
        self.metric_read(
            "liquidity_snapshot",
            &serde_json::json!({"pool":pool,"lens":lens,"bins":bins,"block":block}),
        )
    }
    pub fn oracle_stop_loss_status(&self, request: &Value) -> Result<Value> {
        self.metric_read("oracle_stop_loss_status", request)
    }
    pub fn prepare_route(&self, request: &Value) -> Result<Value> {
        self.metric_read("prepare_route", request)
    }
}

use crate::{Error, OrderDomain, PositionSystem};
use alloy_primitives::{Address, Bytes, U256};
use alloy_sol_types::SolCall;
pub use aux_rs_evm::{
    ChainConfig, Error as EvmError, EvmChainClient, PendingTransaction, PreparedTransaction,
    SignedTransaction,
};
use serde_json::Value;
pub type PolymarketChainClient<T = aux_rs_core::DefaultTransport> = EvmChainClient<T>;
/// Preserve native outcome order and the position system's decimal asset IDs.
pub fn market_asset_id(market: &Value, outcome: &str) -> Result<String, Error> {
    let outcomes = match &market["outcomes"] {
        Value::String(s) => serde_json::from_str::<Vec<String>>(s)?,
        v => serde_json::from_value(v.clone())?,
    };
    let index = outcomes
        .iter()
        .position(|s| s == outcome)
        .ok_or(Error::InvalidRequest("outcome not found in market"))?;
    let ids: Vec<String> = match market["version"].as_str() {
        Some("v1") => match &market["clobTokenIds"] {
            Value::String(s) => serde_json::from_str(s)?,
            v => serde_json::from_value(v.clone())?,
        },
        Some("v2") => serde_json::from_value(market["positionIds"].clone())?,
        _ => {
            return Err(Error::InvalidRequest(
                "unsupported or missing market version",
            ));
        }
    };
    if ids.len() != outcomes.len() {
        return Err(Error::InvalidRequest("outcome and asset ID lengths differ"));
    }
    let id = ids
        .get(index)
        .ok_or(Error::InvalidRequest("missing outcome asset ID"))?;
    if id.is_empty() || !id.bytes().all(|b| b.is_ascii_digit()) || id.parse::<U256>().is_err() {
        return Err(Error::InvalidRequest("invalid decimal asset ID"));
    }
    Ok(id.clone())
}
pub fn market_order_domain(market: &Value) -> Result<OrderDomain, Error> {
    let system = match market["version"].as_str() {
        Some("v1") => PositionSystem::Ctf,
        Some("v2") => PositionSystem::V2,
        _ => return Err(Error::InvalidRequest("unsupported market version")),
    };
    let negative = market
        .get("negRisk")
        .and_then(Value::as_bool)
        .ok_or(Error::InvalidRequest(
            "explicit negative-risk metadata required",
        ))?;
    Ok(OrderDomain::polygon(system, negative))
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ContractCall {
    pub to: Address,
    pub value: U256,
    pub data: Bytes,
}
impl ContractCall {
    pub fn new<C: SolCall>(to: Address, call: &C) -> Self {
        Self {
            to,
            value: U256::ZERO,
            data: call.abi_encode().into(),
        }
    }
}
pub fn deployments() -> Value {
    serde_json::from_str(include_str!("../schema/deployments.json")).expect("pinned deployments")
}
/// Owner or session signer authorization for an explicit Deposit Wallet call batch.
pub fn deposit_wallet_batch_typed_data(
    chain_id: u64,
    wallet: Address,
    nonce: U256,
    deadline: U256,
    calls: &[ContractCall],
) -> Result<Value, Error> {
    if chain_id == 0 || calls.is_empty() {
        return Err(Error::InvalidRequest(
            "positive chain and nonempty wallet batch required",
        ));
    }
    let payload = serde_json::json!({"domain":{"name":"DepositWallet","version":"1","chainId":chain_id,"verifyingContract":wallet},"primaryType":"Batch","types":{"EIP712Domain":[{"name":"name","type":"string"},{"name":"version","type":"string"},{"name":"chainId","type":"uint256"},{"name":"verifyingContract","type":"address"}],"Call":[{"name":"target","type":"address"},{"name":"value","type":"uint256"},{"name":"data","type":"bytes"}],"Batch":[{"name":"wallet","type":"address"},{"name":"nonce","type":"uint256"},{"name":"deadline","type":"uint256"},{"name":"calls","type":"Call[]"}]},"message":{"wallet":wallet,"nonce":nonce.to_string(),"deadline":deadline.to_string(),"calls":calls.iter().map(|c|serde_json::json!({"target":c.to,"value":c.value.to_string(),"data":c.data})).collect::<Vec<_>>()}});
    crate::typed_data_hash(&payload)?;
    Ok(payload)
}
pub fn wrap_session_signature(session_signer: Address, signature: &[u8]) -> Result<Bytes, Error> {
    use alloy_primitives::B256;
    use alloy_sol_types::SolValue;
    if signature.is_empty() {
        return Err(Error::InvalidRequest("session signature must not be empty"));
    }
    let mut word = [0u8; 32];
    word[12..].copy_from_slice(session_signer.as_slice());
    let mut out = (
        B256::from(word),
        B256::ZERO,
        Bytes::copy_from_slice(signature),
    )
        .abi_encode_params();
    out.extend(
        alloy_primitives::hex::decode(
            "6492649264926492649264926492649264926492649264926492649264926492",
        )
        .expect("magic bytes"),
    );
    Ok(out.into())
}

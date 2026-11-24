//! Pascal collateral transfer preparation, signing and Solana RPC.
use crate::{Result, http};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Serialize, Deserialize)]
pub struct DepositParams {
    pub owner: String,
    pub source: String,
    pub destination: String,
    pub mint: String,
    pub amount: u64,
    pub decimals: u8,
    pub recent_blockhash: String,
    pub last_valid_block_height: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PreparedDeposit {
    #[serde(flatten)]
    pub params: DepositParams,
    pub message: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct SignedDeposit {
    pub signature: String,
    pub transaction_base64: String,
    pub last_valid_block_height: u64,
}

pub fn associated_token_address(owner: &str, mint: &str) -> Result<String> {
    http::call(29, json!({ "owner": owner, "mint": mint }))
}
pub fn prepare_deposit(params: &DepositParams) -> Result<PreparedDeposit> {
    http::call(30, json!(params))
}
pub fn sign_deposit(seed: &str, prepared: &PreparedDeposit) -> Result<SignedDeposit> {
    http::call(31, json!({ "seed": seed, "prepared": prepared }))
}

/// In-process submission state is shared by clones of this client.
#[derive(Clone)]
pub struct PascalChainClient {
    config: Value,
}
impl PascalChainClient {
    pub fn new(endpoint: &str, mint: &str, decimals: u8) -> Result<Self> {
        let mut config =
            json!({"operation":"new", "endpoint":endpoint,"mint":mint,"decimals":decimals});
        config["client"] = http::call(37, config.clone())?;
        Ok(Self { config })
    }
    fn call<T: serde::de::DeserializeOwned>(&self, op: &str, params: Value) -> Result<T> {
        let mut input = self.config.clone();
        input["operation"] = op.into();
        input["params"] = params;
        http::call(37, input)
    }
    pub fn account_info(&self, address: &str) -> Result<Value> {
        self.call("account_info", json!({"address":address}))
    }
    pub fn token_balance(&self, address: &str) -> Result<Value> {
        self.call("token_balance", json!({"address":address}))
    }
    pub fn block_height(&self) -> Result<u64> {
        self.call("block_height", json!({}))
    }
    pub fn signature_status(&self, signature: &str) -> Result<Value> {
        self.call("signature_status", json!({"signature":signature}))
    }
    pub fn prepare_deposit(
        &self,
        owner: &str,
        source: &str,
        destination: &str,
        amount: u64,
    ) -> Result<PreparedDeposit> {
        self.call(
            "prepare_deposit",
            json!({"owner":owner,"source":source,"destination":destination,"amount":amount}),
        )
    }
    pub fn submit(&self, signed: &SignedDeposit) -> Result<String> {
        self.call("submit", json!(signed))
    }
    pub fn pending(&self) -> Result<Vec<String>> {
        self.call("pending", json!({}))
    }
    pub fn reconcile(&self, signature: &str) -> Result<Value> {
        self.call("reconcile", json!({"signature":signature}))
    }
    pub fn clear_reconciled_submission(&self, signature: &str) -> Result<()> {
        self.call(
            "clear_reconciled_submission",
            json!({"signature":signature}),
        )
    }
    pub fn reconcile_deposit_credit(
        &self,
        api: &crate::PascalClient,
        signature: &str,
        request: &Value,
    ) -> Result<Value> {
        let confirmation = self.reconcile(signature)?;
        let transfers = api.transfer_history(request)?;
        Ok(json!({"signature":signature,"confirmation":confirmation,"transfers":transfers}))
    }
}

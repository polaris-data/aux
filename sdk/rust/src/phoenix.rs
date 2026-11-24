//! Legacy and v0 Solana transactions prepared by the native Phoenix connector.
use crate::{Result, http};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;
pub type Signatures = BTreeMap<String, String>;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreparedTransaction {
    pub message_base64: String,
    pub required_signers: Vec<String>,
    pub recent_blockhash: String,
    pub last_valid_block_height: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedTransaction {
    pub transaction_base64: String,
    pub signature: String,
    pub last_valid_block_height: u64,
}
fn local<T: serde::de::DeserializeOwned>(op: &str, params: Value) -> Result<T> {
    http::call(47, json!({"operation":op,"params":params}))
}
impl PreparedTransaction {
    pub fn new(request: &Value) -> Result<Self> {
        local("prepare_transaction", request.clone())
    }
    pub fn sign(&self, key: &str, signatures: &Signatures) -> Result<Signatures> {
        local(
            "sign_transaction",
            json!({"key":key,"prepared":self,"signatures":signatures}),
        )
    }
    pub fn assemble(&self, signatures: &Signatures) -> Result<SignedTransaction> {
        local(
            "assemble_transaction",
            json!({"prepared":self,"signatures":signatures}),
        )
    }
    pub fn validate_server_transaction(&self, transaction_base64: &str) -> Result<()> {
        local(
            "validate_server_transaction",
            json!({"prepared":self,"transaction_base64":transaction_base64}),
        )
    }
}
impl SignedTransaction {
    pub fn validate(&self) -> Result<()> {
        local("validate_signed_transaction", json!(self))
    }
}
impl crate::PhoenixClient {
    pub fn build_instruction(&self, name: &str, request: &Value) -> Result<Value> {
        local("build_instruction", json!({"name":name,"request":request}))
    }

    pub fn prepare_transaction(&self, request: &Value) -> Result<PreparedTransaction> {
        PreparedTransaction::new(request)
    }

    pub fn assemble_transaction(
        &self,
        prepared: &PreparedTransaction,
        signatures: &Signatures,
    ) -> Result<SignedTransaction> {
        prepared.assemble(signatures)
    }

    pub fn validate_server_transaction(
        &self,
        prepared: &PreparedTransaction,
        transaction_base64: &str,
    ) -> Result<()> {
        prepared.validate_server_transaction(transaction_base64)
    }
}
#[derive(Clone)]
pub struct PhoenixChainClient {
    config: Value,
}
impl PhoenixChainClient {
    pub fn new(endpoint: &str) -> Result<Self> {
        let id: String = http::call(48, json!({"endpoint":endpoint,"operation":"new"}))?;
        Ok(Self {
            config: json!({"client":id,"endpoint":endpoint}),
        })
    }
    fn call<T: serde::de::DeserializeOwned>(&self, op: &str, params: Value) -> Result<T> {
        let mut cfg = self.config.clone();
        cfg["operation"] = op.into();
        cfg["params"] = params;
        http::call(48, cfg)
    }
    pub fn latest_blockhash(&self) -> Result<Value> {
        self.call("latest_blockhash", json!({}))
    }
    pub fn block_height(&self) -> Result<u64> {
        self.call("block_height", json!({}))
    }
    pub fn prepare(&self, instructions: &Value, fee_payer: &str) -> Result<PreparedTransaction> {
        self.call(
            "prepare",
            json!({"instructions":instructions,"fee_payer":fee_payer}),
        )
    }
    pub fn simulate(&self, signed: &SignedTransaction) -> Result<Value> {
        self.call("simulate", json!(signed))
    }
    pub fn submit(&self, signed: &SignedTransaction) -> Result<String> {
        self.call("submit", json!(signed))
    }
    pub fn pending(&self) -> Result<Option<SignedTransaction>> {
        self.call("pending", json!({}))
    }
    pub fn clear_reconciled_submission(&self) -> Result<()> {
        self.call("clear_reconciled_submission", json!({}))
    }
    pub fn signature_status(&self, signature: &str) -> Result<Value> {
        self.call("signature_status", json!({"signature":signature}))
    }
    pub fn transaction(&self, signature: &str) -> Result<Value> {
        self.call("transaction", json!({"signature":signature}))
    }
}

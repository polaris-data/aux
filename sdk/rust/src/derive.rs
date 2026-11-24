//! Derive Matching module actions. Monetary amounts use decimal strings.
use crate::{Result, http::call};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SigningLeg {
    pub instrument_name: String,
    pub asset_address: String,
    pub sub_id: String,
    pub price: String,
    pub amount: String,
    pub direction: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ActionData {
    Trade {
        asset: String,
        sub_id: String,
        limit_price: String,
        amount: String,
        max_fee: String,
        recipient_id: String,
        is_bid: bool,
    },
    Transfer {
        to_subaccount_id: String,
        new_subaccount_manager: String,
        asset: String,
        sub_id: String,
        amount: String,
        max_fee_usd: String,
        recipient: Option<String>,
    },
    Withdrawal {
        asset: String,
        max_fee_usd: String,
        recipient: String,
        amount: String,
        decimals: u8,
        force_batch: bool,
    },
    SessionKey {
        session_key: String,
        expiry_sec: u64,
        scopes: Vec<u8>,
        subaccount_ids: Vec<String>,
    },
    WhitelistedRecipients {
        add: Vec<String>,
        remove: Vec<String>,
    },
    RfqQuote {
        max_fee: String,
        direction: String,
        legs: Vec<SigningLeg>,
    },
    RfqExecute {
        max_fee: String,
        direction: String,
        legs: Vec<SigningLeg>,
    },
    VaultCreate {
        manager_id: String,
        deposit_spot_asset: String,
        initial_deposit: String,
        management_fee_bps: u32,
        performance_fee_bps: u32,
        max_slippage_bps: u32,
        cooldown_sec: u32,
        max_fee_usd: String,
        initial_share_price_usd: String,
        benchmark_asset: Option<String>,
    },
    VaultDeposit {
        vault_subaccount_id: String,
        deposit_spot_asset: String,
        amount: String,
    },
    VaultWithdraw {
        vault_subaccount_id: String,
        shares_to_burn: String,
    },
    VaultCancel {
        vault_subaccount_id: String,
    },
    VaultMint {
        share_price: String,
        deposit_hash: String,
    },
    VaultBurn {
        share_price: String,
        withdraw_hash: String,
    },
    VaultForceBurn {
        holder: String,
    },
    DeleteSubaccount,
    Encoded {
        module: String,
        data: Vec<u8>,
    },
    Liquidation {
        liquidate_subaccount_id: String,
        percent_of_acc: String,
        price_limit: String,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionPayload {
    pub subaccount_id: String,
    pub nonce: String,
    pub expiry: u64,
    pub owner: String,
    pub signer: String,
    pub module: String,
    pub encoded_data: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Action {
    pub subaccount_id: String,
    pub nonce: String,
    pub expiry: u64,
    pub owner: String,
    pub chain_id: u64,
    pub data: ActionData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignedAction {
    pub module: String,
    pub encoded: String,
    pub digest: String,
    pub signature: String,
    pub signer: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginProof {
    pub wallet: String,
    pub timestamp: String,
    pub signature: String,
}

pub fn sign_action(key: &str, action: &Action) -> Result<SignedAction> {
    let mut args = json!(action);
    args["key"] = json!(key);
    call(24, args)
}

pub fn login(key: &str, owner: &str, timestamp: u64) -> Result<LoginProof> {
    call(23, json!({"key":key,"owner":owner,"timestamp":timestamp}))
}

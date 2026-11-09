//! Withdraw funds instruction construction.
//!
//! This module provides instruction building for withdrawing Phoenix tokens
//! from the Phoenix protocol.

use solana_pubkey::Pubkey;

use crate::instructions::constants::{
    PHOENIX_GLOBAL_CONFIGURATION, PHOENIX_LOG_AUTHORITY, PHOENIX_PROGRAM_ID, SPL_TOKEN_PROGRAM_ID,
};
use crate::instructions::error::PhoenixIxError;
use crate::instructions::types::{AccountMeta, Instruction, push_trader_index_accounts};

/// Parameters for withdrawing Phoenix tokens from the protocol.
#[derive(Debug, Clone)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawFundsParams {
    /// The trader's authority (wallet) - must sign.
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader: Pubkey,
    /// The trader's PDA account.
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader_account: Pubkey,
    /// The perp asset map account.
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    perp_asset_map: Pubkey,
    /// The global vault (Phoenix protocol vault for the mint).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    global_vault: Pubkey,
    /// The trader's token account (ATA for Phoenix tokens) - destination.
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader_token_account: Pubkey,
    /// Global trader index addresses (header + arenas).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    global_trader_index: Vec<Pubkey>,
    /// Active trader buffer addresses (header + arenas).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    active_trader_buffer: Vec<Pubkey>,
    /// The withdraw queue account.
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    withdraw_queue: Pubkey,
    /// Amount to withdraw in token base units.
    amount: u64,
}

impl WithdrawFundsParams {
    /// Start building with the builder pattern.
    pub fn builder() -> WithdrawFundsParamsBuilder {
        WithdrawFundsParamsBuilder::new()
    }

    pub fn trader(&self) -> Pubkey {
        self.trader
    }

    pub fn trader_account(&self) -> Pubkey {
        self.trader_account
    }

    pub fn perp_asset_map(&self) -> Pubkey {
        self.perp_asset_map
    }

    pub fn global_vault(&self) -> Pubkey {
        self.global_vault
    }

    pub fn trader_token_account(&self) -> Pubkey {
        self.trader_token_account
    }

    pub fn global_trader_index(&self) -> &[Pubkey] {
        &self.global_trader_index
    }

    pub fn active_trader_buffer(&self) -> &[Pubkey] {
        &self.active_trader_buffer
    }

    pub fn withdraw_queue(&self) -> Pubkey {
        self.withdraw_queue
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }
}

/// Builder for `WithdrawFundsParams`.
#[derive(Default)]
pub struct WithdrawFundsParamsBuilder {
    trader: Option<Pubkey>,
    trader_account: Option<Pubkey>,
    perp_asset_map: Option<Pubkey>,
    global_vault: Option<Pubkey>,
    trader_token_account: Option<Pubkey>,
    global_trader_index: Option<Vec<Pubkey>>,
    active_trader_buffer: Option<Vec<Pubkey>>,
    withdraw_queue: Option<Pubkey>,
    amount: Option<u64>,
}

impl WithdrawFundsParamsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn trader(mut self, trader: Pubkey) -> Self {
        self.trader = Some(trader);
        self
    }

    pub fn trader_account(mut self, trader_account: Pubkey) -> Self {
        self.trader_account = Some(trader_account);
        self
    }

    pub fn perp_asset_map(mut self, perp_asset_map: Pubkey) -> Self {
        self.perp_asset_map = Some(perp_asset_map);
        self
    }

    pub fn global_vault(mut self, global_vault: Pubkey) -> Self {
        self.global_vault = Some(global_vault);
        self
    }

    pub fn trader_token_account(mut self, trader_token_account: Pubkey) -> Self {
        self.trader_token_account = Some(trader_token_account);
        self
    }

    pub fn global_trader_index(mut self, global_trader_index: Vec<Pubkey>) -> Self {
        self.global_trader_index = Some(global_trader_index);
        self
    }

    pub fn active_trader_buffer(mut self, active_trader_buffer: Vec<Pubkey>) -> Self {
        self.active_trader_buffer = Some(active_trader_buffer);
        self
    }

    pub fn withdraw_queue(mut self, withdraw_queue: Pubkey) -> Self {
        self.withdraw_queue = Some(withdraw_queue);
        self
    }

    pub fn amount(mut self, amount: u64) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn build(self) -> Result<WithdrawFundsParams, PhoenixIxError> {
        let amount = self.amount.ok_or(PhoenixIxError::MissingField("amount"))?;
        if amount == 0 {
            return Err(PhoenixIxError::InvalidWithdrawAmount);
        }

        let global_trader_index = self
            .global_trader_index
            .ok_or(PhoenixIxError::MissingField("global_trader_index"))?;
        if global_trader_index.is_empty() {
            return Err(PhoenixIxError::EmptyGlobalTraderIndex);
        }

        let active_trader_buffer = self
            .active_trader_buffer
            .ok_or(PhoenixIxError::MissingField("active_trader_buffer"))?;
        if active_trader_buffer.is_empty() {
            return Err(PhoenixIxError::EmptyActiveTraderBuffer);
        }

        Ok(WithdrawFundsParams {
            trader: self.trader.ok_or(PhoenixIxError::MissingField("trader"))?,
            trader_account: self
                .trader_account
                .ok_or(PhoenixIxError::MissingField("trader_account"))?,
            perp_asset_map: self
                .perp_asset_map
                .ok_or(PhoenixIxError::MissingField("perp_asset_map"))?,
            global_vault: self
                .global_vault
                .ok_or(PhoenixIxError::MissingField("global_vault"))?,
            trader_token_account: self
                .trader_token_account
                .ok_or(PhoenixIxError::MissingField("trader_token_account"))?,
            global_trader_index,
            active_trader_buffer,
            withdraw_queue: self
                .withdraw_queue
                .ok_or(PhoenixIxError::MissingField("withdraw_queue"))?,
            amount,
        })
    }
}

/// Create a withdraw funds instruction.
///
/// This instruction withdraws Phoenix tokens from the Phoenix protocol
/// to the trader's token account.
///
/// # Arguments
///
/// * `params` - The withdraw funds parameters
///
/// # Returns
///
/// A Solana instruction ready to be included in a transaction.
///
/// # Errors
///
/// Returns an error if required parameters are missing, amount is zero,
/// or trader index arrays are empty.
pub fn create_withdraw_funds_ix(
    params: WithdrawFundsParams,
) -> Result<Instruction, PhoenixIxError> {
    let data = encode_withdraw_funds(&params);
    let accounts = build_accounts(&params);

    Ok(Instruction {
        program_id: *PHOENIX_PROGRAM_ID,
        accounts,
        data,
    })
}

fn encode_withdraw_funds(params: &WithdrawFundsParams) -> Vec<u8> {
    let mut data = Vec::with_capacity(16);

    // Instruction discriminant (8 bytes)
    data.extend_from_slice(&crate::instructions::PhoenixInstruction::WithdrawFunds.discriminant());

    // Amount (8 bytes, little-endian u64)
    data.extend_from_slice(&params.amount().to_le_bytes());

    data
}

fn build_accounts(params: &WithdrawFundsParams) -> Vec<AccountMeta> {
    let mut accounts = Vec::new();

    // 1. phoenix_program (readonly) - Log accounts
    accounts.push(AccountMeta::readonly(*PHOENIX_PROGRAM_ID));
    // 2. PHOENIX_LOG_AUTHORITY (readonly)
    accounts.push(AccountMeta::readonly(*PHOENIX_LOG_AUTHORITY));
    // 3. global_configuration_account (writable)
    accounts.push(AccountMeta::writable(*PHOENIX_GLOBAL_CONFIGURATION));
    // 4. trader_wallet (signer, readonly)
    accounts.push(AccountMeta::readonly_signer(params.trader()));
    // 5. trader_account (writable) - Trader PDA
    accounts.push(AccountMeta::writable(params.trader_account()));
    // 6. perp_asset_map (writable)
    accounts.push(AccountMeta::writable(params.perp_asset_map()));
    // 7. global_vault (writable)
    accounts.push(AccountMeta::writable(params.global_vault()));
    // 8. destination_token_account (writable) - Owner's Phoenix token ATA
    accounts.push(AccountMeta::writable(params.trader_token_account()));
    // 9. token_program (readonly)
    accounts.push(AccountMeta::readonly(SPL_TOKEN_PROGRAM_ID));

    push_trader_index_accounts(
        &mut accounts,
        params.global_trader_index(),
        params.active_trader_buffer(),
    );

    // M+1. withdraw_queue (writable)
    accounts.push(AccountMeta::writable(params.withdraw_queue()));

    accounts
}

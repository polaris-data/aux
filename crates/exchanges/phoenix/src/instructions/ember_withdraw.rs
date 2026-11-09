//! Ember withdraw instruction construction.
//!
//! This module provides instruction building for withdrawing Phoenix tokens
//! and receiving USDC via the Ember program.

use solana_pubkey::Pubkey;

use crate::instructions::EmberInstruction;
use crate::instructions::constants::{
    EMBER_PROGRAM_ID, SPL_TOKEN_PROGRAM_ID, get_ember_state_address, get_ember_vault_address,
};
use crate::instructions::error::PhoenixIxError;
use crate::instructions::types::{AccountMeta, Instruction};

/// Parameters for withdrawing Phoenix tokens and receiving USDC via Ember.
#[derive(Debug, Clone)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct EmberWithdrawParams {
    /// The trader's authority (wallet) - must sign.
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader: Pubkey,
    /// USDC mint (input - what user receives).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    usdc_mint: Pubkey,
    /// Phoenix token mint (output - what user gives).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    canonical_mint: Pubkey,
    /// Trader's USDC token account (destination).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader_usdc_account: Pubkey,
    /// Trader's Phoenix token account (source).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader_phoenix_account: Pubkey,
    /// Amount of Phoenix tokens to withdraw (None = full withdrawal).
    amount: Option<u64>,
}

impl EmberWithdrawParams {
    /// Start building with the builder pattern.
    pub fn builder() -> EmberWithdrawParamsBuilder {
        EmberWithdrawParamsBuilder::new()
    }

    pub fn trader(&self) -> Pubkey {
        self.trader
    }

    pub fn usdc_mint(&self) -> Pubkey {
        self.usdc_mint
    }

    pub fn canonical_mint(&self) -> Pubkey {
        self.canonical_mint
    }

    pub fn trader_usdc_account(&self) -> Pubkey {
        self.trader_usdc_account
    }

    pub fn trader_phoenix_account(&self) -> Pubkey {
        self.trader_phoenix_account
    }

    pub fn amount(&self) -> Option<u64> {
        self.amount
    }
}

/// Builder for `EmberWithdrawParams`.
#[derive(Default)]
pub struct EmberWithdrawParamsBuilder {
    trader: Option<Pubkey>,
    usdc_mint: Option<Pubkey>,
    canonical_mint: Option<Pubkey>,
    trader_usdc_account: Option<Pubkey>,
    trader_phoenix_account: Option<Pubkey>,
    amount: Option<Option<u64>>,
}

impl EmberWithdrawParamsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn trader(mut self, trader: Pubkey) -> Self {
        self.trader = Some(trader);
        self
    }

    pub fn usdc_mint(mut self, usdc_mint: Pubkey) -> Self {
        self.usdc_mint = Some(usdc_mint);
        self
    }

    pub fn canonical_mint(mut self, canonical_mint: Pubkey) -> Self {
        self.canonical_mint = Some(canonical_mint);
        self
    }

    pub fn trader_usdc_account(mut self, trader_usdc_account: Pubkey) -> Self {
        self.trader_usdc_account = Some(trader_usdc_account);
        self
    }

    pub fn trader_phoenix_account(mut self, trader_phoenix_account: Pubkey) -> Self {
        self.trader_phoenix_account = Some(trader_phoenix_account);
        self
    }

    /// Set the withdrawal amount. Pass `Some(amount)` for a specific amount,
    /// or `None` for a full withdrawal.
    pub fn amount(mut self, amount: Option<u64>) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn build(self) -> Result<EmberWithdrawParams, PhoenixIxError> {
        let amount = self.amount.ok_or(PhoenixIxError::MissingField("amount"))?;

        // Validate amount if specified
        if let Some(amt) = amount {
            if amt == 0 {
                return Err(PhoenixIxError::InvalidWithdrawAmount);
            }
        }

        Ok(EmberWithdrawParams {
            trader: self.trader.ok_or(PhoenixIxError::MissingField("trader"))?,
            usdc_mint: self
                .usdc_mint
                .ok_or(PhoenixIxError::MissingField("usdc_mint"))?,
            canonical_mint: self
                .canonical_mint
                .ok_or(PhoenixIxError::MissingField("canonical_mint"))?,
            trader_usdc_account: self
                .trader_usdc_account
                .ok_or(PhoenixIxError::MissingField("trader_usdc_account"))?,
            trader_phoenix_account: self
                .trader_phoenix_account
                .ok_or(PhoenixIxError::MissingField("trader_phoenix_account"))?,
            amount,
        })
    }
}

/// Create an Ember withdraw instruction.
///
/// This instruction burns Phoenix tokens and releases USDC from the Ember vault
/// to the trader's account.
///
/// # Arguments
///
/// * `params` - The Ember withdraw parameters
///
/// # Returns
///
/// A Solana instruction ready to be included in a transaction.
///
/// # Errors
///
/// Returns an error if required parameters are missing or amount is zero.
pub fn create_ember_withdraw_ix(
    params: EmberWithdrawParams,
) -> Result<Instruction, PhoenixIxError> {
    let data = encode_ember_withdraw(&params);
    let accounts = build_accounts(&params)?;

    Ok(Instruction {
        program_id: EMBER_PROGRAM_ID,
        accounts,
        data,
    })
}

fn encode_ember_withdraw(params: &EmberWithdrawParams) -> Vec<u8> {
    // Capacity: 8 (discriminant) + 1 (Option tag) + 8 (amount if Some) = 17 max
    let mut data = Vec::with_capacity(17);

    // Instruction discriminant (8 bytes)
    data.extend_from_slice(&EmberInstruction::Withdraw.discriminant());

    // Amount as Borsh-encoded Option<u64>
    match params.amount() {
        Some(amount) => {
            data.push(1); // Some variant tag
            data.extend_from_slice(&amount.to_le_bytes());
        }
        None => {
            data.push(0); // None variant tag
        }
    }

    data
}

fn build_accounts(params: &EmberWithdrawParams) -> Result<Vec<AccountMeta>, PhoenixIxError> {
    Ok(vec![
        // 1. owner (signer, readonly)
        AccountMeta::readonly_signer(params.trader()),
        // 2. ember_state (readonly)
        AccountMeta::readonly(get_ember_state_address()?),
        // 3. input_mint (readonly) - USDC
        AccountMeta::readonly(params.usdc_mint()),
        // 4. output_mint (writable) - Phoenix token
        AccountMeta::writable(params.canonical_mint()),
        // 5. input_token_account (writable) - owner's USDC ATA
        AccountMeta::writable(params.trader_usdc_account()),
        // 6. output_token_account (writable) - owner's Phoenix token ATA
        AccountMeta::writable(params.trader_phoenix_account()),
        // 7. ember_vault (writable)
        AccountMeta::writable(get_ember_vault_address()?),
        // 8. spl_token (readonly)
        AccountMeta::readonly(SPL_TOKEN_PROGRAM_ID),
    ])
}

//! Claim protocol fees instruction construction.
//!
//! This is the risk-authority path for moving accrued protocol fees from the
//! Phoenix global vault to a destination token account. Flight builder fees are
//! transferred to the builder's trader account as collateral and should be
//! withdrawn with [`crate::instructions::create_withdraw_funds_ix`] instead.

use solana_pubkey::Pubkey;

use crate::instructions::constants::{
    PHOENIX_GLOBAL_CONFIGURATION, PHOENIX_LOG_AUTHORITY, PHOENIX_PROGRAM_ID, SPL_TOKEN_PROGRAM_ID,
};
use crate::instructions::error::PhoenixIxError;
use crate::instructions::types::{AccountMeta, Instruction};

/// Parameters for claiming accrued protocol fees.
#[derive(Debug, Clone)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct ClaimFeesParams {
    /// Risk authority wallet. Must sign the transaction.
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    authority: Pubkey,
    /// Phoenix global vault for the canonical token mint.
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    global_vault: Pubkey,
    /// Token account receiving the claimed fees.
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    destination: Pubkey,
    /// Amount to claim in canonical token base units.
    amount: u64,
}

impl ClaimFeesParams {
    /// Start building with the builder pattern.
    pub fn builder() -> ClaimFeesParamsBuilder {
        ClaimFeesParamsBuilder::new()
    }

    pub fn authority(&self) -> Pubkey {
        self.authority
    }

    pub fn global_vault(&self) -> Pubkey {
        self.global_vault
    }

    pub fn destination(&self) -> Pubkey {
        self.destination
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }
}

/// Builder for [`ClaimFeesParams`].
#[derive(Default)]
pub struct ClaimFeesParamsBuilder {
    authority: Option<Pubkey>,
    global_vault: Option<Pubkey>,
    destination: Option<Pubkey>,
    amount: Option<u64>,
}

impl ClaimFeesParamsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn authority(mut self, authority: Pubkey) -> Self {
        self.authority = Some(authority);
        self
    }

    pub fn global_vault(mut self, global_vault: Pubkey) -> Self {
        self.global_vault = Some(global_vault);
        self
    }

    pub fn destination(mut self, destination: Pubkey) -> Self {
        self.destination = Some(destination);
        self
    }

    pub fn amount(mut self, amount: u64) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn build(self) -> Result<ClaimFeesParams, PhoenixIxError> {
        Ok(ClaimFeesParams {
            authority: self
                .authority
                .ok_or(PhoenixIxError::MissingField("authority"))?,
            global_vault: self
                .global_vault
                .ok_or(PhoenixIxError::MissingField("global_vault"))?,
            destination: self
                .destination
                .ok_or(PhoenixIxError::MissingField("destination"))?,
            amount: self.amount.ok_or(PhoenixIxError::MissingField("amount"))?,
        })
    }
}

/// Create a Phoenix `claim_fees` instruction.
pub fn create_claim_fees_ix(params: ClaimFeesParams) -> Result<Instruction, PhoenixIxError> {
    let mut data = Vec::with_capacity(16);
    data.extend_from_slice(&crate::instructions::PhoenixInstruction::ClaimFees.discriminant());
    data.extend_from_slice(&params.amount().to_le_bytes());

    let accounts = vec![
        AccountMeta::readonly(*PHOENIX_PROGRAM_ID),
        AccountMeta::readonly(*PHOENIX_LOG_AUTHORITY),
        AccountMeta::readonly(*PHOENIX_GLOBAL_CONFIGURATION),
        AccountMeta::readonly_signer(params.authority()),
        AccountMeta::writable(params.global_vault()),
        AccountMeta::writable(params.destination()),
        AccountMeta::readonly(SPL_TOKEN_PROGRAM_ID),
    ];

    Ok(Instruction {
        program_id: *PHOENIX_PROGRAM_ID,
        accounts,
        data,
    })
}

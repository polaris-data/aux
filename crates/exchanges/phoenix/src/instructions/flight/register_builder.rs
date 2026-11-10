//! Flight: Register Builder instruction construction.
//!
//! Mirrors the TS SDK's `buildRegisterBuilderIx`
//! (`ts/src/flight/core/ixBuilders/RegisterBuilder`).

use solana_pubkey::Pubkey;

use crate::instructions::FlightInstruction;
use crate::instructions::constants::{PHOENIX_PROGRAM_ID, SYSTEM_PROGRAM_ID};
use crate::instructions::error::PhoenixIxError;
use crate::instructions::flight::constants::{
    FLIGHT_PROGRAM_ID, get_flight_builder_state_address, get_flight_global_state_address,
};
use crate::instructions::types::{AccountMeta, Instruction};

/// Parameters for registering a new builder account on the Flight program.
#[derive(Debug, Clone)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct RegisterBuilderParams {
    /// The builder's authority (readonly signer)
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader_authority: Pubkey,
    /// The builder's Phoenix trader subaccount PDA (writable)
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader_account: Pubkey,
    /// Phoenix trader PDA index for `trader_account` derivation (0-255)
    trader_pda_index: u8,
    /// Phoenix subaccount index for `trader_account` derivation. `0` for
    /// cross-margin, `1-100` for isolated margin
    subaccount_index: u8,
    /// Fee charged by this builder in basis points.
    fee_bps: u64,
}

impl RegisterBuilderParams {
    /// Start building with the builder pattern.
    pub fn builder() -> RegisterBuilderParamsBuilder {
        RegisterBuilderParamsBuilder::new()
    }

    pub fn trader_authority(&self) -> Pubkey {
        self.trader_authority
    }

    pub fn trader_account(&self) -> Pubkey {
        self.trader_account
    }

    pub fn trader_pda_index(&self) -> u8 {
        self.trader_pda_index
    }

    pub fn subaccount_index(&self) -> u8 {
        self.subaccount_index
    }

    pub fn fee_bps(&self) -> u64 {
        self.fee_bps
    }
}

/// Builder for `RegisterBuilderParams`.
///
/// `trader_pda_index` and `subaccount_index` default to 0 when not set,
/// matching the TS SDK's defaults.
#[derive(Default)]
pub struct RegisterBuilderParamsBuilder {
    trader_authority: Option<Pubkey>,
    trader_account: Option<Pubkey>,
    trader_pda_index: Option<u8>,
    subaccount_index: Option<u8>,
    fee_bps: Option<u64>,
}

impl RegisterBuilderParamsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn trader_authority(mut self, trader_authority: Pubkey) -> Self {
        self.trader_authority = Some(trader_authority);
        self
    }

    pub fn trader_account(mut self, trader_account: Pubkey) -> Self {
        self.trader_account = Some(trader_account);
        self
    }

    pub fn trader_pda_index(mut self, trader_pda_index: u8) -> Self {
        self.trader_pda_index = Some(trader_pda_index);
        self
    }

    pub fn subaccount_index(mut self, subaccount_index: u8) -> Self {
        self.subaccount_index = Some(subaccount_index);
        self
    }

    pub fn fee_bps(mut self, fee_bps: u64) -> Self {
        self.fee_bps = Some(fee_bps);
        self
    }

    pub fn build(self) -> Result<RegisterBuilderParams, PhoenixIxError> {
        Ok(RegisterBuilderParams {
            trader_authority: self
                .trader_authority
                .ok_or(PhoenixIxError::MissingField("trader_authority"))?,
            trader_account: self
                .trader_account
                .ok_or(PhoenixIxError::MissingField("trader_account"))?,
            trader_pda_index: self.trader_pda_index.unwrap_or(0),
            subaccount_index: self.subaccount_index.unwrap_or(0),
            fee_bps: self
                .fee_bps
                .ok_or(PhoenixIxError::MissingField("fee_bps"))?,
        })
    }
}

/// Create a Flight `register_builder` instruction.
pub fn create_register_builder_ix(
    params: RegisterBuilderParams,
) -> Result<Instruction, PhoenixIxError> {
    let data = encode_register_builder(&params);
    let accounts = build_accounts(&params)?;

    Ok(Instruction {
        program_id: FLIGHT_PROGRAM_ID,
        accounts,
        data,
    })
}

fn encode_register_builder(params: &RegisterBuilderParams) -> Vec<u8> {
    let mut data = Vec::with_capacity(18);
    data.extend_from_slice(&FlightInstruction::RegisterBuilder.discriminant());
    data.push(params.trader_pda_index());
    data.push(params.subaccount_index());
    data.extend_from_slice(&params.fee_bps().to_le_bytes());
    data
}

fn build_accounts(params: &RegisterBuilderParams) -> Result<Vec<AccountMeta>, PhoenixIxError> {
    Ok(vec![
        AccountMeta::readonly(get_flight_global_state_address()?),
        AccountMeta::readonly(*PHOENIX_PROGRAM_ID),
        AccountMeta::writable_signer(params.trader_authority()),
        AccountMeta::writable(params.trader_account()),
        AccountMeta::writable(get_flight_builder_state_address(
            &params.trader_authority(),
        )?),
        AccountMeta::readonly(SYSTEM_PROGRAM_ID),
    ])
}

//! Flight: Set Onboarder Signer instruction construction.
//!
//! Mirrors the TS SDK's `buildSetOnboarderSignerIx`
//! (`ts/src/flight/core/ixBuilders/SetOnboarderSigner`).

use solana_pubkey::Pubkey;

use crate::instructions::FlightInstruction;
use crate::instructions::constants::PHOENIX_PROGRAM_ID;
use crate::instructions::error::PhoenixIxError;
use crate::instructions::flight::constants::{
    FLIGHT_PROGRAM_ID, get_flight_builder_state_address, get_flight_global_state_address,
};
use crate::instructions::types::{AccountMeta, Instruction};

/// Parameters for setting a builder's onboarder signer.
#[derive(Debug, Clone)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct SetOnboarderSignerParams {
    /// The builder's authority (readonly signer)
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    builder_authority: Pubkey,
    /// New onboarder signer.
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    signer: Pubkey,
}

impl SetOnboarderSignerParams {
    pub fn builder() -> SetOnboarderSignerParamsBuilder {
        SetOnboarderSignerParamsBuilder::new()
    }

    pub fn builder_authority(&self) -> Pubkey {
        self.builder_authority
    }

    pub fn signer(&self) -> Pubkey {
        self.signer
    }
}

#[derive(Default)]
pub struct SetOnboarderSignerParamsBuilder {
    builder_authority: Option<Pubkey>,
    signer: Option<Pubkey>,
}

impl SetOnboarderSignerParamsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder_authority(mut self, builder_authority: Pubkey) -> Self {
        self.builder_authority = Some(builder_authority);
        self
    }

    pub fn signer(mut self, signer: Pubkey) -> Self {
        self.signer = Some(signer);
        self
    }

    pub fn build(self) -> Result<SetOnboarderSignerParams, PhoenixIxError> {
        Ok(SetOnboarderSignerParams {
            builder_authority: self
                .builder_authority
                .ok_or(PhoenixIxError::MissingField("builder_authority"))?,
            signer: self.signer.ok_or(PhoenixIxError::MissingField("signer"))?,
        })
    }
}

/// Create a Flight `set_onboarder_signer` instruction.
pub fn create_set_onboarder_signer_ix(
    params: SetOnboarderSignerParams,
) -> Result<Instruction, PhoenixIxError> {
    let mut data = Vec::with_capacity(40);
    data.extend_from_slice(&FlightInstruction::SetOnboarderSigner.discriminant());
    data.extend_from_slice(&params.signer().to_bytes());

    let accounts = vec![
        AccountMeta::readonly(get_flight_global_state_address()?),
        AccountMeta::readonly(*PHOENIX_PROGRAM_ID),
        AccountMeta::readonly_signer(params.builder_authority()),
        AccountMeta::writable(get_flight_builder_state_address(
            &params.builder_authority(),
        )?),
    ];

    Ok(Instruction {
        program_id: FLIGHT_PROGRAM_ID,
        accounts,
        data,
    })
}

//! Trader preference update instruction construction.
//!
//! `UpdateTraderPreferenceBits` enables or disables per-trader capabilities.
//! Preferences not named in the instruction are left unchanged.

use borsh::{BorshSerialize, to_vec};
use solana_pubkey::Pubkey;

use crate::instructions::constants::{
    PHOENIX_GLOBAL_CONFIGURATION, PHOENIX_LOG_AUTHORITY, PHOENIX_PROGRAM_ID,
};
use crate::instructions::error::PhoenixIxError;
use crate::instructions::types::{AccountMeta, Instruction};

/// A trader preference, named after the capability it allows.
#[derive(Debug, Copy, Clone, PartialEq, Eq, BorshSerialize)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
#[borsh(use_discriminant = true)]
#[repr(u8)]
pub enum TraderPreferenceToggleTarget {
    /// Permissionless sweeps of collateral from an isolated child trader back
    /// to its parent.
    CollateralSweep = 0,
    /// Collateral swaps signed by the trader's position authority.
    PositionAuthoritySwap = 1,
}

/// `set: true` enables the capability; `set: false` disables it.
#[derive(Debug, Copy, Clone, PartialEq, Eq, BorshSerialize)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct TraderPreferenceToggle {
    pub target: TraderPreferenceToggleTarget,
    pub set: bool,
}

impl TraderPreferenceToggle {
    pub const fn new(target: TraderPreferenceToggleTarget, set: bool) -> Self {
        Self { target, set }
    }
}

/// Parameters for a trader-wallet-signed preference update.
#[derive(Debug, Clone)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateTraderPreferenceBitsParams {
    /// Trader wallet authority. Must sign the transaction.
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader_wallet: Pubkey,
    /// Trader account whose preferences will be updated.
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader_account: Pubkey,
    /// Applied in order.
    toggles: Vec<TraderPreferenceToggle>,
}

impl UpdateTraderPreferenceBitsParams {
    /// Start building with the builder pattern.
    pub fn builder() -> UpdateTraderPreferenceBitsParamsBuilder {
        UpdateTraderPreferenceBitsParamsBuilder::new()
    }

    pub fn trader_wallet(&self) -> Pubkey {
        self.trader_wallet
    }

    pub fn trader_account(&self) -> Pubkey {
        self.trader_account
    }

    pub fn toggles(&self) -> &[TraderPreferenceToggle] {
        &self.toggles
    }
}

/// Builder for [`UpdateTraderPreferenceBitsParams`].
#[derive(Default)]
pub struct UpdateTraderPreferenceBitsParamsBuilder {
    trader_wallet: Option<Pubkey>,
    trader_account: Option<Pubkey>,
    toggles: Vec<TraderPreferenceToggle>,
}

impl UpdateTraderPreferenceBitsParamsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn trader_wallet(mut self, trader_wallet: Pubkey) -> Self {
        self.trader_wallet = Some(trader_wallet);
        self
    }

    pub fn trader_account(mut self, trader_account: Pubkey) -> Self {
        self.trader_account = Some(trader_account);
        self
    }

    pub fn toggle(mut self, target: TraderPreferenceToggleTarget, set: bool) -> Self {
        self.toggles.push(TraderPreferenceToggle::new(target, set));
        self
    }

    pub fn build(self) -> Result<UpdateTraderPreferenceBitsParams, PhoenixIxError> {
        if self.toggles.is_empty() {
            return Err(PhoenixIxError::MissingField("toggles"));
        }
        Ok(UpdateTraderPreferenceBitsParams {
            trader_wallet: self
                .trader_wallet
                .ok_or(PhoenixIxError::MissingField("trader_wallet"))?,
            trader_account: self
                .trader_account
                .ok_or(PhoenixIxError::MissingField("trader_account"))?,
            toggles: self.toggles,
        })
    }
}

/// Create a Phoenix `update_trader_preference_bits` instruction signed by the
/// trader wallet.
pub fn create_update_trader_preference_bits_ix(
    params: UpdateTraderPreferenceBitsParams,
) -> Result<Instruction, PhoenixIxError> {
    let mut data = crate::instructions::PhoenixInstruction::UpdateTraderPreferenceBits
        .discriminant()
        .to_vec();
    data.extend_from_slice(&to_vec(&params.toggles).expect("serialization should not fail"));

    let accounts = vec![
        // LogAccountGroupAccounts
        AccountMeta::readonly(*PHOENIX_PROGRAM_ID),
        AccountMeta::readonly(*PHOENIX_LOG_AUTHORITY),
        // TraderOrAuthorityInstructionGroupAccounts; the permission slot takes
        // the signer itself when no permission account applies.
        AccountMeta::readonly(*PHOENIX_GLOBAL_CONFIGURATION),
        AccountMeta::readonly_signer(params.trader_wallet()),
        AccountMeta::readonly(params.trader_wallet()),
        AccountMeta::writable(params.trader_account()),
    ];

    Ok(Instruction {
        program_id: *PHOENIX_PROGRAM_ID,
        accounts,
        data,
    })
}

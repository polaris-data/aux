//! Sync parent-to-child instruction construction.
//!
//! This module provides instruction building for syncing a parent trader
//! account's state to a child (isolated) subaccount, including global trader
//! index updates.

use solana_pubkey::Pubkey;

use crate::instructions::constants::{
    PHOENIX_GLOBAL_CONFIGURATION, PHOENIX_LOG_AUTHORITY, PHOENIX_PROGRAM_ID,
};
use crate::instructions::error::PhoenixIxError;
use crate::instructions::types::{AccountMeta, Instruction, push_writable_accounts};

/// Parameters for syncing parent state to a child subaccount.
#[derive(Debug, Clone)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct SyncParentToChildParams {
    /// The trader wallet authority (readonly signer).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader_wallet: Pubkey,
    /// The parent trader account (readonly) - source of state.
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    parent_trader_account: Pubkey,
    /// The child trader account (writable) - destination.
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    child_trader_account: Pubkey,
    /// Global trader index addresses (header + arenas).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    global_trader_index: Vec<Pubkey>,
}

impl SyncParentToChildParams {
    pub fn builder() -> SyncParentToChildParamsBuilder {
        SyncParentToChildParamsBuilder::new()
    }

    pub fn trader_wallet(&self) -> Pubkey {
        self.trader_wallet
    }

    pub fn parent_trader_account(&self) -> Pubkey {
        self.parent_trader_account
    }

    pub fn child_trader_account(&self) -> Pubkey {
        self.child_trader_account
    }

    pub fn global_trader_index(&self) -> &[Pubkey] {
        &self.global_trader_index
    }
}

/// Builder for `SyncParentToChildParams`.
#[derive(Default)]
pub struct SyncParentToChildParamsBuilder {
    trader_wallet: Option<Pubkey>,
    parent_trader_account: Option<Pubkey>,
    child_trader_account: Option<Pubkey>,
    global_trader_index: Option<Vec<Pubkey>>,
}

impl SyncParentToChildParamsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn trader_wallet(mut self, trader_wallet: Pubkey) -> Self {
        self.trader_wallet = Some(trader_wallet);
        self
    }

    pub fn parent_trader_account(mut self, parent_trader_account: Pubkey) -> Self {
        self.parent_trader_account = Some(parent_trader_account);
        self
    }

    pub fn child_trader_account(mut self, child_trader_account: Pubkey) -> Self {
        self.child_trader_account = Some(child_trader_account);
        self
    }

    pub fn global_trader_index(mut self, global_trader_index: Vec<Pubkey>) -> Self {
        self.global_trader_index = Some(global_trader_index);
        self
    }

    pub fn build(self) -> Result<SyncParentToChildParams, PhoenixIxError> {
        let parent_trader_account = self
            .parent_trader_account
            .ok_or(PhoenixIxError::MissingField("parent_trader_account"))?;
        let child_trader_account = self
            .child_trader_account
            .ok_or(PhoenixIxError::MissingField("child_trader_account"))?;

        if parent_trader_account == child_trader_account {
            return Err(PhoenixIxError::MissingField(
                "parent and child trader accounts must be different",
            ));
        }

        let global_trader_index = self
            .global_trader_index
            .ok_or(PhoenixIxError::MissingField("global_trader_index"))?;
        if global_trader_index.is_empty() {
            return Err(PhoenixIxError::EmptyGlobalTraderIndex);
        }

        Ok(SyncParentToChildParams {
            trader_wallet: self
                .trader_wallet
                .ok_or(PhoenixIxError::MissingField("trader_wallet"))?,
            parent_trader_account,
            child_trader_account,
            global_trader_index,
        })
    }
}

/// Create a sync parent-to-child instruction.
///
/// Syncs a parent trader account's state to a child (isolated) subaccount.
/// The trader wallet must be the authority for both accounts.
pub fn create_sync_parent_to_child_ix(
    params: SyncParentToChildParams,
) -> Result<Instruction, PhoenixIxError> {
    let data = crate::instructions::PhoenixInstruction::SyncParentToChild
        .discriminant()
        .to_vec();
    let accounts = build_accounts(&params);

    Ok(Instruction {
        program_id: *PHOENIX_PROGRAM_ID,
        accounts,
        data,
    })
}

fn build_accounts(params: &SyncParentToChildParams) -> Vec<AccountMeta> {
    let mut accounts = vec![
        // LogAccountGroupAccounts (2 accounts)
        AccountMeta::readonly(*PHOENIX_PROGRAM_ID),
        AccountMeta::readonly(*PHOENIX_LOG_AUTHORITY),
        // SyncParentToChildInstructionGroupAccounts
        AccountMeta::readonly(*PHOENIX_GLOBAL_CONFIGURATION),
        AccountMeta::readonly_signer(params.trader_wallet()),
        AccountMeta::readonly(params.parent_trader_account()),
        AccountMeta::writable(params.child_trader_account()),
    ];

    push_writable_accounts(&mut accounts, params.global_trader_index());

    accounts
}

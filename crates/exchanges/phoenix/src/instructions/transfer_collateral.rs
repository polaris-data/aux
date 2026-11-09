//! Transfer collateral instruction construction.
//!
//! This module provides instruction building for transferring collateral
//! between subaccounts (e.g., cross-margin to isolated margin).

use solana_pubkey::Pubkey;

use crate::instructions::constants::{
    PHOENIX_GLOBAL_CONFIGURATION, PHOENIX_LOG_AUTHORITY, PHOENIX_PROGRAM_ID,
};
use crate::instructions::error::PhoenixIxError;
use crate::instructions::types::{AccountMeta, Instruction, push_trader_index_accounts};

/// Parameters for transferring collateral between subaccounts.
#[derive(Debug, Clone)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct TransferCollateralParams {
    /// The trader's authority (wallet) — must sign.
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader: Pubkey,
    /// The source trader PDA account (writable).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    src_trader_account: Pubkey,
    /// The destination trader PDA account (writable).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    dst_trader_account: Pubkey,
    /// The perp asset map account (readonly).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    perp_asset_map: Pubkey,
    /// Global trader index addresses (header + arenas).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    global_trader_index: Vec<Pubkey>,
    /// Active trader buffer addresses (header + arenas).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    active_trader_buffer: Vec<Pubkey>,
    /// Optional permission account authorizing `trader` as a secondary position
    /// authority.
    #[cfg_attr(
        all(),
        serde(with = "crate::instructions::serde_helpers::pubkey_option")
    )]
    permission_account: Option<Pubkey>,
    /// Amount to transfer in token base units.
    amount: u64,
}

impl TransferCollateralParams {
    /// Start building with the builder pattern.
    pub fn builder() -> TransferCollateralParamsBuilder {
        TransferCollateralParamsBuilder::new()
    }

    pub fn trader(&self) -> Pubkey {
        self.trader
    }

    pub fn src_trader_account(&self) -> Pubkey {
        self.src_trader_account
    }

    pub fn dst_trader_account(&self) -> Pubkey {
        self.dst_trader_account
    }

    pub fn perp_asset_map(&self) -> Pubkey {
        self.perp_asset_map
    }

    pub fn global_trader_index(&self) -> &[Pubkey] {
        &self.global_trader_index
    }

    pub fn active_trader_buffer(&self) -> &[Pubkey] {
        &self.active_trader_buffer
    }

    pub fn permission_account(&self) -> Option<Pubkey> {
        self.permission_account
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }
}

/// Builder for `TransferCollateralParams`.
#[derive(Default)]
pub struct TransferCollateralParamsBuilder {
    trader: Option<Pubkey>,
    src_trader_account: Option<Pubkey>,
    dst_trader_account: Option<Pubkey>,
    perp_asset_map: Option<Pubkey>,
    global_trader_index: Option<Vec<Pubkey>>,
    active_trader_buffer: Option<Vec<Pubkey>>,
    permission_account: Option<Pubkey>,
    amount: Option<u64>,
}

impl TransferCollateralParamsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn trader(mut self, trader: Pubkey) -> Self {
        self.trader = Some(trader);
        self
    }

    pub fn src_trader_account(mut self, src_trader_account: Pubkey) -> Self {
        self.src_trader_account = Some(src_trader_account);
        self
    }

    pub fn dst_trader_account(mut self, dst_trader_account: Pubkey) -> Self {
        self.dst_trader_account = Some(dst_trader_account);
        self
    }

    pub fn perp_asset_map(mut self, perp_asset_map: Pubkey) -> Self {
        self.perp_asset_map = Some(perp_asset_map);
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

    pub fn permission_account(mut self, permission_account: Pubkey) -> Self {
        self.permission_account = Some(permission_account);
        self
    }

    pub fn amount(mut self, amount: u64) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn build(self) -> Result<TransferCollateralParams, PhoenixIxError> {
        let amount = self.amount.ok_or(PhoenixIxError::MissingField("amount"))?;
        if amount == 0 {
            return Err(PhoenixIxError::InvalidTransferAmount);
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

        Ok(TransferCollateralParams {
            trader: self.trader.ok_or(PhoenixIxError::MissingField("trader"))?,
            src_trader_account: self
                .src_trader_account
                .ok_or(PhoenixIxError::MissingField("src_trader_account"))?,
            dst_trader_account: self
                .dst_trader_account
                .ok_or(PhoenixIxError::MissingField("dst_trader_account"))?,
            perp_asset_map: self
                .perp_asset_map
                .ok_or(PhoenixIxError::MissingField("perp_asset_map"))?,
            global_trader_index,
            active_trader_buffer,
            permission_account: self.permission_account,
            amount,
        })
    }
}

/// Create a transfer collateral instruction.
///
/// Transfers collateral between two subaccounts (e.g., from cross-margin
/// subaccount 0 to an isolated margin subaccount).
///
/// # Arguments
///
/// * `params` - The transfer collateral parameters
///
/// # Returns
///
/// A Solana instruction ready to be included in a transaction.
pub fn create_transfer_collateral_ix(
    params: TransferCollateralParams,
) -> Result<Instruction, PhoenixIxError> {
    let data = encode_transfer_collateral(&params);
    let accounts = build_accounts(&params);

    Ok(Instruction {
        program_id: *PHOENIX_PROGRAM_ID,
        accounts,
        data,
    })
}

fn encode_transfer_collateral(params: &TransferCollateralParams) -> Vec<u8> {
    let mut data = Vec::with_capacity(16);

    // Instruction discriminant (8 bytes)
    data.extend_from_slice(
        &crate::instructions::PhoenixInstruction::TransferCollateral.discriminant(),
    );

    // Amount (8 bytes, little-endian u64)
    data.extend_from_slice(&params.amount().to_le_bytes());

    data
}

fn build_accounts(params: &TransferCollateralParams) -> Vec<AccountMeta> {
    build_transfer_accounts(
        params.trader(),
        params.src_trader_account(),
        params.dst_trader_account(),
        params.perp_asset_map(),
        params.global_trader_index(),
        params.active_trader_buffer(),
        params.permission_account(),
    )
}

fn build_transfer_accounts(
    trader: Pubkey,
    src_trader_account: Pubkey,
    dst_trader_account: Pubkey,
    perp_asset_map: Pubkey,
    global_trader_index: &[Pubkey],
    active_trader_buffer: &[Pubkey],
    permission_account: Option<Pubkey>,
) -> Vec<AccountMeta> {
    let mut accounts = vec![
        // 1. phoenix_program (readonly)
        AccountMeta::readonly(*PHOENIX_PROGRAM_ID),
        // 2. PHOENIX_LOG_AUTHORITY (readonly)
        AccountMeta::readonly(*PHOENIX_LOG_AUTHORITY),
        // 3. global_configuration (readonly — differs from deposit which is writable)
        AccountMeta::readonly(*PHOENIX_GLOBAL_CONFIGURATION),
        // 4. trader (readonly signer)
        AccountMeta::readonly_signer(trader),
        // 5. src_trader_account (writable)
        AccountMeta::writable(src_trader_account),
        // 6. dst_trader_account (writable)
        AccountMeta::writable(dst_trader_account),
        // 7. perp_asset_map (readonly)
        AccountMeta::readonly(perp_asset_map),
    ];

    push_trader_index_accounts(&mut accounts, global_trader_index, active_trader_buffer);

    if let Some(permission_account) = permission_account {
        accounts.push(AccountMeta::writable(permission_account));
    }

    accounts
}

/// Parameters for returning an isolated subaccount's collateral above a target
/// leverage to its cross-margin parent.
#[derive(Debug, Clone)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawToTargetLeverageParams {
    /// The trader's authority (wallet) — must sign.
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader: Pubkey,
    /// The isolated child trader PDA account (writable).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    src_trader_account: Pubkey,
    /// The cross-margin parent trader PDA account (writable).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    dst_trader_account: Pubkey,
    /// The perp asset map account (readonly).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    perp_asset_map: Pubkey,
    /// Global trader index addresses (header + arenas).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    global_trader_index: Vec<Pubkey>,
    /// Active trader buffer addresses (header + arenas).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    active_trader_buffer: Vec<Pubkey>,
    /// Optional permission account authorizing `trader` as a secondary position
    /// authority.
    #[cfg_attr(
        all(),
        serde(with = "crate::instructions::serde_helpers::pubkey_option")
    )]
    permission_account: Option<Pubkey>,
    /// Target leverage in basis points (10_000 = 1x).
    target_leverage_bps: u64,
}

impl WithdrawToTargetLeverageParams {
    /// Start building with the builder pattern.
    pub fn builder() -> WithdrawToTargetLeverageParamsBuilder {
        WithdrawToTargetLeverageParamsBuilder::new()
    }

    pub fn trader(&self) -> Pubkey {
        self.trader
    }

    pub fn src_trader_account(&self) -> Pubkey {
        self.src_trader_account
    }

    pub fn dst_trader_account(&self) -> Pubkey {
        self.dst_trader_account
    }

    pub fn perp_asset_map(&self) -> Pubkey {
        self.perp_asset_map
    }

    pub fn global_trader_index(&self) -> &[Pubkey] {
        &self.global_trader_index
    }

    pub fn active_trader_buffer(&self) -> &[Pubkey] {
        &self.active_trader_buffer
    }

    pub fn permission_account(&self) -> Option<Pubkey> {
        self.permission_account
    }

    pub fn target_leverage_bps(&self) -> u64 {
        self.target_leverage_bps
    }
}

/// Builder for `WithdrawToTargetLeverageParams`.
#[derive(Default)]
pub struct WithdrawToTargetLeverageParamsBuilder {
    trader: Option<Pubkey>,
    src_trader_account: Option<Pubkey>,
    dst_trader_account: Option<Pubkey>,
    perp_asset_map: Option<Pubkey>,
    global_trader_index: Option<Vec<Pubkey>>,
    active_trader_buffer: Option<Vec<Pubkey>>,
    permission_account: Option<Pubkey>,
    target_leverage_bps: Option<u64>,
}

impl WithdrawToTargetLeverageParamsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn trader(mut self, trader: Pubkey) -> Self {
        self.trader = Some(trader);
        self
    }

    pub fn src_trader_account(mut self, src_trader_account: Pubkey) -> Self {
        self.src_trader_account = Some(src_trader_account);
        self
    }

    pub fn dst_trader_account(mut self, dst_trader_account: Pubkey) -> Self {
        self.dst_trader_account = Some(dst_trader_account);
        self
    }

    pub fn perp_asset_map(mut self, perp_asset_map: Pubkey) -> Self {
        self.perp_asset_map = Some(perp_asset_map);
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

    pub fn permission_account(mut self, permission_account: Pubkey) -> Self {
        self.permission_account = Some(permission_account);
        self
    }

    pub fn target_leverage_bps(mut self, target_leverage_bps: u64) -> Self {
        self.target_leverage_bps = Some(target_leverage_bps);
        self
    }

    pub fn build(self) -> Result<WithdrawToTargetLeverageParams, PhoenixIxError> {
        let target_leverage_bps = self
            .target_leverage_bps
            .ok_or(PhoenixIxError::MissingField("target_leverage_bps"))?;
        if target_leverage_bps == 0 {
            return Err(PhoenixIxError::InvalidTargetLeverage);
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

        Ok(WithdrawToTargetLeverageParams {
            trader: self.trader.ok_or(PhoenixIxError::MissingField("trader"))?,
            src_trader_account: self
                .src_trader_account
                .ok_or(PhoenixIxError::MissingField("src_trader_account"))?,
            dst_trader_account: self
                .dst_trader_account
                .ok_or(PhoenixIxError::MissingField("dst_trader_account"))?,
            perp_asset_map: self
                .perp_asset_map
                .ok_or(PhoenixIxError::MissingField("perp_asset_map"))?,
            global_trader_index,
            active_trader_buffer,
            permission_account: self.permission_account,
            target_leverage_bps,
        })
    }
}

/// Create a withdraw-to-target-leverage instruction.
///
/// Transfers collateral above `target_leverage_bps` from an isolated child
/// subaccount to its cross-margin parent. Uses the same accounts as
/// [`create_transfer_collateral_ix`].
pub fn create_withdraw_to_target_leverage_ix(
    params: WithdrawToTargetLeverageParams,
) -> Result<Instruction, PhoenixIxError> {
    let mut data = Vec::with_capacity(16);
    data.extend_from_slice(
        &crate::instructions::PhoenixInstruction::WithdrawToTargetLeverage.discriminant(),
    );
    data.extend_from_slice(&params.target_leverage_bps().to_le_bytes());

    let accounts = build_transfer_accounts(
        params.trader(),
        params.src_trader_account(),
        params.dst_trader_account(),
        params.perp_asset_map(),
        params.global_trader_index(),
        params.active_trader_buffer(),
        params.permission_account(),
    );

    Ok(Instruction {
        program_id: *PHOENIX_PROGRAM_ID,
        accounts,
        data,
    })
}

/// Parameters for transferring all collateral from a child subaccount back to
/// the parent (subaccount 0).
#[derive(Debug, Clone)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct TransferCollateralChildToParentParams {
    /// The trader's authority (wallet) — must sign.
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader: Pubkey,
    /// The child trader PDA account (writable).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    child_trader_account: Pubkey,
    /// The parent trader PDA account (writable).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    parent_trader_account: Pubkey,
    /// The perp asset map account (readonly).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    perp_asset_map: Pubkey,
    /// Global trader index addresses (header + arenas).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    global_trader_index: Vec<Pubkey>,
    /// Active trader buffer addresses (header + arenas).
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    active_trader_buffer: Vec<Pubkey>,
}

impl TransferCollateralChildToParentParams {
    pub fn builder() -> TransferCollateralChildToParentParamsBuilder {
        TransferCollateralChildToParentParamsBuilder::new()
    }

    pub fn trader(&self) -> Pubkey {
        self.trader
    }

    pub fn child_trader_account(&self) -> Pubkey {
        self.child_trader_account
    }

    pub fn parent_trader_account(&self) -> Pubkey {
        self.parent_trader_account
    }

    pub fn perp_asset_map(&self) -> Pubkey {
        self.perp_asset_map
    }

    pub fn global_trader_index(&self) -> &[Pubkey] {
        &self.global_trader_index
    }

    pub fn active_trader_buffer(&self) -> &[Pubkey] {
        &self.active_trader_buffer
    }
}

/// Builder for `TransferCollateralChildToParentParams`.
#[derive(Default)]
pub struct TransferCollateralChildToParentParamsBuilder {
    trader: Option<Pubkey>,
    child_trader_account: Option<Pubkey>,
    parent_trader_account: Option<Pubkey>,
    perp_asset_map: Option<Pubkey>,
    global_trader_index: Option<Vec<Pubkey>>,
    active_trader_buffer: Option<Vec<Pubkey>>,
}

impl TransferCollateralChildToParentParamsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn trader(mut self, trader: Pubkey) -> Self {
        self.trader = Some(trader);
        self
    }

    pub fn child_trader_account(mut self, child_trader_account: Pubkey) -> Self {
        self.child_trader_account = Some(child_trader_account);
        self
    }

    pub fn parent_trader_account(mut self, parent_trader_account: Pubkey) -> Self {
        self.parent_trader_account = Some(parent_trader_account);
        self
    }

    pub fn perp_asset_map(mut self, perp_asset_map: Pubkey) -> Self {
        self.perp_asset_map = Some(perp_asset_map);
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

    pub fn build(self) -> Result<TransferCollateralChildToParentParams, PhoenixIxError> {
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

        Ok(TransferCollateralChildToParentParams {
            trader: self.trader.ok_or(PhoenixIxError::MissingField("trader"))?,
            child_trader_account: self
                .child_trader_account
                .ok_or(PhoenixIxError::MissingField("child_trader_account"))?,
            parent_trader_account: self
                .parent_trader_account
                .ok_or(PhoenixIxError::MissingField("parent_trader_account"))?,
            perp_asset_map: self
                .perp_asset_map
                .ok_or(PhoenixIxError::MissingField("perp_asset_map"))?,
            global_trader_index,
            active_trader_buffer,
        })
    }
}

/// Create a transfer collateral child-to-parent instruction.
///
/// Transfers **all** collateral from a child subaccount back to the parent
/// (subaccount 0). No-ops if the child has open positions, open orders, or
/// zero collateral.
pub fn create_transfer_collateral_child_to_parent_ix(
    params: TransferCollateralChildToParentParams,
) -> Result<Instruction, PhoenixIxError> {
    let data = crate::instructions::PhoenixInstruction::TransferCollateralChildToParent
        .discriminant()
        .to_vec();
    let accounts = build_child_to_parent_accounts(&params);

    Ok(Instruction {
        program_id: *PHOENIX_PROGRAM_ID,
        accounts,
        data,
    })
}

fn build_child_to_parent_accounts(
    params: &TransferCollateralChildToParentParams,
) -> Vec<AccountMeta> {
    let mut accounts = Vec::new();

    // 1. phoenix_program (readonly)
    accounts.push(AccountMeta::readonly(*PHOENIX_PROGRAM_ID));
    // 2. PHOENIX_LOG_AUTHORITY (readonly)
    accounts.push(AccountMeta::readonly(*PHOENIX_LOG_AUTHORITY));
    // 3. global_configuration (readonly)
    accounts.push(AccountMeta::readonly(*PHOENIX_GLOBAL_CONFIGURATION));
    // 4. trader (readonly signer)
    accounts.push(AccountMeta::readonly_signer(params.trader()));
    // 5. child_trader_account (writable)
    accounts.push(AccountMeta::writable(params.child_trader_account()));
    // 6. parent_trader_account (writable)
    accounts.push(AccountMeta::writable(params.parent_trader_account()));
    // 7. perp_asset_map (readonly)
    accounts.push(AccountMeta::readonly(params.perp_asset_map()));

    push_trader_index_accounts(
        &mut accounts,
        params.global_trader_index(),
        params.active_trader_buffer(),
    );

    accounts
}

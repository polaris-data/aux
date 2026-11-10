//! Native SOL spot collateral instructions.
//!
//! Native SOL can be posted as collateral alongside the canonical quote token.
//! Its lamports live in the trader account itself, above the rent floor, and
//! the *accounted* portion is tracked as a header-extension entry in the trader
//! position map. Lamports beyond the accounted balance ("excess") carry no
//! margin value and can be swept out freely.
//!
//! Native SOL user and liquidation instructions are exposed here:
//!
//! - [`create_sync_native_ix`] reconciles the accounted balance against the
//!   account's actual lamports. Permissionless.
//! - [`create_withdraw_native_sol_ix`] moves lamports out.
//! - [`create_transfer_native_sol_ix`] and
//!   [`create_transfer_native_sol_from_child_to_parent_ix`] move collateral
//!   between trader accounts.
//! - [`create_swap_native_ix`] swaps between native SOL and quote collateral
//!   through an external venue.
//! - [`create_liquidate_native_sol_ix`] lets the risk authority or a delegated
//!   liquidator seize native SOL collateral.
//!
//! Admin instructions (configure, activate) are deliberately not exposed.
//!
//! Two custom program errors are specific to these instructions:
//!
//! - **7100 `FeatureDisabled`** — the exchange has native SOL collateral turned
//!   off, or the asset is not active.
//! - **7101 `PositionAuthoritySwapDisabled`** — a `SwapNative` was signed by a
//!   position authority while either opt-out gate is set. See
//!   [`create_swap_native_ix`].

use solana_pubkey::Pubkey;

use crate::instructions::constants::{
    PHOENIX_GLOBAL_CONFIGURATION, PHOENIX_LOG_AUTHORITY, PHOENIX_PROGRAM_ID, SPL_TOKEN_PROGRAM_ID,
    SYSTEM_PROGRAM_ID, get_global_vault_address, get_native_sol_authority_address,
};
use crate::instructions::error::PhoenixIxError;
use crate::instructions::types::{AccountMeta, Instruction, push_trader_index_accounts};

/// The packed venue-instruction encoding addresses accounts with 6 bits, so a
/// swap or liquidation can reference at most this many distinct accounts.
pub const MAX_PACKED_EXTERNAL_ACCOUNTS: usize = 64;

/// Reserve capacity for a non-position entry without increasing the trader's
/// permitted perpetual-position count. The contract grows by one entry only
/// when needed and otherwise succeeds without resizing; it never shrinks.
/// `payer` funds additional rent and `trader_wallet` identifies the owner.
pub fn create_realloc_trader_ix(
    payer: Pubkey,
    trader_wallet: Pubkey,
    trader_account: Pubkey,
) -> Instruction {
    Instruction {
        program_id: *PHOENIX_PROGRAM_ID,
        accounts: vec![
            AccountMeta::readonly(*PHOENIX_PROGRAM_ID),
            AccountMeta::readonly(*PHOENIX_LOG_AUTHORITY),
            AccountMeta::readonly(*PHOENIX_GLOBAL_CONFIGURATION),
            AccountMeta::writable_signer(payer),
            AccountMeta::readonly(trader_wallet),
            AccountMeta::writable(trader_account),
            AccountMeta::readonly(SYSTEM_PROGRAM_ID),
        ],
        data: crate::instructions::PhoenixInstruction::ReallocTrader
            .discriminant()
            .to_vec(),
    }
}

/// Deposit native SOL: reserve extension capacity, transfer lamports, then sync
/// collateral. Reallocation runs before the transfer so newly deposited SOL is
/// not used to pay additional rent. `payer` pays rent; `trader_wallet` supplies
/// the SOL. The trader must exist, or be registered earlier in the transaction.
/// The contract may retain one spare entry for another non-position extension.
pub fn create_deposit_native_sol_ixs(
    payer: Pubkey,
    trader_wallet: Pubkey,
    lamports: u64,
    sync: SyncNativeParams,
) -> Result<Vec<Instruction>, PhoenixIxError> {
    if lamports == 0 {
        return Err(PhoenixIxError::InvalidDepositAmount);
    }
    let trader_account = sync.trader_account();
    let mut transfer_data = 2_u32.to_le_bytes().to_vec(); // SystemInstruction::Transfer
    transfer_data.extend_from_slice(&lamports.to_le_bytes());
    Ok(vec![
        create_realloc_trader_ix(payer, trader_wallet, trader_account),
        Instruction {
            program_id: SYSTEM_PROGRAM_ID,
            accounts: vec![
                AccountMeta::writable_signer(trader_wallet),
                AccountMeta::writable(trader_account),
            ],
            data: transfer_data,
        },
        create_sync_native_ix(sync)?,
    ])
}

////////////////////////////////////////////////////////////////////////////////
// SyncNative
////////////////////////////////////////////////////////////////////////////////

/// Parameters for [`create_sync_native_ix`].
#[derive(Debug, Clone)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct SyncNativeParams {
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader_account: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    global_trader_index: Vec<Pubkey>,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    active_trader_buffer: Vec<Pubkey>,
}

impl SyncNativeParams {
    pub fn builder() -> SyncNativeParamsBuilder {
        SyncNativeParamsBuilder::default()
    }

    pub fn trader_account(&self) -> Pubkey {
        self.trader_account
    }

    pub fn global_trader_index(&self) -> &[Pubkey] {
        &self.global_trader_index
    }

    pub fn active_trader_buffer(&self) -> &[Pubkey] {
        &self.active_trader_buffer
    }
}

#[derive(Debug, Clone, Default)]
pub struct SyncNativeParamsBuilder {
    trader_account: Option<Pubkey>,
    global_trader_index: Option<Vec<Pubkey>>,
    active_trader_buffer: Option<Vec<Pubkey>>,
}

impl SyncNativeParamsBuilder {
    pub fn trader_account(mut self, trader_account: Pubkey) -> Self {
        self.trader_account = Some(trader_account);
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

    pub fn build(self) -> Result<SyncNativeParams, PhoenixIxError> {
        let (global_trader_index, active_trader_buffer) =
            require_index_accounts(self.global_trader_index, self.active_trader_buffer)?;

        Ok(SyncNativeParams {
            trader_account: self
                .trader_account
                .ok_or(PhoenixIxError::MissingField("trader_account"))?,
            global_trader_index,
            active_trader_buffer,
        })
    }
}

/// Reconcile a trader's accounted native SOL collateral against the lamports
/// actually in the account.
///
/// Permissionless — anyone may crank it. An upward reconciliation is clamped to
/// the per-trader and exchange-wide caps; whatever the clamp leaves over stays
/// in the account as uncounted excess, recoverable by a later sync once
/// headroom frees up.
pub fn create_sync_native_ix(params: SyncNativeParams) -> Result<Instruction, PhoenixIxError> {
    let mut accounts = log_accounts();
    // The global configuration is writable: syncing moves the exchange-wide
    // collateral tally.
    accounts.push(AccountMeta::writable(*PHOENIX_GLOBAL_CONFIGURATION));
    accounts.push(AccountMeta::writable(params.trader_account()));
    push_trader_index_accounts(
        &mut accounts,
        params.global_trader_index(),
        params.active_trader_buffer(),
    );

    Ok(Instruction {
        program_id: *PHOENIX_PROGRAM_ID,
        accounts,
        data: crate::instructions::PhoenixInstruction::SyncNative
            .discriminant()
            .to_vec(),
    })
}

////////////////////////////////////////////////////////////////////////////////
// WithdrawNativeSol
////////////////////////////////////////////////////////////////////////////////

/// What a [`create_withdraw_native_sol_ix`] call takes out.
///
/// Accounted collateral is margin checked and consumes its quote-notional value
/// from the exchange-wide withdrawal throttle. Uncounted excess does neither,
/// because it was never protocol collateral.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub enum WithdrawNativeSolAction {
    /// Sweep every uncounted excess lamport, leaving accounted collateral and
    /// the rent floor untouched. Margin-free.
    AllExcess,
    /// Withdraw `amount` lamports, spending uncounted excess first and only
    /// then accounted collateral.
    WithExcess { amount: u64 },
    /// Withdraw `amount` lamports of accounted collateral, leaving any excess
    /// in place.
    WithoutExcess { amount: u64 },
}

impl WithdrawNativeSolAction {
    pub(crate) fn encode(&self, out: &mut Vec<u8>) {
        match self {
            Self::AllExcess => out.push(0),
            Self::WithExcess { amount } => {
                out.push(1);
                out.extend_from_slice(&amount.to_le_bytes());
            }
            Self::WithoutExcess { amount } => {
                out.push(2);
                out.extend_from_slice(&amount.to_le_bytes());
            }
        }
    }

    pub(crate) fn amount(&self) -> Option<u64> {
        match self {
            Self::AllExcess => None,
            Self::WithExcess { amount } | Self::WithoutExcess { amount } => Some(*amount),
        }
    }
}

/// Parameters for [`create_withdraw_native_sol_ix`].
#[derive(Debug, Clone)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawNativeSolParams {
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader_account: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    perp_asset_map: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    destination: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    withdraw_queue: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    global_trader_index: Vec<Pubkey>,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    active_trader_buffer: Vec<Pubkey>,
    action: WithdrawNativeSolAction,
}

impl WithdrawNativeSolParams {
    pub fn builder() -> WithdrawNativeSolParamsBuilder {
        WithdrawNativeSolParamsBuilder::default()
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

    pub fn destination(&self) -> Pubkey {
        self.destination
    }

    pub fn withdraw_queue(&self) -> Pubkey {
        self.withdraw_queue
    }

    pub fn global_trader_index(&self) -> &[Pubkey] {
        &self.global_trader_index
    }

    pub fn active_trader_buffer(&self) -> &[Pubkey] {
        &self.active_trader_buffer
    }

    pub fn action(&self) -> WithdrawNativeSolAction {
        self.action
    }
}

#[derive(Debug, Clone, Default)]
pub struct WithdrawNativeSolParamsBuilder {
    trader: Option<Pubkey>,
    trader_account: Option<Pubkey>,
    perp_asset_map: Option<Pubkey>,
    destination: Option<Pubkey>,
    withdraw_queue: Option<Pubkey>,
    global_trader_index: Option<Vec<Pubkey>>,
    active_trader_buffer: Option<Vec<Pubkey>>,
    action: Option<WithdrawNativeSolAction>,
}

impl WithdrawNativeSolParamsBuilder {
    /// The trader's wallet authority, which must sign. A position authority
    /// cannot sign this instruction.
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

    /// Where the lamports go. Must be a system-owned account, and must be
    /// neither the trader account nor the native SOL authority PDA.
    pub fn destination(mut self, destination: Pubkey) -> Self {
        self.destination = Some(destination);
        self
    }

    pub fn withdraw_queue(mut self, withdraw_queue: Pubkey) -> Self {
        self.withdraw_queue = Some(withdraw_queue);
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

    pub fn action(mut self, action: WithdrawNativeSolAction) -> Self {
        self.action = Some(action);
        self
    }

    pub fn build(self) -> Result<WithdrawNativeSolParams, PhoenixIxError> {
        let action = self.action.ok_or(PhoenixIxError::MissingField("action"))?;
        if action.amount() == Some(0) {
            return Err(PhoenixIxError::InvalidWithdrawAmount);
        }

        let (global_trader_index, active_trader_buffer) =
            require_index_accounts(self.global_trader_index, self.active_trader_buffer)?;

        let trader_account = self
            .trader_account
            .ok_or(PhoenixIxError::MissingField("trader_account"))?;
        let destination = self
            .destination
            .ok_or(PhoenixIxError::MissingField("destination"))?;
        if destination == trader_account {
            return Err(PhoenixIxError::InvalidWithdrawDestination);
        }

        Ok(WithdrawNativeSolParams {
            trader: self.trader.ok_or(PhoenixIxError::MissingField("trader"))?,
            trader_account,
            perp_asset_map: self
                .perp_asset_map
                .ok_or(PhoenixIxError::MissingField("perp_asset_map"))?,
            destination,
            withdraw_queue: self
                .withdraw_queue
                .ok_or(PhoenixIxError::MissingField("withdraw_queue"))?,
            global_trader_index,
            active_trader_buffer,
            action,
        })
    }
}

/// Withdraw native SOL spot collateral.
///
/// Unlike a quote withdrawal this charges no fee, ignores the deposit cooldown,
/// and is never enqueued: if the exchange-wide throttle cannot absorb the
/// withdrawal's quote-notional value right now, the instruction fails rather
/// than queueing. Excess-only withdrawals bypass the throttle entirely.
pub fn create_withdraw_native_sol_ix(
    params: WithdrawNativeSolParams,
) -> Result<Instruction, PhoenixIxError> {
    let mut data = crate::instructions::PhoenixInstruction::WithdrawNativeSol
        .discriminant()
        .to_vec();
    params.action().encode(&mut data);

    let mut accounts = log_accounts();
    accounts.push(AccountMeta::writable(*PHOENIX_GLOBAL_CONFIGURATION));
    accounts.push(AccountMeta::readonly_signer(params.trader()));
    accounts.push(AccountMeta::writable(params.trader_account()));
    accounts.push(AccountMeta::writable(params.perp_asset_map()));
    accounts.push(AccountMeta::writable(params.destination()));
    // The withdraw queue precedes the index arenas here, unlike a quote
    // withdrawal where it trails them.
    accounts.push(AccountMeta::writable(params.withdraw_queue()));
    push_trader_index_accounts(
        &mut accounts,
        params.global_trader_index(),
        params.active_trader_buffer(),
    );

    Ok(Instruction {
        program_id: *PHOENIX_PROGRAM_ID,
        accounts,
        data,
    })
}

////////////////////////////////////////////////////////////////////////////////
// TransferNativeSol
////////////////////////////////////////////////////////////////////////////////

/// Parameters for [`create_transfer_native_sol_ix`].
#[derive(Debug, Clone)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct TransferNativeSolParams {
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    src_trader_account: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    dst_trader_account: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    perp_asset_map: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    global_trader_index: Vec<Pubkey>,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    active_trader_buffer: Vec<Pubkey>,
    #[cfg_attr(
        all(),
        serde(with = "crate::instructions::serde_helpers::pubkey_option")
    )]
    permission_account: Option<Pubkey>,
    lamports: u64,
}

impl TransferNativeSolParams {
    pub fn builder() -> TransferNativeSolParamsBuilder {
        TransferNativeSolParamsBuilder::default()
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

    /// Amount in **lamports**, not quote units.
    pub fn lamports(&self) -> u64 {
        self.lamports
    }
}

#[derive(Debug, Clone, Default)]
pub struct TransferNativeSolParamsBuilder {
    trader: Option<Pubkey>,
    src_trader_account: Option<Pubkey>,
    dst_trader_account: Option<Pubkey>,
    perp_asset_map: Option<Pubkey>,
    global_trader_index: Option<Vec<Pubkey>>,
    active_trader_buffer: Option<Vec<Pubkey>>,
    permission_account: Option<Pubkey>,
    lamports: Option<u64>,
}

impl TransferNativeSolParamsBuilder {
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

    /// Amount in **lamports**, not quote units.
    pub fn lamports(mut self, lamports: u64) -> Self {
        self.lamports = Some(lamports);
        self
    }

    pub fn build(self) -> Result<TransferNativeSolParams, PhoenixIxError> {
        let lamports = self
            .lamports
            .ok_or(PhoenixIxError::MissingField("lamports"))?;
        if lamports == 0 {
            return Err(PhoenixIxError::InvalidTransferAmount);
        }

        let (global_trader_index, active_trader_buffer) =
            require_index_accounts(self.global_trader_index, self.active_trader_buffer)?;

        Ok(TransferNativeSolParams {
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
            lamports,
        })
    }
}

/// Move native SOL spot collateral between two of a trader's accounts.
///
/// The source debit is margin checked. The destination is **rejected**, not
/// clamped, if the credit would push it past the per-trader cap. The
/// exchange-wide tally is unchanged, since the collateral never leaves the
/// protocol.
///
/// This shares its account layout exactly with a quote collateral transfer.
pub fn create_transfer_native_sol_ix(
    params: TransferNativeSolParams,
) -> Result<Instruction, PhoenixIxError> {
    let mut data = crate::instructions::PhoenixInstruction::TransferNativeSol
        .discriminant()
        .to_vec();
    data.extend_from_slice(&params.lamports().to_le_bytes());

    let mut accounts = log_accounts();
    // Both the global configuration and the perp asset map are readonly for a
    // transfer, unlike a withdrawal where both are writable.
    accounts.push(AccountMeta::readonly(*PHOENIX_GLOBAL_CONFIGURATION));
    accounts.push(AccountMeta::readonly_signer(params.trader()));
    accounts.push(AccountMeta::writable(params.src_trader_account()));
    accounts.push(AccountMeta::writable(params.dst_trader_account()));
    accounts.push(AccountMeta::readonly(params.perp_asset_map()));
    push_trader_index_accounts(
        &mut accounts,
        params.global_trader_index(),
        params.active_trader_buffer(),
    );
    if let Some(permission_account) = params.permission_account() {
        accounts.push(AccountMeta::writable(permission_account));
    }

    Ok(Instruction {
        program_id: *PHOENIX_PROGRAM_ID,
        accounts,
        data,
    })
}

////////////////////////////////////////////////////////////////////////////////
// TransferNativeSolFromChildToParent
////////////////////////////////////////////////////////////////////////////////

/// Parameters for [`create_transfer_native_sol_from_child_to_parent_ix`].
#[derive(Debug, Clone)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct TransferNativeSolFromChildToParentParams {
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    child_trader_account: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    parent_trader_account: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    perp_asset_map: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    global_trader_index: Vec<Pubkey>,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    active_trader_buffer: Vec<Pubkey>,
    trader_signs: bool,
}

impl TransferNativeSolFromChildToParentParams {
    pub fn builder() -> TransferNativeSolFromChildToParentParamsBuilder {
        TransferNativeSolFromChildToParentParamsBuilder::default()
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

    pub fn trader_signs(&self) -> bool {
        self.trader_signs
    }
}

#[derive(Debug, Clone)]
pub struct TransferNativeSolFromChildToParentParamsBuilder {
    trader: Option<Pubkey>,
    child_trader_account: Option<Pubkey>,
    parent_trader_account: Option<Pubkey>,
    perp_asset_map: Option<Pubkey>,
    global_trader_index: Option<Vec<Pubkey>>,
    active_trader_buffer: Option<Vec<Pubkey>>,
    trader_signs: bool,
}

impl Default for TransferNativeSolFromChildToParentParamsBuilder {
    fn default() -> Self {
        Self {
            trader: None,
            child_trader_account: None,
            parent_trader_account: None,
            perp_asset_map: None,
            global_trader_index: None,
            active_trader_buffer: None,
            trader_signs: true,
        }
    }
}

impl TransferNativeSolFromChildToParentParamsBuilder {
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

    /// Whether the trader wallet signs. Defaults to `true`, matching the quote
    /// collateral sweep builder.
    ///
    /// The sweep is permissionless unless the child opted out with the
    /// `disable_collateral_sweep` preference, so a crank that does not hold the
    /// wallet key should set this to `false`.
    pub fn trader_signs(mut self, trader_signs: bool) -> Self {
        self.trader_signs = trader_signs;
        self
    }

    pub fn build(self) -> Result<TransferNativeSolFromChildToParentParams, PhoenixIxError> {
        let (global_trader_index, active_trader_buffer) =
            require_index_accounts(self.global_trader_index, self.active_trader_buffer)?;

        Ok(TransferNativeSolFromChildToParentParams {
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
            trader_signs: self.trader_signs,
        })
    }
}

/// Sweep a flat isolated child account's native SOL spot collateral into its
/// parent.
///
/// This is a silent no-op — not an error — when the child still has splines,
/// open orders, a position, or a negative quote balance, or when it holds no
/// native SOL.
pub fn create_transfer_native_sol_from_child_to_parent_ix(
    params: TransferNativeSolFromChildToParentParams,
) -> Result<Instruction, PhoenixIxError> {
    let mut accounts = log_accounts();
    accounts.push(AccountMeta::readonly(*PHOENIX_GLOBAL_CONFIGURATION));
    accounts.push(if params.trader_signs() {
        AccountMeta::readonly_signer(params.trader())
    } else {
        AccountMeta::readonly(params.trader())
    });
    accounts.push(AccountMeta::writable(params.child_trader_account()));
    accounts.push(AccountMeta::writable(params.parent_trader_account()));
    accounts.push(AccountMeta::readonly(params.perp_asset_map()));
    push_trader_index_accounts(
        &mut accounts,
        params.global_trader_index(),
        params.active_trader_buffer(),
    );

    Ok(Instruction {
        program_id: *PHOENIX_PROGRAM_ID,
        accounts,
        data: crate::instructions::PhoenixInstruction::TransferNativeSolFromChildToParent
            .discriminant()
            .to_vec(),
    })
}

////////////////////////////////////////////////////////////////////////////////
// SwapNative
////////////////////////////////////////////////////////////////////////////////

/// Which way a [`create_swap_native_ix`] call trades.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub enum SwapDirection {
    /// Native SOL in, quote collateral out.
    Sell,
    /// Quote collateral in, native SOL out.
    Buy,
}

impl SwapDirection {
    pub(crate) fn as_u8(self) -> u8 {
        match self {
            Self::Sell => 0,
            Self::Buy => 1,
        }
    }

    /// The asset `min_amount_out` is denominated in, for rendering to users.
    pub fn output_asset(self) -> &'static str {
        match self {
            Self::Sell => "quote lots",
            Self::Buy => "lamports",
        }
    }
}

/// An account reference inside a packed venue instruction: an index into the
/// deduplicated account list plus its signer/writable bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct PackedAccountMeta(u8);

impl PackedAccountMeta {
    const INDEX_SHIFT: u8 = 2;
    const SIGNER_BIT: u8 = 0b10;
    const WRITABLE_BIT: u8 = 0b01;

    fn new(index: u8, is_signer: bool, is_writable: bool) -> Self {
        let mut state = index << Self::INDEX_SHIFT;
        if is_signer {
            state |= Self::SIGNER_BIT;
        }
        if is_writable {
            state |= Self::WRITABLE_BIT;
        }
        Self(state)
    }

    pub fn state(self) -> u8 {
        self.0
    }

    pub fn index(self) -> u8 {
        self.0 >> Self::INDEX_SHIFT
    }

    pub fn is_signer(self) -> bool {
        self.0 & Self::SIGNER_BIT != 0
    }

    pub fn is_writable(self) -> bool {
        self.0 & Self::WRITABLE_BIT != 0
    }
}

/// One venue instruction, with its accounts addressed by index rather than by
/// pubkey.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct PackedInstruction {
    pub program_id_index: u8,
    pub data: Vec<u8>,
    pub account_metas: Vec<PackedAccountMeta>,
}

/// Venue instructions packed for [`create_swap_native_ix`] or
/// [`create_liquidate_native_sol_ix`], together with the deduplicated account
/// list they index into.
///
/// The two halves must stay together — an index list is meaningless against a
/// different account list — so [`pack_venue_instructions`] returns them as one
/// value.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PackedVenueInstructions {
    instructions: Vec<PackedInstruction>,
    accounts: Vec<AccountMeta>,
}

impl PackedVenueInstructions {
    pub fn instructions(&self) -> &[PackedInstruction] {
        &self.instructions
    }

    pub fn accounts(&self) -> &[AccountMeta] {
        &self.accounts
    }
}

/// Pack venue instructions for a swap.
///
/// Accounts are deduplicated by pubkey, unioning their writable and signer
/// bits, and each instruction's metas become indices into that list.
///
/// Only `signer` — the swap's signer — may be marked as a signer by a venue
/// instruction: the program has no other key to sign with.
pub fn pack_venue_instructions(
    signer: &Pubkey,
    instructions: &[Instruction],
) -> Result<PackedVenueInstructions, PhoenixIxError> {
    let mut accounts: Vec<AccountMeta> = Vec::new();
    let mut packed = Vec::with_capacity(instructions.len());

    let index_of = |accounts: &mut Vec<AccountMeta>,
                    pubkey: Pubkey,
                    is_signer: bool,
                    is_writable: bool|
     -> Result<u8, PhoenixIxError> {
        if let Some(position) = accounts.iter().position(|meta| meta.pubkey == pubkey) {
            accounts[position].is_signer |= is_signer;
            accounts[position].is_writable |= is_writable;
            return u8::try_from(position).map_err(|_| PhoenixIxError::TooManyVenueAccounts);
        }
        if accounts.len() >= MAX_PACKED_EXTERNAL_ACCOUNTS {
            return Err(PhoenixIxError::TooManyVenueAccounts);
        }
        accounts.push(AccountMeta {
            pubkey,
            is_signer,
            is_writable,
        });
        u8::try_from(accounts.len() - 1).map_err(|_| PhoenixIxError::TooManyVenueAccounts)
    };

    for instruction in instructions {
        let program_id_index = index_of(&mut accounts, instruction.program_id, false, false)?;

        let mut account_metas = Vec::with_capacity(instruction.accounts.len());
        for meta in &instruction.accounts {
            if meta.is_signer && meta.pubkey != *signer {
                return Err(PhoenixIxError::UnexpectedVenueSigner);
            }
            let index = index_of(&mut accounts, meta.pubkey, meta.is_signer, meta.is_writable)?;
            account_metas.push(PackedAccountMeta::new(
                index,
                meta.is_signer,
                meta.is_writable,
            ));
        }

        packed.push(PackedInstruction {
            program_id_index,
            data: instruction.data.clone(),
            account_metas,
        });
    }

    Ok(PackedVenueInstructions {
        instructions: packed,
        accounts,
    })
}

////////////////////////////////////////////////////////////////////////////////
// LiquidateNativeSol
////////////////////////////////////////////////////////////////////////////////

/// Parameters for [`create_liquidate_native_sol_ix`].
#[derive(Debug, Clone)]
pub struct LiquidateNativeSolParams {
    signer: Pubkey,
    permission_account: Option<Pubkey>,
    liquidatee_account: Pubkey,
    mint: Pubkey,
    perp_asset_map: Pubkey,
    signer_quote_token_account: Pubkey,
    withdraw_queue: Pubkey,
    global_trader_index: Vec<Pubkey>,
    active_trader_buffer: Vec<Pubkey>,
    max_native_sol_amount: u64,
    extra_trader_accounts: Vec<Pubkey>,
    venue: PackedVenueInstructions,
}

impl LiquidateNativeSolParams {
    pub fn builder() -> LiquidateNativeSolParamsBuilder {
        LiquidateNativeSolParamsBuilder::default()
    }
}

#[derive(Debug, Clone, Default)]
pub struct LiquidateNativeSolParamsBuilder {
    signer: Option<Pubkey>,
    permission_account: Option<Pubkey>,
    liquidatee_account: Option<Pubkey>,
    mint: Option<Pubkey>,
    perp_asset_map: Option<Pubkey>,
    signer_quote_token_account: Option<Pubkey>,
    withdraw_queue: Option<Pubkey>,
    global_trader_index: Option<Vec<Pubkey>>,
    active_trader_buffer: Option<Vec<Pubkey>>,
    max_native_sol_amount: Option<u64>,
    extra_trader_accounts: Vec<Pubkey>,
    venue: Option<PackedVenueInstructions>,
}

impl LiquidateNativeSolParamsBuilder {
    pub fn signer(mut self, signer: Pubkey) -> Self {
        self.signer = Some(signer);
        self
    }

    /// Delegated liquidation permission PDA. Omit for the direct risk-authority
    /// path, where the signer occupies both account slots.
    pub fn permission_account(mut self, permission_account: Pubkey) -> Self {
        self.permission_account = Some(permission_account);
        self
    }

    pub fn liquidatee_account(mut self, liquidatee_account: Pubkey) -> Self {
        self.liquidatee_account = Some(liquidatee_account);
        self
    }

    pub fn mint(mut self, mint: Pubkey) -> Self {
        self.mint = Some(mint);
        self
    }

    pub fn perp_asset_map(mut self, perp_asset_map: Pubkey) -> Self {
        self.perp_asset_map = Some(perp_asset_map);
        self
    }

    pub fn signer_quote_token_account(mut self, account: Pubkey) -> Self {
        self.signer_quote_token_account = Some(account);
        self
    }

    pub fn withdraw_queue(mut self, withdraw_queue: Pubkey) -> Self {
        self.withdraw_queue = Some(withdraw_queue);
        self
    }

    pub fn global_trader_index(mut self, accounts: Vec<Pubkey>) -> Self {
        self.global_trader_index = Some(accounts);
        self
    }

    pub fn active_trader_buffer(mut self, accounts: Vec<Pubkey>) -> Self {
        self.active_trader_buffer = Some(accounts);
        self
    }

    pub fn max_native_sol_amount(mut self, amount: u64) -> Self {
        self.max_native_sol_amount = Some(amount);
        self
    }

    pub fn extra_trader_accounts(mut self, accounts: Vec<Pubkey>) -> Self {
        self.extra_trader_accounts = accounts;
        self
    }

    pub fn venue(mut self, venue: PackedVenueInstructions) -> Self {
        self.venue = Some(venue);
        self
    }

    pub fn build(self) -> Result<LiquidateNativeSolParams, PhoenixIxError> {
        let (global_trader_index, active_trader_buffer) =
            require_index_accounts(self.global_trader_index, self.active_trader_buffer)?;
        Ok(LiquidateNativeSolParams {
            signer: self.signer.ok_or(PhoenixIxError::MissingField("signer"))?,
            permission_account: self.permission_account,
            liquidatee_account: self
                .liquidatee_account
                .ok_or(PhoenixIxError::MissingField("liquidatee_account"))?,
            mint: self.mint.ok_or(PhoenixIxError::MissingField("mint"))?,
            perp_asset_map: self
                .perp_asset_map
                .ok_or(PhoenixIxError::MissingField("perp_asset_map"))?,
            signer_quote_token_account: self
                .signer_quote_token_account
                .ok_or(PhoenixIxError::MissingField("signer_quote_token_account"))?,
            withdraw_queue: self
                .withdraw_queue
                .ok_or(PhoenixIxError::MissingField("withdraw_queue"))?,
            global_trader_index,
            active_trader_buffer,
            max_native_sol_amount: self
                .max_native_sol_amount
                .ok_or(PhoenixIxError::MissingField("max_native_sol_amount"))?,
            extra_trader_accounts: self.extra_trader_accounts,
            venue: self.venue.ok_or(PhoenixIxError::MissingField("venue"))?,
        })
    }
}

/// Seize a liquidatee's native SOL collateral at the discounted index price.
/// The signer must be the risk authority or hold the supplied delegated
/// liquidation permission.
pub fn create_liquidate_native_sol_ix(
    params: LiquidateNativeSolParams,
) -> Result<Instruction, PhoenixIxError> {
    let mut data = crate::instructions::PhoenixInstruction::LiquidateNativeSol
        .discriminant()
        .to_vec();
    data.extend_from_slice(&params.max_native_sol_amount.to_le_bytes());
    data.extend_from_slice(
        &u64::try_from(params.extra_trader_accounts.len())
            .map_err(|_| PhoenixIxError::InstructionDataTooLarge)?
            .to_le_bytes(),
    );
    encode_packed_instructions(&mut data, params.venue.instructions());

    let mut accounts = log_accounts();
    accounts.push(AccountMeta::writable(*PHOENIX_GLOBAL_CONFIGURATION));
    accounts.push(AccountMeta::writable(params.perp_asset_map));
    accounts.push(AccountMeta::writable(get_global_vault_address(
        &params.mint,
    )?));
    accounts.push(AccountMeta::readonly(SPL_TOKEN_PROGRAM_ID));
    accounts.push(AccountMeta::readonly(get_native_sol_authority_address()?));
    accounts.push(AccountMeta::writable_signer(params.signer));
    accounts.push(AccountMeta::writable(
        params.permission_account.unwrap_or(params.signer),
    ));
    accounts.push(AccountMeta::writable(params.signer_quote_token_account));
    accounts.push(AccountMeta::writable(params.liquidatee_account));
    accounts.push(AccountMeta::writable(params.withdraw_queue));
    push_trader_index_accounts(
        &mut accounts,
        &params.global_trader_index,
        &params.active_trader_buffer,
    );
    accounts.extend(
        params
            .extra_trader_accounts
            .into_iter()
            .map(AccountMeta::writable),
    );
    accounts.extend_from_slice(params.venue.accounts());

    Ok(Instruction {
        program_id: *PHOENIX_PROGRAM_ID,
        accounts,
        data,
    })
}

/// Slippage protection for a swap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub enum SwapSlippage {
    /// Fail the swap unless the venue returns at least this much of the output
    /// asset. See [`SwapNativeParamsBuilder::min_amount_out`] for the units.
    MinAmountOut(u64),
    /// No slippage protection at all: the swap executes at whatever price the
    /// venue returns.
    ///
    /// Only use this when some other mechanism bounds the price. The program
    /// applies no oracle floor to swaps.
    Unprotected,
}

impl SwapSlippage {
    pub(crate) fn as_min_amount_out(self) -> u64 {
        match self {
            Self::MinAmountOut(min_amount_out) => min_amount_out,
            Self::Unprotected => 0,
        }
    }
}

/// Parameters for [`create_swap_native_ix`].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SwapNativeParams {
    #[serde(with = "crate::instructions::serde_helpers::pubkey")]
    signer: Pubkey,
    #[serde(with = "crate::instructions::serde_helpers::pubkey")]
    trader_account: Pubkey,
    #[serde(with = "crate::instructions::serde_helpers::pubkey")]
    mint: Pubkey,
    #[serde(with = "crate::instructions::serde_helpers::pubkey")]
    perp_asset_map: Pubkey,
    #[serde(with = "crate::instructions::serde_helpers::pubkey")]
    signer_quote_token_account: Pubkey,
    #[serde(with = "crate::instructions::serde_helpers::pubkey")]
    withdraw_queue: Pubkey,
    #[serde(with = "crate::instructions::serde_helpers::pubkey_vec")]
    global_trader_index: Vec<Pubkey>,
    #[serde(with = "crate::instructions::serde_helpers::pubkey_vec")]
    active_trader_buffer: Vec<Pubkey>,
    direction: SwapDirection,
    amount_in: u64,
    slippage: SwapSlippage,
    venue: PackedVenueInstructions,
}

impl SwapNativeParams {
    pub fn builder() -> SwapNativeParamsBuilder {
        SwapNativeParamsBuilder::default()
    }

    pub fn direction(&self) -> SwapDirection {
        self.direction
    }

    pub fn amount_in(&self) -> u64 {
        self.amount_in
    }

    pub fn slippage(&self) -> SwapSlippage {
        self.slippage
    }

    pub fn venue(&self) -> &PackedVenueInstructions {
        &self.venue
    }
}

#[derive(Debug, Clone, Default)]
pub struct SwapNativeParamsBuilder {
    signer: Option<Pubkey>,
    trader_account: Option<Pubkey>,
    mint: Option<Pubkey>,
    perp_asset_map: Option<Pubkey>,
    signer_quote_token_account: Option<Pubkey>,
    withdraw_queue: Option<Pubkey>,
    global_trader_index: Option<Vec<Pubkey>>,
    active_trader_buffer: Option<Vec<Pubkey>>,
    direction: Option<SwapDirection>,
    amount_in: Option<u64>,
    slippage: Option<SwapSlippage>,
    venue: Option<PackedVenueInstructions>,
}

impl SwapNativeParamsBuilder {
    /// The swap signer: either the trader's wallet or its position authority.
    /// Both legs of the swap transit this key's accounts.
    pub fn signer(mut self, signer: Pubkey) -> Self {
        self.signer = Some(signer);
        self
    }

    pub fn trader_account(mut self, trader_account: Pubkey) -> Self {
        self.trader_account = Some(trader_account);
        self
    }

    /// The exchange's canonical quote mint.
    pub fn mint(mut self, mint: Pubkey) -> Self {
        self.mint = Some(mint);
        self
    }

    pub fn perp_asset_map(mut self, perp_asset_map: Pubkey) -> Self {
        self.perp_asset_map = Some(perp_asset_map);
        self
    }

    pub fn signer_quote_token_account(mut self, signer_quote_token_account: Pubkey) -> Self {
        self.signer_quote_token_account = Some(signer_quote_token_account);
        self
    }

    pub fn withdraw_queue(mut self, withdraw_queue: Pubkey) -> Self {
        self.withdraw_queue = Some(withdraw_queue);
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

    pub fn direction(mut self, direction: SwapDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    /// Amount of the *input* asset: lamports for a sell, quote lots for a buy.
    pub fn amount_in(mut self, amount_in: u64) -> Self {
        self.amount_in = Some(amount_in);
        self
    }

    /// Minimum acceptable amount of the **output** asset — quote lots for a
    /// [`SwapDirection::Sell`], lamports for a [`SwapDirection::Buy`].
    ///
    /// Getting the unit wrong either disables protection or makes every swap
    /// fail, so this is required rather than defaulted.
    pub fn min_amount_out(mut self, min_amount_out: u64) -> Self {
        self.slippage = Some(SwapSlippage::MinAmountOut(min_amount_out));
        self
    }

    /// Disable slippage protection entirely.
    ///
    /// The program applies no oracle price floor to swaps, so an unprotected
    /// swap can execute at any price the venue returns. Prefer
    /// [`Self::min_amount_out`].
    pub fn without_slippage_protection(mut self) -> Self {
        self.slippage = Some(SwapSlippage::Unprotected);
        self
    }

    pub fn venue(mut self, venue: PackedVenueInstructions) -> Self {
        self.venue = Some(venue);
        self
    }

    pub fn build(self) -> Result<SwapNativeParams, PhoenixIxError> {
        let amount_in = self
            .amount_in
            .ok_or(PhoenixIxError::MissingField("amount_in"))?;
        if amount_in == 0 {
            return Err(PhoenixIxError::InvalidSwapAmount);
        }

        let (global_trader_index, active_trader_buffer) =
            require_index_accounts(self.global_trader_index, self.active_trader_buffer)?;

        Ok(SwapNativeParams {
            signer: self.signer.ok_or(PhoenixIxError::MissingField("signer"))?,
            trader_account: self
                .trader_account
                .ok_or(PhoenixIxError::MissingField("trader_account"))?,
            mint: self.mint.ok_or(PhoenixIxError::MissingField("mint"))?,
            perp_asset_map: self
                .perp_asset_map
                .ok_or(PhoenixIxError::MissingField("perp_asset_map"))?,
            signer_quote_token_account: self
                .signer_quote_token_account
                .ok_or(PhoenixIxError::MissingField("signer_quote_token_account"))?,
            withdraw_queue: self
                .withdraw_queue
                .ok_or(PhoenixIxError::MissingField("withdraw_queue"))?,
            global_trader_index,
            active_trader_buffer,
            direction: self
                .direction
                .ok_or(PhoenixIxError::MissingField("direction"))?,
            amount_in,
            // Deliberately not defaulted: on-chain a zero disables the check.
            slippage: self
                .slippage
                .ok_or(PhoenixIxError::MissingField("min_amount_out"))?,
            venue: self.venue.ok_or(PhoenixIxError::MissingField("venue"))?,
        })
    }
}

/// Swap between native SOL spot collateral and quote collateral through an
/// external venue.
///
/// The program withdraws the input leg to the signer, runs the caller-supplied
/// venue instructions, deposits the output leg, and then checks that the
/// trader's margin state did not worsen. The exchange-wide withdrawal throttle
/// is charged only the swap's *value loss* (slippage plus venue fees), not the
/// full amount moved.
///
/// # Slippage
///
/// [`SwapNativeParamsBuilder::min_amount_out`] is the **only** price protection
/// this instruction has — there is no oracle floor. Its unit is the output
/// asset: quote lots for a sell, lamports for a buy.
///
/// # Delegated authorities
///
/// The signer may be the trader's *position authority* rather than its wallet,
/// and the venue instructions are entirely caller-supplied. A position
/// authority you do not control can therefore route a delegator's collateral
/// through a venue of its choosing, bounded only by `min_amount_out`. Two
/// The trader's own `disable_position_authority_swap` preference bit gates
/// that path, making such a swap fail with error **7101
/// `PositionAuthoritySwapDisabled`**.
///
/// Swaps signed by the trader's wallet are never gated. Use
/// `phoenix_rise_accounts` to check the preference bit before building the
/// instruction.
///
/// A [`SwapDirection::Buy`] additionally honors the deposit cooldown and is
/// rejected while the trader has a queued withdrawal.
pub fn create_swap_native_ix(params: SwapNativeParams) -> Result<Instruction, PhoenixIxError> {
    let data = encode_swap_data(
        crate::instructions::PhoenixInstruction::SwapNative,
        params.direction,
        params.amount_in,
        params.slippage,
        &params.venue,
    );

    let mut accounts = log_accounts();
    accounts.push(AccountMeta::writable(*PHOENIX_GLOBAL_CONFIGURATION));
    accounts.push(AccountMeta::writable(params.perp_asset_map));
    accounts.push(AccountMeta::writable(get_global_vault_address(
        &params.mint,
    )?));
    accounts.push(AccountMeta::readonly(SPL_TOKEN_PROGRAM_ID));
    accounts.push(AccountMeta::readonly(SYSTEM_PROGRAM_ID));
    accounts.push(AccountMeta::readonly(get_native_sol_authority_address()?));
    accounts.push(AccountMeta::writable_signer(params.signer));
    accounts.push(AccountMeta::writable(params.signer_quote_token_account));
    accounts.push(AccountMeta::writable(params.trader_account));
    accounts.push(AccountMeta::writable(params.withdraw_queue));
    push_trader_index_accounts(
        &mut accounts,
        &params.global_trader_index,
        &params.active_trader_buffer,
    );
    accounts.extend_from_slice(params.venue.accounts());

    Ok(Instruction {
        program_id: *PHOENIX_PROGRAM_ID,
        accounts,
        data,
    })
}

/// Swap instruction data: discriminant, direction, `amount_in`,
/// `min_amount_out`, then the packed venue instructions.
pub(crate) fn encode_swap_data(
    instruction: crate::instructions::PhoenixInstruction,
    direction: SwapDirection,
    amount_in: u64,
    slippage: SwapSlippage,
    venue: &PackedVenueInstructions,
) -> Vec<u8> {
    let mut data = instruction.discriminant().to_vec();
    data.push(direction.as_u8());
    data.extend_from_slice(&amount_in.to_le_bytes());
    data.extend_from_slice(&slippage.as_min_amount_out().to_le_bytes());
    encode_packed_instructions(&mut data, venue.instructions());
    data
}

pub(crate) fn encode_packed_instructions(data: &mut Vec<u8>, instructions: &[PackedInstruction]) {
    data.extend_from_slice(&(instructions.len() as u32).to_le_bytes());
    for instruction in instructions {
        data.push(instruction.program_id_index);
        data.extend_from_slice(&(instruction.data.len() as u32).to_le_bytes());
        data.extend_from_slice(&instruction.data);
        data.extend_from_slice(&(instruction.account_metas.len() as u32).to_le_bytes());
        for meta in &instruction.account_metas {
            data.push(meta.state());
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Shared helpers
////////////////////////////////////////////////////////////////////////////////

pub(crate) fn log_accounts() -> Vec<AccountMeta> {
    vec![
        AccountMeta::readonly(*PHOENIX_PROGRAM_ID),
        AccountMeta::readonly(*PHOENIX_LOG_AUTHORITY),
    ]
}

pub(crate) fn require_index_accounts(
    global_trader_index: Option<Vec<Pubkey>>,
    active_trader_buffer: Option<Vec<Pubkey>>,
) -> Result<(Vec<Pubkey>, Vec<Pubkey>), PhoenixIxError> {
    let global_trader_index =
        global_trader_index.ok_or(PhoenixIxError::MissingField("global_trader_index"))?;
    if global_trader_index.is_empty() {
        return Err(PhoenixIxError::EmptyGlobalTraderIndex);
    }

    let active_trader_buffer =
        active_trader_buffer.ok_or(PhoenixIxError::MissingField("active_trader_buffer"))?;
    if active_trader_buffer.is_empty() {
        return Err(PhoenixIxError::EmptyActiveTraderBuffer);
    }

    Ok((global_trader_index, active_trader_buffer))
}

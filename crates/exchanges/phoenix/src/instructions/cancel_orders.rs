//! Cancel orders by ID instruction construction.

use borsh::{BorshSerialize, to_vec};
use solana_pubkey::Pubkey;

use crate::instructions::constants::{
    PHOENIX_GLOBAL_CONFIGURATION, PHOENIX_LOG_AUTHORITY, PHOENIX_PROGRAM_ID,
};
use crate::instructions::error::PhoenixIxError;
use crate::instructions::types::{
    AccountMeta, CancelId, Instruction, Side, push_trader_index_accounts,
};

/// Maximum number of orders that can be cancelled in a single instruction.
pub const MAX_CANCEL_ORDER_IDS: usize = 100;

/// Parameters for cancelling all orders on one market.
#[derive(Debug, Clone)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct CancelAllParams {
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader_account: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    perp_asset_map: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    orderbook: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    spline_collection: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    global_trader_index: Vec<Pubkey>,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    active_trader_buffer: Vec<Pubkey>,
}

impl CancelAllParams {
    /// Start building with the builder pattern.
    pub fn builder() -> CancelAllParamsBuilder {
        CancelAllParamsBuilder::new()
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

    pub fn orderbook(&self) -> Pubkey {
        self.orderbook
    }

    pub fn spline_collection(&self) -> Pubkey {
        self.spline_collection
    }

    pub fn global_trader_index(&self) -> &[Pubkey] {
        &self.global_trader_index
    }

    pub fn active_trader_buffer(&self) -> &[Pubkey] {
        &self.active_trader_buffer
    }
}

/// Builder for `CancelAllParams`.
#[derive(Default)]
pub struct CancelAllParamsBuilder {
    trader: Option<Pubkey>,
    trader_account: Option<Pubkey>,
    perp_asset_map: Option<Pubkey>,
    orderbook: Option<Pubkey>,
    spline_collection: Option<Pubkey>,
    global_trader_index: Option<Vec<Pubkey>>,
    active_trader_buffer: Option<Vec<Pubkey>>,
}

impl CancelAllParamsBuilder {
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

    pub fn orderbook(mut self, orderbook: Pubkey) -> Self {
        self.orderbook = Some(orderbook);
        self
    }

    pub fn spline_collection(mut self, spline_collection: Pubkey) -> Self {
        self.spline_collection = Some(spline_collection);
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

    pub fn build(self) -> Result<CancelAllParams, PhoenixIxError> {
        Ok(CancelAllParams {
            trader: self.trader.ok_or(PhoenixIxError::MissingField("trader"))?,
            trader_account: self
                .trader_account
                .ok_or(PhoenixIxError::MissingField("trader_account"))?,
            perp_asset_map: self
                .perp_asset_map
                .ok_or(PhoenixIxError::MissingField("perp_asset_map"))?,
            orderbook: self
                .orderbook
                .ok_or(PhoenixIxError::MissingField("orderbook"))?,
            spline_collection: self
                .spline_collection
                .ok_or(PhoenixIxError::MissingField("spline_collection"))?,
            global_trader_index: self
                .global_trader_index
                .ok_or(PhoenixIxError::MissingField("global_trader_index"))?,
            active_trader_buffer: self
                .active_trader_buffer
                .ok_or(PhoenixIxError::MissingField("active_trader_buffer"))?,
        })
    }
}

/// Parameters for cancelling orders up to a side/limit on one market.
#[derive(Debug, Clone)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct CancelUpToParams {
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader_account: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    perp_asset_map: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    orderbook: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    spline_collection: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    global_trader_index: Vec<Pubkey>,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    active_trader_buffer: Vec<Pubkey>,
    side: Side,
    num_orders_to_cancel: Option<u64>,
    tick_limit: Option<u64>,
}

impl CancelUpToParams {
    /// Start building with the builder pattern.
    pub fn builder() -> CancelUpToParamsBuilder {
        CancelUpToParamsBuilder::new()
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

    pub fn orderbook(&self) -> Pubkey {
        self.orderbook
    }

    pub fn spline_collection(&self) -> Pubkey {
        self.spline_collection
    }

    pub fn global_trader_index(&self) -> &[Pubkey] {
        &self.global_trader_index
    }

    pub fn active_trader_buffer(&self) -> &[Pubkey] {
        &self.active_trader_buffer
    }

    pub fn side(&self) -> Side {
        self.side
    }

    pub fn num_orders_to_cancel(&self) -> Option<u64> {
        self.num_orders_to_cancel
    }

    pub fn tick_limit(&self) -> Option<u64> {
        self.tick_limit
    }
}

/// Builder for `CancelUpToParams`.
#[derive(Default)]
pub struct CancelUpToParamsBuilder {
    trader: Option<Pubkey>,
    trader_account: Option<Pubkey>,
    perp_asset_map: Option<Pubkey>,
    orderbook: Option<Pubkey>,
    spline_collection: Option<Pubkey>,
    global_trader_index: Option<Vec<Pubkey>>,
    active_trader_buffer: Option<Vec<Pubkey>>,
    side: Option<Side>,
    num_orders_to_cancel: Option<u64>,
    tick_limit: Option<u64>,
}

impl CancelUpToParamsBuilder {
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

    pub fn orderbook(mut self, orderbook: Pubkey) -> Self {
        self.orderbook = Some(orderbook);
        self
    }

    pub fn spline_collection(mut self, spline_collection: Pubkey) -> Self {
        self.spline_collection = Some(spline_collection);
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

    pub fn side(mut self, side: Side) -> Self {
        self.side = Some(side);
        self
    }

    pub fn num_orders_to_cancel(mut self, num_orders_to_cancel: u64) -> Self {
        self.num_orders_to_cancel = Some(num_orders_to_cancel);
        self
    }

    pub fn tick_limit(mut self, tick_limit: u64) -> Self {
        self.tick_limit = Some(tick_limit);
        self
    }

    pub fn build(self) -> Result<CancelUpToParams, PhoenixIxError> {
        Ok(CancelUpToParams {
            trader: self.trader.ok_or(PhoenixIxError::MissingField("trader"))?,
            trader_account: self
                .trader_account
                .ok_or(PhoenixIxError::MissingField("trader_account"))?,
            perp_asset_map: self
                .perp_asset_map
                .ok_or(PhoenixIxError::MissingField("perp_asset_map"))?,
            orderbook: self
                .orderbook
                .ok_or(PhoenixIxError::MissingField("orderbook"))?,
            spline_collection: self
                .spline_collection
                .ok_or(PhoenixIxError::MissingField("spline_collection"))?,
            global_trader_index: self
                .global_trader_index
                .ok_or(PhoenixIxError::MissingField("global_trader_index"))?,
            active_trader_buffer: self
                .active_trader_buffer
                .ok_or(PhoenixIxError::MissingField("active_trader_buffer"))?,
            side: self.side.ok_or(PhoenixIxError::MissingField("side"))?,
            num_orders_to_cancel: self.num_orders_to_cancel,
            tick_limit: self.tick_limit,
        })
    }
}

/// Parameters for cancelling orders by ID.
#[derive(Debug, Clone)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct CancelOrdersByIdParams {
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    trader_account: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    perp_asset_map: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    orderbook: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    spline_collection: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    global_trader_index: Vec<Pubkey>,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    active_trader_buffer: Vec<Pubkey>,
    order_ids: Vec<CancelId>,
}

impl CancelOrdersByIdParams {
    /// Start building with the builder pattern.
    pub fn builder() -> CancelOrdersByIdParamsBuilder {
        CancelOrdersByIdParamsBuilder::new()
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

    pub fn orderbook(&self) -> Pubkey {
        self.orderbook
    }

    pub fn spline_collection(&self) -> Pubkey {
        self.spline_collection
    }

    pub fn global_trader_index(&self) -> &[Pubkey] {
        &self.global_trader_index
    }

    pub fn active_trader_buffer(&self) -> &[Pubkey] {
        &self.active_trader_buffer
    }

    pub fn order_ids(&self) -> &[CancelId] {
        &self.order_ids
    }
}

/// Builder for `CancelOrdersByIdParams`.
#[derive(Default)]
pub struct CancelOrdersByIdParamsBuilder {
    trader: Option<Pubkey>,
    trader_account: Option<Pubkey>,
    perp_asset_map: Option<Pubkey>,
    orderbook: Option<Pubkey>,
    spline_collection: Option<Pubkey>,
    global_trader_index: Option<Vec<Pubkey>>,
    active_trader_buffer: Option<Vec<Pubkey>>,
    order_ids: Option<Vec<CancelId>>,
}

impl CancelOrdersByIdParamsBuilder {
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

    pub fn orderbook(mut self, orderbook: Pubkey) -> Self {
        self.orderbook = Some(orderbook);
        self
    }

    pub fn spline_collection(mut self, spline_collection: Pubkey) -> Self {
        self.spline_collection = Some(spline_collection);
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

    pub fn order_ids(mut self, order_ids: Vec<CancelId>) -> Self {
        self.order_ids = Some(order_ids);
        self
    }

    pub fn build(self) -> Result<CancelOrdersByIdParams, PhoenixIxError> {
        Ok(CancelOrdersByIdParams {
            trader: self.trader.ok_or(PhoenixIxError::MissingField("trader"))?,
            trader_account: self
                .trader_account
                .ok_or(PhoenixIxError::MissingField("trader_account"))?,
            perp_asset_map: self
                .perp_asset_map
                .ok_or(PhoenixIxError::MissingField("perp_asset_map"))?,
            orderbook: self
                .orderbook
                .ok_or(PhoenixIxError::MissingField("orderbook"))?,
            spline_collection: self
                .spline_collection
                .ok_or(PhoenixIxError::MissingField("spline_collection"))?,
            global_trader_index: self
                .global_trader_index
                .ok_or(PhoenixIxError::MissingField("global_trader_index"))?,
            active_trader_buffer: self
                .active_trader_buffer
                .ok_or(PhoenixIxError::MissingField("active_trader_buffer"))?,
            order_ids: self
                .order_ids
                .ok_or(PhoenixIxError::MissingField("order_ids"))?,
        })
    }
}

/// Create a cancel all instruction.
///
/// # Arguments
///
/// * `params` - The cancel all parameters
///
/// # Returns
///
/// A Solana instruction ready to be included in a transaction.
pub fn create_cancel_all_ix(params: CancelAllParams) -> Result<Instruction, PhoenixIxError> {
    validate_cancel_account_arrays(params.global_trader_index(), params.active_trader_buffer())?;

    Ok(Instruction {
        program_id: *PHOENIX_PROGRAM_ID,
        accounts: build_cancel_order_accounts(
            params.trader(),
            params.trader_account(),
            params.perp_asset_map(),
            params.global_trader_index(),
            params.active_trader_buffer(),
            params.orderbook(),
            params.spline_collection(),
        ),
        data: crate::instructions::PhoenixInstruction::CancelAll
            .discriminant()
            .to_vec(),
    })
}

/// Create a cancel up to instruction.
///
/// # Arguments
///
/// * `params` - The cancel up to parameters
///
/// # Returns
///
/// A Solana instruction ready to be included in a transaction.
pub fn create_cancel_up_to_ix(params: CancelUpToParams) -> Result<Instruction, PhoenixIxError> {
    validate_cancel_account_arrays(params.global_trader_index(), params.active_trader_buffer())?;

    Ok(Instruction {
        program_id: *PHOENIX_PROGRAM_ID,
        accounts: build_cancel_order_accounts(
            params.trader(),
            params.trader_account(),
            params.perp_asset_map(),
            params.global_trader_index(),
            params.active_trader_buffer(),
            params.orderbook(),
            params.spline_collection(),
        ),
        data: encode_cancel_up_to(&params),
    })
}

/// Create a cancel orders by ID instruction.
///
/// # Arguments
///
/// * `params` - The cancel order parameters
///
/// # Returns
///
/// A Solana instruction ready to be included in a transaction.
///
/// # Errors
///
/// Returns an error if:
/// - No order IDs are provided
/// - Too many order IDs (> 100)
/// - Required account arrays are empty
pub fn create_cancel_orders_by_id_ix(
    params: CancelOrdersByIdParams,
) -> Result<Instruction, PhoenixIxError> {
    validate(&params)?;

    let data = encode_cancel_orders(&params);
    let accounts = build_accounts(&params);

    Ok(Instruction {
        program_id: *PHOENIX_PROGRAM_ID,
        accounts,
        data,
    })
}

fn validate(params: &CancelOrdersByIdParams) -> Result<(), PhoenixIxError> {
    validate_cancel_account_arrays(params.global_trader_index(), params.active_trader_buffer())?;
    if params.order_ids().is_empty() {
        return Err(PhoenixIxError::NoOrderIds);
    }
    if params.order_ids().len() > MAX_CANCEL_ORDER_IDS {
        return Err(PhoenixIxError::TooManyOrderIds);
    }
    Ok(())
}

fn validate_cancel_account_arrays(
    global_trader_index: &[Pubkey],
    active_trader_buffer: &[Pubkey],
) -> Result<(), PhoenixIxError> {
    if global_trader_index.is_empty() {
        return Err(PhoenixIxError::EmptyGlobalTraderIndex);
    }
    if active_trader_buffer.is_empty() {
        return Err(PhoenixIxError::EmptyActiveTraderBuffer);
    }
    Ok(())
}

#[derive(BorshSerialize)]
struct CancelUpToData {
    side: Side,
    num_orders_to_cancel: Option<u64>,
    tick_limit: Option<u64>,
}

fn encode_cancel_up_to(params: &CancelUpToParams) -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(&crate::instructions::PhoenixInstruction::CancelUpTo.discriminant());
    data.extend_from_slice(
        &to_vec(&CancelUpToData {
            side: params.side(),
            num_orders_to_cancel: params.num_orders_to_cancel(),
            tick_limit: params.tick_limit(),
        })
        .expect("serialization should not fail"),
    );
    data
}

fn encode_cancel_orders(params: &CancelOrdersByIdParams) -> Vec<u8> {
    let mut data = Vec::new();

    // Instruction discriminant (8 bytes)
    data.extend_from_slice(
        &crate::instructions::PhoenixInstruction::CancelOrdersById.discriminant(),
    );

    // Array length as u32 (Borsh array encoding)
    let len = params.order_ids().len() as u32;
    data.extend_from_slice(&len.to_le_bytes());

    // Each CancelId
    for cancel_id in params.order_ids() {
        data.extend_from_slice(&to_vec(cancel_id).expect("serialization should not fail"));
    }

    data
}

fn build_accounts(params: &CancelOrdersByIdParams) -> Vec<AccountMeta> {
    build_cancel_order_accounts(
        params.trader(),
        params.trader_account(),
        params.perp_asset_map(),
        params.global_trader_index(),
        params.active_trader_buffer(),
        params.orderbook(),
        params.spline_collection(),
    )
}

fn build_cancel_order_accounts(
    trader: Pubkey,
    trader_account: Pubkey,
    perp_asset_map: Pubkey,
    global_trader_index: &[Pubkey],
    active_trader_buffer: &[Pubkey],
    orderbook: Pubkey,
    spline_collection: Pubkey,
) -> Vec<AccountMeta> {
    let mut accounts = Vec::new();

    // LogAccountGroupAccounts (2 accounts)
    accounts.push(AccountMeta::readonly(*PHOENIX_PROGRAM_ID));
    accounts.push(AccountMeta::readonly(*PHOENIX_LOG_AUTHORITY));

    // CancelOrderInstructionGroupAccounts
    accounts.push(AccountMeta::writable(*PHOENIX_GLOBAL_CONFIGURATION));
    accounts.push(AccountMeta::readonly_signer(trader));
    accounts.push(AccountMeta::writable(trader_account));
    accounts.push(AccountMeta::writable(perp_asset_map));

    push_trader_index_accounts(&mut accounts, global_trader_index, active_trader_buffer);

    accounts.push(AccountMeta::writable(orderbook));
    accounts.push(AccountMeta::writable(spline_collection));

    accounts
}

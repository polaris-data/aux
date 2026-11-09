//! Phoenix Hawkeye instruction builders and return-data decoding.

use core::mem::size_of;

use borsh::BorshSerialize;
use bytemuck::{Contiguous, Pod, Zeroable, try_pod_read_unaligned};
use solana_pubkey::Pubkey;
use thiserror::Error;

use crate::instructions::discriminants::HawkeyeInstruction;
use crate::instructions::order_packet::OrderPacket;
use crate::instructions::sha2_const;
use crate::instructions::types::{AccountMeta, Instruction};

/// Phoenix Hawkeye program ID.
pub const HAWKEYE_PROGRAM_ID: Pubkey =
    solana_pubkey::pubkey!("RiSeVw3ZjNfsaXPRb4mgaqYaEEt41pNNJoDvVh7pgQj");

/// Current Hawkeye return-data version.
pub const HAWKEYE_RETURN_VERSION: u16 = 1;

const VIEW_MARGIN_RETURN_MAGIC: u64 = sha2_const(b"return:phoenix_hawkeye_margin");
const VIEW_ASSET_RETURN_MAGIC: u64 = sha2_const(b"return:phoenix_hawkeye_asset");
const VIEW_LIQUIDATION_PRICE_RETURN_MAGIC: u64 =
    sha2_const(b"return:phoenix_hawkeye_liquidation_price");
const VIEW_BBO_RETURN_MAGIC: u64 = sha2_const(b"return:phoenix_hawkeye_bbo");
const VIEW_FUNDING_RETURN_MAGIC: u64 = sha2_const(b"return:phoenix_hawkeye_funding");
const VIEW_ORDER_QUOTE_RETURN_MAGIC: u64 = sha2_const(b"return:phoenix_hawkeye_order_quote");

const VIEW_BBO_HAS_BID: u8 = 1 << 0;
const VIEW_BBO_HAS_ASK: u8 = 1 << 1;
const VIEW_FUNDING_HAS_ACCUMULATED: u8 = 1 << 0;
const VIEW_FUNDING_HAS_UNSETTLED: u8 = 1 << 1;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Pod, Zeroable)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct ViewAssetParams {
    pub asset_id: u32,
    pub _padding: [u8; 4],
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Pod, Zeroable)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct ViewMarginReturn {
    pub magic: u64,
    pub version: u16,
    pub position_count: u16,
    pub risk_state: u8,
    pub risk_tier: u8,
    pub is_liquidatable: u8,
    pub _padding: u8,
    pub collateral_quote_lots: i64,
    pub effective_collateral_quote_lots: i64,
    pub free_collateral_quote_lots: i64,
    pub withdrawable_collateral_quote_lots: u64,
    pub initial_margin_quote_lots: u64,
    pub maintenance_margin_quote_lots: u64,
    pub cancel_margin_quote_lots: u64,
    pub backstop_margin_quote_lots: u64,
    pub high_risk_margin_quote_lots: u64,
    pub unrealized_pnl_quote_lots: i64,
    pub discounted_unrealized_pnl_quote_lots: i64,
    pub unsettled_funding_quote_lots: i64,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Pod, Zeroable)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct ViewAssetReturn {
    pub magic: u64,
    pub asset_id: u32,
    pub version: u16,
    pub has_position_or_orders: u8,
    pub _padding: u8,
    pub risk_state: u8,
    pub risk_tier: u8,
    pub _padding1: [u8; 6],
    pub base_lots: i64,
    pub virtual_quote_lots: i64,
    pub mark_price_ticks: u64,
    pub entry_price_quote_lots_per_base_lot: u64,
    pub position_value_quote_lots: i64,
    pub unrealized_pnl_quote_lots: i64,
    pub discounted_unrealized_pnl_quote_lots: i64,
    pub unsettled_funding_quote_lots: i64,
    pub initial_margin_quote_lots: u64,
    pub maintenance_margin_quote_lots: u64,
    pub cancel_margin_quote_lots: u64,
    pub backstop_margin_quote_lots: u64,
    pub high_risk_margin_quote_lots: u64,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Pod, Zeroable)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct ViewLiquidationPriceReturn {
    pub magic: u64,
    pub asset_id: u32,
    pub version: u16,
    pub status: u8,
    pub side: u8,
    pub liquidation_price_ticks: u64,
    pub mark_price_ticks: u64,
    pub entry_price_quote_lots_per_base_lot: u64,
    pub effective_collateral_quote_lots: i64,
    pub maintenance_margin_quote_lots: u64,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Pod, Zeroable)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct ViewBboReturn {
    pub magic: u64,
    pub version: u16,
    pub flags: u8,
    pub _padding: [u8; 5],
    pub best_bid_ticks: u64,
    pub best_ask_ticks: u64,
    pub mark_price_ticks: u64,
    pub index_price_ticks: u64,
    pub mark_price_last_updated_slot: u64,
    pub index_price_last_updated_slot: u64,
}

impl ViewBboReturn {
    pub fn best_bid_ticks(&self) -> Option<u64> {
        ((self.flags & VIEW_BBO_HAS_BID) != 0).then_some(self.best_bid_ticks)
    }

    pub fn best_ask_ticks(&self) -> Option<u64> {
        ((self.flags & VIEW_BBO_HAS_ASK) != 0).then_some(self.best_ask_ticks)
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Pod, Zeroable)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct ViewFundingReturn {
    pub magic: u64,
    pub asset_id: u32,
    pub version: u16,
    pub flags: u8,
    pub _padding: u8,
    pub total_accumulated_funding_quote_lots: i64,
    pub total_unsettled_funding_quote_lots: i64,
    pub current_funding_rate_micro_bps: i64,
    pub projected_1h_funding_rate_micro_bps: i64,
}

impl ViewFundingReturn {
    pub fn total_accumulated_funding_quote_lots(&self) -> Option<i64> {
        ((self.flags & VIEW_FUNDING_HAS_ACCUMULATED) != 0)
            .then_some(self.total_accumulated_funding_quote_lots)
    }

    pub fn total_unsettled_funding_quote_lots(&self) -> Option<i64> {
        ((self.flags & VIEW_FUNDING_HAS_UNSETTLED) != 0)
            .then_some(self.total_unsettled_funding_quote_lots)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub enum HawkeyeReturnData {
    Margin(ViewMarginReturn),
    Asset(ViewAssetReturn),
    LiquidationPrice(ViewLiquidationPriceReturn),
    Bbo(ViewBboReturn),
    Funding(ViewFundingReturn),
    OrderQuote(ViewOrderQuoteReturn),
}

#[derive(Debug, Error, PartialEq, Eq)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub enum HawkeyeReturnDataError {
    #[error("Hawkeye return data is too short: got {actual} bytes, expected at least 8")]
    MissingMagic { actual: usize },

    #[error("Unknown Hawkeye return data magic {magic}")]
    UnknownMagic { magic: u64 },

    #[error(
        "Invalid Hawkeye return data length for {kind}: got {actual} bytes, expected {expected}"
    )]
    InvalidLength {
        kind: &'static str,
        actual: usize,
        expected: usize,
    },

    #[error("Invalid Hawkeye return data layout for {kind}")]
    InvalidLayout { kind: &'static str },
}

pub fn decode_hawkeye_return_data(
    bytes: &[u8],
) -> Result<HawkeyeReturnData, HawkeyeReturnDataError> {
    if bytes.len() < size_of::<u64>() {
        return Err(HawkeyeReturnDataError::MissingMagic {
            actual: bytes.len(),
        });
    }

    let Some((magic_bytes, _)) = bytes.split_first_chunk::<8>() else {
        return Err(HawkeyeReturnDataError::MissingMagic {
            actual: bytes.len(),
        });
    };
    let magic = u64::from_le_bytes(*magic_bytes);
    match magic {
        VIEW_MARGIN_RETURN_MAGIC => {
            decode_typed(bytes, "view_margin").map(HawkeyeReturnData::Margin)
        }
        VIEW_ASSET_RETURN_MAGIC => {
            decode_typed(bytes, "view_margin_for_asset").map(HawkeyeReturnData::Asset)
        }
        VIEW_LIQUIDATION_PRICE_RETURN_MAGIC => {
            decode_typed(bytes, "view_liquidation_price").map(HawkeyeReturnData::LiquidationPrice)
        }
        VIEW_BBO_RETURN_MAGIC => decode_typed(bytes, "view_bbo").map(HawkeyeReturnData::Bbo),
        VIEW_FUNDING_RETURN_MAGIC => {
            decode_typed(bytes, "view_funding").map(HawkeyeReturnData::Funding)
        }
        VIEW_ORDER_QUOTE_RETURN_MAGIC => {
            decode_typed(bytes, "view_order_quote").map(HawkeyeReturnData::OrderQuote)
        }
        magic => Err(HawkeyeReturnDataError::UnknownMagic { magic }),
    }
}

pub fn decode_hawkeye_return<T: Pod + Copy>(
    bytes: &[u8],
    kind: &'static str,
) -> Result<T, HawkeyeReturnDataError> {
    decode_typed(bytes, kind)
}

fn decode_typed<T: Pod + Copy>(
    bytes: &[u8],
    kind: &'static str,
) -> Result<T, HawkeyeReturnDataError> {
    if bytes.len() != size_of::<T>() {
        return Err(HawkeyeReturnDataError::InvalidLength {
            kind,
            actual: bytes.len(),
            expected: size_of::<T>(),
        });
    }

    try_pod_read_unaligned::<T>(bytes).map_err(|_| HawkeyeReturnDataError::InvalidLayout { kind })
}

#[derive(Debug, Clone)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct HawkeyeTraderViewAccounts {
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    pub phoenix_program_id: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    pub global_config: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    pub global_trader_index: Vec<Pubkey>,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    pub active_trader_buffer: Vec<Pubkey>,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    pub perp_asset_map: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    pub trader: Pubkey,
}

#[derive(Debug, Clone)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct HawkeyeBboViewAccounts {
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    pub phoenix_program_id: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    pub global_config: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    pub global_trader_index: Vec<Pubkey>,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey_vec"))]
    pub active_trader_buffer: Vec<Pubkey>,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    pub perp_asset_map: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    pub orderbook: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    pub spline_collection: Pubkey,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Contiguous)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub enum OrderQuoteOutcome {
    Accepted = 0,
    Rejected = 1,
    Unsupported = 2,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Contiguous)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub enum OrderQuoteRejectionReason {
    None = 0,
    CancelExisting = 1,
    InvalidOrderPacket = 2,
    MinimumFillNotMet = 3,
    SelfTradeAbort = 4,
    PostOnlyCross = 5,
    ReduceOnlyIncreaseExposure = 6,
    TooManyLimitOrders = 7,
    InsufficientAggression = 8,
    InvalidTimeInForce = 9,
    ZeroPrice = 10,
    ZeroSize = 11,
    OutsideExecutionPriceBand = 12,
    Expired = 13,
}

/// Matching-engine quote; does not establish taker margin or authorization.
/// Expected rejections return successfully with zero retained fills, fees and
/// posts. Acceptance does not guarantee placement; enforce placement limits.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Pod, Zeroable)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct ViewOrderQuoteReturn {
    pub magic: u64,
    pub version: u16,
    /// Bit 0: average price; bit 1: slippage; bit 2: effective size is present.
    pub flags: u8,
    /// 0 = bid, 1 = ask.
    pub side: u8,
    pub asset_id: u32,
    pub slot: u64,
    pub requested_base_lots: u64,
    pub filled_base_lots: u64,
    /// Gross execution notional, excluding taker fees.
    pub filled_quote_lots: u64,
    pub fee_quote_lots: u64,
    pub posted_base_lots: u64,
    /// Requested minus filled; includes any posted remainder.
    pub unfilled_base_lots: u64,
    pub average_price_quote_lots_per_base_lot: u64,
    pub reference_price_ticks: u64,
    /// Positive means adverse execution; rounded toward zero. Excludes fees.
    pub slippage_bps: i64,
    /// Size after reduce-only clamping. Absent for rejected, unsupported or
    /// expired orders.
    pub effective_base_lots: u64,
    /// Valid only when posted_base_lots is nonzero; includes post-only sliding.
    pub posted_price_ticks: u64,
    /// 0 = accepted by matcher, 1 = rejected, 2 = unsupported.
    pub outcome: u8,
    /// Stable OrderQuoteRejectionReason code; zero for accepted orders.
    pub rejection_reason: u8,
    pub _padding: [u8; 6],
}

impl ViewOrderQuoteReturn {
    pub fn outcome(&self) -> Option<OrderQuoteOutcome> {
        OrderQuoteOutcome::from_integer(self.outcome)
    }

    pub fn rejection_reason(&self) -> Option<OrderQuoteRejectionReason> {
        OrderQuoteRejectionReason::from_integer(self.rejection_reason)
    }

    pub fn effective_base_lots(&self) -> Option<u64> {
        (self.flags & 4 != 0).then_some(self.effective_base_lots)
    }

    pub fn posted_price_ticks(&self) -> Option<u64> {
        (self.posted_base_lots != 0).then_some(self.posted_price_ticks)
    }

    /// Whether the original requested size filled, excluding any posted lots.
    pub fn is_fully_filled(&self) -> bool {
        self.outcome() == Some(OrderQuoteOutcome::Accepted)
            && self.requested_base_lots > 0
            && self.filled_base_lots == self.requested_base_lots
    }

    /// Signed net quote flow: positive received, negative spent, including
    /// fees. Unknown outcomes and sides cannot be interpreted.
    pub fn net_quote_lots(&self) -> Option<i128> {
        match self.outcome()? {
            OrderQuoteOutcome::Accepted => match self.side {
                0 => Some(-i128::from(self.filled_quote_lots) - i128::from(self.fee_quote_lots)),
                1 => Some(i128::from(self.filled_quote_lots) - i128::from(self.fee_quote_lots)),
                _ => None,
            },
            OrderQuoteOutcome::Rejected | OrderQuoteOutcome::Unsupported => Some(0),
        }
    }

    pub fn average_price_quote_lots_per_base_lot(&self) -> Option<u64> {
        (self.flags & 1 != 0).then_some(self.average_price_quote_lots_per_base_lot)
    }

    pub fn slippage_bps(&self) -> Option<i64> {
        (self.flags & 2 != 0).then_some(self.slippage_bps)
    }
}

const _: () = assert!(size_of::<ViewOrderQuoteReturn>() == 120);

#[derive(Debug, Clone)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct HawkeyeOrderQuoteAccounts {
    pub market: HawkeyeBboViewAccounts,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    pub trader: Pubkey,
    /// Writable, zero-lamport, Hawkeye-owned temporary account. Allocate it in
    /// a preceding top-level System Program instruction in the same
    /// transaction. It is closed after a successful quote. All Phoenix
    /// accounts are readonly.
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    pub scratch: Pubkey,
}

/// Quote an order against copied state. `reference_price_ticks = 0` omits
/// slippage. Packets with margin-dependent `cancel_existing` are unsupported.
///
/// Scratch must fit the source accounts (each rounded up to eight bytes), plus
/// one global-trader-index node and active-position nodes for each spline,
/// the taker's cold positions, and the quoted market. Insufficient scratch
/// fails with AccountDataTooSmall. The buffer is
/// subject to Solana's account-size and transaction-allocation limits.
pub fn create_hawkeye_view_order_quote_ix(
    accounts: HawkeyeOrderQuoteAccounts,
    order_packet: &OrderPacket,
    reference_price_ticks: u64,
) -> Instruction {
    let mut instruction = create_hawkeye_view_bbo_ix(accounts.market);
    instruction
        .accounts
        .push(AccountMeta::readonly(accounts.trader));
    instruction
        .accounts
        .push(AccountMeta::writable(accounts.scratch));
    instruction.data = Vec::from(HawkeyeInstruction::ViewOrderQuote.discriminant());
    instruction
        .data
        .extend_from_slice(&reference_price_ticks.to_le_bytes());
    order_packet
        .serialize(&mut instruction.data)
        .expect("OrderPacket serialization into a Vec cannot fail");
    instruction
}

pub fn create_hawkeye_view_margin_ix(accounts: HawkeyeTraderViewAccounts) -> Instruction {
    hawkeye_trader_instruction(HawkeyeInstruction::ViewMargin, &[], accounts)
}

pub fn create_hawkeye_view_margin_for_asset_ix(
    accounts: HawkeyeTraderViewAccounts,
    asset_id: u32,
) -> Instruction {
    let params = ViewAssetParams {
        asset_id,
        _padding: [0; 4],
    };
    hawkeye_trader_instruction(
        HawkeyeInstruction::ViewMarginForAsset,
        bytemuck::bytes_of(&params),
        accounts,
    )
}

pub fn create_hawkeye_view_liquidation_price_ix(
    accounts: HawkeyeTraderViewAccounts,
    asset_id: u32,
) -> Instruction {
    let params = ViewAssetParams {
        asset_id,
        _padding: [0; 4],
    };
    hawkeye_trader_instruction(
        HawkeyeInstruction::ViewLiquidationPrice,
        bytemuck::bytes_of(&params),
        accounts,
    )
}

pub fn create_hawkeye_view_funding_ix(
    accounts: HawkeyeTraderViewAccounts,
    asset_id: u32,
) -> Instruction {
    let params = ViewAssetParams {
        asset_id,
        _padding: [0; 4],
    };
    hawkeye_trader_instruction(
        HawkeyeInstruction::ViewFunding,
        bytemuck::bytes_of(&params),
        accounts,
    )
}

pub fn create_hawkeye_view_bbo_ix(accounts: HawkeyeBboViewAccounts) -> Instruction {
    let mut account_metas = Vec::with_capacity(
        5 + accounts.global_trader_index.len() + accounts.active_trader_buffer.len(),
    );
    push_hawkeye_base_accounts(
        &mut account_metas,
        accounts.phoenix_program_id,
        accounts.global_config,
        &accounts.global_trader_index,
        &accounts.active_trader_buffer,
        accounts.perp_asset_map,
    );
    account_metas.push(AccountMeta::readonly(accounts.orderbook));
    account_metas.push(AccountMeta::readonly(accounts.spline_collection));
    hawkeye_instruction(HawkeyeInstruction::ViewBbo, &[], account_metas)
}

fn hawkeye_trader_instruction(
    instruction: HawkeyeInstruction,
    params: &[u8],
    accounts: HawkeyeTraderViewAccounts,
) -> Instruction {
    let mut account_metas = Vec::with_capacity(
        4 + accounts.global_trader_index.len() + accounts.active_trader_buffer.len(),
    );
    push_hawkeye_base_accounts(
        &mut account_metas,
        accounts.phoenix_program_id,
        accounts.global_config,
        &accounts.global_trader_index,
        &accounts.active_trader_buffer,
        accounts.perp_asset_map,
    );
    account_metas.push(AccountMeta::readonly(accounts.trader));
    hawkeye_instruction(instruction, params, account_metas)
}

fn push_hawkeye_base_accounts(
    account_metas: &mut Vec<AccountMeta>,
    phoenix_program_id: Pubkey,
    global_config: Pubkey,
    global_trader_index: &[Pubkey],
    active_trader_buffer: &[Pubkey],
    perp_asset_map: Pubkey,
) {
    account_metas.push(AccountMeta::readonly(phoenix_program_id));
    account_metas.push(AccountMeta::readonly(global_config));
    account_metas.extend(
        global_trader_index
            .iter()
            .copied()
            .map(AccountMeta::readonly),
    );
    account_metas.extend(
        active_trader_buffer
            .iter()
            .copied()
            .map(AccountMeta::readonly),
    );
    account_metas.push(AccountMeta::readonly(perp_asset_map));
}

fn hawkeye_instruction(
    instruction: HawkeyeInstruction,
    params: &[u8],
    accounts: Vec<AccountMeta>,
) -> Instruction {
    let mut data = Vec::with_capacity(size_of::<u64>() + params.len());
    data.extend_from_slice(&instruction.discriminant());
    data.extend_from_slice(params);

    Instruction {
        program_id: HAWKEYE_PROGRAM_ID,
        accounts,
        data,
    }
}

//! OrderPacket types for Phoenix instruction serialization.
//!
//! These types match the wire format expected by the Phoenix program,
//! using proper Borsh serialization with `Option<T>` types.

use std::num::NonZeroU8;

use borsh::{BorshDeserialize, BorshSerialize};

use crate::instructions::error::PhoenixIxError;
use crate::instructions::types::{OrderFlags, SelfTradeBehavior, Side};

/// An order packet for Phoenix instructions.
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct OrderPacket {
    pub(crate) kind: OrderPacketKind,
}

impl OrderPacket {
    /// Create a new post-only order packet.
    pub fn post_only(
        side: Side,
        price_in_ticks: u64,
        num_base_lots: u64,
        client_order_id: [u8; 16],
        slide: bool,
        last_valid_slot: Option<u64>,
        order_flags: OrderFlags,
        cancel_existing: bool,
    ) -> Self {
        Self {
            kind: OrderPacketKind::PostOnly {
                side,
                price_in_ticks,
                num_base_lots,
                client_order_id,
                slide,
                last_valid_slot,
                order_flags,
                cancel_existing,
            },
        }
    }

    /// Create a new limit order packet.
    pub fn limit(
        side: Side,
        price_in_ticks: u64,
        num_base_lots: u64,
        self_trade_behavior: SelfTradeBehavior,
        match_limit: Option<u64>,
        client_order_id: [u8; 16],
        last_valid_slot: Option<u64>,
        order_flags: OrderFlags,
        cancel_existing: bool,
    ) -> Self {
        Self {
            kind: OrderPacketKind::Limit {
                side,
                price_in_ticks,
                num_base_lots,
                self_trade_behavior,
                match_limit,
                client_order_id,
                last_valid_slot,
                order_flags,
                cancel_existing,
            },
        }
    }

    /// Create a new immediate-or-cancel order packet (used for market orders).
    pub fn immediate_or_cancel(
        side: Side,
        price_in_ticks: Option<u64>,
        num_base_lots: u64,
        num_quote_lots: Option<u64>,
        min_base_lots_to_fill: u64,
        min_quote_lots_to_fill: u64,
        self_trade_behavior: SelfTradeBehavior,
        match_limit: Option<u64>,
        client_order_id: [u8; 16],
        last_valid_slot: Option<u64>,
        order_flags: OrderFlags,
        cancel_existing: bool,
    ) -> Self {
        Self {
            kind: OrderPacketKind::ImmediateOrCancel {
                side,
                price_in_ticks,
                num_base_lots,
                num_quote_lots,
                min_base_lots_to_fill,
                min_quote_lots_to_fill,
                self_trade_behavior,
                match_limit,
                client_order_id,
                last_valid_slot,
                order_flags,
                cancel_existing,
            },
        }
    }
}

/// The kind of order packet, matching the Phoenix program's enum.
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub(crate) enum OrderPacketKind {
    /// Post-only order that will not match against existing orders.
    PostOnly {
        side: Side,
        price_in_ticks: u64,
        num_base_lots: u64,
        client_order_id: [u8; 16],
        slide: bool,
        last_valid_slot: Option<u64>,
        order_flags: OrderFlags,
        cancel_existing: bool,
    },
    /// Limit order that can match against existing orders.
    Limit {
        side: Side,
        price_in_ticks: u64,
        num_base_lots: u64,
        self_trade_behavior: SelfTradeBehavior,
        match_limit: Option<u64>,
        client_order_id: [u8; 16],
        last_valid_slot: Option<u64>,
        order_flags: OrderFlags,
        cancel_existing: bool,
    },
    /// Immediate-or-cancel order (used for market orders).
    ImmediateOrCancel {
        side: Side,
        price_in_ticks: Option<u64>,
        num_base_lots: u64,
        num_quote_lots: Option<u64>,
        min_base_lots_to_fill: u64,
        min_quote_lots_to_fill: u64,
        self_trade_behavior: SelfTradeBehavior,
        match_limit: Option<u64>,
        client_order_id: [u8; 16],
        last_valid_slot: Option<u64>,
        order_flags: OrderFlags,
        cancel_existing: bool,
    },
}

/// Convert a u128 client order ID to the [u8; 16] format expected by the
/// program.
pub fn client_order_id_to_bytes(id: u128) -> [u8; 16] {
    id.to_le_bytes()
}

/// A condensed order for use in multi-limit-order instructions.
///
/// Contains only price, size, and optional expiry — the minimal data
/// needed per order in a batch.
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct CondensedOrder {
    pub price_in_ticks: u64,
    pub size_in_base_lots: u64,
    pub last_valid_slot: Option<u64>,
}

/// A batch of post-only limit orders (bids and asks) sent in a single
/// instruction.
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct MultipleOrderPacket {
    pub bids: Vec<CondensedOrder>,
    pub asks: Vec<CondensedOrder>,
    pub client_order_id: Option<[u8; 16]>,
    /// Whether orders should slide to the top of the book if they would cross.
    pub slide: bool,
}

/// Fixed-width optional `u64`: `0` encodes `None` on the wire (no Borsh
/// `Option` tag). Same pattern as the market-event type in
/// `phoenix-rise-events`.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, BorshDeserialize, BorshSerialize)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct OptionalNonZeroU64(u64);

impl OptionalNonZeroU64 {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn null() -> Self {
        Self(0)
    }

    pub const fn get(&self) -> Option<u64> {
        if self.0 == 0 { None } else { Some(self.0) }
    }
}

/// One-byte per-order wire flags for [`CondensedOrderV2`]; matches
/// `program-core`'s `CondensedOrderFlags` and the TS enum. Distinct from the
/// account-layout [`OrderFlags`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct CondensedOrderFlags(u8);

impl CondensedOrderFlags {
    pub const REDUCE_ONLY: u8 = 1 << 1;
    /// Slide to the top of the book instead of erroring when the order would
    /// cross.
    pub const SLIDE: u8 = 1 << 0;

    pub const fn from_parts(slide: bool, reduce_only: bool) -> Self {
        let mut bits = 0;
        if slide {
            bits |= Self::SLIDE;
        }
        if reduce_only {
            bits |= Self::REDUCE_ONLY;
        }
        Self(bits)
    }

    pub const fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    pub const fn as_u8(self) -> u8 {
        self.0
    }

    pub const fn is_slide(self) -> bool {
        self.0 & Self::SLIDE != 0
    }

    pub const fn is_reduce_only(self) -> bool {
        self.0 & Self::REDUCE_ONLY != 0
    }
}

/// V2 of [`CondensedOrder`]: fixed-width expiry plus a trailing per-order
/// flags byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct CondensedOrderV2 {
    pub price_in_ticks: u64,
    pub size_in_base_lots: u64,
    pub last_valid_slot: OptionalNonZeroU64,
    pub flags: CondensedOrderFlags,
}

impl CondensedOrderV2 {
    /// `last_valid_slot: Some(0)` collapses to the no-expiry sentinel
    /// ([`OptionalNonZeroU64`] encodes 0 as none), matching program-core's
    /// `CondensedOrderV2Builder`; the TS encoder instead throws on an explicit
    /// 0.
    pub const fn new(
        price_in_ticks: u64,
        size_in_base_lots: u64,
        last_valid_slot: Option<u64>,
        slide: bool,
        reduce_only: bool,
    ) -> Self {
        Self {
            price_in_ticks,
            size_in_base_lots,
            last_valid_slot: match last_valid_slot {
                Some(slot) => OptionalNonZeroU64::new(slot),
                None => OptionalNonZeroU64::null(),
            },
            flags: CondensedOrderFlags::from_parts(slide, reduce_only),
        }
    }
}

/// V2 of [`MultipleOrderPacket`] for `place_multi_limit_order_v2`: per-order
/// flags instead of packet-wide fields, plus a trailing `scale_set_id`.
/// Matches `program-core`'s `MultipleOrderPacketV2` and the TS type.
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct MultipleOrderPacketV2 {
    pub bids: Vec<CondensedOrderV2>,
    pub asks: Vec<CondensedOrderV2>,
    pub client_order_id: Option<[u8; 16]>,
    /// 0 = not part of a scale-order set; 1-127 = caller-assigned ladder id;
    /// bit 7 set = continuation packet. See [`encode_scale_set_tag`].
    pub scale_set_id: u8,
}

/// Largest valid scale-order-set id (the low 7 bits of `scale_set_id`).
pub const MAX_SCALE_SET_ID: u8 = 127;

/// Bit 7 of `scale_set_id`: marks a continuation packet (chunk 2..N of a
/// ladder split across transactions), which the program lets skip its
/// duplicate-id check.
pub const SCALE_SET_CONTINUATION_BIT: u8 = 0x80;

/// Pack a ladder id and continuation flag into the wire `scale_set_id` byte.
/// Rejects `id > MAX_SCALE_SET_ID`.
pub fn encode_scale_set_tag(id: NonZeroU8, continuation: bool) -> Result<u8, PhoenixIxError> {
    if id.get() > MAX_SCALE_SET_ID {
        return Err(PhoenixIxError::InvalidScaleSetId {
            scale_set_id: id.get(),
        });
    }
    Ok(if continuation {
        id.get() | SCALE_SET_CONTINUATION_BIT
    } else {
        id.get()
    })
}

#![allow(clippy::all, unexpected_cfgs)]
//! Local Phoenix perpetuals instruction encoders adapted from the pinned Rise reference.
//! Account addresses, integer lots/ticks, authority and fee context remain explicit.
//! See NOTICE and RISE_LICENSE for source provenance and attribution.
use sha2_const_stable::Sha256;

pub const fn sha2_const(input: &[u8]) -> u64 {
    let hash = Sha256::new().update(input).finalize();
    u64::from_le_bytes([
        hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7],
    ])
}

pub mod cancel_orders;
pub mod cancel_stop_loss;
pub mod claim_fees;
pub mod conditional_order;
pub mod constants;

pub mod create_ata;
pub mod data;
pub mod delegate_trader;
pub mod deposit_funds;
pub mod discriminants;
pub mod ember_deposit;
pub mod ember_withdraw;
pub mod error;
pub mod flight;

pub mod limit_order;
pub mod market_order;
pub mod multi_limit_order;
pub mod native_sol;
pub mod onboard_trader_delegated;
pub mod order_packet;
pub mod place_stop_loss;
pub mod register_trader;

#[cfg(all())]
mod serde_helpers;
pub mod spl_approve;
pub mod spline;
pub mod spot;
pub mod sync_parent_to_child;
pub mod transfer_collateral;
pub mod twap;
pub mod types;
pub mod uncross_crank;
pub mod update_trader_preference_bits;
pub mod withdraw_funds;

pub use constants::{
    EMBER_PROGRAM_ID, PHOENIX_INSTRUCTION_ADDRESSES, PHOENIX_PROGRAM_ID,
    PhoenixInstructionAddresses, phoenix_instruction_addresses, phoenix_program_id,
    resolve_phoenix_instruction_addresses_for_env,
};
pub use discriminants::{
    EmberInstruction, FlickerInstruction, FlightAccount, FlightInstruction, HawkeyeInstruction,
    PhoenixInstruction,
};
pub use error::PhoenixIxError;

pub use twap::FLICKER_PROGRAM_ID;

/// Broad import surface for callers that want the old flat IX API explicitly.
pub mod prelude {

    pub use super::cancel_orders::*;
    pub use super::cancel_stop_loss::*;
    pub use super::claim_fees::*;
    pub use super::conditional_order::*;
    pub use super::constants::*;
    pub use super::create_ata::*;
    pub use super::data::*;
    pub use super::delegate_trader::*;
    pub use super::deposit_funds::*;
    pub use super::discriminants::*;
    pub use super::ember_deposit::*;
    pub use super::ember_withdraw::*;
    pub use super::error::*;
    pub use super::flight::*;

    pub use super::limit_order::*;
    pub use super::market_order::*;
    pub use super::multi_limit_order::*;
    pub use super::native_sol::*;
    pub use super::onboard_trader_delegated::*;
    pub use super::order_packet::*;
    pub use super::place_stop_loss::*;
    pub use super::register_trader::*;

    pub use super::spl_approve::*;
    pub use super::spline::*;
    pub use super::spot::*;
    pub use super::sync_parent_to_child::*;
    pub use super::transfer_collateral::*;
    pub use super::twap::*;
    pub use super::types::*;
    pub use super::uncross_crank::*;
    pub use super::update_trader_preference_bits::*;
    pub use super::withdraw_funds::*;
}

pub mod hawkeye;
pub mod return_data;
pub mod side;

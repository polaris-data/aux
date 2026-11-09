use borsh::{BorshDeserialize, BorshSerialize};
use std::fmt::Display;
/// Order side (Bid or Ask).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BorshSerialize, BorshDeserialize)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(all(), serde(rename_all = "lowercase"))]
#[borsh(use_discriminant = true)]
#[repr(u8)]
pub enum Side {
    #[cfg_attr(all(), serde(alias = "Bid"))]
    Bid = 0,
    #[cfg_attr(all(), serde(alias = "Ask"))]
    Ask = 1,
}

impl Side {
    /// Returns the API wire string for this side (`"buy"` or `"sell"`).
    pub const fn to_api_string(self) -> &'static str {
        match self {
            Self::Bid => "buy",
            Self::Ask => "sell",
        }
    }

    /// Determine side from a FIFO order sequence number.
    #[inline(always)]
    pub const fn from_order_sequence_number(seq: u64) -> Self {
        if seq & (1 << 63) != 0 {
            Self::Bid
        } else {
            Self::Ask
        }
    }
}

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bid => write!(f, "bid"),
            Self::Ask => write!(f, "ask"),
        }
    }
}

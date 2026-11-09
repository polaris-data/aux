//! Return-data layouts emitted by Phoenix CPI entrypoints.

use bytemuck::{Pod, Zeroable, try_pod_read_unaligned};

#[repr(C)]
#[derive(Debug, Eq, PartialEq, Default, Copy, Clone, Pod, Zeroable)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct MatchingEngineFifoOrderId {
    pub price_in_ticks: u64,
    pub order_sequence_number: u64,
}

#[repr(C)]
#[derive(Debug, Eq, PartialEq, Default, Copy, Clone, Pod, Zeroable)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct MatchingEngineCPIResponse {
    order_id: MatchingEngineFifoOrderId,
    pub num_quote_lots_in: u64,
    pub num_base_lots_in: u64,
    pub num_quote_lots_out: u64,
    pub num_base_lots_out: u64,
    pub num_quote_lots_posted: u64,
    pub num_base_lots_posted: u64,
}

impl MatchingEngineCPIResponse {
    pub fn order_id(&self) -> Option<MatchingEngineFifoOrderId> {
        (self.order_id != MatchingEngineFifoOrderId::default()).then_some(self.order_id)
    }
}

pub fn decode_matching_engine_cpi_response(
    data: &[u8],
) -> Result<MatchingEngineCPIResponse, MatchingEngineCPIResponseError> {
    if data.len() != core::mem::size_of::<MatchingEngineCPIResponse>() {
        return Err(MatchingEngineCPIResponseError::InvalidLength {
            expected: core::mem::size_of::<MatchingEngineCPIResponse>(),
            actual: data.len(),
        });
    }
    try_pod_read_unaligned::<MatchingEngineCPIResponse>(data)
        .map_err(|_| MatchingEngineCPIResponseError::InvalidLayout)
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub enum MatchingEngineCPIResponseError {
    #[error("invalid matching-engine CPI response length: expected {expected}, got {actual}")]
    InvalidLength { expected: usize, actual: usize },
    #[error("invalid matching-engine CPI response layout")]
    InvalidLayout,
}

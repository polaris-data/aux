use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Unknown fields are retained so additive native metadata remains accessible.
pub type ExtraFields = BTreeMap<String, Value>;

#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub universe: Vec<Asset>,
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub margin_tables: Vec<(u32, MarginTable)>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}

#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub name: String,
    pub sz_decimals: u32,
    pub max_leverage: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub only_isolated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub is_delisted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub margin_table_id: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub margin_mode: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}

#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginTable {
    pub description: String,
    pub margin_tiers: Vec<MarginTier>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}

#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginTier {
    pub lower_bound: String,
    pub max_leverage: u32,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}

/// Hyperliquid sends full L2 snapshots, with bids first and asks second.
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct L2Book {
    pub coin: String,
    pub time: u64,
    pub levels: [Vec<Level>; 2],
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}

#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Level {
    pub px: String,
    pub sz: String,
    pub n: u64,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}

#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trade {
    pub coin: String,
    pub side: String,
    pub px: String,
    pub sz: String,
    pub hash: String,
    pub time: u64,
    /// Native ID; globally identify trades using (time, coin, tid).
    pub tid: u64,
    pub users: [String; 2],
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}

// Generated from the pinned SBE schema.
#![allow(clippy::derivable_impls)]
#[derive(Clone, serde::Serialize)]
pub struct TradeEvent {
    #[serde(rename = "ts")]
    pub ts: serde_json::Number,
    #[serde(rename = "seq")]
    pub seq: serde_json::Number,
    #[serde(rename = "tradeId")]
    pub trade_id: serde_json::Number,
    #[serde(rename = "side")]
    pub side: Option<String>,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "createdAt")]
    pub created_at: serde_json::Number,
    #[serde(rename = "tradeType")]
    pub trade_type: Option<String>,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "tradeIdStr")]
    pub trade_id_str: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
impl Default for TradeEvent {
    fn default() -> Self {
        Self {
            ts: serde_json::Number::from(0),
            seq: serde_json::Number::from(0),
            trade_id: serde_json::Number::from(0),
            side: Default::default(),
            price: Default::default(),
            size: Default::default(),
            created_at: serde_json::Number::from(0),
            trade_type: Default::default(),
            market: Default::default(),
            trade_id_str: Default::default(),
            extra: Default::default(),
        }
    }
}
impl<'de> serde::Deserialize<'de> for TradeEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Known {
            #[serde(rename = "ts")]
            pub ts: serde_json::Number,
            #[serde(rename = "seq")]
            pub seq: serde_json::Number,
            #[serde(rename = "tradeId")]
            pub trade_id: serde_json::Number,
            #[serde(rename = "side")]
            pub side: Option<String>,
            #[serde(rename = "price")]
            pub price: String,
            #[serde(rename = "size")]
            pub size: String,
            #[serde(rename = "createdAt")]
            pub created_at: serde_json::Number,
            #[serde(rename = "tradeType")]
            pub trade_type: Option<String>,
            #[serde(rename = "market")]
            pub market: String,
            #[serde(rename = "tradeIdStr")]
            pub trade_id_str: String,
        }
        let mut extra = serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;
        let mut fields = serde_json::Map::new();
        for name in [
            "ts",
            "seq",
            "tradeId",
            "side",
            "price",
            "size",
            "createdAt",
            "tradeType",
            "market",
            "tradeIdStr",
        ] {
            if let Some(value) = extra.remove(name) {
                fields.insert(name.to_owned(), value);
            }
        }
        let known: Known = serde_json::from_value(serde_json::Value::Object(fields))
            .map_err(serde::de::Error::custom)?;
        Ok(Self {
            ts: known.ts,
            seq: known.seq,
            trade_id: known.trade_id,
            side: known.side,
            price: known.price,
            size: known.size,
            created_at: known.created_at,
            trade_type: known.trade_type,
            market: known.market,
            trade_id_str: known.trade_id_str,
            extra,
        })
    }
}

#[derive(Clone, serde::Serialize)]
pub struct BboEvent {
    #[serde(rename = "ts")]
    pub ts: serde_json::Number,
    #[serde(rename = "seq")]
    pub seq: serde_json::Number,
    #[serde(rename = "bidPrice")]
    pub bid_price: Option<String>,
    #[serde(rename = "bidSize")]
    pub bid_size: Option<String>,
    #[serde(rename = "askPrice")]
    pub ask_price: Option<String>,
    #[serde(rename = "askSize")]
    pub ask_size: Option<String>,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
impl Default for BboEvent {
    fn default() -> Self {
        Self {
            ts: serde_json::Number::from(0),
            seq: serde_json::Number::from(0),
            bid_price: Default::default(),
            bid_size: Default::default(),
            ask_price: Default::default(),
            ask_size: Default::default(),
            market: Default::default(),
            extra: Default::default(),
        }
    }
}
impl<'de> serde::Deserialize<'de> for BboEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Known {
            #[serde(rename = "ts")]
            pub ts: serde_json::Number,
            #[serde(rename = "seq")]
            pub seq: serde_json::Number,
            #[serde(rename = "bidPrice")]
            pub bid_price: Option<String>,
            #[serde(rename = "bidSize")]
            pub bid_size: Option<String>,
            #[serde(rename = "askPrice")]
            pub ask_price: Option<String>,
            #[serde(rename = "askSize")]
            pub ask_size: Option<String>,
            #[serde(rename = "market")]
            pub market: String,
        }
        let mut extra = serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;
        let mut fields = serde_json::Map::new();
        for name in [
            "ts", "seq", "bidPrice", "bidSize", "askPrice", "askSize", "market",
        ] {
            if let Some(value) = extra.remove(name) {
                fields.insert(name.to_owned(), value);
            }
        }
        let known: Known = serde_json::from_value(serde_json::Value::Object(fields))
            .map_err(serde::de::Error::custom)?;
        Ok(Self {
            ts: known.ts,
            seq: known.seq,
            bid_price: known.bid_price,
            bid_size: known.bid_size,
            ask_price: known.ask_price,
            ask_size: known.ask_size,
            market: known.market,
            extra,
        })
    }
}

#[derive(Clone, serde::Serialize)]
pub struct BookEventBidsItem {
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
impl Default for BookEventBidsItem {
    fn default() -> Self {
        Self {
            price: Default::default(),
            size: Default::default(),
            extra: Default::default(),
        }
    }
}
impl<'de> serde::Deserialize<'de> for BookEventBidsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Known {
            #[serde(rename = "price")]
            pub price: String,
            #[serde(rename = "size")]
            pub size: String,
        }
        let mut extra = serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;
        let mut fields = serde_json::Map::new();
        for name in ["price", "size"] {
            if let Some(value) = extra.remove(name) {
                fields.insert(name.to_owned(), value);
            }
        }
        let known: Known = serde_json::from_value(serde_json::Value::Object(fields))
            .map_err(serde::de::Error::custom)?;
        Ok(Self {
            price: known.price,
            size: known.size,
            extra,
        })
    }
}

#[derive(Clone, serde::Serialize)]
pub struct BookEventAsksItem {
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
impl Default for BookEventAsksItem {
    fn default() -> Self {
        Self {
            price: Default::default(),
            size: Default::default(),
            extra: Default::default(),
        }
    }
}
impl<'de> serde::Deserialize<'de> for BookEventAsksItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Known {
            #[serde(rename = "price")]
            pub price: String,
            #[serde(rename = "size")]
            pub size: String,
        }
        let mut extra = serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;
        let mut fields = serde_json::Map::new();
        for name in ["price", "size"] {
            if let Some(value) = extra.remove(name) {
                fields.insert(name.to_owned(), value);
            }
        }
        let known: Known = serde_json::from_value(serde_json::Value::Object(fields))
            .map_err(serde::de::Error::custom)?;
        Ok(Self {
            price: known.price,
            size: known.size,
            extra,
        })
    }
}

#[derive(Clone, serde::Serialize)]
pub struct BookEvent {
    #[serde(rename = "ts")]
    pub ts: serde_json::Number,
    #[serde(rename = "seq")]
    pub seq: serde_json::Number,
    #[serde(rename = "pkgType")]
    pub pkg_type: Option<String>,
    #[serde(rename = "bestBidPrice")]
    pub best_bid_price: Option<String>,
    #[serde(rename = "bestBidSize")]
    pub best_bid_size: Option<String>,
    #[serde(rename = "bestAskPrice")]
    pub best_ask_price: Option<String>,
    #[serde(rename = "bestAskSize")]
    pub best_ask_size: Option<String>,
    #[serde(rename = "relativeSpread")]
    pub relative_spread: Option<String>,
    #[serde(rename = "bestBidApiPrice")]
    pub best_bid_api_price: Option<String>,
    #[serde(rename = "bestBidApiSize")]
    pub best_bid_api_size: Option<String>,
    #[serde(rename = "bestAskApiPrice")]
    pub best_ask_api_price: Option<String>,
    #[serde(rename = "bestAskApiSize")]
    pub best_ask_api_size: Option<String>,
    #[serde(rename = "bids")]
    pub bids: Vec<BookEventBidsItem>,
    #[serde(rename = "asks")]
    pub asks: Vec<BookEventAsksItem>,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
impl Default for BookEvent {
    fn default() -> Self {
        Self {
            ts: serde_json::Number::from(0),
            seq: serde_json::Number::from(0),
            pkg_type: Default::default(),
            best_bid_price: Default::default(),
            best_bid_size: Default::default(),
            best_ask_price: Default::default(),
            best_ask_size: Default::default(),
            relative_spread: Default::default(),
            best_bid_api_price: Default::default(),
            best_bid_api_size: Default::default(),
            best_ask_api_price: Default::default(),
            best_ask_api_size: Default::default(),
            bids: Default::default(),
            asks: Default::default(),
            market: Default::default(),
            extra: Default::default(),
        }
    }
}
impl<'de> serde::Deserialize<'de> for BookEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Known {
            #[serde(rename = "ts")]
            pub ts: serde_json::Number,
            #[serde(rename = "seq")]
            pub seq: serde_json::Number,
            #[serde(rename = "pkgType")]
            pub pkg_type: Option<String>,
            #[serde(rename = "bestBidPrice")]
            pub best_bid_price: Option<String>,
            #[serde(rename = "bestBidSize")]
            pub best_bid_size: Option<String>,
            #[serde(rename = "bestAskPrice")]
            pub best_ask_price: Option<String>,
            #[serde(rename = "bestAskSize")]
            pub best_ask_size: Option<String>,
            #[serde(rename = "relativeSpread")]
            pub relative_spread: Option<String>,
            #[serde(rename = "bestBidApiPrice")]
            pub best_bid_api_price: Option<String>,
            #[serde(rename = "bestBidApiSize")]
            pub best_bid_api_size: Option<String>,
            #[serde(rename = "bestAskApiPrice")]
            pub best_ask_api_price: Option<String>,
            #[serde(rename = "bestAskApiSize")]
            pub best_ask_api_size: Option<String>,
            #[serde(rename = "bids")]
            pub bids: Vec<BookEventBidsItem>,
            #[serde(rename = "asks")]
            pub asks: Vec<BookEventAsksItem>,
            #[serde(rename = "market")]
            pub market: String,
        }
        let mut extra = serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;
        let mut fields = serde_json::Map::new();
        for name in [
            "ts",
            "seq",
            "pkgType",
            "bestBidPrice",
            "bestBidSize",
            "bestAskPrice",
            "bestAskSize",
            "relativeSpread",
            "bestBidApiPrice",
            "bestBidApiSize",
            "bestAskApiPrice",
            "bestAskApiSize",
            "bids",
            "asks",
            "market",
        ] {
            if let Some(value) = extra.remove(name) {
                fields.insert(name.to_owned(), value);
            }
        }
        let known: Known = serde_json::from_value(serde_json::Value::Object(fields))
            .map_err(serde::de::Error::custom)?;
        Ok(Self {
            ts: known.ts,
            seq: known.seq,
            pkg_type: known.pkg_type,
            best_bid_price: known.best_bid_price,
            best_bid_size: known.best_bid_size,
            best_ask_price: known.best_ask_price,
            best_ask_size: known.best_ask_size,
            relative_spread: known.relative_spread,
            best_bid_api_price: known.best_bid_api_price,
            best_bid_api_size: known.best_bid_api_size,
            best_ask_api_price: known.best_ask_api_price,
            best_ask_api_size: known.best_ask_api_size,
            bids: known.bids,
            asks: known.asks,
            market: known.market,
            extra,
        })
    }
}

#[derive(Clone, serde::Serialize)]
pub struct MarketSummaryEvent {
    #[serde(rename = "ts")]
    pub ts: serde_json::Number,
    #[serde(rename = "seq")]
    pub seq: serde_json::Number,
    #[serde(rename = "markPrice")]
    pub mark_price: String,
    #[serde(rename = "indexPrice")]
    pub index_price: String,
    #[serde(rename = "lastPrice")]
    pub last_price: String,
    #[serde(rename = "volumeUsd24h")]
    pub volume_usd24h: String,
    #[serde(rename = "openInterest")]
    pub open_interest: String,
    #[serde(rename = "fundingRate")]
    pub funding_rate: String,
    #[serde(rename = "bid")]
    pub bid: Option<String>,
    #[serde(rename = "ask")]
    pub ask: Option<String>,
    #[serde(rename = "totalVolume")]
    pub total_volume: String,
    #[serde(rename = "priceChangeRate24h")]
    pub price_change_rate24h: String,
    #[serde(rename = "markIv")]
    pub mark_iv: Option<String>,
    #[serde(rename = "bidIv")]
    pub bid_iv: Option<String>,
    #[serde(rename = "askIv")]
    pub ask_iv: Option<String>,
    #[serde(rename = "lastIv")]
    pub last_iv: Option<String>,
    #[serde(rename = "delta")]
    pub delta: Option<String>,
    #[serde(rename = "gamma")]
    pub gamma: Option<String>,
    #[serde(rename = "vega")]
    pub vega: Option<String>,
    #[serde(rename = "theta")]
    pub theta: Option<String>,
    #[serde(rename = "rho")]
    pub rho: Option<String>,
    #[serde(rename = "volga")]
    pub volga: Option<String>,
    #[serde(rename = "vanna")]
    pub vanna: Option<String>,
    #[serde(rename = "futureFundingRate")]
    pub future_funding_rate: Option<String>,
    #[serde(rename = "externalFairPrice")]
    pub external_fair_price: Option<String>,
    #[serde(rename = "bidSize")]
    pub bid_size: Option<String>,
    #[serde(rename = "askSize")]
    pub ask_size: Option<String>,
    #[serde(rename = "forwardRate")]
    pub forward_rate: Option<String>,
    #[serde(rename = "riskFreeRate")]
    pub risk_free_rate: Option<String>,
    #[serde(rename = "fundingRatePrecise")]
    pub funding_rate_precise: Option<String>,
    #[serde(rename = "settlementTwapEstimate")]
    pub settlement_twap_estimate: Option<String>,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
impl Default for MarketSummaryEvent {
    fn default() -> Self {
        Self {
            ts: serde_json::Number::from(0),
            seq: serde_json::Number::from(0),
            mark_price: Default::default(),
            index_price: Default::default(),
            last_price: Default::default(),
            volume_usd24h: Default::default(),
            open_interest: Default::default(),
            funding_rate: Default::default(),
            bid: Default::default(),
            ask: Default::default(),
            total_volume: Default::default(),
            price_change_rate24h: Default::default(),
            mark_iv: Default::default(),
            bid_iv: Default::default(),
            ask_iv: Default::default(),
            last_iv: Default::default(),
            delta: Default::default(),
            gamma: Default::default(),
            vega: Default::default(),
            theta: Default::default(),
            rho: Default::default(),
            volga: Default::default(),
            vanna: Default::default(),
            future_funding_rate: Default::default(),
            external_fair_price: Default::default(),
            bid_size: Default::default(),
            ask_size: Default::default(),
            forward_rate: Default::default(),
            risk_free_rate: Default::default(),
            funding_rate_precise: Default::default(),
            settlement_twap_estimate: Default::default(),
            market: Default::default(),
            extra: Default::default(),
        }
    }
}
impl<'de> serde::Deserialize<'de> for MarketSummaryEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Known {
            #[serde(rename = "ts")]
            pub ts: serde_json::Number,
            #[serde(rename = "seq")]
            pub seq: serde_json::Number,
            #[serde(rename = "markPrice")]
            pub mark_price: String,
            #[serde(rename = "indexPrice")]
            pub index_price: String,
            #[serde(rename = "lastPrice")]
            pub last_price: String,
            #[serde(rename = "volumeUsd24h")]
            pub volume_usd24h: String,
            #[serde(rename = "openInterest")]
            pub open_interest: String,
            #[serde(rename = "fundingRate")]
            pub funding_rate: String,
            #[serde(rename = "bid")]
            pub bid: Option<String>,
            #[serde(rename = "ask")]
            pub ask: Option<String>,
            #[serde(rename = "totalVolume")]
            pub total_volume: String,
            #[serde(rename = "priceChangeRate24h")]
            pub price_change_rate24h: String,
            #[serde(rename = "markIv")]
            pub mark_iv: Option<String>,
            #[serde(rename = "bidIv")]
            pub bid_iv: Option<String>,
            #[serde(rename = "askIv")]
            pub ask_iv: Option<String>,
            #[serde(rename = "lastIv")]
            pub last_iv: Option<String>,
            #[serde(rename = "delta")]
            pub delta: Option<String>,
            #[serde(rename = "gamma")]
            pub gamma: Option<String>,
            #[serde(rename = "vega")]
            pub vega: Option<String>,
            #[serde(rename = "theta")]
            pub theta: Option<String>,
            #[serde(rename = "rho")]
            pub rho: Option<String>,
            #[serde(rename = "volga")]
            pub volga: Option<String>,
            #[serde(rename = "vanna")]
            pub vanna: Option<String>,
            #[serde(rename = "futureFundingRate")]
            pub future_funding_rate: Option<String>,
            #[serde(rename = "externalFairPrice")]
            pub external_fair_price: Option<String>,
            #[serde(rename = "bidSize")]
            pub bid_size: Option<String>,
            #[serde(rename = "askSize")]
            pub ask_size: Option<String>,
            #[serde(rename = "forwardRate")]
            pub forward_rate: Option<String>,
            #[serde(rename = "riskFreeRate")]
            pub risk_free_rate: Option<String>,
            #[serde(rename = "fundingRatePrecise")]
            pub funding_rate_precise: Option<String>,
            #[serde(rename = "settlementTwapEstimate")]
            pub settlement_twap_estimate: Option<String>,
            #[serde(rename = "market")]
            pub market: String,
        }
        let mut extra = serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;
        let mut fields = serde_json::Map::new();
        for name in [
            "ts",
            "seq",
            "markPrice",
            "indexPrice",
            "lastPrice",
            "volumeUsd24h",
            "openInterest",
            "fundingRate",
            "bid",
            "ask",
            "totalVolume",
            "priceChangeRate24h",
            "markIv",
            "bidIv",
            "askIv",
            "lastIv",
            "delta",
            "gamma",
            "vega",
            "theta",
            "rho",
            "volga",
            "vanna",
            "futureFundingRate",
            "externalFairPrice",
            "bidSize",
            "askSize",
            "forwardRate",
            "riskFreeRate",
            "fundingRatePrecise",
            "settlementTwapEstimate",
            "market",
        ] {
            if let Some(value) = extra.remove(name) {
                fields.insert(name.to_owned(), value);
            }
        }
        let known: Known = serde_json::from_value(serde_json::Value::Object(fields))
            .map_err(serde::de::Error::custom)?;
        Ok(Self {
            ts: known.ts,
            seq: known.seq,
            mark_price: known.mark_price,
            index_price: known.index_price,
            last_price: known.last_price,
            volume_usd24h: known.volume_usd24h,
            open_interest: known.open_interest,
            funding_rate: known.funding_rate,
            bid: known.bid,
            ask: known.ask,
            total_volume: known.total_volume,
            price_change_rate24h: known.price_change_rate24h,
            mark_iv: known.mark_iv,
            bid_iv: known.bid_iv,
            ask_iv: known.ask_iv,
            last_iv: known.last_iv,
            delta: known.delta,
            gamma: known.gamma,
            vega: known.vega,
            theta: known.theta,
            rho: known.rho,
            volga: known.volga,
            vanna: known.vanna,
            future_funding_rate: known.future_funding_rate,
            external_fair_price: known.external_fair_price,
            bid_size: known.bid_size,
            ask_size: known.ask_size,
            forward_rate: known.forward_rate,
            risk_free_rate: known.risk_free_rate,
            funding_rate_precise: known.funding_rate_precise,
            settlement_twap_estimate: known.settlement_twap_estimate,
            market: known.market,
            extra,
        })
    }
}

#[derive(Clone, serde::Serialize)]
pub struct FundingDataEvent {
    #[serde(rename = "ts")]
    pub ts: serde_json::Number,
    #[serde(rename = "seq")]
    pub seq: serde_json::Number,
    #[serde(rename = "fundingRate")]
    pub funding_rate: String,
    #[serde(rename = "fundingIndex")]
    pub funding_index: String,
    #[serde(rename = "createdAt")]
    pub created_at: serde_json::Number,
    #[serde(rename = "fundingPremium")]
    pub funding_premium: String,
    #[serde(rename = "fundingRate8h")]
    pub funding_rate8h: String,
    #[serde(rename = "fundingPeriodHours")]
    pub funding_period_hours: serde_json::Number,
    #[serde(rename = "impactPremiumRate")]
    pub impact_premium_rate: Option<String>,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
impl Default for FundingDataEvent {
    fn default() -> Self {
        Self {
            ts: serde_json::Number::from(0),
            seq: serde_json::Number::from(0),
            funding_rate: Default::default(),
            funding_index: Default::default(),
            created_at: serde_json::Number::from(0),
            funding_premium: Default::default(),
            funding_rate8h: Default::default(),
            funding_period_hours: serde_json::Number::from(0),
            impact_premium_rate: Default::default(),
            market: Default::default(),
            extra: Default::default(),
        }
    }
}
impl<'de> serde::Deserialize<'de> for FundingDataEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Known {
            #[serde(rename = "ts")]
            pub ts: serde_json::Number,
            #[serde(rename = "seq")]
            pub seq: serde_json::Number,
            #[serde(rename = "fundingRate")]
            pub funding_rate: String,
            #[serde(rename = "fundingIndex")]
            pub funding_index: String,
            #[serde(rename = "createdAt")]
            pub created_at: serde_json::Number,
            #[serde(rename = "fundingPremium")]
            pub funding_premium: String,
            #[serde(rename = "fundingRate8h")]
            pub funding_rate8h: String,
            #[serde(rename = "fundingPeriodHours")]
            pub funding_period_hours: serde_json::Number,
            #[serde(rename = "impactPremiumRate")]
            pub impact_premium_rate: Option<String>,
            #[serde(rename = "market")]
            pub market: String,
        }
        let mut extra = serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;
        let mut fields = serde_json::Map::new();
        for name in [
            "ts",
            "seq",
            "fundingRate",
            "fundingIndex",
            "createdAt",
            "fundingPremium",
            "fundingRate8h",
            "fundingPeriodHours",
            "impactPremiumRate",
            "market",
        ] {
            if let Some(value) = extra.remove(name) {
                fields.insert(name.to_owned(), value);
            }
        }
        let known: Known = serde_json::from_value(serde_json::Value::Object(fields))
            .map_err(serde::de::Error::custom)?;
        Ok(Self {
            ts: known.ts,
            seq: known.seq,
            funding_rate: known.funding_rate,
            funding_index: known.funding_index,
            created_at: known.created_at,
            funding_premium: known.funding_premium,
            funding_rate8h: known.funding_rate8h,
            funding_period_hours: known.funding_period_hours,
            impact_premium_rate: known.impact_premium_rate,
            market: known.market,
            extra,
        })
    }
}

#[derive(Clone, serde::Serialize)]
pub struct FundingRateComparisonEvent {
    #[serde(rename = "ts")]
    pub ts: serde_json::Number,
    #[serde(rename = "lastUpdatedAt")]
    pub last_updated_at: serde_json::Number,
    #[serde(rename = "hourlyFundingRate")]
    pub hourly_funding_rate: String,
    #[serde(rename = "assetKind")]
    pub asset_kind: Option<String>,
    #[serde(rename = "source")]
    pub source: Option<String>,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
impl Default for FundingRateComparisonEvent {
    fn default() -> Self {
        Self {
            ts: serde_json::Number::from(0),
            last_updated_at: serde_json::Number::from(0),
            hourly_funding_rate: Default::default(),
            asset_kind: Default::default(),
            source: Default::default(),
            market: Default::default(),
            extra: Default::default(),
        }
    }
}
impl<'de> serde::Deserialize<'de> for FundingRateComparisonEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Known {
            #[serde(rename = "ts")]
            pub ts: serde_json::Number,
            #[serde(rename = "lastUpdatedAt")]
            pub last_updated_at: serde_json::Number,
            #[serde(rename = "hourlyFundingRate")]
            pub hourly_funding_rate: String,
            #[serde(rename = "assetKind")]
            pub asset_kind: Option<String>,
            #[serde(rename = "source")]
            pub source: Option<String>,
            #[serde(rename = "market")]
            pub market: String,
        }
        let mut extra = serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;
        let mut fields = serde_json::Map::new();
        for name in [
            "ts",
            "lastUpdatedAt",
            "hourlyFundingRate",
            "assetKind",
            "source",
            "market",
        ] {
            if let Some(value) = extra.remove(name) {
                fields.insert(name.to_owned(), value);
            }
        }
        let known: Known = serde_json::from_value(serde_json::Value::Object(fields))
            .map_err(serde::de::Error::custom)?;
        Ok(Self {
            ts: known.ts,
            last_updated_at: known.last_updated_at,
            hourly_funding_rate: known.hourly_funding_rate,
            asset_kind: known.asset_kind,
            source: known.source,
            market: known.market,
            extra,
        })
    }
}

#[derive(Clone, serde::Serialize)]
pub struct OrderEvent {
    #[serde(rename = "ts")]
    pub ts: serde_json::Number,
    #[serde(rename = "seq")]
    pub seq: serde_json::Number,
    #[serde(rename = "status")]
    pub status: Option<String>,
    #[serde(rename = "side")]
    pub side: Option<String>,
    #[serde(rename = "orderType")]
    pub order_type: Option<String>,
    #[serde(rename = "timeInForce")]
    pub time_in_force: Option<String>,
    #[serde(rename = "price")]
    pub price: Option<String>,
    #[serde(rename = "triggerPrice")]
    pub trigger_price: Option<String>,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "sizeOpen")]
    pub size_open: String,
    #[serde(rename = "avgFillPrice")]
    pub avg_fill_price: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: serde_json::Number,
    #[serde(rename = "updatedAt")]
    pub updated_at: serde_json::Number,
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "receivedAt")]
    pub received_at: serde_json::Number,
    #[serde(rename = "publishedAt")]
    pub published_at: serde_json::Number,
    #[serde(rename = "stp")]
    pub stp: Option<String>,
    #[serde(rename = "flags")]
    pub flags: Vec<String>,
    #[serde(rename = "requestStatus")]
    pub request_status: Option<String>,
    #[serde(rename = "requestType")]
    pub request_type: Option<String>,
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "cancelReason")]
    pub cancel_reason: String,
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[serde(rename = "requestMessage")]
    pub request_message: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
impl Default for OrderEvent {
    fn default() -> Self {
        Self {
            ts: serde_json::Number::from(0),
            seq: serde_json::Number::from(0),
            status: Default::default(),
            side: Default::default(),
            order_type: Default::default(),
            time_in_force: Default::default(),
            price: Default::default(),
            trigger_price: Default::default(),
            size: Default::default(),
            size_open: Default::default(),
            avg_fill_price: Default::default(),
            created_at: serde_json::Number::from(0),
            updated_at: serde_json::Number::from(0),
            account: Default::default(),
            received_at: serde_json::Number::from(0),
            published_at: serde_json::Number::from(0),
            stp: Default::default(),
            flags: Default::default(),
            request_status: Default::default(),
            request_type: Default::default(),
            order_id: Default::default(),
            client_order_id: Default::default(),
            market: Default::default(),
            cancel_reason: Default::default(),
            request_id: Default::default(),
            request_message: Default::default(),
            extra: Default::default(),
        }
    }
}
impl<'de> serde::Deserialize<'de> for OrderEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Known {
            #[serde(rename = "ts")]
            pub ts: serde_json::Number,
            #[serde(rename = "seq")]
            pub seq: serde_json::Number,
            #[serde(rename = "status")]
            pub status: Option<String>,
            #[serde(rename = "side")]
            pub side: Option<String>,
            #[serde(rename = "orderType")]
            pub order_type: Option<String>,
            #[serde(rename = "timeInForce")]
            pub time_in_force: Option<String>,
            #[serde(rename = "price")]
            pub price: Option<String>,
            #[serde(rename = "triggerPrice")]
            pub trigger_price: Option<String>,
            #[serde(rename = "size")]
            pub size: String,
            #[serde(rename = "sizeOpen")]
            pub size_open: String,
            #[serde(rename = "avgFillPrice")]
            pub avg_fill_price: Option<String>,
            #[serde(rename = "createdAt")]
            pub created_at: serde_json::Number,
            #[serde(rename = "updatedAt")]
            pub updated_at: serde_json::Number,
            #[serde(rename = "account")]
            pub account: String,
            #[serde(rename = "receivedAt")]
            pub received_at: serde_json::Number,
            #[serde(rename = "publishedAt")]
            pub published_at: serde_json::Number,
            #[serde(rename = "stp")]
            pub stp: Option<String>,
            #[serde(rename = "flags")]
            pub flags: Vec<String>,
            #[serde(rename = "requestStatus")]
            pub request_status: Option<String>,
            #[serde(rename = "requestType")]
            pub request_type: Option<String>,
            #[serde(rename = "orderId")]
            pub order_id: String,
            #[serde(rename = "clientOrderId")]
            pub client_order_id: String,
            #[serde(rename = "market")]
            pub market: String,
            #[serde(rename = "cancelReason")]
            pub cancel_reason: String,
            #[serde(rename = "requestId")]
            pub request_id: String,
            #[serde(rename = "requestMessage")]
            pub request_message: String,
        }
        let mut extra = serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;
        let mut fields = serde_json::Map::new();
        for name in [
            "ts",
            "seq",
            "status",
            "side",
            "orderType",
            "timeInForce",
            "price",
            "triggerPrice",
            "size",
            "sizeOpen",
            "avgFillPrice",
            "createdAt",
            "updatedAt",
            "account",
            "receivedAt",
            "publishedAt",
            "stp",
            "flags",
            "requestStatus",
            "requestType",
            "orderId",
            "clientOrderId",
            "market",
            "cancelReason",
            "requestId",
            "requestMessage",
        ] {
            if let Some(value) = extra.remove(name) {
                fields.insert(name.to_owned(), value);
            }
        }
        let known: Known = serde_json::from_value(serde_json::Value::Object(fields))
            .map_err(serde::de::Error::custom)?;
        Ok(Self {
            ts: known.ts,
            seq: known.seq,
            status: known.status,
            side: known.side,
            order_type: known.order_type,
            time_in_force: known.time_in_force,
            price: known.price,
            trigger_price: known.trigger_price,
            size: known.size,
            size_open: known.size_open,
            avg_fill_price: known.avg_fill_price,
            created_at: known.created_at,
            updated_at: known.updated_at,
            account: known.account,
            received_at: known.received_at,
            published_at: known.published_at,
            stp: known.stp,
            flags: known.flags,
            request_status: known.request_status,
            request_type: known.request_type,
            order_id: known.order_id,
            client_order_id: known.client_order_id,
            market: known.market,
            cancel_reason: known.cancel_reason,
            request_id: known.request_id,
            request_message: known.request_message,
            extra,
        })
    }
}

#[derive(Clone, serde::Serialize)]
pub struct FillEvent {
    #[serde(rename = "ts")]
    pub ts: serde_json::Number,
    #[serde(rename = "seq")]
    pub seq: Option<serde_json::Number>,
    #[serde(rename = "fillType")]
    pub fill_type: Option<String>,
    #[serde(rename = "side")]
    pub side: Option<String>,
    #[serde(rename = "liquidity")]
    pub liquidity: Option<String>,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "fee")]
    pub fee: String,
    #[serde(rename = "realizedPnl")]
    pub realized_pnl: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: serde_json::Number,
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "underlyingPrice")]
    pub underlying_price: String,
    #[serde(rename = "realizedFunding")]
    pub realized_funding: String,
    #[serde(rename = "flags")]
    pub flags: Vec<String>,
    #[serde(rename = "orderbookSeqNo")]
    pub orderbook_seq_no: serde_json::Number,
    #[serde(rename = "fillId")]
    pub fill_id: String,
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    #[serde(rename = "tradeId")]
    pub trade_id: String,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "feeCurrency")]
    pub fee_currency: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
impl Default for FillEvent {
    fn default() -> Self {
        Self {
            ts: serde_json::Number::from(0),
            seq: Default::default(),
            fill_type: Default::default(),
            side: Default::default(),
            liquidity: Default::default(),
            price: Default::default(),
            size: Default::default(),
            fee: Default::default(),
            realized_pnl: Default::default(),
            created_at: serde_json::Number::from(0),
            account: Default::default(),
            underlying_price: Default::default(),
            realized_funding: Default::default(),
            flags: Default::default(),
            orderbook_seq_no: serde_json::Number::from(0),
            fill_id: Default::default(),
            order_id: Default::default(),
            client_order_id: Default::default(),
            trade_id: Default::default(),
            market: Default::default(),
            fee_currency: Default::default(),
            extra: Default::default(),
        }
    }
}
impl<'de> serde::Deserialize<'de> for FillEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Known {
            #[serde(rename = "ts")]
            pub ts: serde_json::Number,
            #[serde(rename = "seq")]
            pub seq: Option<serde_json::Number>,
            #[serde(rename = "fillType")]
            pub fill_type: Option<String>,
            #[serde(rename = "side")]
            pub side: Option<String>,
            #[serde(rename = "liquidity")]
            pub liquidity: Option<String>,
            #[serde(rename = "price")]
            pub price: String,
            #[serde(rename = "size")]
            pub size: String,
            #[serde(rename = "fee")]
            pub fee: String,
            #[serde(rename = "realizedPnl")]
            pub realized_pnl: Option<String>,
            #[serde(rename = "createdAt")]
            pub created_at: serde_json::Number,
            #[serde(rename = "account")]
            pub account: String,
            #[serde(rename = "underlyingPrice")]
            pub underlying_price: String,
            #[serde(rename = "realizedFunding")]
            pub realized_funding: String,
            #[serde(rename = "flags")]
            pub flags: Vec<String>,
            #[serde(rename = "orderbookSeqNo")]
            pub orderbook_seq_no: serde_json::Number,
            #[serde(rename = "fillId")]
            pub fill_id: String,
            #[serde(rename = "orderId")]
            pub order_id: String,
            #[serde(rename = "clientOrderId")]
            pub client_order_id: String,
            #[serde(rename = "tradeId")]
            pub trade_id: String,
            #[serde(rename = "market")]
            pub market: String,
            #[serde(rename = "feeCurrency")]
            pub fee_currency: String,
        }
        let mut extra = serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;
        let mut fields = serde_json::Map::new();
        for name in [
            "ts",
            "seq",
            "fillType",
            "side",
            "liquidity",
            "price",
            "size",
            "fee",
            "realizedPnl",
            "createdAt",
            "account",
            "underlyingPrice",
            "realizedFunding",
            "flags",
            "orderbookSeqNo",
            "fillId",
            "orderId",
            "clientOrderId",
            "tradeId",
            "market",
            "feeCurrency",
        ] {
            if let Some(value) = extra.remove(name) {
                fields.insert(name.to_owned(), value);
            }
        }
        let known: Known = serde_json::from_value(serde_json::Value::Object(fields))
            .map_err(serde::de::Error::custom)?;
        Ok(Self {
            ts: known.ts,
            seq: known.seq,
            fill_type: known.fill_type,
            side: known.side,
            liquidity: known.liquidity,
            price: known.price,
            size: known.size,
            fee: known.fee,
            realized_pnl: known.realized_pnl,
            created_at: known.created_at,
            account: known.account,
            underlying_price: known.underlying_price,
            realized_funding: known.realized_funding,
            flags: known.flags,
            orderbook_seq_no: known.orderbook_seq_no,
            fill_id: known.fill_id,
            order_id: known.order_id,
            client_order_id: known.client_order_id,
            trade_id: known.trade_id,
            market: known.market,
            fee_currency: known.fee_currency,
            extra,
        })
    }
}

#[derive(Clone, serde::Serialize)]
pub struct PositionEvent {
    #[serde(rename = "ts")]
    pub ts: serde_json::Number,
    #[serde(rename = "seq")]
    pub seq: serde_json::Number,
    #[serde(rename = "side")]
    pub side: Option<String>,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "avgEntryPrice")]
    pub avg_entry_price: String,
    #[serde(rename = "unrealizedPnl")]
    pub unrealized_pnl: String,
    #[serde(rename = "realizedPnl")]
    pub realized_pnl: String,
    #[serde(rename = "markPrice")]
    pub mark_price: Option<String>,
    #[serde(rename = "liquidationPrice")]
    pub liquidation_price: Option<String>,
    #[serde(rename = "leverage")]
    pub leverage: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: serde_json::Number,
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "avgEntryPriceUsd")]
    pub avg_entry_price_usd: String,
    #[serde(rename = "cost")]
    pub cost: String,
    #[serde(rename = "costUsd")]
    pub cost_usd: String,
    #[serde(rename = "cachedFundingIndex")]
    pub cached_funding_index: String,
    #[serde(rename = "unrealizedFundingPnl")]
    pub unrealized_funding_pnl: String,
    #[serde(rename = "status")]
    pub status: Option<String>,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "lastFillId")]
    pub last_fill_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
impl Default for PositionEvent {
    fn default() -> Self {
        Self {
            ts: serde_json::Number::from(0),
            seq: serde_json::Number::from(0),
            side: Default::default(),
            size: Default::default(),
            avg_entry_price: Default::default(),
            unrealized_pnl: Default::default(),
            realized_pnl: Default::default(),
            mark_price: Default::default(),
            liquidation_price: Default::default(),
            leverage: Default::default(),
            updated_at: serde_json::Number::from(0),
            account: Default::default(),
            avg_entry_price_usd: Default::default(),
            cost: Default::default(),
            cost_usd: Default::default(),
            cached_funding_index: Default::default(),
            unrealized_funding_pnl: Default::default(),
            status: Default::default(),
            market: Default::default(),
            last_fill_id: Default::default(),
            extra: Default::default(),
        }
    }
}
impl<'de> serde::Deserialize<'de> for PositionEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Known {
            #[serde(rename = "ts")]
            pub ts: serde_json::Number,
            #[serde(rename = "seq")]
            pub seq: serde_json::Number,
            #[serde(rename = "side")]
            pub side: Option<String>,
            #[serde(rename = "size")]
            pub size: String,
            #[serde(rename = "avgEntryPrice")]
            pub avg_entry_price: String,
            #[serde(rename = "unrealizedPnl")]
            pub unrealized_pnl: String,
            #[serde(rename = "realizedPnl")]
            pub realized_pnl: String,
            #[serde(rename = "markPrice")]
            pub mark_price: Option<String>,
            #[serde(rename = "liquidationPrice")]
            pub liquidation_price: Option<String>,
            #[serde(rename = "leverage")]
            pub leverage: String,
            #[serde(rename = "updatedAt")]
            pub updated_at: serde_json::Number,
            #[serde(rename = "account")]
            pub account: String,
            #[serde(rename = "avgEntryPriceUsd")]
            pub avg_entry_price_usd: String,
            #[serde(rename = "cost")]
            pub cost: String,
            #[serde(rename = "costUsd")]
            pub cost_usd: String,
            #[serde(rename = "cachedFundingIndex")]
            pub cached_funding_index: String,
            #[serde(rename = "unrealizedFundingPnl")]
            pub unrealized_funding_pnl: String,
            #[serde(rename = "status")]
            pub status: Option<String>,
            #[serde(rename = "market")]
            pub market: String,
            #[serde(rename = "lastFillId")]
            pub last_fill_id: String,
        }
        let mut extra = serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;
        let mut fields = serde_json::Map::new();
        for name in [
            "ts",
            "seq",
            "side",
            "size",
            "avgEntryPrice",
            "unrealizedPnl",
            "realizedPnl",
            "markPrice",
            "liquidationPrice",
            "leverage",
            "updatedAt",
            "account",
            "avgEntryPriceUsd",
            "cost",
            "costUsd",
            "cachedFundingIndex",
            "unrealizedFundingPnl",
            "status",
            "market",
            "lastFillId",
        ] {
            if let Some(value) = extra.remove(name) {
                fields.insert(name.to_owned(), value);
            }
        }
        let known: Known = serde_json::from_value(serde_json::Value::Object(fields))
            .map_err(serde::de::Error::custom)?;
        Ok(Self {
            ts: known.ts,
            seq: known.seq,
            side: known.side,
            size: known.size,
            avg_entry_price: known.avg_entry_price,
            unrealized_pnl: known.unrealized_pnl,
            realized_pnl: known.realized_pnl,
            mark_price: known.mark_price,
            liquidation_price: known.liquidation_price,
            leverage: known.leverage,
            updated_at: known.updated_at,
            account: known.account,
            avg_entry_price_usd: known.avg_entry_price_usd,
            cost: known.cost,
            cost_usd: known.cost_usd,
            cached_funding_index: known.cached_funding_index,
            unrealized_funding_pnl: known.unrealized_funding_pnl,
            status: known.status,
            market: known.market,
            last_fill_id: known.last_fill_id,
            extra,
        })
    }
}

#[derive(Clone, serde::Serialize)]
pub struct AccountEvent {
    #[serde(rename = "ts")]
    pub ts: serde_json::Number,
    #[serde(rename = "seq")]
    pub seq: serde_json::Number,
    #[serde(rename = "totalCollateral")]
    pub total_collateral: String,
    #[serde(rename = "freeCollateral")]
    pub free_collateral: String,
    #[serde(rename = "initialMarginReq")]
    pub initial_margin_req: String,
    #[serde(rename = "maintenanceMarginReq")]
    pub maintenance_margin_req: String,
    #[serde(rename = "accountValue")]
    pub account_value: String,
    #[serde(rename = "unrealizedPnl")]
    pub unrealized_pnl: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: serde_json::Number,
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "marginCushion")]
    pub margin_cushion: String,
    #[serde(rename = "status")]
    pub status: Option<String>,
    #[serde(rename = "lastSeenNotification")]
    pub last_seen_notification: serde_json::Number,
    #[serde(rename = "settlementAsset")]
    pub settlement_asset: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
impl Default for AccountEvent {
    fn default() -> Self {
        Self {
            ts: serde_json::Number::from(0),
            seq: serde_json::Number::from(0),
            total_collateral: Default::default(),
            free_collateral: Default::default(),
            initial_margin_req: Default::default(),
            maintenance_margin_req: Default::default(),
            account_value: Default::default(),
            unrealized_pnl: Default::default(),
            updated_at: serde_json::Number::from(0),
            account: Default::default(),
            margin_cushion: Default::default(),
            status: Default::default(),
            last_seen_notification: serde_json::Number::from(0),
            settlement_asset: Default::default(),
            extra: Default::default(),
        }
    }
}
impl<'de> serde::Deserialize<'de> for AccountEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Known {
            #[serde(rename = "ts")]
            pub ts: serde_json::Number,
            #[serde(rename = "seq")]
            pub seq: serde_json::Number,
            #[serde(rename = "totalCollateral")]
            pub total_collateral: String,
            #[serde(rename = "freeCollateral")]
            pub free_collateral: String,
            #[serde(rename = "initialMarginReq")]
            pub initial_margin_req: String,
            #[serde(rename = "maintenanceMarginReq")]
            pub maintenance_margin_req: String,
            #[serde(rename = "accountValue")]
            pub account_value: String,
            #[serde(rename = "unrealizedPnl")]
            pub unrealized_pnl: Option<String>,
            #[serde(rename = "updatedAt")]
            pub updated_at: serde_json::Number,
            #[serde(rename = "account")]
            pub account: String,
            #[serde(rename = "marginCushion")]
            pub margin_cushion: String,
            #[serde(rename = "status")]
            pub status: Option<String>,
            #[serde(rename = "lastSeenNotification")]
            pub last_seen_notification: serde_json::Number,
            #[serde(rename = "settlementAsset")]
            pub settlement_asset: String,
        }
        let mut extra = serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;
        let mut fields = serde_json::Map::new();
        for name in [
            "ts",
            "seq",
            "totalCollateral",
            "freeCollateral",
            "initialMarginReq",
            "maintenanceMarginReq",
            "accountValue",
            "unrealizedPnl",
            "updatedAt",
            "account",
            "marginCushion",
            "status",
            "lastSeenNotification",
            "settlementAsset",
        ] {
            if let Some(value) = extra.remove(name) {
                fields.insert(name.to_owned(), value);
            }
        }
        let known: Known = serde_json::from_value(serde_json::Value::Object(fields))
            .map_err(serde::de::Error::custom)?;
        Ok(Self {
            ts: known.ts,
            seq: known.seq,
            total_collateral: known.total_collateral,
            free_collateral: known.free_collateral,
            initial_margin_req: known.initial_margin_req,
            maintenance_margin_req: known.maintenance_margin_req,
            account_value: known.account_value,
            unrealized_pnl: known.unrealized_pnl,
            updated_at: known.updated_at,
            account: known.account,
            margin_cushion: known.margin_cushion,
            status: known.status,
            last_seen_notification: known.last_seen_notification,
            settlement_asset: known.settlement_asset,
            extra,
        })
    }
}

#[derive(Clone, serde::Serialize)]
pub struct HeartbeatEvent {
    #[serde(rename = "ts")]
    pub ts: serde_json::Number,
    #[serde(rename = "seq")]
    pub seq: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
impl Default for HeartbeatEvent {
    fn default() -> Self {
        Self {
            ts: serde_json::Number::from(0),
            seq: serde_json::Number::from(0),
            extra: Default::default(),
        }
    }
}
impl<'de> serde::Deserialize<'de> for HeartbeatEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Known {
            #[serde(rename = "ts")]
            pub ts: serde_json::Number,
            #[serde(rename = "seq")]
            pub seq: serde_json::Number,
        }
        let mut extra = serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;
        let mut fields = serde_json::Map::new();
        for name in ["ts", "seq"] {
            if let Some(value) = extra.remove(name) {
                fields.insert(name.to_owned(), value);
            }
        }
        let known: Known = serde_json::from_value(serde_json::Value::Object(fields))
            .map_err(serde::de::Error::custom)?;
        Ok(Self {
            ts: known.ts,
            seq: known.seq,
            extra,
        })
    }
}

#[derive(Clone, serde::Serialize)]
pub struct SubscribedEvent {
    #[serde(rename = "ts")]
    pub ts: serde_json::Number,
    #[serde(rename = "seq")]
    pub seq: serde_json::Number,
    #[serde(rename = "status")]
    pub status: serde_json::Number,
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
impl Default for SubscribedEvent {
    fn default() -> Self {
        Self {
            ts: serde_json::Number::from(0),
            seq: serde_json::Number::from(0),
            status: serde_json::Number::from(0),
            channel: Default::default(),
            extra: Default::default(),
        }
    }
}
impl<'de> serde::Deserialize<'de> for SubscribedEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Known {
            #[serde(rename = "ts")]
            pub ts: serde_json::Number,
            #[serde(rename = "seq")]
            pub seq: serde_json::Number,
            #[serde(rename = "status")]
            pub status: serde_json::Number,
            #[serde(rename = "channel")]
            pub channel: String,
        }
        let mut extra = serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;
        let mut fields = serde_json::Map::new();
        for name in ["ts", "seq", "status", "channel"] {
            if let Some(value) = extra.remove(name) {
                fields.insert(name.to_owned(), value);
            }
        }
        let known: Known = serde_json::from_value(serde_json::Value::Object(fields))
            .map_err(serde::de::Error::custom)?;
        Ok(Self {
            ts: known.ts,
            seq: known.seq,
            status: known.status,
            channel: known.channel,
            extra,
        })
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "template", content = "data")]
pub enum SbeMessage {
    #[serde(rename = "1")]
    TradeEvent(TradeEvent),
    #[serde(rename = "2")]
    BboEvent(BboEvent),
    #[serde(rename = "3")]
    BookEvent(BookEvent),
    #[serde(rename = "4")]
    MarketSummaryEvent(MarketSummaryEvent),
    #[serde(rename = "5")]
    FundingDataEvent(FundingDataEvent),
    #[serde(rename = "6")]
    FundingRateComparisonEvent(FundingRateComparisonEvent),
    #[serde(rename = "20")]
    OrderEvent(OrderEvent),
    #[serde(rename = "21")]
    FillEvent(FillEvent),
    #[serde(rename = "22")]
    PositionEvent(PositionEvent),
    #[serde(rename = "23")]
    AccountEvent(AccountEvent),
    #[serde(rename = "40")]
    HeartbeatEvent(HeartbeatEvent),
    #[serde(rename = "41")]
    SubscribedEvent(SubscribedEvent),
}

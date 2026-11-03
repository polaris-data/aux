// Generated from reference/paradex_1_0.xml by scripts/generate-paradex-sbe.py.
use super::{Field, Kind, Layout};
pub const TRADE_EVENT: Layout = Layout {
    template: 1,
    minimum: 50,
    fields: &[
        Field {
            name: "ts",
            offset: 0,
            size: 8,
            kind: Kind::Timestamp,
        },
        Field {
            name: "seq",
            offset: 8,
            size: 8,
            kind: Kind::Integer {
                bytes: 8,
                signed: true,
            },
        },
        Field {
            name: "tradeId",
            offset: 16,
            size: 8,
            kind: Kind::Integer {
                bytes: 8,
                signed: true,
            },
        },
        Field {
            name: "side",
            offset: 24,
            size: 1,
            kind: Kind::Enum(&[(1, "BUY"), (2, "SELL"), (254, "NON_REPRESENTABLE")]),
        },
        Field {
            name: "price",
            offset: 25,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "size",
            offset: 33,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "createdAt",
            offset: 41,
            size: 8,
            kind: Kind::Timestamp,
        },
        Field {
            name: "tradeType",
            offset: 49,
            size: 1,
            kind: Kind::Enum(&[
                (1, "FILL"),
                (2, "LIQUIDATION"),
                (3, "UNWIND_TRANSFER"),
                (4, "SETTLE_MARKET"),
                (5, "RPI"),
                (6, "BLOCK_TRADE"),
                (254, "NON_REPRESENTABLE"),
            ]),
        },
    ],
    variable: &[("market", None), ("tradeIdStr", None)],
};
pub const BBO_EVENT: Layout = Layout {
    template: 2,
    minimum: 48,
    fields: &[
        Field {
            name: "ts",
            offset: 0,
            size: 8,
            kind: Kind::Timestamp,
        },
        Field {
            name: "seq",
            offset: 8,
            size: 8,
            kind: Kind::Integer {
                bytes: 8,
                signed: true,
            },
        },
        Field {
            name: "bidPrice",
            offset: 16,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "bidSize",
            offset: 24,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "askPrice",
            offset: 32,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "askSize",
            offset: 40,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
    ],
    variable: &[("market", None)],
};
pub const BOOK_EVENT: Layout = Layout {
    template: 3,
    minimum: 89,
    fields: &[
        Field {
            name: "ts",
            offset: 0,
            size: 8,
            kind: Kind::Timestamp,
        },
        Field {
            name: "seq",
            offset: 8,
            size: 8,
            kind: Kind::Integer {
                bytes: 8,
                signed: true,
            },
        },
        Field {
            name: "pkgType",
            offset: 16,
            size: 1,
            kind: Kind::Enum(&[(0, "SNAPSHOT"), (1, "DELTA")]),
        },
        Field {
            name: "bestBidPrice",
            offset: 17,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "bestBidSize",
            offset: 25,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "bestAskPrice",
            offset: 33,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "bestAskSize",
            offset: 41,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "relativeSpread",
            offset: 49,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "bestBidApiPrice",
            offset: 57,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "bestBidApiSize",
            offset: 65,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "bestAskApiPrice",
            offset: 73,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "bestAskApiSize",
            offset: 81,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
    ],
    variable: &[
        (
            "bids",
            Some((
                16,
                &[
                    Field {
                        name: "price",
                        offset: 0,
                        size: 8,
                        kind: Kind::Decimal {
                            places: 8,
                            nullable: false,
                        },
                    },
                    Field {
                        name: "size",
                        offset: 8,
                        size: 8,
                        kind: Kind::Decimal {
                            places: 8,
                            nullable: false,
                        },
                    },
                ],
            )),
        ),
        (
            "asks",
            Some((
                16,
                &[
                    Field {
                        name: "price",
                        offset: 0,
                        size: 8,
                        kind: Kind::Decimal {
                            places: 8,
                            nullable: false,
                        },
                    },
                    Field {
                        name: "size",
                        offset: 8,
                        size: 8,
                        kind: Kind::Decimal {
                            places: 8,
                            nullable: false,
                        },
                    },
                ],
            )),
        ),
        ("market", None),
    ],
};
pub const MARKET_SUMMARY_EVENT: Layout = Layout {
    template: 4,
    minimum: 248,
    fields: &[
        Field {
            name: "ts",
            offset: 0,
            size: 8,
            kind: Kind::Timestamp,
        },
        Field {
            name: "seq",
            offset: 8,
            size: 8,
            kind: Kind::Integer {
                bytes: 8,
                signed: true,
            },
        },
        Field {
            name: "markPrice",
            offset: 16,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "indexPrice",
            offset: 24,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "lastPrice",
            offset: 32,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "volumeUsd24h",
            offset: 40,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "openInterest",
            offset: 48,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "fundingRate",
            offset: 56,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "bid",
            offset: 64,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "ask",
            offset: 72,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "totalVolume",
            offset: 80,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "priceChangeRate24h",
            offset: 88,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "markIv",
            offset: 96,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "bidIv",
            offset: 104,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "askIv",
            offset: 112,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "lastIv",
            offset: 120,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "delta",
            offset: 128,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "gamma",
            offset: 136,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "vega",
            offset: 144,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "theta",
            offset: 152,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "rho",
            offset: 160,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "volga",
            offset: 168,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "vanna",
            offset: 176,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "futureFundingRate",
            offset: 184,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "externalFairPrice",
            offset: 192,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "bidSize",
            offset: 200,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "askSize",
            offset: 208,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "forwardRate",
            offset: 216,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "riskFreeRate",
            offset: 224,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "fundingRatePrecise",
            offset: 232,
            size: 8,
            kind: Kind::Decimal {
                places: 12,
                nullable: true,
            },
        },
        Field {
            name: "settlementTwapEstimate",
            offset: 240,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
    ],
    variable: &[("market", None)],
};
pub const FUNDING_DATA_EVENT: Layout = Layout {
    template: 5,
    minimum: 66,
    fields: &[
        Field {
            name: "ts",
            offset: 0,
            size: 8,
            kind: Kind::Timestamp,
        },
        Field {
            name: "seq",
            offset: 8,
            size: 8,
            kind: Kind::Integer {
                bytes: 8,
                signed: true,
            },
        },
        Field {
            name: "fundingRate",
            offset: 16,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "fundingIndex",
            offset: 24,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "createdAt",
            offset: 32,
            size: 8,
            kind: Kind::Timestamp,
        },
        Field {
            name: "fundingPremium",
            offset: 40,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "fundingRate8h",
            offset: 48,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "fundingPeriodHours",
            offset: 56,
            size: 2,
            kind: Kind::Integer {
                bytes: 2,
                signed: true,
            },
        },
        Field {
            name: "impactPremiumRate",
            offset: 58,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
    ],
    variable: &[("market", None)],
};
pub const FUNDING_RATE_COMPARISON_EVENT: Layout = Layout {
    template: 6,
    minimum: 26,
    fields: &[
        Field {
            name: "ts",
            offset: 0,
            size: 8,
            kind: Kind::Timestamp,
        },
        Field {
            name: "lastUpdatedAt",
            offset: 8,
            size: 8,
            kind: Kind::Timestamp,
        },
        Field {
            name: "hourlyFundingRate",
            offset: 16,
            size: 8,
            kind: Kind::Decimal {
                places: 12,
                nullable: false,
            },
        },
        Field {
            name: "assetKind",
            offset: 24,
            size: 1,
            kind: Kind::Enum(&[
                (0, "UNSPECIFIED"),
                (1, "PERPETUAL_FUTURE"),
                (2, "OPTION"),
                (3, "SPOT"),
            ]),
        },
        Field {
            name: "source",
            offset: 25,
            size: 1,
            kind: Kind::Enum(&[
                (0, "UNSPECIFIED"),
                (1, "PARADEX"),
                (2, "BINANCE"),
                (3, "BYBIT"),
                (4, "OKX"),
                (5, "DERIBIT"),
                (6, "DERIVE"),
                (7, "HYPERLIQUID"),
                (8, "LIGHTER"),
                (9, "DATABENTO"),
                (10, "PUBLISHER"),
                (11, "PYTH"),
                (12, "STORK"),
            ]),
        },
    ],
    variable: &[("market", None)],
};
pub const ORDER_EVENT: Layout = Layout {
    template: 20,
    minimum: 128,
    fields: &[
        Field {
            name: "ts",
            offset: 0,
            size: 8,
            kind: Kind::Timestamp,
        },
        Field {
            name: "seq",
            offset: 8,
            size: 8,
            kind: Kind::Integer {
                bytes: 8,
                signed: true,
            },
        },
        Field {
            name: "status",
            offset: 16,
            size: 1,
            kind: Kind::Enum(&[
                (1, "NEW"),
                (2, "UNTRIGGERED"),
                (3, "OPEN"),
                (4, "CLOSED"),
                (254, "NON_REPRESENTABLE"),
            ]),
        },
        Field {
            name: "side",
            offset: 17,
            size: 1,
            kind: Kind::Enum(&[(1, "BUY"), (2, "SELL"), (254, "NON_REPRESENTABLE")]),
        },
        Field {
            name: "orderType",
            offset: 18,
            size: 1,
            kind: Kind::Enum(&[
                (1, "LIMIT"),
                (2, "MARKET"),
                (3, "STOP_LIMIT"),
                (4, "STOP_MARKET"),
                (5, "TAKE_PROFIT_LIMIT"),
                (6, "TAKE_PROFIT_MARKET"),
                (7, "STOP_LOSS_LIMIT"),
                (8, "STOP_LOSS_MARKET"),
                (254, "NON_REPRESENTABLE"),
            ]),
        },
        Field {
            name: "timeInForce",
            offset: 19,
            size: 1,
            kind: Kind::Enum(&[
                (1, "GTC"),
                (2, "IOC"),
                (3, "POST_ONLY"),
                (4, "RPI"),
                (254, "NON_REPRESENTABLE"),
            ]),
        },
        Field {
            name: "price",
            offset: 20,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "triggerPrice",
            offset: 28,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "size",
            offset: 36,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "sizeOpen",
            offset: 44,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "avgFillPrice",
            offset: 52,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "createdAt",
            offset: 60,
            size: 8,
            kind: Kind::Timestamp,
        },
        Field {
            name: "updatedAt",
            offset: 68,
            size: 8,
            kind: Kind::Timestamp,
        },
        Field {
            name: "account",
            offset: 76,
            size: 32,
            kind: Kind::Address,
        },
        Field {
            name: "receivedAt",
            offset: 108,
            size: 8,
            kind: Kind::Timestamp,
        },
        Field {
            name: "publishedAt",
            offset: 116,
            size: 8,
            kind: Kind::Timestamp,
        },
        Field {
            name: "stp",
            offset: 124,
            size: 1,
            kind: Kind::Enum(&[
                (0, "NONE"),
                (1, "EXPIRE_MAKER"),
                (2, "EXPIRE_TAKER"),
                (3, "EXPIRE_BOTH"),
                (254, "NON_REPRESENTABLE"),
            ]),
        },
        Field {
            name: "flags",
            offset: 125,
            size: 1,
            kind: Kind::Flags(&[
                (0, "REDUCE_ONLY"),
                (1, "STOP_CONDITION_BELOW_TRIGGER"),
                (2, "STOP_CONDITION_ABOVE_TRIGGER"),
                (3, "INTERACTIVE"),
                (4, "TARGET_STRATEGY_VWAP"),
                (5, "MMP"),
            ]),
        },
        Field {
            name: "requestStatus",
            offset: 126,
            size: 1,
            kind: Kind::Enum(&[
                (0, "UNSPECIFIED"),
                (1, "PENDING"),
                (2, "PROCESSED"),
                (3, "REJECTED"),
                (4, "SUCCESS"),
                (254, "NON_REPRESENTABLE"),
            ]),
        },
        Field {
            name: "requestType",
            offset: 127,
            size: 1,
            kind: Kind::Enum(&[
                (0, "UNSPECIFIED"),
                (1, "MODIFY_ORDER"),
                (254, "NON_REPRESENTABLE"),
            ]),
        },
    ],
    variable: &[
        ("orderId", None),
        ("clientOrderId", None),
        ("market", None),
        ("cancelReason", None),
        ("requestId", None),
        ("requestMessage", None),
    ],
};
pub const FILL_EVENT: Layout = Layout {
    template: 21,
    minimum: 116,
    fields: &[
        Field {
            name: "ts",
            offset: 0,
            size: 8,
            kind: Kind::Timestamp,
        },
        Field {
            name: "seq",
            offset: 8,
            size: 8,
            kind: Kind::OptionalI64,
        },
        Field {
            name: "fillType",
            offset: 16,
            size: 1,
            kind: Kind::Enum(&[
                (1, "FILL"),
                (2, "LIQUIDATION"),
                (3, "UNWIND_TRANSFER"),
                (4, "SETTLE_MARKET"),
                (5, "RPI"),
                (6, "BLOCK_TRADE"),
                (254, "NON_REPRESENTABLE"),
            ]),
        },
        Field {
            name: "side",
            offset: 17,
            size: 1,
            kind: Kind::Enum(&[(1, "BUY"), (2, "SELL"), (254, "NON_REPRESENTABLE")]),
        },
        Field {
            name: "liquidity",
            offset: 18,
            size: 1,
            kind: Kind::Enum(&[(1, "MAKER"), (2, "TAKER"), (254, "NON_REPRESENTABLE")]),
        },
        Field {
            name: "price",
            offset: 19,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "size",
            offset: 27,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "fee",
            offset: 35,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "realizedPnl",
            offset: 43,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "createdAt",
            offset: 51,
            size: 8,
            kind: Kind::Timestamp,
        },
        Field {
            name: "account",
            offset: 59,
            size: 32,
            kind: Kind::Address,
        },
        Field {
            name: "underlyingPrice",
            offset: 91,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "realizedFunding",
            offset: 99,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "flags",
            offset: 107,
            size: 1,
            kind: Kind::Flags(&[
                (0, "INTERACTIVE"),
                (1, "RPI"),
                (2, "FASTFILL"),
                (7, "UNKNOWN"),
            ]),
        },
        Field {
            name: "orderbookSeqNo",
            offset: 108,
            size: 8,
            kind: Kind::Integer {
                bytes: 8,
                signed: false,
            },
        },
    ],
    variable: &[
        ("fillId", None),
        ("orderId", None),
        ("clientOrderId", None),
        ("tradeId", None),
        ("market", None),
        ("feeCurrency", None),
    ],
};
pub const POSITION_EVENT: Layout = Layout {
    template: 22,
    minimum: 154,
    fields: &[
        Field {
            name: "ts",
            offset: 0,
            size: 8,
            kind: Kind::Timestamp,
        },
        Field {
            name: "seq",
            offset: 8,
            size: 8,
            kind: Kind::Integer {
                bytes: 8,
                signed: true,
            },
        },
        Field {
            name: "side",
            offset: 16,
            size: 1,
            kind: Kind::Enum(&[(1, "BUY"), (2, "SELL"), (254, "NON_REPRESENTABLE")]),
        },
        Field {
            name: "size",
            offset: 17,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "avgEntryPrice",
            offset: 25,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "unrealizedPnl",
            offset: 33,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "realizedPnl",
            offset: 41,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "markPrice",
            offset: 49,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "liquidationPrice",
            offset: 57,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "leverage",
            offset: 65,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "updatedAt",
            offset: 73,
            size: 8,
            kind: Kind::Timestamp,
        },
        Field {
            name: "account",
            offset: 81,
            size: 32,
            kind: Kind::Address,
        },
        Field {
            name: "avgEntryPriceUsd",
            offset: 113,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "cost",
            offset: 121,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "costUsd",
            offset: 129,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "cachedFundingIndex",
            offset: 137,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "unrealizedFundingPnl",
            offset: 145,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "status",
            offset: 153,
            size: 1,
            kind: Kind::Enum(&[
                (1, "OPEN"),
                (2, "CLOSED"),
                (3, "UNSPECIFIED"),
                (254, "NON_REPRESENTABLE"),
            ]),
        },
    ],
    variable: &[("market", None), ("lastFillId", None)],
};
pub const ACCOUNT_EVENT: Layout = Layout {
    template: 23,
    minimum: 121,
    fields: &[
        Field {
            name: "ts",
            offset: 0,
            size: 8,
            kind: Kind::Timestamp,
        },
        Field {
            name: "seq",
            offset: 8,
            size: 8,
            kind: Kind::Integer {
                bytes: 8,
                signed: true,
            },
        },
        Field {
            name: "totalCollateral",
            offset: 16,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "freeCollateral",
            offset: 24,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "initialMarginReq",
            offset: 32,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "maintenanceMarginReq",
            offset: 40,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "accountValue",
            offset: 48,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "unrealizedPnl",
            offset: 56,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: true,
            },
        },
        Field {
            name: "updatedAt",
            offset: 64,
            size: 8,
            kind: Kind::Timestamp,
        },
        Field {
            name: "account",
            offset: 72,
            size: 32,
            kind: Kind::Address,
        },
        Field {
            name: "marginCushion",
            offset: 104,
            size: 8,
            kind: Kind::Decimal {
                places: 8,
                nullable: false,
            },
        },
        Field {
            name: "status",
            offset: 112,
            size: 1,
            kind: Kind::Enum(&[
                (1, "ACTIVE"),
                (2, "LIQUIDATION"),
                (254, "NON_REPRESENTABLE"),
            ]),
        },
        Field {
            name: "lastSeenNotification",
            offset: 113,
            size: 8,
            kind: Kind::Timestamp,
        },
    ],
    variable: &[("settlementAsset", None)],
};
pub const HEARTBEAT_EVENT: Layout = Layout {
    template: 40,
    minimum: 16,
    fields: &[
        Field {
            name: "ts",
            offset: 0,
            size: 8,
            kind: Kind::Timestamp,
        },
        Field {
            name: "seq",
            offset: 8,
            size: 8,
            kind: Kind::Integer {
                bytes: 8,
                signed: true,
            },
        },
    ],
    variable: &[],
};
pub const SUBSCRIBED_EVENT: Layout = Layout {
    template: 41,
    minimum: 17,
    fields: &[
        Field {
            name: "ts",
            offset: 0,
            size: 8,
            kind: Kind::Timestamp,
        },
        Field {
            name: "seq",
            offset: 8,
            size: 8,
            kind: Kind::Integer {
                bytes: 8,
                signed: true,
            },
        },
        Field {
            name: "status",
            offset: 16,
            size: 1,
            kind: Kind::Integer {
                bytes: 1,
                signed: false,
            },
        },
    ],
    variable: &[("channel", None)],
};
pub fn layout(template: u16) -> Option<&'static Layout> {
    Some(match template {
        1 => &TRADE_EVENT,
        2 => &BBO_EVENT,
        3 => &BOOK_EVENT,
        4 => &MARKET_SUMMARY_EVENT,
        5 => &FUNDING_DATA_EVENT,
        6 => &FUNDING_RATE_COMPARISON_EVENT,
        20 => &ORDER_EVENT,
        21 => &FILL_EVENT,
        22 => &POSITION_EVENT,
        23 => &ACCOUNT_EVENT,
        40 => &HEARTBEAT_EVENT,
        41 => &SUBSCRIBED_EVENT,
        _ => return None,
    })
}

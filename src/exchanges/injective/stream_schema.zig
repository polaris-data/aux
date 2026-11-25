// Field numbers from InjectiveLabs/injective-proto commit 5ad1071de2529b6699efb9996563d589d51f00cf.
pub const Field = struct { number: u29, name: []const u8, kind: enum { text, sint, uint, boolean, message }, fields: []const Field = &.{}, repeated: bool = false };
pub const StreamOrderbookV2Response = [_]Field{
    .{ .number = 1, .name = "orderbook", .kind = .message, .fields = &DerivativeLimitOrderbookV2 },
    .{ .number = 2, .name = "operation_type", .kind = .text },
    .{ .number = 3, .name = "timestamp", .kind = .sint },
    .{ .number = 4, .name = "market_id", .kind = .text },
};

pub const DerivativeLimitOrderbookV2 = [_]Field{
    .{ .number = 1, .name = "buys", .kind = .message, .fields = &PriceLevel, .repeated = true },
    .{ .number = 2, .name = "sells", .kind = .message, .fields = &PriceLevel, .repeated = true },
    .{ .number = 3, .name = "sequence", .kind = .uint },
    .{ .number = 4, .name = "timestamp", .kind = .sint },
    .{ .number = 5, .name = "height", .kind = .sint },
};

pub const PriceLevel = [_]Field{
    .{ .number = 1, .name = "price", .kind = .text },
    .{ .number = 2, .name = "quantity", .kind = .text },
    .{ .number = 3, .name = "timestamp", .kind = .sint },
};

pub const StreamTradesV2Response = [_]Field{
    .{ .number = 1, .name = "trade", .kind = .message, .fields = &DerivativeTrade },
    .{ .number = 2, .name = "operation_type", .kind = .text },
    .{ .number = 3, .name = "timestamp", .kind = .sint },
};

pub const DerivativeTrade = [_]Field{
    .{ .number = 1, .name = "order_hash", .kind = .text },
    .{ .number = 2, .name = "subaccount_id", .kind = .text },
    .{ .number = 3, .name = "market_id", .kind = .text },
    .{ .number = 4, .name = "trade_execution_type", .kind = .text },
    .{ .number = 5, .name = "is_liquidation", .kind = .boolean },
    .{ .number = 6, .name = "position_delta", .kind = .message, .fields = &PositionDelta },
    .{ .number = 7, .name = "payout", .kind = .text },
    .{ .number = 8, .name = "fee", .kind = .text },
    .{ .number = 9, .name = "executed_at", .kind = .sint },
    .{ .number = 10, .name = "fee_recipient", .kind = .text },
    .{ .number = 11, .name = "trade_id", .kind = .text },
    .{ .number = 12, .name = "execution_side", .kind = .text },
    .{ .number = 13, .name = "cid", .kind = .text },
    .{ .number = 14, .name = "pnl", .kind = .text },
};

pub const PositionDelta = [_]Field{
    .{ .number = 1, .name = "trade_direction", .kind = .text },
    .{ .number = 2, .name = "execution_price", .kind = .text },
    .{ .number = 3, .name = "execution_quantity", .kind = .text },
    .{ .number = 4, .name = "execution_margin", .kind = .text },
};

pub const StreamOrdersResponse = [_]Field{
    .{ .number = 1, .name = "order", .kind = .message, .fields = &DerivativeLimitOrder },
    .{ .number = 2, .name = "operation_type", .kind = .text },
    .{ .number = 3, .name = "timestamp", .kind = .sint },
};

pub const DerivativeLimitOrder = [_]Field{
    .{ .number = 1, .name = "order_hash", .kind = .text },
    .{ .number = 2, .name = "order_side", .kind = .text },
    .{ .number = 3, .name = "market_id", .kind = .text },
    .{ .number = 4, .name = "subaccount_id", .kind = .text },
    .{ .number = 5, .name = "is_reduce_only", .kind = .boolean },
    .{ .number = 6, .name = "margin", .kind = .text },
    .{ .number = 7, .name = "price", .kind = .text },
    .{ .number = 8, .name = "quantity", .kind = .text },
    .{ .number = 9, .name = "unfilled_quantity", .kind = .text },
    .{ .number = 10, .name = "trigger_price", .kind = .text },
    .{ .number = 11, .name = "fee_recipient", .kind = .text },
    .{ .number = 12, .name = "state", .kind = .text },
    .{ .number = 13, .name = "created_at", .kind = .sint },
    .{ .number = 14, .name = "updated_at", .kind = .sint },
    .{ .number = 15, .name = "order_number", .kind = .sint },
    .{ .number = 16, .name = "order_type", .kind = .text },
    .{ .number = 17, .name = "is_conditional", .kind = .boolean },
    .{ .number = 18, .name = "trigger_at", .kind = .uint },
    .{ .number = 19, .name = "placed_order_hash", .kind = .text },
    .{ .number = 20, .name = "execution_type", .kind = .text },
    .{ .number = 21, .name = "tx_hash", .kind = .text },
    .{ .number = 22, .name = "cid", .kind = .text },
    .{ .number = 23, .name = "account_address", .kind = .text },
};

pub const StreamPositionsV2Response = [_]Field{
    .{ .number = 1, .name = "position", .kind = .message, .fields = &DerivativePositionV2 },
    .{ .number = 2, .name = "timestamp", .kind = .sint },
    .{ .number = 3, .name = "operation_type", .kind = .text },
};

pub const DerivativePositionV2 = [_]Field{
    .{ .number = 1, .name = "ticker", .kind = .text },
    .{ .number = 2, .name = "market_id", .kind = .text },
    .{ .number = 3, .name = "subaccount_id", .kind = .text },
    .{ .number = 4, .name = "direction", .kind = .text },
    .{ .number = 5, .name = "quantity", .kind = .text },
    .{ .number = 6, .name = "entry_price", .kind = .text },
    .{ .number = 7, .name = "margin", .kind = .text },
    .{ .number = 8, .name = "liquidation_price", .kind = .text },
    .{ .number = 9, .name = "mark_price", .kind = .text },
    .{ .number = 11, .name = "updated_at", .kind = .sint },
    .{ .number = 12, .name = "denom", .kind = .text },
    .{ .number = 13, .name = "funding_last", .kind = .text },
    .{ .number = 14, .name = "funding_sum", .kind = .text },
    .{ .number = 15, .name = "cumulative_funding_entry", .kind = .text },
    .{ .number = 16, .name = "effective_cumulative_funding_entry", .kind = .text },
    .{ .number = 17, .name = "upnl", .kind = .text },
};

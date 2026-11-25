const r = @import("../../core/routes.zig");

pub const operations = [_]r.Operation{
    .{ .venue = .perpl, .name = "markets", .verb = "GET", .path = "/api/v1/pub/context", .private = false, .origin = "https://app.perpl.xyz", .params = &.{} },
    .{ .venue = .perpl, .name = "order_book", .verb = "GET", .path = "/api/v1/market-data/{market_id}/book", .private = false, .origin = "https://app.perpl.xyz", .params = &.{ .{ .key = "market_id", .loc = .path, .required = true, .collection = "" }, .{ .key = "levels", .loc = .query, .required = false, .collection = "" } } },
    .{ .venue = .perpl, .name = "account", .verb = "GET", .path = "/api/v1/trading/wallet", .private = true, .origin = "https://app.perpl.xyz", .params = &.{} },
    .{ .venue = .perpl, .name = "open_orders", .verb = "GET", .path = "/api/v1/trading/orders", .private = true, .origin = "https://app.perpl.xyz", .params = &.{} },
    .{ .venue = .perpl, .name = "positions", .verb = "GET", .path = "/api/v1/trading/positions", .private = true, .origin = "https://app.perpl.xyz", .params = &.{} },
    .{ .venue = .perpl, .name = "fills", .verb = "GET", .path = "/api/v1/trading/fills", .private = true, .origin = "https://app.perpl.xyz", .params = &.{ .{ .key = "count", .loc = .query, .required = false, .collection = "" }, .{ .key = "page", .loc = .query, .required = false, .collection = "" } } },
    .{ .venue = .perpl, .name = "order_history", .verb = "GET", .path = "/api/v1/trading/order-history", .private = true, .origin = "https://app.perpl.xyz", .params = &.{ .{ .key = "count", .loc = .query, .required = false, .collection = "" }, .{ .key = "page", .loc = .query, .required = false, .collection = "" } } },
    .{ .venue = .perpl, .name = "place_order", .verb = "POST", .path = "/api/v1/trading/orders", .private = true, .origin = "https://app.perpl.xyz", .params = &.{.{ .key = "body", .loc = .body, .required = true, .collection = "" }} },
    .{ .venue = .perpl, .name = "cancel_order", .verb = "POST", .path = "/api/v1/trading/orders", .private = true, .origin = "https://app.perpl.xyz", .params = &.{.{ .key = "body", .loc = .body, .required = true, .collection = "" }} },
};

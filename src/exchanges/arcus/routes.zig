const Operation = @import("../../core/routes.zig").Operation;

// zig fmt: off
pub const operations = [_]Operation{
    .{ .venue = .arcus, .name = "place_order", .verb = "POST", .path = "/v1/placeOrder", .private = true, .origin = "https://api.arcus.xyz", .params = &.{} },
    .{ .venue = .arcus, .name = "cancel_order", .verb = "POST", .path = "/v1/cancelOrder", .private = true, .origin = "https://api.arcus.xyz", .params = &.{} },
    .{ .venue = .arcus, .name = "markets", .verb = "GET", .path = "/v1/markets", .private = false, .origin = "https://api.arcus.xyz", .params = &.{.{ .key = "market", .loc = .query, .required = false, .collection = "multi", .header = "" }} },
    .{ .venue = .arcus, .name = "order_book", .verb = "GET", .path = "/v1/l2OrderBook/{market}", .private = false, .origin = "https://api.arcus.xyz", .params = &.{.{ .key = "market", .loc = .path, .required = true, .collection = "multi", .header = "" }, .{ .key = "nLevels", .loc = .query, .required = true, .collection = "multi", .header = "" }} },
    .{ .venue = .arcus, .name = "account", .verb = "GET", .path = "/v1/account", .private = false, .origin = "https://api.arcus.xyz", .params = &.{.{ .key = "address", .loc = .query, .required = true, .collection = "multi", .header = "" }, .{ .key = "accountIndex", .loc = .query, .required = true, .collection = "multi", .header = "" }} },
    .{ .venue = .arcus, .name = "trades", .verb = "GET", .path = "/v1/trades", .private = false, .origin = "https://api.arcus.xyz", .params = &.{.{ .key = "market", .loc = .query, .required = true, .collection = "multi", .header = "" }, .{ .key = "limit", .loc = .query, .required = true, .collection = "multi", .header = "" }} },
    .{ .venue = .arcus, .name = "positions", .verb = "GET", .path = "/v1/positions", .private = false, .origin = "https://api.arcus.xyz", .params = &.{.{ .key = "address", .loc = .query, .required = true, .collection = "multi", .header = "" }, .{ .key = "accountIndex", .loc = .query, .required = true, .collection = "multi", .header = "" }} },
    .{ .venue = .arcus, .name = "open_orders", .verb = "GET", .path = "/v1/openOrders", .private = false, .origin = "https://api.arcus.xyz", .params = &.{.{ .key = "address", .loc = .query, .required = true, .collection = "multi", .header = "" }, .{ .key = "accountIndex", .loc = .query, .required = true, .collection = "multi", .header = "" }} },
};

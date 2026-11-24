const Operation = @import("../../core/routes.zig").Operation;

// zig fmt: off
pub const operations = [_]Operation{
    .{ .venue = .lighter, .name = "active_orders", .verb = "GET", .path = "/api/v1/accountActiveOrders", .private = true, .origin = "https://mainnet.zklighter.elliot.ai", .params = &.{.{ .key = "account_index", .loc = .query, .required = true, .collection = "multi" }, .{ .key = "market_id", .loc = .query, .required = false, .collection = "multi" }} },
    .{ .venue = .lighter, .name = "next_nonce", .verb = "GET", .path = "/api/v1/nextNonce", .private = false, .origin = "https://mainnet.zklighter.elliot.ai", .params = &.{.{ .key = "account_index", .loc = .query, .required = true, .collection = "multi" }, .{ .key = "api_key_index", .loc = .query, .required = true, .collection = "multi" }} },
    .{ .venue = .lighter, .name = "order_books", .verb = "GET", .path = "/api/v1/orderBooks", .private = false, .origin = "https://mainnet.zklighter.elliot.ai", .params = &.{.{ .key = "market_id", .loc = .query, .required = false, .collection = "multi", .header = "" }} },
    .{ .venue = .lighter, .name = "order_book_orders", .verb = "GET", .path = "/api/v1/orderBookOrders", .private = false, .origin = "https://mainnet.zklighter.elliot.ai", .params = &.{.{ .key = "market_id", .loc = .query, .required = true, .collection = "multi", .header = "" }, .{ .key = "limit", .loc = .query, .required = true, .collection = "multi", .header = "" }} },
    .{ .venue = .lighter, .name = "recent_trades", .verb = "GET", .path = "/api/v1/recentTrades", .private = false, .origin = "https://mainnet.zklighter.elliot.ai", .params = &.{.{ .key = "market_id", .loc = .query, .required = true, .collection = "multi", .header = "" }, .{ .key = "limit", .loc = .query, .required = true, .collection = "multi", .header = "" }} },
    .{ .venue = .lighter, .name = "account", .verb = "GET", .path = "/api/v1/account", .private = false, .origin = "https://mainnet.zklighter.elliot.ai", .params = &.{.{ .key = "by", .loc = .query, .required = true, .collection = "multi", .header = "" }, .{ .key = "value", .loc = .query, .required = true, .collection = "multi", .header = "" }} },
};

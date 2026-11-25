const r = @import("../../core/routes.zig");

pub const operations = [_]r.Operation{
    .{ .venue = .pacifica, .name = "markets", .verb = "GET", .path = "/api/v1/info", .private = false, .origin = "https://api.pacifica.fi", .params = &.{} },
    .{ .venue = .pacifica, .name = "order_book", .verb = "GET", .path = "/api/v1/book", .private = false, .origin = "https://api.pacifica.fi", .params = &.{ .{ .key = "symbol", .loc = .query, .required = true, .collection = "" }, .{ .key = "agg_level", .loc = .query, .required = false, .collection = "" } } },
    .{ .venue = .pacifica, .name = "trades", .verb = "GET", .path = "/api/v1/trades", .private = false, .origin = "https://api.pacifica.fi", .params = &.{.{ .key = "symbol", .loc = .query, .required = true, .collection = "" }} },
    .{ .venue = .pacifica, .name = "account", .verb = "GET", .path = "/api/v1/account", .private = false, .origin = "https://api.pacifica.fi", .params = &.{.{ .key = "account", .loc = .query, .required = true, .collection = "" }} },
    .{ .venue = .pacifica, .name = "positions", .verb = "GET", .path = "/api/v1/positions", .private = false, .origin = "https://api.pacifica.fi", .params = &.{.{ .key = "account", .loc = .query, .required = true, .collection = "" }} },
    .{ .venue = .pacifica, .name = "open_orders", .verb = "GET", .path = "/api/v1/orders", .private = false, .origin = "https://api.pacifica.fi", .params = &.{.{ .key = "account", .loc = .query, .required = true, .collection = "" }} },
    .{ .venue = .pacifica, .name = "order_status", .verb = "GET", .path = "/api/v1/orders/history_by_id", .private = false, .origin = "https://api.pacifica.fi", .params = &.{.{ .key = "order_id", .loc = .query, .required = true, .collection = "" }} },
    .{ .venue = .pacifica, .name = "place_order", .verb = "POST", .path = "/api/v1/orders/create", .private = true, .origin = "https://api.pacifica.fi", .params = &.{.{ .key = "body", .loc = .body, .required = true, .collection = "" }} },
    .{ .venue = .pacifica, .name = "place_market_order", .verb = "POST", .path = "/api/v1/orders/create_market", .private = true, .origin = "https://api.pacifica.fi", .params = &.{.{ .key = "body", .loc = .body, .required = true, .collection = "" }} },
    .{ .venue = .pacifica, .name = "cancel_order", .verb = "POST", .path = "/api/v1/orders/cancel", .private = true, .origin = "https://api.pacifica.fi", .params = &.{.{ .key = "body", .loc = .body, .required = true, .collection = "" }} },
};

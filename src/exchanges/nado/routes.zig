const r = @import("../../core/routes.zig");

pub const operations = [_]r.Operation{
    .{ .venue = .nado, .name = "markets", .verb = "GET", .path = "/archive/v2/contracts", .private = false, .origin = "https://api.prod.nado.xyz", .params = &.{} },
    .{ .venue = .nado, .name = "order_book", .verb = "GET", .path = "/gateway/v1/query", .private = false, .origin = "https://api.prod.nado.xyz", .params = &.{ .{ .key = "product_id", .loc = .query, .required = true, .collection = "" }, .{ .key = "depth", .loc = .query, .required = true, .collection = "" } } },
    .{ .venue = .nado, .name = "trades", .verb = "GET", .path = "/archive/v2/trades", .private = false, .origin = "https://api.prod.nado.xyz", .params = &.{ .{ .key = "ticker_id", .loc = .query, .required = true, .collection = "" }, .{ .key = "limit", .loc = .query, .required = false, .collection = "" } } },
    .{ .venue = .nado, .name = "account", .verb = "GET", .path = "/gateway/v1/query", .private = false, .origin = "https://api.prod.nado.xyz", .params = &.{.{ .key = "subaccount", .loc = .query, .required = true, .collection = "" }} },
    .{ .venue = .nado, .name = "positions", .verb = "GET", .path = "/gateway/v1/query", .private = false, .origin = "https://api.prod.nado.xyz", .params = &.{.{ .key = "subaccount", .loc = .query, .required = true, .collection = "" }} },
    .{ .venue = .nado, .name = "open_orders", .verb = "GET", .path = "/gateway/v1/query", .private = false, .origin = "https://api.prod.nado.xyz", .params = &.{ .{ .key = "sender", .loc = .query, .required = true, .collection = "" }, .{ .key = "product_id", .loc = .query, .required = true, .collection = "" } } },
    .{ .venue = .nado, .name = "contracts", .verb = "GET", .path = "/gateway/v1/query", .private = false, .origin = "https://api.prod.nado.xyz", .params = &.{} },
    .{ .venue = .nado, .name = "order_status", .verb = "POST", .path = "/archive/v1", .private = false, .origin = "https://api.prod.nado.xyz", .params = &.{.{ .key = "body", .loc = .body, .required = true, .collection = "" }} },
    .{ .venue = .nado, .name = "place_order", .verb = "POST", .path = "/gateway/v1/execute", .private = true, .origin = "https://api.prod.nado.xyz", .params = &.{.{ .key = "body", .loc = .body, .required = true, .collection = "" }} },
    .{ .venue = .nado, .name = "cancel_order", .verb = "POST", .path = "/gateway/v1/execute", .private = true, .origin = "https://api.prod.nado.xyz", .params = &.{.{ .key = "body", .loc = .body, .required = true, .collection = "" }} },
};

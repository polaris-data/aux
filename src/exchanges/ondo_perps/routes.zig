const r = @import("../../core/routes.zig");

pub const operations = [_]r.Operation{
    .{ .venue = .ondo_perps, .name = "markets", .verb = "GET", .path = "/v1/perps/contracts", .private = false, .origin = "https://api.ondoperps.xyz", .params = &.{.{ .key = "sparkline", .loc = .query, .required = false, .collection = "" }} },
    .{ .venue = .ondo_perps, .name = "order_book", .verb = "GET", .path = "/v1/perps/depth", .private = false, .origin = "https://api.ondoperps.xyz", .params = &.{ .{ .key = "market", .loc = .query, .required = true, .collection = "" }, .{ .key = "depth", .loc = .query, .required = false, .collection = "" } } },
    .{ .venue = .ondo_perps, .name = "trades", .verb = "GET", .path = "/v1/perps/trades", .private = false, .origin = "https://api.ondoperps.xyz", .params = &.{ .{ .key = "market", .loc = .query, .required = true, .collection = "" }, .{ .key = "limit", .loc = .query, .required = false, .collection = "" }, .{ .key = "cursor", .loc = .query, .required = false, .collection = "" } } },
    .{ .venue = .ondo_perps, .name = "account", .verb = "GET", .path = "/v1/perps/balance", .private = true, .origin = "https://api.ondoperps.xyz", .params = &.{} },
    .{ .venue = .ondo_perps, .name = "positions", .verb = "GET", .path = "/v1/perps/positions", .private = true, .origin = "https://api.ondoperps.xyz", .params = &.{} },
    .{ .venue = .ondo_perps, .name = "open_orders", .verb = "GET", .path = "/v1/perps/orders", .private = true, .origin = "https://api.ondoperps.xyz", .params = &.{ .{ .key = "market", .loc = .query, .required = false, .collection = "" }, .{ .key = "limit", .loc = .query, .required = false, .collection = "" }, .{ .key = "cursor", .loc = .query, .required = false, .collection = "" } } },
    .{ .venue = .ondo_perps, .name = "order_status", .verb = "GET", .path = "/v1/perps/orders/{orderID}", .private = true, .origin = "https://api.ondoperps.xyz", .params = &.{.{ .key = "orderID", .loc = .path, .required = true, .collection = "" }} },
    .{ .venue = .ondo_perps, .name = "place_order", .verb = "POST", .path = "/v1/perps/orders", .private = true, .origin = "https://api.ondoperps.xyz", .params = &.{.{ .key = "body", .loc = .body, .required = true, .collection = "" }} },
    .{ .venue = .ondo_perps, .name = "cancel_order", .verb = "DELETE", .path = "/v1/perps/orders/{orderID}", .private = true, .origin = "https://api.ondoperps.xyz", .params = &.{.{ .key = "orderID", .loc = .path, .required = true, .collection = "" }} },
};

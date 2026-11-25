const r = @import("../../core/routes.zig");

pub const operations = [_]r.Operation{
    .{ .venue = .aster, .name = "markets", .verb = "GET", .path = "/fapi/v3/exchangeInfo", .private = false, .origin = "https://fapi.asterdex.com", .params = &.{} },
    .{ .venue = .aster, .name = "order_book", .verb = "GET", .path = "/fapi/v3/depth", .private = false, .origin = "https://fapi.asterdex.com", .params = &.{ .{ .key = "symbol", .loc = .query, .required = true, .collection = "" }, .{ .key = "limit", .loc = .query, .required = false, .collection = "" } } },
    .{ .venue = .aster, .name = "trades", .verb = "GET", .path = "/fapi/v3/trades", .private = false, .origin = "https://fapi.asterdex.com", .params = &.{ .{ .key = "symbol", .loc = .query, .required = true, .collection = "" }, .{ .key = "limit", .loc = .query, .required = false, .collection = "" } } },
    .{ .venue = .aster, .name = "account", .verb = "GET", .path = "/fapi/v3/account", .private = true, .origin = "https://fapi.asterdex.com", .params = &.{} },
    .{ .venue = .aster, .name = "positions", .verb = "GET", .path = "/fapi/v3/positionRisk", .private = true, .origin = "https://fapi.asterdex.com", .params = &.{.{ .key = "symbol", .loc = .query, .required = false, .collection = "" }} },
    .{ .venue = .aster, .name = "open_orders", .verb = "GET", .path = "/fapi/v3/openOrders", .private = true, .origin = "https://fapi.asterdex.com", .params = &.{.{ .key = "symbol", .loc = .query, .required = false, .collection = "" }} },
    .{ .venue = .aster, .name = "order_status", .verb = "GET", .path = "/fapi/v3/order", .private = true, .origin = "https://fapi.asterdex.com", .params = &.{ .{ .key = "symbol", .loc = .query, .required = true, .collection = "" }, .{ .key = "orderId", .loc = .query, .required = false, .collection = "" }, .{ .key = "origClientOrderId", .loc = .query, .required = false, .collection = "" } } },
    .{ .venue = .aster, .name = "place_order", .verb = "POST", .path = "/fapi/v3/order", .private = true, .origin = "https://fapi.asterdex.com", .params = &.{.{ .key = "body", .loc = .body, .required = true, .collection = "" }} },
    .{ .venue = .aster, .name = "cancel_order", .verb = "DELETE", .path = "/fapi/v3/order", .private = true, .origin = "https://fapi.asterdex.com", .params = &.{.{ .key = "body", .loc = .body, .required = true, .collection = "" }} },
};

const r = @import("../../core/routes.zig");

pub const operations = [_]r.Operation{
    .{ .venue = .trade_xyz, .name = "perp_dexs", .verb = "POST", .path = "/info", .private = false, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .trade_xyz, .name = "markets", .verb = "POST", .path = "/info", .private = false, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .trade_xyz, .name = "order_book", .verb = "POST", .path = "/info", .private = false, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .trade_xyz, .name = "trades", .verb = "POST", .path = "/info", .private = false, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .trade_xyz, .name = "account", .verb = "POST", .path = "/info", .private = false, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .trade_xyz, .name = "positions", .verb = "POST", .path = "/info", .private = false, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .trade_xyz, .name = "open_orders", .verb = "POST", .path = "/info", .private = false, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .trade_xyz, .name = "order_status", .verb = "POST", .path = "/info", .private = false, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .trade_xyz, .name = "place_order", .verb = "POST", .path = "/exchange", .private = true, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .trade_xyz, .name = "cancel_order", .verb = "POST", .path = "/exchange", .private = true, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
};

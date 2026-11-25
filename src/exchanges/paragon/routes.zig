const r = @import("../../core/routes.zig");

pub const operations = [_]r.Operation{
    .{ .venue = .paragon, .name = "perp_dexs", .verb = "POST", .path = "/info", .private = false, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .paragon, .name = "markets", .verb = "POST", .path = "/info", .private = false, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .paragon, .name = "order_book", .verb = "POST", .path = "/info", .private = false, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .paragon, .name = "trades", .verb = "POST", .path = "/info", .private = false, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .paragon, .name = "account", .verb = "POST", .path = "/info", .private = false, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .paragon, .name = "positions", .verb = "POST", .path = "/info", .private = false, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .paragon, .name = "open_orders", .verb = "POST", .path = "/info", .private = false, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .paragon, .name = "order_status", .verb = "POST", .path = "/info", .private = false, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .paragon, .name = "place_order", .verb = "POST", .path = "/exchange", .private = true, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .paragon, .name = "cancel_order", .verb = "POST", .path = "/exchange", .private = true, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
};

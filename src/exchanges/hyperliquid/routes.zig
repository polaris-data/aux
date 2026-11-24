const Operation = @import("../../core/routes.zig").Operation;

// zig fmt: off
pub const operations = [_]Operation{
    .{ .venue = .hyperliquid, .name = "order", .verb = "POST", .path = "/exchange", .private = true, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .hyperliquid, .name = "cancel", .verb = "POST", .path = "/exchange", .private = true, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .hyperliquid, .name = "cancel_by_cloid", .verb = "POST", .path = "/exchange", .private = true, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .hyperliquid, .name = "meta", .verb = "POST", .path = "/info", .private = false, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .hyperliquid, .name = "l2_book", .verb = "POST", .path = "/info", .private = false, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .hyperliquid, .name = "clearinghouse_state", .verb = "POST", .path = "/info", .private = false, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
    .{ .venue = .hyperliquid, .name = "open_orders", .verb = "POST", .path = "/info", .private = false, .origin = "https://api.hyperliquid.xyz", .params = &.{} },
};

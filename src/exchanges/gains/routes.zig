const routes = @import("../../core/routes.zig");

pub const operations = [_]routes.Operation{
    .{ .venue = .gains, .name = "markets", .verb = "GET", .path = "/v1/arbitrum/markets", .private = false, .origin = "https://api.gains.trade", .params = &.{} },
};

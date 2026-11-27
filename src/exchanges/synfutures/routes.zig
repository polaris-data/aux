const routes = @import("../../core/routes.zig");

pub const operations = [_]routes.Operation{
    .{ .venue = .synfutures, .name = "virtual_trades", .verb = "POST", .path = "/thegraph/v3-base", .private = false, .origin = "https://api.synfutures.com", .params = &.{} },
    .{ .venue = .synfutures, .name = "candles", .verb = "POST", .path = "/thegraph/v3-base", .private = false, .origin = "https://api.synfutures.com", .params = &.{} },
};

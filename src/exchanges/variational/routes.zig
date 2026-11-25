const r = @import("../../core/routes.zig");

pub const operations = [_]r.Operation{
    .{ .venue = .variational, .name = "market_stats", .verb = "GET", .path = "/metadata/v2/stats", .private = false, .origin = "https://omni-client-api.prod.ap-northeast-1.variational.io", .params = &.{} },
};

const routes = @import("../../core/routes.zig");

pub const operations = [_]routes.Operation{
    .{ .venue = .vest, .name = "exchange_info", .verb = "GET", .path = "/v2/exchangeInfo", .private = false, .origin = "https://server-prod.hz.vestmarkets.com", .params = &.{} },
};

const r = @import("../../core/routes.zig");

const origin = "https://omni.apex.exchange";

pub const operations = [_]r.Operation{
    .{ .venue = .apex_omni, .name = "markets", .verb = "GET", .path = "/api/v3/symbols", .private = false, .origin = origin, .params = &.{} },
    .{ .venue = .apex_omni, .name = "ticker", .verb = "GET", .path = "/api/v3/ticker", .private = false, .origin = origin, .params = &.{.{ .key = "symbol", .loc = .query, .required = true, .collection = "" }} },
    .{ .venue = .apex_omni, .name = "trades", .verb = "GET", .path = "/api/v3/trades", .private = false, .origin = origin, .params = &.{ .{ .key = "symbol", .loc = .query, .required = true, .collection = "" }, .{ .key = "limit", .loc = .query, .required = false, .collection = "" } } },
};

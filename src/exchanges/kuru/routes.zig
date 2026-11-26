const r = @import("../../core/routes.zig");

const origin = "https://api.mainnet.kuru.io";

pub const operations = [_]r.Operation{
    .{ .venue = .kuru, .name = "markets", .verb = "GET", .path = "/api/v1/markets", .private = false, .origin = origin, .params = &.{ .{ .key = "status", .loc = .query, .required = false, .collection = "" }, .{ .key = "limit", .loc = .query, .required = false, .collection = "" }, .{ .key = "cursor", .loc = .query, .required = false, .collection = "" } } },
    .{ .venue = .kuru, .name = "market", .verb = "GET", .path = "/api/v1/markets/{marketAddress}", .private = false, .origin = origin, .params = &.{.{ .key = "marketAddress", .loc = .path, .required = true, .collection = "" }} },
    .{ .venue = .kuru, .name = "trades", .verb = "GET", .path = "/api/v1/markets/{marketAddress}/trades", .private = false, .origin = origin, .params = &.{ .{ .key = "marketAddress", .loc = .path, .required = true, .collection = "" }, .{ .key = "limit", .loc = .query, .required = false, .collection = "" }, .{ .key = "cursor", .loc = .query, .required = false, .collection = "" }, .{ .key = "from", .loc = .query, .required = false, .collection = "" }, .{ .key = "to", .loc = .query, .required = false, .collection = "" } } },
};

const r = @import("../../core/routes.zig");

const origin = "https://api.reya.xyz";

pub const operations = [_]r.Operation{
    .{ .venue = .reya, .name = "markets", .verb = "GET", .path = "/v2/perpMarketDefinitions", .private = false, .origin = origin, .params = &.{} },
    .{ .venue = .reya, .name = "market_summaries", .verb = "GET", .path = "/v2/perpMarkets/summary", .private = false, .origin = origin, .params = &.{} },
    .{ .venue = .reya, .name = "market_summary", .verb = "GET", .path = "/v2/perpMarket/{symbol}/summary", .private = false, .origin = origin, .params = &.{.{ .key = "symbol", .loc = .path, .required = true, .collection = "" }} },
    .{ .venue = .reya, .name = "market_executions", .verb = "GET", .path = "/v2/market/{symbol}/perpExecutions", .private = false, .origin = origin, .params = &.{ .{ .key = "symbol", .loc = .path, .required = true, .collection = "" }, .{ .key = "startTime", .loc = .query, .required = false, .collection = "" }, .{ .key = "endTime", .loc = .query, .required = false, .collection = "" }, .{ .key = "type", .loc = .query, .required = false, .collection = "" }, .{ .key = "limit", .loc = .query, .required = false, .collection = "" }, .{ .key = "cursor", .loc = .query, .required = false, .collection = "" } } },
    .{ .venue = .reya, .name = "wallet_executions", .verb = "GET", .path = "/v2/wallet/{address}/perpExecutions", .private = false, .origin = origin, .params = &.{ .{ .key = "address", .loc = .path, .required = true, .collection = "" }, .{ .key = "startTime", .loc = .query, .required = false, .collection = "" }, .{ .key = "endTime", .loc = .query, .required = false, .collection = "" }, .{ .key = "type", .loc = .query, .required = false, .collection = "" }, .{ .key = "limit", .loc = .query, .required = false, .collection = "" }, .{ .key = "cursor", .loc = .query, .required = false, .collection = "" } } },
};

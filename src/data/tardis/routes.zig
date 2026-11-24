const Operation = @import("../../core/routes.zig").Operation;

// zig fmt: off
pub const operations = [_]Operation{
    .{ .venue = .tardis, .name = "exchanges", .verb = "GET", .path = "/v1/exchanges", .private = false, .origin = "https://api.tardis.dev", .params = &.{} },
    .{ .venue = .tardis, .name = "exchange_details", .verb = "GET", .path = "/v1/exchanges/{exchange}", .private = false, .origin = "https://api.tardis.dev", .params = &.{.{ .key = "exchange", .loc = .path, .required = true, .collection = "multi", .header = "" }} },
    .{ .venue = .tardis, .name = "exchange_summary", .verb = "GET", .path = "/v1/exchanges/{exchange}/summary", .private = false, .origin = "https://api.tardis.dev", .params = &.{.{ .key = "exchange", .loc = .path, .required = true, .collection = "multi", .header = "" }} },
    .{ .venue = .tardis, .name = "symbols", .verb = "GET", .path = "/v1/symbols", .private = false, .origin = "https://api.tardis.dev", .params = &.{} },
    .{ .venue = .tardis, .name = "api_key_info", .verb = "GET", .path = "/v1/api-key-info", .private = true, .origin = "https://api.tardis.dev", .params = &.{} },
    .{ .venue = .tardis, .name = "instruments", .verb = "GET", .path = "/v1/instruments/{exchange}", .private = false, .origin = "https://api.tardis.dev", .params = &.{.{ .key = "exchange", .loc = .path, .required = true, .collection = "multi", .header = "" }} },
    .{ .venue = .tardis, .name = "instrument", .verb = "GET", .path = "/v1/instruments/{exchange}/{symbol}", .private = false, .origin = "https://api.tardis.dev", .params = &.{.{ .key = "exchange", .loc = .path, .required = true, .collection = "multi", .header = "" }, .{ .key = "symbol", .loc = .path, .required = true, .collection = "multi", .header = "" }} },
};

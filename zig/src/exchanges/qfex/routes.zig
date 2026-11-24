const Operation = @import("../../core/routes.zig").Operation;

// zig fmt: off
pub const operations = [_]Operation{
    .{ .venue = .qfex, .name = "contracts", .verb = "GET", .path = "/md/contracts", .private = false, .origin = "https://api.qfex.com", .params = &.{.{ .key = "symbol", .loc = .query, .required = false, .collection = "multi", .header = "" }} },
    .{ .venue = .qfex, .name = "account_balances", .verb = "GET", .path = "/user/subaccounts/balance", .private = true, .origin = "https://api.qfex.com", .params = &.{} },
    .{ .venue = .qfex, .name = "refdata", .verb = "GET", .path = "/refdata", .private = false, .origin = "https://api.qfex.com", .params = &.{.{ .key = "ticker", .loc = .query, .required = false, .collection = "multi", .header = "" }} },
    .{ .venue = .qfex, .name = "order_book", .verb = "GET", .path = "/md/orderbook/{symbol}", .private = false, .origin = "https://api.qfex.com", .params = &.{.{ .key = "symbol", .loc = .path, .required = true, .collection = "multi", .header = "" }} },
    .{ .venue = .qfex, .name = "positions", .verb = "GET", .path = "/user/positions", .private = true, .origin = "https://api.qfex.com", .params = &.{} },
};

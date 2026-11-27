const std = @import("std");
const wire = @import("../../core/wire.zig");
const p = @import("../../core/perps.zig");
const eq = std.mem.eql;

pub const Interval = enum {
    @"1h",
    @"4h",
    @"1d",
    @"1w",

    fn field(self: Interval) []const u8 {
        return switch (self) {
            .@"1h" => "hourlyAmmDatas",
            .@"4h" => "per4HourAmmDatas",
            .@"1d" => "dailyAmmDatas",
            .@"1w" => "weeklyAmmDatas",
        };
    }
};
pub const Feed = union(enum) { virtual_trades, candles: Interval };
pub const page_size = 1000;

pub fn request(a: wire.A, operation: []const u8, fields: wire.V) !wire.V {
    const trades = eq(u8, operation, "virtual_trades");
    const from = try wire.unsigned(u64, try wire.field(fields, "since"));
    const until = try wire.unsigned(u64, try wire.field(fields, "until"));
    if (from > until) return error.InvalidTimeRange;
    const cursor = p.get(fields, "last");
    const last = if (cursor == .null) "" else try wire.string(cursor);
    const size = p.get(fields, "first");
    const first = if (size == .null) page_size else try wire.unsigned(u16, size);
    if (first == 0 or first > page_size) return error.InvalidPageSize;

    var variables = p.object();
    try variables.object.put(a, "since", wire.text(try std.fmt.allocPrint(a, "{d}", .{from})));
    try variables.object.put(a, "until", wire.text(try std.fmt.allocPrint(a, "{d}", .{until})));
    try variables.object.put(a, "last", wire.text(last));
    try variables.object.put(a, "first", .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{first}) });
    const query = if (trades)
        "query($since:BigInt!,$until:BigInt!,$last:String!,$first:Int!){_meta{block{number timestamp}} virtualTrades(first:$first,orderBy:id,orderDirection:asc,where:{timestamp_gte:$since,timestamp_lte:$until,id_gt:$last}){id type price size tradeValue fee trader timestamp blockNumber original{name args logIndex transaction{id}} amm{id expiry symbol instrument{id}}}}"
    else blk: {
        const interval = std.meta.stringToEnum(Interval, try wire.string(try wire.field(fields, "interval"))) orelse return error.InvalidInterval;
        const ids = try wire.field(fields, "amm_ids");
        if (ids != .array or ids.array.items.len == 0 or ids.array.items.len > page_size) return error.InvalidMarkets;
        for (ids.array.items) |item| if ((try wire.string(item)).len == 0) return error.InvalidMarkets;
        try variables.object.put(a, "ids", ids);
        break :blk try std.fmt.allocPrint(a, "query($ids:[String!],$since:BigInt!,$until:BigInt!,$last:String!,$first:Int!){{{s}(first:$first,orderBy:id,orderDirection:asc,where:{{amm_in:$ids,timestamp_gte:$since,timestamp_lte:$until,open_gt:0,id_gt:$last}}){{id timestamp open high low close volume baseVolume amm{{id}}}}}}", .{interval.field()});
    };
    var body = p.object();
    try body.object.put(a, "query", wire.text(query));
    try body.object.put(a, "variables", variables);
    return body;
}

/// The returned cursor borrows the response. A short page completes the window.
pub fn nextCursor(response: wire.V, feed: Feed, last: []const u8, first: u16) !?[]const u8 {
    if (first == 0 or first > page_size) return error.InvalidPageSize;
    const errors = p.get(response, "errors");
    if (errors != .null and (errors != .array or errors.array.items.len != 0)) return error.GraphQlError;
    const field = switch (feed) {
        .virtual_trades => "virtualTrades",
        .candles => |interval| interval.field(),
    };
    const rows = try wire.field(try wire.field(response, "data"), field);
    if (rows != .array or rows.array.items.len > first) return error.InvalidPage;
    var cursor = last;
    for (rows.array.items) |row| {
        const id = try wire.string(try wire.field(row, "id"));
        if (std.mem.order(u8, id, cursor) != .gt) return error.InvalidCursor;
        cursor = id;
    }
    return if (rows.array.items.len == first) cursor else null;
}

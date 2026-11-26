const std = @import("std");
const x = @import("../../core/exchange_stream.zig");
const w = @import("../../core/wire.zig");
const p = @import("../../core/perps.zig");

pub fn init(s: *x.State, a: w.A, params: w.V, channel: []const u8, endpoint: ?[]const u8, testnet: bool) !void {
    s.endpoint = endpoint orelse if (testnet) "wss://websocket-testnet.reya.xyz" else "wss://ws.reya.xyz";
    if (std.mem.eql(u8, channel, "market_summaries")) {
        s.topic = "/v2/perpMarkets/summary";
    } else if (std.mem.eql(u8, channel, "market_summary")) {
        s.topic = try std.fmt.allocPrint(a, "/v2/perpMarket/{s}/summary", .{try symbol(params)});
    } else if (std.mem.eql(u8, channel, "market_executions")) {
        s.topic = try std.fmt.allocPrint(a, "/v2/market/{s}/perpExecutions", .{try symbol(params)});
        s.auto_reconnect = false;
    } else if (std.mem.eql(u8, channel, "wallet_executions")) {
        const address = try x.requiredText(params, "address");
        if (address.len != 42 or !std.mem.startsWith(u8, address, "0x")) return error.InvalidSubscription;
        for (address[2..]) |digit| if (!std.ascii.isHex(digit)) return error.InvalidSubscription;
        s.topic = try std.fmt.allocPrint(a, "/v2/wallet/{s}/perpExecutions", .{address});
        s.auto_reconnect = false;
    } else return error.InvalidSubscription;
    s.frame = try x.stringify(a, .{ .type = "subscribe", .channel = s.topic });
}

fn symbol(params: w.V) ![]const u8 {
    const value = try x.requiredText(params, "symbol");
    for (value) |c| if (!std.ascii.isAlphanumeric(c)) return error.InvalidSubscription;
    return value;
}

pub fn event(s: *x.State, frame: w.V) x.Event {
    if (x.same(frame, "type", "disconnectNotice")) return .deliver;
    if (!x.same(frame, "channel", s.topic)) return .skip;
    if (x.same(frame, "type", "channel_data") and p.get(frame, "data") != .null) return .deliver;
    if (x.same(frame, "type", "subscribed") and p.get(frame, "contents") != .null) return .deliver;
    return .skip;
}

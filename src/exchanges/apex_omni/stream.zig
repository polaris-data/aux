const std = @import("std");
const x = @import("../../core/exchange_stream.zig");
const w = @import("../../core/wire.zig");
const p = @import("../../core/perps.zig");
const eq = std.mem.eql;

pub fn init(s: *x.State, a: w.A, params: w.V, channel: []const u8, endpoint: ?[]const u8, testnet: bool) !void {
    s.endpoint = endpoint orelse if (testnet)
        "wss://qa-quote.omni.apex.exchange/realtime_public?v=2"
    else
        "wss://quote.omni.apex.exchange/realtime_public?v=2";
    if (eq(u8, channel, "tickers")) {
        s.topic = "instrumentInfo.all";
    } else {
        const symbol = try x.requiredText(params, "symbol");
        if (eq(u8, channel, "book")) {
            s.book = true;
            const depth = try x.optionalDepth(params, 200);
            if (depth != 25 and depth != 200) return error.InvalidSubscription;
            const frequency = (try x.optionalText(params, "frequency")) orelse "H";
            if (!x.in(frequency, &.{ "H", "M" })) return error.InvalidSubscription;
            s.topic = try std.fmt.allocPrint(a, "orderBook{d}.{s}.{s}", .{ depth, frequency, symbol });
        } else if (eq(u8, channel, "ticker")) {
            s.topic = try std.fmt.allocPrint(a, "instrumentInfo.H.{s}", .{symbol});
        } else if (eq(u8, channel, "trades")) {
            s.topic = try std.fmt.allocPrint(a, "recentlyTrade.H.{s}", .{symbol});
        } else return error.InvalidSubscription;
    }
    s.frame = try subscriptionFrame(a, true, &.{s.topic});
}

pub fn subscriptionFrame(a: w.A, subscribe: bool, topics: []const []const u8) ![]const u8 {
    return x.stringify(a, .{ .op = if (subscribe) "subscribe" else "unsubscribe", .args = topics });
}

pub fn openingUrl(a: w.A, input: w.V, io: std.Io, url: std.Uri) !?std.Uri {
    if (p.get(input, "origin") != .null) return null;
    return try timestampedUrl(a, url, try p.epoch(io, .ms));
}

pub fn timestampedUrl(a: w.A, url: std.Uri, timestamp_ms: u64) !std.Uri {
    var endpoint = url;
    endpoint.query = .{ .raw = try std.fmt.allocPrint(a, "v=2&timestamp={d}", .{timestamp_ms}) };
    return endpoint;
}

pub fn heartbeat(buffer: []u8, io: std.Io, comptime op: []const u8) ![]const u8 {
    return std.fmt.bufPrint(buffer, "{{\"op\":\"" ++ op ++ "\",\"args\":[\"{d}\"]}}", .{try p.epoch(io, .ms)});
}

pub fn event(s: *x.State, a: w.A, frame: w.V, io: std.Io) !x.Event {
    if (x.same(frame, "op", "ping")) {
        var buffer: [80]u8 = undefined;
        return .{ .reply = try a.dupe(u8, try heartbeat(&buffer, io, "pong")) };
    }
    if (x.same(frame, "topic", s.topic) and p.get(frame, "data") != .null) return .deliver;
    return .skip;
}

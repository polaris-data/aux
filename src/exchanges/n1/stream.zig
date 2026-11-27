const std = @import("std");
const wire = @import("../../core/wire.zig");
const ws = @import("../../core/websocket.zig");
const Event = @import("../derive/stream.zig").State.Event;
const eq = std.mem.eql;

const Channel = enum { trades, deltas, candle, rfq_fills, liquidations };
const resolutions: []const []const u8 = &.{ "1", "5", "15", "30", "60", "4H", "1D", "1W", "1M" };
pub const endpoint = "wss://api-mainnet.n1.xyz";

pub const State = struct {
    url: []const u8,
    channels: std.EnumSet(Channel),

    pub fn init(a: wire.A, override: ?[]const u8, spec: wire.V) !State {
        const items = try wire.field(spec, "subscriptions");
        if (items != .array) return error.InvalidSubscription;
        if (items.array.items.len == 0) return error.EmptySubscription;

        const base = override orelse endpoint;
        const uri = try ws.Session.parseEndpoint(base);
        if (uri.query != null) return error.InvalidEndpoint;
        var url: std.Io.Writer.Allocating = .init(a);
        errdefer url.deinit();
        try url.writer.print("{s}/ws/", .{std.mem.trimEnd(u8, base, "/")});
        var channels: std.EnumSet(Channel) = .empty;
        for (items.array.items, 0..) |item, index| {
            const kind = std.meta.stringToEnum(Channel, try wire.string(try wire.field(item, "channel"))) orelse return error.UnknownChannel;
            const symbol = item.object.get("symbol") orelse .null;
            const resolution = item.object.get("resolution") orelse .null;
            for (item.object.keys()) |key| {
                if (!eq(u8, key, "channel") and !eq(u8, key, "symbol") and !eq(u8, key, "resolution")) return error.UnknownParameter;
            }
            if (index != 0) try url.writer.writeByte('&');
            try url.writer.writeAll(if (kind == .rfq_fills) "rfq-fills" else @tagName(kind));
            if (kind == .liquidations) {
                if (symbol != .null) return error.InvalidSymbol;
            } else {
                const name = wire.string(symbol) catch return error.InvalidSymbol;
                if (name.len == 0) return error.InvalidSymbol;
                for (name) |c| if (!std.ascii.isAlphanumeric(c) and std.mem.indexOfScalar(u8, "-._", c) == null) return error.InvalidSymbol;
                try url.writer.print("@{s}", .{name});
            }
            if (kind == .candle) {
                const period = wire.string(resolution) catch return error.InvalidResolution;
                for (resolutions) |valid| {
                    if (eq(u8, period, valid)) break;
                } else return error.InvalidResolution;
                try url.writer.print(":{s}", .{period});
            } else if (resolution != .null) return error.InvalidResolution;
            channels.insert(kind);
        }
        return .{ .url = try url.toOwnedSlice(), .channels = channels };
    }

    pub fn event(self: State, a: wire.A, raw: []const u8) !Event {
        const message = try wire.parse(a, raw);
        if (message != .object) return error.InvalidResponse;
        const object = message.object;
        const kind = object.get("type") orelse .null;
        if ((object.get("error") orelse .null) != .null or (kind == .string and eq(u8, kind.string, "error"))) return error.ExchangeApi;
        const channel: Channel = if (object.contains("trades"))
            .trades
        else if (object.contains("delta"))
            .deltas
        else if (object.contains("rfqFills"))
            .rfq_fills
        else if (object.contains("liquidation"))
            .liquidations
        else if (object.contains("res") and object.contains("mid"))
            .candle
        else
            return .skip;
        return if (self.channels.contains(channel)) .{ .deliver = raw } else .skip;
    }
};

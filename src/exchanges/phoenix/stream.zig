const std = @import("std");
const w = @import("../../core/wire.zig");
const eq = std.mem.eql;
pub const State = struct {
    sequence: ?u64 = null,
    waiting: bool = false,
    snapshots: bool = false,
    pub fn reset(self: *State) void {
        self.sequence = null;
        self.waiting = self.snapshots;
    }
    pub fn init(params: w.V, authenticated: bool) !State {
        const channel = try w.string(try w.field(params, "channel"));
        if (eq(u8, channel, "notifications")) {
            if (!authenticated) return error.AuthenticationRequired;
        } else if (eq(u8, channel, "traderState")) {
            if ((try w.string(try w.field(params, "authority"))).len == 0) return error.InvalidAuthority;
        } else if (!eq(u8, channel, "allMids") and !eq(u8, channel, "exchange")) {
            for ([_][]const u8{ "fundingRate", "orderbook", "market", "trades", "candles" }) |v| {
                if (eq(u8, v, channel)) break;
            } else return error.UnknownChannel;
            if ((try w.string(try w.field(params, "symbol"))).len == 0) return error.InvalidSymbol;
        }
        return .{ .snapshots = eq(u8, channel, "exchange") or eq(u8, channel, "traderState") };
    }
    pub fn process(self: *State, a: w.A, raw: []const u8) !?[]const u8 {
        var event = try w.parse(a, raw);
        if (event != .object) return error.InvalidEvent;
        const kind = optionalString(event, "type");
        const channel = optionalString(event, "channel");
        if (eq(u8, kind, "subscriptionError") or eq(u8, channel, "error")) return error.ExchangeApi;
        const typ = optionalString(event, "messageType");
        const encoded = eq(u8, typ, "encodedSnapshot");
        const snapshot = encoded or eq(u8, typ, "snapshot");
        if (self.waiting and !snapshot and !eq(u8, kind, "subscriptionConfirmed")) return null;
        if (snapshot) self.waiting = false;
        if (event.object.get("sequenceNumber")) |n| {
            const text = if (n == .number_string) n.number_string else if (n == .string) n.string else return error.InvalidSequence;
            const seq = try std.fmt.parseInt(u64, text, 10);
            if (self.sequence) |previous| if (!snapshot and (previous == std.math.maxInt(u64) or seq != previous + 1)) return error.SequenceGap;
            self.sequence = seq;
        }
        if (!encoded) return raw;
        if (!eq(u8, optionalString(event, "encoding"), "base64+zstd")) return error.InvalidEncoding;
        const base64 = try w.string(try w.field(event, "payload"));
        const decoder = std.base64.standard.Decoder;
        const compressed = try a.alloc(u8, try decoder.calcSizeForSlice(base64));
        try decoder.decode(compressed, base64);
        var reader = std.Io.Reader.fixed(compressed);
        const zstd = std.compress.zstd;
        var decompressed = zstd.Decompress.init(&reader, try a.alloc(u8, zstd.default_window_len + zstd.block_size_max), .{});
        const json = try decompressed.reader.allocRemaining(a, .limited(8 * 1024 * 1024));
        try event.object.put(a, "decodedPayload", try w.parse(a, json));
        return try std.json.Stringify.valueAlloc(a, event, .{});
    }
};
fn optionalString(v: w.V, k: []const u8) []const u8 {
    if (v.object.get(k)) |s| if (s == .string) return s.string;
    return "";
}

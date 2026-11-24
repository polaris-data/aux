// Pascal and Kalshi dedicate a connection to each subscription.
const std = @import("std");
const w = @import("wire.zig");
const eq = std.mem.eql;
const Service = enum { read, predictions, predictions_warp, perps, perps_warp };
pub const State = struct {
    service: Service,
    channel: []const u8,
    frame: []const u8,
    key: ?[]const u8 = null,
    seed: ?[]const u8 = null,
    ready: bool = false,
    seq: ?u64 = null,
    sid: ?u64 = null,

    pub fn init(a: w.A, venue: []const u8, config: w.V, sub: w.V) !State {
        const service = std.meta.stringToEnum(Service, try w.string(try w.field(sub, "service"))) orelse return error.UnknownService;
        const pascal = eq(u8, venue, "pascal");
        if ((service == .read) != pascal) return error.UnknownService;
        var params = try w.field(sub, "params");
        const channel = try w.string(try w.field(params, "channel"));
        if (pascal) {
            for ([_][]const u8{ "book", "trades", "all_trades", "filtered_trades", "candles", "mark_price_candles", "account", "market_update" }) |allowed| {
                if (eq(u8, allowed, channel)) break;
            } else return error.UnknownChannel;
            return .{ .service = service, .channel = channel, .frame = try std.json.Stringify.valueAlloc(a, .{ .type = "subscribe", .channels = .{params} }, .{}) };
        }
        const warp = service == .predictions_warp or service == .perps_warp;
        if (warp and !eq(u8, channel, "fill") and !eq(u8, channel, "user_orders")) return error.UnknownChannel;
        const inventory = if (service == .predictions or service == .predictions_warp) @embedFile("kalshi-predictions-channels.txt") else @embedFile("kalshi-perps-channels.txt");
        var list = std.mem.tokenizeScalar(u8, inventory, '\n');
        while (list.next()) |name| {
            if (eq(u8, name, channel)) break;
        } else return error.UnknownChannel;
        _ = params.object.swapRemove("channel");
        var channels = std.array_list.Managed(w.V).init(a);
        try channels.append(w.text(channel));
        try params.object.put(a, "channels", .{ .array = channels });
        return .{ .service = service, .channel = channel, .key = try w.string(config.object.get("api_key") orelse return error.AuthenticationRequired), .seed = try w.string(config.object.get("api_secret") orelse return error.AuthenticationRequired), .frame = try std.json.Stringify.valueAlloc(a, .{ .id = @as(u32, 1), .cmd = "subscribe", .params = params }, .{}) };
    }
    pub fn endpoint(self: State) []const u8 {
        return switch (self.service) {
            .read => "wss://data.pascal.trade/ws",
            .predictions => "wss://external-api-ws.kalshi.com/trade-api/ws/v2",
            .perps => "wss://external-api-margin-ws.kalshi.com/trade-api/ws/v2/margin",
            .predictions_warp => "wss://warp-ws.kalshi.com/trade-api/ws/v2",
            .perps_warp => "wss://margin-warp-ws.kalshi.com/trade-api/ws/v2/margin",
        };
    }
    pub fn reset(self: *State) void {
        self.ready = false;
        self.sid = null;
        self.seq = null;
    }
    pub fn process(self: *State, a: w.A, raw: []const u8) !?[]const u8 {
        if (eq(u8, raw, "PING")) return "{\"type\":\"PING\"}";
        if (eq(u8, raw, "PONG")) return "{\"type\":\"PONG\"}";
        const v = try w.parse(a, raw);
        const kind = try w.string(try w.field(v, "type"));
        if (eq(u8, kind, "error")) return error.ExchangeApi;
        if (self.service == .read) {
            if (eq(u8, kind, "snapshot")) self.ready = true;
            if (!self.ready and eq(u8, kind, "update") and (eq(u8, self.channel, "account") or eq(u8, self.channel, "book") or eq(u8, self.channel, "market_update"))) return null;
        } else {
            if (eq(u8, kind, "subscribed")) {
                if (v.object.get("id")) |id| if (try w.unsigned(u64, id) != 1) return null;
                self.sid = try w.unsigned(u64, try w.field(try w.field(v, "msg"), "sid"));
            }
            if (self.sid) |sid| if (v.object.get("sid")) |val| if (try w.unsigned(u64, val) != sid) return null;
            if (eq(u8, kind, "orderbook_snapshot")) self.ready = true;
            if (!self.ready and eq(u8, self.channel, "orderbook_delta") and eq(u8, kind, "orderbook_delta")) return null;
            if (v.object.get("seq")) |val| {
                const seq = try w.unsigned(u64, val);
                if (self.seq) |prev| if (seq != prev +| 1) return error.SequenceGap;
                self.seq = seq;
            }
        }
        return raw;
    }
};

const std = @import("std");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const host = @import("../../core/host_stream.zig");
const Event = @import("../derive/stream.zig").State.Event;

const Channel = enum { trade, book_depth, best_bid_offer, order_update, fill, position_change };

pub const State = struct {
    endpoint: []const u8,
    config: p.V,
    channel: Channel,
    subaccount: p.V,
    product: ?u32,
    frame: []const u8,
    authenticated: bool = false,
    sequence: ?u64,
    sequence_started: bool = false,
    connected: bool = false,
    lost_book: bool = false,

    pub fn init(a: p.A, config: p.V, params: p.V) !State {
        const name = try p.text(params, "type");
        const channel = std.meta.stringToEnum(Channel, name) orelse return error.InvalidSubscription;
        const raw_product = p.get(params, "product_id");
        const product = if (raw_product == .null) null else try w.unsigned(u32, raw_product);
        const subaccount = p.get(params, "subaccount");
        if (channel == .order_update or channel == .fill or channel == .position_change) {
            _ = try @import("../../core/evm.zig").hex(32, try w.string(subaccount));
        } else if (product == null or subaccount != .null) return error.InvalidSubscription;
        var stream = p.object();
        try p.put(&stream, a, "type", w.text(name));
        try p.put(&stream, a, "product_id", raw_product);
        if (subaccount != .null) try p.put(&stream, a, "subaccount", subaccount);
        const timestamp = p.get(params, "snapshot_timestamp");
        if (timestamp != .null and channel != .book_depth) return error.InvalidSubscription;
        return .{
            .endpoint = if (try p.testnet(config)) "wss://api.test.nado.xyz/gateway/v1/subscribe" else "wss://api.prod.nado.xyz/gateway/v1/subscribe",
            .config = config,
            .channel = channel,
            .subaccount = subaccount,
            .product = product,
            .frame = try std.json.Stringify.valueAlloc(a, .{ .method = "subscribe", .id = 2, .stream = stream }, .{}),
            .sequence = if (timestamp == .null) null else try w.unsigned(u64, timestamp),
        };
    }

    pub fn start(self: *State, a: p.A, io: std.Io, dial: ?host.Dial) ![]const u8 {
        if (self.connected and self.channel == .book_depth) {
            self.sequence = null;
            self.sequence_started = false;
            self.lost_book = true;
        }
        self.connected = true;
        self.authenticated = false;
        if (self.channel != .order_update) return self.frame;
        const auth = p.get(self.config, "auth");
        var key = try p.evmKey(auth);
        std.crypto.secureZero(u8, &key);
        var args = try p.toValue(a, .{ .venue = "nado", .operation = "contracts", .params = p.object(), .auth = auth });
        const origin = p.get(auth, "rest_origin");
        if (origin != .null) try p.put(&args, a, "origin", origin);
        const result = try @import("../../core/transport.zig").executeWithDial(a, args, dial);
        return @import("trading.zig").authenticate(a, auth, self.subaccount, try w.field(result, "data"), try p.epoch(io, .ms));
    }

    pub fn event(self: *State, a: p.A, raw: []const u8) !Event {
        const v = try w.parse(a, raw);
        const request_id = p.get(v, "id");
        const result = p.get(v, "result");
        const failure = p.get(v, "error");
        if (failure != .null or p.get(v, "error_code") != .null) return error.ExchangeApi;
        const tag = p.get(v, "type");
        if (tag == .null) {
            if (request_id == .null) return .skip;
            const id = try w.unsigned(u64, request_id);
            if (id == 1 and self.channel == .order_update and !self.authenticated) {
                if (result != .null) return error.ExchangeApi;
                self.authenticated = true;
                return .subscribe;
            }
            return .skip;
        }
        if (!p.eq(u8, try w.string(tag), @tagName(self.channel))) return .skip;
        if (self.channel == .order_update and !self.authenticated) return .skip;
        if (self.product) |product| if (try w.unsigned(u32, try w.field(v, "product_id")) != product) return .skip;
        if (self.subaccount == .string) {
            const account = p.get(v, "subaccount");
            if (account != .null and !std.ascii.eqlIgnoreCase(try w.string(account), self.subaccount.string)) return .skip;
        }
        if (self.channel == .book_depth) {
            const current = try w.unsigned(u64, try w.field(v, "max_timestamp"));
            const previous = try w.unsigned(u64, try w.field(v, "last_max_timestamp"));
            if (previous > current) return error.InvalidEvent;
            if (self.sequence) |sequence| {
                if (current <= sequence) return .skip;
                if (previous > sequence or (self.sequence_started and previous != sequence)) self.lost_book = true;
            }
            self.sequence = current;
            self.sequence_started = true;
            if (self.lost_book) {
                self.lost_book = false;
                return error.RecoverableStreamGap;
            }
        }
        return .{ .deliver = raw };
    }
};

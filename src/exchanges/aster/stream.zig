const std = @import("std");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const host = @import("../../core/host_stream.zig");
const transport = @import("../../core/transport.zig");
const Event = @import("../derive/stream.zig").State.Event;
const gpa = std.heap.c_allocator;

const Channel = enum { agg_trade, order_book, account };
const ListenId = [32]u8;
var listen_keys: std.AutoHashMapUnmanaged(ListenId, u32) = .empty;
var keys_mutex: std.Io.Mutex = .init;

pub const State = struct {
    channel: Channel,
    symbol: []const u8 = "",
    endpoint: []const u8,
    auth: p.V,
    listen_key: ?[]u8 = null,
    listen_id: ListenId = undefined,
    expired: std.atomic.Value(bool) = .init(false),

    pub fn init(a: p.A, config: p.V, params: p.V) !State {
        if (try p.testnet(config)) return error.InvalidNetwork;
        const channel = std.meta.stringToEnum(Channel, try p.text(params, "channel")) orelse return error.InvalidSubscription;
        const configured = p.get(config, "origin");
        const endpoint = if (configured != .null) try w.string(configured) else "wss://fstream.asterdex.com/ws";
        const uri = try std.Uri.parse(endpoint);
        if (uri.fragment != null or uri.query != null) return error.InvalidEndpoint;
        const auth = p.get(config, "auth");
        if (channel == .account) {
            var key = try p.evmKey(auth);
            std.crypto.secureZero(u8, &key);
            return .{ .channel = channel, .endpoint = endpoint, .auth = auth };
        }
        const symbol = try p.text(params, "symbol");
        if (symbol.len == 0) return error.InvalidMarket;
        for (symbol) |c| if (!std.ascii.isAlphanumeric(c) and c != '_') return error.InvalidMarket;
        const stream = if (channel == .agg_trade) "aggTrade" else "depth5";
        const url = try std.fmt.allocPrint(a, "{s}/{s}@{s}", .{ std.mem.trimEnd(u8, endpoint, "/"), try std.ascii.allocLowerString(a, symbol), stream });
        return .{ .channel = channel, .symbol = symbol, .endpoint = url, .auth = auth };
    }

    pub fn hasListenKey(self: State) bool {
        return self.channel == .account;
    }

    pub fn connect(self: *State, a: p.A, dial: ?host.Dial) ![]const u8 {
        if (!self.hasListenKey()) return self.endpoint;
        if (self.listen_key != null and !self.expired.load(.acquire)) self.renew(dial) catch {};
        if (self.expired.swap(false, .acq_rel)) {
            self.releaseKey(dial, false);
        }
        if (self.listen_key == null) {
            try self.acquireKey(a, dial);
        }
        return std.fmt.allocPrint(a, "{s}/{s}", .{ std.mem.trimEnd(u8, self.endpoint, "/"), self.listen_key.? });
    }

    pub fn renew(self: *State, dial: ?host.Dial) !void {
        var arena = std.heap.ArenaAllocator.init(gpa);
        defer arena.deinit();
        errdefer self.expired.store(true, .release);
        _ = try self.listen(arena.allocator(), "PUT", dial);
    }

    fn restOrigin(self: State) ![]const u8 {
        const configured = p.get(self.auth, "rest_origin");
        return if (configured == .null) "https://fapi.asterdex.com" else try w.string(configured);
    }

    fn acquireKey(self: *State, a: p.A, dial: ?host.Dial) !void {
        var runtime: host.Runtime = .{};
        keys_mutex.lockUncancelable(runtime.io());
        defer keys_mutex.unlock(runtime.io());

        const result = try self.listen(a, "POST", dial);
        const key = try p.text(result, "listenKey");
        if (key.len == 0 or key.len > 256) return error.InvalidResponse;
        for (key) |char| if (!std.ascii.isAlphanumeric(char) and char != '_' and char != '-') return error.InvalidResponse;
        var hash = std.crypto.hash.sha2.Sha256.init(.{});
        hash.update(try self.restOrigin());
        hash.update(&.{0});
        hash.update(key);
        self.listen_id = hash.finalResult();
        const owned = try gpa.dupe(u8, key);
        errdefer gpa.free(owned);
        const refs = try listen_keys.getOrPut(gpa, self.listen_id);
        if (!refs.found_existing) refs.value_ptr.* = 0;
        refs.value_ptr.* += 1;
        self.listen_key = owned;
    }

    fn releaseKey(self: *State, dial: ?host.Dial, delete: bool) void {
        const key = self.listen_key orelse return;
        var arena = std.heap.ArenaAllocator.init(gpa);
        defer arena.deinit();
        var runtime: host.Runtime = .{};
        keys_mutex.lockUncancelable(runtime.io());
        defer keys_mutex.unlock(runtime.io());
        const refs = listen_keys.getPtr(self.listen_id).?;
        refs.* -= 1;
        if (refs.* == 0) {
            _ = listen_keys.remove(self.listen_id);
            if (delete) _ = self.listen(arena.allocator(), "DELETE", dial) catch .null;
        }
        gpa.free(key);
        self.listen_key = null;
    }

    pub fn deinit(self: *State, dial: ?host.Dial) void {
        self.releaseKey(dial, !self.expired.load(.acquire));
    }

    fn listen(self: *State, a: p.A, method: []const u8, dial: ?host.Dial) !p.V {
        const base = try self.restOrigin();
        const uri = try std.Uri.parse(base);
        if (uri.host == null or uri.user != null or uri.password != null or uri.query != null or uri.fragment != null or
            !(p.eq(u8, uri.scheme, "http") or p.eq(u8, uri.scheme, "https"))) return error.InvalidOrigin;
        var rt: host.Runtime = .{ .dial = dial };
        const io = rt.io();
        var client: std.http.Client = .{ .allocator = gpa, .io = io };
        defer client.deinit();
        const signed = try @import("trading.zig").sign(a, self.auth, method, "/fapi/v3/listenKey", .null, try p.epoch(io, .us));
        const headers = try p.toValue(a, .{ .@"content-type" = "application/x-www-form-urlencoded" });
        const url = try std.mem.concat(a, u8, &.{ std.mem.trimEnd(u8, base, "/"), signed.path });
        var request = try transport.sendRequest(a, &client, method, url, headers, signed.body);
        defer request.deinit();
        errdefer if (request.connection) |conn| {
            conn.closing = true;
        };
        var reply = request.receiveHead(&.{}) catch return error.Network;
        var buffer: [8192]u8 = undefined;
        var decompressor: std.http.Decompress = undefined;
        const reader = try transport.bodyReader(a, &reply, &buffer, &decompressor);
        const bytes = reader.allocRemaining(a, .limited(64 * 1024)) catch return error.Network;
        return @import("../../core/http.zig").decode(a, .aster, @backingInt(reply.head.status), bytes);
    }

    pub fn event(self: *State, a: p.A, raw: []const u8) !Event {
        const v = try w.parse(a, raw);
        if (p.get(v, "code") != .null) return error.ExchangeApi;
        const kind = try p.text(v, "e");
        switch (self.channel) {
            .agg_trade, .order_book => {
                if (!p.eq(u8, kind, if (self.channel == .agg_trade) "aggTrade" else "depthUpdate")) return .skip;
                if (!std.ascii.eqlIgnoreCase(self.symbol, try p.text(v, "s"))) return .skip;
            },
            .account => {
                if (p.eq(u8, kind, "listenKeyExpired")) {
                    self.expired.store(true, .release);
                    return error.SequenceGap;
                }
                if (!p.eq(u8, kind, "ORDER_TRADE_UPDATE") and !p.eq(u8, kind, "ACCOUNT_UPDATE") and
                    !p.eq(u8, kind, "ACCOUNT_CONFIG_UPDATE") and !p.eq(u8, kind, "MARGIN_CALL")) return .skip;
            },
        }
        return .{ .deliver = raw };
    }
};

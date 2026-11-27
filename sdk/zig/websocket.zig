const std = @import("std");
const core = @import("aux_core");
const rt = @import("runtime.zig");
const Session = core.websocket.Session;
const Allocator = std.mem.Allocator;

pub const PreparedSubscription = core.websocket.PreparedSubscription;
pub const qfex = core.qfex_stream;
pub const deribit = core.deribit_stream;
pub const gains = core.gains_stream;

fn endpointOverride(options: rt.Options) ?[]const u8 {
    if (options.endpoint) |endpoint| return endpoint;
    if (options.origin) |origin| {
        if (std.mem.startsWith(u8, origin, "ws://") or std.mem.startsWith(u8, origin, "wss://")) return origin;
    }
    return null;
}

pub fn prepareQfex(client: rt.Client, spec: qfex.SubscriptionSpec) !rt.Owned(PreparedSubscription) {
    var arena = std.heap.ArenaAllocator.init(client.allocator);
    errdefer arena.deinit();
    const value = try qfex.prepare(arena.allocator(), endpointOverride(client.options), spec);
    return .{ .arena = arena, .value = value };
}

pub fn prepareDeribit(client: rt.Client, spec: deribit.SubscriptionSpec) !rt.Owned(PreparedSubscription) {
    var arena = std.heap.ArenaAllocator.init(client.allocator);
    errdefer arena.deinit();
    const value = try deribit.prepare(arena.allocator(), endpointOverride(client.options), client.options.auth orelse .null, spec);
    return .{ .arena = arena, .value = value };
}

pub fn prepare(client: rt.Client, comptime venue: core.subscription_prepare.Venue, spec: anytype) !rt.Owned(PreparedSubscription) {
    var arena = std.heap.ArenaAllocator.init(client.allocator);
    errdefer arena.deinit();
    const a = arena.allocator();
    const stringify = @import("signing.zig").value;
    const auth = client.options.auth orelse .null;
    const network = if (auth == .object) auth.object.get("network") else null;
    const chain_id = if (venue == .lighter and auth == .object) auth.object.get("chain_id") else null;
    const origin = if (venue == .monaco) client.options.endpoint orelse client.options.origin else endpointOverride(client.options);
    const config = try stringify(a, .{ .origin = origin, .auth = .{ .network = network, .chain_id = chain_id } });
    const result = try core.subscription_prepare.prepare(a, venue, config, try stringify(a, spec));
    return .{ .arena = arena, .value = result };
}

/// One physical WebSocket. A single reader may run concurrently with send or cancel.
/// Join all users before deinit.
pub const RawWebSocket = struct {
    session: ?*Session,
    allocator: Allocator,
    receive_index: u64 = 0,

    pub const Options = struct { dial: ?rt.Dial = null, headers: []const std.http.Header = &.{} };
    pub const Kind = core.websocket.MessageKind;
    pub const Message = struct {
        kind: Kind,
        payload: []u8,
        receive_index: u64,
    };

    pub fn connect(allocator: Allocator, endpoint: []const u8, options: Options) !RawWebSocket {
        for (options.headers) |header| try validateHeader(header);
        return .{ .session = try Session.dedicatedHeaders(endpoint, options.headers, options.dial), .allocator = allocator };
    }

    fn validateHeader(header: std.http.Header) !void {
        if (header.name.len == 0) return error.InvalidHeader;
        for (header.name) |c| {
            if (!std.ascii.isAlphanumeric(c) and std.mem.findScalar(u8, "!#$%&'*+-.^_~", c) == null) return error.InvalidHeader;
        }
        for (header.value) |c| {
            if ((c < 32 and c != '\t') or c == 127) return error.InvalidHeader;
        }
        for ([_][]const u8{ "host", "connection", "upgrade", "content-length", "transfer-encoding" }) |reserved| {
            if (std.ascii.eqlIgnoreCase(header.name, reserved)) return error.InvalidHeader;
        }
        if (std.ascii.startsWithIgnoreCase(header.name, "sec-websocket-")) return error.InvalidHeader;
    }

    pub fn send(self: *const RawWebSocket, kind: Kind, payload: []const u8) !void {
        const socket = self.session orelse return error.Closed;
        if (socket.canceled.load(.acquire)) return error.Closed;
        socket.sendData(kind, payload) catch |err| {
            if (err != error.FrameTooLarge) socket.close();
            return err;
        };
    }

    /// Reads one application message. Free its payload with the connect allocator.
    /// Clean close and cancellation return null. A read error ends the connection.
    pub fn next(self: *RawWebSocket) !?Message {
        const socket = self.session orelse return null;
        if (socket.canceled.load(.acquire)) return null;
        const payload = socket.rawMessage(self.allocator) catch |err| {
            if (socket.canceled.load(.acquire)) return null;
            socket.close();
            return err;
        } orelse {
            socket.close();
            return null;
        };
        if (socket.canceled.load(.acquire)) {
            self.allocator.free(payload);
            return null;
        }
        self.receive_index += 1;
        return .{
            .kind = if (socket.binary) .binary else .text,
            .payload = payload,
            .receive_index = self.receive_index,
        };
    }

    pub fn cancel(self: *const RawWebSocket) void {
        if (self.session) |socket| socket.close();
    }

    pub fn deinit(self: *RawWebSocket) void {
        if (self.session) |socket| socket.destroy();
        self.session = null;
    }
};

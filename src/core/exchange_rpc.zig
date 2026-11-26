const std = @import("std");
const ws = @import("exchange_stream.zig");
const http = @import("exchange_http.zig");
const p = @import("perps.zig");
const w = @import("wire.zig");
const Venue = @import("routes.zig").Venue;
const Dial = @import("host_stream.zig").Dial;
const A = w.A;
const V = w.V;
const eq = std.mem.eql;

fn inventory(v: Venue) []const u8 {
    return switch (v) {
        .aevo => @embedFile("../exchanges/aevo/rpc-routes.txt"),
        .binance_spot => @embedFile("../exchanges/binance_spot/rpc-routes.txt"),
        .bybit => @embedFile("../exchanges/bybit/rpc-routes.txt"),
        .deribit => @embedFile("../exchanges/deribit/rpc-routes.txt"),
        .hibachi => @embedFile("../exchanges/hibachi/rpc-routes.txt"),
        .kraken_spot => @embedFile("../exchanges/kraken_spot/rpc-routes.txt"),
        .okx => @embedFile("../exchanges/okx/rpc-routes.txt"),
        else => "",
    };
}

pub fn supports(v: Venue) bool {
    return inventory(v).len > 0;
}

fn method(v: Venue, name: []const u8) ![]const u8 {
    var lines = std.mem.tokenizeScalar(u8, inventory(v), '\n');
    while (lines.next()) |line| {
        const sep = std.mem.indexOfScalar(u8, line, ' ') orelse unreachable;
        if (eq(u8, name, line[0..sep])) return line[sep + 1 ..];
    }
    return error.InvalidOperation;
}

pub fn endpoint(a: A, v: Venue, input: V) ![]const u8 {
    const custom = p.get(input, "origin");
    const testnet = try p.testnet(input);
    const base = if (custom != .null) try w.string(custom) else switch (v) {
        .aevo => if (testnet) "wss://ws-testnet.aevo.xyz" else "wss://ws.aevo.xyz",
        .binance_spot => if (testnet) "wss://ws-api.testnet.binance.vision/ws-api/v3" else "wss://ws-api.binance.com/ws-api/v3",
        .bybit => if (testnet) "wss://stream-testnet.bybit.com/v5/trade" else "wss://stream.bybit.com/v5/trade",
        .deribit => if (testnet) "wss://test.deribit.com/ws/api/v2" else "wss://www.deribit.com/ws/api/v2",
        .hibachi => if (testnet) return error.InvalidNetwork else "wss://api.hibachi.xyz/ws/trade",
        .kraken_spot => if (testnet) return error.InvalidNetwork else "wss://ws-auth.kraken.com/v2",
        .okx => if (testnet) "wss://wspap.okx.com/ws/v5/private" else "wss://ws.okx.com/ws/v5/private",
        else => unreachable,
    };
    if (v != .hibachi) return base;
    const id = try ws.hibachiAccount(p.get(input, "params"), input);
    const joiner: u8 = if (std.mem.indexOfScalar(u8, base, '?') == null) '?' else '&';
    return std.fmt.allocPrint(a, "{s}{c}accountId={d}", .{ base, joiner, id });
}

pub fn requiresLogin(v: Venue) bool {
    return switch (v) {
        .aevo, .bybit, .deribit, .okx => true,
        else => false,
    };
}

pub const Encoded = struct { method: []const u8, frame: []const u8 };

pub fn prepare(a: A, v: Venue, input: V, token: ?[]const u8, op: []const u8, params: V, id: u64, io: std.Io, dial: ?Dial) !Encoded {
    const native = try method(v, op);
    const id_text = try std.fmt.allocPrint(a, "{d}", .{id});
    if (v == .okx and params == .array) {
        if (params.array.items.len == 0) return error.InvalidParameter;
    } else if (params != .object) return error.ExpectedObject;
    const frame = switch (v) {
        .aevo => blk: {
            const data = if (eq(u8, op, "place_order") or eq(u8, op, "amend_order")) try @import("../exchanges/aevo/trading.zig").sign(a, input, params, io) else params;
            break :blk try ws.stringify(a, .{ .id = id, .op = native, .data = data });
        },
        .binance_spot => try ws.stringify(a, .{ .id = id, .method = native, .params = try ws.binanceSigned(a, input, params, io) }),
        .bybit => blk: {
            const window = p.get(p.get(input, "auth"), "recv_window_ms");
            break :blk try ws.stringify(a, .{
                .reqId = id_text,
                .op = native,
                .header = .{
                    .@"X-BAPI-TIMESTAMP" = try http.timeText(a, io),
                    .@"X-BAPI-RECV-WINDOW" = if (window == .null) "5000" else try @import("http.zig").scalar(window),
                },
                .args = .{params},
            });
        },
        .deribit => try ws.stringify(a, .{ .jsonrpc = "2.0", .id = id, .method = native, .params = params }),
        .hibachi => blk: {
            var payload = V{ .object = try params.object.clone(a) };
            const id_value = try ws.hibachiAccount(p.get(input, "params"), input);
            try p.put(&payload, a, "accountId", .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{id_value}) });
            var signature: ?V = null;
            if (http.isWrite(v, op)) {
                var metadata: V = .null;
                if (eq(u8, op, "place_order") or eq(u8, op, "amend_order") or eq(u8, op, "place_orders")) {
                    const custom = p.get(p.get(input, "auth"), "market_origin");
                    var job = try @import("perps_connectors.zig").apiRequest(a, input, v, "markets", p.object(), custom);
                    _ = job.object.swapRemove("api_key");
                    _ = job.object.swapRemove("api_secret");
                    metadata = try @import("transport.zig").executeWithDial(a, job, dial);
                }
                payload = try @import("../exchanges/hibachi/trading.zig").sign(a, input, op, payload, metadata, io);
                if (payload.object.fetchSwapRemove("signature")) |item| signature = item.value;
            }
            break :blk try ws.stringify(a, .{ .id = id, .method = native, .params = payload, .signature = signature });
        },
        .kraken_spot => blk: {
            var payload = V{ .object = try params.object.clone(a) };
            try http.set(a, &payload, "token", token orelse return error.AuthenticationRequired);
            break :blk try ws.stringify(a, .{ .method = native, .req_id = id, .params = payload });
        },
        .okx => blk: {
            const data = if (params == .array) params else try p.array(a, &.{params});
            break :blk try ws.stringify(a, .{ .id = id_text, .op = native, .args = data });
        },
        else => unreachable,
    };
    return .{ .method = native, .frame = frame };
}

pub const Control = union(enum) { pass, skip, reply: []const u8 };

pub fn control(v: Venue, raw: []const u8, frame: V) Control {
    if (eq(u8, raw, "pong")) return .skip;
    if (eq(u8, raw, "ping")) return .{ .reply = "pong" };
    if (v == .deribit) {
        if (ws.same(frame, "method", "heartbeat")) {
            if (ws.same(p.get(frame, "params"), "type", "test_request"))
                return .{ .reply = "{\"jsonrpc\":\"2.0\",\"id\":0,\"method\":\"public/test\",\"params\":{}}" };
            return .skip;
        }
        const id = p.get(frame, "id");
        if (id != .null and (w.unsigned(u64, id) catch 1) == 0) return .skip;
    }
    if (v == .aevo and ws.same(frame, "data", "pong")) return .skip;
    if (v == .bybit and (ws.same(frame, "op", "pong") or ws.same(frame, "op", "ping"))) return .skip;
    if (v == .kraken_spot and (ws.same(frame, "channel", "heartbeat") or ws.same(frame, "channel", "status"))) return .skip;
    return .pass;
}

pub fn match(v: Venue, frame: V, id: u64, native: []const u8) !?bool {
    if (eq(u8, native, "aux_exchange_login")) return ws.acknowledged(v, frame, id);
    const key = switch (v) {
        .bybit => "reqId",
        .kraken_spot => "req_id",
        else => "id",
    };
    if ((w.unsigned(u64, p.get(frame, key)) catch 0) != id) return null;
    const fault = p.get(frame, "error");
    const success = p.get(frame, "success");
    const nested_success = p.get(p.get(frame, "data"), "success");
    return switch (v) {
        .aevo => if (fault != .null or ws.same(frame, "type", "error") or (success == .bool and !success.bool) or (nested_success == .bool and !nested_success.bool))
            false
        else if (success == .bool or frame.object.contains("data") or frame.object.contains("result"))
            true
        else
            error.PendingOutcome,
        .binance_spot, .hibachi => blk: {
            const status = w.unsigned(u16, p.get(frame, "status")) catch return error.PendingOutcome;
            if (status == 408 or status >= 500) return error.PendingOutcome;
            break :blk status >= 200 and status < 300 and fault == .null;
        },
        .deribit => if (fault != .null) false else if (frame.object.contains("result")) true else error.PendingOutcome,
        .bybit => blk: {
            const code = w.unsigned(u32, p.get(frame, "retCode")) catch return error.PendingOutcome;
            if (code == 10016) return error.PendingOutcome;
            break :blk code == 0;
        },
        .okx => if (p.get(frame, "code") == .string) ws.same(frame, "code", "0") or ws.same(frame, "code", "2") else error.PendingOutcome,
        .kraken_spot => if (success == .bool) success.bool else error.PendingOutcome,
        else => unreachable,
    };
}

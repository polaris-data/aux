// EVM read-only JSON-RPC. Wire quantities remain exact through every binding.
const std = @import("std");
const w = @import("wire.zig");
const transport = @import("transport.zig");
const A = w.A;
const V = w.V;
const Op = enum { chain_id, code, eth_call, logs, transaction_count, receipt, transaction, estimate_gas, block_number, block, native_balance, gas_price };
const eq = std.mem.eql;
var next_id = std.atomic.Value(u64).init(1);
fn op(v: V) !Op {
    return std.meta.stringToEnum(Op, try w.string(try w.field(v, "operation"))) orelse error.InvalidOperation;
}
pub fn endpoint(v: V) ![]const u8 {
    return checkedUrl(try w.string(try w.field(v, "endpoint")));
}
pub fn checkedUrl(value: []const u8) ![]const u8 {
    const url = value;
    for (url) |c| if (c <= 32 or c == 127 or c == '\\') return error.InvalidEndpoint;
    const uri = try std.Uri.parse(url);
    if ((!eq(u8, uri.scheme, "http") and !eq(u8, uri.scheme, "https")) or uri.host == null or uri.user != null or uri.password != null or uri.fragment != null) return error.InvalidEndpoint;
    return url;
}
fn digits(v: V) ![]const u8 {
    const s = try w.string(v);
    if (!std.mem.startsWith(u8, s, "0x")) return error.InvalidHex;
    for (s[2..]) |c| if (!std.ascii.isHex(c)) return error.InvalidHex;
    return s[2..];
}
fn bytes(v: V, size: ?usize) !V {
    const hex = try digits(v);
    if (hex.len % 2 != 0 or (size != null and hex.len != size.? * 2)) return error.InvalidHex;
    return v;
}
pub fn quantity(comptime T: type, v: V) !T {
    const hex = try digits(v);
    if (hex.len == 0 or (hex.len > 1 and hex[0] == '0')) return error.InvalidQuantity;
    return std.fmt.parseInt(T, hex, 16);
}
fn block(p: V) !V {
    const v = try w.field(p, "block");
    const tag = try w.string(v);
    for ([_][]const u8{ "latest", "earliest", "pending", "safe", "finalized" }) |valid| if (eq(u8, tag, valid)) return v;
    _ = try quantity(u256, v);
    return v;
}
fn object(p: V, name: []const u8) !V {
    const v = try w.field(p, name);
    if (v != .object) return error.InvalidObject;
    return v;
}
pub fn prepare(a: A, v: V) !V {
    const url = try endpoint(v);
    const operation = try op(v);
    const p = try w.field(v, "params");
    if (p != .object) return error.InvalidParams;
    var args: std.array_list.Managed(V) = .init(a);
    const method: []const u8 = switch (operation) {
        .chain_id => "eth_chainId",
        .block_number => "eth_blockNumber",
        .gas_price => "eth_gasPrice",
        .code, .native_balance, .transaction_count => blk: {
            try args.append(try bytes(try w.field(p, "address"), 20));
            try args.append(if (operation == .transaction_count) w.text("pending") else try block(p));
            break :blk switch (operation) {
                .code => "eth_getCode",
                .native_balance => "eth_getBalance",
                else => "eth_getTransactionCount",
            };
        },
        .receipt, .transaction => blk: {
            try args.append(try bytes(try w.field(p, "hash"), 32));
            break :blk if (operation == .receipt) "eth_getTransactionReceipt" else "eth_getTransactionByHash";
        },
        .block => blk: {
            try args.append(try block(p));
            try args.append(.{ .bool = false });
            break :blk "eth_getBlockByNumber";
        },
        .logs => blk: {
            try args.append(try object(p, "filter"));
            break :blk "eth_getLogs";
        },
        .estimate_gas => blk: {
            try args.append(try object(p, "request"));
            break :blk "eth_estimateGas";
        },
        .eth_call => blk: {
            var call: V = .{ .object = .empty };
            try call.object.put(a, "to", try bytes(try w.field(p, "to"), 20));
            try call.object.put(a, "data", try bytes(try w.field(p, "data"), null));
            if (p.object.get("from")) |from| if (from != .null) {
                try call.object.put(a, "from", try bytes(from, 20));
            };
            const value = p.object.get("value") orelse w.text("0x0");
            _ = try quantity(u256, value);
            try call.object.put(a, "value", value);
            try args.append(call);
            try args.append(try block(p));
            break :blk "eth_call";
        },
    };
    return envelope(a, url, method, .{ .array = args });
}
pub fn envelope(a: A, url: []const u8, method: []const u8, args: V) !V {
    const id = next_id.fetchAdd(1, .monotonic);
    var prepared: V = .{ .object = .empty };
    try prepared.object.put(a, "id", w.text(try std.fmt.allocPrint(a, "{d}", .{id})));
    try prepared.object.put(a, "url", w.text(url));
    const body = try std.json.Stringify.valueAlloc(a, .{ .jsonrpc = "2.0", .id = id, .method = method, .params = args }, .{});
    try prepared.object.put(a, "body", w.text(body));
    return prepared;
}
fn resultValue(a: A, id: []const u8, status: u16, body: []const u8) !V {
    if (status < 200 or status >= 300) return error.ExchangeApi;
    if (body.len > 16 * 1024 * 1024) return error.BodyTooLarge;
    const v = try w.parse(a, body);
    if (!eq(u8, try w.string(try w.field(v, "jsonrpc")), "2.0")) return error.InvalidResponse;
    const reply_id = try w.field(v, "id");
    if (reply_id != .number_string or !eq(u8, reply_id.number_string, id)) return error.InvalidResponse;
    if (v.object.contains("error")) return error.ExchangeApi;
    return w.field(v, "result");
}
fn convert(a: A, operation: Op, result: V) !V {
    switch (operation) {
        .code, .eth_call => return bytes(result, null),
        .chain_id, .block_number, .transaction_count, .estimate_gas => return .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{try quantity(u64, result)}) },
        .native_balance, .gas_price => return .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{try quantity(u256, result)}) },
        .logs => if (result != .array) return error.InvalidResponse,
        .receipt, .transaction, .block => if (result != .null and result != .object) return error.InvalidResponse,
    }
    return result;
}
pub fn finish(a: A, v: V) !V {
    const status = try w.field(v, "status");
    if (status != .number_string) return error.InvalidStatus;
    return convert(a, try op(v), try resultValue(a, try w.string(try w.field(v, "id")), try std.fmt.parseInt(u16, status.number_string, 10), try w.string(try w.field(v, "body"))));
}
pub fn execute(a: A, v: V) !V {
    return convert(a, try op(v), try send(a, try prepare(a, v), null));
}
pub fn send(a: A, prepared: V, attempted: ?*bool) !V {
    var io: std.Io.Threaded = .init_single_threaded;
    var client: std.http.Client = .{ .io = io.io(), .allocator = std.heap.c_allocator };
    defer client.deinit();
    var headers: V = .{ .object = .empty };
    try headers.object.put(a, "content-type", w.text("application/json"));
    var request = try transport.sendTracked(a, &client, "POST", try w.string(try w.field(prepared, "url")), headers, try w.string(try w.field(prepared, "body")), attempted);
    defer request.deinit();
    var response = request.receiveHead(&.{}) catch return error.Network;
    var buf: [8192]u8 = undefined;
    var decompressor: std.http.Decompress = undefined;
    const reader = try transport.bodyReader(a, &response, &buf, &decompressor);
    const body = reader.allocRemaining(a, .limited(16 * 1024 * 1024)) catch return error.Network;
    return resultValue(a, try w.string(try w.field(prepared, "id")), @backingInt(response.head.status), body);
}

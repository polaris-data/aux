const std = @import("std");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const transport = @import("../../core/transport.zig");
const proto = @import("proto.zig");
const schema = @import("stream_schema.zig");
const allocator = std.heap.c_allocator;

const Channel = enum { trades, order_book, orders, positions, fills };
const service = "/injective_derivative_exchange_rpc.InjectiveDerivativeExchangeRPC/";
const max_message = 8 * 1024 * 1024;

pub const Subscription = struct {
    channel: Channel,
    market: ?[]const u8,
    account: ?[]const u8,

    pub fn init(params: p.V) !Subscription {
        const channel = std.meta.stringToEnum(Channel, try p.text(params, "channel")) orelse return error.InvalidSubscription;
        const market = p.get(params, "market_id");
        const account = p.get(params, "subaccount_id");
        if ((channel == .order_book or channel == .trades) and market == .null) return error.InvalidSubscription;
        if ((channel == .orders or channel == .positions or channel == .fills) and account == .null) return error.InvalidSubscription;
        const evm = @import("../../core/evm.zig");
        if (market != .null) _ = try evm.hex(32, try w.string(market));
        if (account != .null) _ = try evm.hex(32, try w.string(account));
        return .{ .channel = channel, .market = if (market != .null) market.string else null, .account = if (account != .null) account.string else null };
    }

    fn rpc(sub: Subscription) []const u8 {
        return switch (sub.channel) {
            .trades, .fills => "StreamTradesV2",
            .order_book => "StreamOrderbookV2",
            .orders => "StreamOrders",
            .positions => "StreamPositionsV2",
        };
    }

    pub fn open(sub: Subscription, a: p.A, config: p.V, client: *std.http.Client) !std.http.Client.Request {
        const configured = p.get(config, "origin");
        const base = if (configured == .null) try p.origin(.injective, "subscribe", config, "https://k8s.mainnet.exchange.grpc-web.injective.network") else try w.string(configured);
        const uri = try std.Uri.parse(base);
        if (uri.host == null or uri.user != null or uri.password != null or uri.query != null or uri.fragment != null or
            !(p.eq(u8, uri.scheme, "http") or p.eq(u8, uri.scheme, "https"))) return error.InvalidOrigin;
        var payload = proto.Encoder{ .allocator = a };
        if (sub.market) |market| try payload.data(if (sub.channel == .positions) 2 else 1, market);
        if (sub.account) |account| {
            const field: u32 = switch (sub.channel) {
                .positions => 1,
                .orders => 3,
                .trades, .fills => 4,
                .order_book => return error.InvalidSubscription,
            };
            try payload.data(field, account);
        }
        var frame: [5]u8 = @splat(0);
        std.mem.writeInt(u32, frame[1..], @intCast(payload.slice().len), .big);
        const bytes = try std.mem.concat(a, u8, &.{ &frame, payload.slice() });
        const b64 = std.base64.standard.Encoder;
        const encoded = b64.encode(try a.alloc(u8, b64.calcSize(bytes.len)), bytes);
        const headers = try p.toValue(a, .{ .@"content-type" = "application/grpc-web-text+proto", .accept = "application/grpc-web-text+proto", .@"x-grpc-web" = "1" });
        const url = try std.mem.concat(a, u8, &.{ std.mem.trimEnd(u8, base, "/"), service, sub.rpc() });
        return transport.sendRequest(a, client, "POST", url, headers, encoded);
    }

    fn fields(sub: Subscription) []const schema.Field {
        return switch (sub.channel) {
            .order_book => &schema.StreamOrderbookV2Response,
            .trades, .fills => &schema.StreamTradesV2Response,
            .orders => &schema.StreamOrdersResponse,
            .positions => &schema.StreamPositionsV2Response,
        };
    }

    fn matches(sub: Subscription, event: p.V) !bool {
        const value = switch (sub.channel) {
            .order_book => event,
            .trades, .fills => try w.field(event, "trade"),
            .orders => try w.field(event, "order"),
            .positions => try w.field(event, "position"),
        };
        if (sub.market) |market| if (!std.ascii.eqlIgnoreCase(market, try p.text(value, "market_id"))) return false;
        if (sub.account) |account| if (!std.ascii.eqlIgnoreCase(account, try p.text(value, "subaccount_id"))) return false;
        return true;
    }
};

const DecodeType = enum { data, trailer };

pub const Decoder = struct {
    sub: Subscription,
    bytes: [3]u8 = undefined,
    offset: usize = 0,
    count: usize = 0,
    rejection: ?[]u8 = null,

    fn byte(self: *Decoder, reader: *std.Io.Reader) !u8 {
        if (self.offset == self.count) {
            var input: [4]u8 = undefined;
            for (&input) |*char| {
                char.* = reader.takeByte() catch |err| switch (err) {
                    error.EndOfStream => return if (char == &input[0]) error.EndOfStream else error.Truncated,
                    else => return error.Network,
                };
            }
            self.count = std.base64.standard.Decoder.calcSizeForSlice(&input) catch return error.InvalidGrpcFrame;
            std.base64.standard.Decoder.decode(self.bytes[0..self.count], &input) catch return error.InvalidGrpcFrame;
            self.offset = 0;
        }
        const result = self.bytes[self.offset];
        self.offset += 1;
        return result;
    }

    fn consume(self: *Decoder, reader: *std.Io.Reader, bytes: []u8) !void {
        for (bytes) |*item| item.* = self.byte(reader) catch |err| return if (err == error.EndOfStream) error.Truncated else err;
    }

    fn finish(self: *Decoder, a: p.A, status: u16, message: []const u8) !?[]u8 {
        if (status == 0) return null;
        if (status == 14 or status == 4) return error.Network;
        if (status == 16 or status == 7) return error.AuthenticationRequired;
        self.rejection = try std.json.Stringify.valueAlloc(a, .{ .venue = "injective", .body = .{ .grpc_status = status, .grpc_message = message } }, .{});
        return error.ExchangeApi;
    }

    fn trailer(self: *Decoder, a: p.A, bytes: []const u8) !?[]u8 {
        var lines = std.mem.tokenizeAny(u8, bytes, "\r\n");
        var code: ?u16 = null;
        var message: []const u8 = "";
        while (lines.next()) |line| {
            const split = std.mem.findScalar(u8, line, ':') orelse return error.InvalidGrpcFrame;
            const key = line[0..split];
            const value = std.mem.trim(u8, line[split + 1 ..], " \t");
            if (std.ascii.eqlIgnoreCase(key, "grpc-status")) {
                if (code != null) return error.InvalidGrpcFrame;
                code = std.fmt.parseInt(u16, value, 10) catch return error.InvalidGrpcFrame;
            }
            if (std.ascii.eqlIgnoreCase(key, "grpc-message")) message = value;
        }
        return self.finish(a, code orelse return error.InvalidGrpcFrame, message);
    }

    pub fn responseHead(self: *Decoder, a: p.A, head: std.http.Client.Response.Head) !bool {
        var code: ?u16 = null;
        var message: []const u8 = "";
        var content_type = false;
        var headers = head.iterateHeaders();
        while (headers.next()) |header| {
            if (std.ascii.eqlIgnoreCase(header.name, "grpc-status")) {
                code = std.fmt.parseInt(u16, header.value, 10) catch return error.InvalidGrpcFrame;
            } else if (std.ascii.eqlIgnoreCase(header.name, "grpc-message")) {
                message = header.value;
            } else if (std.ascii.eqlIgnoreCase(header.name, "content-type")) {
                content_type = std.mem.startsWith(u8, header.value, "application/grpc-web-text");
            }
        }
        if (code) |status| {
            _ = self.finish(a, status, message) catch |err| {
                if (err == error.ExchangeApi) {
                    const capture = @import("../../core/http_failure.zig").current;
                    if (capture) |target| target.value = try target.allocator.dupe(u8, self.rejection.?);
                }
                return err;
            };
            return true;
        }
        if (!content_type) return error.InvalidGrpcFrame;
        return false;
    }

    pub fn next(self: *Decoder, reader: *std.Io.Reader, arena_allocator: p.A) !?[]u8 {
        while (true) {
            var arena = std.heap.ArenaAllocator.init(allocator);
            defer arena.deinit();
            const a = arena.allocator();
            const flags = self.byte(reader) catch |err| return if (err == error.EndOfStream) error.Network else err;
            const kind: DecodeType = switch (flags) {
                0 => .data,
                0x80 => .trailer,
                else => return error.InvalidGrpcFrame,
            };
            var size: [4]u8 = undefined;
            try self.consume(reader, &size);
            const length = std.mem.readInt(u32, &size, .big);
            if (length > max_message) return error.RecordTooLarge;
            const bytes = try a.alloc(u8, length);
            try self.consume(reader, bytes);
            if (kind == .trailer) return self.trailer(arena_allocator, bytes);
            const event = try decode(a, self.sub.fields(), bytes);
            if (try self.sub.matches(event)) return try std.json.Stringify.valueAlloc(allocator, event, .{});
        }
    }
};

fn decode(a: p.A, fields: []const schema.Field, bytes: []const u8) anyerror!p.V {
    var reader = proto.Reader{ .bytes = bytes };
    var result = p.object();
    for (fields) |field| {
        if (field.repeated) try p.put(&result, a, field.name, try p.array(a, &.{}));
    }
    while (try reader.next()) |wire_field| {
        for (fields) |field| {
            if (field.number != wire_field.number) continue;
            const wire_value = wire_field.value;
            const value: p.V = switch (field.kind) {
                .text, .message => blk: {
                    if (wire_value != .data) return error.InvalidProtobuf;
                    if (field.kind == .message) break :blk try decode(a, field.fields, wire_value.data);
                    if (!std.unicode.utf8ValidateSlice(wire_value.data)) return error.InvalidProtobuf;
                    break :blk w.text(wire_value.data);
                },
                .sint, .uint, .boolean => blk: {
                    if (wire_value != .integer) return error.InvalidProtobuf;
                    const number = wire_value.integer;
                    break :blk switch (field.kind) {
                        .boolean => .{ .bool = number != 0 },
                        .uint => .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{number}) },
                        .sint => .{ .integer = @as(i64, @intCast(number >> 1)) ^ -@as(i64, @intCast(number & 1)) },
                        else => unreachable,
                    };
                },
            };
            if (field.repeated) {
                try result.object.getPtr(field.name).?.array.append(value);
            } else try p.put(&result, a, field.name, value);
            break;
        }
    }
    return result;
}

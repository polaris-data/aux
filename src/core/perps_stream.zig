const std = @import("std");
const w = @import("wire.zig");
const p = @import("perps.zig");
const connectors = @import("perps_connectors.zig");
const Venue = @import("routes.zig").Venue;
const Dial = @import("host_stream.zig").Dial;
pub const Event = @import("../exchanges/derive/stream.zig").State.Event;
const gpa = std.heap.c_allocator;
const perpl = @import("../exchanges/perpl/stream.zig");
const risex = @import("../exchanges/risex/stream.zig");
const decibel = @import("../exchanges/decibel/stream.zig");
const edgex = @import("../exchanges/edgex/stream.zig");
const grvt = @import("../exchanges/grvt/stream.zig");
const katana = @import("../exchanges/katana/stream.zig");
const eq = std.mem.eql;

pub fn venue(name: []const u8) ?Venue {
    const parsed = std.meta.stringToEnum(Venue, name) orelse return null;
    return if (connectors.supports(parsed)) parsed else null;
}

pub const Opening = struct {
    url: ?std.Uri = null,
    frame: []const u8,
    protocol: ?[]const u8 = null,
};

pub const State = struct {
    kind: Venue,
    config: p.V,
    endpoint: []const u8,
    channel: []const u8,
    request: []const u8 = "",
    selector: []const u8 = "",
    market: ?[]const u8 = null,
    account: ?[]const u8 = null,
    ids: []const u64 = &.{},
    private: bool = false,
    book: bool = false,
    ready: bool = false,
    snapshot: bool = false,
    sequence: ?u64 = null,
    subscription_id: ?u64 = null,
    rise_books: std.AutoHashMapUnmanaged(u64, void) = .empty,
    pending_snapshot: ?[]u8 = null,

    pub fn init(a: p.A, kind: Venue, config: p.V, params: p.V) !State {
        const channel = try p.text(params, "channel");
        var self: State = .{ .kind = kind, .config = config, .endpoint = undefined, .channel = channel };
        const testnet = try p.testnet(config);
        switch (kind) {
            .perpl => try perpl.init(&self, a, testnet, params),
            .risex => try risex.init(&self, a, testnet, params),
            .decibel => try decibel.init(&self, a, testnet, params),
            .edgex => try edgex.init(&self, a, testnet, params),
            .grvt => try grvt.init(&self, a, testnet, params),
            .katana => try katana.init(&self, a, testnet, params),
            else => unreachable,
        }
        return self;
    }

    pub fn reset(self: *State) void {
        self.ready = false;
        self.snapshot = false;
        self.sequence = null;
        self.subscription_id = null;
        self.rise_books.clearRetainingCapacity();
        if (self.pending_snapshot) |data| gpa.free(data);
        self.pending_snapshot = null;
    }

    pub fn deinit(self: *State) void {
        self.reset();
        self.rise_books.deinit(gpa);
    }

    pub fn start(self: *State, a: p.A, io: std.Io, dial: ?Dial, url: std.Uri, headers: *std.ArrayList(std.http.Header)) !Opening {
        self.reset();
        var opening: Opening = .{ .frame = self.request };
        var fields = p.object();
        switch (self.kind) {
            .perpl => if (self.private) {
                opening.frame = try @import("../exchanges/perpl/trading.zig").signIn(a, self.config, io);
            },
            .risex => if (self.private) {
                opening.frame = try @import("../exchanges/risex/trading.zig").signIn(a, self.config, dial);
            },
            .decibel => {
                const key = try p.credential(p.get(self.config, "api_key"));
                for (key) |c| {
                    if (!(std.ascii.isAlphanumeric(c) or std.mem.findScalar(u8, "!#$%&'*+-.^_`|~", c) != null)) return error.InvalidHeader;
                }
                opening.protocol = "decibel";
                const value = try std.mem.concat(a, u8, &.{ "decibel, ", key });
                try headers.append(a, .{ .name = "Sec-WebSocket-Protocol", .value = value });
            },
            .edgex => if (self.private) {
                const id = try w.unsigned(u64, p.get(p.get(self.config, "auth"), "account_id"));
                const timestamp = try std.fmt.allocPrint(a, "{d}", .{try p.epoch(io, .ms)});
                const query = try std.fmt.allocPrint(a, "accountId={d}&timestamp={s}", .{ id, timestamp });
                if (url.query != null) return error.InvalidEndpoint;
                var private_url = url;
                private_url.query = .{ .percent_encoded = query };
                opening.url = private_url;
                const target = try std.mem.concat(a, u8, &.{ try url.path.toRawMaybeAlloc(a), "?", query });
                try @import("../exchanges/edgex/trading.zig").headers(a, self.config, "GET", target, .null, timestamp, &fields);
            } else {
                opening.frame = "";
            },
            .grvt => if (self.private) {
                try @import("../exchanges/grvt/trading.zig").sessionHeaders(a, self.config, &fields);
            },
            .katana => if (self.private) {
                const token = try connectors.lookup(a, self.config, .katana, "ws_token", .{ .wallet = self.account.? }, dial);
                opening.frame = try std.json.Stringify.valueAlloc(a, .{ .method = "subscribe", .token = try p.text(token, "token"), .subscriptions = .{self.channel} }, .{});
            },
            else => unreachable,
        }
        for (fields.object.keys(), fields.object.values()) |name, value| try headers.append(a, .{ .name = name, .value = try w.string(value) });
        return opening;
    }

    pub fn connected(self: *State, a: p.A, dial: ?Dial) !void {
        if (self.kind != .katana or !self.book) return;
        const book = try connectors.lookup(a, self.config, .katana, "order_book", .{ .market = self.market.?, .level = 2, .limit = 0 }, dial);
        self.sequence = try w.unsigned(u64, try w.field(book, "sequence"));
        self.pending_snapshot = try std.json.Stringify.valueAlloc(gpa, .{ .type = "l2orderbook", .snapshot = true, .data = book }, .{});
    }

    pub fn event(self: *State, a: p.A, raw: []const u8) !Event {
        const message = try w.parse(a, raw);
        if (p.get(message, "error") != .null) return error.ExchangeApi;
        return switch (self.kind) {
            .perpl => perpl.event(self, message, raw),
            .risex => risex.event(self, message, raw),
            .decibel => decibel.event(self, message, raw),
            .edgex => edgex.event(self, a, message, raw),
            .grvt => grvt.event(self, message, raw),
            .katana => katana.event(self, message, raw),
            else => unreachable,
        };
    }
};

pub fn same(value: p.V, field: []const u8, expected: []const u8) bool {
    const actual = p.get(value, field);
    return actual == .string and eq(u8, actual.string, expected);
}

pub fn sameScalar(value: p.V, field: []const u8, expected: []const u8) bool {
    const actual = p.get(value, field);
    return switch (actual) {
        .string => |text| eq(u8, text, expected),
        .number_string => |text| eq(u8, text, expected),
        else => false,
    };
}

pub fn optional(comptime T: type, specification: p.V, name: []const u8, default: T) !T {
    const value = p.get(specification, name);
    return if (value == .null) default else w.unsigned(T, value);
}

pub fn jsonPrefix(a: p.A, frame: []const u8) ![]const u8 {
    var scanner = std.json.Scanner.initCompleteInput(a, frame);
    defer scanner.deinit();
    if (try scanner.next() != .object_begin) return error.InvalidResponse;
    var depth: usize = 1;
    while (depth > 0) {
        switch (try scanner.next()) {
            .object_begin, .array_begin => {
                depth += 1;
                if (depth >= 128) return error.TooDeep;
            },
            .object_end, .array_end => depth -= 1,
            .end_of_document => return error.InvalidResponse,
            else => {},
        }
    }
    return frame[0..scanner.cursor];
}

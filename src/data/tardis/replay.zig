const std = @import("std");
const w = @import("../../core/wire.zig");
const http = @import("../../core/http.zig");
const A = w.A;
const V = w.V;
const eq = std.mem.eql;
const List = std.ArrayList(u8);

fn required(p: V, k: []const u8) ![]const u8 {
    const s = try w.string(try w.field(p, k));
    if (s.len == 0) return error.InvalidParameter;
    return s;
}
fn segment(a: A, s: []const u8) ![]const u8 {
    if (eq(u8, s, ".") or eq(u8, s, "..")) return error.InvalidPath;
    return http.escape(a, s, false);
}
fn digits(s: []const u8) !u32 {
    for (s) |c| if (!std.ascii.isDigit(c)) return error.InvalidDate;
    return std.fmt.parseInt(u32, s, 10);
}
pub fn minute(s: []const u8) !u64 {
    if (s.len < 10 or s[4] != '-' or s[7] != '-') return error.InvalidDate;
    const y = try digits(s[0..4]);
    const m = try digits(s[5..7]);
    const d = try digits(s[8..10]);
    if (m == 0 or m > 12 or d == 0 or d > std.time.epoch.getDaysInMonth(@intCast(y), @fromBackingInt(@intCast(m)))) return error.InvalidDate;
    var days: u64 = 365 * @as(u64, y) + (y + 3) / 4 - (y + 99) / 100 + (y + 399) / 400;
    for (1..m) |month| days += std.time.epoch.getDaysInMonth(@intCast(y), @fromBackingInt(@intCast(month)));
    days += d - 1;
    if (s.len == 10) return days * 1440;
    if (s.len < 20 or std.ascii.toUpper(s[10]) != 'T' or s[13] != ':' or s[16] != ':' or !eq(u8, s[17..19], "00")) return error.InvalidDate;
    const hh = try digits(s[11..13]);
    const mm = try digits(s[14..16]);
    if (hh > 23 or mm > 59) return error.InvalidDate;
    var zone = s[19..];
    if (zone[0] == '.') {
        var i: usize = 1;
        while (i < zone.len and zone[i] == '0') i += 1;
        if (i == 1) return error.InvalidDate;
        zone = zone[i..];
    }
    if (!std.ascii.eqlIgnoreCase(zone, "Z") and !eq(u8, zone, "+00:00") and !eq(u8, zone, "-00:00")) return error.InvalidDate;
    return days * 1440 + hh * 60 + mm;
}
pub fn feedPath(a: A, p: V) ![]const u8 {
    const exchange = try segment(a, try required(p, "exchange"));
    var q: List = .empty;
    const filters = p.object.get("filters") orelse V{ .array = .init(a) };
    if (filters != .array) return error.InvalidFilters;
    var clean = V{ .array = .init(a) };
    for (filters.array.items) |f| {
        var item = V{ .object = .empty };
        try item.object.put(a, "channel", w.text(try required(f, "channel")));
        if (f.object.get("symbols")) |symbols| {
            if (symbols != .array or symbols.array.items.len > 50) return error.InvalidFilters;
            for (symbols.array.items) |symbol| _ = try w.string(symbol);
            if (symbols.array.items.len != 0) try item.object.put(a, "symbols", symbols);
        }
        try clean.array.append(item);
    }
    if (clean.array.items.len != 0) try http.pair(a, &q, "filters", try std.json.Stringify.valueAlloc(a, clean, .{}), true);
    const slice = if (p.object.get("sliceSize")) |v| try w.unsigned(u8, v) else 1;
    if (slice == 0 or slice > 10 or (slice > 1 and clean.array.items.len == 0)) return error.InvalidSlice;
    const offset = if (p.object.get("offset")) |v| try w.unsigned(u64, v) else 0;
    try http.pair(a, &q, "from", try required(p, "from"), true);
    try http.pair(a, &q, "offset", try std.fmt.allocPrint(a, "{d}", .{offset}), true);
    try http.pair(a, &q, "sliceSize", try std.fmt.allocPrint(a, "{d}", .{slice}), true);
    return std.mem.concat(a, u8, &.{ "/v1/data-feeds/", exchange, q.items });
}
pub fn prepare(a: A, input: V, name: []const u8, p: V) !http.Request {
    var headers = V{ .object = .empty };
    try headers.object.put(a, "accept", w.text("application/json"));
    var url: []const u8 = "https://api.tardis.dev";
    const path = if (eq(u8, name, "data_feeds")) try feedPath(a, p) else if (eq(u8, name, "dataset")) path: {
        const date = try required(p, "date");
        if (date.len != 10) return error.InvalidDate;
        _ = try minute(date);
        if (p.object.get("range")) |r| if (r != .null) {
            const range = try w.string(r);
            if (!std.mem.startsWith(u8, range, "bytes=")) return error.InvalidRange;
            const dash = std.mem.findScalar(u8, range[6..], '-') orelse return error.InvalidRange;
            const start = range[6..][0..dash];
            const end = range[7 + dash ..];
            const lo = if (start.len != 0) try w.unsigned(u64, w.text(start)) else 0;
            if (end.len != 0) {
                if (try w.unsigned(u64, w.text(end)) < lo) return error.InvalidRange;
            } else if (start.len == 0) return error.InvalidRange;
            try headers.object.put(a, "range", r);
        };
        url = "https://datasets.tardis.dev";
        break :path try std.fmt.allocPrint(a, "/v1/{s}/{s}/{s}/{s}/{s}/{s}.csv.gz", .{ try segment(a, try required(p, "exchange")), try segment(a, try required(p, "data_type")), date[0..4], date[5..7], date[8..10], try segment(a, try required(p, "symbol")) });
    } else path: {
        if (!input.object.contains("origin")) return error.InvalidEndpoint;
        const opts = try w.field(p, "options");
        if (opts != .object and opts != .array) return error.InvalidOptions;
        var q: List = .empty;
        try http.pair(a, &q, "options", try std.json.Stringify.valueAlloc(a, opts, .{}), true);
        break :path try std.mem.concat(a, u8, &.{ if (eq(u8, name, "machine_replay")) "/replay" else "/replay-normalized", q.items });
    };
    return .{ .method = "GET", .path = path, .headers = headers, .body = .null, .authenticated = false, .origin = url };
}
pub fn line(a: A, raw: []const u8, hosted: bool) ![]u8 {
    const data = std.mem.trimEnd(u8, raw, "\r");
    if (data.len == 0) return a.dupe(u8, "{\"type\":\"disconnect\"}");
    if (!hosted) {
        _ = try w.parse(a, data);
        return a.dupe(u8, data);
    }
    const split = std.mem.findScalar(u8, data, ' ') orelse return error.InvalidEvent;
    if (!std.unicode.utf8ValidateSlice(data[0..split])) return error.InvalidEvent;
    return std.json.Stringify.valueAlloc(a, .{ .localTimestamp = data[0..split], .message = try w.parse(a, data[split + 1 ..]) }, .{});
}
test "UTC replay boundaries include leap days and reject partial minutes" {
    try std.testing.expectEqual(@as(u64, 1441), try minute("2024-03-01T00:01:00Z") - try minute("2024-02-29"));
    try std.testing.expectError(error.InvalidDate, minute("2023-02-29"));
    try std.testing.expectError(error.InvalidDate, minute("2024-01-01T00:00:00.01Z"));
}

const Socket = @import("../../core/websocket.zig").Session;
const gpa = std.heap.c_allocator;
pub const Machine = struct {
    ws: *Socket,
    done: bool = false,
    batch_error: ?anyerror = null,
    pub fn open(a: A, config: V, op: []const u8, p: V, dial: ?@import("../../core/host_stream.zig").Dial) !*Machine {
        const base = try required(config, "origin");
        if (std.mem.findAny(u8, base, "?#") != null) return error.InvalidEndpoint;
        const sep = std.mem.find(u8, base, "://") orelse return error.InvalidEndpoint;
        const scheme = base[0..sep];
        const ws_scheme = if (eq(u8, scheme, "https") or eq(u8, scheme, "wss")) "wss" else if (eq(u8, scheme, "http") or eq(u8, scheme, "ws")) "ws" else return error.InvalidEndpoint;
        var q: List = .empty;
        var messages: V = .null;
        const path = if (eq(u8, op, "ws_replay")) path: {
            for ([_]([]const u8){ "exchange", "from", "session", "to" }) |k| {
                if (eq(u8, k, "session") and (p.object.get(k) orelse .null) == .null) continue;
                try http.pair(a, &q, k, try required(p, k), true);
            }
            messages = p.object.get("subscribe_messages") orelse .null;
            if (messages != .null and messages != .array) return error.InvalidSubscription;
            break :path "/ws-replay";
        } else path: {
            const opts = try w.field(p, "options");
            if (opts != .object and opts != .array) return error.InvalidOptions;
            try http.pair(a, &q, "options", try std.json.Stringify.valueAlloc(a, opts, .{}), true);
            break :path if (eq(u8, op, "ws_replay_normalized")) "/ws-replay-normalized" else "/ws-stream-normalized";
        };
        const url = try std.mem.concat(a, u8, &.{ ws_scheme, std.mem.trimEnd(u8, base[sep..], "/"), path, q.items });
        const self = try gpa.create(Machine);
        errdefer gpa.destroy(self);
        self.* = .{ .ws = try Socket.dedicated(url, dial) };
        errdefer self.ws.destroy();
        if (messages == .array) for (messages.array.items) |msg| try self.ws.sendText(try std.json.Stringify.valueAlloc(a, msg, .{}));
        return self;
    }
    pub fn next(self: *Machine) !?[]u8 {
        if (self.batch_error) |e| {
            self.batch_error = null;
            return e;
        }
        if (self.done or self.ws.canceled.load(.acquire)) return null;
        errdefer {
            self.done = true;
            self.ws.close();
        }
        const bytes = try self.ws.message() orelse {
            self.done = true;
            return null;
        };
        errdefer gpa.free(bytes);
        var scratch = std.heap.ArenaAllocator.init(gpa);
        defer scratch.deinit();
        _ = try w.parse(scratch.allocator(), bytes);
        return bytes;
    }
    pub fn command(self: *Machine, a: A, input: V) !V {
        if (eq(u8, try required(input, "operation"), "send")) {
            if (self.done) return error.Closed;
            try self.ws.sendText(try std.json.Stringify.valueAlloc(a, try w.field(input, "message"), .{}));
            return .null;
        }
        return @import("../../core/stream.zig").batchNext(self, a, input);
    }
    pub fn destroy(self: *Machine) void {
        self.ws.destroy();
        gpa.destroy(self);
    }
};

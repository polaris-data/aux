// Pull one DBN event at a time. The compressed input is owned for the cursor's lifetime.
const std = @import("std");
const w = @import("wire.zig");
const gpa = std.heap.c_allocator;
const zstd = std.compress.zstd;
const schemas = [_][]const u8{ "mbo", "mbp-1", "mbp-10", "tbbo", "trades", "ohlcv-1s", "ohlcv-1m", "ohlcv-1h", "ohlcv-1d", "definition", "statistics", "status", "imbalance", "ohlcv-eod", "cmbp-1", "cbbo-1s", "cbbo-1m", "tcbbo", "bbo-1s", "bbo-1m" };
const stypes = [_][]const u8{ "instrument_id", "raw_symbol", "smart", "continuous", "parent", "nasdaq_symbol", "cms_symbol", "isin", "us_code", "bbg_comp_id", "bbg_comp_ticker", "figi", "figi_ticker", "listing_id", "issuer_id", "security_id" };
fn name(table: []const []const u8, n: usize) !w.V {
    if (n >= table.len) return error.InvalidMetadata;
    return w.text(table[n]);
}
fn date(reader: *std.Io.Reader) !w.V {
    const n = try reader.takeInt(u32, .little);
    const y = n / 10000;
    const m = n / 100 % 100;
    const d = n % 100;
    if (y == 0 or y > 9999 or m == 0 or m > 12) return error.InvalidDate;
    const days: u8 = switch (m) {
        2 => if (y % 4 == 0 and (y % 100 != 0 or y % 400 == 0)) @as(u8, 29) else 28,
        4, 6, 9, 11 => 30,
        else => 31,
    };
    if (d == 0 or d > days) return error.InvalidDate;
    return .{ .integer = n };
}
fn symbol(reader: *std.Io.Reader, width: usize) !w.V {
    const data = std.mem.trimEnd(u8, try reader.take(width), "\x00");
    if (!std.unicode.utf8ValidateSlice(data)) return error.InvalidUtf8;
    return w.text(data);
}
fn count(reader: *std.Io.Reader, entry: usize) !u32 {
    const n = try reader.takeInt(u32, .little);
    if (entry == 0 or n > reader.bufferedLen() / entry or n > 65536) return error.InvalidCount;
    return n;
}
fn decimal(a: w.A, n: u64) !w.V {
    return w.text(try std.fmt.allocPrint(a, "{d}", .{n}));
}
fn metadata(a: w.A, version: u8, bytes: []const u8) !w.V {
    var r = std.Io.Reader.fixed(bytes);
    var out: w.V = .{ .object = .empty };
    try out.object.put(a, "version", .{ .integer = 3 });
    try out.object.put(a, "dataset", try symbol(&r, 16));
    const schema = try r.takeInt(u16, .little);
    try out.object.put(a, "schema", if (schema == 65535) .null else try name(&schemas, schema));
    try out.object.put(a, "start", try decimal(a, try r.takeInt(u64, .little)));
    const end = try r.takeInt(u64, .little);
    try out.object.put(a, "end", if (end == 0 or end == std.math.maxInt(u64)) .null else try decimal(a, end));
    const limit = try r.takeInt(u64, .little);
    try out.object.put(a, "limit", if (limit == 0) .null else try decimal(a, limit));
    if (version == 1) try r.discardAll(8); // Deprecated record count.
    const in_type = try r.takeByte();
    try out.object.put(a, "stype_in", if (in_type == 255) .null else try name(&stypes, in_type));
    try out.object.put(a, "stype_out", try name(&stypes, try r.takeByte()));
    try out.object.put(a, "ts_out", .{ .bool = try r.takeByte() != 0 });
    const width: u16 = if (version == 1) 22 else try r.takeInt(u16, .little);
    if (width == 0) return error.InvalidMetadata;
    try out.object.put(a, "symbol_cstr_len", .{ .integer = if (version == 1) 71 else width });
    try r.discardAll(if (version == 1) 47 else 53);
    if (try r.takeInt(u32, .little) != 0) return error.InvalidMetadata;
    for ([_][]const u8{ "symbols", "partial", "not_found" }) |key| {
        var list = std.array_list.Managed(w.V).init(a);
        for (0..try count(&r, width)) |_| try list.append(try symbol(&r, width));
        try out.object.put(a, key, .{ .array = list });
    }
    var mappings = std.array_list.Managed(w.V).init(a);
    for (0..try count(&r, @as(usize, width) + 4)) |_| {
        var row: w.V = .{ .object = .empty };
        try row.object.put(a, "raw_symbol", try symbol(&r, width));
        var intervals = std.array_list.Managed(w.V).init(a);
        for (0..try count(&r, @as(usize, width) + 8)) |_| {
            var item: w.V = .{ .object = .empty };
            try item.object.put(a, "start_date", try date(&r));
            try item.object.put(a, "end_date", try date(&r));
            try item.object.put(a, "symbol", try symbol(&r, width));
            try intervals.append(item);
        }
        try row.object.put(a, "intervals", .{ .array = intervals });
        try mappings.append(row);
    }
    try out.object.put(a, "mappings", .{ .array = mappings });
    // The encoder may pad the declared metadata up to an eight-byte boundary.
    const tail = r.buffered();
    if (tail.len > 7 or std.mem.trimEnd(u8, tail, "\x00").len != 0) return error.InvalidMetadata;
    return out;
}
pub const File = struct {
    arena: std.heap.ArenaAllocator,
    input: std.Io.Reader,
    decoder: zstd.Decompress,
    reader: *std.Io.Reader,
    cursor: Cursor = .{},
    canceled: std.atomic.Value(bool) = .init(false),

    pub fn open(input: []const u8) !*File {
        const self = try gpa.create(File);
        errdefer gpa.destroy(self);
        self.* = .{ .arena = .init(gpa), .input = undefined, .decoder = undefined, .reader = undefined };
        errdefer self.arena.deinit();
        const a = self.arena.allocator();
        self.input = .fixed(try a.dupe(u8, input));
        if (std.mem.startsWith(u8, input, "\x28\xb5\x2f\xfd")) {
            self.decoder = zstd.Decompress.init(&self.input, try a.alloc(u8, zstd.default_window_len + zstd.block_size_max), .{});
            self.reader = &self.decoder.reader;
        } else self.reader = &self.input;
        return self;
    }
    pub fn next(self: *File) !?[]u8 {
        if (self.canceled.load(.acquire)) return null;
        errdefer self.canceled.store(true, .release);
        return self.cursor.next(self.reader);
    }
    pub fn destroy(self: *File) void {
        self.arena.deinit();
        gpa.destroy(self);
    }
};
pub const Cursor = struct {
    first: bool = true,
    version: u8 = 3,
    ts_out: bool = false,

    pub fn next(self: *Cursor, reader: *std.Io.Reader) !?[]u8 {
        var scratch = std.heap.ArenaAllocator.init(gpa);
        defer scratch.deinit();
        const a = scratch.allocator();
        if (self.first) {
            self.first = false;
            var prefix: [4]u8 = undefined;
            try reader.readSliceAll(&prefix);
            if (!std.mem.eql(u8, prefix[0..3], "DBN") or prefix[3] < 1 or prefix[3] > 3) return error.InvalidVersion;
            self.version = prefix[3];
            const n = try reader.takeInt(u32, .little);
            if (n > 16 * 1024 * 1024 or n < 120) return error.InvalidMetadata;
            const bytes = try a.alloc(u8, n);
            try reader.readSliceAll(bytes);
            const meta = try metadata(a, self.version, bytes);
            self.ts_out = (try w.field(meta, "ts_out")).bool;
            return try std.json.Stringify.valueAlloc(gpa, .{ .type = "metadata", .metadata = meta }, .{});
        }
        const words = reader.peekByte() catch |err| return if (err == error.EndOfStream) null else err;
        const n = @as(usize, words) * 4;
        if (n < 16) return error.InvalidRecord;
        var record: [1020]u8 = undefined;
        try reader.readSliceAll(record[0..n]);
        return try std.json.Stringify.valueAlloc(gpa, try @import("dbn_legacy.zig").decode(a, self.version, record[0..n], self.ts_out), .{});
    }
};

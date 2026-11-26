// KXMD v1 market frames from the Kuru Exchange Gateway.
const std = @import("std");
const Subscription = @import("stream.zig").Subscription;
const subscription_id = @import("stream.zig").subscription_id;
const A = std.mem.Allocator;

const Reader = struct {
    rest: []const u8,

    fn bytes(self: *Reader, comptime size: usize) !*const [size]u8 {
        if (self.rest.len < size) return error.InvalidFrame;
        const value = self.rest[0..size];
        self.rest = self.rest[size..];
        return value;
    }

    fn int(self: *Reader, comptime T: type) !T {
        return std.mem.readInt(T, try self.bytes(@sizeOf(T)), .big);
    }

    fn side(self: *Reader) ![]const u8 {
        return switch (try self.int(u8)) {
            1 => "buy",
            2 => "sell",
            else => error.InvalidFrame,
        };
    }

    fn records(self: *Reader, count: usize, comptime width: usize) !void {
        if (self.rest.len % width != 0 or count != self.rest.len / width) return error.InvalidFrame;
    }
};

const Block = struct {
    blockNumber: u64,
    blockId: []const u8,

    fn read(a: A, reader: *Reader) !Block {
        return .{
            .blockNumber = try reader.int(u64),
            .blockId = try std.fmt.allocPrint(a, "0x{x}", .{try reader.bytes(32)}),
        };
    }
};

const Delta = struct {
    side: []const u8,
    price: i64,
    totalBaseAfter: u128,
    activeBaseAfter: u128,
    passiveBaseAfter: u128,
    activeOrderCountAfter: u32,

    fn read(reader: *Reader) !Delta {
        return .{
            .side = try reader.side(),
            .price = try reader.int(i64),
            .totalBaseAfter = try reader.int(u128),
            .activeBaseAfter = try reader.int(u128),
            .passiveBaseAfter = try reader.int(u128),
            .activeOrderCountAfter = try reader.int(u32),
        };
    }
};

const Trade = struct {
    tradeId: u64,
    recordIndex: u16,
    takerSide: []const u8,
    price: i64,
    baseFilled: u128,
    blockTimestamp: u64,

    fn read(reader: *Reader) !Trade {
        return .{
            .tradeId = try reader.int(u64),
            .recordIndex = try reader.int(u16),
            .takerSide = try reader.side(),
            .price = try reader.int(i64),
            .baseFilled = try reader.int(u128),
            .blockTimestamp = try reader.int(u64),
        };
    }
};

const CompactLevel = struct {
    price: i128,
    totalBase: u128,

    fn read(reader: *Reader) !CompactLevel {
        return .{ .price = try reader.int(i128), .totalBase = try reader.int(u128) };
    }
};

const ExtendedLevel = struct {
    price: i128,
    totalBase: u128,
    activeBase: u128,
    passiveBase: u128,
    activeOrderCount: u32,

    fn read(reader: *Reader) !ExtendedLevel {
        return .{
            .price = try reader.int(i128),
            .totalBase = try reader.int(u128),
            .activeBase = try reader.int(u128),
            .passiveBase = try reader.int(u128),
            .activeOrderCount = try reader.int(u32),
        };
    }
};

const Grouping = union(enum) {
    none,
    nativeTick: u64,
    significantFigures: struct { figures: u8, mantissa: ?u8 },

    fn read(reader: *Reader) !Grouping {
        const code = try reader.int(u8);
        const tick = try reader.int(u64);
        const figures = try reader.int(u8);
        const mantissa = try reader.int(u8);
        if (try reader.int(u16) != 0) return error.InvalidFrame;
        return switch (code) {
            0 => if (tick == 0 and figures == 0 and mantissa == 0) .none else error.InvalidFrame,
            1 => if (figures == 0 and mantissa == 0) .{ .nativeTick = tick } else error.InvalidFrame,
            2 => if (tick == 0) .{ .significantFigures = .{ .figures = figures, .mantissa = if (mantissa == 0) null else mantissa } } else error.InvalidFrame,
            else => error.InvalidFrame,
        };
    }

    pub fn jsonStringify(self: Grouping, writer: *std.json.Stringify) !void {
        switch (self) {
            .none => try writer.write(.{ .kind = "none" }),
            .nativeTick => |tick| try writer.write(.{ .kind = "nativeTick", .tickSize = tick }),
            .significantFigures => |g| try writer.write(.{ .kind = "significantFigures", .figures = g.figures, .mantissa = g.mantissa }),
        }
    }
};

const Market = struct {
    epoch: u64,
    address: []const u8,
    seq: u64,
};

fn readRows(a: A, reader: *Reader, count: usize, comptime T: type) ![]T {
    const rows = try a.alloc(T, count);
    for (rows) |*row| row.* = try T.read(reader);
    return rows;
}

fn envelope(a: A, message: anytype) ![]u8 {
    return std.json.Stringify.valueAlloc(a, .{ .id = @as(u64, subscription_id), .message = message }, .{});
}

fn snapshot(a: A, reader: *Reader, market: Market, comptime extended: bool) ![]u8 {
    const global_seq = try reader.int(u64);
    const state_head = try Block.read(a, reader);
    const duration = try reader.int(u64);
    const depth = try reader.int(u32);
    const grouping = try Grouping.read(reader);
    const bid_count: usize = try reader.int(u32);
    const ask_count: usize = try reader.int(u32);
    const Level = if (extended) ExtendedLevel else CompactLevel;
    const count = std.math.add(usize, bid_count, ask_count) catch return error.InvalidFrame;
    try reader.records(count, if (extended) 68 else 32);
    const levels = try readRows(a, reader, count, Level);
    return envelope(a, .{
        .wireVersion = 1,
        .feedEpoch = market.epoch,
        .kind = "l2Book",
        .view = "finalized",
        .marketAddress = market.address,
        .marketSeq = market.seq,
        .globalSeq = global_seq,
        .stateHead = state_head,
        .durationMs = duration,
        .maxDepthPerSide = depth,
        .grouping = grouping,
        .levelFormat = if (extended) "extended" else "compact",
        .bids = levels[0..bid_count],
        .asks = levels[bid_count..],
    });
}

fn replayable(a: A, reader: *Reader, market: Market, comptime book: bool) ![]u8 {
    const previous = try reader.int(u64);
    const global_seq = try reader.int(u64);
    const source = try Block.read(a, reader);
    const count = try reader.int(u32);
    try reader.records(count, if (book) 61 else 43);
    const rows = try readRows(a, reader, count, if (book) Delta else Trade);
    return envelope(a, .{
        .wireVersion = 1,
        .feedEpoch = market.epoch,
        .kind = if (book) "l2Delta" else "trades",
        .view = "finalized",
        .marketAddress = market.address,
        .marketSeq = market.seq,
        .previousMarketSeq = if (previous == 0) @as(?u64, null) else previous,
        .globalSeq = global_seq,
        .sourceBlock = source,
        .updates = if (book) rows else {},
        .trades = if (book) {} else rows,
    });
}

pub fn decode(a: A, state: Subscription, raw: []const u8) !?[]u8 {
    var reader = Reader{ .rest = raw };
    if (try reader.int(u64) != subscription_id) return null;
    if (!std.mem.eql(u8, try reader.bytes(4), "KXMD") or try reader.int(u8) != 1) return error.InvalidFrame;
    const kind = try reader.int(u8);
    if (try reader.int(u8) != 3) return error.InvalidFrame;
    const flags = try reader.int(u8);
    const epoch = try reader.int(u64);
    if (state.epoch == null or state.epoch.? != epoch) return error.InvalidFrame;
    if (!std.mem.eql(u8, try reader.bytes(20), &state.address)) return error.InvalidFrame;
    const market = Market{
        .epoch = epoch,
        .address = try std.fmt.allocPrint(a, "0x{x}", .{&state.address}),
        .seq = try reader.int(u64),
    };
    if (state.book and kind == 1) return switch (flags) {
        1 => try snapshot(a, &reader, market, false),
        2 => try snapshot(a, &reader, market, true),
        else => error.InvalidFrame,
    };
    if (flags != 0) return error.InvalidFrame;
    if (state.book and kind == 2) return try replayable(a, &reader, market, true);
    if (!state.book and kind == 3) return try replayable(a, &reader, market, false);
    return error.InvalidFrame;
}

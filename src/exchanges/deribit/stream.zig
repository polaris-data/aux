const std = @import("std");
const core = @import("../../core/exchange_stream.zig");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const websocket = @import("../../core/websocket.zig");
const State = core.State;
const eq = std.mem.eql;
const in = core.in;
const same = core.same;
const optionalText = core.optionalText;
const requiredText = core.requiredText;
const optionalDepth = core.optionalDepth;
const stringify = core.stringify;
const default_interval = "100ms";
const batch_size = 500;

pub const SubscriptionTopic = union(enum) {
    native: []const u8,
    market: struct {
        channel: enum { ticker, trades, book },
        instrument_name: []const u8,
        interval: []const u8 = default_interval,
        book: ?BookAggregation = null,
    },
};

const BookAggregation = struct { group: []const u8, depth: u32 = 10 };

pub const SubscriptionSpec = struct {
    topics: []const SubscriptionTopic,
    subscribe_id: u64,
    unsubscribe_id: u64,
};

pub fn prepare(a: w.A, override: ?[]const u8, auth: w.V, spec: SubscriptionSpec) !websocket.PreparedSubscription {
    const testnet = try p.testnetAuth(auth);
    const url = override orelse endpoint(testnet);
    _ = try websocket.Session.parseEndpoint(url);
    if (spec.topics.len == 0) return error.EmptySubscription;

    const topics = try a.alloc([]const u8, spec.topics.len);
    for (spec.topics, topics) |topic, *name| {
        name.* = switch (topic) {
            .native => |native| native,
            .market => |market| blk: {
                if (!validTopicText(market.instrument_name) or !validTopicText(market.interval)) return error.InvalidSubscription;
                if (market.book) |book| {
                    if (market.channel != .book or !validTopicText(book.group) or book.depth == 0) return error.InvalidSubscription;
                }
                break :blk try marketTopic(a, @tagName(market.channel), market.instrument_name, market.interval, market.book);
            },
        };
        if (!validTopicText(name.*)) return error.InvalidSubscription;
        if (isPrivate(name.*)) return error.AuthenticationRequired;
    }

    const count = std.math.divCeil(usize, topics.len, batch_size) catch unreachable;
    const subscribe = try a.alloc([]const u8, count);
    const unsubscribe = try a.alloc([]const u8, count);
    for (subscribe, unsubscribe, 0..) |*on, *off, i| {
        const start = i * batch_size;
        const batch = topics[start..][0..@min(batch_size, topics.len - start)];
        const on_id = std.math.add(u64, spec.subscribe_id, i) catch return error.InvalidRequestId;
        const off_id = std.math.add(u64, spec.unsubscribe_id, i) catch return error.InvalidRequestId;
        on.* = try subscriptionFrame(a, "public/subscribe", on_id, batch);
        off.* = try subscriptionFrame(a, "public/unsubscribe", off_id, batch);
    }
    return .{ .endpoint = try a.dupe(u8, url), .subscribe = subscribe, .unsubscribe = unsubscribe };
}

fn validTopicText(text: []const u8) bool {
    return text.len > 0 and std.mem.findAny(u8, text, &std.ascii.whitespace) == null;
}

fn marketTopic(a: w.A, family: []const u8, instrument: []const u8, interval: []const u8, book: ?BookAggregation) ![]const u8 {
    if (book) |b| return std.fmt.allocPrint(a, "book.{s}.{s}.{d}.{s}", .{ instrument, b.group, b.depth, interval });
    return std.fmt.allocPrint(a, "{s}.{s}.{s}", .{ family, instrument, interval });
}

fn endpoint(testnet: bool) []const u8 {
    return if (testnet) "wss://test.deribit.com/ws/api/v2" else "wss://www.deribit.com/ws/api/v2";
}

fn isPrivate(topic: []const u8) bool {
    return std.mem.startsWith(u8, topic, "user.") or std.mem.endsWith(u8, topic, ".raw");
}

fn subscriptionFrame(a: w.A, method: []const u8, id: u64, topics: []const []const u8) ![]const u8 {
    return stringify(a, .{ .jsonrpc = "2.0", .id = id, .method = method, .params = .{ .channels = topics } });
}

pub fn init(s: *State, a: w.A, _: w.V, params: w.V, channel: []const u8, t: ?[]const u8, override: ?[]const u8, testnet: bool) !void {
    s.endpoint = override orelse endpoint(testnet);
    const name = try optionalText(params, "instrument_name");
    const interval = (try optionalText(params, "interval")) orelse default_interval;
    const family = if (eq(u8, channel, "orders")) "user.orders" else if (eq(u8, channel, "fills")) "user.trades" else if (eq(u8, channel, "order_book")) "book" else channel;
    s.topic = t orelse if (in(family, &.{ "book", "ticker", "trades", "user.orders", "user.trades", "user.changes" })) topic: {
        const instrument = name orelse break :topic try std.fmt.allocPrint(a, "{s}.{s}.{s}.{s}", .{ family, (try optionalText(params, "kind")) orelse "any", try requiredText(params, "currency"), interval });
        const group = try optionalText(params, "group");
        const book: ?BookAggregation = if (group != null and eq(u8, family, "book")) .{ .group = group.?, .depth = try optionalDepth(params, 10) } else null;
        break :topic try marketTopic(a, family, instrument, interval, book);
    } else channel;
    s.private = isPrivate(s.topic);
    s.book = std.mem.startsWith(u8, s.topic, "book.");
    s.frame = try subscriptionFrame(a, if (s.private) "private/subscribe" else "public/subscribe", 1, &.{s.topic});
}

pub fn event(s: *State, _: w.A, frame: w.V) !core.Event {
    if (same(frame, "method", "subscription") and same(p.get(frame, "params"), "channel", s.topic)) return .deliver;
    return .skip;
}

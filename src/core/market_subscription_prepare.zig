const std = @import("std");
const wire = @import("wire.zig");
const p = @import("perps.zig");
const exchange = @import("exchange_stream.zig");
const socket = @import("websocket.zig");
const Venue = @import("subscription_prepare.zig").Venue;
const aevo = @import("../exchanges/aevo/stream.zig");
const apex = @import("../exchanges/apex_omni/stream.zig");
const arcus = @import("../exchanges/arcus/stream.zig");
const binance = @import("../exchanges/binance_spot/stream.zig");
const gatedex = @import("../exchanges/gatedex/stream.zig");
const gains = @import("../exchanges/gains/stream.zig");
const kuru = @import("../exchanges/kuru/stream.zig");
const monaco = @import("../exchanges/monaco/stream.zig");
const okx = @import("../exchanges/okx/stream.zig");
const perpl = @import("../exchanges/perpl/stream.zig");
const phoenix = @import("../exchanges/phoenix/stream.zig");
const risex = @import("../exchanges/risex/stream.zig");
const vest = @import("../exchanges/vest/stream.zig");
const eq = std.mem.eql;
const Plan = socket.PreparedSubscription;

pub fn prepare(a: wire.A, comptime venue: Venue, config: wire.V, spec: wire.V, items: []const wire.V) !Plan {
    const override = try exchange.optionalText(config, "origin");
    const plan = switch (venue) {
        .aevo, .apex_omni, .binance_spot => try exchangePlan(a, @field(@import("routes.zig").Venue, @tagName(venue)), config, spec, items),
        .arcus, .kuru, .phoenix => try individualPlan(a, venue, config, override, items),
        .gatedex => try gatedex.prepare(a, override, spec, items),
        .gains => try gains.prepare(a, override, items),
        .monaco => try monacoPlan(a, override, items),
        .okx => try okx.prepare(a, config, override, items),
        .perpl => try perplPlan(a, override orelse perpl.publicEndpoint(try p.testnet(config)), items),
        .risex => try risexPlan(a, override orelse risex.endpoint(try p.testnet(config)), items),
        .vest => try vest.prepare(a, override, items),
        else => unreachable,
    };
    _ = try socket.Session.parseEndpoint(plan.endpoint);
    return plan;
}

fn batchPlan(a: wire.A, endpoint: []const u8, subscribe: []const u8, unsubscribe: []const u8) !Plan {
    return .{
        .endpoint = endpoint,
        .subscribe = try a.dupe([]const u8, &.{subscribe}),
        .unsubscribe = try a.dupe([]const u8, &.{unsubscribe}),
    };
}

fn exchangePlan(a: wire.A, comptime venue: @import("routes.zig").Venue, config: wire.V, spec: wire.V, items: []const wire.V) !Plan {
    const topics = try a.alloc([]const u8, items.len);
    var endpoint: []const u8 = undefined;
    for (items, topics) |item, *topic| {
        const state = try exchange.State.init(a, venue, config, item);
        if (state.private) return error.AuthenticationRequired;
        const valid = switch (venue) {
            .aevo => if (std.mem.cutPrefix(u8, state.topic, "orderbook-500ms:")) |symbol| symbol.len != 0 else false,
            .apex_omni => std.mem.startsWith(u8, state.topic, "orderBook200."),
            .binance_spot => binance.isPreparedTopic(state.topic),
            else => unreachable,
        };
        if (!valid) return error.InvalidSubscription;
        endpoint = state.endpoint;
        topic.* = state.topic;
    }
    if (venue == .apex_omni and p.get(config, "origin") == .null) {
        const value = p.get(spec, "timestamp_ms");
        var clock: std.Io.Threaded = .init_single_threaded;
        const timestamp = if (value == .null) try p.epoch(clock.io(), .ms) else try wire.unsigned(u64, value);
        const uri = try apex.timestampedUrl(a, try std.Uri.parse(endpoint), timestamp);
        endpoint = try std.fmt.allocPrint(a, "{f}", .{uri});
    }
    const frame = switch (venue) {
        .aevo => aevo.subscriptionFrame,
        .apex_omni => apex.subscriptionFrame,
        .binance_spot => binance.subscriptionFrame,
        else => unreachable,
    };
    return batchPlan(a, endpoint, try frame(a, true, topics), try frame(a, false, topics));
}

fn individualPlan(a: wire.A, comptime venue: Venue, config: wire.V, override: ?[]const u8, items: []const wire.V) !Plan {
    var endpoint: []const u8 = switch (venue) {
        .arcus => override orelse arcus.endpoint,
        .phoenix => override orelse phoenix.endpoint,
        .kuru => "",
        else => unreachable,
    };
    const subscribe = try a.alloc([]const u8, items.len);
    const unsubscribe = try a.alloc([]const u8, items.len);
    for (items, subscribe, unsubscribe, 1..) |item, *sub, *unsub, id| {
        switch (venue) {
            .arcus => {
                const channel = try p.text(item, "channel");
                if (exchange.in(channel, &.{ "account", "positions", "orders", "userFills" })) return error.AuthenticationRequired;
                if (!eq(u8, channel, "l2OrderbookUpdates")) return error.InvalidSubscription;
                const state = try arcus.State.init(a, item);
                sub.* = state.frame;
                unsub.* = try state.unsubscribeFrame(a);
            },
            .kuru => {
                const state = try exchange.State.init(a, .kuru, config, item);
                endpoint = state.endpoint;
                sub.* = if (id == kuru.subscription_id) state.frame else try kuru.subscriptionFrame(a, state, id);
                unsub.* = try kuru.unsubscribeFrame(a, id);
            },
            .phoenix => {
                const channel = try p.text(item, "channel");
                if (exchange.in(channel, &.{ "notifications", "traderState" })) return error.AuthenticationRequired;
                if (!exchange.in(channel, &.{ "market", "orderbook" })) return error.InvalidSubscription;
                _ = try phoenix.State.init(item, false);
                const bypass = p.get(item, "bypassExecutionBand");
                if (bypass != .null and (!eq(u8, channel, "orderbook") or bypass != .bool)) return error.InvalidSubscription;
                sub.* = try phoenix.subscriptionFrame(a, true, item);
                unsub.* = try phoenix.subscriptionFrame(a, false, item);
            },
            else => unreachable,
        }
    }
    return .{ .endpoint = endpoint, .subscribe = subscribe, .unsubscribe = unsubscribe };
}

fn monacoPlan(a: wire.A, override: ?[]const u8, items: []const wire.V) !Plan {
    const topics = try a.alloc([]const u8, items.len);
    for (items, topics) |item, *topic| topic.* = try monaco.preparedTopic(a, item);
    return batchPlan(a, try monaco.publicEndpoint(a, override), try monaco.subscriptionFrame(a, true, topics), try monaco.subscriptionFrame(a, false, topics));
}

fn perplPlan(a: wire.A, endpoint: []const u8, items: []const wire.V) !Plan {
    if (items.len > 16) return error.InvalidSubscription;
    const subs = try a.alloc(perpl.Subscription, items.len);
    for (items, subs) |item, *sub| sub.* = .{ .stream = try perpl.publicSelector(a, item), .subscribe = true };
    const subscribe = try perpl.subscriptionFrame(a, subs);
    for (subs) |*sub| sub.subscribe = false;
    return batchPlan(a, endpoint, subscribe, try perpl.subscriptionFrame(a, subs));
}

fn risexPlan(a: wire.A, endpoint: []const u8, items: []const wire.V) !Plan {
    if (items.len != 1) return error.InvalidSubscription;
    const channel = try p.text(items[0], "channel");
    if (exchange.in(channel, &.{ "orders", "positions", "fills", "account" })) return error.AuthenticationRequired;
    if (!eq(u8, channel, "oracle")) return error.InvalidSubscription;
    const ids = try risex.marketIds(a, items[0]);
    return batchPlan(a, endpoint, try risex.subscriptionFrame(a, true, channel, ids), try risex.subscriptionFrame(a, false, channel, null));
}

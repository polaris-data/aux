const std = @import("std");
const wire = @import("wire.zig");
const socket = @import("websocket.zig");
const p = @import("perps.zig");
const pacifica = @import("../exchanges/pacifica/stream.zig");
const hibachi = @import("../exchanges/hibachi/stream.zig");
const hyperliquid = @import("../exchanges/hyperliquid/stream.zig");
const ondo = @import("../exchanges/ondo_perps/stream.zig");
const extended = @import("../exchanges/extended/stream.zig");
const paradex = @import("../exchanges/paradex/stream.zig");
const lighter = @import("../exchanges/lighter/stream.zig");
const polymarket = @import("../exchanges/polymarket/stream.zig");
const market = @import("market_subscription_prepare.zig");

pub const Venue = enum {
    pacifica,
    hibachi,
    hyperliquid,
    paragon,
    trade_xyz,
    ondo_perps,
    extended,
    paradex,
    lighter,
    polymarket_perps,
    aevo,
    apex_omni,
    arcus,
    binance_spot,
    gatedex,
    kuru,
    monaco,
    n1,
    okx,
    perpl,
    phoenix,
    risex,
    vest,
};

pub fn prepare(a: wire.A, comptime venue: Venue, config: wire.V, spec: wire.V) !socket.PreparedSubscription {
    if (venue == .extended) return extended.prepare(a, config, spec);
    if (venue == .n1) {
        const state = try @import("../exchanges/n1/stream.zig").State.init(a, try @import("exchange_stream.zig").optionalText(config, "origin"), spec);
        return .{ .endpoint = state.url, .subscribe = &.{}, .unsubscribe = &.{} };
    }

    const subscriptions = try wire.field(spec, if (venue == .polymarket_perps) "channels" else "subscriptions");
    if (subscriptions != .array) return error.InvalidSubscription;
    const items = subscriptions.array.items;
    if (items.len == 0) return error.EmptySubscription;

    switch (venue) {
        .aevo, .apex_omni, .arcus, .binance_spot, .gatedex, .kuru, .monaco, .okx, .perpl, .phoenix, .risex, .vest => return market.prepare(a, venue, config, spec, items),
        else => {},
    }

    const override = p.get(config, "origin");
    const endpoint = if (venue == .paradex)
        try paradex.preparedEndpoint(a, config, spec)
    else if (override != .null)
        try wire.string(override)
    else switch (venue) {
        .pacifica => try pacifica.endpoint(config),
        .hibachi => if (try p.testnet(config)) return error.InvalidNetwork else hibachi.market_endpoint,
        .hyperliquid, .paragon, .trade_xyz => hyperliquid.endpoint(config),
        .ondo_perps => try ondo.endpoint(config),
        .lighter => try lighter.preparedEndpoint(config),
        .polymarket_perps => polymarket.perps_endpoint,
        else => unreachable,
    };
    _ = try socket.Session.parseEndpoint(endpoint);

    if (venue == .hibachi or venue == .polymarket_perps) {
        const frames = if (venue == .hibachi) try hibachi.prepareFrames(a, items) else try polymarket.preparePerpsFrames(a, subscriptions);
        return .{ .endpoint = endpoint, .subscribe = try a.dupe([]const u8, &.{frames.subscribe}), .unsubscribe = try a.dupe([]const u8, &.{frames.unsubscribe}) };
    }
    const first_subscribe = if (venue == .paradex) try requestId(spec, "subscribe_id", 1) else 0;
    const first_unsubscribe = if (venue == .paradex) blk: {
        const id = p.get(spec, "unsubscribe_id");
        break :blk if (id == .null) try addId(first_subscribe, items.len) else try wire.unsigned(u64, id);
    } else 0;
    var subscribe: std.ArrayList([]const u8) = .empty;
    var unsubscribe: std.ArrayList([]const u8) = .empty;
    for (items, 0..) |item, index| {
        if (venue == .ondo_perps) {
            for (try ondo.prepareFrames(a, item)) |pair| {
                try subscribe.append(a, pair.subscribe);
                try unsubscribe.append(a, pair.unsubscribe);
            }
            continue;
        }
        const pair = switch (venue) {
            .pacifica => try pacifica.prepareFrames(a, config, item),
            .hyperliquid => try hyperliquid.prepareFrames(a, config, item, null),
            .paragon => try hyperliquid.prepareFrames(a, config, item, "para"),
            .trade_xyz => try hyperliquid.prepareFrames(a, config, item, "xyz"),
            .paradex => try paradex.prepareFrames(a, item, try addId(first_subscribe, index), try addId(first_unsubscribe, index)),
            .lighter => try lighter.prepareFrames(a, item),
            else => unreachable,
        };
        try subscribe.append(a, pair.subscribe);
        try unsubscribe.append(a, pair.unsubscribe);
    }
    return .{ .endpoint = endpoint, .subscribe = try subscribe.toOwnedSlice(a), .unsubscribe = try unsubscribe.toOwnedSlice(a) };
}

fn requestId(spec: wire.V, name: []const u8, default: u64) !u64 {
    const value = p.get(spec, name);
    return if (value == .null) default else try wire.unsigned(u64, value);
}

fn addId(first: u64, index: usize) !u64 {
    return std.math.add(u64, first, index) catch error.InvalidRequestId;
}

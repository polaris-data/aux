//! Prepare commands for an authenticated, JSON-only Paradex connection.
const std = @import("std");
const w = @import("../../core/wire.zig");
const Kind = enum { subscribe_channel, unsubscribe_channel, modify_subscription, set_cancel_on_disconnect, create_order, create_batch, modify_order, cancel_order, cancel_by_client_id, cancel_all, cancel_batch };
pub const Command = struct { method: []const u8, params: w.V };
fn arg(p: w.V, name: []const u8) ![]const u8 {
    return w.string(try w.field(p, name));
}
pub fn prepare(a: w.A, operation: []const u8, p: w.V, credentials: w.V, endpoint: []const u8, ms: u64) !Command {
    const kind = std.meta.stringToEnum(Kind, operation) orelse return error.InvalidOperation;
    var body = p;
    const method: []const u8 = switch (kind) {
        .subscribe_channel, .unsubscribe_channel => blk: {
            const spec = try @import("stream.zig").State.initRpc(a, p);
            if (kind == .subscribe_channel and !spec.requires_auth) return error.InvalidChannel;
            body = .{ .object = .empty };
            try body.object.put(a, "channel", w.text(spec.name));
            break :blk if (kind == .subscribe_channel) "subscribe" else "unsubscribe";
        },
        .modify_subscription => "modify_subscription",
        else => blk: {
            const ro = credentials.object.get("read_only") orelse return error.AuthenticationRequired;
            if (ro != .bool or ro.bool) return error.AuthenticationRequired;
            _ = try @import("../../core/venue_auth.zig").bearer(a, credentials, @intCast(ms / 1000));
            switch (kind) {
                .cancel_order => {
                    _ = try arg(p, "id");
                    break :blk "order.cancel";
                },
                .cancel_by_client_id => {
                    _ = try arg(p, "client_id");
                    _ = try arg(p, "market");
                    break :blk "order.cancel";
                },
                .cancel_all => {
                    if (p != .object) return error.InvalidParameter;
                    if (p.object.get("market")) |market| _ = try w.string(market);
                    break :blk "order.cancel_all";
                },
                .cancel_batch => {
                    const ids = try w.field(p, "order_ids");
                    if (ids != .array) return error.InvalidParameter;
                    for (ids.array.items) |id| _ = try w.string(id);
                    break :blk "order.cancel_batch";
                },
                .set_cancel_on_disconnect => {
                    if ((try w.field(p, "enabled")) != .bool) return error.InvalidParameter;
                    break :blk "order.cancel_on_disconnect";
                },
                .create_order, .create_batch, .modify_order => {
                    var ctx = w.V{ .object = .empty };
                    try ctx.object.put(a, "origin", w.text(endpoint));
                    const id = if (kind == .modify_order) try arg(p, "id") else "";
                    if (kind == .modify_order) body = try w.field(p, "body");
                    const path = try std.mem.concat(a, u8, &.{ "/v1/orders/", id });
                    const native: []const u8 = switch (kind) {
                        .create_order => "orders_new",
                        .create_batch => "orders_new_batch",
                        else => "orders_modify",
                    };
                    try @import("sign.zig").dispatch(a, native, ctx, path, &body, credentials, ms);
                    if (kind == .modify_order) try body.object.put(a, "id", w.text(id));
                    break :blk switch (kind) {
                        .create_order => "order.create",
                        .create_batch => "order.create_batch",
                        else => "order.modify",
                    };
                },
                else => unreachable,
            }
        },
    };
    return .{ .method = method, .params = body };
}

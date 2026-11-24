const std = @import("std");
const w = @import("wire.zig");
const eq = std.mem.eql;
const V = w.V;
const A = w.A;
const Kind = enum { auctions_watch, margin_watch, orderbook, spot_feed, balances, best_quotes, orders, quotes, account_trades, ticker_slim, trades, trades_by_type, rfqs };

fn component(v: V, name: []const u8) ![]const u8 {
    const value = try w.string(try w.field(v, name));
    if (value.len == 0) return error.InvalidChannel;
    for (value) |c| if (!std.ascii.isAlphanumeric(c) and c != '-' and c != '_') return error.InvalidChannel;
    return value;
}
fn number(v: V, key: []const u8) !u64 {
    const value = try w.field(v, key);
    if (value != .number_string) return error.InvalidChannel;
    for (value.number_string) |c| if (!std.ascii.isDigit(c)) return error.InvalidChannel;
    return std.fmt.parseInt(u64, value.number_string, 10);
}
pub const Channel = struct { name: []const u8, private: bool };
pub fn channel(a: A, v: V) !Channel {
    const kind = std.meta.stringToEnum(Kind, try w.string(try w.field(v, "type"))) orelse return error.InvalidChannel;
    const name: []const u8 = switch (kind) {
        .auctions_watch => "auctions.watch",
        .margin_watch => "margin.watch",
        .spot_feed => try std.fmt.allocPrint(a, "spot_feed.{s}", .{try component(v, "currency")}),
        .trades => try std.fmt.allocPrint(a, "trades.{s}", .{try component(v, "instrument_name")}),
        .orderbook => blk: {
            const group = try number(v, "group");
            const depth = try number(v, "depth");
            if ((group != 1 and group != 10 and group != 100) or (depth != 1 and depth != 10 and depth != 20 and depth != 100)) return error.InvalidChannel;
            break :blk try std.fmt.allocPrint(a, "orderbook.{s}.{d}.{d}", .{ try component(v, "instrument_name"), group, depth });
        },
        .ticker_slim => blk: {
            const interval = try number(v, "interval");
            if (interval != 100 and interval != 1000) return error.InvalidChannel;
            break :blk try std.fmt.allocPrint(a, "ticker_slim.{s}.{d}", .{ try component(v, "instrument_name"), interval });
        },
        .trades_by_type => blk: {
            const t = try component(v, "instrument_type");
            if (!eq(u8, t, "erc20") and !eq(u8, t, "option") and !eq(u8, t, "perp")) return error.InvalidChannel;
            break :blk try std.fmt.allocPrint(a, "trades.{s}.{s}", .{ t, try component(v, "currency") });
        },
        .rfqs => blk: {
            const wallet = try @import("evm.zig").fromHex(a, try w.string(try w.field(v, "wallet")));
            if (wallet.len != 20) return error.InvalidAddress;
            break :blk try std.fmt.allocPrint(a, "0x{x}.rfqs", .{wallet});
        },
        .balances, .best_quotes, .orders, .quotes, .account_trades => try std.fmt.allocPrint(a, "{d}.{s}", .{ try number(v, "subaccount_id"), switch (kind) {
            .account_trades => "trades",
            .best_quotes => "best.quotes",
            else => @tagName(kind),
        } }),
    };
    return .{ .name = name, .private = switch (kind) {
        .balances, .best_quotes, .orders, .quotes, .account_trades, .rfqs => true,
        else => false,
    } };
}
pub const State = struct {
    owner: ?[]const u8 = null,
    secret: ?[]const u8 = null,
    awaiting_login: bool = false,
    pub fn start(self: *State, a: A, io: std.Io, subscribe: []const u8) ![]const u8 {
        self.awaiting_login = self.secret != null;
        if (!self.awaiting_login) return subscribe;
        const ms = std.Io.Timestamp.now(io, .real).toMilliseconds();
        if (ms < 0) return error.InvalidTimestamp;
        const proof = try @import("derive.zig").login(a, self.owner.?, self.secret.?, @intCast(ms));
        return std.json.Stringify.valueAlloc(a, .{ .id = "aux-login", .method = "public/login", .params = proof }, .{});
    }
    pub const Event = union(enum) { skip, subscribe, reply: []const u8, deliver: []const u8 };
    pub fn event(self: *State, a: A, name: []const u8, raw: []const u8) !Event {
        if (eq(u8, raw, "pong")) return if (self.awaiting_login) .skip else .{ .deliver = "{\"method\":\"pong\"}" };
        const v = try w.parse(a, raw);
        if (v != .object) return error.InvalidEvent;
        const id = v.object.get("id");
        const result = v.object.get("result");
        const failed = (v.object.get("error") orelse V.null) != .null;
        if (self.awaiting_login) {
            if (id == null or id.? != .string or !eq(u8, id.?.string, "aux-login")) return .skip;
            if (failed or result == null or result.? != .array) return error.AuthenticationRequired;
            self.awaiting_login = false;
            return .subscribe;
        }
        if (id) |n| {
            if (n != .number_string or !eq(u8, n.number_string, "1")) return .skip;
        }
        if (failed) return error.ExchangeApi;
        if (result) |r| {
            if (id == null) return .skip;
            if (r == .object) if (r.object.get("status")) |status| {
                if (status != .object) return error.InvalidEvent;
                var it = status.object.iterator();
                while (it.next()) |entry| if (entry.value_ptr.* != .string or !eq(u8, entry.value_ptr.string, "ok")) return error.ExchangeApi;
            };
        } else {
            const p = try w.field(v, "params");
            if (!eq(u8, try w.string(try w.field(p, "channel")), name)) return .skip;
            _ = try w.field(p, "data");
        }
        return .{ .deliver = raw };
    }
};

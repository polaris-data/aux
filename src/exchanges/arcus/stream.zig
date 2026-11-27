const std = @import("std");
const w = @import("../../core/wire.zig");
const Event = @import("../derive/stream.zig").State.Event;
const Channel = enum { markets, l2Orderbook, l2OrderbookUpdates, trades, bbo, account, positions, orders, userFills };
pub const endpoint = "wss://api.arcus.xyz/v1/ws";

pub const State = struct {
    channel: Channel,
    id: ?[]const u8,
    account: ?u8,
    snapshot: bool = false,
    frame: []const u8,

    pub fn init(a: w.A, p: w.V) !State {
        const channel = std.meta.stringToEnum(Channel, try w.string(try w.field(p, "channel"))) orelse return error.InvalidSubscription;
        var frame: w.V = .{ .object = .empty };
        var it = p.object.iterator();
        while (it.next()) |field| {
            for ([_][]const u8{ "channel", "id", "accountIndex", "market", "nLevels" }) |name| {
                if (std.mem.eql(u8, name, field.key_ptr.*)) break;
            } else return error.UnknownField;
            if (field.value_ptr.* != .null) try frame.object.put(a, field.key_ptr.*, field.value_ptr.*);
        }
        const id = if (frame.object.get("id")) |v| try w.string(v) else null;
        const index = if (frame.object.get("accountIndex")) |v| try integer(v) else null;
        switch (channel) {
            .account, .positions, .orders, .userFills => {
                const address = id orelse return error.InvalidSubscription;
                if (address.len != 42 or !std.mem.startsWith(u8, address, "0x") or (index orelse 10) > 9) return error.InvalidSubscription;
                for (address[2..]) |c| if (!std.ascii.isHex(c) or std.ascii.isUpper(c)) return error.InvalidSubscription;
            },
            else => {
                if (index != null or (channel != .markets and std.mem.trim(u8, id orelse "", " \r\n\t").len == 0)) return error.InvalidSubscription;
            },
        }
        if (frame.object.get("market")) |v| {
            if (channel != .orders and channel != .positions and channel != .userFills) return error.InvalidSubscription;
            _ = try w.string(v);
        }
        if (frame.object.get("nLevels")) |v| {
            const levels = try integer(v);
            if (levels < 1 or levels > 100 or (channel != .l2Orderbook and channel != .l2OrderbookUpdates)) return error.InvalidSubscription;
        }
        try frame.object.put(a, "type", w.text("subscribe"));
        return .{ .channel = channel, .id = id, .account = index, .frame = try std.json.Stringify.valueAlloc(a, frame, .{}) };
    }

    fn integer(value: w.V) !u8 {
        if (value != .number_string) return error.InvalidInteger;
        return w.unsigned(u8, value);
    }

    pub fn unsubscribeFrame(self: State, a: w.A) ![]const u8 {
        return std.json.Stringify.valueAlloc(a, .{ .type = "unsubscribe", .channel = @tagName(self.channel), .id = self.id }, .{ .emit_null_optional_fields = false });
    }

    pub fn event(self: *State, a: w.A, raw: []const u8) !Event {
        const obj = try w.parse(a, raw);
        const kind = try w.string(try w.field(obj, "type"));
        if (std.mem.eql(u8, kind, "error")) return error.ExchangeApi;
        if (std.mem.eql(u8, kind, "connected")) return .{ .deliver = raw };
        const channel = obj.object.get("channel") orelse return .skip;
        if (channel != .string or !std.mem.eql(u8, channel.string, @tagName(self.channel))) return .skip;
        if (self.id) |id| {
            const got = obj.object.get("id") orelse return .skip;
            if (got != .string or !std.mem.eql(u8, got.string, id)) return .skip;
        }
        if (self.account) |index| {
            const got = obj.object.get("accountIndex") orelse return .skip;
            if ((integer(got) catch return .skip) != index) return .skip;
        }
        if (self.channel == .l2OrderbookUpdates and !self.snapshot) {
            const sub = std.mem.eql(u8, kind, "subscribed");
            if (sub) {
                if (obj.object.get("contents")) |data| {
                    if (data != .object or (data.object.get("bids") orelse .null) != .array or (data.object.get("asks") orelse .null) != .array) return .skip;
                    self.snapshot = true;
                }
            } else if (std.mem.eql(u8, kind, "channel_data")) return .skip;
        }
        return .{ .deliver = raw };
    }
};

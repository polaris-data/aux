// Map REST writes to the module data authenticated by Matching.
const std = @import("std");
const w = @import("wire.zig");
const derive = @import("derive.zig");
const A = w.A;
const V = w.V;
const eq = std.mem.eql;
const Route = enum {
    private_set_session_key,
    private_transfer_spot,
    private_transfer_spot_external,
    private_withdraw,
    private_execute_quote,
    private_send_quote,
    private_replace_quote,
    private_transfer_positions,
    private_order,
    private_replace,
    private_order_quote,
    private_liquidate,
    private_delete_subaccount,
    private_create_vault,
    private_request_vault_deposit,
    private_request_vault_withdraw,
    private_cancel_all_vault_requests,
    private_mint_vault_shares,
    private_burn_vault_shares,
    private_force_burn,
    private_update_whitelisted_recipients,
};
fn routeFor(name: []const u8) ?Route {
    // Debug endpoints accept the same signed envelope as their trading counterparts.
    return std.meta.stringToEnum(Route, if (std.mem.endsWith(u8, name, "_debug")) name[0 .. name.len - 6] else name);
}
pub fn supported(name: []const u8) bool {
    return routeFor(name) != null;
}
fn text(v: V, k: []const u8) ![]const u8 {
    return w.string(try w.field(v, k));
}
fn presigned(v: V) !bool {
    const sig = v.object.get("signature") orelse return false;
    if (sig == .null) return false;
    const s = try w.string(sig);
    if (s.len == 0) return false;
    // A caller-supplied signature travels with its original envelope.
    if (s.len != 132 or !std.mem.startsWith(u8, s, "0x")) return error.InvalidSignature;
    for (s[2..]) |c| if (!std.ascii.isHex(c)) return error.InvalidSignature;
    _ = try w.field(v, "nonce");
    _ = try w.field(v, "signature_expiry_sec");
    _ = try w.field(v, "signer");
    return true;
}
fn network(input: V) !V {
    const origin = if (input.object.get("origin")) |v| try w.string(v) else "https://api.derive.xyz";
    const uri = try std.Uri.parse(origin);
    const host = uri.host orelse return error.InvalidEndpoint;
    // Compare literal hosts only. Other endpoints must select a network explicitly.
    const hostname = switch (host) {
        .raw, .percent_encoded => |s| s,
    };
    const canonical: u64 = if (std.ascii.eqlIgnoreCase(hostname, "api.derive.xyz")) 1 else if (std.ascii.eqlIgnoreCase(hostname, "testnet.api.derive.xyz")) 11155111 else 0;
    if (input.object.get("chain_id")) |v| {
        const id = try derive.unsigned(u64, v);
        if ((id != 1 and id != 11155111) or (canonical != 0 and id != canonical)) return error.InvalidNetwork;
        return v;
    }
    if (canonical == 0) return error.InvalidNetwork;
    return w.text(if (canonical == 1) "1" else "11155111");
}
const native_asset = "0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";
fn requireRead(a: A, jobs: *V, input: V, instrument: ?[]const u8) !void {
    var params: V = .{ .object = .empty };
    if (instrument) |name| try params.object.put(a, "instrument_name", w.text(name));
    const key = if (instrument) |name| try std.mem.concat(a, u8, &.{ "instrument:", name }) else "currencies";
    if (jobs.object.contains(key)) return;
    var config: V = .{ .object = .empty };
    try config.object.put(a, "venue", w.text("derive"));
    try config.object.put(a, "operation", w.text(if (instrument != null) "public_get_instrument" else "public_get_all_currencies"));
    try config.object.put(a, "params", params);
    if (input.object.get("origin")) |origin| try config.object.put(a, "origin", origin);
    try jobs.object.put(a, key, config);
}
fn collect(a: A, jobs: *V, input: V, route: Route, p: V) anyerror!void {
    if (p != .object) return error.InvalidRequest;
    if (route == .private_transfer_positions) {
        try collect(a, jobs, input, .private_send_quote, try w.field(p, "maker_params"));
        return collect(a, jobs, input, .private_execute_quote, try w.field(p, "taker_params"));
    }
    if (try presigned(p)) return;
    switch (route) {
        .private_execute_quote, .private_send_quote, .private_replace_quote => {
            const legs = try w.field(p, "legs");
            if (legs != .array or legs.array.items.len == 0) return error.InvalidLegs;
            for (legs.array.items) |leg| try requireRead(a, jobs, input, try text(leg, "instrument_name"));
        },
        .private_order, .private_replace, .private_order_quote => try requireRead(a, jobs, input, try text(p, "instrument_name")),
        .private_withdraw, .private_transfer_spot, .private_transfer_spot_external => if (!std.ascii.eqlIgnoreCase(try text(p, "asset_name"), native_asset)) try requireRead(a, jobs, input, null),
        else => {},
    }
}
pub fn dependency(a: A, input: V) !V {
    var jobs: V = .{ .object = .empty };
    if (!eq(u8, try text(input, "venue"), "derive")) return jobs;
    const route = routeFor(try text(input, "operation")) orelse return jobs;
    if (input.object.get("api_key") == null or input.object.get("api_secret") == null) return error.AuthenticationRequired;
    _ = try network(input);
    try collect(a, &jobs, input, route, try w.field(input, "params"));
    return jobs;
}
fn instrumentMetadata(a: A, input: V, name: []const u8) !V {
    const meta = try w.field(try w.field(input, "metadata"), try std.mem.concat(a, u8, &.{ "instrument:", name }));
    if (meta != .object) return error.InvalidInstrument;
    if (meta.object.get("instrument_name")) |n| if (!eq(u8, try w.string(n), name)) return error.InvalidInstrument;
    return meta;
}
const Asset = struct { address: V, decimals: V };
fn asset(input: V, name: []const u8) !Asset {
    if (std.ascii.eqlIgnoreCase(name, native_asset)) return .{ .address = w.text(name), .decimals = w.text("18") };
    const currencies = try w.field(try w.field(input, "metadata"), "currencies");
    if (currencies != .array) return error.InvalidAsset;
    var found: ?Asset = null;
    for (currencies.array.items) |c| {
        const spots = try w.field(c, "spot");
        if (spots != .array) return error.InvalidAsset;
        for (spots.array.items) |spot| {
            if (spot != .object) return error.InvalidAsset;
            const erc = spot.object.get("erc20") orelse continue;
            if (erc != .object) continue;
            const decimals = erc.object.get("decimals") orelse w.text("0");
            const n = try derive.unsigned(u64, decimals);
            if (n == 0 and (erc.object.get("underlying_erc20") orelse V.null) == .null) continue;
            const addr = try text(spot, "address");
            if (std.ascii.eqlIgnoreCase(name, addr) or std.ascii.eqlIgnoreCase(name, try text(c, "currency"))) {
                if (found != null or n > 255) return error.InvalidAsset;
                found = .{ .address = w.text(addr), .decimals = decimals };
            }
        }
    }
    return found orelse error.InvalidAsset;
}
// One process-wide monotonic sequence is unique across concurrently constructed clients.
var previous_nonce = std.atomic.Value(u64).init(0);
fn nonce(ms: u64) !u64 {
    const now = try std.math.mul(u64, ms, 1_000_000);
    var prev = previous_nonce.load(.monotonic);
    while (true) {
        const n = @max(now, try std.math.add(u64, prev, 1));
        prev = previous_nonce.cmpxchgWeak(prev, n, .monotonic, .monotonic) orelse return n;
    }
}
pub fn sign(a: A, input: V, body: *V, ms: u64) !void {
    const route = routeFor(try text(input, "operation")) orelse return error.InvalidOperation;
    return signOne(a, input, body, ms, route);
}
fn signOne(a: A, input: V, body: *V, ms: u64, route: Route) anyerror!void {
    if (body.* != .object) return error.InvalidRequest;
    if (route == .private_transfer_positions) {
        const maker = body.object.getPtr("maker_params") orelse return error.InvalidRequest;
        try signOne(a, input, maker, ms, .private_send_quote);
        const taker = body.object.getPtr("taker_params") orelse return error.InvalidRequest;
        return signOne(a, input, taker, ms, .private_execute_quote);
    }
    const chain = try network(input);
    if (try presigned(body.*)) return;
    var data: V = .{ .object = try body.object.clone(a) };
    const kind: []const u8 = switch (route) {
        .private_order, .private_replace, .private_order_quote => blk: {
            const meta = try instrumentMetadata(a, input, try text(body.*, "instrument_name"));
            try data.object.put(a, "asset", try w.field(meta, "base_asset_address"));
            try data.object.put(a, "sub_id", try w.field(meta, "base_asset_sub_id"));
            try data.object.put(a, "recipient_id", try w.field(body.*, "subaccount_id"));
            const side = try text(body.*, "direction");
            if (!eq(u8, side, "buy") and !eq(u8, side, "sell")) return error.InvalidDirection;
            try data.object.put(a, "is_bid", .{ .bool = eq(u8, side, "buy") });
            break :blk "trade";
        },
        .private_transfer_positions => unreachable,
        .private_set_session_key => blk: {
            const names = [_][]const u8{ "admin", "withdraw", "trade:all", "trade:orderbook:all", "trade:orderbook:spot", "trade:orderbook:perp", "trade:orderbook:option", "trade:rfq:all", "trade:rfq:spot", "trade:rfq:perp", "trade:rfq:option", "transfer:all", "transfer:existing_subaccount", "transfer:new_subaccount", "transfer:different_owner_subaccount", "set_session_key", "liquidate", "vault:all", "vault:curator_create", "vault:curator_mint_and_burn", "vault:user_deposit", "vault:user_withdraw", "vault:user_cancel" };
            const scopes = try w.field(body.*, "protocol_scopes");
            if (scopes != .array) return error.InvalidScopes;
            var values = std.array_list.Managed(V).init(a);
            for (scopes.array.items) |v| {
                const scope = try w.string(v);
                for (names, 0..) |name, n| {
                    if (eq(u8, scope, name)) {
                        try values.append(.{ .number_string = try std.fmt.allocPrint(a, "{d}", .{n}) });
                        break;
                    }
                } else return error.InvalidScope;
            }
            try data.object.put(a, "session_key", try w.field(body.*, "public_session_key"));
            try data.object.put(a, "scopes", .{ .array = values });
            if ((body.object.get("subaccount_ids") orelse V.null) == .null) try data.object.put(a, "subaccount_ids", .{ .array = .init(a) });
            break :blk "session_key";
        },
        .private_transfer_spot, .private_transfer_spot_external, .private_withdraw => blk: {
            const spot = try asset(input, try text(body.*, "asset_name"));
            try data.object.put(a, "asset", spot.address);
            if (route == .private_withdraw) {
                try data.object.put(a, "decimals", spot.decimals);
                try data.object.put(a, "amount", try w.field(body.*, "amount_in_underlying"));
                if ((body.object.get("recipient") orelse V.null) == .null) try data.object.put(a, "recipient", try w.field(input, "api_key"));
                if ((body.object.get("force_batch") orelse V.null) == .null) try data.object.put(a, "force_batch", .{ .bool = false });
                break :blk "withdrawal";
            }
            try data.object.put(a, "recipient", if (route == .private_transfer_spot_external) try w.field(body.*, "recipient_address") else .null);
            break :blk "transfer";
        },
        .private_send_quote, .private_replace_quote, .private_execute_quote => blk: {
            const legs = try w.field(body.*, "legs");
            if (legs != .array) return error.InvalidLegs;
            var values = std.array_list.Managed(V).init(a);
            for (legs.array.items) |leg| {
                const meta = try instrumentMetadata(a, input, try text(leg, "instrument_name"));
                var entry: V = .{ .object = try leg.object.clone(a) };
                try entry.object.put(a, "asset_address", try w.field(meta, "base_asset_address"));
                try entry.object.put(a, "sub_id", try w.field(meta, "base_asset_sub_id"));
                try values.append(entry);
            }
            try data.object.put(a, "legs", .{ .array = values });
            break :blk if (route == .private_execute_quote) "rfq_execute" else "rfq_quote";
        },
        .private_liquidate => "liquidation",
        .private_delete_subaccount => "delete_subaccount",
        .private_create_vault => "vault_create",
        .private_request_vault_deposit => "vault_deposit",
        .private_request_vault_withdraw => "vault_withdraw",
        .private_cancel_all_vault_requests => "vault_cancel",
        .private_mint_vault_shares => "vault_mint",
        .private_burn_vault_shares => "vault_burn",
        .private_force_burn => "vault_force_burn",
        .private_update_whitelisted_recipients => "whitelisted_recipients",
    };
    try data.object.put(a, "type", w.text(kind));
    try signEnvelope(a, input, body, data, chain, ms);
}

pub fn offlineAction(a: A, input: V) !V {
    const chain = try network(input);
    const owner = input.object.get("api_key") orelse return error.AuthenticationRequired;
    const secret = input.object.get("api_secret") orelse return error.AuthenticationRequired;
    if (owner == .null or secret == .null) return error.AuthenticationRequired;
    if (input.object.get("action")) |raw| {
        const expected = try @import("evm.zig").hex(20, try text(raw, "owner"));
        if (!eq(u8, &expected, &try @import("evm.zig").hex(20, try w.string(owner)))) return error.SignerMismatch;
        var data: V = .{ .object = .empty };
        try data.object.put(a, "type", w.text("encoded"));
        try data.object.put(a, "module", try w.field(raw, "module"));
        try data.object.put(a, "data", try w.field(raw, "encoded_data"));
        var action: V = .{ .object = .empty };
        for ([_][]const u8{ "subaccount_id", "nonce", "expiry", "owner", "signer" }) |k| try action.object.put(a, k, try w.field(raw, k));
        try action.object.put(a, "chain_id", chain);
        try action.object.put(a, "data", data);
        try action.object.put(a, "key", secret);
        return w.field(try derive.signAction(a, action), "signature");
    }
    var body = try w.field(input, "request");
    if (body != .object) return error.InvalidRequest;
    var rt: std.Io.Threaded = .init_single_threaded;
    const ms = std.math.cast(u64, std.Io.Timestamp.now(rt.io(), .real).toMilliseconds()) orelse return error.InvalidTimestamp;
    try signEnvelope(a, input, &body, try w.field(input, "module_data"), chain, ms);
    return body;
}

fn signEnvelope(a: A, input: V, body: *V, data: V, chain: V, ms: u64) !void {
    var n = body.object.get("nonce") orelse V.null;
    if (n == .null or (n == .string and n.string.len == 0) or (n == .number_string and eq(u8, n.number_string, "0"))) {
        const digits = try std.fmt.allocPrint(a, "{d}", .{try nonce(ms)});
        n = if (n == .number_string) .{ .number_string = digits } else w.text(digits);
    }
    var expiry = body.object.get("signature_expiry_sec") orelse V.null;
    if (expiry == .null or (try derive.unsigned(u64, expiry)) == 0) expiry = .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{ms / 1000 + 300}) };
    var action: V = .{ .object = .empty };
    try action.object.put(a, "key", try w.field(input, "api_secret"));
    try action.object.put(a, "owner", try w.field(input, "api_key"));
    try action.object.put(a, "chain_id", chain);
    try action.object.put(a, "subaccount_id", body.object.get("subaccount_id") orelse w.text("0"));
    try action.object.put(a, "nonce", n);
    try action.object.put(a, "expiry", expiry);
    try action.object.put(a, "data", data);
    const signed = try derive.signAction(a, action);
    try body.object.put(a, "nonce", n);
    try body.object.put(a, "signature_expiry_sec", .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{try derive.unsigned(u64, expiry)}) });
    for ([_][]const u8{ "signature", "signer" }) |k| try body.object.put(a, k, try w.field(signed, k));
}

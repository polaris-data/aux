const std = @import("std");
const rt = @import("runtime.zig");
const signing = @import("signing.zig");
const w = @import("aux_core").wire;
const clients = @import("clients.zig");

/// Owns session credentials and the values borrowed by client(). Serialize
/// refresh/revoke with requests. Destroy streams before releasing the session.
pub const MonacoSession = struct {
    credentials: rt.Owned(rt.Value),
    identity: rt.Owned(rt.Value),
    base: rt.Client,
    var next_instance = std.atomic.Value(u64).init(1);

    pub fn login(base: rt.Client, options: anytype, wallet: signing.Signer) !MonacoSession {
        const origin = base.options.origin orelse "https://apimonaco.xyz";
        var transport = base;
        transport.options.origin = origin;
        var identity = try base.invoke(.monaco_login, .{ .step = "challenge", .options = options, .origin = origin });
        errdefer identity.deinit();
        var challenge = try transport.request(rt.Value, "monaco", "create_challenge", identity.value);
        defer challenge.deinit();
        var message = try base.invoke(.monaco_login, .{ .step = "message", .options = options, .origin = origin, .challenge = challenge.value });
        defer message.deinit();
        var scratch = std.heap.ArenaAllocator.init(base.allocator);
        defer scratch.deinit();
        const signature = try wallet.signature(scratch.allocator(), message.value);
        var verify = try base.invoke(.monaco_login, .{ .step = "verify", .options = options, .origin = origin, .challenge = challenge.value, .signature = signature });
        defer verify.deinit();
        var reply = try transport.request(rt.Value, "monaco", "verify_signature", verify.value);
        defer reply.deinit();
        return finish(transport, options, identity, reply.value);
    }

    pub fn restore(base: rt.Client, options: anytype, saved: rt.Value) !MonacoSession {
        const origin = base.options.origin orelse "https://apimonaco.xyz";
        var transport = base;
        transport.options.origin = origin;
        var id = try base.invoke(.monaco_login, .{ .step = "challenge", .options = options, .origin = origin });
        errdefer id.deinit();
        for ([_][]const u8{ "sessionPublicKey", "address", "clientId" }) |key| {
            if (!std.mem.eql(u8, try w.string(try w.field(id.value, key)), try w.string(try w.field(saved, key)))) return error.SessionMismatch;
        }
        if (try w.unsigned(u64, try w.field(id.value, "chainId")) != try w.unsigned(u64, try w.field(saved, "chainId"))) return error.SessionMismatch;
        return finish(transport, options, id, saved);
    }

    fn finish(base: rt.Client, options: anytype, identity: rt.Owned(rt.Value), response: rt.Value) !MonacoSession {
        var credentials = try base.invoke(.monaco_login, .{ .step = "finish", .options = options, .origin = base.options.origin, .response = response });
        errdefer credentials.deinit();
        const a = credentials.arena.allocator();
        const instance = try std.fmt.allocPrint(a, "{d}", .{next_instance.fetchAdd(1, .monotonic)});
        try credentials.value.object.put(a, "instance", w.text(instance));
        return .{ .base = base, .identity = identity, .credentials = credentials };
    }

    pub fn client(self: *const MonacoSession) clients.MonacoClient {
        var base = self.base;
        base.options.auth = self.credentials.value;
        return .{ .runtime = base };
    }

    pub fn info(self: *const MonacoSession) !rt.Owned(rt.Value) {
        var arena = std.heap.ArenaAllocator.init(self.base.allocator);
        errdefer arena.deinit();
        const a = arena.allocator();
        var result = try signing.value(a, self.identity.value);
        const expires = try signing.value(a, try w.field(self.credentials.value, "expires_at"));
        try result.object.put(a, "expiresAt", expires);
        return .{ .arena = arena, .value = result };
    }

    pub fn refresh(self: *MonacoSession) !void {
        var response = try self.client().refresh_session(.{});
        defer response.deinit();
        const expiry = response.value.expiresAt orelse return error.InvalidExpiry;
        const a = self.credentials.arena.allocator();
        try self.credentials.value.object.put(a, "expires_at", try signing.value(a, expiry));
    }

    pub fn revoke(self: *MonacoSession) !void {
        defer self.credentials.value.object.getPtr("expires_at").?.* = .{ .number_string = "0" };
        var response = try self.client().revoke_session(.{});
        response.deinit();
    }

    pub fn deinit(self: *MonacoSession) void {
        self.credentials.deinit();
        self.identity.deinit();
    }
};

/// Returns owned authentication options for subsequent private Paradex calls.
pub fn paradexAuthenticate(client: rt.Client) !rt.Owned(rt.Value) {
    var request = try @import("workflows.zig").paradex_onboarding.prepare_auth(client, .{});
    defer request.deinit();
    var reply = try client.request(rt.Value, "paradex", try w.string(try w.field(request.value, "operation")), try w.field(request.value, "params"));
    defer reply.deinit();
    const token = try w.string(try w.field(reply.value, "jwt_token"));
    var valid = try @import("workflows.zig").paradex_onboarding.validate_token(client, .{ .token = token });
    defer valid.deinit();
    var arena = std.heap.ArenaAllocator.init(client.allocator);
    errdefer arena.deinit();
    const a = arena.allocator();
    var auth = try signing.value(a, client.options.auth orelse rt.Value{ .object = .empty });
    try auth.object.put(a, "token", w.text(try a.dupe(u8, token)));
    try auth.object.put(a, "read_only", .{ .bool = false });
    return .{ .arena = arena, .value = auth };
}

pub fn hyperliquidAction(base: rt.Client, action: rt.Value, chain_id: u64) !rt.Owned(rt.Value) {
    var scratch = std.heap.ArenaAllocator.init(base.allocator);
    defer scratch.deinit();
    const a = scratch.allocator();
    var auth = try signing.value(a, base.options.auth orelse return error.AuthenticationRequired);
    const network = try w.field(auth, "network");
    if (base.options.signer) |signer| {
        var digest = try base.invoke(.hyperliquid_user_action, .{ .action = action, .network = network, .signature_chain_id = chain_id });
        defer digest.deinit();
        try auth.object.put(a, "signature", try signer.signature(a, digest.value));
    }
    return base.invoke(.hyperliquid_user_action, .{ .action = action, .auth = auth, .network = network, .signature_chain_id = chain_id });
}

pub fn pascalPermitAt(client: rt.Client, operation: []const u8, body: anytype, timestamp: u64) !rt.Owned(rt.Value) {
    const auth = client.options.auth orelse return error.AuthenticationRequired;
    const signer = client.options.signer orelse return client.invoke(.pascal_sign_permit, .{
        .operation = operation,
        .body = body,
        .timestamp = timestamp,
        .auth = auth,
    });
    var arena = std.heap.ArenaAllocator.init(client.allocator);
    errdefer arena.deinit();
    const a = arena.allocator();
    const signed = try signing.pascal(a, signer, operation, try signing.value(a, body), auth, timestamp);
    return .{ .arena = arena, .value = signed };
}

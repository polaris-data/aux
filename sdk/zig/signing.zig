const std = @import("std");
const core = @import("aux_core");
const w = core.wire;
const Value = w.V;
const Allocator = std.mem.Allocator;

pub const Signer = struct {
    context: ?*anyopaque = null,
    /// The payload is borrowed for the duration of the call. The returned JSON
    /// string or signature object must live at least until the operation returns.
    sign: *const fn (?*anyopaque, Allocator, []const u8) anyerror!Value,

    pub fn signature(self: Signer, a: Allocator, payload: Value) !Value {
        return self.sign(self.context, a, try w.string(payload));
    }
};

pub fn value(a: Allocator, input: anytype) !Value {
    return w.parse(a, try std.json.Stringify.valueAlloc(a, input, .{ .emit_null_optional_fields = false }));
}

fn evaluate(a: Allocator, op: core.native.Operation, input: Value) !Value {
    return core.native.evaluate(a, op, try std.json.Stringify.valueAlloc(a, input, .{}), null);
}

fn objectField(a: Allocator, parent: *Value, key: []const u8) !*Value {
    if (parent.* != .object) return error.ExpectedObject;
    const entry = try parent.object.getOrPut(a, key);
    if (!entry.found_existing or entry.value_ptr.* == .null) entry.value_ptr.* = .{ .object = .empty };
    if (entry.value_ptr.* != .object) return error.ExpectedObject;
    return entry.value_ptr;
}

pub fn pascal(a: Allocator, signer: Signer, operation: []const u8, body: Value, auth: Value, timestamp: ?u64) !Value {
    const proof = try evaluate(a, .pascal_sign_permit, try value(a, .{
        .step = "prepare_external",
        .timestamp = timestamp,
        .operation = operation,
        .body = body,
        .auth = auth,
    }));
    const messages = try w.field(proof, "messages");
    const rows = if (body == .array) body.array.items else &.{body};
    if (messages != .array or messages.array.items.len != rows.len) return error.InvalidPermitMessages;
    var signed = std.array_list.Managed(Value).init(a);
    for (rows, messages.array.items) |row, message| {
        var identity = try value(a, auth);
        if (identity != .object) return error.ExpectedObject;
        try identity.object.put(a, "signature", try signer.signature(a, message));
        try signed.append(try evaluate(a, .pascal_sign_permit, try value(a, .{
            .operation = operation,
            .auth = identity,
            .timestamp = try w.field(proof, "timestamp"),
            .body = row,
        })));
    }
    return if (body == .array) .{ .array = signed } else signed.items[0];
}

pub fn apply(a: Allocator, signer: Signer, venue: []const u8, operation: []const u8, args: *Value) !void {
    const eq = std.mem.eql;
    if (eq(u8, venue, "pascal") and std.mem.startsWith(u8, operation, "write.")) {
        const params = try objectField(a, args, "params");
        const body = try pascal(a, signer, operation, try w.field(params.*, "body"), try w.field(args.*, "auth"), null);
        try params.object.put(a, "body", body);
        return;
    }
    const op: core.native.Operation = if (eq(u8, venue, "kalshi") and !std.mem.startsWith(u8, operation, "clearing.")) .kalshi_request_proof else if (eq(u8, venue, "arcus") and (eq(u8, operation, "place_order") or eq(u8, operation, "cancel_order"))) .arcus_signing_payload else if (eq(u8, venue, "hyperliquid") and (eq(u8, operation, "order") or eq(u8, operation, "cancel") or eq(u8, operation, "cancel_by_cloid"))) .hyperliquid_signing_payload else return;
    const proof = try evaluate(a, op, args.*);
    const auth = try objectField(a, args, "auth");
    const payload = try w.field(proof, if (op == .hyperliquid_signing_payload) "digest" else "payload");
    try auth.object.put(a, "signature", try signer.signature(a, payload));
    if (op == .hyperliquid_signing_payload) {
        const options = try objectField(a, try objectField(a, args, "params"), "options");
        try options.object.put(a, "nonce", try w.field(proof, "nonce"));
    } else {
        try auth.object.put(a, "timestamp", try w.field(proof, "timestamp"));
        if (op == .kalshi_request_proof) try auth.object.put(a, "payload", payload);
    }
}

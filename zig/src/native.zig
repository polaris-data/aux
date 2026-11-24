const std = @import("std");
const chainlink = @import("chainlink.zig");
const evm = @import("evm.zig");
const auth = @import("auth.zig");
const http = @import("http.zig");
const transport = @import("transport.zig");
const wire = @import("wire.zig");
const gpa = std.heap.c_allocator;

pub const Buffer = extern struct { ptr: ?[*]u8 = null, len: usize = 0 };
pub const Operation = enum(u32) {
    chainlink_decode_report = 1,
    metric_encode_call,
    metric_decode_input,
    metric_decode_output,
    metric_decode_error,
    metric_decode_event,
    polymarket_encode_call,
    polymarket_decode_input,
    polymarket_decode_output,
    http_prepare,
    http_decode,
    http_execute,
    qfex_signature,
    databento_cram,
    arcus_sign,
    monaco_sign,
    evm_sign_transaction,
    dbn_decode_record,
    paradex_decode_sbe,
    http_prepare_dispatch,
    http_finish_dispatch,
    kalshi_headers,
    derive_login,
    derive_sign_action,
    evm_prepare_rpc,
    evm_finish_rpc,
    evm_execute_rpc,
    http_dependency,
    solana_associated_token_address,
    solana_prepare_deposit,
    solana_sign_deposit,
    pascal_sign_permit,
    pascal_begin_submission,
    pascal_finish_submission,
    pascal_reconcile_wallet,
    paradex_sign_typed,
    solana_chain,
    evm_chain,
    extended_settlement,
    extended_order,
    hyperliquid_user_action,
    monaco_login,
    arcus_signing_payload,
    metric_helper,
    hyperliquid_signing_payload,
    polymarket_helper,
    phoenix_transaction,
    phoenix_chain,
    extended_onboarding,
    paradex_onboarding,
    metric_chain,
    metric_prepare_call,
    kalshi_request_proof,
    polymarket_prepare_call,
    polymarket_decode_error,
    polymarket_decode_event,
    derive_offline_action,
    pascal_wallet_pending,
    signer,
};

fn process(a: std.mem.Allocator, op: Operation, bytes: []const u8, dial: ?@import("host_stream.zig").Dial) ![]u8 {
    const val = if (op == .chainlink_decode_report) try chainlink.decode(a, bytes) else blk: {
        const parsed = try wire.parse(a, bytes);
        break :blk switch (op) {
            .metric_encode_call => try evm.encodeCall(a, .metric, parsed),
            .metric_decode_input, .metric_decode_output => try evm.decodeCall(a, .metric, parsed, op == .metric_decode_output),
            .metric_decode_error => try evm.decodeError(a, .metric, parsed),
            .metric_decode_event => try evm.decodeEvent(a, .metric, parsed),
            .polymarket_prepare_call => try evm.prepareCall(a, .polymarket, parsed),
            .polymarket_decode_error => try evm.decodeError(a, .polymarket, parsed),
            .polymarket_decode_event => try evm.decodeEvent(a, .polymarket, parsed),
            .kalshi_request_proof => try transport.kalshiProof(a, parsed),
            .metric_prepare_call => try evm.prepareCall(a, .metric, parsed),
            .metric_chain => try @import("metric_chain.zig").execute(a, parsed),
            .paradex_onboarding => try @import("paradex_onboarding.zig").execute(a, parsed),
            .extended_onboarding => try @import("extended_onboarding.zig").execute(a, parsed),
            .phoenix_chain => try @import("phoenix_rpc.zig").execute(a, parsed),
            .phoenix_transaction => result: {
                const phoenix = @import("phoenix_transaction.zig");
                const operation = std.meta.stringToEnum(phoenix.Op, try wire.string(try wire.field(parsed, "operation"))) orelse return error.InvalidOperation;
                break :result try phoenix.execute(a, operation, try wire.field(parsed, "params"));
            },
            .polymarket_helper => try @import("polymarket_chain.zig").execute(a, parsed),
            .polymarket_encode_call => try evm.encodeCall(a, .polymarket, parsed),
            .polymarket_decode_input, .polymarket_decode_output => try evm.decodeCall(a, .polymarket, parsed, op == .polymarket_decode_output),
            .evm_prepare_rpc => try @import("rpc.zig").prepare(a, parsed),
            .evm_finish_rpc => try @import("rpc.zig").finish(a, parsed),
            .evm_execute_rpc => try @import("rpc.zig").execute(a, parsed),
            .solana_associated_token_address => try @import("solana.zig").associated(a, parsed),
            .solana_prepare_deposit => try @import("solana.zig").prepare(a, parsed),
            .solana_sign_deposit => try @import("solana.zig").sign(a, parsed),
            .pascal_sign_permit => try @import("pascal.zig").signAt(a, parsed),
            .pascal_begin_submission => try @import("pascal.zig").beginHost(a, parsed),
            .pascal_finish_submission => try @import("pascal.zig").endHost(parsed),
            .signer => try @import("signer.zig").execute(a, parsed),
            .pascal_wallet_pending => try @import("pascal.zig").walletPending(try wire.field(parsed, "auth")),
            .pascal_reconcile_wallet => try @import("pascal.zig").reconcile(parsed),
            .hyperliquid_signing_payload => result: {
                var runtime: std.Io.Threaded = .init_single_threaded;
                break :result try @import("hyperliquid_trading.zig").signingPayload(a, parsed, runtime.io());
            },
            .metric_helper => result: {
                const math = @import("metric_math.zig");
                const name = try wire.string(try wire.field(parsed, "operation"));
                const workflows = @import("metric_workflows.zig");
                if (std.meta.stringToEnum(workflows.Op, name)) |operation| break :result try workflows.execute(a, operation, try wire.field(parsed, "params"));
                const planning = @import("metric_planning.zig");
                if (std.meta.stringToEnum(planning.Op, name)) |operation| break :result try planning.execute(a, operation, try wire.field(parsed, "params"));
                if (std.meta.stringToEnum(math.Op, name)) |operation| break :result try math.execute(a, operation, try wire.field(parsed, "params"));
                break :result try @import("metric_oracle.zig").execute(a, parsed);
            },
            .arcus_signing_payload => result: {
                var runtime: std.Io.Threaded = .init_single_threaded;
                break :result try @import("arcus_trading.zig").signing(a, parsed, runtime.io(), null);
            },
            .monaco_login => try @import("monaco_login.zig").step(a, parsed),
            .hyperliquid_user_action => try @import("hyperliquid_trading.zig").userAction(a, parsed),
            .extended_order => try @import("extended_order.zig").order(a, parsed),
            .extended_settlement => try @import("extended_settlement.zig").settle(a, parsed),
            .evm_chain => try @import("evm_chain.zig").execute(a, parsed),
            .solana_chain => try @import("solana_rpc.zig").execute(a, parsed),
            .paradex_sign_typed => try @import("paradex_sign.zig").typed(a, parsed),
            .http_dependency => try @import("derive_orders.zig").dependency(a, parsed),
            .derive_login => try @import("derive.zig").loginProof(a, parsed),
            .derive_offline_action => try @import("derive_orders.zig").offlineAction(a, parsed),
            .derive_sign_action => try @import("derive.zig").signAction(a, parsed),
            .kalshi_headers => try auth.kalshi(a, parsed),
            .qfex_signature => try auth.qfex(a, parsed),
            .databento_cram => try auth.databento(a, parsed),
            .arcus_sign => try auth.arcus(a, parsed),
            .paradex_decode_sbe => try @import("sbe.zig").decode(a, parsed),
            .dbn_decode_record => try @import("dbn.zig").decode(a, parsed),
            .evm_sign_transaction => try @import("transaction.zig").sign(a, parsed),
            .monaco_sign => try auth.monaco(a, parsed),
            .http_prepare_dispatch => try transport.prepareDispatch(a, parsed),
            .http_finish_dispatch => try transport.finishDispatch(a, parsed),
            .http_prepare => try (try http.prepare(a, parsed)).json(a),
            .http_execute => try transport.executeWithDial(a, parsed, dial),
            .http_decode => blk2: {
                const status = try wire.field(parsed, "status");
                if (status != .number_string) return error.InvalidStatus;
                break :blk2 try http.decode(a, try http.venue(parsed), try std.fmt.parseInt(u16, status.number_string, 10), try wire.string(try wire.field(parsed, "body")));
            },
            .chainlink_decode_report => unreachable,
        };
    };
    return std.json.Stringify.valueAlloc(gpa, val, .{});
}

export fn aux_v1_invoke(id: u32, ptr: [*]const u8, len: usize, out: *Buffer) u32 {
    return invoke(id, ptr, len, null, out);
}
export fn aux_v1_http_execute_with_transport(ptr: [*]const u8, len: usize, dial: *const @import("host_stream.zig").Dial, out: *Buffer) u32 {
    return invoke(@backingInt(Operation.http_execute), ptr, len, dial.*, out);
}
export fn aux_v1_invoke_detailed(id: u32, ptr: [*]const u8, len: usize, dial: ?*const @import("host_stream.zig").Dial, out: *Buffer) u32 {
    const failure = @import("http_failure.zig");
    var arena = std.heap.ArenaAllocator.init(gpa);
    defer arena.deinit();
    var capture = failure.Capture{ .allocator = arena.allocator() };
    const previous = failure.current;
    failure.current = &capture;
    defer failure.current = previous;
    const code = invoke(id, ptr, len, if (dial) |d| d.* else null, out);
    if (code == 4) if (capture.value) |detail| {
        const owned = gpa.dupe(u8, detail) catch return 2;
        out.* = .{ .ptr = owned.ptr, .len = owned.len };
    };
    return code;
}
fn invoke(id: u32, ptr: [*]const u8, len: usize, dial: ?@import("host_stream.zig").Dial, out: *Buffer) u32 {
    out.* = .{};
    if (len > 34 * 1024 * 1024) return 1;
    const op = std.enums.fromInt(Operation, id) orelse return 1;
    var arena = std.heap.ArenaAllocator.init(gpa);
    defer arena.deinit();
    const result = process(arena.allocator(), op, ptr[0..len], dial) catch |err| return switch (err) {
        error.OutOfMemory => 2,
        error.Network => 3,
        error.PendingOutcome => 7,
        error.ExchangeApi => 4,
        error.AuthenticationRequired => 5,
        else => 1,
    };
    out.* = .{ .ptr = result.ptr, .len = result.len };
    return 0;
}
export fn aux_v1_result_free(buf: Buffer) void {
    if (buf.ptr) |ptr| gpa.free(ptr[0..buf.len]);
}

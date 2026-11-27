const std = @import("std");
const core = @import("aux_core");
const rt = @import("runtime.zig");
const Reader = @import("chain.zig").Reader;
const wire = core.wire;

pub const Feed = core.synfutures_graph.Feed;
pub const Interval = core.synfutures_graph.Interval;
pub const page_size = core.synfutures_graph.page_size;
pub const nextCursor = core.synfutures_graph.nextCursor;

pub const LogPageSpec = struct {
    addresses: []const []const u8,
    from_block: u64,
};

const RpcRequest = struct {
    id: []const u8,
    url: []const u8,
    body: []const u8,
};

pub const LogPage = struct {
    from_block: u64,
    to_block: u64,
    request: RpcRequest,
};

/// Keep 20 blocks behind the Base head. The caller sends the prepared request
/// and advances its block cursor after capturing the response.
pub fn prepareConfirmedLogs(reader: Reader, spec: LogPageSpec) !?rt.Owned(LogPage) {
    if (spec.addresses.len == 0 or spec.addresses.len > 40) return error.InvalidAddressCount;
    for (spec.addresses) |address| {
        if (address.len != 42 or !std.mem.startsWith(u8, address, "0x")) return error.InvalidAddress;
        for (address[2..]) |c| if (!std.ascii.isHex(c)) return error.InvalidAddress;
    }
    var chain_id = try reader.read("chain_id", .{});
    defer chain_id.deinit();
    if (try wire.unsigned(u64, chain_id.value) != 8453) return error.UnexpectedChain;

    var head = try reader.read("block_number", .{});
    defer head.deinit();
    const height = try wire.unsigned(u64, head.value);
    if (height < 20 or spec.from_block > height - 20) return null;
    const end = @min(spec.from_block +| 99, height - 20);
    var from_buf: [18]u8 = undefined;
    var to_buf: [18]u8 = undefined;
    const request = try reader.client.invokeAs(RpcRequest, .evm_prepare_rpc, .{
        .operation = "logs",
        .endpoint = reader.endpoint,
        .params = .{ .filter = .{
            .address = spec.addresses,
            .fromBlock = try std.fmt.bufPrint(&from_buf, "0x{x}", .{spec.from_block}),
            .toBlock = try std.fmt.bufPrint(&to_buf, "0x{x}", .{end}),
        } },
    });
    return .{
        .arena = request.arena,
        .value = .{ .from_block = spec.from_block, .to_block = end, .request = request.value },
    };
}

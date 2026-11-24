const rt = @import("runtime.zig");
const std = @import("std");
const signing = @import("signing.zig");

pub const Evm = struct {
    client: rt.Client,
    chain_id: u64,
    read_rpc: []const u8,
    write_rpc: []const u8,
    code_hashes: ?rt.Value = null,

    pub fn reader(self: Evm) Reader {
        return .{ .client = self.client, .endpoint = self.read_rpc };
    }
    pub fn read(self: Evm, method: []const u8, params: anytype) !rt.Owned(rt.Value) {
        return self.reader().read(method, params);
    }
    pub fn transaction(self: Evm, method: []const u8, params: anytype) !rt.Owned(rt.Value) {
        return self.client.invoke(.evm_chain, .{ .chain_id = self.chain_id, .read_rpc = self.read_rpc, .write_rpc = self.write_rpc, .code_hashes = self.code_hashes orelse rt.Value{ .object = .empty }, .operation = method, .params = rt.paramsObject(params) });
    }
    pub fn metric(self: Evm, method: []const u8, params: anytype) !rt.Owned(rt.Value) {
        return self.client.invoke(.metric_chain, .{ .operation = method, .params = rt.paramsObject(params), .endpoint = self.read_rpc });
    }
    pub fn sendWithSigner(self: Evm, address: []const u8, prepared: rt.Value, signer: signing.Signer) !rt.Owned(rt.Value) {
        var digest = try self.transaction("prepare_external", .{ .address = address, .prepared = prepared });
        defer digest.deinit();
        var scratch = std.heap.ArenaAllocator.init(self.client.allocator);
        defer scratch.deinit();
        const signature = try signer.signature(scratch.allocator(), digest.value);
        return self.transaction("send_external", .{ .address = address, .prepared = prepared, .signature = signature });
    }
};

pub const ReadOptions = struct {
    block: []const u8 = "latest",
    from: ?[]const u8 = null,
    value: []const u8 = "0x0",
};

pub const Contract = struct {
    chain: Evm,
    venue: enum { metric, polymarket },
    name: []const u8,
    address: []const u8,

    pub fn prepare(self: Contract, function: []const u8, args: anytype, value: []const u8) !rt.Owned(rt.Value) {
        return self.chain.client.invoke(if (self.venue == .metric) .metric_prepare_call else .polymarket_prepare_call, .{ .contract = self.name, .function = function, .to = self.address, .args = args, .value = value });
    }
    pub fn call(self: Contract, function: []const u8, args: anytype, options: ReadOptions) !rt.Owned(rt.Value) {
        var encoded = try self.chain.client.invokeAs([]const u8, if (self.venue == .metric) .metric_encode_call else .polymarket_encode_call, .{ .contract = self.name, .function = function, .args = args });
        defer encoded.deinit();
        var reply = try self.chain.read("eth_call", .{ .to = self.address, .data = encoded.value, .block = options.block, .from = options.from, .value = options.value });
        defer reply.deinit();
        return self.chain.client.invoke(if (self.venue == .metric) .metric_decode_output else .polymarket_decode_output, .{ .contract = self.name, .function = function, .data = reply.value });
    }
};

/// The native submission coordinator keys pending transactions by this handle.
pub const Solana = struct {
    client: rt.Client,
    id: rt.Owned([]const u8),
    endpoint: []const u8,
    mint: ?[]const u8,
    decimals: ?u8,

    pub fn phoenix(client: rt.Client, endpoint: []const u8) !Solana {
        return init(client, endpoint, null, null);
    }
    pub fn pascal(client: rt.Client, endpoint: []const u8, mint: []const u8, decimals: u8) !Solana {
        return init(client, endpoint, mint, decimals);
    }
    fn init(client: rt.Client, endpoint: []const u8, mint: ?[]const u8, decimals: ?u8) !Solana {
        const id = try client.invokeAs([]const u8, if (mint == null) .phoenix_chain else .solana_chain, .{ .operation = "new", .endpoint = endpoint, .mint = mint, .decimals = decimals });
        return .{ .client = client, .id = id, .endpoint = endpoint, .mint = mint, .decimals = decimals };
    }
    pub fn command(self: Solana, method: []const u8, params: anytype) !rt.Owned(rt.Value) {
        return self.client.invoke(if (self.mint == null) .phoenix_chain else .solana_chain, .{
            .operation = method,
            .client = self.id.value,
            .endpoint = self.endpoint,
            .mint = self.mint,
            .decimals = self.decimals,
            .params = rt.paramsObject(params),
        });
    }
    pub fn deinit(self: *Solana) void {
        self.id.deinit();
    }
};

/// Pascal reviews a hexadecimal transaction message. Phoenix reviews base64.
/// Both verify the returned Solana signature against the prepared transaction.
pub fn signDeposit(client: rt.Client, key: []const u8, prepared: rt.Value, signer: signing.Signer) !rt.Owned(rt.Value) {
    var message = try client.invoke(.solana_sign_deposit, .{ .public_key = key, .prepared = prepared });
    defer message.deinit();
    var scratch = std.heap.ArenaAllocator.init(client.allocator);
    defer scratch.deinit();
    return client.invoke(.solana_sign_deposit, .{ .public_key = key, .prepared = prepared, .signature = try signer.signature(scratch.allocator(), message.value) });
}

pub fn signPhoenix(client: rt.Client, key: []const u8, prepared: rt.Value, signatures: rt.Value, signer: signing.Signer) !rt.Owned(rt.Value) {
    const tx = @import("workflows.zig").phoenix_transaction;
    var message = try tx.sign_transaction(client, .{ .public_key = key, .prepared = prepared, .signatures = signatures });
    defer message.deinit();
    var scratch = std.heap.ArenaAllocator.init(client.allocator);
    defer scratch.deinit();
    return tx.sign_transaction(client, .{ .public_key = key, .prepared = prepared, .signatures = signatures, .signature = try signer.signature(scratch.allocator(), message.value) });
}

/// Read-only RPC client. An application HTTP transport can also supply the
/// response for block-tagged contract calls.
pub const Reader = struct {
    client: rt.Client,
    endpoint: []const u8,

    pub fn read(self: Reader, operation: []const u8, params: anytype) !rt.Owned(rt.Value) {
        return self.client.invoke(.evm_execute_rpc, .{ .operation = operation, .params = rt.paramsObject(params), .endpoint = self.endpoint });
    }
};

pub fn reconcileDepositCredit(chain: Solana, api: @import("clients.zig").PascalClient, signature: []const u8, request: anytype) !rt.Owned(rt.Value) {
    if (chain.mint == null) return error.PascalClientRequired;
    var confirmation = try chain.command("reconcile", .{ .signature = signature });
    defer confirmation.deinit();
    var transfers = try api.transfer_history(request);
    defer transfers.deinit();
    var arena = std.heap.ArenaAllocator.init(chain.client.allocator);
    errdefer arena.deinit();
    const value = try signing.value(arena.allocator(), .{ .signature = signature, .confirmation = confirmation.value, .transfers = transfers.value });
    return .{ .arena = arena, .value = value };
}

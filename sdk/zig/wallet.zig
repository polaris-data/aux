const std = @import("std");
const rt = @import("runtime.zig");
const signing = @import("signing.zig");
const helpers = @import("workflows.zig");

/// Locally held keys and application signing callbacks use the same signature
/// verification and EIP-712 implementations as the packaged bindings.
pub const Ethereum = struct {
    client: rt.Client,
    identity: union(enum) { key: []const u8, remote: struct { address: []const u8, signer: signing.Signer } },

    pub fn address(self: Ethereum) !rt.Owned([]const u8) {
        switch (self.identity) {
            .key => |key| return self.client.invokeAs([]const u8, .signer, .{ .operation = "ethereum_address", .key = key }),
            .remote => |r| {
                var decoded: [20]u8 = undefined;
                const raw = if (std.mem.startsWith(u8, r.address, "0x")) r.address[2..] else r.address;
                if (raw.len != decoded.len * 2) return error.InvalidAddress;
                _ = try std.fmt.hexToBytes(&decoded, raw);
                var arena = std.heap.ArenaAllocator.init(self.client.allocator);
                errdefer arena.deinit();
                const normalized = try std.fmt.allocPrint(arena.allocator(), "0x{x}", .{&decoded});
                return .{ .arena = arena, .value = normalized };
            },
        }
    }

    pub fn signDigest(self: Ethereum, digest: []const u8) !rt.Owned(rt.Value) {
        return switch (self.identity) {
            .key => |key| helpers.signer.sign_digest(self.client, .{ .key = key, .digest = digest }),
            .remote => |r| blk: {
                var scratch = std.heap.ArenaAllocator.init(self.client.allocator);
                defer scratch.deinit();
                const signature = try r.signer.sign(r.signer.context, scratch.allocator(), digest);
                break :blk helpers.signer.verify_digest(self.client, .{ .address = r.address, .digest = digest, .signature = signature });
            },
        };
    }
    pub fn signPersonal(self: Ethereum, message: []const u8) !rt.Owned(rt.Value) {
        const a = self.client.allocator;
        const hex = try std.fmt.allocPrint(a, "{x}", .{message});
        defer a.free(hex);
        var digest = try helpers.signer.personal_hash(self.client, .{ .message = hex });
        defer digest.deinit();
        return self.signDigest(digest.value.string);
    }
    pub fn signTransaction(self: Ethereum, prepared: rt.Value) !rt.Owned(rt.Value) {
        var digest = try self.client.invoke(.evm_sign_transaction, .{ .prepared = prepared, .digest_only = true });
        defer digest.deinit();
        var proof = try self.signDigest(digest.value.string);
        defer proof.deinit();
        var addr = try self.address();
        defer addr.deinit();
        return self.client.invoke(.evm_sign_transaction, .{ .address = addr.value, .prepared = prepared, .signature = proof.value });
    }
    pub fn signTypedData(self: Ethereum, payload: rt.Value) !rt.Owned(rt.Value) {
        var digest = try helpers.polymarket_helper.typed_data_hash(self.client, .{ .payload = payload });
        defer digest.deinit();
        return self.signDigest(digest.value.string);
    }
    pub fn signPolymarketOrder(self: Ethereum, order: rt.Value, system: enum { v1, v2 }, negative_risk: bool) !rt.Owned(rt.Value) {
        const poly = helpers.polymarket_helper;
        var domain = try poly.market_order_domain(self.client, .{ .market = .{ .version = @tagName(system), .negRisk = negative_risk } });
        defer domain.deinit();
        var typed = try poly.order_typed_data(self.client, .{ .order = order, .domain = domain.value });
        defer typed.deinit();
        var sig = try self.signTypedData(typed.value);
        defer sig.deinit();
        var addr = try self.address();
        defer addr.deinit();
        return poly.order_signature(self.client, .{ .order = order, .address = addr.value, .domain = domain.value, .signature = sig.value });
    }
};

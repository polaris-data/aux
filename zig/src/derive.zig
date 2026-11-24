// Derive EIP-191 session proofs and EIP-712 Matching module actions.
const std = @import("std");
const w = @import("wire.zig");
const evm = @import("evm.zig");
const tx = @import("transaction.zig");
const A = w.A;
const V = w.V;
const Word = [32]u8;

const hex = evm.hex;
const word = evm.encodeWord;
const address = evm.address;
fn text(v: V, name: []const u8) ![]const u8 {
    return w.string(try w.field(v, name));
}
pub const unsigned = w.unsigned;

pub fn loginOnly(operation: []const u8) bool {
    var lines = std.mem.tokenizeScalar(u8, @embedFile("derive-login-routes.txt"), '\n');
    while (lines.next()) |name| if (std.mem.eql(u8, name, operation)) return true;
    return false;
}
pub fn login(a: A, owner: []const u8, secret: []const u8, ms: u64) !V {
    const wallet = try hex(20, owner);
    var key = try hex(32, secret);
    defer std.crypto.secureZero(u8, &key);
    const time = try std.fmt.allocPrint(a, "{d}", .{ms});
    const digest = evm.hash(try std.fmt.allocPrint(a, "\x19Ethereum Signed Message:\n{d}{s}", .{ time.len, time }));
    const sig = try tx.recoverable(key, digest);
    var out: V = .{ .object = .empty };
    try out.object.put(a, "wallet", w.text(try std.fmt.allocPrint(a, "0x{x}", .{&wallet})));
    try out.object.put(a, "timestamp", w.text(time));
    try out.object.put(a, "signature", w.text(try std.fmt.allocPrint(a, "0x{x}", .{&sig})));
    return out;
}
pub fn loginProof(a: A, v: V) !V {
    return login(a, try text(v, "owner"), try text(v, "key"), try unsigned(u64, try w.field(v, "timestamp")));
}

const Amount = enum { unsigned, signed, positive };
fn scaled(s: []const u8, mode: Amount, places: usize, max_fraction: usize) !Word {
    const negative = std.mem.startsWith(u8, s, "-");
    if (negative and mode != .signed) return error.InvalidAmount;
    const digits = s[@intFromBool(negative)..];
    const dot = std.mem.findScalar(u8, digits, '.') orelse digits.len;
    const fraction = if (dot < digits.len) digits[dot + 1 ..] else "";
    if (dot == 0 or fraction.len > places or fraction.len > max_fraction) return error.InvalidPrecision;
    var n: u256 = 0;
    for ([_][]const u8{ digits[0..dot], fraction }) |part| for (part) |c| {
        if (!std.ascii.isDigit(c)) return error.InvalidAmount;
        n = try std.math.add(u256, try std.math.mul(u256, n, 10), c - '0');
    };
    for (fraction.len..places) |_| n = try std.math.mul(u256, n, 10);
    if ((mode == .positive and n == 0) or (mode == .signed and n > std.math.maxInt(i256))) return error.InvalidAmount;
    return word(if (negative) 0 -% n else n);
}
const Module = struct { contract: []const u8, bytes: []const u8 };
const Encoder = struct {
    a: A,
    body: V,
    words: std.ArrayList(Word) = .empty,
    fn push(self: *Encoder, x: Word) !void {
        try self.words.append(self.a, x);
    }
    fn id(self: *Encoder, comptime T: type, k: []const u8) !void {
        try self.push(word(try unsigned(T, try w.field(self.body, k))));
    }
    fn addr(self: *Encoder, k: []const u8) !void {
        try self.push(try address(try text(self.body, k)));
    }
    fn amount(self: *Encoder, k: []const u8, mode: Amount) !void {
        try self.push(try scaled(try text(self.body, k), mode, 18, 12));
    }
    fn boolean(self: *Encoder, k: []const u8) !void {
        const value = try w.field(self.body, k);
        if (value != .bool) return error.InvalidBoolean;
        try self.push(word(@intFromBool(value.bool)));
    }
    fn pairOfArrays(self: *Encoder, first: []const u8, second: []const u8, addresses: bool) !void {
        const left = try w.field(self.body, first);
        const right = try w.field(self.body, second);
        if (left != .array or right != .array) return error.InvalidArray;
        try self.push(word(64));
        try self.push(word(96 + 32 * left.array.items.len));
        for ([_]V{ left, right }, 0..) |list, i| {
            try self.push(word(list.array.items.len));
            for (list.array.items) |v| try self.push(if (addresses) try address(try w.string(v)) else if (i == 0) word(try unsigned(u8, v)) else word(try unsigned(u256, v)));
        }
    }
};
const Kind = enum { trade, transfer, withdrawal, session_key, whitelisted_recipients, rfq_quote, rfq_execute, vault_create, vault_deposit, vault_withdraw, vault_cancel, vault_mint, vault_burn, vault_force_burn, liquidation, delete_subaccount, encoded };
fn sell(v: V) !bool {
    const s = try text(v, "direction");
    if (std.mem.eql(u8, s, "sell")) return true;
    if (std.mem.eql(u8, s, "buy")) return false;
    return error.InvalidDirection;
}
const Leg = struct {
    name: []const u8,
    fields: [4]Word,
    fn less(_: void, l: Leg, r: Leg) bool {
        return std.mem.order(u8, l.name, r.name) == .lt;
    }
};
fn rfq(e: *Encoder, execute: bool) !void {
    const values = try w.field(e.body, "legs");
    if (values != .array or values.array.items.len == 0) return error.InvalidLegs;
    const legs = try e.a.alloc(Leg, values.array.items.len);
    const reverse = try sell(e.body);
    for (legs, values.array.items) |*leg, v| {
        const size = try scaled(try text(v, "amount"), .positive, 18, 12);
        const n = std.mem.readInt(u256, &size, .big);
        const negative = (try sell(v)) != (reverse != execute);
        if (negative and n > std.math.maxInt(i256)) return error.InvalidAmount;
        leg.* = .{ .name = try text(v, "instrument_name"), .fields = .{ try address(try text(v, "asset_address")), word(try unsigned(u256, try w.field(v, "sub_id"))), try scaled(try text(v, "price"), .unsigned, 18, 12), if (negative) word(0 -% n) else size } };
    }
    std.mem.sort(Leg, legs, {}, Leg.less);
    try e.push(word(32));
    if (!execute) {
        try e.amount("max_fee", .unsigned);
        try e.push(word(64));
    }
    try e.push(word(legs.len));
    for (legs, 0..) |leg, i| {
        if (i > 0 and std.mem.eql(u8, legs[i - 1].name, leg.name)) return error.DuplicateLeg;
        try e.words.appendSlice(e.a, &leg.fields);
    }
    if (execute) {
        const hash = evm.hash(std.mem.sliceAsBytes(e.words.items));
        e.words.clearRetainingCapacity();
        try e.push(hash);
        try e.amount("max_fee", .unsigned);
    }
}
fn encode(a: A, v: V) !Module {
    const kind = std.meta.stringToEnum(Kind, try text(v, "type")) orelse return error.InvalidModule;
    var e = Encoder{ .a = a, .body = v };
    const vault = "0x2885c174ebf5524aED9c721d60c12b1537685186";
    const contract: []const u8 = switch (kind) {
        .encoded => {
            const module = try text(v, "module");
            _ = try address(module);
            const bytes = try w.field(v, "data");
            if (bytes != .array) return error.InvalidModule;
            const payload = try a.alloc(u8, bytes.array.items.len);
            for (payload, bytes.array.items) |*out, n| {
                if (n != .number_string) return error.InvalidModule;
                out.* = try unsigned(u8, n);
            }
            return .{ .contract = module, .bytes = payload };
        },
        .trade => blk: {
            try e.addr("asset");
            try e.id(u256, "sub_id");
            try e.amount("limit_price", .signed);
            try e.amount("amount", .positive);
            if (e.words.items[3][0] & 0x80 != 0) return error.InvalidAmount;
            try e.amount("max_fee", .unsigned);
            try e.id(u256, "recipient_id");
            try e.boolean("is_bid");
            break :blk "0xB8D20c2B7a1Ad2EE33Bc50eF10876eD3035b5e7b";
        },
        .transfer => blk: {
            try e.id(u256, "to_subaccount_id");
            try e.id(u256, "new_subaccount_manager");
            try e.addr("asset");
            try e.id(u256, "sub_id");
            try e.amount("amount", .positive);
            try e.amount("max_fee_usd", .unsigned);
            if ((v.object.get("recipient") orelse V.null) != .null) {
                try e.addr("recipient");
                break :blk "0x8F9B8f12ddA05FB1F0DDDDe8f5af8cECF54f8aC9";
            }
            break :blk "0x01259207A40925b794C8ac320456F7F6c8FE2636";
        },
        .withdrawal => blk: {
            try e.addr("asset");
            try e.amount("max_fee_usd", .unsigned);
            try e.addr("recipient");
            try e.push(try scaled(try text(v, "amount"), .positive, try unsigned(u8, try w.field(v, "decimals")), 255));
            try e.boolean("force_batch");
            break :blk "0x9d0E8f5b25384C7310CB8C6aE32C8fbeb645d083";
        },
        .session_key => blk: {
            try e.addr("session_key");
            try e.id(u64, "expiry_sec");
            try e.pairOfArrays("scopes", "subaccount_ids", false);
            // Two scalar fields precede the offset table.
            for (e.words.items[2..4]) |*offset| offset.* = word(std.mem.readInt(u256, offset, .big) + 64);
            break :blk "0xe330CF64ff6EbF41699aad344Cb21d78db1D2bb6";
        },
        .whitelisted_recipients => blk: {
            try e.pairOfArrays("add", "remove", true);
            break :blk "0xB86D6DE1b76c9839e4BA860848CD98A1dABd6B54";
        },
        .rfq_quote, .rfq_execute => blk: {
            try rfq(&e, kind == .rfq_execute);
            break :blk "0x9371352CCef6f5b36EfDFE90942fFE622Ab77F1D";
        },
        .vault_create => blk: {
            try e.push(word(0));
            try e.id(u256, "manager_id");
            try e.addr("deposit_spot_asset");
            try e.amount("initial_deposit", .unsigned);
            for ([_][]const u8{ "management_fee_bps", "performance_fee_bps", "max_slippage_bps", "cooldown_sec" }) |k| try e.id(u32, k);
            try e.amount("max_fee_usd", .unsigned);
            try e.amount("initial_share_price_usd", .unsigned);
            const asset = v.object.get("benchmark_asset") orelse V.null;
            try e.push(if (asset == .null) word(0) else try address(try w.string(asset)));
            try e.push(word(@intFromBool(asset != .null)));
            break :blk vault;
        },
        .vault_deposit => blk: {
            try e.push(word(1));
            try e.id(u256, "vault_subaccount_id");
            try e.addr("deposit_spot_asset");
            try e.amount("amount", .positive);
            break :blk vault;
        },
        .vault_withdraw => blk: {
            try e.push(word(2));
            try e.id(u256, "vault_subaccount_id");
            try e.amount("shares_to_burn", .positive);
            break :blk vault;
        },
        .vault_cancel => blk: {
            try e.push(word(3));
            try e.id(u256, "vault_subaccount_id");
            break :blk vault;
        },
        .vault_mint, .vault_burn => blk: {
            const mint = kind == .vault_mint;
            try e.push(word(if (mint) 4 else 5));
            try e.amount("share_price", .unsigned);
            try e.push(try hex(32, try text(v, if (mint) "deposit_hash" else "withdraw_hash")));
            break :blk vault;
        },
        .vault_force_burn => blk: {
            try e.push(word(6));
            try e.addr("holder");
            break :blk vault;
        },
        .liquidation => blk: {
            try e.id(u256, "liquidate_subaccount_id");
            const pct = try scaled(try text(v, "percent_of_acc"), .positive, 18, 18);
            const n = std.mem.readInt(u256, &pct, .big);
            if (n > 1_000_000_000_000_000_000 or n % 10_000_000_000_000_000 != 0) return error.InvalidAmount;
            try e.push(pct);
            try e.amount("price_limit", .signed);
            break :blk "0x66d23e59DaEEF13904eFA2D4B8658aeD05f59a92";
        },
        .delete_subaccount => "0x1057cF9E6725a0db70932a4691f595068D9e7B30",
    };
    return .{ .contract = contract, .bytes = std.mem.sliceAsBytes(e.words.items) };
}
pub fn signAction(a: A, v: V) !V {
    const data = try encode(a, try w.field(v, "data"));
    const owner = try address(try text(v, "owner"));
    const chain = try unsigned(u64, try w.field(v, "chain_id"));
    if (chain != 1 and chain != 11155111) return error.InvalidNetwork;
    const domain = evm.hash(&(evm.hash("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)") ++ evm.hash("Matching") ++ evm.hash("1.0") ++ word(chain) ++ try address("0xeB8d770ec18DB98Db922E9D83260A585b9F0DeAD")));
    var key = try hex(32, try text(v, "key"));
    defer std.crypto.secureZero(u8, &key);
    const public = (try std.crypto.ecc.Secp256k1.basePoint.mul(key, .big)).toUncompressedSec1();
    const signer = evm.hash(public[1..])[12..32].*;
    if (v.object.get("signer")) |expected| {
        if (!std.mem.eql(u8, &try hex(20, try w.string(expected)), &signer)) return error.SignerMismatch;
    }
    const action = evm.hash(&(evm.hash("Action(uint256 subaccountId,uint256 nonce,address module,bytes data,uint256 expiry,address owner,address signer)") ++ word(try unsigned(u256, try w.field(v, "subaccount_id"))) ++ word(try unsigned(u256, try w.field(v, "nonce"))) ++ try address(data.contract) ++ evm.hash(data.bytes) ++ word(try unsigned(u64, try w.field(v, "expiry"))) ++ owner ++ @as([12]u8, @splat(0)) ++ signer));
    const digest = evm.hash(&([_]u8{ 0x19, 1 } ++ domain ++ action));
    const sig = try tx.recoverable(key, digest);
    var out: V = .{ .object = .empty };
    try out.object.put(a, "module", w.text(data.contract));
    try out.object.put(a, "encoded", w.text(try std.fmt.allocPrint(a, "0x{x}", .{data.bytes})));
    try out.object.put(a, "digest", w.text(try std.fmt.allocPrint(a, "0x{x}", .{&digest})));
    try out.object.put(a, "signature", w.text(try std.fmt.allocPrint(a, "0x{x}", .{&sig})));
    try out.object.put(a, "signer", w.text(try std.fmt.allocPrint(a, "0x{x}", .{&signer})));
    return out;
}

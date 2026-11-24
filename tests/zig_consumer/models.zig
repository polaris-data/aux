const std = @import("std");
const aux = @import("aux");

const compare = @import("compare.zig").compare;

fn check(comptime T: type, a: std.mem.Allocator, wire: aux.Value) !void {
    const original = try std.json.Stringify.valueAlloc(a, wire, .{});
    var parsed = try aux.parseAs(T, a, original);
    defer parsed.deinit();
    const model = parsed.value;
    const bytes = try std.json.Stringify.valueAlloc(a, model, .{});
    var result = try aux.parse(a, bytes);
    defer result.deinit();
    try compare(result.value, wire);
}

const Checker = *const fn (std.mem.Allocator, aux.Value, aux.Value) anyerror!void;
fn checker(comptime Request: type, comptime Response: type) Checker {
    return struct {
        fn run(a: std.mem.Allocator, req: aux.Value, res: aux.Value) anyerror!void {
            try check(Request, a, req);
            try check(Response, a, res);
        }
    }.run;
}

const Entry = struct { []const u8, Checker };
const entries = blk: {
    @setEvalBranchQuota(1000000);
    var count = 0;
    for (@typeInfo(aux.clients).@"struct".decl_names) |name| {
        const venue = @field(aux.clients, name);
        if (@hasDecl(venue, "routes")) count += venue.routes.len;
    }
    var result: [count]Entry = undefined;
    var i = 0;
    for (@typeInfo(aux.clients).@"struct".decl_names) |name| {
        const venue = @field(aux.clients, name);
        if (@hasDecl(venue, "routes")) for (venue.routes) |route| {
            const method = @typeInfo(@TypeOf(@field(venue.Client, route[1]))).@"fn";
            if (method.param_types[1] != route[2] or @typeInfo(method.return_type.?).error_union.payload != aux.Owned(route[3]))
                @compileError("SDK signature differs from the schema for " ++ name ++ "." ++ route[1]);
            result[i] = .{ name ++ "." ++ route[0], checker(route[2], route[3]) };
            i += 1;
        };
    }
    break :blk result;
};

pub fn main(init: std.process.Init) !void {
    @setEvalBranchQuota(2000000);
    const a = init.arena.allocator();
    const args = try init.minimal.args.toSlice(a);
    const bytes = try std.Io.Dir.cwd().readFileAlloc(init.io, args[1], a, .limited(64 * 1024 * 1024));
    var rows = try aux.parse(a, bytes);
    defer rows.deinit();
    var methods: std.StringHashMapUnmanaged(Checker) = .empty;
    for (entries) |entry| try methods.put(a, entry[0], entry[1]);
    for (rows.value.array.items) |row| {
        const group = row.object.get("group").?.string;
        const op = row.object.get("operation").?.string;
        const req = row.object.get("request").?;
        const res = row.object.get("response").?;
        var scratch = std.heap.ArenaAllocator.init(std.heap.page_allocator);
        defer scratch.deinit();
        errdefer std.debug.print("Failed model fixture {s}.{s}\n", .{ group, op });
        const key = try std.mem.concat(scratch.allocator(), u8, &.{ group, ".", op });
        const run = methods.get(key) orelse return error.UnknownOperation;
        try run(scratch.allocator(), req, res);
    }
    std.debug.print("Validated {d} request and response fixture pairs\n", .{rows.value.array.items.len});
}

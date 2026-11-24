// Contract interfaces pinned to the ABI fixtures.
pub const Venue = enum { metric, polymarket };
pub const Kind = enum { function, event, revert };
pub const Param = struct { ty: []const u8, name: []const u8, components: []const Param = &.{}, indexed: bool = false };
pub const Entry = struct { venue: Venue, contract: []const u8, kind: Kind, name: []const u8, signature: []const u8, inputs: []const Param, outputs: []const Param };

pub const entries =
    @import("../exchanges/metric/contracts.zig").entries ++
    @import("../exchanges/polymarket/contracts.zig").entries;

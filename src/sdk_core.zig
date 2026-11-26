// Implementation dependencies of the public Zig SDK.
pub const native = @import("core/native.zig");
pub const stream = @import("core/stream.zig");
pub const failure = @import("core/http_failure.zig");
pub const wire = @import("core/wire.zig");
pub const Dial = @import("core/host_stream.zig").Dial;
pub const transport = @import("core/transport.zig");
pub const dependencies = transport.dependencies;
pub fn exchangeVenue(name: []const u8) bool {
    return @import("core/exchange_stream.zig").venue(name) != null;
}
pub const exchangeWrite = @import("core/exchange_http.zig").isWrite;
pub const operationPrivate = @import("core/http.zig").operationPrivate;
pub const perpsWrite = @import("core/perps.zig").isWrite;
pub const venue = @import("core/http.zig").venue;
pub const reserve = @import("exchanges/pascal/permits.zig").reserve;
pub const monacoWrite = @import("exchanges/monaco/orders.zig").isWrite;
pub const rpc = @import("core/rpc.zig");
pub const hermes = @import("data/pyth/hermes.zig");
pub const encoding = @import("core/encoding.zig");

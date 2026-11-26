// HTTP operation descriptions pinned to the connector fixtures.
pub const Venue = enum { derive, extended, paradex, phoenix, kalshi, polymarket, pascal, birdeye, hyperdash, dune, tardis, hyperliquid, lighter, databento, pyth_hermes, pyth_pro, chainlink_streams, chainlink_candles, qfex, arcus, metric, monaco, injective, nado, ondo_perps, pacifica, paragon, trade_xyz, aster, perpl, risex, decibel, edgex, grvt, katana, variational, aevo, backpack, binance_spot, bybit, deribit, dydx_v4, hibachi, kraken_spot, okx, orderly, reya, kuru, apex_omni };
pub const Param = struct { header: []const u8 = "", key: []const u8, loc: enum { path, query, header, body, member }, required: bool, collection: []const u8 };
pub const Operation = struct { origin: []const u8 = "", extra_query: bool = false, body_limit: usize = 16 * 1024 * 1024, venue: Venue, name: []const u8, verb: []const u8, path: []const u8, private: bool, params: []const Param };

pub const operations =
    @import("../exchanges/aevo/routes.zig").operations ++
    @import("../exchanges/apex_omni/routes.zig").operations ++
    @import("../exchanges/backpack/routes.zig").operations ++
    @import("../exchanges/binance_spot/routes.zig").operations ++
    @import("../exchanges/bybit/routes.zig").operations ++
    @import("../exchanges/deribit/routes.zig").operations ++
    @import("../exchanges/dydx_v4/routes.zig").operations ++
    @import("../exchanges/hibachi/routes.zig").operations ++
    @import("../exchanges/kraken_spot/routes.zig").operations ++
    @import("../exchanges/kuru/routes.zig").operations ++
    @import("../exchanges/okx/routes.zig").operations ++
    @import("../exchanges/orderly/routes.zig").operations ++
    @import("../exchanges/reya/routes.zig").operations ++
    @import("../exchanges/perpl/routes.zig").operations ++
    @import("../exchanges/risex/routes.zig").operations ++
    @import("../exchanges/decibel/routes.zig").operations ++
    @import("../exchanges/edgex/routes.zig").operations ++
    @import("../exchanges/grvt/routes.zig").operations ++
    @import("../exchanges/katana/routes.zig").operations ++
    @import("../exchanges/variational/routes.zig").operations ++
    @import("../exchanges/arcus/routes.zig").operations ++
    @import("../exchanges/aster/routes.zig").operations ++
    @import("../exchanges/injective/routes.zig").operations ++
    @import("../exchanges/nado/routes.zig").operations ++
    @import("../exchanges/ondo_perps/routes.zig").operations ++
    @import("../exchanges/pacifica/routes.zig").operations ++
    @import("../exchanges/paragon/routes.zig").operations ++
    @import("../exchanges/trade_xyz/routes.zig").operations ++
    @import("../data/birdeye/routes.zig").operations ++
    @import("../data/chainlink/routes.zig").operations ++
    @import("../data/databento/routes.zig").operations ++
    @import("../exchanges/derive/routes.zig").operations ++
    @import("../data/dune/routes.zig").operations ++
    @import("../exchanges/extended/routes.zig").operations ++
    @import("../data/hyperdash/routes.zig").operations ++
    @import("../exchanges/hyperliquid/routes.zig").operations ++
    @import("../exchanges/kalshi/routes.zig").operations ++
    @import("../exchanges/lighter/routes.zig").operations ++
    @import("../exchanges/metric/routes.zig").operations ++
    @import("../exchanges/monaco/routes.zig").operations ++
    @import("../exchanges/paradex/routes.zig").operations ++
    @import("../exchanges/pascal/routes.zig").operations ++
    @import("../exchanges/phoenix/routes.zig").operations ++
    @import("../exchanges/polymarket/routes.zig").operations ++
    @import("../data/pyth/routes.zig").operations ++
    @import("../exchanges/qfex/routes.zig").operations ++
    @import("../data/tardis/routes.zig").operations;

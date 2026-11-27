const std = @import("std");
const perps = @import("perps.zig");
const hip3 = @import("../exchanges/hyperliquid/hip3.zig");
const market = @import("market.zig");
const exchange = @import("exchange_http.zig");
const routes = @import("routes.zig");
const wire = @import("wire.zig");
const A = wire.A;
const V = wire.V;
const Bytes = std.ArrayList(u8);
const eq = std.mem.eql;

pub fn scalar(v: V) ![]const u8 {
    return switch (v) {
        .string => v.string,
        .number_string => v.number_string,
        .bool => if (v.bool) "true" else "false",
        else => error.ExpectedScalar,
    };
}
pub fn escape(a: A, s: []const u8, form: bool) ![]const u8 {
    var out: Bytes = .empty;
    const digits = "0123456789ABCDEF";
    for (s) |b| {
        if (form and (b == ' ' or b == '*')) {
            try out.append(a, if (b == ' ') '+' else b);
            continue;
        }
        if (form and b == '~') {
            try out.appendSlice(a, "%7E");
            continue;
        }
        switch (b) {
            'a'...'z', 'A'...'Z', '0'...'9', '-', '.', '_', '~' => try out.append(a, b),
            else => try out.appendSlice(a, &.{ '%', digits[b >> 4], digits[b & 15] }),
        }
    }
    return out.items;
}
pub fn pair(a: A, out: *Bytes, key: []const u8, value: []const u8, form: bool) !void {
    try out.append(a, if (out.items.len == 0) '?' else '&');
    try out.appendSlice(a, try escape(a, key, form));
    try out.append(a, '=');
    try out.appendSlice(a, try escape(a, value, form));
}
fn query(a: A, out: *Bytes, p: routes.Param, v: V, form: bool) !void {
    if (v != .array) return pair(a, out, p.key, try scalar(v), form);
    if (eq(u8, p.collection, "json")) return pair(a, out, p.key, try std.json.Stringify.valueAlloc(a, v, .{}), form);
    if (eq(u8, p.collection, "multi")) {
        for (v.array.items) |item| try pair(a, out, p.key, try scalar(item), form);
    } else {
        const sep: u8 = if (eq(u8, p.collection, "ssv")) ' ' else if (eq(u8, p.collection, "tsv")) '\t' else if (eq(u8, p.collection, "pipes")) '|' else ',';
        var joined: Bytes = .empty;
        for (v.array.items, 0..) |item, i| {
            if (i != 0) try joined.append(a, sep);
            try joined.appendSlice(a, try scalar(item));
        }
        try pair(a, out, p.key, joined.items, form);
    }
}

fn keyLess(_: void, left: []const u8, right: []const u8) bool {
    return std.mem.lessThan(u8, left, right);
}

fn formFields(a: A, fields: V, out: *Bytes) !void {
    if (fields != .object) return error.ExpectedObject;
    const keys = try a.dupe([]const u8, fields.object.keys());
    std.mem.sort([]const u8, keys, {}, keyLess);
    for (keys) |key| {
        const value = fields.object.get(key).?;
        if (value == .null) continue;
        if (value == .array) for (value.array.items) |x| {
            if (x != .string and x != .number_string) return error.InvalidParameter;
        };
        try query(a, out, .{ .key = key, .loc = .query, .required = false, .collection = "csv" }, value, true);
    }
}

pub fn venue(input: V) !routes.Venue {
    return std.meta.stringToEnum(routes.Venue, try wire.string(try wire.field(input, "venue"))) orelse error.InvalidVenue;
}
pub const Framing = enum { grpc_injective, sse, objects, dbn, lines, bytes, download, hosted_feed, machine_feed, dataset };
pub fn streamFraming(v: routes.Venue, name: []const u8) ?Framing {
    if (v == .injective and eq(u8, name, "subscribe")) return .grpc_injective;
    if (v == .tardis) {
        if (eq(u8, name, "data_feeds")) return .hosted_feed;
        if (eq(u8, name, "machine_replay") or eq(u8, name, "machine_replay_normalized")) return .machine_feed;
        if (eq(u8, name, "dataset")) return .dataset;
    }
    if (v == .pyth_hermes and eq(u8, name, "stream_price_updates")) return .sse;
    if (v == .chainlink_candles and eq(u8, name, "stream_prices")) return .objects;
    if (v == .databento) {
        if (eq(u8, name, "get_range")) return .dbn;
        if (eq(u8, name, "download")) return .download;
        if (eq(u8, name, "timeseries")) return .bytes;
        for ([_][]const u8{ "security_master_get_range", "security_master_get_last", "corporate_actions_get_range", "adjustment_factors_get_range" }) |op| if (eq(u8, name, op)) return .lines;
    }
    return null;
}
pub const Format = enum { json, csv, ndjson, text };
pub fn hyperdashFormat(name: []const u8) !struct { base: []const u8, format: Format } {
    const format: Format = if (std.mem.endsWith(u8, name, "_csv")) .csv else if (std.mem.endsWith(u8, name, "_ndjson")) .ndjson else return .{ .base = name, .format = .json };
    const base = name[0 .. name.len - @tagName(format).len - 1];
    var rows = std.mem.tokenizeScalar(u8, @embedFile("../data/hyperdash/text-routes.txt"), '\n');
    while (rows.next()) |row| if (eq(u8, row, base)) return .{ .base = base, .format = format };
    return error.UnknownOperation;
}
pub fn responseFormat(v: routes.Venue, name: []const u8) !Format {
    if (v == .hyperdash) return (try hyperdashFormat(name)).format;
    if (v == .dune and (eq(u8, name, "get_execution_results_csv") or eq(u8, name, "get_latest_query_results_csv"))) return .csv;
    if ((v == .pyth_pro or v == .pyth_hermes) and (eq(u8, name, "live") or eq(u8, name, "ready"))) return .text;
    return .json;
}
fn lookup(input: V) !routes.Operation {
    const v = try venue(input);
    const name = try wire.string(try wire.field(input, "operation"));
    const base = if (v == .hyperdash) (try hyperdashFormat(name)).base else name;
    for (&routes.operations) |*op| if (op.venue == v and eq(u8, op.name, base)) return op.*;
    return error.UnknownOperation;
}

pub fn operationPrivate(input: V) !bool {
    return (try lookup(input)).private;
}

pub const Request = struct {
    method: []const u8,
    path: []const u8,
    headers: V,
    body: V,
    authenticated: bool,
    origin: []const u8,
    pub fn json(req: Request, a: A) !V {
        var out: V = .{ .object = .empty };
        try out.object.put(a, "method", wire.text(req.method));
        try out.object.put(a, "path", wire.text(req.path));
        try out.object.put(a, "headers", req.headers);
        try out.object.put(a, "body", req.body);
        try out.object.put(a, "authenticated", .{ .bool = req.authenticated });
        return out;
    }
};
pub fn oracleVenue(v: routes.Venue) bool {
    return v == .pyth_hermes or v == .pyth_pro or v == .chainlink_streams or v == .chainlink_candles;
}

fn feedId(v: V) !void {
    const s = try wire.string(v);
    const hex = if (std.mem.startsWith(u8, s, "0x")) s[2..] else s;
    if (hex.len != 64) return error.InvalidFeed;
    for (hex) |b| if (!std.ascii.isHex(b)) return error.InvalidFeed;
}
pub fn prepare(a: A, input: V) !Request {
    const operation_name = try wire.string(try wire.field(input, "operation"));
    if (try venue(input) == .tardis and streamFraming(.tardis, operation_name) != null) return @import("../data/tardis/replay.zig").prepare(a, input, operation_name, try wire.field(input, "params"));
    const op = try lookup(input);
    var fields = try wire.field(input, "params");
    if (fields != .object) return error.ExpectedObject;
    const hidden = fields.object.get("hidden");
    const private = op.private or (op.venue == .chainlink_streams and hidden != null and hidden.? == .bool and hidden.?.bool);
    if (op.venue == .pyth_hermes or op.venue == .chainlink_streams or op.venue == .chainlink_candles) {
        for ([_][]const u8{ "ids", "feedID", "feedIDs", "feedId" }) |k| if (fields.object.get(k)) |v| {
            if (v == .null) continue;
            if (v == .array) {
                if (v.array.items.len == 0) return error.InvalidFeed;
                for (v.array.items) |id| try feedId(id);
            } else try feedId(v);
        };
    }
    if (op.venue == .chainlink_candles and eq(u8, op.name, "stream_prices")) {
        const symbols = fields.object.get("symbol") orelse .null;
        const ids = fields.object.get("feedId") orelse .null;
        if (symbols == .null and ids == .null) return error.MissingField;
        if (ids != .null and ids != .array) return error.InvalidParameter;
        if (symbols != .null) {
            if (symbols != .array) return error.InvalidParameter;
            for (symbols.array.items) |sym| _ = try wire.string(sym);
        }
    }
    const prediction = switch (op.venue) {
        .kalshi, .pascal, .polymarket => true,
        else => false,
    };
    const data = op.venue == .birdeye or op.venue == .hyperdash or op.venue == .tardis or op.venue == .dune or op.venue == .databento or oracleVenue(op.venue);
    const form = prediction or data or op.venue == .metric;
    var headers: V = .{ .object = .empty };
    if (data) {
        const format = try responseFormat(op.venue, try wire.string(try wire.field(input, "operation")));
        try headers.object.put(a, "accept", wire.text(switch (format) {
            .json, .text => "application/json",
            .csv => "text/csv",
            .ndjson => "application/x-ndjson",
        }));
        if (op.venue == .birdeye) try headers.object.put(a, "x-chain", wire.text("solana"));
    } else if (op.venue != .lighter and op.venue != .qfex and op.venue != .arcus) try headers.object.put(a, if (prediction or op.venue == .hyperliquid or op.venue == .metric or perps.isVenue(op.venue)) "content-type" else "Content-Type", wire.text("application/json"));
    if (op.venue == .phoenix or op.venue == .extended or op.venue == .monaco or perps.isVenue(op.venue)) try headers.object.put(a, "User-Agent", wire.text("aux-zig/0.0.2"));
    if (op.venue == .arcus and op.private) {
        try headers.object.put(a, "content-type", wire.text("application/json"));
        const order = try @import("../exchanges/arcus/trading.zig").order(a, eq(u8, op.name, "place_order"), fields);
        return .{ .method = op.verb, .path = op.path, .headers = headers, .body = order.body, .authenticated = true, .origin = op.origin };
    }
    if (op.venue == .monaco and !eq(u8, op.verb, "GET")) {
        var request = Request{ .method = op.verb, .path = op.path, .headers = headers, .body = .null, .authenticated = op.private, .origin = op.origin };
        try @import("../exchanges/monaco/orders.zig").prepare(a, op.name, fields, &request);
        return request;
    }
    var body: V = .null;
    var path = op.path;
    var q: Bytes = .empty;
    if (op.venue == .nado) {
        if (@import("../exchanges/nado/trading.zig").queryType(op.name)) |kind| try pair(a, &q, "type", kind, false);
    }
    if (op.venue == .qfex) try pair(a, &q, "api_version", "2026-10-29", false);
    if (op.venue == .qfex or op.venue == .arcus or op.venue == .metric or op.venue == .monaco) try market.validateSpot(a, op.venue, op.name, &fields);
    if (op.venue == .dune) try market.dune(fields);
    if (op.venue == .lighter) try market.lighter(op.name, fields);
    if (op.venue == .databento) {
        if (eq(u8, op.name, "download")) {
            const url = try wire.string(try wire.field(fields, "url"));
            const target = try std.Uri.parse(url);
            if (target.host == null or target.host.?.isEmpty() or target.user != null or target.password != null or target.fragment != null or
                !(eq(u8, target.scheme, "https") or eq(u8, target.scheme, "http"))) return error.InvalidDownloadUrl;
            for (url) |b| if (b <= 0x20 or b == 0x7f or b == 0x5c) return error.InvalidDownloadUrl;
            return .{ .method = "GET", .path = url, .body = .null, .headers = headers, .authenticated = true, .origin = op.origin };
        }
        if (streamFraming(op.venue, op.name)) |framing| {
            if (framing == .bytes) {
                try headers.object.put(a, "accept", wire.text("application/octet-stream"));
            } else if (framing == .dbn) {
                try fields.object.put(a, "encoding", wire.text("dbn"));
                try fields.object.put(a, "compression", wire.text("none"));
                try headers.object.put(a, "accept", wire.text("application/octet-stream"));
            } else {
                const compression = fields.object.get("compression") orelse wire.text("none");
                const name = try wire.string(compression);
                if (!eq(u8, name, "none") and !eq(u8, name, "zstd")) return error.InvalidCompression;
                try fields.object.put(a, "compression", compression);
            }
        }
        try formFields(a, fields, &q);
        if (eq(u8, op.verb, "POST")) {
            body = wire.text(if (q.items.len > 0) q.items[1..] else "");
            q = .empty;
            try headers.object.put(a, "content-type", wire.text("application/x-www-form-urlencoded"));
        }
    } else if (op.venue == .hyperliquid) {
        body = if (op.private) try @import("../exchanges/hyperliquid/trading.zig").action(a, op.name, fields) else try market.hyperliquid(a, op.name, fields);
    } else if (hip3.dex(op.venue)) |dex| {
        body = try hip3.request(a, dex, op.name, fields);
    } else if (op.venue == .derive) {
        body = fields;
    } else {
        const has_range = fields.object.get("range_ms") != null and fields.object.get("range_ms").? != .null;
        for (op.params) |p| {
            const it = fields.object.fetchSwapRemove(p.key);
            if (it == null or it.?.value == .null) {
                if (p.required) return error.MissingField;
                continue;
            }
            const value = it.?.value;
            switch (p.loc) {
                .body => body = value,
                .member => {
                    if (body == .null) body = .{ .object = .empty };
                    try body.object.put(a, p.key, value);
                },
                .path => {
                    const s = try scalar(value);
                    if (s.len == 0 or eq(u8, s, ".") or eq(u8, s, "..") or
                        ((op.venue == .extended or op.venue == .phoenix) and std.mem.findAny(u8, s, "/\\") != null)) return error.InvalidPath;
                    const placeholder = try std.fmt.allocPrint(a, "{{{s}}}", .{p.key});
                    path = try std.mem.replaceOwned(u8, a, path, placeholder, try escape(a, s, false));
                },
                .header => {
                    const s = try scalar(value);
                    for (s) |char| if ((char < 0x20 and char != '\t') or char == 0x7f) return error.InvalidHeader;
                    if (op.venue == .hyperdash and (s.len == 0 or (eq(u8, p.key, "idempotency_key") and s.len > 255))) return error.InvalidHeader;
                    try headers.object.put(a, if (p.header.len > 0) p.header else p.key, wire.text(s));
                },
                .query => {
                    var descriptor = p;
                    if (op.venue == .pyth_hermes and eq(u8, p.key, "ids")) descriptor.key = "ids[]";
                    try query(a, &q, descriptor, value, form);
                },
            }
        }
        if (op.extra_query) if (fields.object.fetchSwapRemove("query")) |extra| {
            if (extra.value != .object) return error.ExpectedObject;
            var pairs = extra.value.object.iterator();
            while (pairs.next()) |entry| {
                const k = entry.key_ptr.*;
                if (eq(u8, k, "route") or eq(u8, k, "range_ms") or (has_range and eq(u8, k, "start_time"))) return error.InvalidQuery;
                try pair(a, &q, k, try scalar(entry.value_ptr.*), true);
            }
        };
        if (data and body != .null) {
            if ((try std.json.Stringify.valueAlloc(a, body, .{})).len > op.body_limit) return error.BodyTooLarge;
            if (op.venue == .chainlink_candles and eq(u8, op.name, "authorize")) {
                var payload: Bytes = .empty;
                try formFields(a, body, &payload);
                body = wire.text(if (payload.items.len > 0) payload.items[1..] else "");
                try headers.object.put(a, "content-type", wire.text("application/x-www-form-urlencoded"));
            } else try headers.object.put(a, "content-type", wire.text("application/json"));
        }
        if (op.venue == .tardis and (eq(u8, op.name, "symbols") or eq(u8, op.name, "instruments"))) {
            var iter = fields.object.iterator();
            while (iter.next()) |entry| {
                const value = entry.value_ptr.*;
                if (value == .null) continue;
                const s = if (value == .array or value == .object) try std.json.Stringify.valueAlloc(a, value, .{}) else try scalar(value);
                try pair(a, &q, entry.key_ptr.*, s, true);
            }
        } else if (fields.object.count() > 0) {
            if (body != .object or form) return error.UnknownParameter;
            var iter = fields.object.iterator();
            while (iter.next()) |entry| try body.object.put(a, entry.key_ptr.*, entry.value_ptr.*);
        }
    }
    if (op.venue == .ondo_perps and eq(u8, op.name, "open_orders")) try pair(a, &q, "status", "open", false);
    if (op.venue == .orderly and (eq(u8, op.name, "market_summary") or eq(u8, op.name, "market_trades"))) {
        try perps.put(&body, a, "type", wire.text(if (eq(u8, op.name, "market_summary")) "marketSummary" else "marketTrades"));
    }
    if (op.venue == .orderly and eq(u8, op.name, "open_orders")) try pair(a, &q, "status", "INCOMPLETE", false);
    if (op.venue == .dydx_v4 and eq(u8, op.name, "open_orders")) try pair(a, &q, "status", "OPEN", false);
    const origin = if (perps.isVenue(op.venue)) try perps.origin(op.venue, op.name, input, op.origin) else if (exchange.supports(op.venue)) try exchange.origin(op.venue, op.name, input, op.origin) else op.origin;
    return .{ .method = op.verb, .path = try std.mem.concat(a, u8, &.{ path, q.items }), .body = body, .headers = headers, .authenticated = private, .origin = origin };
}

pub fn decode(a: A, v: routes.Venue, status: u16, body: []const u8) !V {
    return decodeBody(a, v, status, body) catch |err| {
        if (err == error.ExchangeApi) @import("http_failure.zig").record(@tagName(v), status, body);
        return err;
    };
}
fn decodeBody(a: A, v: routes.Venue, status: u16, body: []const u8) !V {
    if (status < 200 or status >= 300) return error.ExchangeApi;
    const result: V = if (body.len == 0 and v != .derive) .null else try wire.parse(a, body);
    if (exchange.supports(v)) return exchange.validateResponse(v, result);
    if (oracleVenue(v)) {
        if (result == .object) if (result.object.get("s")) |s| if (s == .string and eq(u8, s.string, "error")) return error.ExchangeApi;
        return result;
    }
    if (v == .arcus or v == .qfex) {
        if (result == .object and (result.object.contains("error") or (v == .qfex and result.object.contains("error_code")))) return error.ExchangeApi;
        return result;
    }
    if (v == .birdeye) {
        const success = try wire.field(result, "success");
        if (success != .bool) return error.InvalidResponse;
        if (!success.bool) return error.ExchangeApi;
        return result;
    }
    if (v == .hyperdash or v == .tardis or v == .dune or v == .databento) return result;
    if (v == .lighter) {
        if (result == .object) if (result.object.get("code")) |code| {
            if (code == .number_string and !eq(u8, code.number_string, "200")) return error.ExchangeApi;
        };
        return result;
    }
    if (perps.isVenue(v)) {
        if (perps.replyError(v, result)) return error.ExchangeApi;
        return result;
    }
    if (v == .hyperliquid) {
        if (result == .string) return error.ExchangeApi;
        if (result == .object) if (result.object.get("status")) |s| {
            if (s == .string and eq(u8, s.string, "err")) return error.ExchangeApi;
        };
        return result;
    }
    if (v == .kalshi or v == .polymarket or v == .pascal) {
        if (result == .object) if (result.object.get("status")) |s| {
            if (s == .string and eq(u8, s.string, "error")) return error.ExchangeApi;
        };
        return result;
    }
    if (result == .object) {
        if (result.object.get("error")) |err| if (err != .null) return error.ExchangeApi;
        if (v == .extended) if (result.object.get("status")) |s| {
            if (s == .string and eq(u8, s.string, "ERROR")) return error.ExchangeApi;
        };
    }
    return if (v == .derive) try wire.field(result, "result") else result;
}

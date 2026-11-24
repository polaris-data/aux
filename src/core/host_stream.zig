// An application transport returns a connected plaintext socket. For TLS URLs
// the application drives TLS and forwards plaintext over its own socket pair.
// Numeric loopback is used solely to instantiate std.http's connection reader
// and writer. netConnectIp consumes the supplied descriptor, with no OS dial.
const std = @import("std");
const Io = std.Io;
const net = Io.net;
pub const Dial = extern struct {
    context: *anyopaque,
    connect: *const fn (*anyopaque, [*]const u8, usize, u16, bool, *c_int) callconv(.c) u32,
};

pub const Runtime = struct {
    threaded: Io.Threaded = .init_single_threaded,
    dial: ?Dial = null,
    supplied: std.atomic.Value(c_int) = .init(-1),
    pub fn io(self: *Runtime) Io {
        const original = self.threaded.io();
        return if (self.dial != null) .{ .userdata = original.userdata, .vtable = &delegated } else original;
    }
};
const delegated: Io.VTable = blk: {
    var threaded: Io.Threaded = .init_single_threaded;
    var table = threaded.io().vtable.*;
    table.netConnectIp = takeConnection;
    break :blk table;
};
fn runtime(userdata: ?*anyopaque) *Runtime {
    const threaded: *Io.Threaded = @ptrCast(@alignCast(userdata.?));
    return @fieldParentPtr("threaded", threaded);
}
fn takeConnection(userdata: ?*anyopaque, address: *const net.IpAddress, _: net.IpAddress.ConnectOptions) net.IpAddress.ConnectError!net.Socket {
    const fd = runtime(userdata).supplied.swap(-1, .acq_rel);
    if (fd < 0) return error.ConnectionRefused;
    return .{ .handle = fd, .address = address.* };
}
pub fn request(client: *std.http.Client, method: std.http.Method, uri: std.Uri, options: std.http.Client.RequestOptions) !std.http.Client.Request {
    if (client.io.vtable != &delegated) return client.request(method, uri, options) catch error.Network;
    const r = runtime(client.io.userdata);
    const dial = r.dial.?;
    var host_buf: [net.HostName.max_len]u8 = undefined;
    const host = try net.HostName.fromUri(uri, &host_buf);
    const tls = std.mem.eql(u8, uri.scheme, "https");
    const port = uri.port orelse @as(u16, if (tls) 443 else 80);
    var fd: c_int = -1;
    switch (dial.connect(dial.context, host.bytes.ptr, host.bytes.len, port, tls, &fd)) {
        0 => {},
        1 => return error.UnsupportedTransport,
        5 => return error.AuthenticationRequired,
        else => return error.Network,
    }
    if (fd < 0) return error.Network;
    r.supplied.store(fd, .release);
    defer {
        const unused = r.supplied.swap(-1, .acq_rel);
        if (unused >= 0) _ = std.c.close(unused);
    }
    const conn = client.connectTcpOptions(.{ .host = .{ .bytes = "127.0.0.1" }, .port = port, .protocol = .plain, .proxied_host = host }) catch return error.Network;
    errdefer {
        conn.closing = true;
        client.connection_pool.release(conn, client.io);
    }
    var opts = options;
    opts.connection = conn;
    client.now = Io.Clock.real.now(client.io);
    return client.request(method, uri, opts) catch error.Network;
}

const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const core = b.createModule(.{
        .root_source_file = b.path("src/abi.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
        .pic = true,
    });
    const protocol = b.createModule(.{
        .root_source_file = b.path("src/sdk_core.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    _ = b.addModule("aux", .{
        .root_source_file = b.path("sdk/zig/root.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
        .imports = &.{.{ .name = "aux_core", .module = protocol }},
    });
    const lib = b.addLibrary(.{ .name = "aux_zig_core", .linkage = .static, .root_module = core });
    lib.bundle_compiler_rt = true;
    lib.installHeadersDirectory(b.path("include"), "aux", .{});
    b.installArtifact(lib);
    const tests = b.addRunArtifact(b.addTest(.{ .root_module = core }));
    b.step("test", "Run native protocol tests").dependOn(&tests.step);
}

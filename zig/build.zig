const std = @import("std");

pub fn build(b: *std.Build) void {
    const core = b.addModule("aux", .{
        .root_source_file = b.path("src/root.zig"),
        .target = b.standardTargetOptions(.{}),
        .optimize = b.standardOptimizeOption(.{}),
        .link_libc = true,
        .pic = true,
    });
    const lib = b.addLibrary(.{ .name = "aux_zig_core", .linkage = .static, .root_module = core });
    lib.bundle_compiler_rt = true;
    lib.installHeadersDirectory(b.path("include"), "aux", .{});
    b.installArtifact(lib);
    const tests = b.addRunArtifact(b.addTest(.{ .root_module = core }));
    b.step("test", "Run native protocol tests").dependOn(&tests.step);
}

const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const dependency = b.dependency("aux", .{ .target = target, .optimize = optimize });
    inline for (.{ "zig-consumer", "zig-models" }, .{ "main.zig", "models.zig" }) |name, source| {
        const exe = b.addExecutable(.{
            .name = name,
            .root_module = b.createModule(.{
                .root_source_file = b.path(source),
                .target = target,
                .optimize = optimize,
                .imports = &.{.{ .name = "aux", .module = dependency.module("aux") }},
            }),
        });
        b.installArtifact(exe);
    }
}

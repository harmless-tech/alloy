const std = @import("std");

const NMod = struct {
    []const u8,
    *std.Build.Module,
};

pub fn build(b: *std.Build) void {
    const name = "allot";
    // const root_source_file = b.path("src/root.zig");

    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const strip = optimize != .Debug;

    // Define Modules
    const modules = [_]NMod {
        .{ "allot", b.addModule("allot", .{
            .root_source_file = b.path("src/root.zig"),
            .target = target,
            .optimize = optimize,
            .strip = strip,
        })},
        .{ "allot_asm", b.addModule("allot_asm", .{
            .root_source_file = b.path("lib/asm/src/root.zig"),
            .target = target,
            .optimize = optimize,
            .strip = strip,
        })},
        .{ "allot_bytecode", b.addModule("allot_bytecode", .{
            .root_source_file = b.path("lib/bytecode/src/root.zig"),
            .target = target,
            .optimize = optimize,
            .strip = strip,
        })},
        .{ "allot_runtime", b.addModule("allot_runtime", .{
            .root_source_file = b.path("lib/runtime/src/root.zig"),
            .target = target,
            .optimize = optimize,
            .strip = strip,
        })},
        .{ "allot_std", b.addModule("allot_std", .{
            .root_source_file = b.path("lib/std/src/root.zig"),
            .target = target,
            .optimize = optimize,
            .strip = strip,
        })},
    };

    // Define Module Imports
    for (modules) |module| {
        if(std.mem.eql(u8, module[0], "allot_bytecode")) {
            for (modules) |imod| {
                if(std.mem.eql(u8, imod[0], "allot")) {
                    module[1].addImport(imod[0], imod[1]);
                }
            }
        }
        else if(std.mem.eql(u8, module[0], "allot_runtime")) {
            for (modules) |imod| {
                if(std.mem.eql(u8, imod[0], "allot_bytecode")) {
                    module[1].addImport(imod[0], imod[1]);
                }
            }
        }
    }

    const lib = b.addStaticLibrary(.{
        .name = name,
        .root_source_file = b.path("src/lib.zig"),
        .target = target,
        .optimize = optimize,
        .strip = strip,
    });
    for (modules) |module| {
        lib.root_module.addImport(module[0], module[1]);
    }

    b.installArtifact(lib);

    const dylib = b.addSharedLibrary(.{
        .name = name,
        .root_source_file = b.path("src/lib.zig"),
        .target = target,
        .optimize = optimize,
        .strip = strip,
    });
    for (modules) |module| {
        dylib.root_module.addImport(module[0], module[1]);
    }

    b.installArtifact(dylib);

    const exe = b.addExecutable(.{
        .name = name,
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
        .strip = strip,
    });
    for (modules) |module| {
        exe.root_module.addImport(module[0], module[1]);
    }

    b.installArtifact(exe);

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);

    const lib_unit_tests = b.addTest(.{
        .root_source_file = b.path("src/lib.zig"),
        .target = target,
        .optimize = optimize,
    });
    for (modules) |imod| {
        lib_unit_tests.root_module.addImport(imod[0], imod[1]);
    }

    const run_lib_unit_tests = b.addRunArtifact(lib_unit_tests);

    const exe_unit_tests = b.addTest(.{
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });
    for (modules) |module| {
        exe_unit_tests.root_module.addImport(module[0], module[1]);
    }

    const run_exe_unit_tests = b.addRunArtifact(exe_unit_tests);

    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_lib_unit_tests.step);
    test_step.dependOn(&run_exe_unit_tests.step);

    // Add all modules tests
    for (modules) |module| {
        const mod_unit_tests = b.addTest(.{
            .root_source_file = module[1].root_source_file.?,
            .target = target,
            .optimize = optimize,
        });
        for (modules) |imod| {
            mod_unit_tests.root_module.addImport(imod[0], imod[1]);
        }

        const run_mod_unit_tests = b.addRunArtifact(mod_unit_tests);

        test_step.dependOn(&run_mod_unit_tests.step);
    }
}

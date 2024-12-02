const std = @import("std");

const Mods = enum {
    liballot,
    allot,
    allot_asm,
    allot_bytecode,
    allot_runtime,
    allot_std,

    const Self = @This();

    inline fn str(self: Self) [:0]const u8 {
        return switch (self) {
            .liballot => "liballot",
            .allot => "allot",
            .allot_asm => "allot_asm",
            .allot_bytecode => "allot_bytecode",
            .allot_runtime => "allot_runtime",
            .allot_std => "allot_std",
        };
    }
};

const NMod = struct {
    Mods,
    *std.Build.Module,
};

inline fn add_imports(module: *const NMod, modules: []const NMod) void {
    for (modules) |imod| {
        if (module[0] != imod[0]) module[1].addImport(imod[0].str(), imod[1]);
    }
}

inline fn add_all_imports_to_module(module: *std.Build.Module, modules: []const NMod) void {
    for (modules) |mod| {
        module.addImport(mod[0].str(), mod[1]);
    }
}

pub fn build(b: *std.Build) void {
    const name = "allot";

    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const strip = optimize != .Debug;

    // Define Modules
    const mod_liballot: NMod = .{ .liballot, b.addModule(Mods.liballot.str(), .{
        .root_source_file = b.path("src/root.zig"),
        .target = target,
        .optimize = optimize,
        .strip = strip,
    })};
    const mod_allot: NMod = .{ .allot, b.addModule(Mods.allot.str(), .{
        .root_source_file = b.path("lib/allot/src/root.zig"),
        .target = target,
        .optimize = optimize,
        .strip = strip,
    })};
    const mod_allot_asm: NMod = .{ .allot_asm, b.addModule(Mods.allot_asm.str(), .{
        .root_source_file = b.path("lib/asm/src/root.zig"),
        .target = target,
        .optimize = optimize,
        .strip = strip,
    })};
    const mod_allot_bytecode: NMod = .{ .allot_bytecode, b.addModule(Mods.allot_bytecode.str(), .{
        .root_source_file = b.path("lib/bytecode/src/root.zig"),
        .target = target,
        .optimize = optimize,
        .strip = strip,
    })};
    const mod_allot_runtime: NMod = .{ .allot_runtime, b.addModule(Mods.allot_runtime.str(), .{
        .root_source_file = b.path("lib/runtime/src/root.zig"),
        .target = target,
        .optimize = optimize,
        .strip = strip,
    })};
    const mod_allot_std: NMod = .{ .allot_std, b.addModule(Mods.allot_std.str(), .{
        .root_source_file = b.path("lib/std/src/root.zig"),
        .target = target,
        .optimize = optimize,
        .strip = strip,
    })};

    const modules = [_]NMod {
        mod_liballot,
        mod_allot,
        mod_allot_asm,
        mod_allot_bytecode,
        mod_allot_runtime,
        mod_allot_std,
    };

    // All modules should import self!
    add_imports(&mod_liballot, &.{ mod_liballot });
    add_imports(&mod_allot, &.{ mod_allot });
    add_imports(&mod_allot_asm, &.{ mod_allot_asm });
    add_imports(&mod_allot_bytecode, &.{ mod_allot_bytecode });
    add_imports(&mod_allot_runtime, &.{ mod_allot_runtime });
    add_imports(&mod_allot_std, &.{ mod_allot_std });

    // Define Module Imports
    add_imports(&mod_liballot, &modules);
    add_imports(&mod_allot_bytecode, &.{ mod_allot });

    const lib = b.addStaticLibrary(.{
        .name = name,
        .root_source_file = b.path("src/root.zig"),
        .target = target,
        .optimize = optimize,
        .strip = strip,
    });
    add_all_imports_to_module(&lib.root_module, &modules);

    b.installArtifact(lib);

    const dylib = b.addSharedLibrary(.{
        .name = name,
        .root_source_file = b.path("src/root.zig"),
        .target = target,
        .optimize = optimize,
        .strip = strip,
    });
    add_all_imports_to_module(&dylib.root_module, &modules);

    b.installArtifact(dylib);

    const exe = b.addExecutable(.{
        .name = name,
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
        .strip = strip,
    });
    add_all_imports_to_module(&exe.root_module, &modules);

    b.installArtifact(exe);

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);

    const exe_unit_tests = b.addTest(.{
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });
    add_all_imports_to_module(&exe_unit_tests.root_module, &modules);

    const run_exe_unit_tests = b.addRunArtifact(exe_unit_tests);

    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_exe_unit_tests.step);

    // Add all modules tests
    var test_mod_list = std.ArrayList(NMod).init(b.allocator);

    for (modules) |module| {
        const mod_unit_tests = b.addTest(.{
            .name = module[0].str(),
            .root_source_file = module[1].root_source_file.?,
            .target = target,
            .optimize = optimize,
        });

        // Rewrite modules to self
        const new_mod: NMod = .{ module[0], &mod_unit_tests.root_module };
        test_mod_list.append(new_mod) catch @panic("OOM");

        const run_mod_unit_tests = b.addRunArtifact(mod_unit_tests);

        test_step.dependOn(&run_mod_unit_tests.step);
    }

    const test_mod_slice = test_mod_list.toOwnedSlice() catch @panic("OOM");
    for (test_mod_slice) |tmod| {
        for (test_mod_slice) |imod| {
            tmod[1].addImport(imod[0].str(), imod[1]);
        }
    }
}

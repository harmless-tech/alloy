const std = @import("std");
const testing = std.testing;

const liballot = @import("liballot");
const allot = liballot.allot;
const bytecode = liballot.abytecode;

pub fn main() !void {
    // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
    std.debug.print("All your {s} are belong to us.\n", .{"codebase"});

    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    try stdout.print("Run `zig build test` to run the tests.\n", .{});

    try bw.flush(); // don't forget to flush!

    // const uint: u32 = 0xFFAA3490;
    // const i: bytecode.Instruction = @bitCast(uint);
    // std.debug.print("WTF is {}\n", .{ i.b0 });

    // const instruct: bytecode.Instruction = .{
    //     .b3 = 0xFF,
    //     .b2 = 0xAA,
    //     .b1 = 0x34,
    //     .b0 = 0x90,
    // };
    // const uii: u32 = @bitCast(instruct);
    // std.debug.print("WTF is {}\n", .{ uii });
}

test "all in module" {
    // _ = @import(".zig");

    std.testing.refAllDeclsRecursive(@This());
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}

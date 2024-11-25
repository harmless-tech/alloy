const std = @import("std");
const testing = std.testing;

const bytecode = @import("allot_bytecode");
const runtime = @import("allot_runtime");

pub fn add(a: i32, b: i32) i32 {
    return a + b;
}

test "basic add functionality" {
    try testing.expect(add(3, 7) == 10);
}

const std = @import("std");
const testing = std.testing;

const allot = @import("allot");
const runtime = @import("allot_runtime");

pub fn trytry() void {
    _ = allot.add(1, 333);
}

test {
    try testing.expect(runtime.sub(1, 1) == 0);
    try testing.expect(allot.add(1, 2) == 3);
}

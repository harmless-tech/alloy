const std = @import("std");
const testing = std.testing;

test "all in module" {
    // _ = @import(".zig");

    std.testing.refAllDeclsRecursive(@This());
}

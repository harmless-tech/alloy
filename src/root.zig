pub const allot = @import("allot");
pub const aasm = @import("allot_asm");
pub const abytecode = @import("allot_bytecode");
pub const aruntime = @import("allot_runtime");
pub const astd = @import("allot_std");

const std = @import("std");
const testing = std.testing;

test "all in module" {
    // _ = @import(".zig");

    std.testing.refAllDeclsRecursive(@This());
}

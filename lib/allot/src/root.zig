const std = @import("std");
const testing = std.testing;

pub const Byte = packed struct {
    b0: u8,

    pub const len = 1;
};
pub const Halfword = packed struct {
    b1: u8,
    b0: u8,

    pub const len = 2;
};
pub const Word = packed struct {
    b3: u8,
    b2: u8,
    b1: u8,
    b0: u8,

    pub const len = 4;
};
pub const Doubleword = packed struct {
    b7: u8,
    b6: u8,
    b5: u8,
    b4: u8,
    b3: u8,
    b2: u8,
    b1: u8,
    b0: u8,

    pub const len = 8;
};
pub const Quadword = packed struct {
    b15: u8,
    b14: u8,
    b13: u8,
    b12: u8,
    b11: u8,
    b10: u8,
    b9: u8,
    b8: u8,
    b7: u8,
    b6: u8,
    b5: u8,
    b4: u8,
    b3: u8,
    b2: u8,
    b1: u8,
    b0: u8,

    pub const len = 16;
};

test "all in module" {
    // _ = @import(".zig");

    std.testing.refAllDeclsRecursive(@This());
}

const std = @import("std");
const testing = std.testing;

//
// ~~~Because of Fat Instructions, it is probably best to do:
// OP_PAGE, OPCODE, LE1, LE0 - Normal
// OP_PAGE, OPCODE, LE5, LE4, LE3, LE2, LE1, LE0 - Fat~~~
//
// LE1, LE0, OP_PAGE, OPCODE - Normal
// LE1, LE0, OP_PAGE, OPCODE, LE5, LE4, LE3, LE2 - Fat
//
// OPCODE determines instruction type!!!
// [4]u8 -> @bitCast -> Instruction!
//
// All Instuctions are 4 byte aligned!!!
// For Jump instuctions:
// Encode imm as signed factor of 4 so that
// jump to [pc + (imm << 2)]
//

/// RI Type instruction.
pub const InstructionTypeI = packed struct {
    imm: u17, // 0xFF, 0xFF, 0x01, 0x00
    rd: u5, // 0x00, 0x00, 0x3E, 0x00
    op_page: u3, // 0x00, 0x00, 0xC0, 0x01
    opcode: u7, // 0x00, 0x00, 0x00, 0xFE
};

pub const InstructionTypeTemp = packed struct {
    imm: u5, //
    rd: u3, //
    op_page: u17, //
    opcode: u7, //
};

comptime {
    const assert = std.debug.assert;

    // Asserts all instructions sizes are right.
    assert(@bitSizeOf(InstructionTypeI) == @bitSizeOf(u32));
    assert(@bitSizeOf(InstructionTypeTemp) == @bitSizeOf(u32));
}

test "temp" {
    const expect = testing.expect;
    const max = std.math.maxInt;
    const eql = std.mem.eql;

    const maxImm: InstructionTypeI = .{
        .imm = max(u17),
        .rd = 0,
        .op_page = 0,
        .opcode = 0,
    };
    const maxImmBytes: [4]u8 = @bitCast(maxImm);
    const maxRd: InstructionTypeI = .{
        .imm = 0,
        .rd = max(u5),
        .op_page = 0,
        .opcode = 0,
    };
    const maxRdBytes: [4]u8 = @bitCast(maxRd);
    const maxOpPage: InstructionTypeI = .{
        .imm = 0,
        .rd = 0,
        .op_page = max(u3),
        .opcode = 0,
    };
    const maxOpPageBytes: [4]u8 = @bitCast(maxOpPage);
    const maxOpcode: InstructionTypeI = .{
        .imm = 0,
        .rd = 0,
        .op_page = 0,
        .opcode = max(u7),
    };
    const maxOpcodeBytes: [4]u8 = @bitCast(maxOpcode);

    try expect(eql(u8, &maxImmBytes, &.{ 0xFF, 0xFF, 0x01, 0x00 }));
    try expect(eql(u8, &maxRdBytes, &.{ 0x00, 0x00, 0x3E, 0x00 }));
    try expect(eql(u8, &maxOpPageBytes, &.{ 0x00, 0x00, 0xC0, 0x01 }));
    try expect(eql(u8, &maxOpcodeBytes, &.{ 0x00, 0x00, 0x00, 0xFE }));

    // For testing!
    const f: InstructionTypeTemp = .{
        .imm = 0,
        .rd = 0,
        .op_page = max(u17),
        .opcode = 0,
    };
    std.debug.print("FA: {x}\n", .{ f.imm });
    std.debug.print("FA: {x}\n", .{ f.rd });
    std.debug.print("FA: {x}\n", .{ f.op_page });
    std.debug.print("FA: {x}\n", .{ f.opcode });

    const bytesf: [4]u8 = @bitCast(f);
    std.debug.print("FA: {x}\n", .{ bytesf });

    // const bytes: [4]u8 = .{ 0xFF, 0xFF, 0x01, 0x00 };
    // const i: InstructionTypeI = @bitCast(bytes);

    // std.debug.print("FF: {x}\n", .{ i.imm });
    // std.debug.print("FF: {x}\n", .{ i.rd });
    // std.debug.print("FF: {x}\n", .{ i.op_page });
    // std.debug.print("FF: {x}\n", .{ i.opcode });
    // std.debug.print("FF: {}\n", .{ i.imm });

    // try testing.expect(i.imm == std.math.maxInt(u17));
    // try testing.expect(i.rd == 0);
    // try testing.expect(i.op_page == 0);
    // try testing.expect(i.opcode == 0);
}

// pub const FatInstruction = packed struct {
//     b3: u8,
//     b2: u8,
//     b1: u8,
//     b0: u8,

//     b7: u8,
//     b6: u8,
//     b5: u8,
//     b4: u8,
// };

fn compileAssert(comptime arg: bool) if (arg) void else noreturn { if(!arg) unreachable; }

test "all in module" {
    _ = @import("temp.zig");
    // _ = temp1;

    std.testing.refAllDeclsRecursive(@This());
}

// test "u32 to instruction" {
//     const uint: u32 = 0xFFAA3490;
//     const i: Instruction = @bitCast(uint);

//     try testing.expect(i.b3 == 0x90);
//     try testing.expect(i.b2 == 0x34);
//     try testing.expect(i.b1 == 0xAA);
//     try testing.expect(i.b0 == 0xFF);

//     const uint2: u32 = 2864434397;
//     const i_2: Instruction = @bitCast(uint2);

//     try testing.expect(i_2.b3 == 0xDD);
//     try testing.expect(i_2.b2 == 0xCC);
//     try testing.expect(i_2.b1 == 0xBB);
//     try testing.expect(i_2.b0 == 0xAA);
// }

// test "instruction to u32" {
//     const instruct: Instruction = .{
//         .b3 = 0xFF,
//         .b2 = 0xAA,
//         .b1 = 0x34,
//         .b0 = 0x90,
//     };
//     const i: u32 = @bitCast(instruct);

//     try testing.expect(i == 0x9034AAFF);
// }

// test "fake instruction from file" {
//     var buffer: [4]u8 = .{ 0, 0, 0, 0 };
//     _ = try std.fs.cwd().readFile("./test/lib/bytecode/raw_u32le.bin", &buffer); //HAZV le

//     // Because we bitcast the buffer it is little endian to big endian.
//     const i_before: u32 = @bitCast(buffer);
//     try testing.expect(i_before == 0x565A4148); //VZAH be

//     // Keeps in little endian.
//     const i: Instruction = @bitCast(buffer);
//     try testing.expect(i.b3 == 0x48); // H le
//     try testing.expect(i.b2 == 0x41); // A
//     try testing.expect(i.b1 == 0x5A); // Z
//     try testing.expect(i.b0 == 0x56); // V
// }

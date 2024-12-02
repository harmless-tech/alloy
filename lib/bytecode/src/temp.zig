//
// Instuctions should have a 8 bit opcode and 3 bit diff.
// Some instructions can be given a more bit diff.
//
// 2 different sized instructions for now:
// 4 byte instuction.
// 4 byte instruction + 4 bytes of data.
// ??? 4 byte instruction + 8 bytes of data?
//
// R type - 2-3 registers
// RI type - 1 register + imm
// RI+ type - 1 register + imm + extended imm
//

//
// All sections are 512 byte aligned.
// Programs should be organized like:
// r-- Page Zero
// r-- Native function pointers
// r-- Read Only Data
// r-x Bytecode
// r-- Page Zero
// rw- Read Write Data
//
// rw- Stack (mmap)
// rw- Help (malloc)
//

test {
    const testing = @import("std").testing;
    try testing.expect(0 == 0);
}

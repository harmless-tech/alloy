const bytecode = @import("allot_bytecode");

pub fn sub(a: i32, b: i32) i32 {
    bytecode.trytry();
    return a - b;
}

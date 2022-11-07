use allot_codegen::RawEnum;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::Register;

#[derive(Clone, Debug, PartialEq, PartialOrd, RawEnum)]
pub enum Type {
    None,

    // Numeric
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int(isize),
    Int64(i64),
    Int128(i128),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt(usize),
    UInt64(u64),
    UInt128(u128),

    Float32(f32),
    Float64(f64),

    // Text
    Char(char),
    String(String),

    // Other
    Boolean(bool),
    Address(usize),
    Pointer(usize),
    Register(Register),
}

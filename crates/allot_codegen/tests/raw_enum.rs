#![allow(dead_code)]

use allot_codegen::RawEnum;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(RawEnum)]
enum Test {
    This,
    Is,
    A,
    Enum,
}

#[derive(RawEnum)]
enum Test2 {
    This(i32),
    Is(i32, i32),
    A(i32, i32),
    Enum(i32, i32, i32),
}

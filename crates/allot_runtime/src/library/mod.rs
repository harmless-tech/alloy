// TODO: Allow optional libraries, like gui? (wasm plugins?)

mod standard;

use std::{
    io,
    io::{BufRead, Read},
};

use phf::phf_map;

use crate::{CrossHeap, StackFrame, Type};

type LibraryFunction = fn(Type, &mut StackFrame, CrossHeap) -> Type;

static FUNCTIONS: phf::Map<&'static str, LibraryFunction> = phf_map! {
    // IO
    "print" => print,
    "println" => println,
    "std::print_amt" => standard::print_amt,
    "read" => read,
    "read_line" => read_line,
    "std::read_all" => standard::read_all,
};

pub fn call(function: &str, arg: Type, stack_frame: &mut StackFrame, heap: CrossHeap) -> Type {
    let f = match FUNCTIONS.get(function) {
        None => panic!("Tried to call a function that does not exist."),
        Some(func) => func,
    };

    f(arg, stack_frame, heap)
}

// Library functions
//fn template(arg: Type, stack_frame: &mut StackFrame, heap: CrossHeap) -> Type {}

fn print(arg: Type, _stack_frame: &mut StackFrame, _heap: CrossHeap) -> Type {
    match arg {
        Type::None => print!(""),
        Type::Int8(v) => print!("{}", v),
        Type::Int16(v) => print!("{}", v),
        Type::Int32(v) => print!("{}", v),
        Type::Int(v) => print!("{}", v),
        Type::Int64(v) => print!("{}", v),
        Type::Int128(v) => print!("{}", v),
        Type::UInt8(v) => print!("{}", v),
        Type::UInt16(v) => print!("{}", v),
        Type::UInt32(v) => print!("{}", v),
        Type::UInt(v) => print!("{}", v),
        Type::UInt64(v) => print!("{}", v),
        Type::UInt128(v) => print!("{}", v),
        Type::Float32(v) => print!("{}", v),
        Type::Float64(v) => print!("{}", v),
        Type::Char(v) => print!("{}", v),
        Type::String(v) => print!("{}", v),
        Type::Boolean(v) => print!("{}", v),
        Type::Pointer(v) => print!("{:X?}", v),
        Type::Address(v) => print!("{:X?}", v),
        Type::Register(v) => print!("{:?}", v),
    }
    Type::None
}

fn println(arg: Type, _stack_frame: &mut StackFrame, _heap: CrossHeap) -> Type {
    i_println(arg);
    Type::None
}

#[inline]
fn i_println(arg: Type) {
    match arg {
        Type::None => println!(),
        Type::Int8(v) => println!("{}", v),
        Type::Int16(v) => println!("{}", v),
        Type::Int32(v) => println!("{}", v),
        Type::Int(v) => println!("{}", v),
        Type::Int64(v) => println!("{}", v),
        Type::Int128(v) => println!("{}", v),
        Type::UInt8(v) => println!("{}", v),
        Type::UInt16(v) => println!("{}", v),
        Type::UInt32(v) => println!("{}", v),
        Type::UInt(v) => println!("{}", v),
        Type::UInt64(v) => println!("{}", v),
        Type::UInt128(v) => println!("{}", v),
        Type::Float32(v) => println!("{}", v),
        Type::Float64(v) => println!("{}", v),
        Type::Char(v) => println!("{}", v),
        Type::String(v) => println!("{}", v),
        Type::Boolean(v) => println!("{}", v),
        Type::Pointer(v) => println!("{:X?}", v),
        Type::Address(v) => println!("{:X?}", v),
        Type::Register(v) => println!("{:?}", v),
    }
}

fn read(_arg: Type, _stack_frame: &mut StackFrame, _heap: CrossHeap) -> Type {
    let mut buffer = [0_u8; 1];
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle
        .read_exact(&mut buffer)
        .expect("Failed to read a byte from stdin.");

    Type::UInt8(buffer[0])
}

fn read_line(_arg: Type, _stack_frame: &mut StackFrame, _heap: CrossHeap) -> Type {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle
        .read_line(&mut buffer)
        .expect("Failed to read line from stdin.");

    Type::String(buffer)
}

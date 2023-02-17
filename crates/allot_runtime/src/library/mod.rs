// TODO: Allow optional libraries, like gui? (wasm plugins?)

use std::{
    io,
    io::{BufRead, Read, Write},
};

use allot_codegen::lib_return;
use phf::phf_map;

use crate::{CrossHeap, StackFrame, Type};

mod standard;
mod thread;

type LibraryRegisters<'a> = (&'a Type, &'a Type, &'a Type, &'a Type, &'a Type);
type LibraryReturn = (
    Option<Type>,
    Option<Type>,
    Option<Type>,
    Option<Type>,
    Option<Type>,
);
type LibraryFunction = fn(LibraryRegisters, &mut StackFrame, &mut CrossHeap) -> LibraryReturn;

static FUNCTIONS: phf::Map<&'static str, LibraryFunction> = phf_map! {
    // Control
    "exit" => exit,

    // IO
    "print" => print,
    "println" => println,
    "std::print_amt" => standard::print_amt,
    "read" => read,
    "read_line" => read_line,
    "std::read_all" => standard::read_all,

    // Heap
    "heap::free" => impl_heap_free,

    // Threads
    "thread::sleep" => thread::sleep,

    // Errors
    // "error" => error,

    // String
    "string::trim" => impl_string_trim // TODO: Move to own module?
    // "string::convert"
    //TODO Allow RawTypes as type? Or just use a UInt to convert.
};

pub fn call(
    function: &str,
    args: LibraryRegisters,
    stack_frame: &mut StackFrame,
    heap: &mut CrossHeap,
) -> LibraryReturn {
    let f = match FUNCTIONS.get(function) {
        None => panic!("Tried to call a function that does not exist."),
        Some(func) => func,
    };

    f(args, stack_frame, heap)
}

// Library functions
//fn template(args: LibraryRegisters, stack_frame: &mut StackFrame, heap: &mut CrossHeap) -> LibraryReturn {}

fn exit(
    args: LibraryRegisters,
    _stack_frame: &mut StackFrame,
    _heap: &mut CrossHeap,
) -> LibraryReturn {
    let code = match args.0 {
        Type::Int32(v) => *v,
        _ => {
            i_println(args.0);
            0
        }
    };

    std::process::exit(code);
}

fn print(
    args: LibraryRegisters,
    _stack_frame: &mut StackFrame,
    _heap: &mut CrossHeap,
) -> LibraryReturn {
    match args.0 {
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

    io::stdout().flush().unwrap();

    lib_return!()
}

fn println(
    args: LibraryRegisters,
    _stack_frame: &mut StackFrame,
    _heap: &mut CrossHeap,
) -> LibraryReturn {
    i_println(args.0);
    lib_return!()
}

#[inline]
fn i_println(arg: &Type) {
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

fn read(
    _args: LibraryRegisters,
    _stack_frame: &mut StackFrame,
    _heap: &mut CrossHeap,
) -> LibraryReturn {
    let mut buffer = [0_u8; 1];
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle
        .read_exact(&mut buffer)
        .expect("Failed to read a byte from stdin.");

    lib_return!(Type::UInt8(buffer[0]))
}

fn read_line(
    _args: LibraryRegisters,
    _stack_frame: &mut StackFrame,
    _heap: &mut CrossHeap,
) -> LibraryReturn {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle
        .read_line(&mut buffer)
        .expect("Failed to read line from stdin.");

    lib_return!(Type::String(buffer))
}

fn impl_heap_free(
    args: LibraryRegisters,
    _stack_frame: &mut StackFrame,
    heap: &mut CrossHeap,
) -> LibraryReturn {
    match args.0 {
        Type::Pointer(p) => {
            let mut handle = heap.lock().unwrap();
            handle.free(*p);
        }
        _ => panic!("heap::free expects a pointer."),
    }

    lib_return!()
}

fn impl_string_trim(
    args: LibraryRegisters,
    _stack_frame: &mut StackFrame,
    _heap: &mut CrossHeap,
) -> LibraryReturn {
    let ret = match args.0 {
        Type::String(v) => Type::String(String::from(v.trim())),
        _ => panic!("string::trim expects a string."),
    };

    lib_return!(ret)
}

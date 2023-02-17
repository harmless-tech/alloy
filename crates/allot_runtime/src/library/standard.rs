use std::{io, io::BufRead};

use allot_codegen::lib_return;

use crate::{
    library::{i_println, LibraryRegisters, LibraryReturn},
    CrossHeap, StackFrame, Type,
};

pub fn print_amt(
    args: LibraryRegisters,
    stack_frame: &mut StackFrame,
    _heap: &mut CrossHeap,
) -> LibraryReturn {
    let amount = match args.0 {
        Type::UInt(i) => i,
        _ => panic!("std::printamt expects a uint in the register."),
    };

    for i in 0..*amount {
        let t = stack_frame.clone_offset(i);
        i_println(&t);
    }

    lib_return!()
}

pub fn read_all(
    _args: LibraryRegisters,
    _stack_frame: &mut StackFrame,
    _heap: &mut CrossHeap,
) -> LibraryReturn {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = line.expect("Failed to read lines from stdin.");
        buffer.push_str(line.as_str());
    }

    lib_return!(Type::String(buffer))
}

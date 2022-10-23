use std::{io, io::BufRead};

use crate::{library::i_println, CrossHeap, StackFrame, Type};

pub fn print_amt(arg: Type, stack_frame: &mut StackFrame, _heap: &CrossHeap) -> Type {
    let amount = match arg {
        Type::UInt(i) => i,
        _ => panic!("std::printamt expects a uint in the register."),
    };

    for i in 0..amount {
        let t = stack_frame.clone_offset(i);
        i_println(t);
    }

    Type::None
}

pub fn read_all(_arg: Type, _stack_frame: &mut StackFrame, _heap: &CrossHeap) -> Type {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = line.expect("Failed to read lines from stdin.");
        buffer.push_str(line.as_str());
    }

    Type::String(buffer)
}

use std::time::Duration;

use allot_codegen::lib_return;
use allot_lib::Type;

use crate::{
    library::{LibraryRegisters, LibraryReturn},
    CrossHeap, StackFrame,
};

/// Makes the current thread sleep for Type::UInt64(TIME).
pub fn sleep(
    args: LibraryRegisters,
    _stack_frame: &mut StackFrame,
    _heap: &mut CrossHeap,
) -> LibraryReturn {
    let time = match args.0 {
        Type::UInt64(i) => *i,
        _ => panic!("thread::sleep expects a u64 in the register."),
    };

    std::thread::sleep(Duration::from_millis(time));

    lib_return!()
}

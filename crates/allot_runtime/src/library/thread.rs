use std::time::Duration;

use allot_lib::Type;

use crate::{CrossHeap, StackFrame};

/// Makes the current thread sleep for Type::UInt64(TIME) in ROR.
pub fn sleep(arg: Type, _stack_frame: &mut StackFrame, _heap: &mut CrossHeap) -> Type {
    let time = match arg {
        Type::UInt64(i) => i,
        _ => panic!("thread::sleep expects a u64 in the register."),
    };

    std::thread::sleep(Duration::from_millis(time));

    Type::None
}

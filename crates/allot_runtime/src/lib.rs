use std::sync::Arc;

use allot_lib::{Instruction, Type};
pub use tick::*;

use crate::memory::{CrossHeap, Heap, Registers, StackFrame};

mod library;
mod memory;
mod operations;
#[doc(hidden)]
mod tick;

pub struct AllotRuntime {
    pub current: usize,
    pub instructions: Arc<Vec<Instruction>>,
    pub registers: Registers,
    pub stack_frames: Vec<StackFrame>,
    pub heap: CrossHeap,
}
impl AllotRuntime {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions: Arc::new(instructions),
            registers: Registers::new(),
            stack_frames: vec![StackFrame::default()],
            heap: Heap::cross_new(),
            current: 0,
        }
    }

    pub fn new_arc(instructions: Arc<Vec<Instruction>>) -> Self {
        Self {
            instructions,
            registers: Registers::new(),
            stack_frames: vec![StackFrame::default()],
            heap: Heap::cross_new(),
            current: 0,
        }
    }

    pub fn new_thread(
        instructions: Arc<Vec<Instruction>>,
        stack_frame: StackFrame,
        heap: CrossHeap,
        current: usize,
    ) -> Self {
        Self {
            instructions,
            registers: Registers::new(),
            stack_frames: vec![stack_frame],
            heap,
            current,
        }
    }

    pub fn run(&mut self) -> i32 {
        let mut code = self.tick();
        while code.is_none() {
            code = self.tick();
        }
        code.unwrap()
    }

    pub fn take_stack_frame(&mut self) -> StackFrame {
        self.stack_frames.pop().expect("No stack frames to take.")
    }
}
impl AllotRuntime {
    #[inline]
    fn get_uint(t: &Type, registers: &mut Registers) -> usize {
        match t {
            Type::UInt(i) => *i,
            Type::Register(reg) => match registers.get(*reg) {
                Type::UInt(i) => *i,
                _ => panic!("Register did not hold a UInt type."),
            },
            _ => panic!("Type was not a UInt or Register."),
        }
    }

    #[inline]
    fn get_int32(t: &Type, registers: &mut Registers) -> i32 {
        match t {
            Type::Int32(i) => *i,
            Type::Register(reg) => match registers.get(*reg) {
                Type::Int32(i) => *i,
                _ => panic!("Register did not hold a Int32 type."),
            },
            _ => panic!("Type was not a Int32 or Register."),
        }
    }

    #[inline]
    fn get_address(t: &Type, registers: &mut Registers) -> usize {
        match t {
            Type::Address(i) => *i,
            Type::Register(reg) => match registers.get(*reg) {
                Type::Address(i) => *i,
                _ => panic!("Register did not hold a Label type."),
            },
            _ => panic!("Type was not a Label or Register."),
        }
    }
}

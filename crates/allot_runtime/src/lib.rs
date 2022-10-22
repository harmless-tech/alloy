#![allow(dead_code)] // TODO: Remove?

#[cfg(feature = "bytecode")]
mod bytecode;
pub mod structures;
#[cfg(test)]
mod tests;
mod traits;

#[cfg(feature = "bytecode")]
pub use bytecode::from_bytecode;
#[cfg(feature = "bytecode_gen")]
pub use bytecode::to_bytecode;

use std::sync::{Arc, RwLock};
use structures::*;

pub struct AllotRuntime {
    current: usize,
    instructions: Vec<Instruction>,
    labels: Vec<usize>,
    registers: [Type; 30],
    stack_frames: Vec<StackFrame>,
    heap: Arc<RwLock<Heap>>,
    is_thread: bool,
}
impl AllotRuntime {
    pub fn new(instructions: Vec<Instruction>, labels: Vec<usize>, is_thread: bool) -> Self {
        let registers: [Type; 30] = (0..30)
            .map(|_i| Type::None)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self {
            instructions,
            labels,
            registers,
            stack_frames: vec![StackFrame::default()],
            heap: Arc::new(RwLock::new(Heap::default())),
            current: 0,
            is_thread,
        }
    }

    pub fn tick(&mut self) {
        todo!()
    }

    pub fn run(&mut self) {
        todo!()
    }
}
/// Instructions Impl.
impl AllotRuntime {}

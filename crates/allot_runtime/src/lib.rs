#![allow(dead_code)] // TODO: Remove?

#[cfg(feature = "bytecode")]
mod bytecode;
pub mod structures;
#[cfg(test)]
mod tests;
mod traits;

#[cfg(feature = "bytecode")]
pub use bytecode::bytecode_convert;

use std::sync::{Arc, RwLock};
use structures::*;

pub struct AllotRuntime {
    current: usize,
    instructions: Vec<Instruction>,
    labels: Vec<usize>,
    registers: [Type; 30],
    stack_frames: Vec<StackFrame>,
    heap: Arc<RwLock<Heap>>,
}
impl AllotRuntime {
    pub fn new(instructions: Vec<Instruction>, labels: Vec<usize>) -> Self {
        let registers: [Type; 30] = (0..30)
            .map(|_i| Type::None)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self {
            instructions,
            labels,
            registers,
            stack_frames: vec![StackFrame::new()],
            heap: Arc::new(RwLock::new(Heap::new())),
            current: 0,
        }
    }

    pub fn tick(&mut self) {
        todo!()
    }
}

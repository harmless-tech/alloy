#![allow(dead_code)] // TODO: Remove?

#[cfg(feature = "bytecode")]
mod bytecode;
pub mod structures;
#[cfg(test)]
mod tests;
mod traits;

#[cfg(feature = "bytecode")]
pub use bytecode::bytecode_convert;

use std::collections::HashMap;
use structures::*;

pub struct AllotRuntime {
    instructions: Vec<Instruction>,
    address_map: Vec<usize>,
    registers: [Type; 30],
    stack_frames: Vec<StackFrame>,
    heap: HashMap<usize, Type>,
    current: usize,
}
impl AllotRuntime {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        let registers: [Type; 30] = (0..30)
            .map(|_i| Type::None)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self {
            instructions,
            address_map: Vec::new(),
            registers,
            stack_frames: Vec::new(),
            heap: HashMap::new(),
            current: 0,
        }
    }

    pub fn tick(&mut self) {
        todo!()
    }
}

mod byte_form;

use crate::Instruction;
pub use byte_form::*;

const BYTECODE_VERSION: usize = 0;

pub fn from_bytecode(_bytes: &[u8]) -> (Vec<Instruction>, Vec<usize>) {
    todo!("This is not implemented yet and ")
}

#[cfg(feature = "bytecode_gen")]
pub fn to_bytecode(_instructions: Vec<Instruction>, _labels: Vec<usize>) -> Vec<u8> {
    todo!()
}

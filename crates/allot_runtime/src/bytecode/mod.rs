mod byte_form;

use crate::Instruction;
pub use byte_form::*;

pub fn bytecode_convert(_bytes: &[u8]) -> Vec<Instruction> {
    todo!()
}

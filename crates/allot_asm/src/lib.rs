extern crate core;

use allot_lib::Instruction;

mod lexer;
mod parser;

pub fn compile(program: String) -> Vec<Instruction> {
    let tokens = lexer::lex(&program);
    parser::parse(tokens)
}

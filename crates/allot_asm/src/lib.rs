extern crate core;

use allot_lib::Instruction;

mod lexer;
mod parser;

pub fn compile(program: String) -> Vec<Instruction> {
    let tokens = lexer::lex(&program);
    dbg!(&tokens);
    let _instructions = parser::parse(tokens);

    todo!()
}

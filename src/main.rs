use allot_runtime::{
    structures::{Instruction, Register, Type},
    AllotRuntime,
};
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    let mut runtime = AllotRuntime::new(
        vec![
            Instruction::Call("read_line".to_string()),
            Instruction::Mov(Register::R9, Type::Register(Register::R10)),
            Instruction::Call("println".to_string()),
            Instruction::Exit(Type::Int32(0)),
        ],
        vec![],
    );
    runtime.run();
}

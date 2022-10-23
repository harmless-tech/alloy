use allot_lib::{Instruction, Register, Type};
use allot_runtime::AllotRuntime;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    let mut runtime = AllotRuntime::new(vec![
        Instruction::Call("read_line".to_string()),
        Instruction::Mov(Register::R20, Type::Register(Register::R10)),
        Instruction::Mov(Register::R9, Type::UInt64(2500)),
        Instruction::Call("thread::sleep".to_string()),
        Instruction::Mov(Register::R9, Type::Register(Register::R20)),
        Instruction::Call("println".to_string()),
        Instruction::Exit(Type::Int32(0)),
    ]);
    runtime.run();
}

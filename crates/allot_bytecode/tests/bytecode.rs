use allot_lib::{Instruction, Register, Type};
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[test]
#[cfg(feature = "gen")]
#[cfg(feature = "parse")]
fn gen_parse() {
    use allot_bytecode::{gen, parse};

    let i = vec![
        Instruction::Mov(Register::R9, Type::String("Hello!".to_string())),
        Instruction::Call("println".to_string()),
        Instruction::Exit(Type::Int32(0)),
    ];

    let bytecode = gen(i);
    let i = parse(bytecode);

    assert_eq!(
        i,
        vec![
            Instruction::Mov(Register::R9, Type::String("Hello!".to_string())),
            Instruction::Call("println".to_string()),
            Instruction::Exit(Type::Int32(0))
        ]
    );
}

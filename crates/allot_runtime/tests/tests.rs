use allot_runtime::{
    structures::{
        Instruction::{Assert, Exit, Mov, Op},
        OpPrim2::Add,
        Operation::Prim2,
        Register::{R1, R2},
        Type,
    },
    AllotRuntime,
};

#[test]
fn operations() {}

#[test]
fn mov() {
    let mut runtime = AllotRuntime::new(
        vec![
            Mov(R1, Type::UInt(50)),
            Mov(R2, Type::Register(R1)),
            Assert(R1, Type::None),
            Assert(R2, Type::UInt(50)),
            Exit(Type::Int32(512)),
        ],
        vec![],
    );

    assert_eq!(runtime.run(), 512);
}

#[test]
#[cfg(debug_assertions)]
fn op_add() {
    let mut runtime = AllotRuntime::new(
        vec![
            Mov(R1, Type::UInt(50)),
            Mov(R2, Type::UInt(75)),
            Op(Prim2(Add), [R1, R2]),
            Assert(R1, Type::UInt(125)),
            Exit(Type::Int32(512)),
        ],
        vec![],
    );

    assert_eq!(runtime.run(), 512);
}

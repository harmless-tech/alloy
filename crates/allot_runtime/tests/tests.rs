use allot_runtime::{
    structures::{
        Instruction::{Assert, Cpy, Exit, Mov, Op},
        OpPrim2::Add,
        Operation::Prim2,
        Register::{R1, R10, R2, R3, R4, R5, R6, R7, R8, R9},
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
fn cpy() {
    let mut runtime = AllotRuntime::new(
        vec![
            Mov(R1, Type::UInt(50)),
            Assert(R1, Type::UInt(50)),
            Cpy(R2, R1),
            Assert(R2, Type::UInt(50)),
            Cpy(R3, R1),
            Assert(R3, Type::UInt(50)),
            Cpy(R4, R1),
            Assert(R4, Type::UInt(50)),
            Cpy(R5, R1),
            Assert(R5, Type::UInt(50)),
            Cpy(R6, R1),
            Assert(R6, Type::UInt(50)),
            Cpy(R7, R1),
            Assert(R7, Type::UInt(50)),
            Cpy(R8, R1),
            Assert(R8, Type::UInt(50)),
            Cpy(R9, R1),
            Assert(R9, Type::UInt(50)),
            Cpy(R10, R1),
            Assert(R10, Type::UInt(50)),
            Cpy(R2, R1),
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

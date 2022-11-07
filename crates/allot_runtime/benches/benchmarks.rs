use std::sync::Arc;

use allot_lib::{
    Instruction::{Assert, Cpy, Exit, Mov},
    Register::{R1, R10, R2, R3, R4, R5, R6, R7, R8, R9},
    Type,
};
use allot_runtime::AllotRuntime;
use criterion::{criterion_group, criterion_main, Criterion};

fn copy_speed(c: &mut Criterion) {
    // mimalloc no guard: [350.41 ns 350.56 ns 350.75 ns]

    let instructions = &Arc::new(vec![
        Mov(R1, Type::UInt(50)),
        Assert(R1, Type::UInt(50)),
        Cpy(R2, R1),
        Cpy(R3, R2),
        Cpy(R4, R3),
        Cpy(R5, R4),
        Cpy(R6, R5),
        Cpy(R7, R6),
        Cpy(R8, R7),
        Cpy(R9, R8),
        Cpy(R10, R9),
        Cpy(R2, R10),
        Assert(R2, Type::UInt(50)),
        Exit(Type::Int32(512)),
    ]);

    c.bench_function("copy", |b| {
        b.iter(|| {
            let i = instructions.clone();
            let mut runtime = AllotRuntime::new_arc(i);
            runtime.run();
        })
    });
}

fn move_speed(c: &mut Criterion) {
    // mimalloc no guard: [567.22 ns 568.39 ns 569.61 ns]

    let instructions = &Arc::new(vec![
        Mov(R1, Type::UInt(50)),
        Assert(R1, Type::UInt(50)),
        Mov(R2, Type::Register(R1)),
        Mov(R3, Type::Register(R2)),
        Mov(R4, Type::Register(R3)),
        Mov(R5, Type::Register(R4)),
        Mov(R6, Type::Register(R5)),
        Mov(R7, Type::Register(R6)),
        Mov(R8, Type::Register(R7)),
        Mov(R9, Type::Register(R8)),
        Mov(R10, Type::Register(R9)),
        Mov(R2, Type::Register(R10)),
        Assert(R2, Type::UInt(50)),
        Exit(Type::Int32(512)),
    ]);

    c.bench_function("move", |b| {
        b.iter(|| {
            let i = instructions.clone();
            let mut runtime = AllotRuntime::new_arc(i);
            runtime.run();
        })
    });
}

criterion_group!(benches, copy_speed, move_speed);
criterion_main!(benches);

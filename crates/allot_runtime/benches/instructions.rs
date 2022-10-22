extern crate allot_runtime;
use allot_runtime::{
    structures::{
        Instruction::{Assert, Cpy, Exit, Mov},
        Register::{R1, R10, R2, R3, R4, R5, R6, R7, R8, R9},
        Type,
    },
    AllotRuntime,
};
use mimalloc::MiMalloc;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn copy_speed(c: &mut Criterion) {
    // Default alloc: [541.15 ns 542.11 ns 543.12 ns]
    // mimalloc: [429.13 ns 430.81 ns 432.44 ns]
    // mimalloc no guard: [424.61 ns 426.95 ns 429.57 ns]

    c.bench_function("copy speed", |b| b.iter(|| {
        let mut runtime = AllotRuntime::new(
            vec![
                Mov(R1, Type::UInt(50)),
                Assert(R1, Type::UInt(50)),
                Cpy(R2, R1),
                Cpy(R3, R1),
                Cpy(R4, R1),
                Cpy(R5, R1),
                Cpy(R6, R1),
                Cpy(R7, R1),
                Cpy(R8, R1),
                Cpy(R9, R1),
                Cpy(R10, R1),
                Cpy(R2, R1),
                Exit(Type::Int32(512)),
            ],
            vec![],
        );
        runtime.run();
    }));
}

criterion_group!(benches, copy_speed);
criterion_main!(benches);

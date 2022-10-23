use allot_lib::{
    Instruction::{Assert, Cpy, Exit, Mov},
    Register::{R1, R10, R2, R3, R4, R5, R6, R7, R8, R9},
    Type,
};
use allot_runtime::AllotRuntime;
use criterion::{criterion_group, criterion_main, Criterion};
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn copy_speed(c: &mut Criterion) {
    // mimalloc no guard: [351.27 ns 351.70 ns 352.14 ns]

    c.bench_function("copy", |b| {
        b.iter(|| {
            let mut runtime = AllotRuntime::new(vec![
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
            ]);
            runtime.run();
        })
    });
}

criterion_group!(benches, copy_speed);
criterion_main!(benches);

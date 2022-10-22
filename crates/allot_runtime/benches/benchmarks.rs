use allot_runtime::{
    structures::{
        Heap, HeapType,
        Instruction::{Assert, Cpy, Exit, Mov},
        Register::{R1, R10, R2, R3, R4, R5, R6, R7, R8, R9},
        Type,
    },
    AllotRuntime,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn copy_speed(c: &mut Criterion) {
    // mimalloc no guard: [398.99 ns 399.81 ns 400.66 ns]

    c.bench_function("copy", |b| {
        b.iter(|| {
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
        })
    });
}

#[allow(clippy::all)]
fn heap_perf(c: &mut Criterion) {
    // Default alloc: [523.48 µs 524.38 µs 525.26 µs]
    // mimalloc: [466.03 µs 466.63 µs 467.29 µs]
    // mimalloc no guard: [446.25 µs 446.94 µs 447.72 µs]

    c.bench_function("heap", |b| {
        b.iter(|| {
            let mut heap = Heap::default();
            black_box(for _ in 0..10000 {
                let _ = heap.push(HeapType::None);
            });
        })
    });
}

criterion_group!(benches, copy_speed, heap_perf);
criterion_main!(benches);

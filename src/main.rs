//use allot_runtime::{bytecode_convert, AllotRuntime};

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    println!("Hello, world!");
}

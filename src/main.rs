mod args;

use std::fs;

use allot_runtime::AllotRuntime;
use anyhow::Result;
use clap::Parser;
#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// File Exts: asm: .ala, bytecode (program): .allot
fn main() -> Result<()> {
    let args = args::Args::parse();
    let mut path: std::path::PathBuf = args.path;

    if args.asm || args.run {
        // Compile asm
        #[cfg(feature = "asm")]
        {
            let file = fs::read_to_string(&path)?;
            let instructions = allot_asm::compile(file);
            let bytecode = allot_bytecode::gen(instructions);

            if path.set_extension("allot") {
                fs::write(&path, bytecode)?;
            }
            else {
                anyhow::bail!("Could not configure ext for output file.");
            }
        }

        #[cfg(not(feature = "asm"))]
        anyhow::bail!("The asm feature is not enabled.");
    }

    if !args.asm || args.run {
        // Run
        let bytecode = fs::read(&path)?;
        let instructions = allot_bytecode::parse(bytecode);
        let mut runtime = AllotRuntime::new(instructions);
        runtime.run();
    }

    Ok(())
}

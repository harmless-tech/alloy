mod args;

use std::fs;

use allot_runtime::AllotRuntime;
use anyhow::Result;
use clap::Parser;

#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;

/// File Exts: asm: .ala, bytecode (program): .allot

// TODO: Run away memory problem? Doesn't run on github actions docker.

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> Result<()> {
    let args = args::Args::parse();

    if args.asm {
        // Compile asm
        #[cfg(feature = "asm")]
        {
            let file = fs::read_to_string(&args.path)?;
            let instructions = allot_asm::compile(file);
            let bytecode = allot_bytecode::gen(instructions);

            let mut path: std::path::PathBuf = args.path;
            if path.set_extension("allot") {
                fs::write(path, bytecode)?;
            }
            else {
                anyhow::bail!("Could not configure ext for output file.");
            }
        }

        #[cfg(not(feature = "asm"))]
        anyhow::bail!("The asm feature is not enabled.");
    }
    else {
        // Run
        let bytecode = fs::read(args.path)?;
        let instructions = allot_bytecode::parse(bytecode);
        let mut runtime = AllotRuntime::new(instructions);
        runtime.run();
    }

    Ok(())
}

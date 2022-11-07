use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub path: PathBuf,
    /// Compile an allot_asm file to bytecode. (Requires the asm feature)
    #[arg(short, long)]
    pub asm: bool,
    /// Compile an allot_asm file to bytecode then runs it. (Requires the asm feature)
    #[arg(short, long)]
    pub run: bool,
}

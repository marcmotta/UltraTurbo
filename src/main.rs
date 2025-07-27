// src/main.rs
/*
 * Main executable for UltraTurbo
 */

use clap::Parser;
use ultraturbo::{Result, run};

#[derive(Parser)]
#[command(version, about = "UltraTurbo - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}

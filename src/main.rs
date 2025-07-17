// src/main.rs
/*
 * Main executable for NftMarketplaceEngineTech
 */

use clap::Parser;
use nftmarketplaceenginetech::{Result, run};

#[derive(Parser)]
#[command(version, about = "NftMarketplaceEngineTech - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}

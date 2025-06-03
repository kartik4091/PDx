//! PDx - PDF Anti-Forensics Analysis Tool (CLI)
//! Author: kartik4091
//! Created: 2025-06-03 19:47:23 UTC

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use anyhow::Result;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = "kartik4091";
const BUILD_TIMESTAMP: &str = "2025-06-03 19:47:23 UTC";

#[derive(Parser)]
#[command(
    name = "pdx",
    about = "PDF Anti-Forensics Analysis Tool",
    version = VERSION,
    author = "kartik4091 <pithavakartik@gmail.com>"
)]
struct Cli {
    /// PDF file to analyze
    #[arg(required = true)]
    file: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("PDx v{} by {}", VERSION, AUTHOR);
    println!("Build timestamp: {}", BUILD_TIMESTAMP);
    println!("Analyzing file: {}", cli.file.display());
    Ok(())
}
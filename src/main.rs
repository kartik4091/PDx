//! PDx - PDF Anti-Forensics Analysis Tool (CLI)
//! Author: kartik4091
//! Created: 2025-06-03 19:50:37 UTC

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use tracing::{info, warn, error, Level};
use tracing_subscriber::FmtSubscriber;
use anyhow::Result;
use pdx::PdfAnalyzer;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = "kartik4091";
const BUILD_TIMESTAMP: &str = "2025-06-03 19:50:37 UTC";

#[derive(Parser)]
#[command(
    name = "pdx",
    about = "PDF Anti-Forensics Analysis Tool",
    version = VERSION,
    author = "kartik4091 <pithavakartik@gmail.com>"
)]
struct Cli {
    /// Sets the log level
    #[arg(short, long, default_value = "info")]
    log_level: Level,

    /// PDF file to analyze
    #[arg(required = true)]
    file: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(cli.log_level)
        .with_target(false)
        .pretty()
        .build();

    tracing::subscriber::set_global_default(subscriber)?;

    info!("PDx v{} starting up...", VERSION);
    info!("Build timestamp: {}", BUILD_TIMESTAMP);
    info!("Author: {}", AUTHOR);

    let analyzer = PdfAnalyzer::new(&cli.file)?;
    match analyzer.analyze().await {
        Ok(analysis) => {
            info!("Analysis complete for: {}", analysis.path);
            info!("File size: {} bytes", analysis.size);
            info!("Timestamp: {}", analysis.timestamp);
        }
        Err(e) => error!("Analysis failed: {}", e),
    }

    info!("PDx shutting down");
    Ok(())
}
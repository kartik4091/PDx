//! PDx - PDF Anti-Forensics Analysis Tool (CLI)
//! 
//! Author: kartik4091
//! Created: 2025-06-03 19:38:56 UTC

use std::{
    path::{Path, PathBuf},
    fs,
    io::{self, Write},
};

use clap::{Parser, Subcommand};
use tracing::{info, warn, error, Level};
use tracing_subscriber::{FmtSubscriber, EnvFilter};
use anyhow::Result;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use dialoguer::{Input, Select, Confirm};

use pdx::{
    PdfAnalyzer, Config, SecurityLevel, OutputFormat,
    PdfAnalysis, utils, VERSION, BUILD_TIMESTAMP, AUTHOR
};

#[derive(Parser)]
#[command(
    name = "pdx",
    about = "PDF Anti-Forensics Analysis Tool",
    version = VERSION,
    author = "kartik4091 <pithavakartik@gmail.com>"
)]
struct Cli {
    /// Sets the log level (error, warn, info, debug, trace)
    #[arg(short, long, default_value = "info")]
    log_level: Level,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Configuration file
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Number of threads to use
    #[arg(short = 'j', long, default_value_t = num_cpus::get())]
    threads: usize,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze PDF file(s)
    Analyze {
        /// PDF file(s) to analyze
        #[arg(required = true)]
        files: Vec<PathBuf>,
        
        /// Output format (text, json, detailed)
        #[arg(short, long, default_value = "text")]
        format: String,
        
        /// Output file
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Analysis depth (1-4)
        #[arg(short, long, default_value = "2")]
        depth: u8,
    },
    /// Scan for security issues
    Scan {
        /// PDF file(s) to scan
        #[arg(required = true)]
        files: Vec<PathBuf>,
        
        /// Security level (low, medium, high, paranoid)
        #[arg(short, long, default_value = "medium")]
        level: String,
    },
    /// Show detailed information
    Info {
        /// Show system information
        #[arg(short, long)]
        system: bool,
        
        /// Show configuration
        #[arg(short, long)]
        config: bool,
    },
    /// Interactive mode
    Interactive,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup logging
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(env_filter)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .with_target(cli.verbose)
        .pretty()
        .build();

    tracing::subscriber::set_global_default(subscriber)?;

    info!("PDx v{} starting up...", VERSION);
    info!("Build timestamp: {}", BUILD_TIMESTAMP);
    info!("Author: {}", AUTHOR);

    // Load configuration
    let mut config = if let Some(config_path) = cli.config {
        load_config(&config_path)?
    } else {
        Config::default()
    };

    // Update thread count from CLI
    config.thread_count = cli.threads;

    match cli.command {
        Commands::Analyze { files, format, output, depth } => {
            config.analysis_depth = depth;

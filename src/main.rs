use std::path::PathBuf;
use anyhow::Result;
use clap::Parser;
use tracing::{info, error};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
#[command(name = "pdx", about = "PDF Anti-Forensics Analysis Tool")]
struct Cli {
    /// PDF file to analyze
    #[arg(required = true)]
    file: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .init();

    let cli = Cli::parse();
    let file_path = cli.file;

    info!("PDx Anti-Forensics Tool");
    info!("Author: kartik4091");
    info!("Timestamp: 2025-06-03 19:58:30");

    if !file_path.exists() {
        error!("File not found: {}", file_path.display());
        std::process::exit(1);
    }

    match analyze_pdf(&file_path).await {
        Ok(_) => info!("Analysis complete"),
        Err(e) => error!("Analysis failed: {}", e),
    }

    Ok(())
}

async fn analyze_pdf(path: &PathBuf) -> Result<()> {
    use lopdf::Document;
    
    info!("Loading PDF: {}", path.display());
    let doc = Document::load(path)?;
    
    info!("PDF Version: {}.{}", doc.version.0, doc.version.1);
    info!("Total pages: {}", doc.get_pages().len());
    
    // Start real analysis
    analyze_metadata(&doc)?;
    analyze_javascript(&doc)?;
    analyze_images(&doc)?;
    analyze_security(&doc)?;

    Ok(())
}

fn analyze_metadata(doc: &lopdf::Document) -> Result<()> {
    if let Some(info) = doc.get_info() {
        info!("Analyzing metadata...");
        for (key, value) in info.iter() {
            if let Ok(text) = value.as_text_string() {
                info!("{}: {}", String::from_utf8_lossy(key), text);
            }
        }
    }
    Ok(())
}

fn analyze_javascript(doc: &lopdf::Document) -> Result<()> {
    info!("Scanning for JavaScript...");
    for (_, object) in doc.objects.iter() {
        if let lopdf::Object::Stream(ref stream) = object {
            if let Ok(data) = stream.decompressed_content() {
                if data.windows(3).any(|w| w == b"JS ") {
                    info!("JavaScript content found!");
                }
            }
        }
    }
    Ok(())
}

fn analyze_images(doc: &lopdf::Document) -> Result<()> {
    info!("Analyzing images...");
    let mut image_count = 0;
    for (_, object) in doc.objects.iter() {
        if let lopdf::Object::Stream(ref stream) = object {
            if stream.dict.get(b"Subtype") == Some(&lopdf::Object::Name(b"Image".to_vec())) {
                image_count += 1;
            }
        }
    }
    info!("Found {} images", image_count);
    Ok(())
}

fn analyze_security(doc: &lopdf::Document) -> Result<()> {
    info!("Analyzing security...");
    if doc.trailer.get(b"Encrypt").is_some() {
        info!("Document is encrypted");
    }
    Ok(())
}
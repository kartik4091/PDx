//! PDx - PDF Anti-Forensics Analysis Tool
//! 
//! Author: kartik4091
//! Created: 2025-06-03 19:33:21 UTC
//! 
//! This library provides functionality for PDF analysis and anti-forensics operations.
//! 
//! # Usage
//! ```no_run
//! use pdx::PdfAnalyzer;
//! 
//! let analyzer = PdfAnalyzer::load("example.pdf").unwrap();
//! let analysis = analyzer.analyze().unwrap();
//! println!("PDF Analysis: {:?}", analysis);
//! ```

use std::path::Path;
use anyhow::Result;
use thiserror::Error;
use lopdf::Document;
use chrono::{DateTime, Utc};
use serde::Serialize;

const BUILD_TIMESTAMP: &str = "2025-06-03 19:33:21 UTC";
const AUTHOR: &str = "kartik4091";

#[derive(Error, Debug)]
pub enum PdxError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("PDF error: {0}")]
    Pdf(String),
    
    #[error("Analysis error: {0}")]
    Analysis(String),
    
    #[error("Security error: {0}")]
    Security(String),
}

pub struct PdfAnalyzer {
    document: Document,
    created: DateTime<Utc>,
}

impl PdfAnalyzer {
    pub fn new() -> Self {
        Self {
            document: Document::new(),
            created: Utc::now(),
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, PdxError> {
        let document = Document::load(path)
            .map_err(|e| PdxError::Pdf(e.to_string()))?;
        
        Ok(Self {
            document,
            created: Utc::now(),
        })
    }

    pub fn analyze(&self) -> Result<PdfAnalysis, PdxError> {
        let mut analysis = PdfAnalysis::new();
        
        // Basic PDF information
        analysis.version = self.document.version.clone();
        analysis.page_count = self.document.get_pages().len() as u32;
        
        // Extract metadata
        if let Some(info) = self.document.trailer.get_info() {
            analysis.metadata = Some(info.clone());
        }
        
        Ok(analysis)
    }

    pub fn scan_security(&self) -> Result<SecurityReport, PdxError> {
        let mut report = SecurityReport::new();
        
        // Check for encryption
        report.is_encrypted = self.document.is_encrypted();
        
        // Check for JavaScript
        report.has_javascript = self.document.get_pages().iter().any(|(_, page)| {
            page.has_js()
        });
        
        Ok(report)
    }

    pub fn get_info(&self) -> String {
        format!("PDx Analysis Tool\nBuild: {}\nAuthor: {}", BUILD_TIMESTAMP, AUTHOR)
    }
}

#[derive(Debug, Serialize)]
pub struct PdfAnalysis {
    pub version: String,
    pub page_count: u32,
    #[serde(skip)]
    pub metadata: Option<lopdf::Dictionary>,
    pub timestamp: DateTime<Utc>,
}

impl PdfAnalysis {
    pub fn new() -> Self {
        Self {
            version: String::new(),
            page_count: 0,
            metadata: None,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SecurityReport {
    pub is_encrypted: bool,
    pub has_javascript: bool,
    pub timestamp: DateTime<Utc>,
}

impl SecurityReport {
    pub fn new() -> Self {
        Self {
            is_encrypted: false,
            has_javascript: false,
            timestamp: Utc::now(),
        }
    }
}

pub mod utils {
    use chrono::{DateTime, Utc};
    use sha2::{Sha256, Digest};
    
    pub fn get_timestamp() -> String {
        Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }
    
    pub fn calculate_hash(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        base64::encode(hasher.finalize())
    }

    pub fn validate_timestamp(timestamp: &str) -> bool {
        DateTime::parse_from_str(&format!("{} UTC", timestamp), "%Y-%m-%d %H:%M:%S %Z").is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    
    #[test]
    fn test_timestamp_validation() {
        assert!(utils::validate_timestamp("2025-06-03 19:33:21"));
    }
    
    #[test]
    fn test_new_analyzer() {
        let analyzer = PdfAnalyzer::new();
        assert!(analyzer.created <= Utc::now());
    }
    
    #[test]
    fn test_load_invalid_pdf() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.pdf");
        let mut file = File::create(&path).unwrap();
        writeln!(file, "Not a PDF file").unwrap();
        
        assert!(PdfAnalyzer::load(&path).is_err());
    }
}

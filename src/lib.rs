//! PDx - PDF Anti-Forensics Analysis Tool
//! Author: kartik4091
//! Created: 2025-06-03 19:50:37 UTC

use std::path::Path;
use anyhow::Result;
use thiserror::Error;
use chrono::{DateTime, Utc};

#[derive(Error, Debug)]
pub enum PdxError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("PDF error: {0}")]
    Pdf(String),
    
    #[error("Analysis error: {0}")]
    Analysis(String),
}

pub struct PdfAnalyzer {
    path: String,
    created: DateTime<Utc>,
}

impl PdfAnalyzer {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self {
            path: path.as_ref().to_string_lossy().into_owned(),
            created: Utc::now(),
        })
    }

    pub async fn analyze(&self) -> Result<PdfAnalysis> {
        Ok(PdfAnalysis {
            path: self.path.clone(),
            timestamp: Utc::now(),
            size: tokio::fs::metadata(&self.path).await?.len(),
        })
    }
}

#[derive(Debug)]
pub struct PdfAnalysis {
    pub path: String,
    pub timestamp: DateTime<Utc>,
    pub size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_analyzer_creation() {
        let analyzer = PdfAnalyzer::new("test.pdf").unwrap();
        assert_eq!(analyzer.path, "test.pdf");
    }

    #[tokio::test]
    async fn test_analyze_error() {
        let analyzer = PdfAnalyzer::new("nonexistent.pdf").unwrap();
        assert!(analyzer.analyze().await.is_err());
    }
}
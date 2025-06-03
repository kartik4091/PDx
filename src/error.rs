//! Error types for PDx
//! Author: kartik4091
//! Created: 2025-06-03 19:56:29 UTC

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("PDF error: {0}")]
    Pdf(String),
    
    #[error("Analysis error: {0}")]
    Analysis(String),
    
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
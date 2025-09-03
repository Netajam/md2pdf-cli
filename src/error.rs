// Path: error.rs
use std::path::PathBuf;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O Error")]
    Io(#[from] std::io::Error),
    #[error("Failed to walk directory")]
    Walkdir(#[from] walkdir::Error),
    #[error("Conversion failed")]
    ConversionError {
        #[source]
        source: anyhow::Error,
    },
    #[error("Invalid input path: {path}")]
    InvalidInputPath { path: PathBuf },
}
pub type Result<T> = std::result::Result<T, AppError>;

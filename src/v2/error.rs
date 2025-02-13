use calamine::XlsxError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to list projects, because: {0}")]
    ListProjectFailed(#[from] io::Error),

    #[error("Failed to read config file, because: {0}")]
    ReadConfigFailed(#[from] XlsxError),

    #[error("Invalid project directory")]
    InvalidProjectDir,

    #[error("Failed to fetch metadata")]
    FetchMetadataFailed,

    #[error("Failed in qc result validation")]
    QcFailed,

    #[error("Failed in {0} log validation")]
    LogFailed(String),
}

pub type Result<T> = std::result::Result<T, Error>;

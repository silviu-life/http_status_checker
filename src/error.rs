use thiserror::Error;
use tokio::task::JoinError;

#[derive(Error, Debug)]
pub enum HttpCheckerError {
    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Threading error: {0}")]
    Tokio(#[from] JoinError),
}

pub type Result<T> = std::result::Result<T, HttpCheckerError>;

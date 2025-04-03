use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpCheckerError {
    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, HttpCheckerError>;

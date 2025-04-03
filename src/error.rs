use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpCheckerError {
    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, HttpCheckerError>;

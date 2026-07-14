//! Common types used by data providers.

use std::fmt;

#[derive(Debug)]
pub enum DataError {
    Request(reqwest::Error),
    InvalidResponse(String),
    Parse(String),
    Provider(String),
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataError::Request(err) => write!(f, "HTTP request failed: {}", err),

            DataError::InvalidResponse(msg) => write!(f, "Invalid provider response: {}", msg),

            DataError::Parse(msg) => write!(f, "Failed to parse response: {}", msg),

            DataError::Provider(msg) => write!(f, "Provider error: {}", msg),
        }
    }
}

impl std::error::Error for DataError {}

impl From<reqwest::Error> for DataError {
    fn from(err: reqwest::Error) -> Self {
        DataError::Request(err)
    }
}

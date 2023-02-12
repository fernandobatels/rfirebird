//! Error type and API

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("overflow {value} > {limit}: {msg}")]
    Overflow {
        limit: usize,
        value: usize,
        msg: String,
    },

    #[error("Invalid {desc} page type, expected {expected}, found {tpe}")]
    InvalidPage { tpe: u8, expected: u8, desc: String },

    #[error("error: {0}")]
    Other(String),
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Self::Other(msg)
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Self::Other(msg.to_string())
    }
}

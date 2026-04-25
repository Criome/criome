//! Error type for the criome daemon.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("signal frame decode failed: {0}")]
    SignalDecode(String),

    #[error("validator rejected: {code}: {message}")]
    Rejected { code: String, message: String },

    #[error("sema write failed: {0}")]
    SemaWrite(String),

    #[error("lojix dispatch failed: {0}")]
    LojixDispatch(String),
}

pub type Result<T> = std::result::Result<T, Error>;

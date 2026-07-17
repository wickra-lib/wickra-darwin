//! Error and result types for the evolutionary core.

/// Errors produced by `darwin-core`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A JSON or TOML document failed to parse.
    #[error("parse: {0}")]
    Parse(String),
    /// An indicator name is not in the search allowlist.
    #[error("unknown indicator: {0}")]
    UnknownIndicator(String),
    /// A spec violated a structural invariant.
    #[error("bad spec: {0}")]
    BadSpec(String),
    /// Candle data was missing or malformed.
    #[error("data: {0}")]
    Data(String),
    /// The backtest engine reported an error.
    #[error("backtest: {0}")]
    Backtest(String),
}

/// The crate result type.
pub type Result<T> = core::result::Result<T, Error>;

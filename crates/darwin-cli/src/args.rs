//! Command-line arguments for `wickra-darwin`.

use std::path::PathBuf;

use clap::{Parser, ValueEnum};

/// Evolve trading strategies over candle data with a genetic search.
#[derive(Parser, Debug)]
#[command(name = "wickra-darwin", version, about)]
pub struct Args {
    /// Path to the evolution spec (`.json` or `.toml`).
    #[arg(long, value_name = "PATH")]
    pub spec: PathBuf,

    /// Directory of per-symbol candle files (`<SYMBOL>.csv`).
    #[arg(long, value_name = "DIR", conflicts_with = "stdin")]
    pub data: Option<PathBuf>,

    /// Read candle data as JSON (`{"SYMBOL": [<candle>, ...]}`) from stdin.
    #[arg(long)]
    pub stdin: bool,

    /// Output format.
    #[arg(long, value_enum, default_value_t = Format::Text)]
    pub format: Format,

    /// Override the spec's `top` (how many best strategies to report).
    #[arg(long, value_name = "N")]
    pub top: Option<usize>,

    /// Override the spec's `seed`.
    #[arg(long, value_name = "SEED")]
    pub seed: Option<u64>,
}

/// The output format.
#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Format {
    /// A human-readable table.
    Text,
    /// The raw `EvolveReport` JSON.
    Json,
}

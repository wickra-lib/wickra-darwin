//! Wickra Darwin core — evolutionary strategy search over Wickra strategy specs.
//!
//! A population of [`StrategySpec`] genomes is evolved with genetic operators
//! (mutation + crossover across the indicator search space) and each candidate
//! is scored by the `wickra-backtest` engine. Because the PRNG
//! ([`SplitMix64`](rng::SplitMix64)) lives only in this core and every binding
//! forwards [`Darwin::command_json`] verbatim, a fixed seed yields the
//! byte-identical search on every platform and through every language binding.

// These pedantic casts are pervasive in the numeric search loop, and are the
// same allowances the wickra engine makes.
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::module_name_repetitions
)]

pub mod config;
pub mod darwin;
pub mod error;
pub mod evolve;
pub mod fitness;
pub mod genome;
pub mod rng;
pub mod search_space;
pub mod spec;

pub use config::Config;
pub use darwin::Darwin;
pub use error::{Error, Result};
pub use evolve::{evolve, EvolveReport, GenStats, RankedStrategy};
pub use fitness::Fitness;
pub use genome::{IndicatorGene, ParamRange};
pub use rng::SplitMix64;
pub use search_space::{RuleGrammar, SearchSpace};
pub use spec::EvolveSpec;

// The genome type DARWIN mutates and crosses is the backtest engine's spec.
pub use wickra_backtest::StrategySpec;

/// The crate version.
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// The version of the backtest engine DARWIN evaluates candidates against.
#[must_use]
pub fn engine_version() -> &'static str {
    wickra_backtest::version()
}

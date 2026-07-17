//! Wickra Darwin core — evolutionary strategy search over Wickra strategy specs.
//!
//! Scaffold. The population model, the genetic operators (mutation + crossover
//! over `StrategySpec`s) and the JSON command boundary (`command_json`) land in
//! P-DAR-1; this file pins the crate and its coupling to the backtest engine.

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

#[cfg(test)]
mod tests {
    use super::{engine_version, version};

    #[test]
    fn versions_are_reported() {
        assert!(!version().is_empty());
        assert!(!engine_version().is_empty());
    }
}

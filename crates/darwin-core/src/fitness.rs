//! Fitness: which metric of a backtest is maximised.

use serde::{Deserialize, Serialize};
use wickra_backtest::BacktestReport;

/// The maximised objective. All three are "higher is better".
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Fitness {
    /// Per-bar Sharpe ratio.
    Sharpe,
    /// Absolute profit and loss.
    Pnl,
    /// Calmar ratio (return over max drawdown).
    Calmar,
}

/// Read the selected metric from a report. A non-finite metric (no trades, a
/// degenerate spec) collapses to `NEG_INFINITY` so it is filtered out before it
/// can reach the output or influence selection.
#[must_use]
pub fn score(report: &BacktestReport, fitness: Fitness) -> f64 {
    let raw = match fitness {
        Fitness::Sharpe => report.metrics.sharpe,
        Fitness::Pnl => report.metrics.pnl,
        Fitness::Calmar => report.metrics.calmar,
    };
    if raw.is_finite() {
        raw
    } else {
        f64::NEG_INFINITY
    }
}

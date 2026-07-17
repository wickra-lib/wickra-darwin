#![allow(clippy::cast_precision_loss)]
//! Determinism guarantees: the same seed yields the byte-identical report, and
//! elitism keeps `history[g].best` monotonically non-decreasing.
//!
//! The parallel-vs-sequential byte-equality (the `parallel` feature vs
//! `--no-default-features`) is a CI-matrix property: the golden fixtures in
//! `tests/golden.rs` are a single committed set, so running that test under both
//! feature sets asserts parallel == sequential without a runtime feature switch.

use std::collections::BTreeMap;

use darwin_core::search_space::{RuleGrammar, SearchSpace};
use darwin_core::{evolve, EvolveSpec, Fitness, IndicatorGene, ParamRange};
use wickra_backtest::Candle;

fn candles(n: usize) -> Vec<Candle> {
    (0..n)
        .map(|i| {
            let t = i as f64;
            let close = 100.0 + 10.0 * (t * 0.1).sin() + 0.05 * t;
            let open = 100.0 + 10.0 * ((t - 1.0) * 0.1).sin() + 0.05 * (t - 1.0);
            Candle {
                time: 1_700_000_000 + i64::try_from(i).unwrap() * 3600,
                open,
                high: close.max(open) + 1.0,
                low: close.min(open) - 1.0,
                close,
                volume: 1000.0,
            }
        })
        .collect()
}

fn universe() -> BTreeMap<String, Vec<Candle>> {
    let mut data = BTreeMap::new();
    data.insert("AAA".to_string(), candles(240));
    data.insert("BBB".to_string(), candles(240));
    data
}

fn spec() -> EvolveSpec {
    EvolveSpec {
        seed: 99,
        population: 12,
        generations: 5,
        mutation_rate: 0.25,
        crossover_rate: 0.6,
        fitness: Fitness::Sharpe,
        search_space: SearchSpace {
            indicators: vec![
                IndicatorGene {
                    name: "sma".into(),
                    param_ranges: vec![ParamRange {
                        min: 5.0,
                        max: 30.0,
                        step: 1.0,
                    }],
                },
                IndicatorGene {
                    name: "ema".into(),
                    param_ranges: vec![ParamRange {
                        min: 20.0,
                        max: 100.0,
                        step: 5.0,
                    }],
                },
            ],
            rules: RuleGrammar::CrossoverPair,
            max_conditions: 2,
        },
        elitism: 1,
        top: 5,
    }
}

#[test]
fn same_seed_is_byte_identical() {
    let data = universe();
    let s = spec();
    let a = serde_json::to_string(&evolve(&data, &s).unwrap()).unwrap();
    let b = serde_json::to_string(&evolve(&data, &s).unwrap()).unwrap();
    assert_eq!(a, b, "same seed must give the byte-identical report");
}

#[test]
fn history_best_is_monotonic_with_elitism() {
    let data = universe();
    let report = evolve(&data, &spec()).unwrap();
    for window in report.history.windows(2) {
        assert!(
            window[1].best >= window[0].best - 1e-9,
            "elitism (>=1) keeps the best fitness non-decreasing across generations"
        );
    }
}

#[test]
fn hall_of_fame_respects_top() {
    let data = universe();
    let s = spec();
    let report = evolve(&data, &s).unwrap();
    assert!(
        report.best.len() <= s.top,
        "the hall of fame never exceeds `top`"
    );
}

#![allow(clippy::cast_precision_loss)]
//! Criterion benchmarks for `darwin_core::evolve`.
//!
//! The default build measures the parallel engine; `--no-default-features`
//! measures the single-threaded path (what WASM and the golden fixtures use).
//! Throughput is reported in backtests/second — one backtest per individual per
//! generation per symbol, so `elements = population * generations * symbols`.

use std::collections::BTreeMap;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use darwin_core::search_space::{RuleGrammar, SearchSpace};
use darwin_core::{evolve, EvolveSpec, Fitness, IndicatorGene, ParamRange};
use wickra_backtest::Candle;

const BARS: usize = 200;

fn candles(offset: usize) -> Vec<Candle> {
    (0..BARS)
        .map(|i| {
            let t = (i + offset) as f64;
            let close = 100.0 + 15.0 * (t / 9.0).sin() + 0.03 * t;
            let open = 100.0 + 15.0 * ((t - 1.0) / 9.0).sin() + 0.03 * (t - 1.0);
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

fn universe(symbols: usize) -> BTreeMap<String, Vec<Candle>> {
    (0..symbols)
        .map(|s| (format!("SYM{s:02}"), candles(s * 7)))
        .collect()
}

fn spec(population: usize, generations: usize) -> EvolveSpec {
    EvolveSpec {
        seed: 1,
        population,
        generations,
        mutation_rate: 0.2,
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

fn bench_evolve(c: &mut Criterion) {
    let mut group = c.benchmark_group("evolve");
    for &population in &[16usize, 64, 256] {
        for &generations in &[5usize, 20] {
            for &symbols in &[3usize, 10] {
                let data = universe(symbols);
                let s = spec(population, generations);
                // One backtest per (individual, generation, symbol).
                let backtests = (population * generations.max(1) * symbols) as u64;
                group.throughput(Throughput::Elements(backtests));
                let id = BenchmarkId::from_parameter(format!(
                    "pop={population}/gen={generations}/sym={symbols}"
                ));
                group.bench_with_input(id, &(data, s), |b, (data, s)| {
                    b.iter(|| evolve(data, s).unwrap());
                });
            }
        }
    }
    group.finish();
}

criterion_group!(benches, bench_evolve);
criterion_main!(benches);

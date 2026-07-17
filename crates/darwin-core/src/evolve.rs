//! The evolution loop: seed a population, score each candidate with the
//! backtest engine, rank, select, reproduce — deterministically from the seed.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use wickra_backtest::{run, Candle, StrategySpec};

use crate::error::{Error, Result};
use crate::fitness::score;
use crate::genome::{canonicalize, crossover, round8, spec_hash, to_strategy_spec, Genome};
use crate::rng::SplitMix64;
use crate::search_space::{mutate, sample_spec};
use crate::spec::EvolveSpec;

/// The fixed timeframe every generated spec carries.
const TIMEFRAME: &str = "1h";

/// One ranked strategy in the report.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RankedStrategy {
    /// The found `StrategySpec` (canonical JSON, runnable as-is).
    pub spec: Value,
    /// Its fitness.
    pub fitness: f64,
    /// The generation it was first seen in.
    pub generation: usize,
    /// FNV-1a-64 hex of the canonical spec.
    pub spec_hash: String,
}

/// Per-generation summary statistics.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GenStats {
    /// The generation index.
    pub generation: usize,
    /// Best finite fitness.
    pub best: f64,
    /// Mean of the finite fitness values.
    pub mean: f64,
    /// Worst finite fitness.
    pub worst: f64,
    /// Number of individuals with finite fitness.
    pub evaluated: usize,
}

/// The full evolution report.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EvolveReport {
    /// The globally best strategies (deduplicated, best-first).
    pub best: Vec<RankedStrategy>,
    /// One entry per evaluated generation.
    pub history: Vec<GenStats>,
}

/// Score one genome across every symbol: the mean of the finite per-symbol
/// metric values, or `NEG_INFINITY` if none is finite. No RNG is involved, so
/// the parallel and sequential evaluations are identical.
fn evaluate(genome: &Genome, data: &BTreeMap<String, Vec<Candle>>, spec: &EvolveSpec) -> f64 {
    let mut sum = 0.0;
    let mut count = 0u32;
    for (symbol, candles) in data {
        let value = to_strategy_spec(genome, symbol, TIMEFRAME);
        let Ok(strategy) = serde_json::from_value::<StrategySpec>(value) else {
            continue;
        };
        let Ok(report) = run(&strategy, candles) else {
            continue;
        };
        let s = score(&report, spec.fitness);
        if s.is_finite() {
            sum += s;
            count += 1;
        }
    }
    if count > 0 {
        round8(sum / f64::from(count))
    } else {
        f64::NEG_INFINITY
    }
}

#[cfg(feature = "parallel")]
fn eval_population(
    pop: &[Genome],
    data: &BTreeMap<String, Vec<Candle>>,
    spec: &EvolveSpec,
) -> Vec<f64> {
    use rayon::prelude::*;
    pop.par_iter().map(|g| evaluate(g, data, spec)).collect()
}

#[cfg(not(feature = "parallel"))]
fn eval_population(
    pop: &[Genome],
    data: &BTreeMap<String, Vec<Candle>>,
    spec: &EvolveSpec,
) -> Vec<f64> {
    pop.iter().map(|g| evaluate(g, data, spec)).collect()
}

/// Rank indices by `(fitness desc, spec_hash asc)`.
fn ranked_order(fits: &[f64], hashes: &[String]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..fits.len()).collect();
    order.sort_by(|&i, &j| {
        fits[j]
            .partial_cmp(&fits[i])
            .unwrap_or(Ordering::Equal)
            .then_with(|| hashes[i].cmp(&hashes[j]))
    });
    order
}

fn gen_stats(generation: usize, order: &[usize], fits: &[f64]) -> GenStats {
    let finite: Vec<f64> = order
        .iter()
        .map(|&i| fits[i])
        .filter(|x| x.is_finite())
        .collect();
    if finite.is_empty() {
        return GenStats {
            generation,
            best: 0.0,
            mean: 0.0,
            worst: 0.0,
            evaluated: 0,
        };
    }
    let sum: f64 = finite.iter().sum();
    GenStats {
        generation,
        best: round8(finite[0]),
        mean: round8(sum / finite.len() as f64),
        worst: round8(*finite.last().unwrap()),
        evaluated: finite.len(),
    }
}

/// Evolve a population of strategies over `data`, deterministically from the
/// spec's seed.
///
/// # Errors
/// Returns an error if the spec is invalid or the data is empty.
pub fn evolve(data: &BTreeMap<String, Vec<Candle>>, spec: &EvolveSpec) -> Result<EvolveReport> {
    spec.validate()?;
    let Some(primary) = data.keys().next() else {
        return Err(Error::Data("no candle data provided".into()));
    };

    let mut rng = SplitMix64::new(spec.seed);
    let mut population: Vec<Genome> = (0..spec.population)
        .map(|_| sample_spec(&mut rng, &spec.search_space))
        .collect();

    let mut history: Vec<GenStats> = Vec::with_capacity(spec.generations + 1);
    let mut hall: Vec<RankedStrategy> = Vec::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();

    for generation in 0..=spec.generations {
        let fits = eval_population(&population, data, spec);
        let hashes: Vec<String> = population
            .iter()
            .map(|g| spec_hash(&to_strategy_spec(g, primary, TIMEFRAME)))
            .collect();
        let order = ranked_order(&fits, &hashes);

        history.push(gen_stats(generation, &order, &fits));

        for &idx in &order {
            if !fits[idx].is_finite() || !seen.insert(hashes[idx].clone()) {
                continue;
            }
            hall.push(RankedStrategy {
                spec: canonicalize(&to_strategy_spec(&population[idx], primary, TIMEFRAME)),
                fitness: round8(fits[idx]),
                generation,
                spec_hash: hashes[idx].clone(),
            });
        }

        if generation == spec.generations {
            break;
        }

        // Reproduce the next generation.
        let mut rank_pos = vec![0usize; population.len()];
        for (pos, &idx) in order.iter().enumerate() {
            rank_pos[idx] = pos;
        }
        let pop_n = population.len() as u64;
        let mut next: Vec<Genome> = order
            .iter()
            .take(spec.elitism)
            .map(|&i| population[i].clone())
            .collect();
        while next.len() < population.len() {
            let p1 = tournament(&mut rng, pop_n, &rank_pos);
            let p2 = tournament(&mut rng, pop_n, &rank_pos);
            let mut child = if rng.next_f64() < spec.crossover_rate {
                crossover(&population[p1], &population[p2], &mut rng)
            } else {
                population[p1].clone()
            };
            child = mutate(&child, &spec.search_space, spec.mutation_rate, &mut rng);
            next.push(child);
        }
        population = next;
    }

    hall.sort_by(|a, b| {
        b.fitness
            .partial_cmp(&a.fitness)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.spec_hash.cmp(&b.spec_hash))
    });
    hall.truncate(spec.top);

    Ok(EvolveReport {
        best: hall,
        history,
    })
}

/// Size-2 tournament: draw two population indices, return the better-ranked.
fn tournament(rng: &mut SplitMix64, pop_n: u64, rank_pos: &[usize]) -> usize {
    let a = rng.below(pop_n) as usize;
    let b = rng.below(pop_n) as usize;
    if rank_pos[a] <= rank_pos[b] {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::{evolve, EvolveSpec};
    use crate::fitness::Fitness;
    use crate::genome::{IndicatorGene, ParamRange};
    use crate::search_space::{RuleGrammar, SearchSpace};
    use std::collections::BTreeMap;
    use wickra_backtest::{Candle, StrategySpec};

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

    fn spec() -> EvolveSpec {
        EvolveSpec {
            seed: 1,
            population: 8,
            generations: 3,
            mutation_rate: 0.2,
            crossover_rate: 0.6,
            fitness: Fitness::Sharpe,
            search_space: SearchSpace {
                indicators: vec![
                    IndicatorGene {
                        name: "rsi".into(),
                        param_ranges: vec![ParamRange {
                            min: 2.0,
                            max: 30.0,
                            step: 1.0,
                        }],
                    },
                    IndicatorGene {
                        name: "ema".into(),
                        param_ranges: vec![ParamRange {
                            min: 5.0,
                            max: 100.0,
                            step: 5.0,
                        }],
                    },
                ],
                rules: RuleGrammar::ConjunctionAll,
                max_conditions: 2,
            },
            elitism: 1,
            top: 3,
        }
    }

    #[test]
    fn evolve_is_deterministic_and_produces_valid_specs() {
        let mut data = BTreeMap::new();
        data.insert("AAA".to_string(), candles(250));
        data.insert("BBB".to_string(), candles(250));
        let s = spec();
        let r1 = evolve(&data, &s).unwrap();
        let r2 = evolve(&data, &s).unwrap();
        assert_eq!(r1, r2, "same seed must give identical reports");
        assert_eq!(r1.history.len(), s.generations + 1);
        for ranked in &r1.best {
            let parsed: Result<StrategySpec, _> = serde_json::from_value(ranked.spec.clone());
            assert!(parsed.is_ok(), "every best spec is a valid StrategySpec");
            assert!(ranked.fitness.is_finite());
        }
    }

    #[test]
    fn history_best_is_monotonic_with_elitism() {
        let mut data = BTreeMap::new();
        data.insert("AAA".to_string(), candles(250));
        let report = evolve(&data, &spec()).unwrap();
        for window in report.history.windows(2) {
            assert!(
                window[1].best >= window[0].best - 1e-9,
                "elitism keeps the best fitness non-decreasing"
            );
        }
    }

    #[test]
    fn empty_data_errors() {
        let data: BTreeMap<String, Vec<Candle>> = BTreeMap::new();
        assert!(evolve(&data, &spec()).is_err());
    }
}

//! The search has to actually evaluate the strategies it samples.
//!
//! `evaluate` builds a `StrategySpec` from each genome, hands it to
//! `wickra_backtest::run`, and scores the report. Every one of those steps drops
//! a candidate with `continue` when it fails, and a candidate that scores nothing
//! becomes `NEG_INFINITY` -- which is not finite, so it is filtered out of the
//! generation statistics *and* out of the hall of fame.
//!
//! That failure is silent by construction: a run in which nothing deserialised
//! returns a well-formed report with `evaluated: 0`, `best: 0.0` and an empty
//! `best` list, and every binding prints it without complaint. It is what this
//! repository did until `to_strategy_spec` learned to spell an operand the way
//! `wickra_backtest::Operand` reads it -- an untagged enum whose `Ref` variant is
//! a bare string and whose `Const` variant is a bare number, not `{"ref": ...}`
//! and `{"const": ...}`.
//!
//! The existing tests could not see it. The one that walks `report.best` to
//! check each spec parses iterated an empty list, and the monotonic-fitness test
//! compared a history of zeros against itself.

use std::collections::BTreeMap;

use wickra_backtest::Candle;
use wickra_darwin_core::{evolve, EvolveSpec};

const SPEC: &str = r#"{
    "seed": 11, "population": 10, "generations": 4,
    "mutation_rate": 0.2, "crossover_rate": 0.6, "fitness": "sharpe",
    "search_space": {
        "indicators": [{ "name": "rsi", "param_ranges": [{ "min": 2, "max": 30 }] }],
        "rules": "single_threshold", "max_conditions": 1
    },
    "elitism": 1, "top": 3
}"#;

/// A path with enough shape for an RSI rule to fire on it.
fn candles() -> Vec<Candle> {
    (0..64)
        .map(|i| {
            let t = f64::from(i);
            let close = 100.0 + (t * 0.35).sin() * 8.0 + t * 0.1;
            let open = 100.0 + ((t - 1.0) * 0.35).sin() * 8.0 + (t - 1.0) * 0.1;
            Candle {
                time: 1_700_000_000 + i64::from(i) * 3600,
                open,
                high: open.max(close) + 1.0,
                low: open.min(close) - 1.0,
                close,
                volume: 1000.0,
            }
        })
        .collect()
}

fn spec() -> EvolveSpec {
    serde_json::from_str(SPEC).expect("the spec parses")
}

fn data() -> BTreeMap<String, Vec<Candle>> {
    let mut map = BTreeMap::new();
    map.insert("SYM".to_string(), candles());
    map
}

#[test]
fn every_generation_evaluates_its_whole_population() {
    let spec = spec();
    let report = evolve(&data(), &spec).expect("evolve runs");

    assert_eq!(
        report.history.len(),
        spec.generations + 1,
        "one statistics row per generation, plus the initial one"
    );
    for gen in &report.history {
        assert_eq!(
            gen.evaluated, spec.population,
            "generation {} scored {} of {} candidates; a candidate that fails to \
             deserialise or run is dropped silently",
            gen.generation, gen.evaluated, spec.population
        );
    }
}

#[test]
fn the_hall_of_fame_is_filled() {
    let spec = spec();
    let report = evolve(&data(), &spec).expect("evolve runs");

    assert_eq!(
        report.best.len(),
        spec.top,
        "the spec asks for {} best strategies and the search is expected to find \
         that many distinct ones",
        spec.top
    );
    for ranked in &report.best {
        assert!(
            ranked.fitness.is_finite(),
            "a ranked strategy carries a real fitness, not the NEG_INFINITY a \
             candidate gets when nothing scored: {ranked:?}"
        );
    }
}

#[test]
fn a_sampled_strategy_is_one_the_engine_accepts() {
    // The narrow version of the same property, at the seam where it broke: the
    // JSON a genome serialises to has to round-trip into the engine's own type.
    use wickra_backtest::StrategySpec;
    use wickra_darwin_core::genome::to_strategy_spec;
    use wickra_darwin_core::rng::SplitMix64;
    use wickra_darwin_core::search_space::sample_spec;

    let spec = spec();
    let mut rng = SplitMix64::new(spec.seed);
    for candidate in 0..spec.population {
        let genome = sample_spec(&mut rng, &spec.search_space);
        let value = to_strategy_spec(&genome, "SYM", "1h");
        let json = serde_json::to_string(&value).expect("the value serialises");
        serde_json::from_value::<StrategySpec>(value).unwrap_or_else(|err| {
            panic!("candidate {candidate} is not a StrategySpec: {err}\n  {json}")
        });
    }
}

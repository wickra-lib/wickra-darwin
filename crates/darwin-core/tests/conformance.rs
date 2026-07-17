//! Serde and validation conformance: every public enum variant and struct
//! round-trips through JSON, the search-space validator rejects unknown
//! indicators and wrong arities, and the SplitMix64 stream is deterministic and
//! seed-sensitive.

use darwin_core::genome::{IndicatorGene, ParamRange};
use darwin_core::search_space::{RuleGrammar, SearchSpace};
use darwin_core::{EvolveSpec, Fitness, SplitMix64};

fn json_round_trip<T>(value: &T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let text = serde_json::to_string(value).expect("serialize");
    let back: T = serde_json::from_str(&text).expect("deserialize");
    assert_eq!(&back, value, "JSON round-trip must be stable");
}

#[test]
fn fitness_variants_round_trip() {
    for f in [Fitness::Sharpe, Fitness::Pnl, Fitness::Calmar] {
        json_round_trip(&f);
    }
    // Fitness serializes snake_case.
    assert_eq!(
        serde_json::to_string(&Fitness::Sharpe).unwrap(),
        "\"sharpe\""
    );
    assert_eq!(serde_json::to_string(&Fitness::Pnl).unwrap(), "\"pnl\"");
    assert_eq!(
        serde_json::to_string(&Fitness::Calmar).unwrap(),
        "\"calmar\""
    );
}

#[test]
fn rule_grammar_variants_round_trip() {
    for r in [
        RuleGrammar::SingleThreshold,
        RuleGrammar::CrossoverPair,
        RuleGrammar::ConjunctionAll,
    ] {
        json_round_trip(&r);
    }
    assert_eq!(
        serde_json::to_string(&RuleGrammar::SingleThreshold).unwrap(),
        "\"single_threshold\""
    );
    assert_eq!(
        serde_json::to_string(&RuleGrammar::CrossoverPair).unwrap(),
        "\"crossover_pair\""
    );
    assert_eq!(
        serde_json::to_string(&RuleGrammar::ConjunctionAll).unwrap(),
        "\"conjunction_all\""
    );
}

fn sample_search_space() -> SearchSpace {
    SearchSpace {
        indicators: vec![IndicatorGene {
            name: "rsi".into(),
            param_ranges: vec![ParamRange {
                min: 2.0,
                max: 30.0,
                step: 1.0,
            }],
        }],
        rules: RuleGrammar::SingleThreshold,
        max_conditions: 1,
    }
}

#[test]
fn structs_round_trip() {
    json_round_trip(&ParamRange {
        min: 1.0,
        max: 10.0,
        step: 0.5,
    });
    json_round_trip(&IndicatorGene {
        name: "ema".into(),
        param_ranges: vec![ParamRange {
            min: 5.0,
            max: 50.0,
            step: 1.0,
        }],
    });
    json_round_trip(&sample_search_space());

    let spec = EvolveSpec {
        seed: 7,
        population: 12,
        generations: 4,
        mutation_rate: 0.25,
        crossover_rate: 0.6,
        fitness: Fitness::Pnl,
        search_space: sample_search_space(),
        elitism: 1,
        top: 3,
    };
    json_round_trip(&spec);
}

#[test]
fn validate_accepts_known_indicators() {
    assert!(sample_search_space().validate().is_ok());
}

#[test]
fn validate_rejects_unknown_indicator() {
    let mut sp = sample_search_space();
    sp.indicators[0].name = "not_an_indicator".into();
    assert!(sp.validate().is_err(), "unknown indicator must be rejected");
}

#[test]
fn validate_rejects_wrong_arity() {
    let mut sp = sample_search_space();
    // rsi takes one parameter range; two is the wrong arity.
    sp.indicators[0].param_ranges.push(ParamRange {
        min: 1.0,
        max: 2.0,
        step: 1.0,
    });
    assert!(sp.validate().is_err(), "wrong arity must be rejected");
}

#[test]
fn splitmix64_is_deterministic_and_seed_sensitive() {
    let seq = |seed: u64| {
        let mut rng = SplitMix64::new(seed);
        core::array::from_fn::<u64, 8, _>(|_| rng.next_u64())
    };
    // Deterministic: the same seed yields the same stream.
    assert_eq!(seq(42), seq(42));
    // Seed-sensitive: a different seed yields a different stream.
    assert_ne!(seq(42), seq(43));
    // The stream is not trivially constant.
    let s = seq(42);
    assert!(s.windows(2).any(|w| w[0] != w[1]));
}

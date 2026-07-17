#![allow(clippy::cast_precision_loss, clippy::float_cmp)]
//! Property-based invariants of the evolutionary search: over randomised specs
//! and price paths, `evolve` never panics, and its report always respects the
//! structural guarantees — bounded hall of fame, valid strategy specs, finite
//! fitness, canonical ranking, and stable `spec_hash`.

use std::collections::BTreeMap;

use darwin_core::genome::{spec_hash, IndicatorGene, ParamRange};
use darwin_core::search_space::{RuleGrammar, SearchSpace};
use darwin_core::{evolve, EvolveSpec, Fitness};
use proptest::prelude::*;
use wickra_backtest::{Candle, StrategySpec};

fn candles(n: usize, base: f64, amp: f64, k: f64, drift: f64) -> Vec<Candle> {
    (0..n)
        .map(|i| {
            let t = i as f64;
            let close = base + amp * (t / k).sin() + drift * t;
            let open = base + amp * ((t - 1.0) / k).sin() + drift * (t - 1.0);
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

prop_compose! {
    fn arb_spec()(
        seed in any::<u64>(),
        population in 2usize..12,
        generations in 1usize..5,
        mutation_rate in 0.0f64..1.0,
        crossover_rate in 0.0f64..1.0,
        fitness in prop_oneof![Just(Fitness::Sharpe), Just(Fitness::Pnl), Just(Fitness::Calmar)],
        rules in prop_oneof![
            Just(RuleGrammar::SingleThreshold),
            Just(RuleGrammar::CrossoverPair),
            Just(RuleGrammar::ConjunctionAll),
        ],
        indicator in prop_oneof![Just("sma"), Just("ema"), Just("rsi"), Just("atr")],
        lo in 2.0f64..15.0,
        span in 5.0f64..40.0,
        elitism in 0usize..3,
        top in 1usize..6,
    ) -> EvolveSpec {
        EvolveSpec {
            seed,
            population,
            generations,
            mutation_rate,
            crossover_rate,
            fitness,
            search_space: SearchSpace {
                indicators: vec![
                    IndicatorGene { name: indicator.into(), param_ranges: vec![ParamRange { min: lo, max: lo + span, step: 1.0 }] },
                    IndicatorGene { name: "ema".into(), param_ranges: vec![ParamRange { min: 10.0, max: 60.0, step: 5.0 }] },
                ],
                rules,
                max_conditions: 3,
            },
            elitism,
            top,
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(48))]

    #[test]
    fn evolve_upholds_report_invariants(spec in arb_spec()) {
        let mut data = BTreeMap::new();
        data.insert("SYM1".to_string(), candles(180, 100.0, 20.0, 8.0, 0.03));
        data.insert("SYM2".to_string(), candles(180, 130.0, 12.0, 5.0, 0.00));

        // Never panics; a valid spec always yields Ok.
        let report = evolve(&data, &spec).expect("evolve on a valid spec must succeed");

        // Exactly generations + 1 history rows.
        prop_assert_eq!(report.history.len(), spec.generations + 1);

        // The hall of fame is bounded by `top`.
        prop_assert!(report.best.len() <= spec.top);

        // Every hall-of-fame entry is a valid StrategySpec, has finite fitness,
        // and its spec_hash is the stable hash of its spec.
        for ranked in &report.best {
            let parsed: Result<StrategySpec, _> = serde_json::from_value(ranked.spec.clone());
            prop_assert!(parsed.is_ok(), "every best spec parses as a StrategySpec");
            prop_assert!(ranked.fitness.is_finite(), "no NaN/inf fitness in the output");
            prop_assert_eq!(&ranked.spec_hash, &spec_hash(&ranked.spec));
        }

        // The hall of fame is ranked by (fitness desc, spec_hash asc).
        for w in report.best.windows(2) {
            let ordered = w[0].fitness > w[1].fitness
                || (w[0].fitness == w[1].fitness && w[0].spec_hash <= w[1].spec_hash);
            prop_assert!(ordered, "hall of fame is sorted by (fitness desc, spec_hash asc)");
        }

        // Determinism: a second run yields the byte-identical report.
        let again = evolve(&data, &spec).unwrap();
        prop_assert_eq!(
            serde_json::to_string(&report).unwrap(),
            serde_json::to_string(&again).unwrap()
        );
    }
}

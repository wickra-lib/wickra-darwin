#![no_main]
//! Fuzz crossover and mutation: two genomes sampled from a validated space are
//! recombined and mutated. Neither operation may panic, and the child is always
//! a well-formed StrategySpec-shaped value.

use libfuzzer_sys::fuzz_target;
use darwin_core::search_space::{mutate, sample_spec, SearchSpace};
use darwin_core::genome::{crossover, to_strategy_spec};
use darwin_core::SplitMix64;

fuzz_target!(|data: &[u8]| {
    if data.len() < 8 {
        return;
    }
    let seed = u64::from_le_bytes(data[..8].try_into().unwrap());
    let Ok(text) = std::str::from_utf8(&data[8..]) else {
        return;
    };
    let Ok(space) = serde_json::from_str::<SearchSpace>(text) else {
        return;
    };
    if space.validate().is_err() {
        return;
    }
    let mut rng = SplitMix64::new(seed);
    let a = sample_spec(&mut rng, &space);
    let b = sample_spec(&mut rng, &space);
    let child = crossover(&a, &b, &mut rng);
    let mutated = mutate(&child, &space, 0.5, &mut rng);
    // Both stages produce serializable, StrategySpec-shaped values.
    let _ = to_strategy_spec(&child, "SYM", "1h");
    let _ = to_strategy_spec(&mutated, "SYM", "1h");
});

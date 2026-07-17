#![no_main]
//! Fuzz genome sampling: a parsed, validated `SearchSpace` is sampled with a
//! seed-derived RNG. Sampling a validated space must never panic and must
//! produce a StrategySpec-shaped JSON value.

use libfuzzer_sys::fuzz_target;
use darwin_core::search_space::{sample_spec, SearchSpace};
use darwin_core::genome::{spec_hash, to_strategy_spec};
use darwin_core::SplitMix64;

fuzz_target!(|data: &[u8]| {
    // First 8 bytes seed the RNG; the rest is the SearchSpace JSON.
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
    let genome = sample_spec(&mut rng, &space);
    let spec = to_strategy_spec(&genome, "SYM", "1h");
    // The spec hash is stable for the sampled genome.
    assert_eq!(spec_hash(&spec), spec_hash(&spec));
});

#![no_main]
//! Fuzz a full evolution run: a parsed spec (with population and generations
//! clamped so the run stays cheap) drives `evolve` over a fixed tiny universe via
//! the JSON command surface. No input may panic; a domain error comes back
//! in-band as JSON.

use libfuzzer_sys::fuzz_target;
use darwin_core::{Darwin, EvolveSpec};

// A fixed, tiny two-symbol universe so the fuzzer varies the spec, not the data.
const EVOLVE_CMD: &str = r#"{"cmd":"evolve","data":{"AAA":[
{"time":1700000000,"open":100,"high":101,"low":99,"close":100.5,"volume":10},
{"time":1700003600,"open":100.5,"high":102,"low":100,"close":101.5,"volume":10},
{"time":1700007200,"open":101.5,"high":103,"low":101,"close":102.0,"volume":10}
]}}"#;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let Ok(mut spec) = EvolveSpec::from_json(text) else {
        return;
    };
    // Clamp the cost drivers so the fuzzer cannot force a huge run.
    spec.population = spec.population.clamp(2, 16);
    spec.generations = spec.generations.min(4);
    if spec.search_space.validate().is_err() {
        return;
    }
    let spec_json = serde_json::to_string(&spec).expect("re-serialize clamped spec");
    let Ok(mut darwin) = Darwin::new(&spec_json) else {
        return;
    };
    // A command failure is returned as a Result; a panic would be a bug.
    let _ = darwin.command_json(EVOLVE_CMD);
});

//! The CLI's config wrapper and the two version functions.
//!
//! `Config` is what `--config` reads, in either of the two formats the CLI
//! accepts, and the version pair is what `version` answers with -- the crate's
//! own and the backtest engine it evaluates candidates against, which is the
//! number a report has to be reproducible under. Both are public surface the
//! Rust suite never touched: the CLI exercises them, and the CLI is not what
//! coverage measures.

use std::fs;
use std::path::PathBuf;

use wickra_darwin_core::{engine_version, version, Config, EvolveSpec};

fn a_spec_json() -> String {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../golden/specs");
    let mut specs: Vec<_> = fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    specs.sort();
    fs::read_to_string(&specs[0]).unwrap()
}

#[test]
fn a_config_reads_as_json() {
    let spec = a_spec_json();
    let config = Config::from_json(&format!(r#"{{"spec":{spec}}}"#)).expect("a config parses");
    // The spec inside is the spec itself, not a copy that drifted in parsing.
    let direct: EvolveSpec = serde_json::from_str(&spec).unwrap();
    assert_eq!(
        serde_json::to_value(&config.spec).unwrap(),
        serde_json::to_value(&direct).unwrap()
    );
    config.spec.validate().expect("a golden spec validates");
}

#[test]
fn a_config_reads_as_toml() {
    // TOML is the other format `--config` accepts, written the way a person
    // writes one: the scalars of the spec, then its search space as a table.
    let toml_text = r#"
[spec]
seed = 7
population = 8
generations = 2
mutation_rate = 0.2
crossover_rate = 0.5
fitness = "sharpe"
elitism = 1
top = 3

[spec.search_space]
rules = "single_threshold"
max_conditions = 1

[[spec.search_space.indicators]]
name = "sma"

[[spec.search_space.indicators.param_ranges]]
min = 5
max = 60
"#;
    let config = Config::from_toml(toml_text).expect("a TOML config parses");
    config.spec.validate().expect("and validates");
    assert_eq!(config.spec.seed, 7);
    assert_eq!(config.spec.population, 8);
    assert_eq!(config.spec.top, 3);
    // The same document as JSON is the same spec: the two formats are two
    // spellings of one config, which is the only reason to accept both.
    let as_json = serde_json::to_string(&config.spec).unwrap();
    assert_eq!(EvolveSpec::from_json(&as_json).unwrap(), config.spec);
}

#[test]
fn malformed_configs_are_refused_in_both_formats() {
    let json_err = Config::from_json("{ not json").expect_err("malformed JSON is refused");
    assert!(!json_err.to_string().is_empty());
    let toml_err = Config::from_toml("[spec\nbroken").expect_err("malformed TOML is refused");
    assert!(!toml_err.to_string().is_empty());
    // A well-formed document that is not a config is refused too.
    assert!(Config::from_json(r#"{"nope":1}"#).is_err());
}

#[test]
fn the_versions_are_the_crate_and_the_engine() {
    assert_eq!(version(), env!("CARGO_PKG_VERSION"));
    assert_eq!(engine_version(), wickra_backtest::version());
    // Both read like versions; an empty string would pass a weaker assertion.
    for v in [version(), engine_version()] {
        assert_eq!(v.split('.').count(), 3, "{v} is a three-part version");
        assert!(v.split('.').all(|p| p.parse::<u32>().is_ok()), "{v}");
    }
}

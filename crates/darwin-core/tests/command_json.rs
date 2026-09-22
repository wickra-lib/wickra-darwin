//! The `command_json` envelope, from Rust.
//!
//! Every binding forwards its calls to this one function, so the ten language
//! suites all exercise it -- and none of them is Rust, which left the boundary
//! itself unmeasured while the code behind it was covered several times over.
//! This drives each command and each refusal directly: the handle's two
//! constructors, `set_spec`, `evolve`, `best` before and after a run, `version`,
//! and every error the envelope can produce.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use serde_json::{json, Value};
use wickra_backtest::data::load_candles;
use wickra_backtest::Candle;
use wickra_darwin_core::{Darwin, EvolveSpec};

fn golden_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../golden")
}

/// Two symbols of the golden universe: enough for a real evolution, small
/// enough that the test stays a test.
fn universe() -> BTreeMap<String, Vec<Candle>> {
    let mut out = BTreeMap::new();
    for symbol in ["sym-01", "sym-02"] {
        let path = golden_dir().join("data").join(format!("{symbol}.csv"));
        out.insert(
            symbol.to_owned(),
            load_candles(&path).unwrap_or_else(|e| panic!("load {}: {e}", path.display())),
        );
    }
    out
}

fn a_spec() -> Value {
    let path = golden_dir().join("specs");
    let mut specs: Vec<_> = fs::read_dir(&path)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    specs.sort();
    let text = fs::read_to_string(&specs[0]).unwrap();
    serde_json::from_str(&text).unwrap()
}

#[test]
fn an_empty_spec_defers_configuration() {
    for empty in ["", "  ", "{}"] {
        let mut handle = Darwin::new(empty).expect("empty spec is allowed");
        let err = handle
            .command_json(&json!({ "cmd": "evolve", "data": {} }).to_string())
            .expect_err("evolve without a spec is refused");
        assert!(err.to_string().contains("no spec set"), "{err}");
    }
}

#[test]
fn a_malformed_spec_is_refused_at_construction() {
    let Err(err) = Darwin::new("{ not json") else {
        panic!("invalid JSON is refused");
    };
    assert!(!err.to_string().is_empty());
}

#[test]
fn version_answers_the_crate_version() {
    let mut handle = Darwin::new("{}").unwrap();
    let out: Value = serde_json::from_str(&handle.command_json(r#"{"cmd":"version"}"#).unwrap())
        .expect("version answers JSON");
    assert_eq!(out["version"], Darwin::version());
    assert_eq!(out["version"], env!("CARGO_PKG_VERSION"));
}

#[test]
fn set_spec_then_evolve_then_best() {
    let mut handle = Darwin::new("{}").unwrap();

    let ack: Value = serde_json::from_str(
        &handle
            .command_json(&json!({ "cmd": "set_spec", "spec": a_spec() }).to_string())
            .expect("set_spec"),
    )
    .unwrap();
    assert_eq!(ack["ok"], true);

    // `best` before a run is a refusal, not an empty list: nothing has been
    // ranked yet, and saying "none" would read as "nothing scored".
    let err = handle
        .command_json(r#"{"cmd":"best","n":3}"#)
        .expect_err("best before evolve is refused");
    assert!(err.to_string().contains("no evolve run yet"), "{err}");

    let data = serde_json::to_value(universe()).unwrap();
    let report: Value = serde_json::from_str(
        &handle
            .command_json(&json!({ "cmd": "evolve", "data": data }).to_string())
            .expect("evolve"),
    )
    .unwrap();
    let ranked = report["best"].as_array().expect("report carries best");
    assert!(
        !ranked.is_empty(),
        "an evolution ranks at least one strategy"
    );

    let best: Value =
        serde_json::from_str(&handle.command_json(r#"{"cmd":"best","n":2}"#).unwrap()).unwrap();
    let top = best["best"].as_array().unwrap();
    assert!(top.len() <= 2, "n bounds the answer");
    assert_eq!(top[0], ranked[0], "best is the report's own ranking");

    // No `n` means every ranked strategy.
    let all: Value =
        serde_json::from_str(&handle.command_json(r#"{"cmd":"best"}"#).unwrap()).unwrap();
    assert_eq!(all["best"].as_array().unwrap().len(), ranked.len());
}

#[test]
fn the_handle_can_be_constructed_with_its_spec() {
    let mut handle = Darwin::new(&a_spec().to_string()).expect("a valid spec constructs");
    let data = serde_json::to_value(universe()).unwrap();
    let out = handle
        .command_json(&json!({ "cmd": "evolve", "data": data }).to_string())
        .expect("evolve runs off the constructor's spec");
    assert!(out.contains("\"best\""));
}

#[test]
fn set_spec_rebinds_the_handle() {
    let mut handle = Darwin::new("{}").unwrap();
    let spec: EvolveSpec = serde_json::from_value(a_spec()).unwrap();
    handle.set_spec(spec);
    let data = serde_json::to_value(universe()).unwrap();
    assert!(handle
        .command_json(&json!({ "cmd": "evolve", "data": data }).to_string())
        .is_ok());
    assert!(!handle.best(1).expect("a run has happened").is_empty());
}

#[test]
fn every_malformed_envelope_is_named() {
    let mut handle = Darwin::new("{}").unwrap();
    let cases = [
        ("{ not json", "parse"),
        (r#"{"no":"cmd"}"#, "missing cmd"),
        (r#"{"cmd":"fly"}"#, "unknown cmd: fly"),
        (r#"{"cmd":"set_spec"}"#, "set_spec requires a spec"),
        (r#"{"cmd":"set_spec","spec":{"nope":1}}"#, ""),
    ];
    for (envelope, expected) in cases {
        let err = handle
            .command_json(envelope)
            .expect_err("a malformed envelope is refused");
        assert!(
            expected.is_empty() || err.to_string().contains(expected),
            "{envelope} -> {err}"
        );
    }

    // With a spec set, `evolve` still needs its data, and the data still has to
    // be candles.
    handle
        .command_json(&json!({ "cmd": "set_spec", "spec": a_spec() }).to_string())
        .unwrap();
    let err = handle
        .command_json(r#"{"cmd":"evolve"}"#)
        .expect_err("evolve without data is refused");
    assert!(err.to_string().contains("evolve requires data"), "{err}");
    let err = handle
        .command_json(r#"{"cmd":"evolve","data":{"sym":[{"open":1}]}}"#)
        .expect_err("evolve over non-candles is refused");
    assert!(!err.to_string().is_empty());
}

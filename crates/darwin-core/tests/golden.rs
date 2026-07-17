//! The golden invariant, from Rust: every `golden/specs/*.json` regenerates its
//! `golden/expected/*.json` byte-for-byte over the fixed `golden/data/*.csv`
//! universe. `serde_json::to_string(&evolve(..))` is exactly what the CLI's
//! `--format json` and every language binding produce, so this file is the
//! reference the whole cross-language corpus is pinned to.
//!
//! A missing expected file is written (bless mode) so the corpus can be
//! regenerated after an intended change; a present file is asserted byte-for-byte.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use darwin_core::{evolve, EvolveSpec};
use wickra_backtest::data::load_candles;
use wickra_backtest::Candle;

fn golden_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../golden")
}

fn load_universe(dir: &Path) -> BTreeMap<String, Vec<Candle>> {
    let mut universe = BTreeMap::new();
    let mut paths: Vec<_> = fs::read_dir(dir.join("data"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "csv"))
        .collect();
    paths.sort();
    for path in paths {
        let symbol = path.file_stem().unwrap().to_str().unwrap().to_owned();
        let candles =
            load_candles(&path).unwrap_or_else(|e| panic!("load {}: {e}", path.display()));
        universe.insert(symbol, candles);
    }
    universe
}

#[test]
fn every_spec_matches_its_expected_output() {
    let dir = golden_dir();
    let universe = load_universe(&dir);

    let mut specs: Vec<_> = fs::read_dir(dir.join("specs"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    specs.sort();

    let mut checked = 0;
    for spec_path in specs {
        let stem = spec_path.file_stem().unwrap().to_str().unwrap();
        let spec_json = fs::read_to_string(&spec_path).unwrap();
        let spec: EvolveSpec = serde_json::from_str(&spec_json)
            .unwrap_or_else(|e| panic!("parse {}: {e}", spec_path.display()));
        let report = evolve(&universe, &spec).unwrap();
        let got = serde_json::to_string(&report).unwrap();

        let expected_path = dir.join("expected").join(format!("{stem}.json"));
        if expected_path.exists() {
            let expected = fs::read_to_string(&expected_path).unwrap();
            assert_eq!(
                got,
                expected.trim_end(),
                "golden mismatch for {stem}: the core output no longer matches the \
                 committed fixture (re-bless if the change is intended)"
            );
        } else {
            fs::write(&expected_path, format!("{got}\n")).unwrap();
        }
        checked += 1;
    }
    assert_eq!(checked, 5, "expected 5 golden specs, checked {checked}");
}

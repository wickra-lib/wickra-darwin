//! The search space is the registry, not a hand-kept list.
//!
//! `indicator_kind` was a four-name allowlist — `sma`, `ema`, `rsi`, `atr` —
//! mapping onto the `wickra-core` type names, while the README described
//! searching the whole indicator space. Every other name was rejected, so the
//! space the engine could actually reach was four indicators wide, against the
//! 497 names the registry resolves.
//!
//! These tests pin the two properties that matter: any name the engine can
//! execute can also be searched, and the arity comes from the registry rather
//! than from an assumption that every indicator takes one period.

use darwin_core::search_space::indicator_kind;

#[test]
fn the_four_the_allowlist_carried_still_resolve() {
    for (name, kind) in [
        ("sma", "Sma"),
        ("ema", "Ema"),
        ("rsi", "Rsi"),
        ("atr", "Atr"),
    ] {
        assert_eq!(
            indicator_kind(name),
            Some((kind.to_string(), 1)),
            "{name} must still resolve to a one-parameter {kind}"
        );
    }
}

#[test]
fn the_names_the_allowlist_rejected_resolve_too() {
    for (name, kind) in [("wma", "Wma"), ("dema", "Dema"), ("roc", "Roc")] {
        assert_eq!(indicator_kind(name), Some((kind.to_string(), 1)), "{name}");
    }
}

#[test]
fn the_arity_comes_from_the_registry_not_from_an_assumption() {
    // The allowlist declared every indicator as taking one parameter, which is
    // true of the four it carried and false in general.
    assert_eq!(indicator_kind("macd"), Some(("Macd".to_string(), 3)));
    assert_eq!(
        indicator_kind("BollingerBands"),
        Some(("BollingerBands".to_string(), 2))
    );
}

#[test]
fn the_registry_spelling_is_accepted_as_well_as_the_lower_case_one() {
    assert_eq!(indicator_kind("Macd"), indicator_kind("macd"));
    assert_eq!(indicator_kind("Rsi"), indicator_kind("rsi"));
}

#[test]
fn an_unknown_name_is_still_rejected() {
    assert_eq!(indicator_kind("notanindicator"), None);
    assert_eq!(indicator_kind(""), None);
    // A multi-word name in all lower case is not the registry's spelling, and
    // capitalising the first letter alone does not reach it. That is a name the
    // registry does not know, reported as such rather than guessed at.
    assert_eq!(indicator_kind("bollingerbands"), None);
}

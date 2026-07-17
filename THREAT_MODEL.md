# Threat Model

## Assets

Wickra Darwin holds no secrets, keys or funds. It reads strategy specs and candle
data and emits search results. The asset is availability: an untrusted spec must
not exhaust host resources.

## Actors

- **Operator** — runs a search on trusted or semi-trusted specs.
- **Spec author** — supplies the `StrategySpec` genome and search configuration,
  which may be untrusted.

## Threats & mitigations

- **Resource exhaustion via search size.** Population × generations bounds total
  work; both are explicit, validated configuration with documented limits. A
  malformed or oversized configuration is rejected, not silently expanded.
- **Malformed spec JSON.** Specs round-trip through `wickra_backtest::StrategySpec`
  and are rejected on parse/validation failure; errors are reported in-band over
  the binding boundary, never as a panic.
- **Supply chain.** Dependencies are pinned; `cargo-deny`, OSV and Scorecard run
  in CI. The one git dependency (`wickra-backtest`) carries a version pin.

## Out of scope

Network access, credential handling and order execution — Darwin performs none of
these.

# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Repository scaffold: governance, supply-chain configuration (`deny.toml`,
  `lychee.toml`, `osv-scanner.toml`, `repo-metadata.toml`), the Rust workspace
  (`darwin-core`, `darwin-cli`, `darwin-bench`) with the language-binding crates,
  and the `wickra-backtest` git dependency (the O(1) engine DARWIN evolves
  strategies against).
- `darwin-core`: the evolutionary search — `EvolveSpec`, `SearchSpace` /
  `RuleGrammar` / `IndicatorGene`, the `StrategySpec` genome with seeded
  `SplitMix64` crossover and mutation, `Fitness` (Sharpe / PnL / Calmar), and the
  `evolve` loop returning a deterministic `EvolveReport` (hall of fame + history).
- `wickra-darwin` CLI over the core (`--spec`, `--data`, `--stdin`, `--format`,
  `--seed`, `--top`).
- Ten language bindings (Rust, Python, Node.js, WASM natively; C, C++, C#, Go,
  Java, R over a C ABI hub), each forwarding `command_json` verbatim for a
  byte-identical search.
- Golden corpus (fixed-seed specs + blessed reports) and the test suite
  (conformance, golden replay, determinism equivalence, proptest invariants),
  fuzz targets and the criterion benchmark crate.
- The full CI/CD matrix (fmt, clippy, tests on 3 OS × 2 feature sets, MSRV,
  coverage, cargo-deny, the ten-language jobs, CodeQL, Scorecard, zizmor, link
  and metadata checks) plus a USER-GO-gated release pipeline.
- Documentation: `README`, per-binding READMEs, and `docs/` (architecture,
  evolution, genome, fitness, determinism, cookbook); `BENCHMARKS.md` with
  measured backtests-per-second figures.

[Unreleased]: https://github.com/wickra-lib/wickra-darwin/commits/main

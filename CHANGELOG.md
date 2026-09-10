# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- **The search space was four indicators wide while the README claimed 514.**
  `indicator_kind` was an allowlist of `sma`, `ema`, `rsi` and `atr`, and every
  other name was rejected. It also declared each of them as taking one
  parameter, which is true of those four and false in general — `Macd` takes
  three, `BollingerBands` two — so a multi-parameter indicator was unreachable
  twice over.

- **The throughput claim contradicted this repository's own benchmark.** The
  README, the banner alt text and `ARCHITECTURE.md` all said "millions of
  backtests per second"; `BENCHMARKS.md` reports **~110 K–285 K**. The claim is
  now the measured range, and `ARCHITECTURE.md` cites the file rather than
  restating a number that can drift from it.

- **A sampler fallback substituted `Sma` for an unresolvable name.** Unreachable,
  because `validate` resolves every name first — and the worst possible answer
  if it ever were reached: searching a different indicator than the spec asked
  for, silently.

- **CONTRIBUTING.md described a repository that is not this one** — feature
  kinds, label kinds, `docs/FEATURES.md`, `docs/LABELS.md`, all of it
  wickra-feature-store's. This is what has been failing the link check on
  `main`.

- **`CMAKE_CXX_STANDARD` asked for C++14** while the C++ hull requires C++17.
  Nothing compiled it, so nothing found out.

### Added

- **Registry-resolved search space.** Any name the engine can execute can be
  searched, with the arity the registry reports, so the space DARWIN samples
  from and the space the engine can run are the same set by construction rather
  than by two lists happening to agree.

- **Batch-search tests in every binding.** Python, Node, Go, Java, C#, R, WASM
  and C each check that the same seed reproduces the same search through that
  boundary, that a three-parameter indicator is searchable at all, and that an
  unknown name is refused.

- **A golden test for the C binding**, which had none: all five committed specs,
  byte-identical, plus `golden/data.json` so any binding can load the corpus
  without a CSV parser of its own.

- The blueprint scaffold: `LICENSES/`, `docs/README.md`, the five long-form
  issue templates, the CodeQL config, the actionlint and CodSpeed workflows, the
  five check scripts, a C++ hull, licence copies in every published crate and
  npm package, and a WASM example.

- CI gains `osv`, `links`, `binding-surface`, `semver`, `fuzz-smoke`,
  `examples` and `python-wheel-container-smoke`; the release pipeline gains the
  `gate` and `guard` jobs, provenance over the nupkg, jar and C ABI archives, a
  Maven artifact on the release page, and a Go mirror that builds before it
  publishes.

### Changed

- **The family pins move to the published releases.** `wickra-backtest` comes
  from crates.io at 0.1.4 rather than a git rev, and `wickra-core` rises from
  0.9 to 1.0, so the tree carries one set of indicator types rather than two
  that share none.

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

<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Darwin — evolutionary strategy search at millions of backtests per second" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-darwin)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![OpenSSF Scorecard](https://img.shields.io/badge/OpenSSF-Scorecard-3b82f6)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-darwin)
[![Deterministic across 10 languages](https://img.shields.io/badge/deterministic%20across-10%20languages-3b82f6)](#use-in-any-language)
[![Docs](https://img.shields.io/badge/docs-wickra.org-3b82f6)](https://wickra.org)

---

# Wickra Darwin

**Evolutionary strategy search at millions of backtests per second — mutates and
crosses JSON strategy specs to brute-force alpha across the 514-indicator space.**

> **Part of the [Wickra ecosystem](https://github.com/wickra-lib):** Darwin evolves
> the same [`StrategySpec`](https://github.com/wickra-lib/wickra-backtest) that
> `wickra-backtest` runs — mutation and crossover over that JSON genome, scored by
> the O(1)-per-tick engine, so the search evaluates candidates orders of magnitude
> faster than pandas-based tooling.

Wickra Darwin is one data-driven core, `darwin-core`: a population of
`StrategySpec` genomes is evolved with genetic operators (mutation + crossover
across the indicator search space) and each candidate is scored by
`wickra-backtest`. Because the engine is O(1) per tick, the loop sustains a very
high backtests-per-second rate — "AlphaZero for trading strategies." The core is
exposed as a **JSON-over-C-ABI data API** (`command_json`) in **Rust, Python,
Node.js, WASM, C, C++, C#, Go, Java and R**, plus a reference CLI.

## Status

Early development (0.1.0, unreleased). This scaffold pins the repository,
governance and supply-chain configuration ahead of the evolutionary core, the
CLI, the ten language bindings and the golden harness.

## Use in any language

The same handle + `command_json` + `version` surface ships for every supported
language; each binding forwards the command string verbatim, so a search with a
fixed seed yields identical results in all of them.

## Building from source

```bash
cargo build
cargo test
```

## Requirements

- Rust 1.86+ (MSRV). Darwin depends on the `wickra-backtest` engine (git).

## Security

See [SECURITY.md](SECURITY.md) and [THREAT_MODEL.md](THREAT_MODEL.md). Search
runs on untrusted spec JSON — resource limits (population × generations) bound
the work.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Dual-licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option. Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in this work, as defined in the Apache-2.0
license, shall be dual-licensed as above, without any additional terms or
conditions.

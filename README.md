<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Darwin — evolutionary strategy search at millions of backtests per second" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-darwin)
[![CI](https://github.com/wickra-lib/wickra-darwin/actions/workflows/ci.yml/badge.svg)](https://github.com/wickra-lib/wickra-darwin/actions/workflows/ci.yml)
[![CodeQL](https://github.com/wickra-lib/wickra-darwin/actions/workflows/codeql.yml/badge.svg)](https://github.com/wickra-lib/wickra-darwin/actions/workflows/codeql.yml)
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

Early development (0.1.0, unreleased). The evolutionary core, the reference CLI,
the ten-language binding surface, the golden corpus and the full CI matrix are in
place; the first published release is still pending.

## How it works

An `EvolveSpec` names a `seed`, a `population` and a `generations` count,
mutation and crossover rates, a `fitness` objective (`sharpe` / `pnl` /
`calmar`) and a `SearchSpace` — the indicators the genome may draw on and the
`RuleGrammar` that wires them into entry/exit conditions. The core:

1. seeds a `SplitMix64` PRNG from `seed` and samples an initial population of
   `StrategySpec` genomes from the search space;
2. scores every genome by running it through the `wickra-backtest` engine and
   reducing the equity curve to the chosen fitness value;
3. ranks the population, keeps the elite, and breeds the next generation with
   crossover and mutation drawn from the same PRNG;
4. repeats for `generations` rounds and returns an `EvolveReport` — the hall of
   fame (best genomes, ranked by fitness then `spec_hash`) plus per-generation
   statistics.

## Determinism

The search is the golden moat: the `SplitMix64` PRNG lives in the Rust core, the
population is held in ordered collections, genomes are hashed from a canonical
serialisation, and NaN/inf fitness collapses to `NEG_INFINITY` before ranking.
The same `EvolveSpec` + candle data yields a **byte-identical `EvolveReport`** on
every run, and — because each binding forwards the command string verbatim — in
every language. The single-threaded and the `rayon`-parallel fitness paths are
byte-identical by construction (each genome is scored independently).

## Quickstart

```bash
# Evolve over a folder of candle CSVs and print the ranked hall of fame.
wickra-darwin --spec golden/specs/evolve_small.json --data golden/data

# Emit the full EvolveReport as JSON (hall of fame + per-generation history).
wickra-darwin --spec golden/specs/evolve_small.json --data golden/data --format json

# Override the seed without editing the spec — a different search, same machinery.
wickra-darwin --spec golden/specs/evolve_small.json --data golden/data --seed 42
```

## Use in any language

The same handle + `command_json` + `version` surface ships for Rust, Python,
Node.js, WASM, and — over a C ABI hub — C, C++, C#, Go, Java and R. Each binding
passes the command string through verbatim, so the `EvolveReport` they return is
identical.

```python
import json
from wickra_darwin import Darwin

spec = open("golden/specs/evolve_small.json").read()
data = json.load(open("universe.json"))  # {"SYM": [{"time":..,"open":..,...}, ...]}
report = json.loads(Darwin(spec).command(json.dumps({"cmd": "evolve", "data": data})))
print(report["best"][0]["spec_hash"] if report["best"] else "no survivors")
```

See [`examples/`](examples/) for the same program in all ten languages.

## Documentation

- [ARCHITECTURE.md](docs/ARCHITECTURE.md) — the crates and the evolution pipeline.
- [EVOLUTION.md](docs/EVOLUTION.md) — the loop: sampling, selection, elitism.
- [GENOME.md](docs/GENOME.md) — the `StrategySpec` genome, mutation and crossover.
- [FITNESS.md](docs/FITNESS.md) — the Sharpe / PnL / Calmar objectives.
- [DETERMINISM.md](docs/DETERMINISM.md) — why the report is reproducible everywhere.
- [Cookbook.md](docs/Cookbook.md) — practical recipes.

## Project layout

```
crates/darwin-core    the library: spec, search space, genome, evolve loop, fitness
crates/darwin-cli     the wickra-darwin CLI
crates/darwin-bench   criterion micro-benchmarks (backtests/second)
bindings/*            ten language surfaces (c, python, node, wasm, csharp, go, java, r)
golden/               specs + blessed reports (the cross-language corpus)
examples/             one runnable example per language
docs/                 architecture, evolution, genome, fitness, determinism, cookbook
```

## Building from source

```bash
cargo build
cargo test
```

## Benchmarks

The headline figure is **backtests per second** — the rate at which the loop
scores candidate specs through the `wickra-backtest` engine. See
[BENCHMARKS.md](BENCHMARKS.md); reproduce with `cargo bench -p darwin-bench`.

## Requirements

- Rust 1.86+ (MSRV). Darwin depends on the `wickra-backtest` engine (git).

## Security

See [SECURITY.md](SECURITY.md) and [THREAT_MODEL.md](THREAT_MODEL.md). Search
runs on untrusted spec JSON — resource limits (population × generations) bound
the work.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Disclaimer

Wickra Darwin is a research tool, provided "as is" without warranty of any kind.
Evolutionary search optimises a fitness objective over historical data — a strong
in-sample fitness is not evidence of out-of-sample performance, and overfitting is
the default outcome, not the exception. Nothing here is financial advice; any
strategy you deploy is your responsibility, and trading carries risk of loss.

## License

Dual-licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option. Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in this work, as defined in the Apache-2.0
license, shall be dual-licensed as above, without any additional terms or
conditions.

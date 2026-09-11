<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Darwin — evolutionary strategy search at hundreds of thousands of backtests per second" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-darwin)
[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/ci.svg)](https://github.com/wickra-lib/wickra-darwin/actions/workflows/ci.yml)
[![CodeQL](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/codeql.svg)](https://github.com/wickra-lib/wickra-darwin/actions/workflows/codeql.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-darwin)
[![GitHub release](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/release.svg)](https://github.com/wickra-lib/wickra-darwin/releases/latest)
[![crates.io](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/crates.svg)](https://crates.io/crates/wickra-darwin)
[![PyPI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/pypi.svg)](https://pypi.org/project/wickra-darwin/)
[![npm](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/npm.svg)](https://www.npmjs.com/package/wickra-darwin)
[![NuGet](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/nuget.svg)](https://www.nuget.org/packages/Wickra.Darwin)
[![Maven Central](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/maven.svg)](https://central.sonatype.com/artifact/org.wickra/wickra-darwin)
[![Go module](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/go.svg)](https://pkg.go.dev/github.com/wickra-lib/wickra-darwin-go)
[![R-universe](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/r-universe.svg)](https://wickra-lib.r-universe.dev)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/license.svg)](#license)
[![OpenSSF Scorecard](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/scorecard.svg)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-darwin)
[![OpenSSF Best Practices](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/best-practices.svg)](https://www.bestpractices.dev)
[![Build provenance](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/provenance.svg)](https://github.com/wickra-lib/wickra-darwin/attestations)
[![Docs](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/docs.svg)](https://wickra.org)
[![Verified across 10 languages](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/verified.svg)](golden/)

---

# Wickra Darwin

**Evolutionary strategy search at hundreds of thousands of backtests per second
— mutates and crosses JSON strategy specs to brute-force alpha across the whole
indicator registry.**

> **Part of the [Wickra ecosystem](https://github.com/wickra-lib):** the same data-driven core and ten-language binding surface also power [wickra-exchange](https://github.com/wickra-lib/wickra-exchange), [wickra-backtest](https://github.com/wickra-lib/wickra-backtest), [wickra-terminal](https://github.com/wickra-lib/wickra-terminal) and 20 more — see [the full list](https://github.com/wickra-lib).
> the same [`StrategySpec`](https://github.com/wickra-lib/wickra-backtest) that
> `wickra-backtest` runs — mutation and crossover over that JSON genome, scored by
> the O(1)-per-tick engine, so the search evaluates candidates orders of magnitude
> faster than pandas-based tooling.

Wickra Darwin is one data-driven core, `wickra-darwin-core`: a population of
`StrategySpec` genomes is evolved with genetic operators (mutation + crossover
across the indicator search space) and each candidate is scored by
`wickra-backtest`. Because the engine is O(1) per tick, the loop sustains
~122 K-448 K backtests per second (see [BENCHMARKS.md](BENCHMARKS.md)) —
"AlphaZero for trading strategies." The core is
exposed as a **JSON-over-C-ABI data API** (`command_json`) in **Rust, Python,
Node.js, WASM, C, C++, C#, Go, Java and R**, plus a reference CLI.

```rust
use wickra_darwin_core::{evolve, EvolveSpec};

// A bounded search: three indicator genes over the registry, scored by Sharpe.
let spec: EvolveSpec = serde_json::from_str(r#"{
    "search_space": {
        "indicators": [
            {"name": "rsi",  "param_ranges": [{"min": 5, "max": 30, "step": 1}]},
            {"name": "sma",  "param_ranges": [{"min": 5, "max": 50, "step": 5}]},
            {"name": "macd", "param_ranges": [{"min": 8,  "max": 16, "step": 2},
                                              {"min": 20, "max": 30, "step": 2},
                                              {"min": 5,  "max": 12, "step": 1}]}
        ],
        "rules": "single_threshold"
    },
    "population": 64, "generations": 20, "fitness": "sharpe", "seed": 42
}"#)?;

let report = evolve(&data, &spec)?;   // same seed, same winner, every time
```

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

## Building everything from source

```bash
cargo build --workspace --all-features                 # Rust core + CLI + C ABI
(cd bindings/python && maturin develop --release)      # Python
(cd bindings/node   && npm ci && npm run build)        # Node
(cd bindings/wasm   && wasm-pack build --target web)   # WASM
(cd bindings/csharp && dotnet build)                   # C#
(cd bindings/go     && go build ./...)                 # Go
(cd bindings/java   && mvn -q package)                 # Java
R CMD INSTALL bindings/r                               # R
```

The C-ABI consumers (C/C++, C#, Go, Java, R) need the C ABI library first —
`cargo build --release -p wickra-darwin-c` — on the loader path.

## Testing

```bash
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all --check
```

Every binding replays the same golden searches from [`golden/`](golden/) and must
produce the identical bytes; that corpus is the cross-language contract, not a
per-language approximation. `python scripts/check_binding_surface.py` asserts the
ten surfaces stayed in step.

## Benchmarks

The headline figure is **backtests per second** — the rate at which the loop
scores candidate specs through the `wickra-backtest` engine. See
[BENCHMARKS.md](BENCHMARKS.md); reproduce with `cargo bench -p darwin-bench`.

## Requirements

- **Rust 1.86+** — the workspace MSRV; the Node binding needs **Rust 1.88**.
- **Python 3.9+** — the Python binding.
- **Node 22+** — the Node binding.
- **Go 1.23+** — the Go binding.
- **Java 22+** — the Java binding.
- **R 2.10+** — the R package.
- **.NET 8+** — the C# binding.
- A **C11 / C++17** compiler with CMake for the C and C++ examples.

Darwin depends on `wickra-backtest` for the engine it scores candidates with and
for the name -> indicator registry the search space is drawn from, and on
`wickra-core` for the indicator types. Both come from crates.io.

## Security

See [SECURITY.md](SECURITY.md) and [THREAT_MODEL.md](THREAT_MODEL.md). Search
runs on untrusted spec JSON — resource limits (population × generations) bound
the work.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Ecosystem

Part of the [Wickra](https://github.com/wickra-lib/wickra) family — each one a
data-driven core with a CLI and the same ten-language binding surface:

- [**wickra**](https://github.com/wickra-lib/wickra) — main library (Rust core + Python / Node.js / WASM bindings + a C ABI for C / C++ / C# / Go / Java / R)
- [**wickra-playground**](https://github.com/wickra-lib/wickra-playground) — a polyglot strategy playground: one StrategySpec live side by side in Python, Rust, JS and Go, entirely in the browser
- [**wickra-exchange**](https://github.com/wickra-lib/wickra-exchange) — unified market-data + execution across ten crypto exchanges
- [**wickra-backtest**](https://github.com/wickra-lib/wickra-backtest) — event-driven backtester over the Wickra core
- [**wickra-terminal**](https://github.com/wickra-lib/wickra-terminal) — the trading terminal: a TUI and a browser renderer over the stack
- [**wickra-xray**](https://github.com/wickra-lib/wickra-xray) — market-microstructure explorer: footprint, order-book heatmap, liquidation map, funding/OI divergence
- [**wickra-radar**](https://github.com/wickra-lib/wickra-radar) — perp-universe alert radar: OI delta, funding flip, book imbalance, liquidation clusters, OI/price divergence
- [**wickra-copilot**](https://github.com/wickra-lib/wickra-copilot) — local market copilot grounded in real order-book, liquidation and funding microstructure
- [**wickra-shazam**](https://github.com/wickra-lib/wickra-shazam) — match an asset's current microstructure fingerprint against its entire history
- [**wickra-benchmark**](https://github.com/wickra-lib/wickra-benchmark) — reproducible, golden-verified benchmark suite — recompute any (strategy, dataset, report) in ten languages and confirm it byte-for-byte
- [**wickra-strategy-ci**](https://github.com/wickra-lib/wickra-strategy-ci) — Jest for trading strategies: golden-pin the report, catch regressions in CI, property-test against fuzzed data
- [**wickra-verify**](https://github.com/wickra-lib/wickra-verify) — confirm or refute a claimed backtest report against its strategy and data, in ten languages
- [**wickra-proof**](https://github.com/wickra-lib/wickra-proof) — Proof-of-Backtest: deterministic (spec, data) → report + blake3 hash, recomputable byte-for-byte in ten languages
- [**wickra-zk**](https://github.com/wickra-lib/wickra-zk) — prove a backtest zero-knowledge — on-chain-verifiable performance without revealing the data or the strategy
- [**wickra-impact**](https://github.com/wickra-lib/wickra-impact) — the backtester that knows you would have moved the market: agent-based fills on the real historical L2 order book
- [**wickra-gym**](https://github.com/wickra-lib/wickra-gym) — a Gymnasium-compatible, microstructure-aware backtest environment with O(1) steps for deterministic RL rollouts
- [**wickra-feature-store**](https://github.com/wickra-lib/wickra-feature-store) — OHLCV and microstructure streams into ML-ready feature matrices over 497 O(1) streaming indicators
- [**wickra-genome**](https://github.com/wickra-lib/wickra-genome) — a vector database of the whole market: every asset a live vector over the indicator registry, for similarity search, clustering and anomaly detection
- [**wickra-timemachine**](https://github.com/wickra-lib/wickra-timemachine) — scrub the whole market like a video — every symbol, full order book, rewound to any moment via deterministic re-fold
- [**wickra-synth**](https://github.com/wickra-lib/wickra-synth) — deterministic synthetic market microstructure: OHLCV, order book, trades and funding from a single seed
- [**wickra-compile**](https://github.com/wickra-lib/wickra-compile) — compile a strategy spec into a standalone deployable: a WASM module, a self-contained binary, or a `no_std` artifact
- [**wickra-embed**](https://github.com/wickra-lib/wickra-embed) — allocation-free, `no_std` streaming indicators for bare-metal and HFT, byte-for-byte identical to the core
- [**wickra-pico**](https://github.com/wickra-lib/wickra-pico) — the O(1) indicator core running bare-metal on a $5 Raspberry Pi Pico — the LED blinks on the EMA cross

The screener's own guides live in [`docs/`](docs/) beside the code; its site,
with the in-browser demo and the benchmark figures, is at
[screener.wickra.org](https://screener.wickra.org). The indicator library's
reference is at [docs.wickra.org](https://docs.wickra.org) and the org landing
page at [wickra.org](https://wickra.org).

## License

Dual-licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option. Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in this work, as defined in the Apache-2.0
license, shall be dual-licensed as above, without any additional terms or
conditions.

---

<p align="center">
  <a href="https://github.com/wickra-lib/wickra-darwin">
    <img alt="GitHub stars" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/stars.svg">
  </a>
  <a href="https://github.com/wickra-lib/wickra-darwin/network/members">
    <img alt="GitHub forks" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/forks.svg">
  </a>
  <a href="https://github.com/wickra-lib/wickra-darwin/issues">
    <img alt="GitHub issues" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/issues.svg">
  </a>
</p>

<p align="center">
  Built on <a href="https://github.com/wickra-lib/wickra">Wickra</a>. If it saved you time, the cheapest way to say thanks is to ⭐ the repo.
</p>

<p align="center">
  <img alt="wickra-darwin star history" width="640"
       src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/star-history.svg">
</p>

## Disclaimer

Wickra Darwin is a research tool, provided "as is" without warranty of any kind.
Evolutionary search optimises a fitness objective over historical data — a strong
in-sample fitness is not evidence of out-of-sample performance, and overfitting is
the default outcome, not the exception. Nothing here is financial advice; any
strategy you deploy is your responsibility, and trading carries risk of loss.

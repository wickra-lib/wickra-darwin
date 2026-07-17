# Architecture

Wickra Darwin evolves a population of `StrategySpec` genomes and scores each with
the `wickra-backtest` engine. Everything reproducible lives in one Rust crate,
`darwin-core`; the CLI and the ten language bindings are thin shells around it.

## Workspace

| Crate | Role |
|-------|------|
| `darwin-core` | The library: population model, genetic operators (mutation + crossover over `StrategySpec`s), the fitness loop and the JSON command boundary. |
| `darwin-cli` (`wickra-darwin`) | The reference CLI. |
| `darwin-bench` | Criterion micro-benchmarks (backtests/second). |
| `bindings/*` | The language surfaces over the C ABI hub. |

## The evolution loop

1. **Seed** a population of `StrategySpec` genomes.
2. **Evaluate** each candidate with `wickra_backtest::run` — O(1) per tick, so
   the loop sustains millions of backtests per second.
3. **Select** the fittest by a metric from `BacktestReport` (e.g. Sharpe).
4. **Mutate + cross** the survivors to form the next generation.
5. Repeat for a bounded number of generations.

The RNG lives only in the Rust core and is portable-deterministic, so a fixed
seed yields the same search on every platform and through every binding — the
basis for the cross-language golden corpus.

## The command boundary

Bindings never re-implement any of this. `command_json(&str) -> String` takes a
command envelope and returns a response JSON string that each binding forwards
verbatim.

## See also

- [THREAT_MODEL.md](THREAT_MODEL.md) · [ROADMAP.md](ROADMAP.md) · [BENCHMARKS.md](BENCHMARKS.md)

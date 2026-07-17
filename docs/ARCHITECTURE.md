# Architecture

Wickra Darwin is a single data-driven core, `darwin-core`, wrapped by a reference
CLI and ten language bindings. Everything the search does is reachable through one
seam: a `Darwin` handle and a `command_json` string.

## Crates

```
crates/darwin-core    the library — spec, search space, genome, evolve loop, fitness
crates/darwin-cli     the wickra-darwin CLI (clap front end over the core)
crates/darwin-bench   criterion micro-benchmarks (backtests/second)
bindings/{c,python,node,wasm,csharp,go,java,r}   the language surfaces
```

`darwin-core` depends on [`wickra-backtest`](https://github.com/wickra-lib/wickra-backtest)
for the engine that scores each candidate, and re-exports
`wickra_backtest::StrategySpec` — the genome the search evolves.

## Modules in `darwin-core`

| Module         | Responsibility |
|----------------|----------------|
| `spec`         | `EvolveSpec` — the search request (seed, population, generations, rates, fitness, search space, elitism, top). |
| `search_space` | `SearchSpace`, `RuleGrammar`, `IndicatorGene` — what the genome may draw on; `sample_spec` and `mutate`. |
| `genome`       | the `StrategySpec` genome: `crossover`, `spec_hash`, canonicalisation, `to_strategy_spec`. |
| `fitness`      | `Fitness` — Sharpe / PnL / Calmar; reduces a backtest result to one score. |
| `evolve`       | the loop — `evolve(data, spec) -> EvolveReport`, `GenStats`, `RankedStrategy`. |
| `rng`          | `SplitMix64` — the deterministic PRNG that drives sampling, mutation and crossover. |
| `darwin`       | the `Darwin` handle and `command_json` dispatch. |
| `config`       | limits and defaults. |
| `error`        | `Error` / `Result`. |

## The pipeline

```
EvolveSpec (JSON)
   │  validate + seed SplitMix64
   ▼
sample initial population  ──►  score each genome via wickra-backtest  ──►  fitness
   │                                                                          │
   │  ◄──────────  breed next generation (elitism + crossover + mutation)  ◄──┘
   ▼  (repeat `generations` times)
EvolveReport = hall of fame (ranked) + per-generation history
```

The core never spawns work the caller did not ask for: population × generations
bounds the number of backtests, and the fitness pass is the only place the engine
runs. The optional `parallel` feature scores genomes across a `rayon` pool; the
result is identical to the single-threaded path because each genome is scored
independently (see [DETERMINISM.md](DETERMINISM.md)).

## The `command_json` seam

Every binding calls the same core method:

```rust
let mut darwin = Darwin::new(spec_json)?;      // parse + validate EvolveSpec
let report = darwin.command_json(command)?;    // {"cmd":"evolve","data":{...}} -> EvolveReport JSON
```

Bindings never re-implement the loop, the PRNG or the fitness maths — they marshal
strings in and out. That is what makes the search byte-identical across languages.

## See also

- [EVOLUTION.md](EVOLUTION.md) — the loop in detail.
- [GENOME.md](GENOME.md) — the genome and its operators.
- [FITNESS.md](FITNESS.md) — the objectives.
- [DETERMINISM.md](DETERMINISM.md) — why the report reproduces everywhere.

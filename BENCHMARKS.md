# Benchmarks

The headline figure is **backtests per second** — the rate at which the evolution
loop scores candidate specs through the `wickra-backtest` engine. One "backtest"
is one individual evaluated over one symbol, so the total work of a search is
`population × (generations + 1) × symbols` — the loop scores the initial
population before it breeds, so one more generation is scored than the spec's
`generations` count names.

Reproduce with:

```bash
cargo bench -p darwin-bench
```

## Measured throughput

Criterion `evolve` group, single machine (Windows, release build, default
`parallel` feature), sweeping population × generations × symbol count over the
in-bench synthetic universe. Throughput is total backtests over wall-clock:

| population | generations | symbols | throughput (backtests/s) |
|-----------:|------------:|--------:|--------------------------:|
| 16         | 5           | 3       | ~122 K |
| 16         | 5           | 10      | ~261 K |
| 16         | 20          | 3       | ~127 K |
| 16         | 20          | 10      | ~264 K |
| 64         | 5           | 3       | ~188 K |
| 64         | 5           | 10      | ~393 K |
| 64         | 20          | 3       | ~191 K |
| 64         | 20          | 10      | ~394 K |
| 256        | 5           | 3       | ~214 K |
| 256        | 5           | 10      | ~435 K |
| 256        | 20          | 3       | ~221 K |
| 256        | 20          | 10      | ~448 K |

Throughput rises with the symbol count because per-search fixed overhead
(sampling, ranking, breeding) amortises over more backtests, and with population
and generations for the same reason. On these short synthetic series the loop
sustains **~122 K–448 K backtests/second**; longer candle histories raise the
per-backtest cost but the engine stays O(1) per tick, so the rate is governed by
total bars processed, not by strategy or indicator complexity.

## Method notes

- These figures were re-measured after `to_strategy_spec` was fixed. The earlier
  ones were taken while every candidate failed to deserialise into a
  `StrategySpec`, so the loop never reached the backtest engine at all: a table
  headed "backtests per second" was timing a search that ran none.
- Numbers are indicative of relative scaling, not a hardware datasheet — absolute
  values depend on CPU, candle-series length and the indicators the genome uses.
- The `parallel` (rayon) and single-threaded paths produce **byte-identical**
  reports; parallelism affects wall-clock only (see
  [docs/DETERMINISM.md](docs/DETERMINISM.md)).
- Re-bless these figures when the `wickra-backtest` engine is bumped, since engine
  changes move the per-tick cost.

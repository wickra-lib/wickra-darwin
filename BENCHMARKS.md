# Benchmarks

The headline figure is **backtests per second** — the rate at which the evolution
loop scores candidate specs through the `wickra-backtest` engine. One "backtest"
is one individual evaluated over one symbol, so the total work of a search is
`population × generations × symbols`.

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
| 16         | 5           | 3       | ~112 K |
| 16         | 5           | 10      | ~210 K |
| 16         | 20          | 3       | ~129 K |
| 16         | 20          | 10      | ~233 K |
| 64         | 5           | 3       | ~124 K |
| 64         | 5           | 10      | ~233 K |
| 64         | 20          | 3       | ~138 K |
| 64         | 20          | 10      | ~263 K |
| 256        | 5           | 3       | ~132 K |
| 256        | 5           | 10      | ~241 K |
| 256        | 20          | 3       | ~149 K |
| 256        | 20          | 10      | ~285 K |

Throughput rises with the symbol count because per-search fixed overhead
(sampling, ranking, breeding) amortises over more backtests, and with population
and generations for the same reason. On these short synthetic series the loop
sustains **~110 K–285 K backtests/second**; longer candle histories raise the
per-backtest cost but the engine stays O(1) per tick, so the rate is governed by
total bars processed, not by strategy or indicator complexity.

## Method notes

- Numbers are indicative of relative scaling, not a hardware datasheet — absolute
  values depend on CPU, candle-series length and the indicators the genome uses.
- The `parallel` (rayon) and single-threaded paths produce **byte-identical**
  reports; parallelism affects wall-clock only (see
  [docs/DETERMINISM.md](docs/DETERMINISM.md)).
- Re-bless these figures when the `wickra-backtest` engine is bumped, since engine
  changes move the per-tick cost.

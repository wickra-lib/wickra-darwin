# Cookbook

Practical recipes for `wickra-darwin`. All commands run from the repository root
against the golden corpus; swap in your own spec and data folder.

## Run a search and read the hall of fame

```bash
wickra-darwin --spec golden/specs/evolve_small.json --data golden/data
```

The text output prints a `history:` line (one row per generation) and a ranked
table `rank | fitness | gen | spec_hash | spec`. An empty table means no sampled
genome traded on this data — see
[EVOLUTION.md](EVOLUTION.md#degenerate-case-an-empty-hall-of-fame).

## Get machine-readable output

```bash
wickra-darwin --spec golden/specs/evolve_small.json --data golden/data --format json \
  | jq '.best[0]'          # the top-ranked strategy, or null
```

## Change the objective without touching the spec file

The objective lives in the spec, so the corpus ships one per fitness:

```bash
wickra-darwin --spec golden/specs/evolve_pnl.json      --data golden/data   # raw PnL
wickra-darwin --spec golden/specs/evolve_calmar.json   --data golden/data   # return / max drawdown
```

See [FITNESS.md](FITNESS.md) for how to choose.

## Reproduce, then explore

A fixed seed fixes the whole search — re-running is byte-identical. To explore a
different region of the space, override the seed:

```bash
wickra-darwin --spec golden/specs/evolve_small.json --data golden/data --seed 1
wickra-darwin --spec golden/specs/evolve_small.json --data golden/data --seed 2
```

Each seed is its own reproducible run.

## Widen the search space

Edit the spec's `search_space`. To let the genome pick from more indicators and
AND several conditions:

```jsonc
"search_space": {
  "indicators": [
    { "name": "rsi", "param_ranges": [ { "min": 2,  "max": 30, "step": 1 } ] },
    { "name": "sma", "param_ranges": [ { "min": 5,  "max": 50, "step": 5 } ] },
    { "name": "ema", "param_ranges": [ { "min": 5,  "max": 50, "step": 5 } ] }
  ],
  "rules": "conjunction_all",
  "max_conditions": 3
}
```

The allowlist is `sma`, `ema`, `rsi`, `atr` (each takes one parameter). An unknown
name, a wrong arity or a non-positive `step` is rejected at parse time.

## Drive it from another language

Every binding exposes the same `command_json`; the returned report is identical.

```javascript
// Node.js
const { Darwin } = require("wickra-darwin");
const spec = require("fs").readFileSync("golden/specs/evolve_small.json", "utf8");
const data = { SYM: [/* candles: {time, open, high, low, close, volume} */] };
const report = JSON.parse(new Darwin(spec).command(JSON.stringify({ cmd: "evolve", data })));
console.log(report.history.length, "generations");
```

See [`examples/`](../examples/) for the same program in all ten languages.

## Scale down for a smoke test, up for a real search

`population` × `generations` bounds the number of backtests. Start small
(`population: 10`, `generations: 4`) to validate a spec, then scale both up — the
loop stays deterministic and the throughput is high because each backtest is O(1)
per tick (see [BENCHMARKS.md](../BENCHMARKS.md)).

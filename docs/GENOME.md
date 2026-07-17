# The genome

Darwin does not invent a strategy representation — it evolves the same
[`StrategySpec`](https://github.com/wickra-lib/wickra-backtest) that
`wickra-backtest` already runs. A genome **is** a `StrategySpec`; the operators
below produce valid specs by construction.

## The search space

A `SearchSpace` bounds what a genome may contain:

```jsonc
{
  "indicators": [                              // the palette the genome draws from
    { "name": "rsi", "param_ranges": [ { "min": 2, "max": 30, "step": 1 } ] },
    { "name": "sma", "param_ranges": [ { "min": 5, "max": 50, "step": 5 } ] }
  ],
  "rules": "single_threshold",                 // the RuleGrammar (see below)
  "max_conditions": 1                          // how many conditions a rule may AND
}
```

- **`IndicatorGene`** = a `name` plus one `ParamRange` per parameter. The name
  must be in the allowlist (`sma`, `ema`, `rsi`, `atr` — each arity 1); the arity
  must match; every `ParamRange` needs `step > 0` and `max >= min`.
- **`ParamRange`** = `{ min, max, step }`. Sampling and mutation snap parameter
  values to this grid, so genomes stay on a discrete, reproducible lattice.
- **`SearchSpace::validate`** rejects an empty indicator list, `max_conditions < 1`,
  unknown indicators, wrong arity and non-positive steps. It does **not** reject
  duplicate indicator names — repetition is a legitimate way to weight the palette.

## The rule grammar

`RuleGrammar` decides how sampled indicators become entry/exit conditions:

| Grammar             | JSON tag            | Shape |
|---------------------|---------------------|-------|
| `SingleThreshold`   | `single_threshold`  | one indicator compared to a constant (`rsi < 30`). |
| `CrossoverPair`     | `crossover_pair`    | two indicators crossing (`sma(10)` crosses `sma(30)`). |
| `ConjunctionAll`    | `conjunction_all`   | up to `max_conditions` conditions ANDed together. |

## Operators

All operators draw from the run's single `SplitMix64` PRNG, so a fixed seed fixes
every choice.

- **`sample_spec`** — draw a fresh genome: pick indicators from the palette,
  snap parameters to their `ParamRange` grids, wire a rule per the grammar, and
  assemble a `StrategySpec` (with a fixed `sizing` of 10% fixed fraction).
- **`crossover(a, b)`** — combine two parent genomes gene-by-gene into a child.
- **`mutate(genome)`** — perturb one or more genes: nudge a parameter to an
  adjacent grid point, or swap an indicator, staying inside the search space.

## Identity and canonicalisation

- **`to_strategy_spec(genome, symbol, timeframe)`** renders the genome as the
  `StrategySpec` JSON the engine runs — entry/exit conditions built from
  `ref`/`const` operands and comparison tags (`gt`, `lt`, …), crosses as
  `{ tag: [ref, ref] }`.
- **`spec_hash`** hashes the genome from a **canonical** serialisation
  (`canonicalize` + `round8` on floats), so two genomes that differ only by
  float noise or key order hash equal. The hash is the ranking tie-break and the
  golden identity — see [DETERMINISM.md](DETERMINISM.md).

## See also

- [EVOLUTION.md](EVOLUTION.md) — where the operators are called.
- [FITNESS.md](FITNESS.md) — how a genome is scored.

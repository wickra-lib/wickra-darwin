# The evolution loop

`evolve(data, spec) -> EvolveReport` is the whole search. This document walks the
loop; the operators it calls live in [GENOME.md](GENOME.md) and the scoring in
[FITNESS.md](FITNESS.md).

## Inputs

- `data: &BTreeMap<String, Vec<Candle>>` — the universe, one candle series per
  symbol. Ordered by symbol so iteration is deterministic.
- `spec: &EvolveSpec` — the search request:

  | Field           | Meaning |
  |-----------------|---------|
  | `seed`          | seeds the `SplitMix64` PRNG — fixes the entire search. |
  | `population`    | genomes per generation. |
  | `generations`   | rounds of selection + breeding (must be ≥ 1). |
  | `mutation_rate` | probability a gene mutates when breeding. |
  | `crossover_rate`| probability two parents are crossed rather than copied. |
  | `fitness`       | the objective — `sharpe` / `pnl` / `calmar`. |
  | `search_space`  | the indicators and `RuleGrammar` the genome may use. |
  | `elitism`       | how many top genomes carry over unchanged (default 0). |
  | `top`           | how many ranked genomes the hall of fame keeps. |

`evolve` rejects an empty universe and `generations == 0` with `BadSpec`.

## The loop

1. **Seed.** `SplitMix64::new(seed)` — one PRNG threads the whole run.
2. **Sample.** Draw `population` genomes from the search space
   (`search_space::sample_spec`). Each is a `StrategySpec` built from indicators
   and a rule wired per the grammar.
3. **Score.** Run every genome through `wickra-backtest` over `data` and reduce
   the result to its `fitness` value. NaN/inf collapses to `NEG_INFINITY`.
4. **Rank.** Sort by `(fitness desc, spec_hash asc)` — the hash tie-break keeps
   ranking total and deterministic.
5. **Record.** Append a `GenStats { generation, best, mean, worst, evaluated }`
   to the history.
6. **Breed** (skipped after the final generation):
   - copy the top `elitism` genomes unchanged;
   - fill the rest by selecting parents, applying `crossover` with probability
     `crossover_rate`, then `mutate` with probability `mutation_rate` per gene —
     all draws from the same PRNG.
7. Repeat from step 3 for `generations` rounds.

## Output

`EvolveReport { best: Vec<RankedStrategy>, history: Vec<GenStats> }`:

- `best` — the hall of fame across all generations, ranked, truncated to `top`.
  Each `RankedStrategy { spec, fitness, generation, spec_hash }` is the genome as
  `StrategySpec` JSON plus where and how well it scored.
- `history` — one `GenStats` per generation, for convergence plots.

## Degenerate case: an empty hall of fame

If no sampled genome ever trades — for example on smooth synthetic data where no
threshold is crossed — every fitness is `NEG_INFINITY`, so `best` is empty and
every `GenStats` reads `{ best: 0, mean: 0, worst: 0, evaluated: 0 }`. This is a
**correct, deterministic** result, not a failure: the golden corpus uses exactly
such a universe to pin the machinery and its determinism. Use varied data (a real
series, or a synthetic path with genuine turning points) to get survivors.

## See also

- [GENOME.md](GENOME.md) · [FITNESS.md](FITNESS.md) · [DETERMINISM.md](DETERMINISM.md)

# Documentation

These pages are the guides that live beside the code, because they describe how
this repository behaves and have to change in the same commit the behaviour does.

| Page | What it answers |
|------|-----------------|
| [GENOME.md](GENOME.md) | What a genome is: genes, rules, and the `StrategySpec` it becomes |
| [EVOLUTION.md](EVOLUTION.md) | The loop: seeding, selection, mutation, crossover, elitism |
| [FITNESS.md](FITNESS.md) | Which metric of a backtest is maximised, and what each one rewards |
| [DETERMINISM.md](DETERMINISM.md) | Why the same seed yields the same winner, in every language |
| [ARCHITECTURE.md](ARCHITECTURE.md) | The internals: how a candidate is scored and a generation is formed |
| [Cookbook.md](Cookbook.md) | Worked searches |

The API reference for each language is generated from the source rather than
committed here — `cargo doc` for Rust, the `.d.ts` beside the Node binding, the
docstrings in the Python module, the C header. Keeping a second copy in this
repository would drift from the code that generates it, and a reader opening
`docs/` would have no way to tell which copy was current.

The engine each candidate is scored with documents itself at
<https://github.com/wickra-lib/wickra-backtest>, and the indicator library the
search space is drawn from at <https://docs.wickra.org>.

What stays here is what a generator cannot produce: the meaning of a field, the
reason a case is refused rather than answered, and the worked examples.

Elsewhere in the repository:

- [`../ARCHITECTURE.md`](../ARCHITECTURE.md) — the crate and binding layout
- [`../BENCHMARKS.md`](../BENCHMARKS.md) — what is measured and how
- [`../golden/README.md`](../golden/README.md) — the cross-language corpus and how to re-bless it
- [`../CONTRIBUTING.md`](../CONTRIBUTING.md) — how to build, test and propose a change
- [`../THREAT_MODEL.md`](../THREAT_MODEL.md) — what the search does and does not touch

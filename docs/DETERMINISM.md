# Determinism

The whole value of Darwin is that a search is **reproducible**: the same
`EvolveSpec` and the same candle data yield a byte-identical `EvolveReport` on
every run, on every machine, and in every language binding. This document explains
why.

## The moat

1. **One PRNG, in the core.** A single `SplitMix64`, seeded from `spec.seed`,
   drives every stochastic choice — initial sampling, parent selection, crossover
   and mutation. No binding has its own RNG; no wall-clock, thread id or address
   ever feeds a decision.
2. **Ordered collections.** The universe is a `BTreeMap<String, Vec<Candle>>` and
   the population is held in order, so iteration — and therefore the sequence of
   PRNG draws — is fixed.
3. **Canonical hashing.** `spec_hash` hashes a genome from a canonical
   serialisation with floats rounded (`round8`), so ranking ties break the same
   way regardless of float noise or map order.
4. **A finite fitness floor.** NaN/inf fitness collapses to `NEG_INFINITY` before
   ranking, so IEEE-754 edge cases can never reorder the hall of fame.

## Parallel ≡ sequential

The `parallel` feature (default) scores genomes across a `rayon` pool. Because
each genome is scored **independently** — the fitness of one candidate never
depends on another — the set of scores is identical whether computed on one thread
or many. Ranking then runs on that identical set. So:

```
cargo test                          # default features (rayon)
cargo test --no-default-features    # single-threaded
```

produce the **same** blessed golden reports. The fuzzers and the golden replay run
with `default-features = false` precisely to pin this equivalence.

## Cross-language byte-identity

Every binding calls the same `command_json` and returns the core's response
string **verbatim** — no re-serialisation, no per-language float formatting, no
JSON deep-equal. So the report a Python caller sees is the exact bytes the Rust
core produced, which is the exact bytes the Go, Java, C# … callers see. The golden
corpus is generated once in Rust and replayed unchanged everywhere.

## What is *not* guaranteed

- **Across engine versions.** A change to `wickra-backtest`'s maths can change
  fitness values and therefore the report. Determinism is pinned per engine
  version; the golden corpus is re-blessed when the engine is bumped.
- **Financial meaning.** Determinism is about reproducibility, not edge — see
  [FITNESS.md](FITNESS.md#a-caveat-that-is-not-a-bug).

## See also

- [ARCHITECTURE.md](ARCHITECTURE.md) — the `command_json` seam.
- [EVOLUTION.md](EVOLUTION.md) — the loop the PRNG drives.

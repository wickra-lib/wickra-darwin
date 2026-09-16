<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Darwin — evolutionary strategy search at hundreds of thousands of backtests per second" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/ci.svg)](https://github.com/wickra-lib/wickra-darwin/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-darwin)
[![npm](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/npm.svg)](https://www.npmjs.com/package/wickra-darwin)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/license.svg)](https://github.com/wickra-lib/wickra-darwin#license)

# Wickra Darwin — Node.js

---

**Part of the [Wickra ecosystem](#ecosystem): — for Node.js. `npm install wickra-darwin` — prebuilt native binary, no system dependencies.**

Node.js bindings for [`wickra-darwin`](https://github.com/wickra-lib/wickra-darwin),
powered by Rust via [napi-rs](https://napi.rs/): evolve a population of Wickra
strategy specs and get a **byte-identical search** — reproducible from its seed
across every language binding.

## Install

```bash
npm install wickra-darwin
```

The native addon ships as a prebuilt binary per platform (Linux, macOS,
Windows — x64 and arm64), selected automatically through optional
dependencies. There is nothing to compile.

Requires Node.js >= 22. The correct native binary is installed automatically as
an optional dependency for your platform.

## Quick start

```js
const { Darwin } = require("wickra-darwin");

const spec = {
  seed: 1, population: 8, generations: 3,
  mutation_rate: 0.2, crossover_rate: 0.6, fitness: "sharpe",
  search_space: {
    indicators: [{ name: "rsi", param_ranges: [{ min: 2, max: 30, step: 1 }] }],
    rules: "single_threshold", max_conditions: 1,
  },
  elitism: 1, top: 5,
};

const darwin = new Darwin(JSON.stringify(spec));
const data = {
  BTCUSDT: [
    { time: 1700000000, open: 100, high: 101, low: 99, close: 100.5, volume: 10 },
  ],
};
const report = JSON.parse(darwin.command(JSON.stringify({ cmd: "evolve", data })));
console.log(report.best);  // deterministic across runs and languages
```

`command` mirrors `Darwin::command_json`: the commands are `set_spec`, `evolve`,
`best` and `version`. A `new Darwin("{}")` defers configuration to a later
`set_spec`. An invalid spec throws; a command failure throws too.

### Determinism

The search's PRNG lives only in the Rust core; this binding forwards the command
string verbatim, so a given seed produces the byte-identical report here and in
every other binding — the exact cross-language golden invariant.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of napi-rs, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-darwin/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-darwin>
- **Docs** (guides, spec reference, cookbook): <https://darwin.wickra.org>
- **Runnable example:** [`examples/node/`](https://github.com/wickra-lib/wickra-darwin/tree/main/examples/node)

- The main project: <https://github.com/wickra-lib/wickra-darwin>
- Documentation: <https://wickra.org>

Wickra Darwin ships native bindings for Python, Node.js, WASM and Rust, plus a C ABI hub that any
C-capable language (C, C++, C#, Go, Java, R) links against — all forwarding to the
same data-driven, `unsafe`-forbidden Rust core.

## Security

Found a security issue? **Please don't open a public issue.** Report it privately
via the repository's *Security* tab (*"Report a vulnerability"*) or email
**support@wickra.org** with a subject line starting `[wickra security]`. Full
policy: <https://github.com/wickra-lib/wickra-darwin/blob/main/SECURITY.md>.

## Disclaimer

Wickra Darwin is a research tool, provided "as is" without warranty of any kind.
Evolutionary search optimises a fitness objective over historical data — a strong
in-sample fitness is not evidence of out-of-sample performance, and overfitting is
the default outcome, not the exception. Nothing here is financial advice; any
strategy you deploy is your responsibility, and trading carries risk of loss.

## License

Licensed under either of [Apache-2.0](https://github.com/wickra-lib/wickra-darwin/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/wickra-lib/wickra-darwin/blob/main/LICENSE-MIT) at your option.

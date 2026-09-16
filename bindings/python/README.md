<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Darwin — evolutionary strategy search at hundreds of thousands of backtests per second" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/ci.svg)](https://github.com/wickra-lib/wickra-darwin/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-darwin)
[![PyPI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/pypi.svg)](https://pypi.org/project/wickra-darwin/)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/license.svg)](https://github.com/wickra-lib/wickra-darwin#license)

# Wickra Darwin — Python

---

**Part of the [Wickra ecosystem](#ecosystem): — for Python. `pip install wickra-darwin` — prebuilt wheels for Linux, macOS and Windows, nothing to compile.**

Python bindings for the Wickra evolutionary strategy search, built with PyO3 and
maturin. A `Darwin` handle is driven over a JSON boundary, so a fixed seed yields
the byte-identical search as every other Wickra Darwin binding.

## Install

```bash
pip install wickra-darwin
```

Pre-built wheels ship for Linux, macOS and Windows — there is nothing to
compile and no C library to track down.

## Quick start

```python
import json
from wickra_darwin import Darwin

spec = {
    "seed": 1, "population": 8, "generations": 3,
    "mutation_rate": 0.2, "crossover_rate": 0.6, "fitness": "sharpe",
    "search_space": {
        "indicators": [{"name": "rsi", "param_ranges": [{"min": 2, "max": 30, "step": 1}]}],
        "rules": "single_threshold", "max_conditions": 1,
    },
    "elitism": 1, "top": 5,
}

darwin = Darwin(json.dumps(spec))
data = {"BTCUSDT": [ { "time": 1700000000, "open": 100, "high": 101, "low": 99, "close": 100.5, "volume": 10 } ]}
report = json.loads(darwin.command(json.dumps({"cmd": "evolve", "data": data})))
print(report["best"])
```

### Surface

- **`Darwin(spec_json)`** — construct a search handle from an `EvolveSpec` JSON
  (`"{}"` defers configuration to a later `set_spec`). Raises `ValueError` on an
  invalid spec.
- **`Darwin.command(cmd_json)`** — apply a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `set_spec`,
  `evolve`, `best`, `version`. Raises `RuntimeError` on a command failure.
- **`Darwin.version()`** — the library version.

### Determinism

The search's PRNG lives only in the Rust core; this binding forwards the command
string verbatim, so a given seed produces the byte-identical report here and in
every other binding — the exact cross-language golden invariant.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of PyO3, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-darwin/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-darwin>
- **Docs** (guides, spec reference, cookbook): <https://darwin.wickra.org>
- **Runnable example:** [`examples/python/`](https://github.com/wickra-lib/wickra-darwin/tree/main/examples/python)

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

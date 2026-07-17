# Wickra Darwin — Python

Python bindings for the Wickra evolutionary strategy search, built with PyO3 and
maturin. A `Darwin` handle is driven over a JSON boundary, so a fixed seed yields
the byte-identical search as every other Wickra Darwin binding.

## Install

```bash
pip install wickra-darwin
```

## Usage

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

## Surface

- **`Darwin(spec_json)`** — construct a search handle from an `EvolveSpec` JSON
  (`"{}"` defers configuration to a later `set_spec`). Raises `ValueError` on an
  invalid spec.
- **`Darwin.command(cmd_json)`** — apply a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `set_spec`,
  `evolve`, `best`, `version`. Raises `RuntimeError` on a command failure.
- **`Darwin.version()`** — the library version.

## Determinism

The search's PRNG lives only in the Rust core; this binding forwards the command
string verbatim, so a given seed produces the byte-identical report here and in
every other binding — the exact cross-language golden invariant.

## See also

- The main project: <https://github.com/wickra-lib/wickra-darwin>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either [MIT](../../LICENSE-MIT) or
[Apache-2.0](../../LICENSE-APACHE), at your option.

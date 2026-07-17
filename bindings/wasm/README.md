# Wickra Darwin — WASM

WebAssembly bindings for the Wickra evolutionary strategy search, compiled from
Rust with [wasm-bindgen](https://wasm-bindgen.github.io/wasm-bindgen/). A
`Darwin` is built from a spec JSON and driven by command JSONs over a JSON
boundary, so a browser front-end runs against the exact same core as every other
Wickra Darwin binding.

## Build

```bash
wasm-pack build --target web      # for a browser bundler
wasm-pack build --target nodejs   # for node:test / Node.js
```

The output lands in `pkg/`.

## Usage

```js
import init, { Darwin } from "./pkg/wickra_darwin_wasm.js";

await init();

const spec = JSON.stringify({
  seed: 1, population: 8, generations: 3,
  mutation_rate: 0.2, crossover_rate: 0.6, fitness: "sharpe",
  search_space: {
    indicators: [{ name: "rsi", param_ranges: [{ min: 2, max: 30, step: 1 }] }],
    rules: "single_threshold", max_conditions: 1,
  },
  elitism: 1, top: 5,
});

const data = {
  BTCUSDT: [
    { time: 1700000000, open: 100, high: 101, low: 99, close: 100.5, volume: 10 },
  ],
};

const darwin = new Darwin(spec);
const report = JSON.parse(darwin.command(JSON.stringify({ cmd: "evolve", data })));
console.log(report.best);
```

`command` mirrors `Darwin::command_json`: the commands are `set_spec`, `evolve`,
`best` and `version`. An invalid spec throws; a command failure throws too.

## Determinism

The search runs single-threaded here — no rayon thread pool in a browser
sandbox — which is byte-identical to the native, parallel run. A given seed
produces the byte-identical report here and in every other binding: the exact
cross-language golden invariant.

## See also

- The main project: <https://github.com/wickra-lib/wickra-darwin>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either [MIT](../../LICENSE-MIT) or
[Apache-2.0](../../LICENSE-APACHE), at your option.

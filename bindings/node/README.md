# wickra-darwin (Node.js)

Node.js bindings for [`wickra-darwin`](https://github.com/wickra-lib/wickra-darwin),
powered by Rust via [napi-rs](https://napi.rs/): evolve a population of Wickra
strategy specs and get a **byte-identical search** — reproducible from its seed
across every language binding.

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

## Install

```bash
npm install wickra-darwin
```

Requires Node.js >= 22. The correct native binary is installed automatically as
an optional dependency for your platform.

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

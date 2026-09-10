"use strict";

// The batch search, through the WASM boundary.
//
// There is no streaming half in DARWIN: a search is a batch computation over a
// whole dataset. What stands in its place is the property that makes the batch
// usable -- the same seed reproduces the same search -- and the WASM build has
// to reach it too, sequentially where the native build is parallel.
//
// The second test names `macd`, which takes three parameters. Until the search
// space resolved through the registry, `indicator_kind` was a four-name
// allowlist that declared every indicator as taking one.
//
// Skips cleanly when `pkg/` has not been built yet
// (`wasm-pack build --target nodejs`).

const { test } = require("node:test");
const assert = require("node:assert");
const path = require("node:path");

let wasm = null;
try {
  wasm = require(path.resolve(__dirname, "..", "pkg", "wickra_darwin_wasm.js"));
} catch {
  wasm = null;
}

const spec = (indicators) => JSON.stringify({
  seed: 7,
  population: 8,
  generations: 3,
  mutation_rate: 0.2,
  crossover_rate: 0.6,
  fitness: "sharpe",
  elitism: 1,
  top: 3,
  search_space: { indicators, rules: "single_threshold", max_conditions: 2 },
});

const ONE_PARAMETER = [
  { name: "rsi", param_ranges: [{ min: 2, max: 30, step: 1 }] },
];

const THREE_PARAMETERS = [
  {
    name: "macd",
    param_ranges: [
      { min: 8, max: 16, step: 2 },
      { min: 20, max: 30, step: 2 },
      { min: 5, max: 12, step: 1 },
    ],
  },
];

const candles = (n) =>
  Array.from({ length: n }, (_, i) => {
    const close = 100.0 + 10.0 * Math.sin(i * 0.1) + 0.05 * i;
    const opn = 100.0 + 10.0 * Math.sin((i - 1) * 0.1) + 0.05 * (i - 1);
    return {
      time: 1700000000 + i * 3600,
      open: opn,
      high: Math.max(close, opn) + 1.0,
      low: Math.min(close, opn) - 1.0,
      close,
      volume: 1000.0,
    };
  });

const DATA = { AAA: candles(250), BBB: candles(250) };

function search(s) {
  return new wasm.Darwin(s).command(JSON.stringify({ cmd: "evolve", data: DATA }));
}

test("the batch search is reproducible from its seed", { skip: wasm === null }, () => {
  const s = spec(ONE_PARAMETER);
  assert.strictEqual(search(s), search(s));
});

test("a three-parameter indicator is searchable", { skip: wasm === null }, () => {
  const report = JSON.parse(search(spec(THREE_PARAMETERS)));
  assert.ok(report.history.length > 0);
  assert.ok(report.best.length > 0);
});

"use strict";

// Golden test over the wasm-pack (nodejs target) output: the WebAssembly build
// evolves byte-identically to the native run — the single-threaded search in the
// browser sandbox reproduces the same seed exactly. Skips cleanly when `pkg/` has
// not been built yet (`wasm-pack build --target nodejs`).

const { test } = require("node:test");
const assert = require("node:assert");
const path = require("node:path");

let wasm = null;
try {
  wasm = require(path.resolve(__dirname, "..", "pkg", "wickra_darwin_wasm.js"));
} catch {
  wasm = null;
}

const SPEC = JSON.stringify({
  seed: 1,
  population: 8,
  generations: 3,
  mutation_rate: 0.2,
  crossover_rate: 0.6,
  fitness: "sharpe",
  search_space: {
    indicators: [{ name: "rsi", param_ranges: [{ min: 2, max: 30, step: 1 }] }],
    rules: "single_threshold",
    max_conditions: 1,
  },
  elitism: 1,
  top: 3,
});

function candles(n) {
  const out = [];
  for (let i = 0; i < n; i++) {
    const close = 100.0 + 10.0 * Math.sin(i * 0.1) + 0.05 * i;
    const opn = 100.0 + 10.0 * Math.sin((i - 1) * 0.1) + 0.05 * (i - 1);
    out.push({
      time: 1_700_000_000 + i * 3600,
      open: opn,
      high: Math.max(close, opn) + 1.0,
      low: Math.min(close, opn) - 1.0,
      close,
      volume: 1000.0,
    });
  }
  return out;
}

function evolveCmd() {
  return JSON.stringify({ cmd: "evolve", data: { AAA: candles(250) } });
}

test("wasm build present or skipped", (t) => {
  if (!wasm) t.skip("run `wasm-pack build --target nodejs` first");
});

if (wasm) {
  test("wasm evolve returns the expected history length", () => {
    const out = JSON.parse(new wasm.Darwin(SPEC).command(evolveCmd()));
    assert.strictEqual(out.history.length, 4);
    assert.ok(Array.isArray(out.best));
  });

  test("wasm evolve is byte-identical across calls", () => {
    const a = new wasm.Darwin(SPEC).command(evolveCmd());
    const b = new wasm.Darwin(SPEC).command(evolveCmd());
    assert.strictEqual(a, b);
  });

  test("wasm version matches the module export", () => {
    assert.strictEqual(new wasm.Darwin(SPEC).version(), wasm.version());
  });

  test("wasm throws on an invalid spec", () => {
    assert.throws(() => new wasm.Darwin("{ not valid json"));
  });
}

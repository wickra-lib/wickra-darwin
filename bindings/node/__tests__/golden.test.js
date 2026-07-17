"use strict";

// Determinism: a fixed seed yields the byte-identical report string. The full
// cross-language golden (asserting the response equals a blessed
// golden/expected file) lands with the golden corpus in P-DAR-4; here we pin the
// core invariant that the search is byte-reproducible from its seed, which every
// binding must preserve by forwarding the command string verbatim.

const { test } = require("node:test");
const assert = require("node:assert");
const { Darwin } = require("../index.js");

const SPEC = {
  seed: 7,
  population: 6,
  generations: 2,
  mutation_rate: 0.3,
  crossover_rate: 0.5,
  fitness: "pnl",
  search_space: {
    indicators: [{ name: "rsi", param_ranges: [{ min: 5, max: 25, step: 1 }] }],
    rules: "single_threshold",
    max_conditions: 1,
  },
  elitism: 1,
  top: 3,
};

function data() {
  const candles = [];
  for (let i = 0; i < 220; i++) {
    const close = 100.0 + 8.0 * Math.sin(i * 0.15) + 0.03 * i;
    const opn = 100.0 + 8.0 * Math.sin((i - 1) * 0.15) + 0.03 * (i - 1);
    candles.push({
      time: 1_700_000_000 + i * 3600,
      open: opn,
      high: Math.max(close, opn) + 1.0,
      low: Math.min(close, opn) - 1.0,
      close,
      volume: 1000.0,
    });
  }
  return { SYM: candles };
}

test("the same seed yields the byte-identical report string", () => {
  const cmd = JSON.stringify({ cmd: "evolve", data: data() });
  const a = new Darwin(JSON.stringify(SPEC)).command(cmd);
  const b = new Darwin(JSON.stringify(SPEC)).command(cmd);
  assert.strictEqual(a, b);
});

test("a different seed still produces a valid report", () => {
  const cmd = JSON.stringify({ cmd: "evolve", data: data() });
  const a = JSON.parse(new Darwin(JSON.stringify(SPEC)).command(cmd));
  const b = JSON.parse(
    new Darwin(JSON.stringify({ ...SPEC, seed: 99 })).command(cmd),
  );
  assert.ok(a.history.length > 0);
  assert.ok(b.history.length > 0);
});

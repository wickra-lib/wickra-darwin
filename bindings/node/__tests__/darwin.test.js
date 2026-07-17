"use strict";

const { test } = require("node:test");
const assert = require("node:assert");
const { Darwin } = require("../index.js");

const SPEC = {
  seed: 1,
  population: 8,
  generations: 3,
  mutation_rate: 0.2,
  crossover_rate: 0.6,
  fitness: "sharpe",
  search_space: {
    indicators: [
      { name: "rsi", param_ranges: [{ min: 2, max: 30, step: 1 }] },
      { name: "ema", param_ranges: [{ min: 5, max: 100, step: 5 }] },
    ],
    rules: "conjunction_all",
    max_conditions: 2,
  },
  elitism: 1,
  top: 3,
};

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

function evolve(darwin) {
  const data = { AAA: candles(250), BBB: candles(250) };
  return JSON.parse(darwin.command(JSON.stringify({ cmd: "evolve", data })));
}

test("evolve returns a report with the expected history length", () => {
  const report = evolve(new Darwin(JSON.stringify(SPEC)));
  assert.strictEqual(report.history.length, SPEC.generations + 1);
  assert.ok(Array.isArray(report.best));
});

test("the same seed yields byte-identical output", () => {
  const data = { AAA: candles(250), BBB: candles(250) };
  const cmd = JSON.stringify({ cmd: "evolve", data });
  const a = new Darwin(JSON.stringify(SPEC)).command(cmd);
  const b = new Darwin(JSON.stringify(SPEC)).command(cmd);
  assert.strictEqual(a, b);
});

test("set_spec then evolve produces a report", () => {
  const darwin = new Darwin("{}");
  const ok = JSON.parse(
    darwin.command(JSON.stringify({ cmd: "set_spec", spec: SPEC })),
  );
  assert.strictEqual(ok.ok, true);
  const report = evolve(darwin);
  assert.ok("history" in report);
});

test("an invalid spec throws", () => {
  assert.throws(() => new Darwin("{ not valid json"));
});

test("version is a string", () => {
  const darwin = new Darwin(JSON.stringify(SPEC));
  assert.strictEqual(typeof darwin.version(), "string");
});

module.exports = { SPEC, candles };

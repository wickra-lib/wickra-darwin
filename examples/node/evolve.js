// A runnable Node.js example: evolve strategy specs over a small deterministic
// universe and print the search summary.
//
//   npm install
//   node examples/node/evolve.js
//
// Every language example builds the same universe and prints the same summary.
"use strict";

const { Darwin } = require("wickra-darwin");

const SPEC = JSON.stringify({
  seed: 7,
  population: 10,
  generations: 4,
  mutation_rate: 0.2,
  crossover_rate: 0.6,
  fitness: "sharpe",
  search_space: {
    indicators: [{ name: "rsi", param_ranges: [{ min: 2, max: 30 }] }],
    rules: "single_threshold",
    max_conditions: 1,
  },
  elitism: 1,
  top: 3,
});

function evolveCommand() {
  const bars = [];
  for (let i = 0; i < 16; i++) {
    const close = 100.0 + 8.0 * Math.sin(i / 4.0) + 0.1 * i;
    const open = 100.0 + 8.0 * Math.sin((i - 1) / 4.0) + 0.1 * (i - 1);
    bars.push({
      time: 1700000000 + i * 3600,
      open: Number(open.toFixed(3)),
      high: Number((Math.max(close, open) + 1.0).toFixed(3)),
      low: Number((Math.min(close, open) - 1.0).toFixed(3)),
      close: Number(close.toFixed(3)),
      volume: 1000,
    });
  }
  return JSON.stringify({ cmd: "evolve", data: { SYM: bars } });
}

const darwin = new Darwin(SPEC);
const report = JSON.parse(darwin.command(evolveCommand()));
console.log(`wickra-darwin ${darwin.version()}`);
console.log(`generations: ${report.history.length}`);
console.log(`hall of fame: ${report.best.length}`);

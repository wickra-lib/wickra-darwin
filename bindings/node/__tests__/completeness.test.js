"use strict";

const { test } = require("node:test");
const assert = require("node:assert");
const { Darwin } = require("../index.js");

test("the Darwin surface exposes command and version", () => {
  const darwin = new Darwin("{}");
  assert.strictEqual(typeof darwin.command, "function");
  assert.strictEqual(typeof darwin.version, "function");
});

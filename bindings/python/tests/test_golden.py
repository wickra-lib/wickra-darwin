"""Determinism: a fixed seed yields the byte-identical report string.

The full cross-language golden (asserting the response equals a blessed
golden/expected file) lands with the golden corpus in P-DAR-4; here we pin the
core invariant that the search is byte-reproducible from its seed, which every
binding must preserve by forwarding the command string verbatim.
"""

import json
import math

from wickra_darwin import Darwin

SPEC = {
    "seed": 7,
    "population": 6,
    "generations": 2,
    "mutation_rate": 0.3,
    "crossover_rate": 0.5,
    "fitness": "pnl",
    "search_space": {
        "indicators": [{"name": "rsi", "param_ranges": [{"min": 5, "max": 25, "step": 1}]}],
        "rules": "single_threshold",
        "max_conditions": 1,
    },
    "elitism": 1,
    "top": 3,
}


def _data() -> dict:
    candles = []
    for i in range(220):
        close = 100.0 + 8.0 * math.sin(i * 0.15) + 0.03 * i
        opn = 100.0 + 8.0 * math.sin((i - 1) * 0.15) + 0.03 * (i - 1)
        candles.append(
            {
                "time": 1_700_000_000 + i * 3600,
                "open": opn,
                "high": max(close, opn) + 1.0,
                "low": min(close, opn) - 1.0,
                "close": close,
                "volume": 1000.0,
            }
        )
    return {"SYM": candles}


def test_same_seed_same_report_string() -> None:
    cmd = json.dumps({"cmd": "evolve", "data": _data()})
    a = Darwin(json.dumps(SPEC)).command(cmd)
    b = Darwin(json.dumps(SPEC)).command(cmd)
    assert a == b


def test_different_seed_may_differ() -> None:
    cmd = json.dumps({"cmd": "evolve", "data": _data()})
    a = Darwin(json.dumps(SPEC)).command(cmd)
    other = {**SPEC, "seed": 99}
    b = Darwin(json.dumps(other)).command(cmd)
    # Byte-reproducible per seed; the strings are valid JSON reports either way.
    assert json.loads(a)["history"]
    assert json.loads(b)["history"]

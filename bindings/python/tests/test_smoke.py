"""Smoke test: construct a search, run a small evolution, parse the report."""

import json
import math

from wickra_darwin import Darwin, __version__

SPEC = {
    "seed": 1,
    "population": 8,
    "generations": 3,
    "mutation_rate": 0.2,
    "crossover_rate": 0.6,
    "fitness": "sharpe",
    "search_space": {
        "indicators": [
            {"name": "rsi", "param_ranges": [{"min": 2, "max": 30, "step": 1}]},
            {"name": "ema", "param_ranges": [{"min": 5, "max": 100, "step": 5}]},
        ],
        "rules": "conjunction_all",
        "max_conditions": 2,
    },
    "elitism": 1,
    "top": 3,
}


def _candles(n: int) -> list[dict]:
    out = []
    for i in range(n):
        close = 100.0 + 10.0 * math.sin(i * 0.1) + 0.05 * i
        opn = 100.0 + 10.0 * math.sin((i - 1) * 0.1) + 0.05 * (i - 1)
        out.append(
            {
                "time": 1_700_000_000 + i * 3600,
                "open": opn,
                "high": max(close, opn) + 1.0,
                "low": min(close, opn) - 1.0,
                "close": close,
                "volume": 1000.0,
            }
        )
    return out


def _evolve(darwin: Darwin) -> dict:
    data = {"AAA": _candles(250), "BBB": _candles(250)}
    return json.loads(darwin.command(json.dumps({"cmd": "evolve", "data": data})))


def test_evolve_returns_report() -> None:
    darwin = Darwin(json.dumps(SPEC))
    report = _evolve(darwin)
    assert len(report["history"]) == SPEC["generations"] + 1
    assert isinstance(report["best"], list)


def test_deterministic_across_instances() -> None:
    a = _evolve(Darwin(json.dumps(SPEC)))
    b = _evolve(Darwin(json.dumps(SPEC)))
    assert a == b


def test_set_spec_then_evolve() -> None:
    darwin = Darwin("{}")
    ok = json.loads(darwin.command(json.dumps({"cmd": "set_spec", "spec": SPEC})))
    assert ok["ok"] is True
    report = _evolve(darwin)
    assert "history" in report


def test_version() -> None:
    assert Darwin.version() == __version__

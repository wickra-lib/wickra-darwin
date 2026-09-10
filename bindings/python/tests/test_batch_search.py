"""The batch search, driven through the JSON command boundary.

There is no streaming half in DARWIN: a search is a batch computation over a
whole dataset. What stands in its place is the property that makes the batch
usable -- the same seed reproduces the same search -- and it has to hold through
this binding, not only in Rust. A binding that let anything of its own into the
loop (a map iteration order, a locale-formatted number in the payload) would
produce a search that is reproducible only on the machine that ran it.

The second test names `macd`, which takes three parameters. Until the search
space resolved through the registry, `indicator_kind` was a four-name allowlist
that declared every indicator as taking one -- so this spec was unreachable
twice over, and no binding could have run it.
"""

import json
import math

from wickra_darwin import Darwin


def _spec(indicators: list) -> str:
    return json.dumps(
        {
            "seed": 7,
            "population": 8,
            "generations": 3,
            "mutation_rate": 0.2,
            "crossover_rate": 0.6,
            "fitness": "sharpe",
            "elitism": 1,
            "top": 3,
            "search_space": {
                "indicators": indicators,
                "rules": "single_threshold",
                "max_conditions": 2,
            },
        }
    )


ONE_PARAMETER = [{"name": "rsi", "param_ranges": [{"min": 2, "max": 30, "step": 1}]}]

# `macd` takes three parameters. The four-name allowlist this search space
# replaced declared every indicator as taking one, so this spec could not be
# expressed at all.
THREE_PARAMETERS = [
    {
        "name": "macd",
        "param_ranges": [
            {"min": 8, "max": 16, "step": 2},
            {"min": 20, "max": 30, "step": 2},
            {"min": 5, "max": 12, "step": 1},
        ],
    }
]


def _candles(n: int) -> list:
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


DATA = {"AAA": _candles(250), "BBB": _candles(250)}


def _search(spec: str) -> str:
    darwin = Darwin(spec)
    return darwin.command(json.dumps({"cmd": "evolve", "data": DATA}))


def test_the_batch_search_is_reproducible_from_its_seed() -> None:
    spec = _spec(ONE_PARAMETER)
    assert _search(spec) == _search(spec)


def test_a_three_parameter_indicator_is_searchable() -> None:
    report = json.loads(_search(_spec(THREE_PARAMETERS)))
    assert report["history"], "the search produced no generations"
    assert report["best"], "the search produced no ranked genomes"


def test_a_name_the_registry_does_not_know_is_refused() -> None:
    spec = _spec([{"name": "notanindicator",
                   "param_ranges": [{"min": 2, "max": 30, "step": 1}]}])
    try:
        Darwin(spec)
    except Exception as err:  # noqa: BLE001 - the binding's own error type
        assert "notanindicator" in str(err).lower()
    else:
        raise AssertionError("an unknown indicator name must be refused")

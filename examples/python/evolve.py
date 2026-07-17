"""A runnable Python example: evolve strategy specs over a small deterministic
universe and print the search summary.

    pip install wickra-darwin
    python examples/python/evolve.py

Every language example builds the same 16-bar universe and the same seeded spec,
so they all print the same summary — that is the cross-language guarantee.
"""

import json
import math

from wickra_darwin import Darwin

SPEC = json.dumps(
    {
        "seed": 7,
        "population": 10,
        "generations": 4,
        "mutation_rate": 0.2,
        "crossover_rate": 0.6,
        "fitness": "sharpe",
        "search_space": {
            "indicators": [{"name": "rsi", "param_ranges": [{"min": 2, "max": 30}]}],
            "rules": "single_threshold",
            "max_conditions": 1,
        },
        "elitism": 1,
        "top": 3,
    }
)


def evolve_command() -> str:
    bars = []
    for i in range(16):
        close = 100.0 + 8.0 * math.sin(i / 4.0) + 0.1 * i
        opn = 100.0 + 8.0 * math.sin((i - 1) / 4.0) + 0.1 * (i - 1)
        bars.append(
            {
                "time": 1700000000 + i * 3600,
                "open": round(opn, 3),
                "high": round(max(close, opn) + 1.0, 3),
                "low": round(min(close, opn) - 1.0, 3),
                "close": round(close, 3),
                "volume": 1000,
            }
        )
    return json.dumps({"cmd": "evolve", "data": {"SYM": bars}})


def main() -> None:
    darwin = Darwin(SPEC)
    report = json.loads(darwin.command(evolve_command()))
    print(f"wickra-darwin {Darwin.version()}")
    print(f"generations: {len(report['history'])}")
    print(f"hall of fame: {len(report['best'])}")


if __name__ == "__main__":
    main()

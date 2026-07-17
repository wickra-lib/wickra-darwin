# Fitness

Fitness reduces one backtest — the equity curve a genome produces when run through
`wickra-backtest` over the universe — to a single `f64` the search maximises.

## Objectives

`Fitness` is chosen by the `fitness` field of the `EvolveSpec`:

| Variant  | JSON     | Rewards |
|----------|----------|---------|
| `Sharpe` | `sharpe` | risk-adjusted return — mean return over its standard deviation. Punishes volatile equity even when total PnL is high. |
| `Pnl`    | `pnl`    | raw terminal profit and loss. Simple, but blind to drawdown and volatility. |
| `Calmar` | `calmar` | return over maximum drawdown — favours strategies that grow without deep valleys. |

All three are **maximised**: a higher score ranks a genome higher.

## The `NEG_INFINITY` floor

A genome that never trades, or whose result is non-finite (NaN/inf — e.g. a Sharpe
with zero variance, or a Calmar with zero drawdown), scores `f64::NEG_INFINITY`.
Such genomes sort to the bottom and never enter the hall of fame. This is why a
search over data with no tradeable signal returns an empty `best` list rather than
a spurious "winner" (see [EVOLUTION.md](EVOLUTION.md#degenerate-case-an-empty-hall-of-fame)).

## Choosing an objective

- **`pnl`** for a first pass, or when you will post-filter on risk yourself.
- **`sharpe`** when consistency matters more than headline return — the usual
  default for strategy search.
- **`calmar`** when survivable drawdown is the binding constraint (leverage, margin).

Because fitness only ranks genomes, switching the objective changes *which*
strategies win but not the determinism of the run: the same seed with the same
objective always yields the same report.

## A caveat that is not a bug

Optimising any of these objectives over historical data **overfits by default**.
A high in-sample Sharpe is not evidence of out-of-sample edge. Treat the hall of
fame as hypotheses to validate on held-out data, never as deployable strategies.

## See also

- [EVOLUTION.md](EVOLUTION.md) — where scoring sits in the loop.
- [GENOME.md](GENOME.md) — what is being scored.

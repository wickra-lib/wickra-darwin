# Wickra Darwin — R

R bindings for the Wickra evolutionary strategy search over its C ABI hub, via
`.Call`. A search is built from a spec JSON and driven over a JSON boundary, so
the result is byte-identical to every other Wickra Darwin binding.

## Build & test

The C ABI header and shared library are provided out-of-tree through two
environment variables (set by CI / the installer):

```bash
export WKDARWIN_INC=/path/to/bindings/c/include   # the header dir
export WKDARWIN_LIB=/path/to/target/release       # the library dir
R CMD INSTALL bindings/r
Rscript bindings/r/tests/run_tests.R
```

At run time the loader must find the shared library on `LD_LIBRARY_PATH`
(Linux), `DYLD_LIBRARY_PATH` (macOS) or `PATH` (Windows).

## Usage

```r
library(wickradarwin)

spec <- paste0(
  '{"seed":1,"population":8,"generations":3,',
  '"mutation_rate":0.2,"crossover_rate":0.6,"fitness":"sharpe",',
  '"search_space":{"indicators":[{"name":"rsi","param_ranges":',
  '[{"min":2,"max":30,"step":1}]}],"rules":"single_threshold",',
  '"max_conditions":1},"elitism":1,"top":5}'
)

darwin <- wkdarwin_new(spec)
data <- '{"BTCUSDT":[{"time":1700000000,"open":100,"high":101,"low":99,"close":100.5,"volume":10}]}'
response <- wkdarwin_command(darwin, paste0('{"cmd":"evolve","data":', data, "}"))
cat(response)
```

## Surface

- **`wkdarwin_new(spec_json)`** — build a search handle from a spec JSON (an
  external pointer; `"{}"` defers configuration to a later `set_spec`).
- **`wkdarwin_command(darwin, cmd_json)`** — apply a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `set_spec`,
  `evolve`, `best`, `version`.
- **`wkdarwin_version()`** — the library version.

## Determinism

The search's PRNG lives only in the Rust core; this binding forwards the command
string verbatim, so a given seed produces the byte-identical report here and in
every other binding — the exact cross-language golden invariant.

## See also

- The main project: <https://github.com/wickra-lib/wickra-darwin>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either MIT or Apache-2.0, at your option.

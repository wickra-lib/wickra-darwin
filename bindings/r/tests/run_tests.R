## Plain-R tests for the wickra-darwin R binding (no testthat dependency).
## Mirrors the Rust/Python/Node/Go/C#/Java tests and doubles as the completeness
## guard: it exercises the full public surface (version + new + command).

library(wickradarwin)

spec <- paste0(
  '{"seed":1,"population":8,"generations":3,',
  '"mutation_rate":0.2,"crossover_rate":0.6,"fitness":"sharpe",',
  '"search_space":{"indicators":[{"name":"rsi","param_ranges":',
  '[{"min":2,"max":30,"step":1}]}],"rules":"single_threshold",',
  '"max_conditions":1},"elitism":1,"top":3}'
)

## A deterministic sine price path over 250 bars.
evolve_cmd <- function() {
  bars <- vapply(0:249, function(i) {
    close <- 100.0 + 10.0 * sin(i * 0.1) + 0.05 * i
    open <- 100.0 + 10.0 * sin((i - 1) * 0.1) + 0.05 * (i - 1)
    high <- max(close, open) + 1.0
    low <- min(close, open) - 1.0
    sprintf(
      '{"time":%d,"open":%s,"high":%s,"low":%s,"close":%s,"volume":1000.0}',
      1700000000 + i * 3600,
      format(open, digits = 17), format(high, digits = 17),
      format(low, digits = 17), format(close, digits = 17)
    )
  }, character(1))
  paste0('{"cmd":"evolve","data":{"AAA":[', paste(bars, collapse = ","), "]}}")
}

## version
stopifnot(nzchar(wkdarwin_version()))

## evolve returns a report with a history and a best list
darwin <- wkdarwin_new(spec)
out <- wkdarwin_command(darwin, evolve_cmd())
stopifnot(grepl('"history"', out, fixed = TRUE))
stopifnot(grepl('"best"', out, fixed = TRUE))

## evolve is byte-identical across handles (the cross-language golden core)
darwin2 <- wkdarwin_new(spec)
out2 <- wkdarwin_command(darwin2, evolve_cmd())
stopifnot(identical(out, out2))

## an invalid spec is a hard error at construction
err <- tryCatch(wkdarwin_new("{ not valid json"), error = function(e) e)
stopifnot(inherits(err, "error"))

## set_spec on a deferred handle, then evolve
deferred <- wkdarwin_new("{}")
ok <- wkdarwin_command(deferred, paste0('{"cmd":"set_spec","spec":', spec, "}"))
stopifnot(grepl('"ok":true', ok, fixed = TRUE))
stopifnot(grepl('"history"', wkdarwin_command(deferred, evolve_cmd()), fixed = TRUE))

cat("wickra-darwin R tests passed\n")

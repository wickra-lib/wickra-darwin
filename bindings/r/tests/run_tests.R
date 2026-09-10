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


## The batch search, through the same boundary.
##
## There is no streaming half in DARWIN: a search is a batch computation over a
## whole dataset. What stands in its place is the property that makes the batch
## usable -- the same seed reproduces the same search -- and it has to hold
## through this binding, not only in Rust.
##
## The second check names `macd`, which takes three parameters. Until the search
## space resolved through the registry, the allowlist was four names and declared
## every indicator as taking one, so this spec was unreachable twice over.

batch_spec <- function(indicators) {
  paste0(
    '{"seed":7,"population":8,"generations":3,',
    '"mutation_rate":0.2,"crossover_rate":0.6,"fitness":"sharpe",',
    '"elitism":1,"top":3,"search_space":{"indicators":[', indicators,
    '],"rules":"single_threshold","max_conditions":2}}'
  )
}

one_parameter <- '{"name":"rsi","param_ranges":[{"min":2,"max":30,"step":1}]}'
three_parameters <- paste0(
  '{"name":"macd","param_ranges":[',
  '{"min":8,"max":16,"step":2},{"min":20,"max":30,"step":2},',
  '{"min":5,"max":12,"step":1}]}'
)

batch_search <- function(spec) {
  handle <- wkdarwin_new(spec)
  wkdarwin_command(handle, evolve_cmd())
}

## The same seed reproduces the same search.
stopifnot(identical(batch_search(batch_spec(one_parameter)),
                    batch_search(batch_spec(one_parameter))))

## A three-parameter indicator is searchable at all.
three_report <- batch_search(batch_spec(three_parameters))
stopifnot(grepl('"history"', three_report, fixed = TRUE))
stopifnot(grepl('"best"', three_report, fixed = TRUE))

## A name the registry does not know is refused.
bad_name <- batch_spec('{"name":"notanindicator","param_ranges":[{"min":2,"max":30,"step":1}]}')
bad_err <- tryCatch(wkdarwin_new(bad_name), error = function(e) e)
stopifnot(inherits(bad_err, "error"))

cat("wickra-darwin R tests passed\n")

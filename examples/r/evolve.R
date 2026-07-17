# A runnable R example: evolve strategy specs over a small deterministic universe
# and print the search summary.
#
#   R CMD INSTALL bindings/r
#   Rscript examples/r/evolve.R
#
# Every language example builds the same universe and prints the same summary.
library(wickradarwin)

spec <- paste0(
  '{"seed":7,"population":10,"generations":4,',
  '"mutation_rate":0.2,"crossover_rate":0.6,"fitness":"sharpe",',
  '"search_space":{"indicators":[{"name":"rsi","param_ranges":[{"min":2,"max":30}]}],',
  '"rules":"single_threshold","max_conditions":1},"elitism":1,"top":3}'
)

bars <- vapply(0:15, function(i) {
  close <- 100.0 + 8.0 * sin(i / 4.0) + 0.1 * i
  open <- 100.0 + 8.0 * sin((i - 1) / 4.0) + 0.1 * (i - 1)
  sprintf(
    '{"time":%d,"open":%.3f,"high":%.3f,"low":%.3f,"close":%.3f,"volume":1000}',
    1700000000 + i * 3600, open, max(close, open) + 1.0, min(close, open) - 1.0, close
  )
}, character(1))
cmd <- paste0('{"cmd":"evolve","data":{"SYM":[', paste(bars, collapse = ","), "]}}")

darwin <- wkdarwin_new(spec)
report <- wkdarwin_command(darwin, cmd)

cat(sprintf("wickra-darwin %s\n", wkdarwin_version()))
n_gen <- length(gregexpr('"generation"', report)[[1]])
cat(sprintf("generations: %d\n", n_gen))

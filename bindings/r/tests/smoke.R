# Shipped with the package and run by `R CMD check`, so it must work inside the
# built tarball -- where no repository sits above it and no fixture file exists.
# It touches nothing but the package: load the library, take a handle, drive one
# command through the boundary, and read the version back.
#
# The cross-language golden parity test lives in run_tests.R, which is excluded
# from the tarball by .Rbuildignore and run from the repository by CI. That one
# needs golden/ above it; this one needs nothing.

library(wickradarwin)

v <- wkdarwin_version()
stopifnot(is.character(v), length(v) == 1L, nzchar(v))

h <- wkdarwin_new('{"seed":1,"population":8,"generations":3,"mutation_rate":0.2,"crossover_rate":0.6,"fitness":"sharpe","search_space":{"indicators":[{"name":"rsi","param_ranges":[{"min":2,"max":30,"step":1}]}],"rules":"single_threshold","max_conditions":1},"elitism":1,"top":3}')
out <- wkdarwin_command(h, '{"cmd":"version"}')
stopifnot(is.character(out), length(out) == 1L)
stopifnot(grepl("version", out, fixed = TRUE))

cat("wickra-darwin R package smoke: ok (version ", v, ")\n", sep = "")

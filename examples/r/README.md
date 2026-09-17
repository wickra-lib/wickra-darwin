# Wickra Darwin examples — R

Runnable R examples for the [Wickra Darwin R binding](../../bindings/r). The package compiles a thin
`.Call` glue layer against the C ABI library, so build the library and install
the package first (the CI examples job does exactly this):

```bash
cargo build -p wickra-darwin-c --release
R CMD INSTALL bindings/r
```

## Run

```bash
Rscript examples/r/evolve.R
```

## The examples

| Example | What it does |
|---------|--------------|
| `evolve.R` | A runnable R example: evolve strategy specs over a small deterministic universe and print the search summary. |

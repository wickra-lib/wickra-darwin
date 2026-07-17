# Examples

Runnable examples in every Wickra Darwin language. Each one builds the same
16-bar deterministic universe and the same seeded `EvolveSpec`, runs the
evolutionary search, and prints the same summary — the cross-language guarantee:

```
wickra-darwin 0.1.0
generations: 5
hall of fame: 0
```

(The smooth demo universe does not produce a trading strategy, so the hall of
fame is empty — the search machinery and its determinism are what the example
demonstrates. See [`golden/README.md`](../golden/README.md).)

The canonical spec and a 40-bar candle CSV are also in
[`data/`](data/) for use with the CLI:

```bash
cargo run -p darwin-cli -- --spec examples/data/specs/evolve.json --data examples/data/candles
```

## Per language

- **Rust** — [`rust/`](rust/): `cargo run --manifest-path examples/rust/Cargo.toml`
- **Python** — [`python/evolve.py`](python/evolve.py): `pip install wickra-darwin && python examples/python/evolve.py`
- **Node.js** — [`node/`](node/): `cd examples/node && npm install && node evolve.js`
- **Go** — [`go/`](go/): `go run examples/go/evolve.go` (with the C ABI library staged, see the Go binding README)
- **C#** — [`csharp/Evolve/`](csharp/Evolve/): `dotnet run --project examples/csharp/Evolve`
- **Java** — [`java/`](java/): `mvn -q compile exec:java -Dexec.mainClass=org.wickra.darwin.examples.Evolve`
- **R** — [`r/evolve.R`](r/evolve.R): `R CMD INSTALL bindings/r && Rscript examples/r/evolve.R`
- **C / C++** — [`c/`](c/): build the C ABI, then CMake + ctest:

  ```bash
  cargo build --release -p wickra-darwin-c
  cmake -S examples/c -B examples/c/build
  cmake --build examples/c/build --config Release
  ctest --test-dir examples/c/build -C Release --output-on-failure
  ```

The binding examples install the published `wickra-darwin` package for their
language; the Rust and C/C++ examples build against the in-repo core.

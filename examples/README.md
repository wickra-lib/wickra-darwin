# Wickra Darwin examples

Runnable examples in every Wickra Darwin language. Each one builds the same
16-bar deterministic universe and the same seeded `EvolveSpec`, runs the
evolutionary search, and prints the same summary — the cross-language guarantee:

## What every example prints

Runnable examples in every Wickra Darwin language. Each one builds the same
16-bar deterministic universe and the same seeded `EvolveSpec`, runs the
evolutionary search, and prints the same summary — the cross-language guarantee:

```
wickra-darwin 0.1.4
generations: 5
hall of fame: 0
```

## Rust — `examples/rust/`

As the CI examples job runs it, from the repository root:

```bash
cargo run -q --manifest-path examples/rust/Cargo.toml
```

| Example | What it does |
| --- | --- |
| `src/main.rs` | A runnable Rust example: evolve strategy specs over a small deterministic universe and print the search summary. |

## C / C++ — `examples/c/`

Build the library first (`cargo build -p wickra-darwin-c --release`), then build and run
the examples via CMake, as the CI C ABI job does:

```bash
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

| Example | What it does |
| --- | --- |
| `evolve.c` | A runnable C example: evolve strategy specs over a small deterministic |
| `evolve.cpp` | A runnable C++ example: evolve strategy specs over a small deterministic universe and print the search summary. |

## C# — `examples/csharp/`

As the CI examples job runs it, from the repository root:

```bash
dotnet run --project examples/csharp/Evolve
```

| Example | What it does |
| --- | --- |
| `Evolve/Program.cs` | A runnable C# example: evolve strategy specs over a small deterministic universe and print the search summary. |

## Go — `examples/go/`

As the CI examples job runs it, from the repository root:

```bash
cd examples/go && go run .
```

| Example | What it does |
| --- | --- |
| `evolve.go` | A runnable Go example: evolve strategy specs over a small deterministic universe and print the search summary. |

## R — `examples/r/`

As the CI examples job runs it, from the repository root:

```bash
R CMD INSTALL bindings/r
Rscript examples/r/evolve.R
```

| Example | What it does |
| --- | --- |
| `evolve.R` | A runnable R example: evolve strategy specs over a small deterministic universe and print the search summary. |

## Java — `examples/java/`

As the CI examples job runs it, from the repository root:

```bash
mvn -f bindings/java/pom.xml -q install -DskipTests
mvn -f examples/java/pom.xml -q compile exec:exec  -Dnative.lib.dir="$PWD/target/release"
```

| Example | What it does |
| --- | --- |
| `src/main/java/org/wickra/darwin/examples/Evolve.java` | A runnable example against this binding. |

## Python — `examples/python/`

As the CI examples job runs it, from the repository root:

```bash
python -m pip install --require-hashes -r .github/requirements/ci-dev-py3.txt
( cd bindings/python && maturin build --release --out dist )
python -m pip install --no-index --find-links bindings/python/dist wickra-darwin
python examples/python/evolve.py
```

| Example | What it does |
| --- | --- |
| `evolve.py` | A runnable Python example: evolve strategy specs over a small deterministic |

## Node.js — `examples/node/`

As the CI examples job runs it, from the repository root:

```bash
( cd bindings/node && npm install --no-audit --no-fund && npx napi build --platform --release )
( cd examples/node && npm install --no-audit --no-fund )
node examples/node/evolve.js
```

| Example | What it does |
| --- | --- |
| `evolve.js` | A runnable Node.js example: evolve strategy specs over a small deterministic universe and print the search summary. |

## WASM — `examples/wasm/`

Build the WASM package, serve the repository root, and open the page in a browser;
the module script inside it is what runs (CI parses it with `node --check`):

```bash
wasm-pack build bindings/wasm --target web
python -m http.server 8000     # then open http://localhost:8000/examples/wasm/
```

| Example | What it does |
| --- | --- |
| `evolve.html` | A runnable example against this binding. |

## Example datasets

The examples read from [`examples/data/`](data/): . The
cross-language golden fixtures, which every binding is checked against byte for
byte, live in [`../golden/`](../golden).

## Per language

- **Rust** — [`rust/`](rust/): `cargo run --manifest-path examples/rust/Cargo.toml`
- **Python** — [`python/evolve.py`](python/evolve.py): `pip install wickra-darwin && python examples/python/evolve.py`
- **Node.js** — [`node/`](node/): `cd examples/node && npm install && node evolve.js`
- **Go** — [`go/`](go/): `go run examples/go/evolve.go` (with the C ABI library staged, see the Go binding README)
- **C#** — [`csharp/Evolve/`](csharp/Evolve/): `dotnet run --project examples/csharp/Evolve`
- **Java** — [`java/`](java/): `mvn -q compile exec:java -Dexec.mainClass=org.wickra.darwin.examples.Evolve`
- **R** — [`r/evolve.R`](r/evolve.R): `R CMD INSTALL bindings/r && Rscript examples/r/evolve.R`
- **WASM** — [`wasm/evolve.html`](wasm/evolve.html): `wasm-pack build bindings/wasm --target web`, serve the repository root, then open `examples/wasm/evolve.html`
- **C / C++** — [`c/`](c/): build the C ABI, then CMake + ctest:

  ```bash
  cargo build --release -p wickra-darwin-c
  cmake -S examples/c -B examples/c/build
  cmake --build examples/c/build --config Release
  ctest --test-dir examples/c/build -C Release --output-on-failure
  ```

The binding examples install the published `wickra-darwin` package for their
language; the Rust and C/C++ examples build against the in-repo core.

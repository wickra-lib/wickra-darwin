# Wickra Darwin examples — Go

Runnable Go examples for the [Wickra Darwin Go binding](../../bindings/go). The binding links against the
prebuilt C ABI library, so build and stage it once before running anything:

```bash
cargo build -p wickra-darwin-c --release
mkdir -p bindings/go/lib/linux_amd64
cp target/release/libwickra_darwin.so bindings/go/lib/linux_amd64/
```

## Run

As the CI examples job runs it, from the repository root:

```bash
cd examples/go && go run .
```

## The examples

| Example | What it does |
|---------|--------------|
| `evolve.go` | A runnable Go example: evolve strategy specs over a small deterministic universe and print the search summary. |

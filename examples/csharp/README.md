# Wickra Darwin examples — C#

Runnable C# examples for the [Wickra Darwin C# binding](../../bindings/csharp). The binding consumes the C ABI
library through P/Invoke, so build it once before running anything:

```bash
cargo build -p wickra-darwin-c --release
```

## Run

As the CI examples job runs it, from the repository root:

```bash
dotnet run --project examples/csharp/Evolve
```

## The examples

| Example | What it does |
|---------|--------------|
| `Evolve/Program.cs` | A runnable C# example: evolve strategy specs over a small deterministic universe and print the search summary. |

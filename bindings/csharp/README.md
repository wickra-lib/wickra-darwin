<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Darwin — evolutionary strategy search at hundreds of thousands of backtests per second" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/ci.svg)](https://github.com/wickra-lib/wickra-darwin/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-darwin)
[![NuGet](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/nuget.svg)](https://www.nuget.org/packages/Wickra.Darwin)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/license.svg)](https://github.com/wickra-lib/wickra-darwin#license)

# Wickra Darwin — C#

---

**Part of the [Wickra ecosystem](#ecosystem): — for C#. `dotnet add package Wickra.Darwin` — prebuilt native library, no system dependencies.**

.NET bindings for [`wickra-darwin`](https://github.com/wickra-lib/wickra-darwin)
over the C ABI hub, via source-generated P/Invoke. Build a `Darwin` from a spec
JSON, run a search and read back the report — the same protocol the CLI and every
other binding speak, returning the same bytes.

## Install

```bash
dotnet add package Wickra.Darwin
```

The native library ships prebuilt per platform under `runtimes/<rid>/native/`,
selected automatically. There is nothing to compile. Targets .NET 8 and later.

Requires .NET 8+. The native library (`wickra_darwin`) must be resolvable on the
loader path — `PATH` on Windows, `LD_LIBRARY_PATH` on Linux, `DYLD_LIBRARY_PATH`
on macOS — or beside the assembly, where the bundled resolver finds it.

## Quick start

```csharp
using Wickra.Darwin;

const string spec = """
{"seed":7,"population":64,"generations":20,"fitness":"sharpe",
 "search_space":{
   "indicators":[
     {"name":"rsi",  "param_ranges":[{"min":5,"max":30,"step":1}]},
     {"name":"macd", "param_ranges":[{"min":8,"max":16,"step":2},
                                     {"min":20,"max":30,"step":2},
                                     {"min":5,"max":12,"step":1}]}],
   "rules":"single_threshold"}}
""";

using var darwin = new Darwin(spec);
string report = darwin.Command("""{"cmd":"evolve","data":{ … }}""");
```

The search space is the indicator registry: any name `wickra-backtest` can
execute can also be searched, with the parameters that name actually takes —
`macd` wants three, `BollingerBands` two. A name the registry does not know is
refused at construction rather than silently replaced.

A search is reproducible from its seed alone: the same spec and the same data
give the same winner, in this binding exactly as in the other nine.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of `[LibraryImport]` P/Invoke over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-darwin/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-darwin>
- **Docs** (guides, spec reference, cookbook): <https://darwin.wickra.org>
- **Runnable example:** [`examples/csharp/`](https://github.com/wickra-lib/wickra-darwin/tree/main/examples/csharp)

Wickra Darwin ships native bindings for Python, Node.js, WASM and Rust, plus a C ABI hub that any
C-capable language (C, C++, C#, Go, Java, R) links against — all forwarding to the
same data-driven, `unsafe`-forbidden Rust core.

## Security

Found a security issue? **Please don't open a public issue.** Report it privately
via the repository's *Security* tab (*"Report a vulnerability"*) or email
**support@wickra.org** with a subject line starting `[wickra security]`. Full
policy: <https://github.com/wickra-lib/wickra-darwin/blob/main/SECURITY.md>.

## Disclaimer

Wickra Darwin is a research tool, provided "as is" without warranty of any kind.
Evolutionary search optimises a fitness objective over historical data — a strong
in-sample fitness is not evidence of out-of-sample performance, and overfitting is
the default outcome, not the exception. Nothing here is financial advice; any
strategy you deploy is your responsibility, and trading carries risk of loss.

## License

Licensed under either of [Apache-2.0](https://github.com/wickra-lib/wickra-darwin/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/wickra-lib/wickra-darwin/blob/main/LICENSE-MIT) at your option.

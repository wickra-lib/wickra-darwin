# Wickra Darwin — C#

.NET bindings for the Wickra evolutionary strategy search over its C ABI hub. A
`Darwin` is built from a spec JSON and driven over a JSON boundary, so the result
is byte-identical to every other Wickra Darwin binding.

## Install

```bash
dotnet add package Wickra.Darwin
```

The package ships the native C ABI library per runtime identifier under
`runtimes/<rid>/native/`. For a local build, `cargo build -p wickra-darwin-c --release`
places the library in `target/release/`; the bundled `DllImportResolver` probes
the Cargo `target/` tree, so tests and apps in the repo find it without extra
steps.

## Usage

```csharp
using Wickra.Darwin;

const string spec = """
{"seed":1,"population":8,"generations":3,
 "mutation_rate":0.2,"crossover_rate":0.6,"fitness":"sharpe",
 "search_space":{"indicators":[{"name":"rsi","param_ranges":[{"min":2,"max":30,"step":1}]}],
 "rules":"single_threshold","max_conditions":1},"elitism":1,"top":5}
""";

using var darwin = new Darwin(spec);
const string data = """{"BTCUSDT":[{"time":1700000000,"open":100,"high":101,"low":99,"close":100.5,"volume":10}]}""";
string report = darwin.Command($"{{\"cmd\":\"evolve\",\"data\":{data}}}");
Console.WriteLine(report);
```

## Surface

- **`new Darwin(specJson)`** — build a search handle (`"{}"` defers to a later
  `set_spec`). Throws `ArgumentException` on an invalid spec.
- **`Command(cmdJson)`** — apply a command envelope (`{"cmd":"...", ...}`) and
  return the response JSON. Commands: `set_spec`, `evolve`, `best`, `version`.
- **`Darwin.Version()`** — the library version.
- **`Dispose()`** — free the native handle (`using` recommended).

## Determinism

The search's PRNG lives only in the Rust core; this binding forwards the command
string verbatim, so a given seed produces the byte-identical report here and in
every other binding — the exact cross-language golden invariant.

## See also

- The main project: <https://github.com/wickra-lib/wickra-darwin>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either MIT or Apache-2.0, at your option.

# wickra-darwin (C#)

.NET bindings for [`wickra-darwin`](https://github.com/wickra-lib/wickra-darwin)
over the C ABI hub, via source-generated P/Invoke. Build a `Darwin` from a spec
JSON, run a search and read back the report — the same protocol the CLI and every
other binding speak, returning the same bytes.

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

Requires .NET 8+. The native library (`wickra_darwin`) must be resolvable on the
loader path — `PATH` on Windows, `LD_LIBRARY_PATH` on Linux, `DYLD_LIBRARY_PATH`
on macOS — or beside the assembly, where the bundled resolver finds it.

Licensed under either of [MIT](../../LICENSE-MIT) or
[Apache-2.0](../../LICENSE-APACHE) at your option.

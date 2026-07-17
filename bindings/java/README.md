# Wickra Darwin — Java

JVM bindings for the Wickra evolutionary strategy search over its C ABI hub,
using the Foreign Function & Memory API (FFM / Panama). A `Darwin` is built from
a spec JSON and driven over a JSON boundary, so the result is byte-identical to
every other Wickra Darwin binding.

## Requirements

- JDK 22+ (the FFM API is stable since Java 22). Run with
  `--enable-native-access=ALL-UNNAMED`.
- The native C ABI library, built by `cargo build -p wickra-darwin-c`.
  The binding loads it from the directory named by the `native.lib.dir` system
  property (the Maven build points it at the workspace `target/debug`).

## Usage

```java
import org.wickra.darwin.Darwin;

String spec = "{\"seed\":1,\"population\":8,\"generations\":3,"
    + "\"mutation_rate\":0.2,\"crossover_rate\":0.6,\"fitness\":\"sharpe\","
    + "\"search_space\":{\"indicators\":[{\"name\":\"rsi\",\"param_ranges\":"
    + "[{\"min\":2,\"max\":30,\"step\":1}]}],\"rules\":\"single_threshold\","
    + "\"max_conditions\":1},\"elitism\":1,\"top\":5}";

try (Darwin darwin = new Darwin(spec)) {
    String data = "{\"BTCUSDT\":[{\"time\":1700000000,\"open\":100,\"high\":101,"
        + "\"low\":99,\"close\":100.5,\"volume\":10}]}";
    String response = darwin.command("{\"cmd\":\"evolve\",\"data\":" + data + "}");
    System.out.println(response);
}
```

## Surface

- **`new Darwin(specJson)`** — build a search handle (`"{}"` defers to a later
  `set_spec`). Throws `IllegalArgumentException` on an invalid spec.
- **`command(cmdJson)`** — apply a command envelope (`{"cmd":"...", ...}`) and
  return the response JSON. Commands: `set_spec`, `evolve`, `best`, `version`.
- **`Darwin.version()`** — the library version.
- **`close()`** — free the native handle (try-with-resources recommended).

## Determinism

The search's PRNG lives only in the Rust core; this binding forwards the command
string verbatim, so a given seed produces the byte-identical report here and in
every other binding — the exact cross-language golden invariant.

## See also

- The main project: <https://github.com/wickra-lib/wickra-darwin>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either MIT or Apache-2.0, at your option.

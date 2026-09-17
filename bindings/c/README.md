<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Darwin — evolutionary strategy search at hundreds of thousands of backtests per second" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/ci.svg)](https://github.com/wickra-lib/wickra-darwin/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-darwin)
[![GitHub release](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/release.svg)](https://github.com/wickra-lib/wickra-darwin/releases/latest)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-darwin/license.svg)](https://github.com/wickra-lib/wickra-darwin#license)

# Wickra Darwin — C / C++

---

**Part of the [Wickra ecosystem](#ecosystem): — for C / C++. `cargo build -p wickra-darwin-c --release` — a prebuilt shared/static library plus a generated `wickra_darwin.h`, no system dependencies.**

The C ABI hub for Wickra Darwin. It builds as a `cdylib` and a `staticlib` and
exposes a tiny JSON-over-C surface that every C-capable language (C, C++, C#, Go,
Java, R) links against. The whole evolutionary search lives in the Rust core;
this layer only marshals JSON strings across the boundary, so a fixed seed yields
the byte-identical search in every language.

## Install

Grab the prebuilt header + library for your platform from the
[GitHub releases](https://github.com/wickra-lib/wickra-darwin/releases) — each archive
has `wickra_darwin.h`, the C++ wrapper where the binding ships one, and the shared/static
library — or build from source:

```bash
cargo build -p wickra-darwin-c --release
# -> target/release/libwickra_darwin.{so,dylib} or wickra_darwin.dll (+ import lib) + a staticlib
```

Then compile against the header and link the library.

### Building from this repository (contributors)

```bash
cargo build -p wickra-darwin-c --release
```

This produces `wickra_darwin.{dll,so,dylib}` (and a static library) under
`target/release/`. The header is committed at
[`include/wickra_darwin.h`](https://github.com/wickra-lib/wickra-darwin/blob/main/bindings/c/include/wickra_darwin.h) and regenerated with:

```bash
cbindgen --config cbindgen.toml --crate wickra-darwin-c --output include/wickra_darwin.h
```

## Quick start

[`examples/c/evolve.c`](https://github.com/wickra-lib/wickra-darwin/blob/main/examples/c/evolve.c) is the runnable example the CI smoke job executes; in full:

```c
/* A runnable C example: evolve strategy specs over a small deterministic
 * universe through the wickra-darwin C ABI and print the search summary. Every
 * language example builds the same universe and prints the same summary. */
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "wickra_darwin.h"

static const char *SPEC =
    "{\"seed\":7,\"population\":10,\"generations\":4,"
    "\"mutation_rate\":0.2,\"crossover_rate\":0.6,\"fitness\":\"sharpe\","
    "\"search_space\":{\"indicators\":[{\"name\":\"rsi\",\"param_ranges\":[{\"min\":2,\"max\":30}]}],"
    "\"rules\":\"single_threshold\",\"max_conditions\":1},\"elitism\":1,\"top\":3}";

/* Build the shared 16-bar universe as an evolve command JSON. */
static void build_command(char *out, size_t cap) {
    size_t n = 0;
    n += (size_t)snprintf(out + n, cap - n, "{\"cmd\":\"evolve\",\"data\":{\"SYM\":[");
    for (int i = 0; i < 16; i++) {
        double close = 100.0 + 8.0 * sin(i / 4.0) + 0.1 * i;
        double open = 100.0 + 8.0 * sin((i - 1) / 4.0) + 0.1 * (i - 1);
        double high = (close > open ? close : open) + 1.0;
        double low = (close < open ? close : open) - 1.0;
        n += (size_t)snprintf(out + n, cap - n, "%s{\"time\":%ld,\"open\":%.3f,\"high\":%.3f,\"low\":%.3f,\"close\":%.3f,\"volume\":1000}",
                              i > 0 ? "," : "", 1700000000L + (long)i * 3600, open, high, low, close);
    }
    snprintf(out + n, cap - n, "]}}");
}

int main(void) {
    WickraDarwin *darwin = wickra_darwin_new(SPEC);
    if (!darwin) {
        fprintf(stderr, "failed to build darwin\n");
        return 1;
    }
    char cmd[2048];
    build_command(cmd, sizeof(cmd));

    int len = wickra_darwin_command(darwin, cmd, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed: code %d\n", len);
        wickra_darwin_free(darwin);
        return 1;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    if (!buf) {
        wickra_darwin_free(darwin);
        return 1;
    }
    wickra_darwin_command(darwin, cmd, buf, (size_t)len + 1);

    printf("wickra-darwin %s\n", wickra_darwin_version());
    printf("report bytes: %d\n", len);

    free(buf);
    wickra_darwin_free(darwin);
    return 0;
}
```

### Surface

```c
typedef struct WickraDarwin WickraDarwin;

WickraDarwin *wickra_darwin_new(const char *spec_json);   /* NULL on an invalid spec */
void          wickra_darwin_free(WickraDarwin *handle);   /* NULL-safe */
int32_t       wickra_darwin_command(WickraDarwin *handle, const char *cmd_json,
                                    char *out, uintptr_t cap);
const char   *wickra_darwin_version(void);                /* static NUL string */
```

- `wickra_darwin_new` takes a spec JSON (`"{}"` defers configuration to a later
  `set_spec` command); it returns `NULL` on a null / non-UTF-8 / invalid spec.
- `wickra_darwin_command` applies a command envelope (`{"cmd":"...", ...}` —
  `set_spec`, `evolve`, `best`, `version`) and uses the classic two-call
  length-out protocol: call with `out = NULL`, `cap = 0` to learn the response
  length, then allocate `len + 1` and call again. A negative return is an
  unusable argument (`-1` null, `-2` non-UTF-8) or a caught panic (`-3`); a
  non-negative return is the response length. Domain errors come back **in-band**
  as `{"ok":false,"error":...}` JSON.
- `wickra_darwin_version` returns a static version string (do not free).

### Determinism

The search's PRNG lives only in the Rust core; this binding forwards the command
string verbatim, so an `evolve` with a fixed seed produces the byte-identical
report here and in every other Wickra Darwin binding.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of the C ABI itself, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-darwin/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-darwin>
- **Docs** (guides, spec reference, cookbook): <https://darwin.wickra.org>
- **Runnable example:** [`examples/c/`](https://github.com/wickra-lib/wickra-darwin/tree/main/examples/c)

- The main project: <https://github.com/wickra-lib/wickra-darwin>
- Documentation: <https://wickra.org>

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

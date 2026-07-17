# Wickra Darwin — C ABI

The C ABI hub for Wickra Darwin. It builds as a `cdylib` and a `staticlib` and
exposes a tiny JSON-over-C surface that every C-capable language (C, C++, C#, Go,
Java, R) links against. The whole evolutionary search lives in the Rust core;
this layer only marshals JSON strings across the boundary, so a fixed seed yields
the byte-identical search in every language.

## Build

```bash
cargo build -p wickra-darwin-c --release
```

This produces `wickra_darwin.{dll,so,dylib}` (and a static library) under
`target/release/`. The header is committed at
[`include/wickra_darwin.h`](include/wickra_darwin.h) and regenerated with:

```bash
cbindgen --config cbindgen.toml --crate wickra-darwin-c --output include/wickra_darwin.h
```

## Surface

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

## Determinism

The search's PRNG lives only in the Rust core; this binding forwards the command
string verbatim, so an `evolve` with a fixed seed produces the byte-identical
report here and in every other Wickra Darwin binding.

## See also

- The main project: <https://github.com/wickra-lib/wickra-darwin>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either [MIT](../../LICENSE-MIT) or
[Apache-2.0](../../LICENSE-APACHE), at your option.

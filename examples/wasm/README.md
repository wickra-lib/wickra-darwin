# wickra-darwin WASM examples

Browser demos for the `wickra-darwin-wasm` binding.

The WASM build carries the whole evolution loop with `--no-default-features`: no
rayon, so candidates are evaluated sequentially rather than in parallel — and the
result is byte-for-byte identical, which is what the golden searches pin down. A
spec is data, not code, so the bytes on this page are the same ones
`examples/node/evolve.js` sends, and the winner is the same winner.

## Build

The module ships as a `wasm-pack` `--target web` bundle. Build it once from the
repository root:

```bash
wasm-pack build bindings/wasm --target web --release
```

That writes `bindings/wasm/pkg/` with the `.wasm` binary, the JS loader and the
type declarations the page imports.

## Run

The page loads its module over `http://`, not `file://`, because ES module
imports and `WebAssembly.instantiateStreaming` both need a real origin. Serve the
repository root:

```bash
python -m http.server 8000
```

Then open `http://localhost:8000/examples/wasm/evolve.html`.

## Pages

| Page | What it does |
|------|--------------|
| `evolve.html` | Evolves a small population over a deterministic universe and shows the best fitness per generation, plus the raw report. The search space names `macd` alongside `rsi` — three parameters against one — which is what a registry-resolved space allows and a four-name allowlist did not. The page counterpart of `examples/node/evolve.js`. |

## See also

- [examples/README.md](../README.md) — the same search in every other language.
- [bindings/wasm/README.md](../../bindings/wasm/README.md) — the binding itself.

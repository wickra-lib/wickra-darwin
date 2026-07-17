# Wickra Darwin — Go

Go bindings for the Wickra evolutionary strategy search over its C ABI hub via
cgo. A `Darwin` is built from a spec JSON and driven over a JSON boundary, so the
result is byte-identical to every other Wickra Darwin binding.

## Install

```bash
go get github.com/wickra-lib/wickra-darwin/bindings/go
```

The prebuilt C ABI library is staged per platform under `lib/<goos>_<goarch>/`
and the header is vendored under `include/`. For a local build, copy the library
built by `cargo build -p wickra-darwin-c --release` into the matching
`lib/<goos>_<goarch>/` directory (on Windows, ensure that directory is on `PATH`
when running tests).

## Usage

```go
package main

import (
	"fmt"

	wickra "github.com/wickra-lib/wickra-darwin/bindings/go"
)

func main() {
	spec := `{"seed":1,"population":8,"generations":3,` +
		`"mutation_rate":0.2,"crossover_rate":0.6,"fitness":"sharpe",` +
		`"search_space":{"indicators":[{"name":"rsi","param_ranges":[{"min":2,"max":30,"step":1}]}],` +
		`"rules":"single_threshold","max_conditions":1},"elitism":1,"top":5}`

	darwin, err := wickra.New(spec)
	if err != nil {
		panic(err)
	}
	defer darwin.Close()

	data := `{"BTCUSDT":[{"time":1700000000,"open":100,"high":101,"low":99,"close":100.5,"volume":10}]}`
	resp, err := darwin.Command(`{"cmd":"evolve","data":` + data + `}`)
	if err != nil {
		panic(err)
	}
	fmt.Println(resp)
}
```

## Surface

- **`New(specJSON)`** — build a search handle (`"{}"` defers to a later
  `set_spec`). Returns an error on an invalid spec.
- **`(*Darwin).Command(cmdJSON)`** — apply a command envelope
  (`{"cmd":"...", ...}`) and return the response JSON. Commands: `set_spec`,
  `evolve`, `best`, `version`.
- **`(*Darwin).Close()`** — free the handle (a finalizer also frees it).
- **`Version()`** — the library version.

## Determinism

The search's PRNG lives only in the Rust core; this binding forwards the command
string verbatim, so a given seed produces the byte-identical report here and in
every other binding — the exact cross-language golden invariant.

## See also

- The main project: <https://github.com/wickra-lib/wickra-darwin>
- Documentation: <https://wickra.org>

## License

Dual-licensed under either [MIT](../../LICENSE-MIT) or
[Apache-2.0](../../LICENSE-APACHE), at your option.

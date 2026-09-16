# Wickra Darwin — C / C++ examples

The Wickra Darwin C ABI is a single shared/static library plus a generated header
([`bindings/c/include/wickra_darwin.h`](../../bindings/c/include/wickra_darwin.h)). Any C-capable
language links against the same artifact; these examples show the plain-C path
and, through [`wickra_darwin.hpp`](../../bindings/c/include/wickra_darwin.hpp), the C++ one.

## Build the library

From the workspace root:

```sh
cargo build -p wickra-darwin-c --release
```

This produces, in `target/release/`:

| Platform | Shared library | Link target |
|----------|----------------|-------------|
| Linux    | `libwickra_darwin.so`     | `-lwickra_darwin` |
| macOS    | `libwickra_darwin.dylib`  | `-lwickra_darwin` |
| Windows (MSVC) | `wickra_darwin.dll` | `wickra_darwin.dll.lib` (import lib) |

A static library (`libwickra_darwin.a` / `wickra_darwin.lib`) is emitted alongside.

## Build and run the examples

With CMake, as the CI C ABI job does:

```sh
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

## The examples

| Example | What it does |
|---------|--------------|
| `evolve.c` | A runnable C example: evolve strategy specs over a small deterministic |
| `evolve.cpp` | A runnable C++ example: evolve strategy specs over a small deterministic universe and print the search summary. |

## Usage shape

Every call follows the same handle discipline: construct from a spec JSON, drive
with command JSON, read the response, free the handle exactly once. `wickra_darwin.h` is
the whole contract; the C++ header, where one ships, wraps the handle in a
move-only RAII type. See [`bindings/c/README.md`](../../bindings/c/README.md).

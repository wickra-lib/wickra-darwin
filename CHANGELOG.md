# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- **`cargo-deny` was set to warn about duplicated crates, so it noted that split
  and moved on.** It is an error now. Only four duplicates exist across this
  workspace and each is a crate part-way through a major release reached through
  two ecosystems; they are skipped by name with the reason recorded, so a fifth
  still fails. Verified by putting 6.1.3 back and watching the check fail on
  `convert_case` before the compiler ever ran.

- **`actionlint` failed on five shell constructs the screener had already
  fixed.** `a && b || c` is not if-then-else -- when the publish succeeded but
  the echo failed, the fallback branch ran and reported "already published";
  `local pkg=$(basename …)` and `export PATH="$(cygpath …)"` hide the command's
  exit status behind `local`/`export`; and an asset count taken from `ls` breaks
  on a filename containing a newline. The runner-label config the linter needs
  for `windows-11-arm` was missing too.

- **The search never evaluated a single candidate.** `to_strategy_spec` wrote
  operands as `{"ref": "rsi"}` and `{"const": 50.46}`, but
  `wickra_backtest::Operand` is an untagged enum whose `Ref` variant is a bare
  string and whose `Const` variant is a bare number. Every candidate failed to
  deserialise, `evaluate` skipped it with `continue`, and a candidate that
  scores nothing becomes `NEG_INFINITY` -- which is not finite, so it is
  filtered out of both the generation statistics and the hall of fame.

- **The failure was silent, and the corpus had blessed it.** A run in which
  nothing deserialised still returns a well-formed report: `evaluated: 0`,
  `best: 0.0`, and an empty `best` list. Every binding printed `hall of fame: 0`
  without complaint, and all five golden expectations recorded that. The two
  tests that should have caught it could not: one walks `report.best` to check
  each spec parses and was iterating an empty list, the other compared a history
  of zeros against itself. Three tests now pin the property directly -- every
  generation scores its whole population, the hall of fame fills to the spec's
  `top`, and a sampled genome round-trips into `StrategySpec`.

- **The throughput table timed a search that ran no backtests.** With every
  candidate failing to parse, the loop never reached the engine, so a table
  headed "backtests per second" was measuring sampling and a failed
  deserialisation. Re-measured after the fix: **~122 K-448 K backtests/second**,
  up from the ~110 K-285 K reported before. The work count was also off by one
  generation -- the loop scores the initial population before it breeds, so the
  total is `population x (generations + 1) x symbols`.

- **Three steps of the `examples` job ran a file that is not here.** Python,
  Node.js and R each invoked `examples/<lang>/scan.*` -- the screener's file
  name, left over from the port -- so they died on a missing file before
  reaching any assertion.

- **Every language step asserted `"symbol":"BBB"`**, a line from the screener's
  scan report that no example here prints. Each step now matches a string its
  own example emits, read off the format string rather than guessed: the
  assertions were checked against a real run of each example.

- **The Rust example's lockfile pinned the pre-migration engine.** It still held
  `wickra-core` 0.9.9 and `wickra-backtest` 0.1.0 while the workspace declares
  1.0 and 0.1.4, so cargo silently repaired the lock on every build and the
  example was the one reach measured against a different engine than the rest.

- **The C++ example now goes through the C++ hull.** It called the C functions
  directly and rebuilt the two-call length protocol by hand -- the very thing
  `wickra_darwin.hpp` exists to remove -- which left the shipped C++ surface
  built by nothing. Verified by running both: the C and C++ examples print
  byte-identical output.

- **Every C++ hull used the include guard `WICKRA_SCREENER_HPP`.** The C headers
  beside them are guarded correctly; only the `.hpp` files shared one name, so
  including two of the family's headers in the same translation unit dropped the
  second silently. Proven by compiling a file that includes two of them and
  names a class from each: `'Env' is not a member of 'wickra'`. All seven now
  compile standalone and together.

- **The hull's usage example could not run.** It showed a spec shaped
  `{"universe":[...]}` and `{"cmd":"scan"}`, the screener's, which this core
  rejects twice over. It now shows this repository's own spec fields and one of
  its own commands.

- **The release notes named the wrong package.** They told a reader
  `install.packages("wickrafeaturestore")` from r-universe, where this package
  is `wickradarwin`. These notes go out with the GitHub release: a reader
  following them installs a different library.

- **The issue and pull-request templates asked for a `FeatureSpec`**, a type
  this repository does not have, so a contributor was asked to attach something
  that does not exist. `GOVERNANCE.md`, `SUPPORT.md` and `CONTRIBUTING.md`
  carried the same substitution, along with the screener's "condition schema"
  for a core that has no conditions.

- **The R `configure` scripts still defined `wkscreen_download`**, the last
  trace of the screener's prefix — the CI-visible half of which already had to
  be fixed once.

- **Six SHA-pinned actions sat on two lines across the family**, and two of the
  splits were inside this repository. `actions/setup-node` is pinned at the same
  commit everywhere, but some call sites annotated it `# v6.4.0`; GitHub's tag
  list says that commit is **v7.0.0** and v6.4.0 is a different one. Dependabot
  reads that comment to decide what to bump, so a wrong one misdirects the tool
  meant to keep the pin current. `Swatinem/rust-cache` ran at two commits at
  once, the older behind a floating `# v2`. Every pin now matches what the
  sibling repositories run, each target checked against the upstream tag list.

- **The CI Java example step compiled a file that is not there.** The `examples`
  job was ported from the screener, whose Java example is a single
  `examples/java/Scan.java` built with `javac`. This repository ships a Maven
  project instead, so the step compiled a missing file and then asserted on
  output the example never prints. It now builds the binding into the local
  repository and runs the example through `mvn exec:exec`, the way the example's
  own javadoc documents -- verified by running it.

- **The `examples` job installed a lockfile that is not here.** It names
  `.github/requirements/ci-dev-py3.txt`, and so does `scripts/update-lockfiles.sh`,
  but the directory held a single `ci-dev.txt` that nothing referenced. The split
  is not cosmetic: the Python matrix includes 3.9, and that single lock pinned
  `pytest==9.1.1` and `iniconfig==2.3.0`, both of which declare
  requires-python >= 3.10.

- **The `python` job installed unpinned.** `pip install maturin pytest` is a
  fetch of whatever the index serves that minute -- the exact thing the locked
  file exists to prevent. It now installs the hash-locked row for its
  interpreter, and the advisory the 3.9 pin sits inside is recorded with its
  reason in `osv-scanner.toml`.

- **Dependabot watched directories that do not exist**, so it reported nothing
  and the silence read as calm. `nuget` pointed at `WickraCompile.Tests`, a
  project name from another repository; `pip` did not cover
  `/.github/requirements` and `npm` did not cover `/examples/node`.

- **The workspace's own core was pinned as a range.** `darwin-core` was named
  six times as `version = "0.1"` -- a caret range -- and the root manifest
  carried no `[workspace.dependencies]` entry for it at all. A published
  `darwin-cli` 0.1.0 would have accepted `darwin-core` 0.1.99, a crate resolving
  against a core it was never built against, in a workspace whose whole point is
  that the pieces move together. It also hid the line from `bump_version.py` and
  `check_version_sync.py`, both of which look for the exact version.

- **`release.yml` overwrote the binding READMEs before packing.** Three steps
  copied the root README over `bindings/python/README.md` (wheel and sdist) and
  `bindings/node/README.md`. They date from when the bindings had no README of
  their own; they do now, one per registry, and `check_readme_links.py` exists to
  keep their links absolute because a relative link is dead on PyPI and npm. The
  copy threw that away and shipped the root README, whose links are relative by
  design. The remaining relative links in the C, C#, Go and WASM READMEs are
  absolute now.

- **The Python wheel would have shipped without its licence texts.**
  `bindings/python/` carried neither `LICENSE-MIT` nor `LICENSE-APACHE`, so
  maturin had nothing to include, while every crate and the release archive
  carry both.

- **`SECURITY.md` named a support policy for releases that do not exist yet.**
  It promised fixes for "the latest `0.x` release line" where there is no
  released line; it now says plainly that nothing is published and names `0.1.0`
  as the first version that will be.

- **`CITATION.cff` described the wrong project.** The abstract and the keyword
  list were the feature store's, describing a feature matrix for an evolutionary
  search. `CITATION.cff` is what GitHub's citation box and Zenodo quote back at
  a reader as the project's own words, so it is the one file where a wrong
  description is the project saying it.

- **The Ecosystem section repeated two claims their own repositories had already
  corrected**: DARWIN at "millions of backtests per second" across "the
  514-indicator space", where its benchmark says hundreds of thousands over the
  registry, and GENOME as "a 514-dim live vector", where the dimension is
  whatever the spec's feature list names.

- **The search space was four indicators wide while the README claimed 514.**
  `indicator_kind` was an allowlist of `sma`, `ema`, `rsi` and `atr`, and every
  other name was rejected. It also declared each of them as taking one
  parameter, which is true of those four and false in general — `Macd` takes
  three, `BollingerBands` two — so a multi-parameter indicator was unreachable
  twice over.

- **The throughput claim contradicted this repository's own benchmark.** The
  README, the banner alt text and `ARCHITECTURE.md` all said "millions of
  backtests per second"; `BENCHMARKS.md` reports **~110 K–285 K**. The claim is
  now the measured range, and `ARCHITECTURE.md` cites the file rather than
  restating a number that can drift from it.

- **A sampler fallback substituted `Sma` for an unresolvable name.** Unreachable,
  because `validate` resolves every name first — and the worst possible answer
  if it ever were reached: searching a different indicator than the spec asked
  for, silently.

- **CONTRIBUTING.md described a repository that is not this one** — feature
  kinds, label kinds, `docs/FEATURES.md`, `docs/LABELS.md`, all of it
  wickra-feature-store's. This is what has been failing the link check on
  `main`.

- **`CMAKE_CXX_STANDARD` asked for C++14** while the C++ hull requires C++17.
  Nothing compiled it, so nothing found out.

### Added

- **Registry-resolved search space.** Any name the engine can execute can be
  searched, with the arity the registry reports, so the space DARWIN samples
  from and the space the engine can run are the same set by construction rather
  than by two lists happening to agree.

- **Batch-search tests in every binding.** Python, Node, Go, Java, C#, R, WASM
  and C each check that the same seed reproduces the same search through that
  boundary, that a three-parameter indicator is searchable at all, and that an
  unknown name is refused.

- **A golden test for the C binding**, which had none: all five committed specs,
  byte-identical, plus `golden/data.json` so any binding can load the corpus
  without a CSV parser of its own.

- The blueprint scaffold: `LICENSES/`, `docs/README.md`, the five long-form
  issue templates, the CodeQL config, the actionlint and CodSpeed workflows, the
  five check scripts, a C++ hull, licence copies in every published crate and
  npm package, and a WASM example.

- CI gains `osv`, `links`, `binding-surface`, `semver`, `fuzz-smoke`,
  `examples` and `python-wheel-container-smoke`; the release pipeline gains the
  `gate` and `guard` jobs, provenance over the nupkg, jar and C ABI archives, a
  Maven artifact on the release page, and a Go mirror that builds before it
  publishes.

### Changed

- **The family pins move to the published releases.** `wickra-backtest` comes
  from crates.io at 0.1.4 rather than a git rev, and `wickra-core` rises from
  0.9 to 1.0, so the tree carries one set of indicator types rather than two
  that share none.

### Added

- Repository scaffold: governance, supply-chain configuration (`deny.toml`,
  `lychee.toml`, `osv-scanner.toml`, `repo-metadata.toml`), the Rust workspace
  (`darwin-core`, `darwin-cli`, `darwin-bench`) with the language-binding crates,
  and the `wickra-backtest` git dependency (the O(1) engine DARWIN evolves
  strategies against).
- `darwin-core`: the evolutionary search — `EvolveSpec`, `SearchSpace` /
  `RuleGrammar` / `IndicatorGene`, the `StrategySpec` genome with seeded
  `SplitMix64` crossover and mutation, `Fitness` (Sharpe / PnL / Calmar), and the
  `evolve` loop returning a deterministic `EvolveReport` (hall of fame + history).
- `wickra-darwin` CLI over the core (`--spec`, `--data`, `--stdin`, `--format`,
  `--seed`, `--top`).
- Ten language bindings (Rust, Python, Node.js, WASM natively; C, C++, C#, Go,
  Java, R over a C ABI hub), each forwarding `command_json` verbatim for a
  byte-identical search.
- Golden corpus (fixed-seed specs + blessed reports) and the test suite
  (conformance, golden replay, determinism equivalence, proptest invariants),
  fuzz targets and the criterion benchmark crate.
- The full CI/CD matrix (fmt, clippy, tests on 3 OS × 2 feature sets, MSRV,
  coverage, cargo-deny, the ten-language jobs, CodeQL, Scorecard, zizmor, link
  and metadata checks) plus a USER-GO-gated release pipeline.
- Documentation: `README`, per-binding READMEs, and `docs/` (architecture,
  evolution, genome, fitness, determinism, cookbook); `BENCHMARKS.md` with
  measured backtests-per-second figures.

[Unreleased]: https://github.com/wickra-lib/wickra-darwin/commits/main

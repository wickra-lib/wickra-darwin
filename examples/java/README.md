# Wickra Darwin examples — Java

Runnable Java examples for the [Wickra Darwin Java binding](../../bindings/java). The binding reaches the C ABI
through the Foreign Function & Memory API (JDK 22+), so build the library once
and point the JVM at it with `-Dnative.lib.dir`:

```bash
cargo build -p wickra-darwin-c --release
```

## Run

As the CI examples job runs it, from the repository root:

```bash
mvn -f bindings/java/pom.xml -q install -DskipTests
mvn -f examples/java/pom.xml -q compile exec:exec  -Dnative.lib.dir="$PWD/target/release"
```

## The examples

| Example | What it does |
|---------|--------------|
| `src/main/java/org/wickra/darwin/examples/Evolve.java` | A runnable example against this binding. |

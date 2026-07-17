//! Wickra Darwin — wasm binding. Scaffold; the real surface lands in P-DAR-3.

/// The crate version, forwarded from the core.
#[must_use]
pub fn version() -> &'static str {
    darwin_core::version()
}

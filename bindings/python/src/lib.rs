//! Python bindings for `wickra-darwin`, exposed under the `wickra_darwin`
//! package.
//!
//! Thin glue over the evolutionary core's command surface: construct a
//! [`Darwin`] from a spec JSON, drive it with a command JSON and read back the
//! response JSON. The same command protocol crosses every binding, so a Python
//! front-end drives the exact same core — and gets the byte-identical search —
//! as the CLI.

// PyO3 protocol methods take `self` by ref regardless of use.
#![allow(clippy::needless_pass_by_value)]

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;

use darwin_core::Darwin;

/// An evolutionary search driven by JSON commands.
///
/// `unsendable`: the handle caches the last report, so it is bound to the thread
/// that created it.
#[pyclass(name = "Darwin", unsendable)]
struct PyDarwin {
    inner: Darwin,
}

#[pymethods]
impl PyDarwin {
    /// Construct a search handle from a spec JSON (`"{}"` defers configuration
    /// to a later `set_spec` command).
    #[new]
    fn new(spec_json: &str) -> PyResult<Self> {
        Darwin::new(spec_json)
            .map(|inner| Self { inner })
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Apply a command JSON and return the response JSON.
    fn command(&mut self, cmd_json: &str) -> PyResult<String> {
        self.inner
            .command_json(cmd_json)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }

    /// The library version.
    #[staticmethod]
    fn version() -> &'static str {
        darwin_core::version()
    }
}

/// The native module (`wickra_darwin._wickra_darwin`).
#[pymodule]
fn _wickra_darwin(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add_class::<PyDarwin>()?;
    Ok(())
}

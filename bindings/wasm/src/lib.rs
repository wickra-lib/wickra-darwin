//! WebAssembly bindings for `wickra-darwin` (wasm-bindgen).
//!
//! Evolve strategy specs in the browser: create a `Darwin` from a spec JSON,
//! drive it with a command JSON (`set_spec`, `evolve`, `best`, `version`) and
//! read back the response JSON. The same command protocol crosses every
//! binding, so a browser front-end runs against the exact same core as the
//! native CLI.
//!
//! The search runs single-threaded here (no rayon thread pool in a browser
//! sandbox), which is byte-identical to the native run — the exact
//! cross-language golden check.

use wasm_bindgen::prelude::*;

use darwin_core::Darwin as CoreDarwin;

/// An evolutionary search driven by JSON commands.
#[wasm_bindgen]
pub struct Darwin {
    inner: CoreDarwin,
}

#[wasm_bindgen]
impl Darwin {
    /// Construct a search handle from a spec JSON (`"{}"` defers configuration
    /// to a later `set_spec` command).
    #[wasm_bindgen(constructor)]
    pub fn new(spec_json: &str) -> Result<Darwin, JsError> {
        CoreDarwin::new(spec_json)
            .map(|inner| Self { inner })
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Apply a command JSON (`{"cmd":"...", ...}`) and return the response JSON.
    pub fn command(&mut self, cmd_json: &str) -> Result<String, JsError> {
        self.inner
            .command_json(cmd_json)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// The library version.
    #[wasm_bindgen(js_name = version)]
    pub fn instance_version(&self) -> String {
        darwin_core::version().to_string()
    }
}

/// The library version.
#[wasm_bindgen]
pub fn version() -> String {
    darwin_core::version().to_string()
}

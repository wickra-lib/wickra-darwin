//! The `Darwin` handle and the `command_json` FFI boundary.

use std::collections::BTreeMap;

use serde_json::{json, Value};
use wickra_backtest::Candle;

use crate::error::{Error, Result};
use crate::evolve::{evolve, EvolveReport, RankedStrategy};
use crate::spec::EvolveSpec;

/// A stateful search handle: an optional configured spec and the last report.
#[derive(Default)]
pub struct Darwin {
    spec: Option<EvolveSpec>,
    last: Option<EvolveReport>,
}

impl Darwin {
    /// Construct from a spec JSON. `"{}"` (or empty) defers configuration to a
    /// later `set_spec` command.
    ///
    /// # Errors
    /// Returns a parse or validation error for a non-empty, invalid spec.
    pub fn new(spec_json: &str) -> Result<Self> {
        let trimmed = spec_json.trim();
        let spec = if trimmed.is_empty() || trimmed == "{}" {
            None
        } else {
            Some(EvolveSpec::from_json(spec_json)?)
        };
        Ok(Self { spec, last: None })
    }

    /// The crate version.
    #[must_use]
    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// Set the active spec.
    pub fn set_spec(&mut self, spec: EvolveSpec) {
        self.spec = Some(spec);
    }

    /// The top-`n` strategies of the last report.
    ///
    /// # Errors
    /// Returns [`Error::BadSpec`] if no `evolve` has run yet.
    pub fn best(&self, n: usize) -> Result<Vec<RankedStrategy>> {
        let report = self
            .last
            .as_ref()
            .ok_or_else(|| Error::BadSpec("no evolve run yet".into()))?;
        Ok(report.best.iter().take(n).cloned().collect())
    }

    /// Dispatch a command envelope and return the response JSON string.
    ///
    /// # Errors
    /// Returns an error for a malformed envelope, an unknown command, a missing
    /// spec, or an evolution failure. Bindings render the error in-band as
    /// `{"ok":false,"error":...}`.
    pub fn command_json(&mut self, cmd_json: &str) -> Result<String> {
        let envelope: Value =
            serde_json::from_str(cmd_json).map_err(|e| Error::Parse(e.to_string()))?;
        let cmd = envelope
            .get("cmd")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::BadSpec("missing cmd".into()))?;
        match cmd {
            "set_spec" => {
                let spec_val = envelope
                    .get("spec")
                    .ok_or_else(|| Error::BadSpec("set_spec requires a spec".into()))?;
                let spec: EvolveSpec = serde_json::from_value(spec_val.clone())
                    .map_err(|e| Error::Parse(e.to_string()))?;
                spec.validate()?;
                self.spec = Some(spec);
                Ok(json!({ "ok": true }).to_string())
            }
            "evolve" => {
                let spec = self
                    .spec
                    .clone()
                    .ok_or_else(|| Error::BadSpec("no spec set".into()))?;
                let data_val = envelope
                    .get("data")
                    .ok_or_else(|| Error::BadSpec("evolve requires data".into()))?;
                let data: BTreeMap<String, Vec<Candle>> = serde_json::from_value(data_val.clone())
                    .map_err(|e| Error::Data(e.to_string()))?;
                let report = evolve(&data, &spec)?;
                let out =
                    serde_json::to_string(&report).map_err(|e| Error::Parse(e.to_string()))?;
                self.last = Some(report);
                Ok(out)
            }
            "best" => {
                let n = envelope
                    .get("n")
                    .and_then(Value::as_u64)
                    .map_or(usize::MAX, |x| x as usize);
                let best = self.best(n)?;
                Ok(json!({ "best": best }).to_string())
            }
            "version" => Ok(json!({ "version": Self::version() }).to_string()),
            other => Err(Error::BadSpec(format!("unknown cmd: {other}"))),
        }
    }
}

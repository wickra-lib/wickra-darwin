//! A file-loadable configuration wrapper for the CLI.

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::spec::EvolveSpec;

/// A config file: just the evolution spec.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    /// The evolution spec.
    pub spec: EvolveSpec,
}

impl Config {
    /// Parse from JSON.
    ///
    /// # Errors
    /// Returns [`Error::Parse`] on malformed JSON.
    pub fn from_json(s: &str) -> Result<Self> {
        serde_json::from_str(s).map_err(|e| Error::Parse(e.to_string()))
    }

    /// Parse from TOML.
    ///
    /// # Errors
    /// Returns [`Error::Parse`] on malformed TOML.
    pub fn from_toml(s: &str) -> Result<Self> {
        toml::from_str(s).map_err(|e| Error::Parse(e.to_string()))
    }
}

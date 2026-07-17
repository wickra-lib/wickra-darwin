//! The evolution specification — the search configuration that drives a run.

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::fitness::Fitness;
use crate::search_space::SearchSpace;

/// A complete evolution configuration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EvolveSpec {
    /// The RNG seed — fixes the entire run byte-for-byte.
    pub seed: u64,
    /// Number of candidates per generation.
    pub population: usize,
    /// Number of generations to evolve.
    pub generations: usize,
    /// Per-locus mutation probability in `[0, 1]`.
    pub mutation_rate: f64,
    /// Probability of crossover (vs cloning) in `[0, 1]`.
    pub crossover_rate: f64,
    /// The maximised objective.
    pub fitness: Fitness,
    /// The bounded strategy search space.
    pub search_space: SearchSpace,
    /// Number of top individuals carried unchanged into the next generation.
    #[serde(default)]
    pub elitism: usize,
    /// Number of best strategies reported.
    #[serde(default = "default_top")]
    pub top: usize,
}

fn default_top() -> usize {
    10
}

impl EvolveSpec {
    /// Parse a spec from JSON.
    ///
    /// # Errors
    /// Returns [`Error::Parse`] on malformed JSON, or a validation error.
    pub fn from_json(s: &str) -> Result<Self> {
        let spec: Self = serde_json::from_str(s).map_err(|e| Error::Parse(e.to_string()))?;
        spec.validate()?;
        Ok(spec)
    }

    /// Parse a spec from TOML.
    ///
    /// # Errors
    /// Returns [`Error::Parse`] on malformed TOML, or a validation error.
    pub fn from_toml(s: &str) -> Result<Self> {
        let spec: Self = toml::from_str(s).map_err(|e| Error::Parse(e.to_string()))?;
        spec.validate()?;
        Ok(spec)
    }

    /// Validate every bound.
    ///
    /// # Errors
    /// Returns [`Error::BadSpec`] (or a search-space error) on violation.
    pub fn validate(&self) -> Result<()> {
        if self.population < 1 {
            return Err(Error::BadSpec("population must be >= 1".into()));
        }
        if self.generations < 1 {
            return Err(Error::BadSpec("generations must be >= 1".into()));
        }
        if !(0.0..=1.0).contains(&self.mutation_rate) {
            return Err(Error::BadSpec("mutation_rate must be in [0, 1]".into()));
        }
        if !(0.0..=1.0).contains(&self.crossover_rate) {
            return Err(Error::BadSpec("crossover_rate must be in [0, 1]".into()));
        }
        if self.elitism > self.population {
            return Err(Error::BadSpec("elitism must be <= population".into()));
        }
        if self.top < 1 {
            return Err(Error::BadSpec("top must be >= 1".into()));
        }
        self.search_space.validate()
    }
}

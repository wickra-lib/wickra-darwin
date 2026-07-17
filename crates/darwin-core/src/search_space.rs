//! The search space and the rule grammar: how genomes are sampled from and
//! mutated within a bounded space of valid strategies.

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::genome::{
    round8, sample_cmp, sample_params, Gene, Genome, IndicatorGene, ParamRange, Rule, Term,
};
use crate::rng::SplitMix64;

/// How a rule (entry/exit condition) is built from the genes.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuleGrammar {
    /// One `gene <cmp> const` threshold.
    SingleThreshold,
    /// `geneA` crosses above/below `geneB`.
    CrossoverPair,
    /// An AND of up to `max_conditions` thresholds.
    ConjunctionAll,
}

/// The bounded space DARWIN searches.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SearchSpace {
    /// The indicators available to rules.
    pub indicators: Vec<IndicatorGene>,
    /// The rule grammar.
    pub rules: RuleGrammar,
    /// The conjunction-length cap (bloat brake).
    #[serde(default = "default_max_conditions")]
    pub max_conditions: usize,
}

fn default_max_conditions() -> usize {
    3
}

/// Map a gene name to its `wickra-core` indicator type and parameter arity.
/// The allowlist is a curated set of scalar, single-parameter indicators; an
/// unknown name is rejected rather than passed blindly to the engine.
#[must_use]
pub fn indicator_kind(name: &str) -> Option<(&'static str, usize)> {
    match name {
        "sma" => Some(("Sma", 1)),
        "ema" => Some(("Ema", 1)),
        "rsi" => Some(("Rsi", 1)),
        "atr" => Some(("Atr", 1)),
        _ => None,
    }
}

impl SearchSpace {
    /// Validate the space: at least one indicator, every indicator known with a
    /// matching arity, and `max_conditions >= 1`.
    ///
    /// # Errors
    /// Returns [`Error::BadSpec`] or [`Error::UnknownIndicator`] on violation.
    pub fn validate(&self) -> Result<()> {
        if self.indicators.is_empty() {
            return Err(Error::BadSpec("search_space.indicators is empty".into()));
        }
        if self.max_conditions < 1 {
            return Err(Error::BadSpec("max_conditions must be >= 1".into()));
        }
        for ind in &self.indicators {
            let Some((_, arity)) = indicator_kind(&ind.name) else {
                return Err(Error::UnknownIndicator(ind.name.clone()));
            };
            if ind.param_ranges.len() != arity {
                return Err(Error::BadSpec(format!(
                    "indicator {} expects {arity} param range(s), got {}",
                    ind.name,
                    ind.param_ranges.len()
                )));
            }
            for r in &ind.param_ranges {
                if r.step <= 0.0 || r.max < r.min {
                    return Err(Error::BadSpec(format!(
                        "indicator {} has an invalid param range",
                        ind.name
                    )));
                }
            }
        }
        Ok(())
    }
}

fn sample_konst(rng: &mut SplitMix64) -> f64 {
    // A threshold in [0, 100): valid for oscillators (RSI) and harmless — the
    // result is always a runnable spec, which is what the search requires.
    round8(rng.next_f64() * 100.0)
}

fn sample_term(rng: &mut SplitMix64, n_genes: usize) -> Term {
    Term {
        gene: rng.below(n_genes as u64) as usize,
        cmp: sample_cmp(rng),
        konst: sample_konst(rng),
    }
}

/// Sample one rule from the grammar. Draw order is fixed and load-bearing for
/// the cross-language golden.
fn sample_rule(rng: &mut SplitMix64, sp: &SearchSpace, n_genes: usize) -> Rule {
    match sp.rules {
        RuleGrammar::SingleThreshold => Rule::Single(sample_term(rng, n_genes)),
        RuleGrammar::CrossoverPair => {
            if n_genes >= 2 {
                let a = rng.below(n_genes as u64) as usize;
                let mut b = rng.below(n_genes as u64) as usize;
                if a == b {
                    b = (b + 1) % n_genes;
                }
                let above = rng.next_f64() < 0.5;
                Rule::Cross { a, b, above }
            } else {
                Rule::Single(sample_term(rng, n_genes))
            }
        }
        RuleGrammar::ConjunctionAll => {
            let k = 1 + rng.below(sp.max_conditions as u64) as usize;
            let terms = (0..k).map(|_| sample_term(rng, n_genes)).collect();
            Rule::Conjunction(terms)
        }
    }
}

/// Sample a full genome from the space. Loci are drawn in a fixed order:
/// every indicator's params (in order), then the entry rule, then the exit rule.
#[must_use]
pub fn sample_spec(rng: &mut SplitMix64, sp: &SearchSpace) -> Genome {
    let genes: Vec<Gene> = sp
        .indicators
        .iter()
        .map(|ind| {
            let kind = indicator_kind(&ind.name).map_or("Sma", |(k, _)| k);
            Gene {
                name: ind.name.clone(),
                kind: kind.to_owned(),
                params: sample_params(rng, &ind.param_ranges),
            }
        })
        .collect();
    let n = genes.len();
    let entry = sample_rule(rng, sp, n);
    let exit = sample_rule(rng, sp, n);
    Genome { genes, entry, exit }
}

fn resample_param(rng: &mut SplitMix64, range: &ParamRange) -> f64 {
    sample_params(rng, std::slice::from_ref(range))[0]
}

fn mutate_term(rng: &mut SplitMix64, t: &mut Term, rate: f64, n_genes: usize) {
    if rng.next_f64() < rate {
        t.gene = rng.below(n_genes as u64) as usize;
    }
    if rng.next_f64() < rate {
        t.cmp = sample_cmp(rng);
    }
    if rng.next_f64() < rate {
        t.konst = sample_konst(rng);
    }
}

/// Mutate a genome: each locus is resampled with probability `rate`. A draw is
/// taken at every locus regardless of outcome, so the RNG stream stays aligned.
#[must_use]
pub fn mutate(g: &Genome, sp: &SearchSpace, rate: f64, rng: &mut SplitMix64) -> Genome {
    let mut out = g.clone();
    let n_genes = out.genes.len();
    for (gene, ind) in out.genes.iter_mut().zip(&sp.indicators) {
        for (param, range) in gene.params.iter_mut().zip(&ind.param_ranges) {
            if rng.next_f64() < rate {
                *param = resample_param(rng, range);
            }
        }
    }
    mutate_rule(rng, &mut out.entry, rate, n_genes);
    mutate_rule(rng, &mut out.exit, rate, n_genes);
    out
}

fn mutate_rule(rng: &mut SplitMix64, rule: &mut Rule, rate: f64, n_genes: usize) {
    match rule {
        Rule::Single(t) => mutate_term(rng, t, rate, n_genes),
        Rule::Cross { a, b, above } => {
            if rng.next_f64() < rate {
                *a = rng.below(n_genes as u64) as usize;
            }
            if rng.next_f64() < rate {
                *b = rng.below(n_genes as u64) as usize;
            }
            if rng.next_f64() < rate {
                *above = rng.next_f64() < 0.5;
            }
        }
        Rule::Conjunction(terms) => {
            for t in terms.iter_mut() {
                mutate_term(rng, t, rate, n_genes);
            }
        }
    }
}

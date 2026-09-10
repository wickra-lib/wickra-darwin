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

/// Resolve a gene name to the registry's indicator kind and its parameter arity.
///
/// This was a four-name allowlist (`sma`, `ema`, `rsi`, `atr`) mapping onto the
/// `wickra-core` type names, while the README described searching the whole
/// indicator space. Every other name was rejected, so the space the engine could
/// actually reach was four indicators wide, against the 497 names the registry
/// resolves.
///
/// The registry is the allowlist now: it knows both halves of the answer, and
/// `wickra-backtest` resolves the very same names when it runs a candidate, so
/// the space DARWIN samples from and the space the engine can execute are the
/// same set by construction rather than by two lists agreeing.
///
/// A name is accepted in the spelling the registry uses (`Sma`, `Rsi`, `Macd`)
/// or in lower case, because the four the allowlist carried were lower case and
/// every committed spec is written that way.
#[must_use]
pub fn indicator_kind(name: &str) -> Option<(String, usize)> {
    let canonical = canonical_name(name)?;
    let arity = arity_of(&canonical)?;
    Some((canonical, arity))
}

/// The registry's spelling of `name`, or `None` if it knows no such indicator.
fn canonical_name(name: &str) -> Option<String> {
    // The registry is case-sensitive and uses PascalCase. Try the name as given
    // first -- a spec that already uses the registry spelling costs nothing --
    // then the capitalised form, which is what `sma` and its three neighbours
    // were being translated into by hand.
    if arity_of(name).is_some() {
        return Some(name.to_string());
    }
    let mut chars = name.chars();
    let capitalised: String = match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => return None,
    };
    arity_of(&capitalised).is_some().then_some(capitalised)
}

/// How many parameters the registry wants for `name`, or `None` if unknown.
///
/// The registry validates a name and its parameters together and exposes no
/// arity table, so this asks it: the smallest parameter count it accepts is the
/// arity. Four is the widest any indicator in the catalogue takes.
fn arity_of(name: &str) -> Option<usize> {
    const PROBE: [f64; 4] = [14.0, 26.0, 9.0, 2.0];
    (0..=PROBE.len()).find(|&n| registry_build(name, &PROBE[..n]).is_ok())
}

/// Whether the registry can build `name` with `params`, discarding the result.
fn registry_build(name: &str, params: &[f64]) -> core::result::Result<(), ()> {
    wickra_backtest::core::registry::build(name, params)
        .map(|_| ())
        .map_err(|_| ())
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
///
/// # Panics
/// If an indicator name does not resolve in the registry. Every caller runs
/// [`SearchSpace::validate`] first, which resolves them all, so this cannot
/// happen; substituting a different indicator for the one the spec named would
/// be worse than failing.
#[must_use]
pub fn sample_spec(rng: &mut SplitMix64, sp: &SearchSpace) -> Genome {
    let genes: Vec<Gene> = sp
        .indicators
        .iter()
        .map(|ind| {
            // `SearchSpace::validate` has already resolved every name;
            // this cannot fail, and substituting a different indicator
            // for one the spec asked for would be worse than failing.
            let (kind, _) =
                indicator_kind(&ind.name).expect("validate resolved every indicator name");
            Gene {
                name: ind.name.clone(),
                kind,
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

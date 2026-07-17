//! The genome: a typed representation of a candidate strategy and the genetic
//! operators over it. The genome serialises to a valid `wickra-backtest`
//! `StrategySpec` (data, not code); operating on a typed structure — rather than
//! doing surgery on a `serde_json::Value` tree — keeps mutation and crossover
//! total and keeps every child a valid spec.

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::rng::SplitMix64;

/// Round to 8 decimals so the same value serialises byte-identically in every
/// binding regardless of the platform's float formatting.
#[must_use]
pub fn round8(x: f64) -> f64 {
    (x * 1e8).round() / 1e8
}

/// The value range of one indicator parameter — a search axis.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ParamRange {
    /// Inclusive lower bound.
    pub min: f64,
    /// Inclusive upper bound.
    pub max: f64,
    /// Step between candidate values.
    #[serde(default = "one")]
    pub step: f64,
}

fn one() -> f64 {
    1.0
}

/// An indicator plus its search axes.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IndicatorGene {
    /// The indicator name (lower-case; mapped to the `wickra-core` type).
    pub name: String,
    /// One range per constructor parameter.
    #[serde(default)]
    pub param_ranges: Vec<ParamRange>,
}

/// A threshold comparator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmp {
    /// Greater than.
    Gt,
    /// Greater or equal.
    Ge,
    /// Less than.
    Lt,
    /// Less or equal.
    Le,
}

impl Cmp {
    fn tag(self) -> &'static str {
        match self {
            Cmp::Gt => "gt",
            Cmp::Ge => "ge",
            Cmp::Lt => "lt",
            Cmp::Le => "le",
        }
    }

    fn sample(rng: &mut SplitMix64) -> Self {
        match rng.below(4) {
            0 => Cmp::Gt,
            1 => Cmp::Ge,
            2 => Cmp::Lt,
            _ => Cmp::Le,
        }
    }
}

/// A single `gene <cmp> const` term.
#[derive(Clone, Debug, PartialEq)]
pub struct Term {
    /// Index into the genome's genes.
    pub gene: usize,
    /// The comparator.
    pub cmp: Cmp,
    /// The threshold constant.
    pub konst: f64,
}

/// A rule instance — an entry or exit condition.
#[derive(Clone, Debug, PartialEq)]
pub enum Rule {
    /// One threshold term.
    Single(Term),
    /// `a` crosses above/below `b`.
    Cross {
        /// First gene index.
        a: usize,
        /// Second gene index.
        b: usize,
        /// Above (true) or below (false).
        above: bool,
    },
    /// All terms true (AND).
    Conjunction(Vec<Term>),
}

/// One sampled indicator: its spec key, its `wickra-core` type and its params.
#[derive(Clone, Debug, PartialEq)]
pub struct Gene {
    /// The spec's indicator key (the gene name).
    pub name: String,
    /// The `wickra-core` indicator type (capitalised).
    pub kind: String,
    /// The sampled constructor parameters.
    pub params: Vec<f64>,
}

/// A complete candidate strategy.
#[derive(Clone, Debug, PartialEq)]
pub struct Genome {
    /// The indicators (one per search-space gene, in order).
    pub genes: Vec<Gene>,
    /// The entry condition.
    pub entry: Rule,
    /// The exit condition.
    pub exit: Rule,
}

/// Sample a comparator (exposed for the search-space sampler).
#[must_use]
pub(crate) fn sample_cmp(rng: &mut SplitMix64) -> Cmp {
    Cmp::sample(rng)
}

/// Draw one value per range: `min + step * k`, `k` uniform in `0..=((max-min)/step)`.
#[must_use]
pub fn sample_params(rng: &mut SplitMix64, ranges: &[ParamRange]) -> Vec<f64> {
    ranges
        .iter()
        .map(|r| {
            let span = ((r.max - r.min) / r.step).floor();
            let choices = if span.is_finite() && span >= 0.0 {
                span as u64 + 1
            } else {
                1
            };
            let k = rng.below(choices);
            round8(r.min + r.step * k as f64)
        })
        .collect()
}

fn ref_operand(name: &str) -> Value {
    json!({ "ref": name })
}

fn const_operand(c: f64) -> Value {
    json!({ "const": c })
}

fn term_cond(t: &Term, genes: &[Gene]) -> Value {
    let mut m = Map::new();
    m.insert(
        t.cmp.tag().to_owned(),
        json!([ref_operand(&genes[t.gene].name), const_operand(t.konst)]),
    );
    Value::Object(m)
}

fn rule_cond(rule: &Rule, genes: &[Gene]) -> Value {
    match rule {
        Rule::Single(t) => term_cond(t, genes),
        Rule::Cross { a, b, above } => {
            let tag = if *above { "cross_above" } else { "cross_below" };
            let mut m = Map::new();
            m.insert(
                tag.to_owned(),
                json!([ref_operand(&genes[*a].name), ref_operand(&genes[*b].name)]),
            );
            Value::Object(m)
        }
        Rule::Conjunction(terms) => {
            let conds: Vec<Value> = terms.iter().map(|t| term_cond(t, genes)).collect();
            json!({ "all": conds })
        }
    }
}

/// Serialise a genome to a valid `StrategySpec` JSON for `symbol` / `timeframe`.
#[must_use]
pub fn to_strategy_spec(g: &Genome, symbol: &str, timeframe: &str) -> Value {
    let mut inds = Map::new();
    for gene in &g.genes {
        inds.insert(
            gene.name.clone(),
            json!({ "type": gene.kind, "params": gene.params }),
        );
    }
    json!({
        "symbol": symbol,
        "timeframe": timeframe,
        "indicators": Value::Object(inds),
        "entry": rule_cond(&g.entry, &g.genes),
        "exit": rule_cond(&g.exit, &g.genes),
        "sizing": { "type": "fixed_fraction", "fraction": 0.1 },
    })
}

/// Recursively sort object keys so serialisation is canonical regardless of the
/// `serde_json` feature set (insurance against `preserve_order`).
#[must_use]
pub fn canonicalize(v: &Value) -> Value {
    match v {
        Value::Object(m) => {
            let mut sorted = Map::new();
            let mut keys: Vec<&String> = m.keys().collect();
            keys.sort();
            for k in keys {
                sorted.insert(k.clone(), canonicalize(&m[k]));
            }
            Value::Object(sorted)
        }
        Value::Array(a) => Value::Array(a.iter().map(canonicalize).collect()),
        other => other.clone(),
    }
}

/// FNV-1a-64 hex (16 lower-case chars) over the canonical JSON of `spec`.
#[must_use]
pub fn spec_hash(spec: &Value) -> String {
    let canonical = canonicalize(spec);
    let s = serde_json::to_string(&canonical).unwrap_or_default();
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in s.as_bytes() {
        h ^= u64::from(*byte);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{h:016x}")
}

fn pick<T: Clone>(rng: &mut SplitMix64, a: &T, b: &T) -> T {
    if rng.next_f64() < 0.5 {
        a.clone()
    } else {
        b.clone()
    }
}

fn cross_term(rng: &mut SplitMix64, a: &Term, b: &Term) -> Term {
    Term {
        gene: pick(rng, &a.gene, &b.gene),
        cmp: pick(rng, &a.cmp, &b.cmp),
        konst: pick(rng, &a.konst, &b.konst),
    }
}

fn cross_rule(rng: &mut SplitMix64, a: &Rule, b: &Rule) -> Rule {
    match (a, b) {
        (Rule::Single(ta), Rule::Single(tb)) => Rule::Single(cross_term(rng, ta, tb)),
        (
            Rule::Cross {
                a: aa,
                b: ab,
                above: aabove,
            },
            Rule::Cross {
                a: ba,
                b: bb,
                above: babove,
            },
        ) => Rule::Cross {
            a: pick(rng, aa, ba),
            b: pick(rng, ab, bb),
            above: pick(rng, aabove, babove),
        },
        (Rule::Conjunction(ta), Rule::Conjunction(tb)) => {
            let n = ta.len().min(tb.len());
            let mut terms: Vec<Term> = (0..n).map(|i| cross_term(rng, &ta[i], &tb[i])).collect();
            terms.extend(ta[n..].iter().cloned());
            Rule::Conjunction(terms)
        }
        // Mismatched shapes: keep the first parent's rule.
        (other, _) => other.clone(),
    }
}

/// Uniform crossover: each gene and each rule locus comes from either parent.
#[must_use]
pub fn crossover(a: &Genome, b: &Genome, rng: &mut SplitMix64) -> Genome {
    let n = a.genes.len().min(b.genes.len());
    let mut genes: Vec<Gene> = (0..n)
        .map(|i| pick(rng, &a.genes[i], &b.genes[i]))
        .collect();
    genes.extend(a.genes[n..].iter().cloned());
    Genome {
        genes,
        entry: cross_rule(rng, &a.entry, &b.entry),
        exit: cross_rule(rng, &a.exit, &b.exit),
    }
}

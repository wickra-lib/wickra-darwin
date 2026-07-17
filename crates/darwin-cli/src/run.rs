//! Load inputs, run the evolution, render the output.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::io::Read;

use darwin_core::{evolve, EvolveReport, EvolveSpec};
use wickra_backtest::{data, Candle};

use crate::args::{Args, Format};

/// Execute the CLI: load the spec and data, evolve, and render.
///
/// # Errors
/// Returns a human-readable message on any load, parse or evolution failure.
pub fn run(args: &Args) -> Result<String, String> {
    let spec = load_spec(args)?;
    let candles = load_data(args)?;
    let report = evolve(&candles, &spec).map_err(|e| e.to_string())?;
    match args.format {
        Format::Json => serde_json::to_string(&report).map_err(|e| e.to_string()),
        Format::Text => Ok(render_text(&report)),
    }
}

fn load_spec(args: &Args) -> Result<EvolveSpec, String> {
    let text = std::fs::read_to_string(&args.spec)
        .map_err(|e| format!("reading spec {}: {e}", args.spec.display()))?;
    let is_toml = args
        .spec
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("toml"));
    let mut spec = if is_toml {
        EvolveSpec::from_toml(&text)
    } else {
        EvolveSpec::from_json(&text)
    }
    .map_err(|e| e.to_string())?;
    if let Some(top) = args.top {
        spec.top = top;
    }
    if let Some(seed) = args.seed {
        spec.seed = seed;
    }
    spec.validate().map_err(|e| e.to_string())?;
    Ok(spec)
}

fn load_data(args: &Args) -> Result<BTreeMap<String, Vec<Candle>>, String> {
    if args.stdin {
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| format!("reading stdin: {e}"))?;
        return serde_json::from_str(&buf).map_err(|e| format!("parsing stdin JSON: {e}"));
    }
    let dir = args
        .data
        .as_ref()
        .ok_or_else(|| "either --data <dir> or --stdin is required".to_owned())?;
    let mut out = BTreeMap::new();
    let entries = std::fs::read_dir(dir).map_err(|e| format!("reading {}: {e}", dir.display()))?;
    for entry in entries {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("csv"))
        {
            let symbol = path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| format!("bad file name: {}", path.display()))?
                .to_owned();
            let candles = data::load_candles(&path)
                .map_err(|e| format!("loading {}: {e}", path.display()))?;
            out.insert(symbol, candles);
        }
    }
    if out.is_empty() {
        return Err(format!("no .csv candle files found in {}", dir.display()));
    }
    Ok(out)
}

fn render_text(report: &EvolveReport) -> String {
    let mut out = String::new();
    out.push_str("rank | fitness  | gen | spec_hash        | spec\n");
    out.push_str("-----+----------+-----+------------------+-----\n");
    for (i, ranked) in report.best.iter().enumerate() {
        let spec_json = serde_json::to_string(&ranked.spec).unwrap_or_default();
        let short: String = spec_json.chars().take(48).collect();
        let _ = writeln!(
            out,
            "{:>4} | {:>8.4} | {:>3} | {} | {}",
            i + 1,
            ranked.fitness,
            ranked.generation,
            ranked.spec_hash,
            short
        );
    }
    out.push_str("\nhistory:\n");
    for h in &report.history {
        let _ = writeln!(
            out,
            "  gen {:>3}: best={:>8.4} mean={:>8.4} worst={:>8.4} evaluated={}",
            h.generation, h.best, h.mean, h.worst, h.evaluated
        );
    }
    out
}

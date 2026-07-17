//! A runnable Rust example: evolve strategy specs over a small deterministic
//! universe and print the search summary. Every language example builds the same
//! 16-bar universe and the same seeded spec, so they all print the same summary
//! — that is the cross-language guarantee.
//!
//! ```bash
//! cargo run --manifest-path examples/rust/Cargo.toml
//! ```

use darwin_core::Darwin;

const SPEC: &str = r#"{
    "seed": 7, "population": 10, "generations": 4,
    "mutation_rate": 0.2, "crossover_rate": 0.6, "fitness": "sharpe",
    "search_space": {
        "indicators": [{ "name": "rsi", "param_ranges": [{ "min": 2, "max": 30 }] }],
        "rules": "single_threshold", "max_conditions": 1
    },
    "elitism": 1, "top": 3
}"#;

/// Build the shared 16-bar universe as an `evolve` command JSON.
fn evolve_command() -> String {
    let mut bars = String::new();
    for i in 0..16 {
        let t = f64::from(i);
        let close = 100.0 + 8.0 * (t / 4.0).sin() + 0.1 * t;
        let open = 100.0 + 8.0 * ((t - 1.0) / 4.0).sin() + 0.1 * (t - 1.0);
        let high = close.max(open) + 1.0;
        let low = close.min(open) - 1.0;
        if i > 0 {
            bars.push(',');
        }
        bars.push_str(&format!(
            "{{\"time\":{},\"open\":{open:.3},\"high\":{high:.3},\"low\":{low:.3},\"close\":{close:.3},\"volume\":1000}}",
            1_700_000_000 + i64::from(i) * 3600
        ));
    }
    format!("{{\"cmd\":\"evolve\",\"data\":{{\"SYM\":[{bars}]}}}}")
}

fn main() {
    let mut darwin = Darwin::new(SPEC).expect("valid spec");
    let report: serde_json::Value =
        serde_json::from_str(&darwin.command_json(&evolve_command()).expect("evolve")).unwrap();

    println!("wickra-darwin {}", darwin_core::version());
    println!("generations: {}", report["history"].as_array().unwrap().len());
    println!("hall of fame: {}", report["best"].as_array().unwrap().len());
}

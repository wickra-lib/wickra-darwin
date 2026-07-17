//! `wickra-darwin` — evolve trading strategies from candle data.

mod args;
mod run;

use std::process::ExitCode;

use clap::Parser;

fn main() -> ExitCode {
    let args = args::Args::parse();
    match run::run(&args) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

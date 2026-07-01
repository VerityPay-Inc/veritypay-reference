//! VerityPay reference interpreter CLI.

use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand};
use vp_reference_spec::{SpecificationLoadOptions, SpecificationLoader};

#[derive(Parser)]
#[command(
    name = "vp-reference",
    about = "VerityPay reference interpreter",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Load a validated `veritypay-spec` checkout through `vp-spec-model`.
    LoadSpec {
        /// Path to a `veritypay-spec` repository root.
        #[arg(long)]
        spec: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        None => {
            println!("vp-reference (bootstrapping)");
        }
        Some(Command::LoadSpec { spec }) => {
            if let Err(code) = run_load_spec(spec) {
                process::exit(code);
            }
        }
    }
}

fn run_load_spec(spec: PathBuf) -> Result<(), i32> {
    let loaded = SpecificationLoader::new()
        .load(&SpecificationLoadOptions::new(spec))
        .map_err(|error| {
            eprintln!("error: {error}");
            1
        })?;

    let summary = &loaded.context().summary;
    println!("Specification loaded");
    println!();
    println!("Terms: {}", summary.term_count);
    println!("RFCs: {}", summary.rfc_count);
    println!("Documents: {}", summary.document_count);
    println!("References: {}", summary.reference_edge_count);

    Ok(())
}

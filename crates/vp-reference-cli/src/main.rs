//! VerityPay reference interpreter CLI.

use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand, ValueEnum};
use vp_reference_cli::{run_verify, OutputFormat, VerifyOptions, EXIT_SUCCESS, EXIT_USER_ERROR};
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
    /// Verify a claim against one evidence file.
    Verify {
        /// Path to a claim JSON file.
        #[arg(long)]
        claim: PathBuf,
        /// Path to an evidence JSON file.
        #[arg(long)]
        evidence: PathBuf,
        /// Output format.
        #[arg(long, value_enum, default_value_t = CliOutputFormat::Human)]
        format: CliOutputFormat,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliOutputFormat {
    Human,
    Json,
}

impl From<CliOutputFormat> for OutputFormat {
    fn from(value: CliOutputFormat) -> Self {
        match value {
            CliOutputFormat::Human => Self::Human,
            CliOutputFormat::Json => Self::Json,
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let exit_code = match cli.command {
        None => {
            println!("vp-reference (bootstrapping)");
            EXIT_SUCCESS
        }
        Some(Command::LoadSpec { spec }) => run_load_spec(spec).err().unwrap_or(EXIT_SUCCESS),
        Some(Command::Verify {
            claim,
            evidence,
            format,
        }) => run_verify_command(claim, evidence, format.into()),
    };

    if exit_code != EXIT_SUCCESS {
        process::exit(exit_code);
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

fn run_verify_command(claim: PathBuf, evidence: PathBuf, format: OutputFormat) -> i32 {
    match run_verify(&VerifyOptions::new(claim, evidence, format)) {
        Ok(output) => {
            println!("{}", output.rendered());
            EXIT_SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}");
            EXIT_USER_ERROR
        }
    }
}

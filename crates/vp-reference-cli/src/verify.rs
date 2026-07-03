//! Verify command pipeline.

use std::path::PathBuf;

use vp_reference_interpreter::Interpreter;

use crate::error::VerifyError;
use crate::input::{build_evaluation_input, read_claim_file, read_evidence_file};
use crate::output::{render_human, render_json, OutputFormat};

/// Options for a single claim and evidence verification run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifyOptions {
    claim_path: PathBuf,
    evidence_path: PathBuf,
    format: OutputFormat,
}

impl VerifyOptions {
    pub fn new(
        claim_path: impl Into<PathBuf>,
        evidence_path: impl Into<PathBuf>,
        format: OutputFormat,
    ) -> Self {
        Self {
            claim_path: claim_path.into(),
            evidence_path: evidence_path.into(),
            format,
        }
    }

    pub fn claim_path(&self) -> &PathBuf {
        &self.claim_path
    }

    pub fn evidence_path(&self) -> &PathBuf {
        &self.evidence_path
    }

    pub fn format(&self) -> OutputFormat {
        self.format
    }
}

/// Rendered verification output for stdout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifyOutput {
    rendered: String,
}

impl VerifyOutput {
    pub fn rendered(&self) -> &str {
        &self.rendered
    }
}

pub fn run_verify(options: &VerifyOptions) -> Result<VerifyOutput, VerifyError> {
    let claim_file = read_claim_file(options.claim_path())?;
    let evidence_file = read_evidence_file(options.evidence_path())?;
    let input = build_evaluation_input(claim_file, evidence_file)?;
    let result = Interpreter::new().evaluate_input(&input);

    let rendered = match options.format() {
        OutputFormat::Human => render_human(&result),
        OutputFormat::Json => render_json(&result)?,
    };

    Ok(VerifyOutput { rendered })
}

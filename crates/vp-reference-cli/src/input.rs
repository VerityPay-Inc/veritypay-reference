//! JSON claim and evidence file loading.

use std::path::Path;

use serde::Deserialize;
use vp_reference_core::{
    ContextBuildError, EvaluationInput, EvaluationOptions, SpecificationContext,
};
use vp_reference_model::{
    Assertion, ClaimBuilder, EvaluationPolicy, EvidenceBuilder, EvidenceContent, EvidenceSetBuilder,
};

use crate::error::VerifyError;

/// Developer CLI claim file shape.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ClaimFile {
    pub claim_id: String,
    pub subject: String,
    pub assertion: AssertionFile,
}

/// Nested assertion payload in a claim file.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AssertionFile {
    pub assertion_type: String,
    pub body: String,
}

/// Developer CLI evidence file shape.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct EvidenceFile {
    pub evidence_id: String,
    pub claim_id: String,
    pub evidence_type: String,
    pub content: EvidenceContentFile,
}

/// Nested evidence content in an evidence file.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct EvidenceContentFile {
    pub content_type: String,
    pub body: String,
}

pub fn read_claim_file(path: &Path) -> Result<ClaimFile, VerifyError> {
    let contents = std::fs::read_to_string(path).map_err(|error| {
        VerifyError::user(format!(
            "failed to read claim file {}: {error}",
            path.display()
        ))
    })?;
    serde_json::from_str(&contents)
        .map_err(|error| VerifyError::user(format!("invalid claim JSON: {error}")))
}

pub fn read_evidence_file(path: &Path) -> Result<EvidenceFile, VerifyError> {
    let contents = std::fs::read_to_string(path).map_err(|error| {
        VerifyError::user(format!(
            "failed to read evidence file {}: {error}",
            path.display()
        ))
    })?;
    serde_json::from_str(&contents)
        .map_err(|error| VerifyError::user(format!("invalid evidence JSON: {error}")))
}

pub fn build_evaluation_input(
    claim_file: ClaimFile,
    evidence_file: EvidenceFile,
) -> Result<EvaluationInput, VerifyError> {
    let claim = ClaimBuilder::new()
        .id(claim_file.claim_id)
        .subject(claim_file.subject)
        .assertion(Assertion::new(
            claim_file.assertion.assertion_type,
            claim_file.assertion.body,
        ))
        .build()
        .map_err(domain_build_error)?;

    let evidence = EvidenceBuilder::new()
        .id(evidence_file.evidence_id)
        .claim_id(evidence_file.claim_id)
        .content(EvidenceContent::new(
            evidence_file.content.content_type,
            evidence_file.content.body,
        ))
        .metadata_entry("evidence_type", evidence_file.evidence_type)
        .build()
        .map_err(domain_build_error)?;

    let evidence_set = EvidenceSetBuilder::new()
        .evidence(evidence)
        .build()
        .map_err(domain_build_error)?;

    let specification = SpecificationContext {
        spec_root_identity: "developer-cli".to_owned(),
        edition_id: None,
        protocol_version: Some("platform-1.3-dev".to_owned()),
        summary: Default::default(),
    };

    EvaluationInput::builder()
        .specification_context(specification)
        .claim(claim)
        .evidence_set(evidence_set)
        .evaluation_policy(EvaluationPolicy::AllRequired)
        .options(EvaluationOptions::default())
        .build()
        .map_err(context_build_error)
}

fn domain_build_error(error: vp_reference_model::BuildError) -> VerifyError {
    VerifyError::user(format!(
        "invalid claim or evidence: missing {}",
        error.field
    ))
}

fn context_build_error(error: ContextBuildError) -> VerifyError {
    VerifyError::user(format!("invalid evaluation input: missing {}", error.field))
}

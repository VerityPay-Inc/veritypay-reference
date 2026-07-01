//! Evaluation context integration tests (Milestone C.2).

use vp_reference_core::{
    ContextBuildError, EvaluationContext, EvaluationContextBuilder, EvaluationOptions,
    SpecificationContext, SpecificationSummary,
};
use vp_reference_model::{Assertion, ClaimBuilder, EvidenceBuilder, EvidenceContent};

fn sample_specification() -> SpecificationContext {
    SpecificationContext {
        spec_root_identity: "fixture-spec".to_owned(),
        edition_id: Some("2026-01".to_owned()),
        protocol_version: Some("0.1.0".to_owned()),
        summary: SpecificationSummary {
            term_count: 1,
            rfc_count: 1,
            document_count: 1,
            reference_edge_count: 1,
        },
    }
}

fn sample_claim() -> vp_reference_model::Claim {
    ClaimBuilder::new()
        .id("claim-001")
        .subject("alice@example.com")
        .assertion(Assertion::new("minimal", "asserted-value"))
        .build()
        .expect("claim")
}

fn sample_evidence(claim_id: &vp_reference_model::ClaimId) -> vp_reference_model::Evidence {
    EvidenceBuilder::new()
        .id("evidence-001")
        .claim_id(claim_id.clone())
        .content(EvidenceContent::new("document", "supporting-body"))
        .build()
        .expect("evidence")
}

#[test]
fn builder_constructs_context_with_default_options() {
    let claim = sample_claim();
    let evidence = sample_evidence(&claim.id);

    let context = EvaluationContext::builder()
        .specification_context(sample_specification())
        .claim(claim)
        .evidence(evidence)
        .build()
        .expect("evaluation context");

    assert!(context.options().deterministic);
    assert!(context.options().trace_enabled);
}

#[test]
fn custom_options_are_preserved() {
    let claim = sample_claim();
    let evidence = sample_evidence(&claim.id);

    let context = EvaluationContext::builder()
        .specification_context(sample_specification())
        .claim(claim)
        .evidence(evidence)
        .options(EvaluationOptions {
            deterministic: false,
            trace_enabled: false,
        })
        .build()
        .expect("evaluation context");

    assert!(!context.options().deterministic);
    assert!(!context.options().trace_enabled);
}

#[test]
fn context_exposes_claim_evidence_and_specification() {
    let claim = sample_claim();
    let evidence = sample_evidence(&claim.id);
    let specification = sample_specification();

    let context = EvaluationContext::builder()
        .specification_context(specification.clone())
        .claim(claim.clone())
        .evidence(evidence.clone())
        .build()
        .expect("evaluation context");

    assert_eq!(context.specification(), &specification);
    assert_eq!(context.claim(), &claim);
    assert_eq!(context.evidence(), &evidence);
    assert_eq!(context.claim().id.as_str(), "claim-001");
    assert_eq!(
        context.specification().protocol_version.as_deref(),
        Some("0.1.0")
    );
}

#[test]
fn no_filesystem_fields_required_to_construct_context() {
    let claim = sample_claim();
    let evidence = sample_evidence(&claim.id);

    let context = EvaluationContextBuilder::new()
        .specification_context(SpecificationContext {
            spec_root_identity: "display-only-identity".to_owned(),
            edition_id: None,
            protocol_version: None,
            summary: SpecificationSummary::default(),
        })
        .claim(claim)
        .evidence(evidence)
        .build()
        .expect("evaluation context");

    assert_eq!(
        context.specification().spec_root_identity,
        "display-only-identity"
    );
}

#[test]
fn missing_required_fields_fail_clearly() {
    let claim = sample_claim();
    let evidence = sample_evidence(&claim.id);
    let specification = sample_specification();

    let err = EvaluationContext::builder()
        .claim(claim.clone())
        .evidence(evidence.clone())
        .build()
        .expect_err("missing specification");
    assert_eq!(err, ContextBuildError::missing("specification"));

    let err = EvaluationContext::builder()
        .specification_context(specification.clone())
        .evidence(evidence)
        .build()
        .expect_err("missing claim");
    assert_eq!(err, ContextBuildError::missing("claim"));

    let err = EvaluationContext::builder()
        .specification_context(specification)
        .claim(claim)
        .build()
        .expect_err("missing evidence");
    assert_eq!(err, ContextBuildError::missing("evidence"));
}

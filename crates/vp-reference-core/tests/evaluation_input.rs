//! Evaluation input integration tests (Platform 1.2 groundwork).

use vp_reference_core::{
    ContextBuildError, EvaluationInput, EvaluationInputBuilder, SpecificationContext,
    SpecificationSummary,
};
use vp_reference_model::{
    Assertion, ClaimBuilder, EvaluationPolicy, EvidenceBuilder, EvidenceContent, EvidenceSet,
};

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

#[test]
fn evaluation_input_defaults_to_all_required_policy() {
    let claim = sample_claim();
    let evidence = EvidenceBuilder::new()
        .id("evidence-001")
        .claim_id(claim.id.clone())
        .content(EvidenceContent::new("document", "supporting-body"))
        .build()
        .expect("evidence");

    let input = EvaluationInput::builder()
        .specification_context(sample_specification())
        .claim(claim)
        .evidence_set(EvidenceSet::from_vec(vec![evidence]))
        .build()
        .expect("evaluation input");

    assert_eq!(input.evaluation_policy(), EvaluationPolicy::AllRequired);
    assert_eq!(input.evaluation_policy().policy_id(), "ALL_REQUIRED");
}

#[test]
fn evaluation_input_exposes_evidence_set_in_order() {
    let claim = sample_claim();
    let first = EvidenceBuilder::new()
        .id("evidence-001")
        .claim_id(claim.id.clone())
        .content(EvidenceContent::new("document", "first"))
        .build()
        .expect("first evidence");
    let second = EvidenceBuilder::new()
        .id("evidence-002")
        .claim_id(claim.id.clone())
        .content(EvidenceContent::new("document", "second"))
        .build()
        .expect("second evidence");

    let input = EvaluationInputBuilder::new()
        .specification_context(sample_specification())
        .claim(claim)
        .evidence_set(EvidenceSet::from_vec(vec![first.clone(), second.clone()]))
        .build()
        .expect("evaluation input");

    assert_eq!(input.evidence_set().evidence(), &[first, second]);
}

#[test]
fn evaluation_input_requires_evidence_set() {
    let err = EvaluationInput::builder()
        .specification_context(sample_specification())
        .claim(sample_claim())
        .build()
        .expect_err("missing evidence set");

    assert_eq!(err, ContextBuildError::missing("evidence_set"));
}

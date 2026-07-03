//! Assertion evaluator dispatch tests (ADR-0009).

use vp_reference_core::{
    EvaluationContext, EvaluationOptions, SpecificationContext, SpecificationSummary,
};
use vp_reference_interpreter::{
    AssertionEvaluatorRegistry, Interpreter, BODY_EQUALITY_ASSERTION_TYPE,
    MINIMAL_PROFILE_ASSERTION_TYPE, NORMALIZED_TEXT_ASSERTION_TYPE,
};
use vp_reference_model::{Assertion, ClaimBuilder, EvidenceBuilder, EvidenceContent, Outcome};

fn sample_specification() -> SpecificationContext {
    SpecificationContext {
        spec_root_identity: "fixture-spec".to_owned(),
        edition_id: Some("2026-01".to_owned()),
        protocol_version: Some("0.1.0".to_owned()),
        summary: SpecificationSummary::default(),
    }
}

fn evaluation_context(assertion_type: &str) -> EvaluationContext {
    let claim = ClaimBuilder::new()
        .id("claim-001")
        .subject("alice@example.com")
        .assertion(Assertion::new(assertion_type, "alpha"))
        .build()
        .expect("claim");

    let evidence = EvidenceBuilder::new()
        .id("evidence-001")
        .claim_id(claim.id.clone())
        .content(EvidenceContent::new("document", "alpha"))
        .build()
        .expect("evidence");

    EvaluationContext::builder()
        .specification_context(sample_specification())
        .claim(claim)
        .evidence(evidence)
        .options(EvaluationOptions::default())
        .build()
        .expect("evaluation context")
}

#[test]
fn registry_supports_normalized_text() {
    let registry = AssertionEvaluatorRegistry::platform_default();

    assert!(registry.supports_assertion_type(NORMALIZED_TEXT_ASSERTION_TYPE));
    assert!(!registry.supports_assertion_type("regex"));
}

#[test]
fn registry_dispatches_normalized_text_to_satisfied() {
    let claim = ClaimBuilder::new()
        .id("claim-001")
        .subject("alice@example.com")
        .assertion(Assertion::new(NORMALIZED_TEXT_ASSERTION_TYPE, "Hello"))
        .build()
        .expect("claim");

    let evidence = EvidenceBuilder::new()
        .id("evidence-001")
        .claim_id(claim.id.clone())
        .content(EvidenceContent::new("document", " Hello "))
        .build()
        .expect("evidence");

    let context = EvaluationContext::builder()
        .specification_context(sample_specification())
        .claim(claim)
        .evidence(evidence)
        .options(EvaluationOptions::default())
        .build()
        .expect("evaluation context");

    let result = AssertionEvaluatorRegistry::platform_default().evaluate(&context);

    assert_eq!(result.outcome, Outcome::Satisfied);
}

#[test]
fn registry_supports_body_equality_and_minimal_profile_alias() {
    let registry = AssertionEvaluatorRegistry::platform_default();

    assert!(registry.supports_assertion_type(BODY_EQUALITY_ASSERTION_TYPE));
    assert!(registry.supports_assertion_type(MINIMAL_PROFILE_ASSERTION_TYPE));
    assert!(!registry.supports_assertion_type("regex"));
}

#[test]
fn registry_dispatches_body_equality_to_satisfied() {
    let context = evaluation_context(BODY_EQUALITY_ASSERTION_TYPE);

    let result = AssertionEvaluatorRegistry::platform_default().evaluate(&context);

    assert_eq!(result.outcome, Outcome::Satisfied);
}

#[test]
fn registry_dispatches_minimal_profile_alias_to_satisfied() {
    let context = evaluation_context(MINIMAL_PROFILE_ASSERTION_TYPE);

    let result = AssertionEvaluatorRegistry::platform_default().evaluate(&context);

    assert_eq!(result.outcome, Outcome::Satisfied);
}

#[test]
fn unknown_assertion_type_yields_indeterminate() {
    let context = evaluation_context("regex");

    let result = Interpreter::new().evaluate(&context);

    assert_eq!(result.outcome, Outcome::Indeterminate);
    assert!(
        result.reasons[0].contains("Unknown assertion type 'regex'"),
        "unexpected reason: {:?}",
        result.reasons
    );
}

#[test]
fn interpreter_evaluate_public_api_unchanged_for_minimal_profile() {
    let context = evaluation_context(MINIMAL_PROFILE_ASSERTION_TYPE);

    let result = Interpreter::new().evaluate(&context);

    assert_eq!(result.outcome, Outcome::Satisfied);
    assert_eq!(result.evaluated_claim_id.as_str(), "claim-001");
}

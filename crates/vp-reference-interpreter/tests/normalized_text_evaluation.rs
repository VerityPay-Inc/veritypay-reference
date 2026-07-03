//! VP-RULE-0011 and NormalizedTextEvaluator integration tests (Platform 1.3).

use vp_reference_core::{
    EvaluationContext, EvaluationInput, EvaluationOptions, SpecificationContext,
    SpecificationSummary,
};
use vp_reference_interpreter::{
    AssertionEvaluatorRegistry, EvaluationRule, Interpreter, NormalizedTextEvaluator, VpRule0011,
    NORMALIZED_TEXT_ASSERTION_TYPE, VP_RULE_0011_REFERENCE,
};
use vp_reference_model::{
    Assertion, ClaimBuilder, EvidenceBuilder, EvidenceContent, EvidenceSetBuilder, Outcome,
};

fn sample_specification() -> SpecificationContext {
    SpecificationContext {
        spec_root_identity: "fixture-spec".to_owned(),
        edition_id: Some("2026-01".to_owned()),
        protocol_version: Some("0.1.0".to_owned()),
        summary: SpecificationSummary::default(),
    }
}

fn normalized_text_context_with_type(
    assertion_type: &str,
    assertion_body: &str,
    evidence_body: &str,
) -> EvaluationContext {
    let claim = ClaimBuilder::new()
        .id("claim-001")
        .subject("alice@example.com")
        .assertion(Assertion::new(assertion_type, assertion_body))
        .build()
        .expect("claim");

    let evidence = EvidenceBuilder::new()
        .id("evidence-001")
        .claim_id(claim.id.clone())
        .content(EvidenceContent::new("document", evidence_body))
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

fn normalized_text_context(assertion_body: &str, evidence_body: &str) -> EvaluationContext {
    normalized_text_context_with_type(
        NORMALIZED_TEXT_ASSERTION_TYPE,
        assertion_body,
        evidence_body,
    )
}

fn evaluate_normalized_text(assertion_body: &str, evidence_body: &str) -> Outcome {
    let context = normalized_text_context(assertion_body, evidence_body);
    AssertionEvaluatorRegistry::platform_default()
        .evaluate(&context)
        .outcome
}

#[test]
fn hello_matches_hello() {
    assert_eq!(
        evaluate_normalized_text("Hello", "Hello"),
        Outcome::Satisfied
    );
}

#[test]
fn leading_whitespace_is_ignored() {
    assert_eq!(
        evaluate_normalized_text("Hello", " Hello"),
        Outcome::Satisfied
    );
}

#[test]
fn internal_whitespace_is_collapsed() {
    assert_eq!(
        evaluate_normalized_text("Hello    World", "Hello World"),
        Outcome::Satisfied
    );
}

#[test]
fn comparison_is_case_sensitive() {
    assert_eq!(
        evaluate_normalized_text("Hello", "hello"),
        Outcome::NotSatisfied
    );
}

#[test]
fn nfc_equivalent_forms_match() {
    let nfc = "café";
    let nfd = "cafe\u{0301}";
    assert_eq!(
        evaluate_normalized_text(nfc, nfd),
        Outcome::Satisfied,
        "NFC-equivalent forms must satisfy"
    );
}

#[test]
fn empty_evidence_body_is_indeterminate() {
    assert_eq!(
        evaluate_normalized_text("Hello", ""),
        Outcome::Indeterminate
    );
}

#[test]
fn whitespace_only_evidence_is_indeterminate() {
    assert_eq!(
        evaluate_normalized_text("Hello", "     "),
        Outcome::Indeterminate
    );
    assert_eq!(
        evaluate_normalized_text("Hello", "\t\n  "),
        Outcome::Indeterminate
    );
}

#[test]
fn trimmed_case_mismatch_is_not_satisfied() {
    assert_eq!(
        evaluate_normalized_text("Hello", "  hello  "),
        Outcome::NotSatisfied
    );
}

#[test]
fn trim_and_collapse_still_satisfies() {
    assert_eq!(
        evaluate_normalized_text("Hello    World", "  Hello World  "),
        Outcome::Satisfied
    );
}

#[test]
fn whitespace_only_assertion_and_evidence_is_indeterminate() {
    assert_eq!(
        evaluate_normalized_text("   \t  ", " \n "),
        Outcome::Indeterminate
    );
}

#[test]
fn mismatched_normalized_values_are_not_satisfied() {
    assert_eq!(
        evaluate_normalized_text("Hello", "World"),
        Outcome::NotSatisfied
    );
}

#[test]
fn registry_dispatches_normalized_text_to_normalized_text_evaluator() {
    let registry = AssertionEvaluatorRegistry::platform_default();
    assert!(registry.supports_assertion_type(NORMALIZED_TEXT_ASSERTION_TYPE));

    let context = normalized_text_context("alpha", " alpha ");
    let result = registry.evaluate(&context);

    assert_eq!(result.outcome, Outcome::Satisfied);
}

#[test]
fn invalid_utf8_bytes_yield_indeterminate_via_normalization_helper() {
    use vp_reference_interpreter::{try_normalize_text_bytes, NormalizationError};

    assert_eq!(
        try_normalize_text_bytes(&[0xFF, 0xFE]),
        Err(NormalizationError::InvalidUtf8)
    );
}

#[test]
fn normalized_text_rule_set_uses_vp_rule_0011() {
    let evaluator = NormalizedTextEvaluator::new();
    let rule_set = evaluator.rule_set();
    let context = normalized_text_context("a", "a");
    let run = vp_reference_interpreter::evaluator_run::run_rule_set(rule_set, &context);

    assert_eq!(run.evaluations.len(), 2);
    assert_eq!(
        run.final_evaluation
            .rule_reference
            .as_ref()
            .map(|reference| reference.as_str()),
        Some(VP_RULE_0011_REFERENCE)
    );
}

#[test]
fn platform_1_1_minimal_profile_unchanged() {
    let context = normalized_text_context_with_type("minimal", "alpha", "alpha");

    let result = Interpreter::new().evaluate(&context);
    assert_eq!(result.outcome, Outcome::Satisfied);
}

#[test]
fn platform_1_2_multi_evidence_normalized_text_all_required() {
    let claim = ClaimBuilder::new()
        .id("claim-001")
        .subject("alice@example.com")
        .assertion(Assertion::new(
            NORMALIZED_TEXT_ASSERTION_TYPE,
            "Hello World",
        ))
        .build()
        .expect("claim");

    let evidence_set = EvidenceSetBuilder::new()
        .evidence(
            EvidenceBuilder::new()
                .id("evidence-001")
                .claim_id("claim-001")
                .content(EvidenceContent::new("document", "Hello  World"))
                .build()
                .expect("evidence"),
        )
        .evidence(
            EvidenceBuilder::new()
                .id("evidence-002")
                .claim_id("claim-001")
                .content(EvidenceContent::new("document", " Hello World "))
                .build()
                .expect("evidence"),
        )
        .build()
        .expect("evidence set");

    let input = EvaluationInput::builder()
        .specification_context(sample_specification())
        .claim(claim)
        .evidence_set(evidence_set)
        .options(EvaluationOptions::default())
        .build()
        .expect("evaluation input");

    let result = Interpreter::new().evaluate_input(&input);
    assert_eq!(result.outcome, Outcome::Satisfied);
}

#[test]
fn unknown_assertion_type_still_indeterminate() {
    let context = normalized_text_context_with_type("regex", "Hello", "Hello");

    let result = Interpreter::new().evaluate(&context);
    assert_eq!(result.outcome, Outcome::Indeterminate);
}

#[test]
fn vp_rule_0011_direct_satisfied_and_not_satisfied() {
    let rule = VpRule0011;
    let satisfied = rule.evaluate(&normalized_text_context("Hello", " Hello"));
    assert_eq!(satisfied.outcome, Outcome::Satisfied);

    let not_satisfied = rule.evaluate(&normalized_text_context("Hello", "hello"));
    assert_eq!(not_satisfied.outcome, Outcome::NotSatisfied);
}

#[test]
fn body_equality_unaffected_by_whitespace_normalization() {
    let context = normalized_text_context_with_type("body_equality", "Hello", " Hello");

    let result = Interpreter::new().evaluate(&context);
    assert_eq!(
        result.outcome,
        Outcome::NotSatisfied,
        "body_equality must remain literal"
    );
}

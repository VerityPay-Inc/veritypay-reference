//! Platform 1.2 multi-evidence interpreter evaluation tests (VP-RFC-0003, VP-RFC-0004).

use vp_reference_core::{
    EvaluationContext, EvaluationInput, EvaluationOptions, SpecificationContext,
    SpecificationSummary,
};
use vp_reference_interpreter::Interpreter;
use vp_reference_model::{
    Assertion, ClaimBuilder, Evidence, EvidenceBuilder, EvidenceContent, EvidenceSet,
    EvidenceSetBuilder, Outcome,
};

fn sample_specification() -> SpecificationContext {
    SpecificationContext {
        spec_root_identity: "fixture-spec".to_owned(),
        edition_id: Some("2026-01".to_owned()),
        protocol_version: Some("0.1.0".to_owned()),
        summary: SpecificationSummary::default(),
    }
}

fn sample_claim(assertion_body: &str) -> vp_reference_model::Claim {
    ClaimBuilder::new()
        .id("claim-001")
        .subject("alice@example.com")
        .assertion(Assertion::new("minimal", assertion_body))
        .build()
        .expect("claim")
}

fn sample_evidence(id: &str, claim_id: &str, body: &str) -> Evidence {
    EvidenceBuilder::new()
        .id(id)
        .claim_id(claim_id)
        .content(EvidenceContent::new("document", body))
        .build()
        .expect("evidence")
}

fn evaluation_input(
    assertion_body: &str,
    evidence: EvidenceSet,
    options: EvaluationOptions,
) -> EvaluationInput {
    EvaluationInput::builder()
        .specification_context(sample_specification())
        .claim(sample_claim(assertion_body))
        .evidence_set(evidence)
        .options(options)
        .build()
        .expect("evaluation input")
}

fn satisfied_evidence(id: &str) -> Evidence {
    sample_evidence(id, "claim-001", "alpha")
}

fn not_satisfied_evidence(id: &str) -> Evidence {
    sample_evidence(id, "claim-001", "beta")
}

fn indeterminate_evidence(id: &str) -> Evidence {
    sample_evidence(id, "claim-999", "alpha")
}

#[test]
fn empty_evidence_set_yields_indeterminate() {
    let input = evaluation_input("alpha", EvidenceSet::empty(), EvaluationOptions::default());

    let result = Interpreter::new().evaluate_input(&input);

    assert_eq!(result.outcome, Outcome::Indeterminate);
    assert_eq!(result.evaluated_claim_id.as_str(), "claim-001");
    assert_eq!(
        result.reasons,
        vec!["Evidence set is empty; ALL_REQUIRED yields indeterminate"]
    );
}

#[test]
fn one_satisfied_evidence_yields_satisfied() {
    let evidence_set = EvidenceSet::from_vec(vec![satisfied_evidence("evidence-001")]);
    let input = evaluation_input("alpha", evidence_set, EvaluationOptions::default());

    let result = Interpreter::new().evaluate_input(&input);

    assert_eq!(result.outcome, Outcome::Satisfied);
    assert_eq!(
        result.reasons,
        vec!["All 1 applicable evidence envelope(s) satisfied (ALL_REQUIRED)"]
    );
}

#[test]
fn two_satisfied_evidence_yields_satisfied() {
    let evidence_set = EvidenceSet::from_vec(vec![
        satisfied_evidence("evidence-001"),
        satisfied_evidence("evidence-002"),
    ]);
    let input = evaluation_input("alpha", evidence_set, EvaluationOptions::default());

    let result = Interpreter::new().evaluate_input(&input);

    assert_eq!(result.outcome, Outcome::Satisfied);
    assert_eq!(
        result.reasons,
        vec!["All 2 applicable evidence envelope(s) satisfied (ALL_REQUIRED)"]
    );
}

#[test]
fn one_satisfied_and_one_not_satisfied_yields_not_satisfied() {
    let evidence_set = EvidenceSet::from_vec(vec![
        satisfied_evidence("evidence-001"),
        not_satisfied_evidence("evidence-002"),
    ]);
    let input = evaluation_input("alpha", evidence_set, EvaluationOptions::default());

    let result = Interpreter::new().evaluate_input(&input);

    assert_eq!(result.outcome, Outcome::NotSatisfied);
    assert_eq!(
        result.reasons,
        vec!["At least one applicable evidence envelope is not_satisfied (ALL_REQUIRED)"]
    );
}

#[test]
fn one_satisfied_and_one_indeterminate_yields_indeterminate() {
    let evidence_set = EvidenceSet::from_vec(vec![
        satisfied_evidence("evidence-001"),
        indeterminate_evidence("evidence-002"),
    ]);
    let input = evaluation_input("alpha", evidence_set, EvaluationOptions::default());

    let result = Interpreter::new().evaluate_input(&input);

    assert_eq!(result.outcome, Outcome::Indeterminate);
    assert_eq!(
        result.reasons,
        vec!["At least one applicable evidence envelope is indeterminate with no not_satisfied (ALL_REQUIRED)"]
    );
}

#[test]
fn all_indeterminate_evidence_yields_indeterminate() {
    let evidence_set = EvidenceSet::from_vec(vec![
        indeterminate_evidence("evidence-001"),
        indeterminate_evidence("evidence-002"),
    ]);
    let input = evaluation_input("alpha", evidence_set, EvaluationOptions::default());

    let result = Interpreter::new().evaluate_input(&input);

    assert_eq!(result.outcome, Outcome::Indeterminate);
}

#[test]
fn outcome_is_independent_of_evidence_order() {
    let first_order = EvidenceSet::from_vec(vec![
        satisfied_evidence("evidence-a"),
        not_satisfied_evidence("evidence-b"),
        indeterminate_evidence("evidence-c"),
    ]);
    let second_order = EvidenceSetBuilder::new()
        .evidence(indeterminate_evidence("evidence-c"))
        .evidence(not_satisfied_evidence("evidence-b"))
        .evidence(satisfied_evidence("evidence-a"))
        .build()
        .expect("evidence set");

    let interpreter = Interpreter::new();
    let first = interpreter.evaluate_input(&evaluation_input(
        "alpha",
        first_order,
        EvaluationOptions::default(),
    ));
    let second = interpreter.evaluate_input(&evaluation_input(
        "alpha",
        second_order,
        EvaluationOptions::default(),
    ));

    assert_eq!(first.outcome, Outcome::NotSatisfied);
    assert_eq!(second.outcome, Outcome::NotSatisfied);
    assert_eq!(first.outcome, second.outcome);
}

#[test]
fn trace_includes_evidence_index_and_id_when_enabled() {
    let evidence_set = EvidenceSet::from_vec(vec![
        satisfied_evidence("evidence-001"),
        not_satisfied_evidence("evidence-002"),
    ]);
    let input = evaluation_input(
        "alpha",
        evidence_set,
        EvaluationOptions {
            deterministic: true,
            trace_enabled: true,
        },
    );

    let result = Interpreter::new().evaluate_input(&input);
    let messages: Vec<_> = result
        .trace
        .events()
        .iter()
        .map(|event| event.message.as_str())
        .collect();

    assert!(
        messages
            .iter()
            .any(|message| message.contains("evaluating evidence[0] evidence-001")),
        "expected first evidence trace marker, got {messages:?}"
    );
    assert!(
        messages
            .iter()
            .any(|message| message.contains("evaluating evidence[1] evidence-002")),
        "expected second evidence trace marker, got {messages:?}"
    );
    assert!(
        messages
            .iter()
            .any(|message| message.contains("aggregation completed with outcome not_satisfied")),
        "expected aggregation trace marker, got {messages:?}"
    );
}

#[test]
fn single_evidence_evaluate_input_matches_evaluate_context() {
    let evidence = satisfied_evidence("evidence-001");
    let input = evaluation_input(
        "alpha",
        EvidenceSet::from_vec(vec![evidence.clone()]),
        EvaluationOptions::default(),
    );

    let context = EvaluationContext::builder()
        .specification_context(sample_specification())
        .claim(sample_claim("alpha"))
        .evidence(evidence)
        .options(EvaluationOptions::default())
        .build()
        .expect("evaluation context");

    let interpreter = Interpreter::new();
    let from_context = interpreter.evaluate(&context);
    let from_input = interpreter.evaluate_input(&input);

    assert_eq!(from_context.outcome, from_input.outcome);
    assert_eq!(from_context.outcome, Outcome::Satisfied);
}

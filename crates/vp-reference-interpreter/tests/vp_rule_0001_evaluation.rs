//! VP-RULE-0001 interpreter evaluation tests (Milestone D.3 / VP-RFC-0001).

use vp_reference_core::{
    EvaluationContext, EvaluationOptions, SpecificationContext, SpecificationSummary,
};
use vp_reference_interpreter::{
    EvaluationRule, Interpreter, RuleEvaluation, RuleSet, VpRule0001, VP_RULE_0001_REFERENCE,
};
use vp_reference_model::{
    Assertion, ClaimBuilder, EvidenceBuilder, EvidenceContent, Outcome, SpecificationBinding,
};

fn sample_specification() -> SpecificationContext {
    SpecificationContext {
        spec_root_identity: "fixture-spec".to_owned(),
        edition_id: Some("2026-01".to_owned()),
        protocol_version: Some("0.1.0".to_owned()),
        summary: SpecificationSummary::default(),
    }
}

fn evaluation_context(
    assertion_body: &str,
    evidence_body: &str,
    claim_id: &str,
    evidence_claim_id: &str,
    options: EvaluationOptions,
) -> EvaluationContext {
    let claim = ClaimBuilder::new()
        .id(claim_id)
        .subject("alice@example.com")
        .assertion(Assertion::new("minimal", assertion_body))
        .build()
        .expect("claim");

    let evidence = EvidenceBuilder::new()
        .id("evidence-001")
        .claim_id(evidence_claim_id)
        .content(EvidenceContent::new("document", evidence_body))
        .build()
        .expect("evidence");

    EvaluationContext::builder()
        .specification_context(sample_specification())
        .claim(claim)
        .evidence(evidence)
        .options(options)
        .build()
        .expect("evaluation context")
}

#[test]
fn fixture_alpha_alpha_matching_claim_id_is_satisfied() {
    let context = evaluation_context(
        "alpha",
        "alpha",
        "claim-001",
        "claim-001",
        EvaluationOptions::default(),
    );

    let result = Interpreter::new().evaluate(&context);

    assert_eq!(result.outcome, Outcome::Satisfied);
    assert_eq!(result.evaluated_claim_id.as_str(), "claim-001");
    assert_eq!(
        result.specification_binding,
        SpecificationBinding::new()
            .with_edition_id("2026-01")
            .with_protocol_version("0.1.0")
    );
}

#[test]
fn fixture_alpha_beta_matching_claim_id_is_not_satisfied() {
    let context = evaluation_context(
        "alpha",
        "beta",
        "claim-001",
        "claim-001",
        EvaluationOptions::default(),
    );

    let result = Interpreter::new().evaluate(&context);

    assert_eq!(result.outcome, Outcome::NotSatisfied);
}

#[test]
fn fixture_alpha_empty_evidence_body_is_indeterminate() {
    let context = evaluation_context(
        "alpha",
        "",
        "claim-001",
        "claim-001",
        EvaluationOptions::default(),
    );

    let result = Interpreter::new().evaluate(&context);

    assert_eq!(result.outcome, Outcome::Indeterminate);
}

#[test]
fn fixture_mismatched_evidence_claim_id_is_indeterminate() {
    let context = evaluation_context(
        "alpha",
        "alpha",
        "claim-001",
        "claim-002",
        EvaluationOptions::default(),
    );

    let result = Interpreter::new().evaluate(&context);

    assert_eq!(result.outcome, Outcome::Indeterminate);
}

#[test]
fn trace_disabled_returns_empty_trace() {
    let context = evaluation_context(
        "alpha",
        "alpha",
        "claim-001",
        "claim-001",
        EvaluationOptions {
            deterministic: true,
            trace_enabled: false,
        },
    );

    let result = Interpreter::new().evaluate(&context);

    assert!(result.trace.is_empty());
    assert_eq!(result.outcome, Outcome::Satisfied);
}

#[test]
fn deterministic_trace_ids_are_stable() {
    let context = evaluation_context(
        "alpha",
        "alpha",
        "claim-001",
        "claim-001",
        EvaluationOptions {
            deterministic: true,
            trace_enabled: true,
        },
    );

    let interpreter = Interpreter::new();
    let first = interpreter.evaluate(&context);
    let second = interpreter.evaluate(&context);

    let first_ids: Vec<_> = first
        .trace
        .events()
        .iter()
        .map(|event| event.id.as_str().to_owned())
        .collect();
    let second_ids: Vec<_> = second
        .trace
        .events()
        .iter()
        .map(|event| event.id.as_str().to_owned())
        .collect();

    assert_eq!(first_ids, vec!["evt-1", "evt-2", "evt-3"]);
    assert_eq!(first_ids, second_ids);

    let rule_event = &first.trace.events()[1];
    assert_eq!(
        rule_event.rule_reference.as_ref().map(|r| r.as_str()),
        Some(VP_RULE_0001_REFERENCE)
    );
}

#[test]
fn interpreter_has_no_filesystem_dependency() {
    let manifest = include_str!("../Cargo.toml");
    assert!(!manifest.contains("std::path"));
    assert!(!manifest.contains("vp-reference-spec"));
}

#[test]
fn vp_rule_0001_evaluates_independently() {
    let context = evaluation_context(
        "alpha",
        "beta",
        "claim-001",
        "claim-001",
        EvaluationOptions::default(),
    );

    let evaluation = VpRule0001.evaluate(&context);

    assert_eq!(evaluation.outcome, Outcome::NotSatisfied);
    assert_eq!(
        evaluation.rule_reference.as_ref().map(|r| r.as_str()),
        Some(VP_RULE_0001_REFERENCE)
    );
    assert!(!evaluation.reason.is_empty());
}

#[test]
fn verification_result_includes_reason() {
    let context = evaluation_context(
        "alpha",
        "alpha",
        "claim-001",
        "claim-001",
        EvaluationOptions::default(),
    );

    let result = Interpreter::new().evaluate(&context);

    assert_eq!(
        result.reasons,
        vec!["Assertion body matches evidence body (VP-RULE-0001)"]
    );
}

struct OrderRecordingRule {
    label: &'static str,
    log: std::sync::Arc<std::sync::Mutex<Vec<&'static str>>>,
}

impl OrderRecordingRule {
    fn new(label: &'static str, log: std::sync::Arc<std::sync::Mutex<Vec<&'static str>>>) -> Self {
        Self { label, log }
    }
}

impl EvaluationRule for OrderRecordingRule {
    fn evaluate(&self, _context: &EvaluationContext) -> RuleEvaluation {
        self.log.lock().expect("order log").push(self.label);
        RuleEvaluation::new(Outcome::Satisfied, format!("from {}", self.label))
    }
}

#[test]
fn interpreter_executes_rules_in_ruleset_order() {
    let order = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

    let rule_set = RuleSet::from_rules(vec![
        Box::new(OrderRecordingRule::new(
            "first",
            std::sync::Arc::clone(&order),
        )),
        Box::new(OrderRecordingRule::new(
            "second",
            std::sync::Arc::clone(&order),
        )),
    ]);

    let context = evaluation_context(
        "alpha",
        "alpha",
        "claim-001",
        "claim-001",
        EvaluationOptions::default(),
    );

    let result = Interpreter::with_rule_set(rule_set).evaluate(&context);

    let recorded = order.lock().expect("order log");
    assert_eq!(*recorded, vec!["first", "second"]);
    assert_eq!(result.outcome, Outcome::Satisfied);
    assert_eq!(result.reasons, vec!["from second"]);
}

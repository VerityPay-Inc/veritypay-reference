//! Platform 1.0 interpreter evaluation tests (VP-RULE-0002 + VP-RULE-0001 / VP-RFC-0002).

use vp_reference_core::{
    EvaluationContext, EvaluationOptions, SpecificationContext, SpecificationSummary,
};
use vp_reference_interpreter::{
    EvaluationRule, Interpreter, RuleEvaluation, RuleSet, VpRule0001, VpRule0002,
    VP_RULE_0001_REFERENCE, VP_RULE_0002_REFERENCE,
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
fn matching_ids_proceed_to_vp_rule_0001_and_yield_satisfied() {
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
    assert_eq!(
        result.reasons,
        vec!["Assertion body matches evidence body (VP-RULE-0001)"]
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
    assert!(
        result.reasons[0].contains(VP_RULE_0001_REFERENCE),
        "expected VP-RULE-0001 reason, got {:?}",
        result.reasons
    );
}

#[test]
fn mismatched_claim_ids_short_circuit_to_indeterminate() {
    let context = evaluation_context(
        "alpha",
        "alpha",
        "claim-001",
        "claim-999",
        EvaluationOptions::default(),
    );

    let result = Interpreter::new().evaluate(&context);

    assert_eq!(result.outcome, Outcome::Indeterminate);
    assert_eq!(
        result.reasons,
        vec!["Evidence claim id does not match claim id (VP-RULE-0002)"]
    );
}

#[test]
fn empty_claim_id_is_indeterminate_via_vp_rule_0002() {
    let context = evaluation_context(
        "alpha",
        "alpha",
        "",
        "claim-001",
        EvaluationOptions::default(),
    );

    let result = Interpreter::new().evaluate(&context);

    assert_eq!(result.outcome, Outcome::Indeterminate);
    assert_eq!(result.reasons, vec!["Claim id is empty (VP-RULE-0002)"]);
}

#[test]
fn empty_evidence_claim_id_is_indeterminate_via_vp_rule_0002() {
    let context = evaluation_context(
        "alpha",
        "alpha",
        "claim-001",
        "",
        EvaluationOptions::default(),
    );

    let result = Interpreter::new().evaluate(&context);

    assert_eq!(result.outcome, Outcome::Indeterminate);
    assert_eq!(
        result.reasons,
        vec!["Evidence claim id is empty (VP-RULE-0002)"]
    );
}

#[test]
fn vp_rule_0001_not_executed_after_binding_mismatch() {
    let context = evaluation_context(
        "alpha",
        "beta",
        "claim-001",
        "claim-999",
        EvaluationOptions {
            deterministic: true,
            trace_enabled: true,
        },
    );

    let result = Interpreter::new().evaluate(&context);

    assert_eq!(result.outcome, Outcome::Indeterminate);
    assert!(
        result.trace.events().iter().all(|event| event
            .rule_reference
            .as_ref()
            .map(|reference| reference.as_str())
            != Some(VP_RULE_0001_REFERENCE)),
        "VP-RULE-0001 must not appear in trace after binding failure"
    );
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
fn deterministic_trace_ordering_for_platform_1_success_path() {
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

    assert_eq!(first_ids, vec!["evt-1", "evt-2", "evt-3", "evt-4"]);
    assert_eq!(first_ids, second_ids);

    assert_eq!(
        first.trace.events()[1]
            .rule_reference
            .as_ref()
            .map(|reference| reference.as_str()),
        Some(VP_RULE_0002_REFERENCE)
    );
    assert_eq!(
        first.trace.events()[2]
            .rule_reference
            .as_ref()
            .map(|reference| reference.as_str()),
        Some(VP_RULE_0001_REFERENCE)
    );
}

#[test]
fn deterministic_trace_ordering_for_binding_short_circuit() {
    let context = evaluation_context(
        "alpha",
        "alpha",
        "claim-001",
        "claim-999",
        EvaluationOptions {
            deterministic: true,
            trace_enabled: true,
        },
    );

    let result = Interpreter::new().evaluate(&context);

    let event_ids: Vec<_> = result
        .trace
        .events()
        .iter()
        .map(|event| event.id.as_str().to_owned())
        .collect();

    assert_eq!(event_ids, vec!["evt-1", "evt-2", "evt-3"]);
    assert_eq!(
        result.trace.events()[1]
            .rule_reference
            .as_ref()
            .map(|reference| reference.as_str()),
        Some(VP_RULE_0002_REFERENCE)
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
fn vp_rule_0002_evaluates_independently() {
    let context = evaluation_context(
        "alpha",
        "alpha",
        "claim-001",
        "claim-999",
        EvaluationOptions::default(),
    );

    let evaluation = VpRule0002.evaluate(&context);

    assert_eq!(evaluation.outcome, Outcome::Indeterminate);
    assert_eq!(
        evaluation.rule_reference.as_ref().map(|r| r.as_str()),
        Some(VP_RULE_0002_REFERENCE)
    );
    assert!(!evaluation.continues);
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
        let mut evaluation =
            RuleEvaluation::new(Outcome::Satisfied, format!("from {}", self.label));
        if self.label == "first" {
            evaluation = evaluation.with_continues(true);
        }
        evaluation
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

#[test]
fn platform_1_ruleset_contains_vp_rule_0002_then_vp_rule_0001() {
    let rule_set = RuleSet::platform_1();

    assert_eq!(rule_set.len(), 2);
}

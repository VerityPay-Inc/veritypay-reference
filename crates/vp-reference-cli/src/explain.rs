//! Informative verification explanations derived from evaluation input and trace.

use vp_reference_core::EvaluationInput;
use vp_reference_model::{Outcome, Trace, VerificationResult};

const VP_RULE_0001: &str = "VP-RULE-0001";
const VP_RULE_0011: &str = "VP-RULE-0011";

/// Developer-facing explanation derived from trace and evaluation context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationExplanation {
    pub applied_rules: Vec<String>,
    pub steps: Vec<String>,
}

/// Summary of one evidence envelope for JSON explain output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceSummary {
    pub evidence_id: String,
    pub claim_id: String,
    pub content_type: String,
}

pub fn evidence_summaries(input: &EvaluationInput) -> Vec<EvidenceSummary> {
    input
        .evidence_set()
        .evidence()
        .iter()
        .map(|evidence| EvidenceSummary {
            evidence_id: evidence.id.as_str().to_owned(),
            claim_id: evidence.claim_id.as_str().to_owned(),
            content_type: evidence.content.content_type.clone(),
        })
        .collect()
}

pub fn build_explanation(
    input: &EvaluationInput,
    result: &VerificationResult,
) -> VerificationExplanation {
    let applied_rules = applied_rules_from_trace(&result.trace);
    let steps = explanation_steps(input, result.outcome, &applied_rules);

    VerificationExplanation {
        applied_rules,
        steps,
    }
}

pub fn applied_rules_from_trace(trace: &Trace) -> Vec<String> {
    let mut rules = Vec::new();

    for event in trace.events() {
        let Some(rule_reference) = event.rule_reference.as_ref() else {
            continue;
        };
        let rule_id = rule_reference.as_str();
        if rules.iter().any(|existing| existing == rule_id) {
            continue;
        }
        rules.push(rule_id.to_owned());
    }

    rules
}

fn explanation_steps(
    input: &EvaluationInput,
    outcome: Outcome,
    applied_rules: &[String],
) -> Vec<String> {
    let mut steps = Vec::new();
    let claim = input.claim();
    let evidence = input
        .evidence_set()
        .evidence()
        .first()
        .expect("verify CLI uses one evidence envelope");

    if claim.id == evidence.claim_id {
        steps.push("Evidence claim_id matched claim claim_id.".to_owned());
    } else {
        steps.push("Evidence claim_id did not match claim claim_id.".to_owned());
    }

    if applied_rules.iter().any(|rule| rule == VP_RULE_0011) {
        steps.push("Evidence text was normalized.".to_owned());
        steps.push(normalized_text_comparison_step(
            outcome,
            &evidence.content.body,
        ));
    } else if applied_rules.iter().any(|rule| rule == VP_RULE_0001) {
        steps.push(body_equality_comparison_step(
            outcome,
            &evidence.content.body,
        ));
    }

    steps.push(format!(
        "ALL_REQUIRED aggregation returned {}.",
        outcome.as_str()
    ));

    steps
}

fn normalized_text_comparison_step(outcome: Outcome, evidence_body: &str) -> String {
    match outcome {
        Outcome::Satisfied => {
            "Normalized assertion body matched normalized evidence body.".to_owned()
        }
        Outcome::NotSatisfied => {
            "Normalized assertion body did not match normalized evidence body.".to_owned()
        }
        Outcome::Indeterminate if evidence_body.trim().is_empty() => {
            "Evidence content was empty or whitespace-only before comparison.".to_owned()
        }
        Outcome::Indeterminate => "Normalized text comparison returned indeterminate.".to_owned(),
    }
}

fn body_equality_comparison_step(outcome: Outcome, evidence_body: &str) -> String {
    match outcome {
        Outcome::Satisfied => "Assertion body matched evidence body literally.".to_owned(),
        Outcome::NotSatisfied => "Assertion body did not match evidence body literally.".to_owned(),
        Outcome::Indeterminate if evidence_body.is_empty() => {
            "Evidence content body was empty.".to_owned()
        }
        Outcome::Indeterminate => "Body equality comparison returned indeterminate.".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vp_reference_core::{EvaluationOptions, SpecificationContext};
    use vp_reference_model::{
        Assertion, ClaimBuilder, EvaluationPolicy, EvidenceBuilder, EvidenceContent,
        EvidenceSetBuilder, Trace, TraceEvent, VerificationResult, VerificationResultBuilder,
    };

    fn sample_input(assertion_type: &str, evidence_body: &str) -> EvaluationInput {
        let claim = ClaimBuilder::new()
            .id("claim-001")
            .subject("subject")
            .assertion(Assertion::new(assertion_type, "Hello"))
            .build()
            .expect("claim");

        let evidence = EvidenceBuilder::new()
            .id("evidence-001")
            .claim_id("claim-001")
            .content(EvidenceContent::new("text/plain", evidence_body))
            .build()
            .expect("evidence");

        let evidence_set = EvidenceSetBuilder::new()
            .evidence(evidence)
            .build()
            .expect("evidence set");

        EvaluationInput::builder()
            .specification_context(SpecificationContext::placeholder())
            .claim(claim)
            .evidence_set(evidence_set)
            .evaluation_policy(EvaluationPolicy::AllRequired)
            .options(EvaluationOptions::default())
            .build()
            .expect("input")
    }

    fn trace_with_rules(rules: &[&str]) -> Trace {
        let mut builder = Trace::builder();
        for (index, rule) in rules.iter().enumerate() {
            builder = builder.event(
                TraceEvent::new(format!("evt-{index}"), format!("applied rule {rule}"))
                    .with_rule_reference(*rule),
            );
        }
        builder.build()
    }

    fn result_with_trace(outcome: Outcome, trace: Trace) -> VerificationResult {
        VerificationResultBuilder::new()
            .evaluated_claim_id("claim-001")
            .outcome(outcome)
            .trace(trace)
            .reason("fixture reason")
            .build()
            .expect("result")
    }

    #[test]
    fn applied_rules_preserves_first_seen_order_without_duplicates() {
        let trace = trace_with_rules(&["VP-RULE-0002", VP_RULE_0011, "VP-RULE-0002"]);
        assert_eq!(
            applied_rules_from_trace(&trace),
            vec!["VP-RULE-0002".to_owned(), VP_RULE_0011.to_owned()]
        );
    }

    #[test]
    fn normalized_text_satisfied_explanation_steps() {
        let input = sample_input("normalized_text", "  Hello  ");
        let result = result_with_trace(
            Outcome::Satisfied,
            trace_with_rules(&["VP-RULE-0002", VP_RULE_0011]),
        );
        let explanation = build_explanation(&input, &result);

        assert_eq!(
            explanation.applied_rules,
            vec!["VP-RULE-0002".to_owned(), VP_RULE_0011.to_owned()]
        );
        assert_eq!(
            explanation.steps,
            vec![
                "Evidence claim_id matched claim claim_id.".to_owned(),
                "Evidence text was normalized.".to_owned(),
                "Normalized assertion body matched normalized evidence body.".to_owned(),
                "ALL_REQUIRED aggregation returned satisfied.".to_owned(),
            ]
        );
    }

    #[test]
    fn body_equality_satisfied_explanation_steps() {
        let input = sample_input("body_equality", "Hello");
        let result = result_with_trace(
            Outcome::Satisfied,
            trace_with_rules(&["VP-RULE-0002", VP_RULE_0001]),
        );
        let explanation = build_explanation(&input, &result);

        assert!(explanation
            .steps
            .contains(&"Assertion body matched evidence body literally.".to_owned()));
    }
}

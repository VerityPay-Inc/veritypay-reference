//! Interpreter orchestration — coordinates rules and builds verification results.

use vp_reference_core::{EvaluationContext, SpecificationContext};
use vp_reference_model::{
    SpecificationBinding, Trace, TraceBuilder, TraceEvent, VerificationResult,
    VerificationResultBuilder,
};

use crate::rule::EvaluationRule;
use crate::rule_evaluation::RuleEvaluation;
use crate::rules::{MinimalBodyEqualityRule, MINIMAL_BODY_EQUALITY_RULE_REFERENCE};

/// Reference interpreter — orchestrates evaluation rules per ADR-0005.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interpreter {
    minimal_body_equality_rule: MinimalBodyEqualityRule,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            minimal_body_equality_rule: MinimalBodyEqualityRule,
        }
    }

    /// Bootstrap-compatible constructor.
    #[must_use]
    pub fn placeholder() -> Self {
        Self::new()
    }

    /// Evaluates the given context through the Milestone D rule set.
    #[must_use]
    pub fn evaluate(&self, context: &EvaluationContext) -> VerificationResult {
        let rule_evaluation = self.minimal_body_equality_rule.evaluate(context);
        let trace = if context.options().trace_enabled {
            build_trace(context, &rule_evaluation)
        } else {
            Trace::default()
        };

        build_verification_result(context, &rule_evaluation, trace)
    }
}

fn specification_binding(specification: &SpecificationContext) -> SpecificationBinding {
    SpecificationBinding {
        edition_id: specification.edition_id.clone(),
        protocol_version: specification.protocol_version.clone(),
    }
}

fn build_verification_result(
    context: &EvaluationContext,
    rule_evaluation: &RuleEvaluation,
    trace: Trace,
) -> VerificationResult {
    VerificationResultBuilder::new()
        .evaluated_claim_id(context.claim().id.clone())
        .outcome(rule_evaluation.outcome)
        .trace(trace)
        .specification_binding(specification_binding(context.specification()))
        .reason(rule_evaluation.reason.clone())
        .build()
        .expect("interpreter sets required verification result fields")
}

fn build_trace(context: &EvaluationContext, rule_evaluation: &RuleEvaluation) -> Trace {
    let claim_id = context.claim().id.as_str();
    let outcome_label = rule_evaluation.outcome.as_str();

    let builder = TraceBuilder::new()
        .message(event_id(1), format!("evaluation started for claim {claim_id}"))
        .event(
            TraceEvent::new(
                event_id(2),
                format!(
                    "applied rule {MINIMAL_BODY_EQUALITY_RULE_REFERENCE} to assertion.body and content.body"
                ),
            )
            .with_rule_reference(MINIMAL_BODY_EQUALITY_RULE_REFERENCE),
        )
        .message(
            event_id(3),
            format!("evaluation completed with outcome {outcome_label}"),
        );

    builder.build()
}

fn event_id(sequence: u32) -> String {
    format!("evt-{sequence}")
}

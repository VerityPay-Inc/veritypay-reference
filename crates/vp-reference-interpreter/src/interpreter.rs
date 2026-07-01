//! Interpreter orchestration — coordinates rules and builds verification results.

use vp_reference_core::{EvaluationContext, SpecificationContext};
use vp_reference_model::{
    SpecificationBinding, Trace, TraceBuilder, TraceEvent, VerificationResult,
    VerificationResultBuilder,
};

use crate::rule_evaluation::RuleEvaluation;
use crate::rule_set::RuleSet;
use crate::rules::MINIMAL_BODY_EQUALITY_RULE_REFERENCE;

/// Reference interpreter — orchestrates evaluation rules per ADR-0005 and ADR-0006.
#[derive(Debug)]
pub struct Interpreter {
    rule_set: RuleSet,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    #[must_use]
    pub fn new() -> Self {
        Self::with_rule_set(RuleSet::milestone_d())
    }

    #[must_use]
    pub fn with_rule_set(rule_set: RuleSet) -> Self {
        Self { rule_set }
    }

    /// Bootstrap-compatible constructor.
    #[must_use]
    pub fn placeholder() -> Self {
        Self::new()
    }

    #[must_use]
    pub fn rule_set(&self) -> &RuleSet {
        &self.rule_set
    }

    /// Evaluates the given context through the configured rule set.
    #[must_use]
    pub fn evaluate(&self, context: &EvaluationContext) -> VerificationResult {
        let rule_evaluation = self.evaluate_rules(context);
        let trace = if context.options().trace_enabled {
            build_trace(context, &rule_evaluation)
        } else {
            Trace::default()
        };

        build_verification_result(context, &rule_evaluation, trace)
    }

    fn evaluate_rules(&self, context: &EvaluationContext) -> RuleEvaluation {
        let mut evaluations = self
            .rule_set
            .rules()
            .iter()
            .map(|rule| rule.evaluate(context));

        let first = evaluations
            .next()
            .expect("rule set must contain at least one rule");

        // Milestone D aggregation: single rule pass-through; last rule wins when multiple exist.
        evaluations.fold(first, |_, next| next)
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

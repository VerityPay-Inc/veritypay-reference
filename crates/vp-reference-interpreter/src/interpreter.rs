//! Interpreter orchestration — coordinates rules and builds verification results.

use vp_reference_core::{EvaluationContext, SpecificationContext};
use vp_reference_model::{
    SpecificationBinding, Trace, TraceBuilder, TraceEvent, VerificationResult,
    VerificationResultBuilder,
};

use crate::rule_evaluation::RuleEvaluation;
use crate::rule_set::RuleSet;

/// Ordered rule evaluations produced by one interpreter invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
struct RuleSetRun {
    evaluations: Vec<RuleEvaluation>,
    final_evaluation: RuleEvaluation,
}

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
        Self::with_rule_set(RuleSet::platform_1())
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
        let run = self.evaluate_rules(context);
        let trace = if context.options().trace_enabled {
            build_trace(context, &run)
        } else {
            Trace::default()
        };

        build_verification_result(context, &run.final_evaluation, trace)
    }

    fn evaluate_rules(&self, context: &EvaluationContext) -> RuleSetRun {
        let mut evaluations = Vec::new();

        for rule in self.rule_set.rules() {
            let evaluation = rule.evaluate(context);
            let continues = evaluation.continues;
            evaluations.push(evaluation);
            if !continues {
                break;
            }
        }

        let final_evaluation = evaluations
            .last()
            .cloned()
            .expect("rule set must contain at least one rule");

        RuleSetRun {
            evaluations,
            final_evaluation,
        }
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

fn build_trace(context: &EvaluationContext, run: &RuleSetRun) -> Trace {
    let claim_id = context.claim().id.as_str();
    let outcome_label = run.final_evaluation.outcome.as_str();

    let mut builder = TraceBuilder::new().message(
        event_id(1),
        format!("evaluation started for claim {claim_id}"),
    );

    let mut sequence = 2_u32;
    for evaluation in &run.evaluations {
        if let Some(rule_reference) = evaluation.rule_reference.as_ref() {
            builder = builder.event(
                TraceEvent::new(
                    event_id(sequence),
                    format!("applied rule {}", rule_reference.as_str()),
                )
                .with_rule_reference(rule_reference.as_str()),
            );
            sequence += 1;
        }

        if !evaluation.continues {
            break;
        }
    }

    builder
        .message(
            event_id(sequence),
            format!("evaluation completed with outcome {outcome_label}"),
        )
        .build()
}

fn event_id(sequence: u32) -> String {
    format!("evt-{sequence}")
}

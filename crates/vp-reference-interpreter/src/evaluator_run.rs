//! Per-rule and per-ruleset evaluation runs shared by assertion evaluators.

use vp_reference_core::EvaluationContext;
use vp_reference_model::{Trace, TraceBuilder, TraceEvent};

use crate::rule_evaluation::RuleEvaluation;
use crate::rule_set::RuleSet;

/// Ordered rule evaluations produced by one evaluator invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleSetRun {
    pub evaluations: Vec<RuleEvaluation>,
    pub final_evaluation: RuleEvaluation,
}

pub fn run_rule_set(rule_set: &RuleSet, context: &EvaluationContext) -> RuleSetRun {
    let mut evaluations = Vec::new();

    for rule in rule_set.rules() {
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

pub fn build_trace(context: &EvaluationContext, run: &RuleSetRun) -> Trace {
    let claim_id = context.claim().id.as_str();
    let outcome_label = run.final_evaluation.outcome.as_str();

    let mut builder = TraceBuilder::new().message(
        event_id(1),
        format!("evaluation started for claim {claim_id}"),
    );

    let mut sequence = 2_u32;
    builder = append_rule_trace_events(builder, &mut sequence, run);

    builder
        .message(
            event_id(sequence),
            format!("evaluation completed with outcome {outcome_label}"),
        )
        .build()
}

pub fn append_rule_trace_events(
    mut builder: TraceBuilder,
    sequence: &mut u32,
    run: &RuleSetRun,
) -> TraceBuilder {
    for evaluation in &run.evaluations {
        if let Some(rule_reference) = evaluation.rule_reference.as_ref() {
            builder = builder.event(
                TraceEvent::new(
                    event_id(*sequence),
                    format!("applied rule {}", rule_reference.as_str()),
                )
                .with_rule_reference(rule_reference.as_str()),
            );
            *sequence += 1;
        }

        if !evaluation.continues {
            break;
        }
    }
    builder
}

fn event_id(sequence: u32) -> String {
    format!("evt-{sequence}")
}

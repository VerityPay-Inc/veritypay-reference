//! Interpreter orchestration — coordinates rules and builds verification results.

use vp_reference_core::{EvaluationContext, EvaluationInput, SpecificationContext};
use vp_reference_model::{
    EvaluationPolicy, Evidence, Outcome, SpecificationBinding, Trace, TraceBuilder, TraceEvent,
    VerificationResult, VerificationResultBuilder,
};

use crate::evaluation_policy::{aggregate_all_required, all_required_aggregation_reason};
use crate::rule_evaluation::RuleEvaluation;
use crate::rule_set::RuleSet;

/// Ordered rule evaluations produced by one interpreter invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
struct RuleSetRun {
    evaluations: Vec<RuleEvaluation>,
    final_evaluation: RuleEvaluation,
}

/// Per-envelope rule pipeline result for multi-evidence evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
struct EnvelopeEvaluation {
    index: usize,
    evidence_id: vp_reference_model::EvidenceId,
    run: RuleSetRun,
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

    /// Evaluates multi-evidence input using the declared evaluation policy (Platform 1.2).
    #[must_use]
    pub fn evaluate_input(&self, input: &EvaluationInput) -> VerificationResult {
        match input.evaluation_policy() {
            EvaluationPolicy::AllRequired => self.evaluate_all_required(input),
        }
    }

    fn evaluate_all_required(&self, input: &EvaluationInput) -> VerificationResult {
        let envelope_evaluations = self.evaluate_envelopes(input);
        let per_envelope_outcomes: Vec<Outcome> = envelope_evaluations
            .iter()
            .map(|envelope| envelope.run.final_evaluation.outcome)
            .collect();

        let aggregated = aggregate_all_required(&per_envelope_outcomes);
        let reason = all_required_aggregation_reason(aggregated, envelope_evaluations.len());

        let trace = if input.options().trace_enabled {
            build_multi_evidence_trace(input, &envelope_evaluations, aggregated)
        } else {
            Trace::default()
        };

        VerificationResultBuilder::new()
            .evaluated_claim_id(input.claim().id.clone())
            .outcome(aggregated)
            .trace(trace)
            .specification_binding(specification_binding(input.specification()))
            .reason(reason)
            .build()
            .expect("interpreter sets required verification result fields")
    }

    fn evaluate_envelopes(&self, input: &EvaluationInput) -> Vec<EnvelopeEvaluation> {
        input
            .evidence_set()
            .evidence()
            .iter()
            .enumerate()
            .map(|(index, evidence)| {
                let context = context_for_evidence(input, evidence);
                let run = self.evaluate_rules(&context);
                EnvelopeEvaluation {
                    index,
                    evidence_id: evidence.id.clone(),
                    run,
                }
            })
            .collect()
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

fn context_for_evidence(input: &EvaluationInput, evidence: &Evidence) -> EvaluationContext {
    EvaluationContext::builder()
        .specification_context(input.specification().clone())
        .claim(input.claim().clone())
        .evidence(evidence.clone())
        .options(input.options().clone())
        .build()
        .expect("evaluation input provides required fields")
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
    builder = append_rule_trace_events(builder, &mut sequence, run);

    builder
        .message(
            event_id(sequence),
            format!("evaluation completed with outcome {outcome_label}"),
        )
        .build()
}

fn build_multi_evidence_trace(
    input: &EvaluationInput,
    envelopes: &[EnvelopeEvaluation],
    aggregated: Outcome,
) -> Trace {
    let claim_id = input.claim().id.as_str();
    let policy_id = input.evaluation_policy().policy_id();

    let mut builder = TraceBuilder::new().message(
        event_id(1),
        format!("evaluation started for claim {claim_id} with policy {policy_id}"),
    );

    let mut sequence = 2_u32;
    for envelope in envelopes {
        builder = builder.message(
            event_id(sequence),
            format!(
                "evaluating evidence[{}] {}",
                envelope.index,
                envelope.evidence_id.as_str()
            ),
        );
        sequence += 1;

        builder = append_rule_trace_events(builder, &mut sequence, &envelope.run);

        builder = builder.message(
            event_id(sequence),
            format!(
                "evidence[{}] ({}) outcome: {}",
                envelope.index,
                envelope.evidence_id.as_str(),
                envelope.run.final_evaluation.outcome.as_str()
            ),
        );
        sequence += 1;
    }

    builder
        .message(
            event_id(sequence),
            format!("aggregation completed with outcome {}", aggregated.as_str()),
        )
        .build()
}

fn append_rule_trace_events(
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

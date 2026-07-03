//! Interpreter orchestration — dispatches assertion evaluators per ADR-0009.

use vp_reference_core::{EvaluationContext, EvaluationInput, SpecificationContext};
use vp_reference_model::{
    EvaluationPolicy, Evidence, Outcome, SpecificationBinding, Trace, TraceBuilder,
    VerificationResult, VerificationResultBuilder,
};

use crate::evaluation_policy::{aggregate_all_required, all_required_aggregation_reason};
use crate::evaluator_run::{append_rule_trace_events, RuleSetRun};
use crate::evaluators::AssertionEvaluatorRegistry;
use crate::rule_evaluation::RuleEvaluation;
use crate::rule_set::RuleSet;

/// Per-envelope rule pipeline result for multi-evidence evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
struct EnvelopeEvaluation {
    index: usize,
    evidence_id: vp_reference_model::EvidenceId,
    run: RuleSetRun,
}

/// Reference interpreter — dispatches assertion evaluators per ADR-0009.
#[derive(Debug)]
pub struct Interpreter {
    registry: AssertionEvaluatorRegistry,
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
            registry: AssertionEvaluatorRegistry::platform_default(),
        }
    }

    #[must_use]
    pub fn with_rule_set(rule_set: RuleSet) -> Self {
        Self {
            registry: AssertionEvaluatorRegistry::with_body_equality_rule_set(rule_set),
        }
    }

    /// Bootstrap-compatible constructor.
    #[must_use]
    pub fn placeholder() -> Self {
        Self::new()
    }

    #[must_use]
    pub fn rule_set(&self) -> &RuleSet {
        self.registry.body_equality_rule_set()
    }

    /// Evaluates the given context through the configured assertion evaluator registry.
    #[must_use]
    pub fn evaluate(&self, context: &EvaluationContext) -> VerificationResult {
        self.registry.evaluate(context)
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
                let run = if self
                    .registry
                    .supports_assertion_type(context.claim().assertion.assertion_type.as_str())
                {
                    self.registry.run_dispatched(&context)
                } else {
                    indeterminate_dispatch_run(&context)
                };
                EnvelopeEvaluation {
                    index,
                    evidence_id: evidence.id.clone(),
                    run,
                }
            })
            .collect()
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

fn indeterminate_dispatch_run(context: &EvaluationContext) -> RuleSetRun {
    let assertion_type = context.claim().assertion.assertion_type.as_str();
    let evaluation = RuleEvaluation::new(
        Outcome::Indeterminate,
        format!(
            "Unknown assertion type '{assertion_type}'; evaluation dispatch yields indeterminate"
        ),
    );
    RuleSetRun {
        evaluations: vec![evaluation.clone()],
        final_evaluation: evaluation,
    }
}

fn specification_binding(specification: &SpecificationContext) -> SpecificationBinding {
    SpecificationBinding {
        edition_id: specification.edition_id.clone(),
        protocol_version: specification.protocol_version.clone(),
    }
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

fn event_id(sequence: u32) -> String {
    format!("evt-{sequence}")
}

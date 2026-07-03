//! Body equality assertion evaluator — VP-RFC-0006 initial mapping.

use vp_reference_core::{EvaluationContext, SpecificationContext};
use vp_reference_model::{
    Outcome, SpecificationBinding, Trace, VerificationResult, VerificationResultBuilder,
};

use crate::evaluator_run::{build_trace, run_rule_set, RuleSetRun};
use crate::evaluators::trait_def::AssertionEvaluator;
use crate::rule_set::RuleSet;

/// Normative assertion type per VP-RFC-0005 / VP-RFC-0006.
pub const BODY_EQUALITY_ASSERTION_TYPE: &str = "body_equality";

/// Body equality evaluator — maps to **VP-RULE-0002** then **VP-RULE-0001**.
#[derive(Debug)]
pub struct BodyEqualityEvaluator {
    rule_set: RuleSet,
}

impl BodyEqualityEvaluator {
    #[must_use]
    pub fn new() -> Self {
        Self::with_rule_set(RuleSet::platform_1())
    }

    #[must_use]
    pub fn with_rule_set(rule_set: RuleSet) -> Self {
        Self { rule_set }
    }

    #[must_use]
    pub fn rule_set(&self) -> &RuleSet {
        &self.rule_set
    }

    pub(crate) fn run_rules(&self, context: &EvaluationContext) -> RuleSetRun {
        run_rule_set(&self.rule_set, context)
    }
}

impl Default for BodyEqualityEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl AssertionEvaluator for BodyEqualityEvaluator {
    fn supported_assertion_types(&self) -> &[&'static str] {
        &[BODY_EQUALITY_ASSERTION_TYPE]
    }

    fn evaluate(&self, context: &EvaluationContext) -> VerificationResult {
        let run = self.run_rules(context);
        let trace = if context.options().trace_enabled {
            build_trace(context, &run)
        } else {
            Trace::default()
        };

        build_verification_result(context, &run, trace)
    }
}

fn build_verification_result(
    context: &EvaluationContext,
    run: &RuleSetRun,
    trace: Trace,
) -> VerificationResult {
    VerificationResultBuilder::new()
        .evaluated_claim_id(context.claim().id.clone())
        .outcome(run.final_evaluation.outcome)
        .trace(trace)
        .specification_binding(specification_binding(context.specification()))
        .reason(run.final_evaluation.reason.clone())
        .build()
        .expect("evaluator sets required verification result fields")
}

fn specification_binding(specification: &SpecificationContext) -> SpecificationBinding {
    SpecificationBinding {
        edition_id: specification.edition_id.clone(),
        protocol_version: specification.protocol_version.clone(),
    }
}

pub(crate) fn unknown_assertion_type_result(context: &EvaluationContext) -> VerificationResult {
    let assertion_type = context.claim().assertion.assertion_type.as_str();

    VerificationResultBuilder::new()
        .evaluated_claim_id(context.claim().id.clone())
        .outcome(Outcome::Indeterminate)
        .trace(Trace::default())
        .specification_binding(specification_binding(context.specification()))
        .reason(format!(
            "Unknown assertion type '{assertion_type}'; evaluation dispatch yields indeterminate"
        ))
        .build()
        .expect("evaluator sets required verification result fields")
}

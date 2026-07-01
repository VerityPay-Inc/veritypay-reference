//! Evaluation rule abstraction.

use vp_reference_core::EvaluationContext;

use crate::rule_evaluation::RuleEvaluation;

/// Verification decision unit evaluated by the interpreter orchestrator.
pub trait EvaluationRule {
    fn evaluate(&self, context: &EvaluationContext) -> RuleEvaluation;
}

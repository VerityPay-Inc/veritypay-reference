//! Assertion evaluator trait — semantic evaluation for one Assertion Type family.

use vp_reference_core::EvaluationContext;
use vp_reference_model::VerificationResult;

/// Protocol-defined semantic evaluator for one or more **Assertion Type** identifiers.
pub trait AssertionEvaluator {
    /// Stable assertion type identifiers handled by this evaluator.
    fn supported_assertion_types(&self) -> &[&'static str];

    /// Evaluates the context and returns a complete verification result.
    fn evaluate(&self, context: &EvaluationContext) -> VerificationResult;
}

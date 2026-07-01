//! Interpreter entry point (placeholder).
//!
//! Accepts loaded contexts and domain types only — no filesystem inputs.

use vp_reference_core::{EvaluationContext, SpecificationContext};
use vp_reference_model::{Claim, Evidence};

/// Reference interpreter (placeholder).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Interpreter;

impl Interpreter {
    /// Placeholder constructor for workspace bootstrap tests.
    #[must_use]
    pub fn placeholder() -> Self {
        Self
    }

    /// Placeholder evaluation hook (no logic until Milestone D).
    #[must_use]
    pub fn evaluate_placeholder(
        &self,
        _evaluation: &EvaluationContext,
        _specification: &SpecificationContext,
        _claim: &Claim,
        _evidence: &Evidence,
    ) -> bool {
        true
    }
}

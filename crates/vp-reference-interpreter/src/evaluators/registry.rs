//! Dispatch registry — maps Assertion Type to AssertionEvaluator.

use vp_reference_core::EvaluationContext;
use vp_reference_model::VerificationResult;

use crate::evaluator_run::RuleSetRun;
use crate::evaluators::body_equality::{
    unknown_assertion_type_result, BodyEqualityEvaluator, BODY_EQUALITY_ASSERTION_TYPE,
};
use crate::evaluators::trait_def::AssertionEvaluator;
use crate::rule_set::RuleSet;

/// VP-RFC-0001 minimal profile label — engineering alias until fixture alignment.
pub const MINIMAL_PROFILE_ASSERTION_TYPE: &str = "minimal";

/// Dispatches evaluation by `assertion_type` per VP-RFC-0006.
#[derive(Debug)]
pub struct AssertionEvaluatorRegistry {
    body_equality: BodyEqualityEvaluator,
}

impl AssertionEvaluatorRegistry {
    #[must_use]
    pub fn platform_default() -> Self {
        Self {
            body_equality: BodyEqualityEvaluator::new(),
        }
    }

    #[must_use]
    pub fn with_body_equality_rule_set(rule_set: RuleSet) -> Self {
        Self {
            body_equality: BodyEqualityEvaluator::with_rule_set(rule_set),
        }
    }

    #[must_use]
    pub fn body_equality_rule_set(&self) -> &RuleSet {
        self.body_equality.rule_set()
    }

    #[must_use]
    pub fn supports_assertion_type(&self, assertion_type: &str) -> bool {
        matches!(
            assertion_type,
            BODY_EQUALITY_ASSERTION_TYPE | MINIMAL_PROFILE_ASSERTION_TYPE
        )
    }

    #[must_use]
    pub fn evaluate(&self, context: &EvaluationContext) -> VerificationResult {
        let assertion_type = context.claim().assertion.assertion_type.as_str();

        if self.supports_assertion_type(assertion_type) {
            self.body_equality.evaluate(context)
        } else {
            unknown_assertion_type_result(context)
        }
    }

    pub(crate) fn run_dispatched(&self, context: &EvaluationContext) -> RuleSetRun {
        debug_assert!(
            self.supports_assertion_type(context.claim().assertion.assertion_type.as_str())
        );
        self.body_equality.run_rules(context)
    }
}

impl Default for AssertionEvaluatorRegistry {
    fn default() -> Self {
        Self::platform_default()
    }
}

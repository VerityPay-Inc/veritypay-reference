//! Ordered collection of evaluation rules per ADR-0006.

use std::fmt;

use crate::rule::EvaluationRule;
use crate::rules::MinimalBodyEqualityRule;

/// Owns a deterministic, ordered sequence of [`EvaluationRule`] implementations.
pub struct RuleSet {
    rules: Vec<Box<dyn EvaluationRule>>,
}

impl fmt::Debug for RuleSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RuleSet")
            .field("rule_count", &self.rules.len())
            .finish()
    }
}

impl RuleSet {
    /// Milestone D rule set: [`MinimalBodyEqualityRule`] only.
    #[must_use]
    pub fn milestone_d() -> Self {
        Self::from_rules(vec![Box::new(MinimalBodyEqualityRule)])
    }

    #[must_use]
    pub fn from_rules(rules: Vec<Box<dyn EvaluationRule>>) -> Self {
        Self { rules }
    }

    #[must_use]
    pub fn rules(&self) -> &[Box<dyn EvaluationRule>] {
        &self.rules
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.rules.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

impl Default for RuleSet {
    fn default() -> Self {
        Self::milestone_d()
    }
}

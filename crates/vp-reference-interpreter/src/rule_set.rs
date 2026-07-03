//! Ordered collection of evaluation rules per ADR-0006.

use std::fmt;

use crate::rule::EvaluationRule;
use crate::rules::{VpRule0001, VpRule0002, VpRule0011};

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
    /// Platform 1.0 rule set: [`VpRule0002`] then [`VpRule0001`].
    #[must_use]
    pub fn platform_1() -> Self {
        Self::from_rules(vec![Box::new(VpRule0002), Box::new(VpRule0001)])
    }

    /// Platform 1.3 rule set: [`VpRule0002`] then [`VpRule0011`].
    #[must_use]
    pub fn platform_1_3() -> Self {
        Self::from_rules(vec![Box::new(VpRule0002), Box::new(VpRule0011)])
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
        Self::platform_1()
    }
}

//! Verification logic and evaluation flow for the reference interpreter.

pub mod interpreter;
pub mod rule;
pub mod rule_evaluation;
pub mod rules;

pub use interpreter::Interpreter;
pub use rule::EvaluationRule;
pub use rule_evaluation::RuleEvaluation;
pub use rules::{MinimalBodyEqualityRule, MINIMAL_BODY_EQUALITY_RULE_REFERENCE};

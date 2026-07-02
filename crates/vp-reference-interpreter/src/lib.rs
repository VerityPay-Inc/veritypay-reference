//! Verification logic and evaluation flow for the reference interpreter.

pub mod evaluation_policy;
pub mod interpreter;
pub mod rule;
pub mod rule_evaluation;
pub mod rule_set;
pub mod rules;

pub use interpreter::Interpreter;
pub use rule::EvaluationRule;
pub use rule_evaluation::RuleEvaluation;
pub use rule_set::RuleSet;
pub use rules::{VpRule0001, VpRule0002, VP_RULE_0001_REFERENCE, VP_RULE_0002_REFERENCE};

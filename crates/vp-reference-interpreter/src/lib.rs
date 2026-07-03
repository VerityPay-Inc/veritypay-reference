//! Verification logic and evaluation flow for the reference interpreter.

pub mod evaluation_policy;
pub mod evaluator_run;
pub mod evaluators;
pub mod interpreter;
pub mod rule;
pub mod rule_evaluation;
pub mod rule_set;
pub mod rules;
pub mod text_normalization;

pub use evaluators::{
    AssertionEvaluator, AssertionEvaluatorRegistry, BodyEqualityEvaluator, NormalizedTextEvaluator,
    BODY_EQUALITY_ASSERTION_TYPE, MINIMAL_PROFILE_ASSERTION_TYPE, NORMALIZED_TEXT_ASSERTION_TYPE,
};
pub use interpreter::Interpreter;
pub use rule::EvaluationRule;
pub use rule_evaluation::RuleEvaluation;
pub use rule_set::RuleSet;
pub use rules::{
    VpRule0001, VpRule0002, VpRule0011, VP_RULE_0001_REFERENCE, VP_RULE_0002_REFERENCE,
    VP_RULE_0011_REFERENCE,
};
pub use text_normalization::{normalize_text, try_normalize_text_bytes, NormalizationError};

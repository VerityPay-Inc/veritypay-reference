//! Assertion evaluator implementations and dispatch registry.

mod body_equality;
mod registry;
mod trait_def;

pub use body_equality::BodyEqualityEvaluator;
pub use body_equality::BODY_EQUALITY_ASSERTION_TYPE;
pub use registry::AssertionEvaluatorRegistry;
pub use registry::MINIMAL_PROFILE_ASSERTION_TYPE;
pub use trait_def::AssertionEvaluator;

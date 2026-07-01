//! Shared context and error contracts for the reference interpreter.

pub mod context;
pub mod context_error;
pub mod error;

pub use context::{
    EvaluationContext, EvaluationContextBuilder, EvaluationOptions, SpecificationContext,
    SpecificationSummary,
};
pub use context_error::ContextBuildError;
pub use error::ReferenceError;

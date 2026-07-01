//! Shared context and error contracts for the reference interpreter.

pub mod context;
pub mod error;

pub use context::{EvaluationContext, SpecificationContext, SpecificationSummary};
pub use error::ReferenceError;

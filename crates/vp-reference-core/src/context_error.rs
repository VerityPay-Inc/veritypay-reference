//! Errors returned when required evaluation context fields are missing.

/// A required field was not set before [`EvaluationContextBuilder::build`](crate::EvaluationContextBuilder::build).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextBuildError {
    pub field: &'static str,
}

impl ContextBuildError {
    #[must_use]
    pub const fn missing(field: &'static str) -> Self {
        Self { field }
    }
}

impl std::fmt::Display for ContextBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "missing required field: {}", self.field)
    }
}

impl std::error::Error for ContextBuildError {}

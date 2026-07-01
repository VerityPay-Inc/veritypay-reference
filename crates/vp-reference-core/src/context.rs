//! Evaluation and specification context types (placeholder).

/// Session context for an evaluation run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluationContext {
    /// Bound specification version identifier.
    pub spec_version: String,
}

impl EvaluationContext {
    /// Placeholder constructor for workspace bootstrap tests.
    #[must_use]
    pub fn placeholder() -> Self {
        Self {
            spec_version: String::new(),
        }
    }
}

/// Loaded, path-free specification context passed to the interpreter.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SpecificationContext;

impl SpecificationContext {
    /// Placeholder constructor for workspace bootstrap tests.
    #[must_use]
    pub fn placeholder() -> Self {
        Self
    }
}

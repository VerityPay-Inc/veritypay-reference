//! Specification input boundary (placeholder).
//!
//! Filesystem and `vp-spec-model` loading will live here in Milestone B.

use vp_reference_core::SpecificationContext;

/// Specification loading entry point (placeholder).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SpecInput;

impl SpecInput {
    /// Placeholder constructor for workspace bootstrap tests.
    #[must_use]
    pub fn placeholder() -> Self {
        Self
    }

    /// Placeholder load hook returning a path-free context.
    #[must_use]
    pub fn load_placeholder(&self) -> SpecificationContext {
        SpecificationContext::placeholder()
    }
}

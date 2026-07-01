//! Errors returned when required builder fields are missing.

/// A required field was not set before [`build`](crate::ClaimBuilder::build).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildError {
    pub field: &'static str,
}

impl BuildError {
    #[must_use]
    pub const fn missing(field: &'static str) -> Self {
        Self { field }
    }
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "missing required field: {}", self.field)
    }
}

impl std::error::Error for BuildError {}

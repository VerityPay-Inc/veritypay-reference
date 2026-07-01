//! Generic execution and input errors (placeholder).

/// Reference interpreter error outside domain types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferenceError {
    /// Placeholder variant for workspace bootstrap.
    Placeholder,
}

impl ReferenceError {
    /// Placeholder value for workspace bootstrap tests.
    #[must_use]
    pub fn placeholder() -> Self {
        Self::Placeholder
    }
}

impl std::fmt::Display for ReferenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Placeholder => f.write_str("placeholder error"),
        }
    }
}

impl std::error::Error for ReferenceError {}

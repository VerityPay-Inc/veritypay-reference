//! Report formatting (placeholder).

/// Evaluation report for human or machine consumption.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Report;

impl Report {
    /// Placeholder constructor for workspace bootstrap tests.
    #[must_use]
    pub fn placeholder() -> Self {
        Self
    }
}

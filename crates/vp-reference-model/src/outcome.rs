//! Normative verification outcome (VP-TERM-011).

/// Verification outcome vocabulary accepted by `veritypay-spec`.
///
/// No other protocol outcome labels belong in the reference domain model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Outcome {
    Satisfied,
    NotSatisfied,
    Indeterminate,
}

impl Outcome {
    /// Canonical snake_case label for reporting and conformance comparison.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
            Self::NotSatisfied => "not_satisfied",
            Self::Indeterminate => "indeterminate",
        }
    }
}

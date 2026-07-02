//! Evaluation Policy vocabulary (VP-RFC-0004).

/// Protocol-defined strategy for deriving one verification outcome from an evidence set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EvaluationPolicy {
    /// Every applicable evidence envelope must be `satisfied` for aggregate `satisfied`.
    #[default]
    AllRequired,
}

impl EvaluationPolicy {
    /// Canonical policy identifier for reporting and conformance comparison.
    #[must_use]
    pub const fn policy_id(self) -> &'static str {
        match self {
            Self::AllRequired => "ALL_REQUIRED",
        }
    }
}

//! Asserted content that verification rules evaluate.

/// Payload being asserted within a claim.
///
/// Intentionally minimal until accepted `DATA_MODEL` fields are wired in Milestone C.2+.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assertion {
    pub assertion_type: String,
    pub body: String,
}

impl Assertion {
    #[must_use]
    pub fn new(assertion_type: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            assertion_type: assertion_type.into(),
            body: body.into(),
        }
    }
}

//! Material offered to support or challenge an assertion.

/// Content that verification rules consume from evidence.
///
/// Intentionally minimal until accepted `DATA_MODEL` fields are wired in Milestone C.2+.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceContent {
    pub content_type: String,
    pub body: String,
}

impl EvidenceContent {
    #[must_use]
    pub fn new(content_type: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            content_type: content_type.into(),
            body: body.into(),
        }
    }
}

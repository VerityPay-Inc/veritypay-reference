//! Evaluation and specification context types.

/// Counts summarizing loaded specification model data.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SpecificationSummary {
    pub term_count: usize,
    pub rfc_count: usize,
    pub document_count: usize,
    pub reference_edge_count: usize,
}

/// Loaded, path-free specification context passed to the interpreter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecificationContext {
    /// Display identity of the spec checkout (not a live filesystem handle).
    pub spec_root_identity: String,
    pub edition_id: Option<String>,
    pub protocol_version: Option<String>,
    pub summary: SpecificationSummary,
}

impl SpecificationContext {
    /// Placeholder constructor for workspace bootstrap tests.
    #[must_use]
    pub fn placeholder() -> Self {
        Self {
            spec_root_identity: String::new(),
            edition_id: None,
            protocol_version: None,
            summary: SpecificationSummary::default(),
        }
    }
}

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

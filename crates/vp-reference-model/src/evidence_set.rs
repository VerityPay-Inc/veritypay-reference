//! Evidence Set — collection of evidence envelopes for one claim (VP-RFC-0003).
//!
//! Insertion order is preserved for trace and debug output only. Protocol semantics
//! must not depend on evidence ordering.

use crate::build_error::BuildError;
use crate::evidence::Evidence;

/// Unordered protocol collection of [`Evidence`] associated with one claim.
///
/// After construction this type is immutable. Order is retained only for trace/debug.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceSet {
    evidence: Vec<Evidence>,
}

/// Constructs an [`EvidenceSet`].
#[derive(Debug, Default)]
pub struct EvidenceSetBuilder {
    evidence: Vec<Evidence>,
}

impl EvidenceSet {
    /// Empty evidence set (zero envelopes).
    #[must_use]
    pub fn empty() -> Self {
        Self {
            evidence: Vec::new(),
        }
    }

    /// Wraps a vector of evidence envelopes, preserving its order.
    #[must_use]
    pub fn from_vec(evidence: Vec<Evidence>) -> Self {
        Self { evidence }
    }

    #[must_use]
    pub fn builder() -> EvidenceSetBuilder {
        EvidenceSetBuilder::new()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.evidence.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.evidence.is_empty()
    }

    /// Evidence envelopes in insertion order (trace/debug only; not normative ordering).
    #[must_use]
    pub fn evidence(&self) -> &[Evidence] {
        &self.evidence
    }
}

impl EvidenceSetBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn evidence(mut self, evidence: Evidence) -> Self {
        self.evidence.push(evidence);
        self
    }

    #[must_use]
    pub fn evidence_vec(mut self, evidence: impl IntoIterator<Item = Evidence>) -> Self {
        self.evidence.extend(evidence);
        self
    }

    pub fn build(self) -> Result<EvidenceSet, BuildError> {
        Ok(EvidenceSet {
            evidence: self.evidence,
        })
    }
}

//! Root verification result returned by the interpreter.

use crate::build_error::BuildError;
use crate::identifiers::{ClaimId, SpecificationBinding};
use crate::metadata::Metadata;
use crate::outcome::Outcome;
use crate::trace::Trace;

/// Combined verification decision for reporting and conformance comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationResult {
    pub evaluated_claim_id: ClaimId,
    pub outcome: Outcome,
    pub trace: Trace,
    pub metadata: Metadata,
    pub specification_binding: SpecificationBinding,
    pub reasons: Vec<String>,
}

/// Constructs a frozen [`VerificationResult`].
#[derive(Debug, Default)]
pub struct VerificationResultBuilder {
    evaluated_claim_id: Option<ClaimId>,
    outcome: Option<Outcome>,
    trace: Option<Trace>,
    metadata: Metadata,
    specification_binding: SpecificationBinding,
    reasons: Vec<String>,
}

impl VerificationResult {
    #[must_use]
    pub fn builder() -> VerificationResultBuilder {
        VerificationResultBuilder::new()
    }
}

impl VerificationResultBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn evaluated_claim_id(mut self, claim_id: impl Into<ClaimId>) -> Self {
        self.evaluated_claim_id = Some(claim_id.into());
        self
    }

    #[must_use]
    pub fn outcome(mut self, outcome: Outcome) -> Self {
        self.outcome = Some(outcome);
        self
    }

    #[must_use]
    pub fn trace(mut self, trace: Trace) -> Self {
        self.trace = Some(trace);
        self
    }

    #[must_use]
    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = metadata;
        self
    }

    #[must_use]
    pub fn specification_binding(mut self, binding: SpecificationBinding) -> Self {
        self.specification_binding = binding;
        self
    }

    #[must_use]
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reasons.push(reason.into());
        self
    }

    #[must_use]
    pub fn reasons(mut self, reasons: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.reasons.extend(reasons.into_iter().map(Into::into));
        self
    }

    pub fn build(self) -> Result<VerificationResult, BuildError> {
        let evaluated_claim_id = self
            .evaluated_claim_id
            .ok_or_else(|| BuildError::missing("evaluated_claim_id"))?;
        let outcome = self.outcome.ok_or_else(|| BuildError::missing("outcome"))?;
        let trace = self.trace.unwrap_or_default();

        Ok(VerificationResult {
            evaluated_claim_id,
            outcome,
            trace,
            metadata: self.metadata,
            specification_binding: self.specification_binding,
            reasons: self.reasons,
        })
    }
}

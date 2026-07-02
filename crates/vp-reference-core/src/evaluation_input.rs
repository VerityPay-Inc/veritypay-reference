//! Multi-evidence evaluation input for Platform 1.2 semantics (VP-RFC-0003, VP-RFC-0004).
//!
//! The Platform 1.1 public contract remains [`EvaluationContext`] and `Interpreter::evaluate`.
//! [`EvaluationInput`] is consumed by `Interpreter::evaluate_input` for multi-evidence paths.

use vp_reference_model::{Claim, EvaluationPolicy, EvidenceSet};

use crate::context::{EvaluationOptions, SpecificationContext};
use crate::context_error::ContextBuildError;

/// Path-free input bundle for multi-evidence evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluationInput {
    pub specification: SpecificationContext,
    pub claim: Claim,
    pub evidence_set: EvidenceSet,
    pub evaluation_policy: EvaluationPolicy,
    pub options: EvaluationOptions,
}

impl EvaluationInput {
    #[must_use]
    pub fn builder() -> EvaluationInputBuilder {
        EvaluationInputBuilder::new()
    }

    #[must_use]
    pub fn specification(&self) -> &SpecificationContext {
        &self.specification
    }

    #[must_use]
    pub fn claim(&self) -> &Claim {
        &self.claim
    }

    #[must_use]
    pub fn evidence_set(&self) -> &EvidenceSet {
        &self.evidence_set
    }

    #[must_use]
    pub fn evaluation_policy(&self) -> EvaluationPolicy {
        self.evaluation_policy
    }

    #[must_use]
    pub fn options(&self) -> &EvaluationOptions {
        &self.options
    }
}

/// Constructs an [`EvaluationInput`] without filesystem inputs.
#[derive(Debug)]
pub struct EvaluationInputBuilder {
    specification: Option<SpecificationContext>,
    claim: Option<Claim>,
    evidence_set: Option<EvidenceSet>,
    evaluation_policy: EvaluationPolicy,
    options: EvaluationOptions,
}

impl EvaluationInputBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            specification: None,
            claim: None,
            evidence_set: None,
            evaluation_policy: EvaluationPolicy::AllRequired,
            options: EvaluationOptions::default(),
        }
    }

    #[must_use]
    pub fn specification_context(mut self, specification: SpecificationContext) -> Self {
        self.specification = Some(specification);
        self
    }

    #[must_use]
    pub fn claim(mut self, claim: Claim) -> Self {
        self.claim = Some(claim);
        self
    }

    #[must_use]
    pub fn evidence_set(mut self, evidence_set: EvidenceSet) -> Self {
        self.evidence_set = Some(evidence_set);
        self
    }

    #[must_use]
    pub fn evaluation_policy(mut self, evaluation_policy: EvaluationPolicy) -> Self {
        self.evaluation_policy = evaluation_policy;
        self
    }

    #[must_use]
    pub fn options(mut self, options: EvaluationOptions) -> Self {
        self.options = options;
        self
    }

    pub fn build(self) -> Result<EvaluationInput, ContextBuildError> {
        let specification = self
            .specification
            .ok_or_else(|| ContextBuildError::missing("specification"))?;
        let claim = self
            .claim
            .ok_or_else(|| ContextBuildError::missing("claim"))?;
        let evidence_set = self
            .evidence_set
            .ok_or_else(|| ContextBuildError::missing("evidence_set"))?;

        Ok(EvaluationInput {
            specification,
            claim,
            evidence_set,
            evaluation_policy: self.evaluation_policy,
            options: self.options,
        })
    }
}

impl Default for EvaluationInputBuilder {
    fn default() -> Self {
        Self::new()
    }
}

//! Evaluation and specification context types.

use vp_reference_model::{Claim, Evidence};

use crate::context_error::ContextBuildError;

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
    /// Minimal path-free specification context for tests and bootstrap wiring.
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

/// Future-ready evaluation knobs passed with each interpreter invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluationOptions {
    pub deterministic: bool,
    pub trace_enabled: bool,
}

impl Default for EvaluationOptions {
    fn default() -> Self {
        Self {
            deterministic: true,
            trace_enabled: true,
        }
    }
}

/// Single path-free input bundle for one interpreter evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluationContext {
    pub specification: SpecificationContext,
    pub claim: Claim,
    pub evidence: Evidence,
    pub options: EvaluationOptions,
}

impl EvaluationContext {
    #[must_use]
    pub fn builder() -> EvaluationContextBuilder {
        EvaluationContextBuilder::new()
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
    pub fn evidence(&self) -> &Evidence {
        &self.evidence
    }

    #[must_use]
    pub fn options(&self) -> &EvaluationOptions {
        &self.options
    }
}

/// Constructs an [`EvaluationContext`] without filesystem inputs.
#[derive(Debug)]
pub struct EvaluationContextBuilder {
    specification: Option<SpecificationContext>,
    claim: Option<Claim>,
    evidence: Option<Evidence>,
    options: EvaluationOptions,
}

impl EvaluationContextBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            specification: None,
            claim: None,
            evidence: None,
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
    pub fn evidence(mut self, evidence: Evidence) -> Self {
        self.evidence = Some(evidence);
        self
    }

    #[must_use]
    pub fn options(mut self, options: EvaluationOptions) -> Self {
        self.options = options;
        self
    }

    #[must_use]
    pub fn deterministic(mut self, deterministic: bool) -> Self {
        self.options.deterministic = deterministic;
        self
    }

    #[must_use]
    pub fn trace_enabled(mut self, trace_enabled: bool) -> Self {
        self.options.trace_enabled = trace_enabled;
        self
    }

    pub fn build(self) -> Result<EvaluationContext, ContextBuildError> {
        let specification = self
            .specification
            .ok_or_else(|| ContextBuildError::missing("specification"))?;
        let claim = self
            .claim
            .ok_or_else(|| ContextBuildError::missing("claim"))?;
        let evidence = self
            .evidence
            .ok_or_else(|| ContextBuildError::missing("evidence"))?;

        Ok(EvaluationContext {
            specification,
            claim,
            evidence,
            options: self.options,
        })
    }
}

impl Default for EvaluationContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

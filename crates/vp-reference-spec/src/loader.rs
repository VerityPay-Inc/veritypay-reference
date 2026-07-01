//! Load validated specification input through `vp-spec-model`.

use std::path::{Path, PathBuf};

use vp_core::SpecRepository;
use vp_reference_core::{SpecificationContext, SpecificationSummary};
use vp_spec_model::{Specification, SpecificationBuilder};

use crate::error::SpecLoadError;

/// Options for loading a `veritypay-spec` checkout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecificationLoadOptions {
    pub spec_root: PathBuf,
    pub edition_id: Option<String>,
    pub protocol_version: Option<String>,
}

impl SpecificationLoadOptions {
    #[must_use]
    pub fn new(spec_root: impl Into<PathBuf>) -> Self {
        Self {
            spec_root: spec_root.into(),
            edition_id: None,
            protocol_version: None,
        }
    }

    #[must_use]
    pub fn with_edition_id(mut self, edition_id: impl Into<String>) -> Self {
        self.edition_id = Some(edition_id.into());
        self
    }

    #[must_use]
    pub fn with_protocol_version(mut self, protocol_version: impl Into<String>) -> Self {
        self.protocol_version = Some(protocol_version.into());
        self
    }
}

/// Loads specification model data from a validated spec checkout.
#[derive(Debug, Clone, Copy, Default)]
pub struct SpecificationLoader;

impl SpecificationLoader {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    pub fn load(
        &self,
        options: &SpecificationLoadOptions,
    ) -> Result<LoadedSpecification, SpecLoadError> {
        validate_spec_root(&options.spec_root)?;

        let repo = SpecRepository::new(&options.spec_root);
        let specification = SpecificationBuilder::new(&repo)
            .build_registries_and_documents()
            .map_err(SpecLoadError::model_build)?;

        let context = specification_context_from_model(
            repo.spec_root().display().to_string(),
            options,
            &specification,
        );

        Ok(LoadedSpecification {
            context,
            specification,
        })
    }
}

/// Typed specification model plus a path-free context summary.
#[derive(Debug, Clone)]
pub struct LoadedSpecification {
    context: SpecificationContext,
    specification: Specification,
}

impl LoadedSpecification {
    #[must_use]
    pub fn context(&self) -> &SpecificationContext {
        &self.context
    }

    #[must_use]
    pub fn specification(&self) -> &Specification {
        &self.specification
    }
}

fn validate_spec_root(spec_root: &Path) -> Result<(), SpecLoadError> {
    if !spec_root.exists() {
        return Err(SpecLoadError::spec_root_not_found(spec_root.to_path_buf()));
    }
    if !spec_root.is_dir() {
        return Err(SpecLoadError::spec_root_not_directory(
            spec_root.to_path_buf(),
        ));
    }
    Ok(())
}

fn specification_context_from_model(
    spec_root_identity: String,
    options: &SpecificationLoadOptions,
    specification: &Specification,
) -> SpecificationContext {
    SpecificationContext {
        spec_root_identity,
        edition_id: options.edition_id.clone(),
        protocol_version: options.protocol_version.clone(),
        summary: SpecificationSummary {
            term_count: specification.registry_set.terminology.entries().len(),
            rfc_count: specification.registry_set.rfcs.entries().len(),
            document_count: specification.document_corpus.documents().len(),
            reference_edge_count: specification.reference_graph().edges().len(),
        },
    }
}

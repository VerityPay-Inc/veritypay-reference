//! Specification loading errors.

use std::path::PathBuf;

use vp_spec_model::BuildError;

/// Failure loading specification input through `vp-spec-model`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecLoadError {
    SpecRootNotFound { path: PathBuf },
    SpecRootNotDirectory { path: PathBuf },
    ModelBuild { source: BuildError },
}

impl SpecLoadError {
    #[must_use]
    pub fn spec_root_not_found(path: PathBuf) -> Self {
        Self::SpecRootNotFound { path }
    }

    #[must_use]
    pub fn spec_root_not_directory(path: PathBuf) -> Self {
        Self::SpecRootNotDirectory { path }
    }

    #[must_use]
    pub fn model_build(source: BuildError) -> Self {
        Self::ModelBuild { source }
    }
}

impl std::fmt::Display for SpecLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SpecRootNotFound { path } => {
                write!(f, "spec root does not exist: {}", path.display())
            }
            Self::SpecRootNotDirectory { path } => {
                write!(f, "spec path is not a directory: {}", path.display())
            }
            Self::ModelBuild { source } => {
                write!(f, "failed to load specification model: {source}")
            }
        }
    }
}

impl std::error::Error for SpecLoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ModelBuild { source } => Some(source),
            _ => None,
        }
    }
}

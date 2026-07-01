//! Evidence envelope — identity separate from offered content.

use crate::build_error::BuildError;
use crate::evidence_content::EvidenceContent;
use crate::identifiers::{ClaimId, EvidenceId};
use crate::metadata::Metadata;

/// Material offered to support or challenge a claim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evidence {
    pub id: EvidenceId,
    pub claim_id: ClaimId,
    pub content: EvidenceContent,
    pub metadata: Metadata,
}

/// Constructs an [`Evidence`] record linked to a claim.
#[derive(Debug, Default)]
pub struct EvidenceBuilder {
    id: Option<EvidenceId>,
    claim_id: Option<ClaimId>,
    content: Option<EvidenceContent>,
    metadata: Metadata,
}

impl Evidence {
    #[must_use]
    pub fn builder() -> EvidenceBuilder {
        EvidenceBuilder::new()
    }
}

impl EvidenceBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn id(mut self, id: impl Into<EvidenceId>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use]
    pub fn claim_id(mut self, claim_id: impl Into<ClaimId>) -> Self {
        self.claim_id = Some(claim_id.into());
        self
    }

    #[must_use]
    pub fn content(mut self, content: EvidenceContent) -> Self {
        self.content = Some(content);
        self
    }

    #[must_use]
    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = metadata;
        self
    }

    #[must_use]
    pub fn metadata_entry(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let mut builder = Metadata::builder();
        for (existing_key, existing_value) in self.metadata.entries() {
            builder = builder.entry(existing_key.clone(), existing_value.clone());
        }
        self.metadata = builder.entry(key, value).build();
        self
    }

    pub fn build(self) -> Result<Evidence, BuildError> {
        let id = self.id.ok_or_else(|| BuildError::missing("id"))?;
        let claim_id = self
            .claim_id
            .ok_or_else(|| BuildError::missing("claim_id"))?;
        let content = self.content.ok_or_else(|| BuildError::missing("content"))?;

        Ok(Evidence {
            id,
            claim_id,
            content,
            metadata: self.metadata,
        })
    }
}

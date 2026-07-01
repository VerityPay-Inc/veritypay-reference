//! Claim envelope — identity and binding separate from asserted content.

use crate::assertion::Assertion;
use crate::build_error::BuildError;
use crate::identifiers::{ClaimId, SpecificationBinding};
use crate::metadata::Metadata;

/// Verifiable protocol claim input (identity + assertion).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Claim {
    pub id: ClaimId,
    pub subject: String,
    pub assertion: Assertion,
    pub specification_binding: Option<SpecificationBinding>,
    pub metadata: Metadata,
}

/// Constructs a [`Claim`] with readable fixture ergonomics.
#[derive(Debug, Default)]
pub struct ClaimBuilder {
    id: Option<ClaimId>,
    subject: Option<String>,
    assertion: Option<Assertion>,
    specification_binding: Option<SpecificationBinding>,
    metadata: Metadata,
}

impl Claim {
    #[must_use]
    pub fn builder() -> ClaimBuilder {
        ClaimBuilder::new()
    }
}

impl ClaimBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn id(mut self, id: impl Into<ClaimId>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use]
    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    #[must_use]
    pub fn assertion(mut self, assertion: Assertion) -> Self {
        self.assertion = Some(assertion);
        self
    }

    #[must_use]
    pub fn specification_binding(mut self, binding: SpecificationBinding) -> Self {
        self.specification_binding = Some(binding);
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

    pub fn build(self) -> Result<Claim, BuildError> {
        let id = self.id.ok_or_else(|| BuildError::missing("id"))?;
        let subject = self.subject.ok_or_else(|| BuildError::missing("subject"))?;
        let assertion = self
            .assertion
            .ok_or_else(|| BuildError::missing("assertion"))?;

        Ok(Claim {
            id,
            subject,
            assertion,
            specification_binding: self.specification_binding,
            metadata: self.metadata,
        })
    }
}

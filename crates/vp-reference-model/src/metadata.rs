//! Non-normative key/value context attached to domain objects.

use std::collections::BTreeMap;

/// Non-normative context that must never decide protocol truth.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Metadata {
    entries: BTreeMap<String, String>,
}

impl Metadata {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn from_pairs(
        pairs: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        let mut metadata = Self::new();
        for (key, value) in pairs {
            metadata.entries.insert(key.into(), value.into());
        }
        metadata
    }

    #[must_use]
    pub fn entries(&self) -> &BTreeMap<String, String> {
        &self.entries
    }

    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(String::as_str)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Mutable builder for [`Metadata`].
#[derive(Debug, Default)]
pub struct MetadataBuilder {
    entries: BTreeMap<String, String>,
}

impl MetadataBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn entry(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.entries.insert(key.into(), value.into());
        self
    }

    #[must_use]
    pub fn build(self) -> Metadata {
        Metadata {
            entries: self.entries,
        }
    }
}

impl Metadata {
    #[must_use]
    pub fn builder() -> MetadataBuilder {
        MetadataBuilder::new()
    }
}

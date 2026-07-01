//! Stable identifiers for conformance comparison.

use std::fmt;

macro_rules! define_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name(String);

        impl $name {
            #[must_use]
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_owned())
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

define_id! {
    /// Stable identifier for a claim under evaluation.
    ClaimId
}

define_id! {
    /// Stable identifier for an evidence record.
    EvidenceId
}

define_id! {
    /// Stable identifier for a single trace event.
    TraceEventId
}

/// Specification version or edition pin governing an evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SpecificationBinding {
    pub edition_id: Option<String>,
    pub protocol_version: Option<String>,
}

impl SpecificationBinding {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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

define_id! {
    /// Reference to a normative rule in the loaded specification.
    RuleReference
}

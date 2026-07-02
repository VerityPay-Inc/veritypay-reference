//! Pure domain types for the reference interpreter.
//!
//! No parse errors, filesystem types, or I/O concerns belong in this crate.

pub mod assertion;
pub mod build_error;
pub mod claim;
pub mod evaluation_policy;
pub mod evidence;
pub mod evidence_content;
pub mod evidence_set;
pub mod identifiers;
pub mod metadata;
pub mod outcome;
pub mod trace;
pub mod verification_result;

pub use assertion::Assertion;
pub use build_error::BuildError;
pub use claim::{Claim, ClaimBuilder};
pub use evaluation_policy::EvaluationPolicy;
pub use evidence::{Evidence, EvidenceBuilder};
pub use evidence_content::EvidenceContent;
pub use evidence_set::{EvidenceSet, EvidenceSetBuilder};
pub use identifiers::{ClaimId, EvidenceId, RuleReference, SpecificationBinding, TraceEventId};
pub use metadata::{Metadata, MetadataBuilder};
pub use outcome::Outcome;
pub use trace::{Trace, TraceBuilder, TraceEvent};
pub use verification_result::{VerificationResult, VerificationResultBuilder};

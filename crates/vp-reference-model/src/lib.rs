//! Pure domain types for the reference interpreter.
//!
//! No parse errors, filesystem types, or I/O concerns belong in this crate.

pub mod claim;
pub mod evidence;
pub mod identifiers;
pub mod outcome;
pub mod trace;
pub mod verification_result;

pub use claim::Claim;
pub use evidence::Evidence;
pub use identifiers::Identifiers;
pub use outcome::Outcome;
pub use trace::Trace;
pub use verification_result::VerificationResult;

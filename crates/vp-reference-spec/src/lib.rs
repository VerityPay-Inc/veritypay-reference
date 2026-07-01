//! Specification input loading for the reference interpreter.

pub mod error;
pub mod loader;

pub use error::SpecLoadError;
pub use loader::{LoadedSpecification, SpecificationLoadOptions, SpecificationLoader};

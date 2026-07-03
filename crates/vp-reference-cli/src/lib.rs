//! Developer-facing CLI library for the VerityPay reference interpreter.

mod error;
mod input;
mod output;
mod verify;

pub use error::{VerifyError, EXIT_SUCCESS, EXIT_USER_ERROR};
pub use output::OutputFormat;
pub use verify::{run_verify, VerifyOptions, VerifyOutput};

//! Developer-facing CLI library for the VerityPay reference interpreter.

mod error;
mod explain;
mod input;
mod output;
mod serve;
mod verify;

pub use error::{VerifyError, EXIT_SUCCESS, EXIT_USER_ERROR};
pub use input::{ClaimFile, EvidenceFile};
pub use output::OutputFormat;
pub use serve::{
    app, run_serve, HealthResponse, ServeError, ServeOptions, VerifyRequest, SERVICE_VERSION,
};
pub use verify::{run_verify, run_verify_documents, VerifyOptions, VerifyOutput};

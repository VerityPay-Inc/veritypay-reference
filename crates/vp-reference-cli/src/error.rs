//! CLI error types and exit codes.

/// Exit code when verification completes successfully.
pub const EXIT_SUCCESS: i32 = 0;

/// Exit code for invalid user input (missing files, malformed JSON, invalid claim shape).
pub const EXIT_USER_ERROR: i32 = 2;

/// Recoverable user-facing CLI error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifyError {
    message: String,
}

impl VerifyError {
    pub fn user(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for VerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for VerifyError {}

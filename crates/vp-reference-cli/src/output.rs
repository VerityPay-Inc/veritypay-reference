//! Human and JSON verification output.

use serde::Serialize;
use vp_reference_model::{Outcome, Trace, VerificationResult};

use crate::error::VerifyError;

/// Report rendering format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    #[default]
    Human,
    Json,
}

impl OutputFormat {
    pub fn parse(value: &str) -> Result<Self, VerifyError> {
        match value {
            "human" => Ok(Self::Human),
            "json" => Ok(Self::Json),
            _ => Err(VerifyError::user(format!(
                "invalid format '{value}'; expected human or json"
            ))),
        }
    }
}

#[derive(Debug, Serialize)]
struct VerifyJsonOutput<'a> {
    claim_id: &'a str,
    outcome: &'static str,
    reason: String,
    trace: Vec<TraceEventJson<'a>>,
}

#[derive(Debug, Serialize)]
struct TraceEventJson<'a> {
    id: &'a str,
    rule_reference: Option<&'a str>,
    message: &'a str,
}

pub fn render_human(result: &VerificationResult) -> String {
    let symbol = outcome_symbol(result.outcome);
    let mut lines = vec![format!(
        "{symbol} {} {}",
        result.outcome.as_str(),
        result.evaluated_claim_id.as_str()
    )];

    if let Some(reason) = primary_reason(result) {
        lines.push(format!("Reason: {reason}"));
    }

    lines.join("\n")
}

pub fn render_json(result: &VerificationResult) -> Result<String, VerifyError> {
    let payload = VerifyJsonOutput {
        claim_id: result.evaluated_claim_id.as_str(),
        outcome: result.outcome.as_str(),
        reason: primary_reason(result).unwrap_or_default(),
        trace: trace_events_json(&result.trace),
    };

    serde_json::to_string_pretty(&payload)
        .map_err(|error| VerifyError::user(format!("failed to encode JSON output: {error}")))
}

fn outcome_symbol(outcome: Outcome) -> char {
    match outcome {
        Outcome::Satisfied => '✓',
        Outcome::NotSatisfied => '✗',
        Outcome::Indeterminate => '?',
    }
}

fn primary_reason(result: &VerificationResult) -> Option<String> {
    if result.reasons.is_empty() {
        None
    } else {
        Some(result.reasons.join("; "))
    }
}

fn trace_events_json(trace: &Trace) -> Vec<TraceEventJson<'_>> {
    trace
        .events()
        .iter()
        .map(|event| TraceEventJson {
            id: event.id.as_str(),
            rule_reference: event.rule_reference.as_ref().map(|rule| rule.as_str()),
            message: &event.message,
        })
        .collect()
}

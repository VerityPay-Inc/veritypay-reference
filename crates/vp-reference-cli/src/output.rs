//! Human and JSON verification output.

use serde::Serialize;
use vp_reference_core::EvaluationInput;
use vp_reference_model::{Outcome, Trace, VerificationResult};

use crate::error::VerifyError;
use crate::explain::{build_explanation, evidence_summaries};

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

/// Input bundle for rendering verification output.
pub struct VerifyRenderInput<'a> {
    pub evaluation_input: &'a EvaluationInput,
    pub result: &'a VerificationResult,
    pub explain: bool,
}

#[derive(Debug, Serialize)]
struct VerifyJsonOutput<'a> {
    claim_id: &'a str,
    outcome: &'static str,
    reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    assertion_type: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evidence: Option<Vec<EvidenceJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    applied_rules: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation: Option<Vec<String>>,
    trace: Vec<TraceEventJson<'a>>,
}

#[derive(Debug, Serialize)]
struct EvidenceJson {
    evidence_id: String,
    claim_id: String,
    content_type: String,
}

#[derive(Debug, Serialize)]
struct TraceEventJson<'a> {
    id: &'a str,
    rule_reference: Option<&'a str>,
    message: &'a str,
}

pub fn render_human(input: &VerifyRenderInput<'_>) -> String {
    if input.explain {
        render_human_explain(input)
    } else {
        render_human_compact(input.result)
    }
}

pub fn render_json(input: &VerifyRenderInput<'_>) -> Result<String, VerifyError> {
    let explanation = if input.explain {
        Some(build_explanation(input.evaluation_input, input.result))
    } else {
        None
    };

    let payload = VerifyJsonOutput {
        claim_id: input.result.evaluated_claim_id.as_str(),
        outcome: input.result.outcome.as_str(),
        reason: primary_reason(input.result).unwrap_or_default(),
        assertion_type: explanation.as_ref().map(|_| {
            input
                .evaluation_input
                .claim()
                .assertion
                .assertion_type
                .as_str()
        }),
        evidence: explanation.as_ref().map(|_| {
            evidence_summaries(input.evaluation_input)
                .into_iter()
                .map(|summary| EvidenceJson {
                    evidence_id: summary.evidence_id,
                    claim_id: summary.claim_id,
                    content_type: summary.content_type,
                })
                .collect()
        }),
        policy: explanation
            .as_ref()
            .map(|_| input.evaluation_input.evaluation_policy().policy_id()),
        applied_rules: explanation
            .as_ref()
            .map(|value| value.applied_rules.clone()),
        explanation: explanation.as_ref().map(|value| value.steps.clone()),
        trace: trace_events_json(&input.result.trace),
    };

    serde_json::to_string_pretty(&payload)
        .map_err(|error| VerifyError::user(format!("failed to encode JSON output: {error}")))
}

fn render_human_compact(result: &VerificationResult) -> String {
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

fn render_human_explain(input: &VerifyRenderInput<'_>) -> String {
    let result = input.result;
    let evaluation_input = input.evaluation_input;
    let explanation = build_explanation(evaluation_input, result);
    let symbol = outcome_symbol(result.outcome);
    let evidence = evaluation_input
        .evidence_set()
        .evidence()
        .first()
        .expect("verify CLI uses one evidence envelope");

    let mut lines = vec![
        format!(
            "{symbol} {} {}",
            result.outcome.as_str(),
            result.evaluated_claim_id.as_str()
        ),
        String::new(),
        format!(
            "Assertion type: {}",
            evaluation_input.claim().assertion.assertion_type
        ),
        format!("Evidence: {}", evidence.id.as_str()),
        format!(
            "Policy: {}",
            evaluation_input.evaluation_policy().policy_id()
        ),
        String::new(),
        "Applied rules:".to_owned(),
    ];

    for rule in &explanation.applied_rules {
        lines.push(format!("- {rule}"));
    }

    lines.push(String::new());
    lines.push("Explanation:".to_owned());
    for (index, step) in explanation.steps.iter().enumerate() {
        lines.push(format!("{}. {step}", index + 1));
    }

    lines.push(String::new());
    lines.push("Reason:".to_owned());
    if let Some(reason) = primary_reason(result) {
        lines.push(reason);
    }

    lines.join("\n")
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

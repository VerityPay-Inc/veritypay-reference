//! Per-rule outcome fragment returned to the interpreter orchestrator.

use vp_reference_model::{Outcome, RuleReference, TraceEvent};

/// Result of applying a single [`EvaluationRule`](crate::rule::EvaluationRule).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleEvaluation {
    pub outcome: Outcome,
    pub reason: String,
    pub rule_reference: Option<RuleReference>,
    pub trace_events: Vec<TraceEvent>,
    /// When true, the orchestrator continues to the next rule without treating this outcome as final.
    pub continues: bool,
}

impl RuleEvaluation {
    #[must_use]
    pub fn new(outcome: Outcome, reason: impl Into<String>) -> Self {
        Self {
            outcome,
            reason: reason.into(),
            rule_reference: None,
            trace_events: Vec::new(),
            continues: false,
        }
    }

    #[must_use]
    pub fn with_continues(mut self, continues: bool) -> Self {
        self.continues = continues;
        self
    }

    #[must_use]
    pub fn with_rule_reference(mut self, rule_reference: impl Into<RuleReference>) -> Self {
        self.rule_reference = Some(rule_reference.into());
        self
    }

    #[must_use]
    pub fn with_trace_events(mut self, trace_events: Vec<TraceEvent>) -> Self {
        self.trace_events = trace_events;
        self
    }
}

//! Ordered evaluation trace (explanatory, not normative).

use crate::identifiers::{RuleReference, TraceEventId};
use crate::metadata::Metadata;

/// Single step in an evaluation trace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceEvent {
    pub id: TraceEventId,
    pub rule_reference: Option<RuleReference>,
    pub message: String,
    pub metadata: Metadata,
}

impl TraceEvent {
    #[must_use]
    pub fn new(id: impl Into<TraceEventId>, message: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            rule_reference: None,
            message: message.into(),
            metadata: Metadata::new(),
        }
    }

    #[must_use]
    pub fn with_rule_reference(mut self, rule_reference: impl Into<RuleReference>) -> Self {
        self.rule_reference = Some(rule_reference.into());
        self
    }

    #[must_use]
    pub fn with_metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = metadata;
        self
    }
}

/// Ordered trace of how evaluation proceeded.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Trace {
    events: Vec<TraceEvent>,
}

impl Trace {
    #[must_use]
    pub fn builder() -> TraceBuilder {
        TraceBuilder::new()
    }

    #[must_use]
    pub fn events(&self) -> &[TraceEvent] {
        &self.events
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

/// Appends trace events in evaluation order.
#[derive(Debug, Default)]
pub struct TraceBuilder {
    events: Vec<TraceEvent>,
}

impl TraceBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn event(mut self, event: TraceEvent) -> Self {
        self.events.push(event);
        self
    }

    #[must_use]
    pub fn message(mut self, id: impl Into<TraceEventId>, message: impl Into<String>) -> Self {
        self.events.push(TraceEvent::new(id, message));
        self
    }

    #[must_use]
    pub fn build(self) -> Trace {
        Trace {
            events: self.events,
        }
    }
}

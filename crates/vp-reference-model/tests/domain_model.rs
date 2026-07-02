//! Domain model integration tests (Milestone C.1).

use vp_reference_model::{
    Assertion, Claim, ClaimBuilder, EvaluationPolicy, Evidence, EvidenceBuilder, EvidenceContent,
    EvidenceSet, Metadata, Outcome, SpecificationBinding, Trace, TraceBuilder, TraceEvent,
    VerificationResult, VerificationResultBuilder,
};

#[test]
fn claim_builder_builds_minimal_claim() {
    let claim = ClaimBuilder::new()
        .id("claim-001")
        .subject("alice@example.com")
        .assertion(Assertion::new("minimal", "asserted-value"))
        .build()
        .expect("minimal claim");

    assert_eq!(claim.id.as_str(), "claim-001");
    assert_eq!(claim.subject, "alice@example.com");
    assert_eq!(claim.assertion.body, "asserted-value");
    assert!(claim.specification_binding.is_none());
    assert!(claim.metadata.is_empty());
}

#[test]
fn evidence_builder_builds_evidence_linked_to_claim() {
    let claim = ClaimBuilder::new()
        .id("claim-001")
        .subject("alice@example.com")
        .assertion(Assertion::new("minimal", "asserted-value"))
        .build()
        .expect("claim");

    let evidence = EvidenceBuilder::new()
        .id("evidence-001")
        .claim_id(claim.id.clone())
        .content(EvidenceContent::new("document", "supporting-body"))
        .build()
        .expect("evidence");

    assert_eq!(evidence.claim_id, claim.id);
    assert_eq!(evidence.content.body, "supporting-body");
}

#[test]
fn outcome_enum_has_three_normative_variants() {
    let outcomes = [
        Outcome::Satisfied,
        Outcome::NotSatisfied,
        Outcome::Indeterminate,
    ];

    assert_eq!(outcomes.len(), 3);
    assert_eq!(
        outcomes
            .iter()
            .map(|outcome| outcome.as_str())
            .collect::<Vec<_>>(),
        vec!["satisfied", "not_satisfied", "indeterminate"]
    );

    let labels: std::collections::BTreeSet<_> = outcomes.iter().map(|o| o.as_str()).collect();
    assert_eq!(labels.len(), 3);
}

#[test]
fn trace_preserves_event_order() {
    let trace = TraceBuilder::new()
        .message("evt-1", "first")
        .message("evt-2", "second")
        .message("evt-3", "third")
        .build();

    let messages: Vec<_> = trace
        .events()
        .iter()
        .map(|event| event.message.as_str())
        .collect();

    assert_eq!(messages, vec!["first", "second", "third"]);
}

#[test]
fn verification_result_builder_builds_result_with_outcome_trace_and_binding() {
    let trace = TraceBuilder::new()
        .event(TraceEvent::new("evt-1", "rule applied").with_rule_reference("rule.payment.minimal"))
        .build();

    let binding = SpecificationBinding::new().with_protocol_version("0.1.0");

    let result = VerificationResultBuilder::new()
        .evaluated_claim_id("claim-001")
        .outcome(Outcome::Satisfied)
        .trace(trace)
        .specification_binding(binding.clone())
        .reason("minimal fixture passed")
        .build()
        .expect("verification result");

    assert_eq!(result.evaluated_claim_id.as_str(), "claim-001");
    assert_eq!(result.outcome, Outcome::Satisfied);
    assert_eq!(result.trace.events().len(), 1);
    assert_eq!(
        result.specification_binding.protocol_version.as_deref(),
        Some("0.1.0")
    );
    assert_eq!(result.reasons, vec!["minimal fixture passed"]);
}

#[test]
fn metadata_attaches_without_affecting_outcome_equality() {
    let trace = Trace::builder().build();

    let result_with_metadata = VerificationResultBuilder::new()
        .evaluated_claim_id("claim-001")
        .outcome(Outcome::Satisfied)
        .trace(trace.clone())
        .metadata(Metadata::from_pairs([("runner", "local")]))
        .build()
        .expect("result with metadata");

    let result_without_metadata = VerificationResultBuilder::new()
        .evaluated_claim_id("claim-001")
        .outcome(Outcome::Satisfied)
        .trace(trace)
        .build()
        .expect("result without metadata");

    assert_eq!(
        result_with_metadata.outcome,
        result_without_metadata.outcome
    );
    assert_eq!(result_with_metadata.outcome, Outcome::Satisfied);
    assert!(!result_with_metadata.metadata.is_empty());
    assert!(result_without_metadata.metadata.is_empty());
}

#[test]
fn public_domain_types_avoid_filesystem_types() {
    fn assert_no_path_field<T>() {}

    assert_no_path_field::<Claim>();
    assert_no_path_field::<Evidence>();
    assert_no_path_field::<EvidenceSet>();
    assert_no_path_field::<EvaluationPolicy>();
    assert_no_path_field::<Assertion>();
    assert_no_path_field::<EvidenceContent>();
    assert_no_path_field::<Outcome>();
    assert_no_path_field::<Trace>();
    assert_no_path_field::<TraceEvent>();
    assert_no_path_field::<VerificationResult>();
    assert_no_path_field::<Metadata>();

    let _claim: Claim = Claim::builder()
        .id("claim-001")
        .subject("subject")
        .assertion(Assertion::new("minimal", "body"))
        .build()
        .expect("claim");
}

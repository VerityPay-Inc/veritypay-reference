//! Platform 1.2 domain model groundwork (VP-RFC-0003, VP-RFC-0004).

use vp_reference_model::{
    Assertion, ClaimBuilder, EvaluationPolicy, Evidence, EvidenceBuilder, EvidenceContent,
    EvidenceSet, EvidenceSetBuilder,
};

fn sample_evidence(id: &str, claim_id: &str, body: &str) -> Evidence {
    EvidenceBuilder::new()
        .id(id)
        .claim_id(claim_id)
        .content(EvidenceContent::new("document", body))
        .build()
        .expect("evidence")
}

#[test]
fn evidence_set_empty_has_zero_envelopes() {
    let set = EvidenceSet::empty();

    assert!(set.is_empty());
    assert_eq!(set.len(), 0);
    assert!(set.evidence().is_empty());
}

#[test]
fn evidence_set_from_vec_preserves_multiple_items() {
    let first = sample_evidence("evidence-001", "claim-001", "alpha");
    let second = sample_evidence("evidence-002", "claim-001", "beta");

    let set = EvidenceSet::from_vec(vec![first.clone(), second.clone()]);

    assert_eq!(set.len(), 2);
    assert_eq!(set.evidence(), &[first, second]);
}

#[test]
fn evidence_set_builder_preserves_insertion_order() {
    let first = sample_evidence("evidence-a", "claim-001", "first");
    let second = sample_evidence("evidence-b", "claim-001", "second");
    let third = sample_evidence("evidence-c", "claim-001", "third");

    let set = EvidenceSetBuilder::new()
        .evidence(first.clone())
        .evidence(second.clone())
        .evidence(third.clone())
        .build()
        .expect("evidence set");

    let ids: Vec<_> = set
        .evidence()
        .iter()
        .map(|evidence| evidence.id.as_str())
        .collect();

    assert_eq!(ids, vec!["evidence-a", "evidence-b", "evidence-c"]);
}

#[test]
fn evaluation_policy_all_required_label_is_all_required() {
    assert_eq!(EvaluationPolicy::AllRequired.policy_id(), "ALL_REQUIRED");
}

#[test]
fn evidence_set_builder_allows_empty_set() {
    let set = EvidenceSetBuilder::new()
        .build()
        .expect("empty evidence set");

    assert!(set.is_empty());
}

#[test]
fn claim_builder_still_builds_minimal_claim_for_platform_1_1_compat() {
    let claim = ClaimBuilder::new()
        .id("claim-001")
        .subject("alice@example.com")
        .assertion(Assertion::new("minimal", "asserted-value"))
        .build()
        .expect("minimal claim");

    assert_eq!(claim.id.as_str(), "claim-001");
}

//! Workspace integration tests.

#[test]
fn workspace_crates_are_linkable() {
    use vp_reference_core::{EvaluationContext, ReferenceError, SpecificationContext};
    use vp_reference_interpreter::Interpreter;
    use vp_reference_model::{
        Assertion, ClaimBuilder, EvidenceBuilder, EvidenceContent, Outcome, Trace,
        VerificationResultBuilder,
    };
    use vp_reference_report::Report;
    use vp_reference_spec::{SpecificationLoadOptions, SpecificationLoader};

    let claim = ClaimBuilder::new()
        .id("claim-001")
        .subject("subject")
        .assertion(Assertion::new("minimal", "body"))
        .build()
        .expect("claim");

    let evidence = EvidenceBuilder::new()
        .id("evidence-001")
        .claim_id(claim.id.clone())
        .content(EvidenceContent::new("document", "body"))
        .build()
        .expect("evidence");

    let result = VerificationResultBuilder::new()
        .evaluated_claim_id(claim.id.clone())
        .outcome(Outcome::Indeterminate)
        .trace(Trace::builder().build())
        .build()
        .expect("result");

    let _ = (
        EvaluationContext::placeholder(),
        SpecificationContext::placeholder(),
        ReferenceError::placeholder(),
        claim,
        evidence,
        result,
        Interpreter::placeholder(),
        Report::placeholder(),
        SpecificationLoader::new(),
        SpecificationLoadOptions::new("."),
    );
}

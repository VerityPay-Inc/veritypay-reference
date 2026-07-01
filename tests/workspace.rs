//! Workspace integration tests.

#[test]
fn workspace_crates_are_linkable() {
    use vp_reference_core::{EvaluationContext, ReferenceError, SpecificationContext};
    use vp_reference_interpreter::Interpreter;
    use vp_reference_model::{Claim, Evidence, Identifiers, Outcome, Trace, VerificationResult};
    use vp_reference_report::Report;
    use vp_reference_spec::{SpecificationLoadOptions, SpecificationLoader};

    let _ = (
        EvaluationContext::placeholder(),
        SpecificationContext::placeholder(),
        ReferenceError::placeholder(),
        Claim,
        Evidence,
        Outcome,
        VerificationResult,
        Trace,
        Identifiers,
        Interpreter::placeholder(),
        Report::placeholder(),
        SpecificationLoader::new(),
        SpecificationLoadOptions::new("."),
    );
}

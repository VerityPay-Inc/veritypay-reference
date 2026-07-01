//! Temporary minimal body-equality rule per ADR-0004.

use vp_reference_core::EvaluationContext;
use vp_reference_model::Outcome;

use crate::rule::EvaluationRule;
use crate::rule_evaluation::RuleEvaluation;

/// Engineering rule reference label (not normative spec text).
pub const MINIMAL_BODY_EQUALITY_RULE_REFERENCE: &str = "vp-ref-minimal.body-equality";

/// Compares `claim.assertion.body` to `evidence.content.body` per ADR-0004.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MinimalBodyEqualityRule;

impl EvaluationRule for MinimalBodyEqualityRule {
    fn evaluate(&self, context: &EvaluationContext) -> RuleEvaluation {
        let claim = context.claim();
        let evidence = context.evidence();

        if evidence.claim_id != claim.id {
            return RuleEvaluation::new(
                Outcome::Indeterminate,
                "evidence is not linked to the claim under evaluation",
            )
            .with_rule_reference(MINIMAL_BODY_EQUALITY_RULE_REFERENCE);
        }

        let assertion_body = &claim.assertion.body;
        let evidence_body = &evidence.content.body;

        if evidence_body.is_empty() {
            return RuleEvaluation::new(Outcome::Indeterminate, "evidence content body is empty")
                .with_rule_reference(MINIMAL_BODY_EQUALITY_RULE_REFERENCE);
        }

        if evidence_body == assertion_body {
            RuleEvaluation::new(
                Outcome::Satisfied,
                "assertion body matches evidence content body",
            )
            .with_rule_reference(MINIMAL_BODY_EQUALITY_RULE_REFERENCE)
        } else {
            RuleEvaluation::new(
                Outcome::NotSatisfied,
                "assertion body does not match evidence content body",
            )
            .with_rule_reference(MINIMAL_BODY_EQUALITY_RULE_REFERENCE)
        }
    }
}

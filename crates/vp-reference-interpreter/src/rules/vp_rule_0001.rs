//! Implementation of VP-RULE-0001 (RFC-0001).

use vp_reference_core::EvaluationContext;
use vp_reference_model::Outcome;

use crate::rule::EvaluationRule;
use crate::rule_evaluation::RuleEvaluation;

/// Normative rule identifier from [VP-RFC-0001](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0001-minimal-claim-evidence-semantics.md).
pub const VP_RULE_0001_REFERENCE: &str = "VP-RULE-0001";

/// Assertion Body Evidence Match — compares `claim.assertion.body` to `evidence.content.body`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VpRule0001;

impl EvaluationRule for VpRule0001 {
    fn evaluate(&self, context: &EvaluationContext) -> RuleEvaluation {
        let claim = context.claim();
        let evidence = context.evidence();

        if evidence.claim_id != claim.id {
            return RuleEvaluation::new(
                Outcome::Indeterminate,
                "Evidence is not linked to the claim under evaluation (VP-RULE-0001)",
            )
            .with_rule_reference(VP_RULE_0001_REFERENCE);
        }

        let assertion_body = &claim.assertion.body;
        let evidence_body = &evidence.content.body;

        if evidence_body.is_empty() {
            return RuleEvaluation::new(
                Outcome::Indeterminate,
                "Evidence content body is empty (VP-RULE-0001)",
            )
            .with_rule_reference(VP_RULE_0001_REFERENCE);
        }

        if evidence_body == assertion_body {
            RuleEvaluation::new(
                Outcome::Satisfied,
                "Assertion body matches evidence body (VP-RULE-0001)",
            )
            .with_rule_reference(VP_RULE_0001_REFERENCE)
        } else {
            RuleEvaluation::new(
                Outcome::NotSatisfied,
                "Assertion body does not match evidence body (VP-RULE-0001)",
            )
            .with_rule_reference(VP_RULE_0001_REFERENCE)
        }
    }
}

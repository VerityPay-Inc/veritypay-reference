//! Implementation of VP-RULE-0002 (RFC-0002).

use vp_reference_core::EvaluationContext;
use vp_reference_model::Outcome;

use crate::rule::EvaluationRule;
use crate::rule_evaluation::RuleEvaluation;

/// Normative rule identifier from [VP-RFC-0002](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0002-claim-identity-binding.md).
pub const VP_RULE_0002_REFERENCE: &str = "VP-RULE-0002";

/// Evidence Claim Binding — decides whether evidence is applicable to the claim under evaluation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VpRule0002;

impl EvaluationRule for VpRule0002 {
    fn evaluate(&self, context: &EvaluationContext) -> RuleEvaluation {
        let claim = context.claim();
        let evidence = context.evidence();

        if claim.id.as_str().is_empty() {
            return RuleEvaluation::new(Outcome::Indeterminate, "Claim id is empty (VP-RULE-0002)")
                .with_rule_reference(VP_RULE_0002_REFERENCE);
        }

        if evidence.claim_id.as_str().is_empty() {
            return RuleEvaluation::new(
                Outcome::Indeterminate,
                "Evidence claim id is empty (VP-RULE-0002)",
            )
            .with_rule_reference(VP_RULE_0002_REFERENCE);
        }

        if evidence.claim_id != claim.id {
            return RuleEvaluation::new(
                Outcome::Indeterminate,
                "Evidence claim id does not match claim id (VP-RULE-0002)",
            )
            .with_rule_reference(VP_RULE_0002_REFERENCE);
        }

        RuleEvaluation::new(
            Outcome::Indeterminate,
            "Evidence is bound to the claim under evaluation (VP-RULE-0002)",
        )
        .with_rule_reference(VP_RULE_0002_REFERENCE)
        .with_continues(true)
    }
}

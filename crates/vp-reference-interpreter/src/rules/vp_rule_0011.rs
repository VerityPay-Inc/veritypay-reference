//! Implementation of VP-RULE-0011 (VP-RFC-0011).

use vp_reference_core::EvaluationContext;
use vp_reference_model::Outcome;

use crate::rule::EvaluationRule;
use crate::rule_evaluation::RuleEvaluation;
use crate::text_normalization::{try_normalize_text_bytes, NormalizationError};

/// Normative rule identifier from [VP-RFC-0011](https://github.com/VerityPay-Inc/veritypay-spec/blob/main/rfcs/0011-normalized-text-assertion.md).
pub const VP_RULE_0011_REFERENCE: &str = "VP-RULE-0011";

/// Normalized Text Equality — compares bodies after the Platform 1.3 normalization pipeline.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VpRule0011;

impl EvaluationRule for VpRule0011 {
    fn evaluate(&self, context: &EvaluationContext) -> RuleEvaluation {
        let claim = context.claim();
        let evidence = context.evidence();

        if evidence.claim_id != claim.id {
            return RuleEvaluation::new(
                Outcome::Indeterminate,
                "Evidence is not linked to the claim under evaluation (VP-RULE-0011)",
            )
            .with_rule_reference(VP_RULE_0011_REFERENCE);
        }

        let assertion_body = &claim.assertion.body;
        let evidence_body = &evidence.content.body;

        if evidence_body.is_empty() {
            return RuleEvaluation::new(
                Outcome::Indeterminate,
                "Evidence content body is empty (VP-RULE-0011)",
            )
            .with_rule_reference(VP_RULE_0011_REFERENCE);
        }

        let normalized_assertion = match try_normalize_text_bytes(assertion_body.as_bytes()) {
            Ok(value) => value,
            Err(NormalizationError::InvalidUtf8) => {
                return RuleEvaluation::new(
                    Outcome::Indeterminate,
                    "Assertion body is not valid UTF-8 (VP-RULE-0011)",
                )
                .with_rule_reference(VP_RULE_0011_REFERENCE);
            }
        };

        let normalized_evidence = match try_normalize_text_bytes(evidence_body.as_bytes()) {
            Ok(value) => value,
            Err(NormalizationError::InvalidUtf8) => {
                return RuleEvaluation::new(
                    Outcome::Indeterminate,
                    "Evidence content body is not valid UTF-8 (VP-RULE-0011)",
                )
                .with_rule_reference(VP_RULE_0011_REFERENCE);
            }
        };

        if normalized_evidence.is_empty() {
            return RuleEvaluation::new(
                Outcome::Indeterminate,
                "Evidence content body is empty after normalization (VP-RULE-0011)",
            )
            .with_rule_reference(VP_RULE_0011_REFERENCE);
        }

        if normalized_evidence == normalized_assertion {
            RuleEvaluation::new(
                Outcome::Satisfied,
                "Normalized assertion body matches normalized evidence body (VP-RULE-0011)",
            )
            .with_rule_reference(VP_RULE_0011_REFERENCE)
        } else {
            RuleEvaluation::new(
                Outcome::NotSatisfied,
                "Normalized assertion body does not match normalized evidence body (VP-RULE-0011)",
            )
            .with_rule_reference(VP_RULE_0011_REFERENCE)
        }
    }
}

//! ALL_REQUIRED aggregation per VP-RFC-0004.

use vp_reference_model::Outcome;

/// Aggregates per-envelope outcomes under the **`ALL_REQUIRED`** policy.
#[must_use]
pub fn aggregate_all_required(per_envelope_outcomes: &[Outcome]) -> Outcome {
    if per_envelope_outcomes.is_empty() {
        return Outcome::Indeterminate;
    }
    if per_envelope_outcomes.contains(&Outcome::NotSatisfied) {
        return Outcome::NotSatisfied;
    }
    if per_envelope_outcomes.contains(&Outcome::Indeterminate) {
        return Outcome::Indeterminate;
    }
    Outcome::Satisfied
}

/// Human-readable reason for an aggregated **`ALL_REQUIRED`** outcome.
#[must_use]
pub fn all_required_aggregation_reason(aggregated: Outcome, envelope_count: usize) -> String {
    if envelope_count == 0 {
        return "Evidence set is empty; ALL_REQUIRED yields indeterminate".to_owned();
    }

    match aggregated {
        Outcome::Satisfied => format!(
            "All {envelope_count} applicable evidence envelope(s) satisfied (ALL_REQUIRED)"
        ),
        Outcome::NotSatisfied => {
            "At least one applicable evidence envelope is not_satisfied (ALL_REQUIRED)".to_owned()
        }
        Outcome::Indeterminate => "At least one applicable evidence envelope is indeterminate with no not_satisfied (ALL_REQUIRED)".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_set_is_indeterminate() {
        assert_eq!(aggregate_all_required(&[]), Outcome::Indeterminate);
    }

    #[test]
    fn all_satisfied_is_satisfied() {
        assert_eq!(
            aggregate_all_required(&[Outcome::Satisfied, Outcome::Satisfied]),
            Outcome::Satisfied
        );
    }

    #[test]
    fn any_not_satisfied_dominates() {
        assert_eq!(
            aggregate_all_required(&[Outcome::Satisfied, Outcome::NotSatisfied]),
            Outcome::NotSatisfied
        );
        assert_eq!(
            aggregate_all_required(&[Outcome::NotSatisfied, Outcome::Satisfied]),
            Outcome::NotSatisfied
        );
    }

    #[test]
    fn indeterminate_without_not_satisfied() {
        assert_eq!(
            aggregate_all_required(&[Outcome::Satisfied, Outcome::Indeterminate]),
            Outcome::Indeterminate
        );
    }
}

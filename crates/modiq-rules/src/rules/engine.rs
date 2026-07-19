use modiq_runtime::assessment::{Evidence, Finding, Recommendation};

/// The result of evaluating Evidence against a single deterministic Rule:
/// one Finding and one Recommendation derived from it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleOutcome {
    pub finding: Finding,
    pub recommendation: Recommendation,
}

/// Coordinates deterministic rule execution.
pub struct RuleEngine;

impl RuleEngine {
    /// Evaluates the given Evidence using a single, representative
    /// deterministic Rule (RuleEngine.md: Evidence Evaluation, Finding
    /// Generation, Recommendation Generation).
    ///
    /// Returns `Some` when Evidence exists to evaluate, and `None`
    /// otherwise. Identical input always produces identical output.
    pub fn evaluate(&self, evidence: &[Evidence]) -> Option<RuleOutcome> {
        if evidence.is_empty() {
            return None;
        }

        Some(RuleOutcome {
            finding: Finding,
            recommendation: Recommendation,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use modiq_runtime::assessment::EvidenceCategory;

    fn sample_evidence() -> Evidence {
        Evidence::new(EvidenceCategory::FileStructureAnalysis, "sample evidence")
            .expect("category and description are valid")
    }

    #[test]
    fn evaluate_returns_none_for_no_evidence() {
        let engine = RuleEngine;

        assert_eq!(engine.evaluate(&[]), None);
    }

    #[test]
    fn evaluate_produces_a_finding_and_recommendation_for_evidence() {
        let engine = RuleEngine;

        let outcome = engine
            .evaluate(&[sample_evidence()])
            .expect("evidence was provided");

        assert_eq!(outcome.finding, Finding);
        assert_eq!(outcome.recommendation, Recommendation);
    }

    #[test]
    fn evaluate_is_deterministic_for_identical_input() {
        let engine = RuleEngine;
        let evidence = [sample_evidence(), sample_evidence()];

        let first = engine.evaluate(&evidence);
        let second = engine.evaluate(&evidence);

        assert_eq!(first, second);
    }
}

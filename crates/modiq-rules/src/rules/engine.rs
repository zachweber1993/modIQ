use modiq_runtime::assessment::{
    Evidence, Finding, FindingSeverity, Recommendation, RuleReference,
};

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
    /// otherwise. The resulting Finding references every evaluated
    /// Evidence item by id. Finding content is identical for identical
    /// input; each Finding's own identity is freshly assigned per
    /// evaluation, by the same convention as every other Runtime
    /// Domain identity.
    pub fn evaluate(&self, evidence: &[Evidence]) -> Option<RuleOutcome> {
        if evidence.is_empty() {
            return None;
        }

        let finding = Finding::new(
            FindingSeverity::Informational,
            "Evidence was collected for this Assessment.",
            evidence.iter().map(Evidence::id).collect(),
            RuleReference::new("evidence-presence-rule"),
        )
        .expect("severity, description, and rule reference are valid");

        let recommendation = Recommendation::new(
            "Review the collected evidence and address any issues found.",
            vec![finding.id()],
            None,
        )
        .expect("action is valid");

        Some(RuleOutcome {
            finding,
            recommendation,
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
        let evidence = sample_evidence();
        let evidence_id = evidence.id();

        let outcome = engine.evaluate(&[evidence]).expect("evidence was provided");

        assert_eq!(outcome.finding.severity(), FindingSeverity::Informational);
        assert!(!outcome.finding.description().is_empty());
        assert_eq!(outcome.finding.evidence_ids(), &[evidence_id]);
        assert_eq!(
            outcome.finding.rule_reference().identifier(),
            "evidence-presence-rule"
        );
        assert!(!outcome.recommendation.action().is_empty());
        assert_eq!(
            outcome.recommendation.finding_ids(),
            &[outcome.finding.id()]
        );
        assert_eq!(outcome.recommendation.repair_recipe_reference(), None);
    }

    #[test]
    fn evaluate_is_deterministic_for_identical_input() {
        let engine = RuleEngine;
        let evidence = [sample_evidence(), sample_evidence()];

        let first = engine.evaluate(&evidence).expect("evidence was provided");
        let second = engine.evaluate(&evidence).expect("evidence was provided");

        // Each evaluation freshly assigns Finding identity by design
        // (mirroring AssessmentId/EvidenceId); determinism is judged by
        // content, not by incidental identity.
        assert_eq!(first.finding.severity(), second.finding.severity());
        assert_eq!(first.finding.description(), second.finding.description());
        assert_eq!(first.finding.evidence_ids(), second.finding.evidence_ids());
        assert_eq!(
            first.finding.rule_reference(),
            second.finding.rule_reference()
        );
        assert_eq!(
            first.recommendation.action(),
            second.recommendation.action()
        );
        assert_eq!(
            first.recommendation.finding_ids().len(),
            second.recommendation.finding_ids().len()
        );
        assert_eq!(
            first.recommendation.repair_recipe_reference(),
            second.recommendation.repair_recipe_reference()
        );
    }
}

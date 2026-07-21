use modiq_runtime::assessment::{
    Evidence, Finding, FindingSeverity, Recommendation, RuleReference,
};

use super::engine::RuleOutcome;

/// The Evidence Presence Rule — the platform's original Rule, unchanged
/// in behavior since Sprint 1.
///
/// Fires unconditionally whenever any Evidence exists, regardless of
/// category or content. Extracted into its own unit at Sprint 5 Phase
/// 3 so `RuleEngine::evaluate` can dispatch to it and
/// `StructuralDuplicationRule` (Phase 2) by explicit declaration order
/// (GOV-012), the same shape every concrete Rule now takes.
pub struct EvidencePresenceRule;

impl EvidencePresenceRule {
    /// Returns `None` for empty Evidence; otherwise a single
    /// `Informational` Finding (`DataModel.md`: Finding Severity —
    /// "a neutral, factual observation... carrying no implication that
    /// anything is wrong") referencing every Evidence item given, and
    /// a Recommendation to review it.
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
    fn returns_none_for_no_evidence() {
        let rule = EvidencePresenceRule;

        assert_eq!(rule.evaluate(&[]), None);
    }

    #[test]
    fn produces_an_informational_finding_for_any_evidence() {
        let rule = EvidencePresenceRule;
        let evidence = sample_evidence();
        let evidence_id = evidence.id();

        let outcome = rule.evaluate(&[evidence]).expect("evidence was provided");

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
    fn is_deterministic_for_identical_input() {
        let rule = EvidencePresenceRule;
        let evidence = [sample_evidence(), sample_evidence()];

        let first = rule.evaluate(&evidence).expect("evidence was provided");
        let second = rule.evaluate(&evidence).expect("evidence was provided");

        // Each evaluation freshly assigns Finding/Recommendation
        // identity by design; determinism is judged by content, not by
        // incidental identity.
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
            first.recommendation.repair_recipe_reference(),
            second.recommendation.repair_recipe_reference()
        );
    }
}

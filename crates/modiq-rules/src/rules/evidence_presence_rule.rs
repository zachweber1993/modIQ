use modiq_runtime::assessment::{
    Evidence, Finding, FindingSeverity, FindingStatus, ModHealthDimension, Recommendation,
    RuleReference,
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
///
/// Assigned `ModHealthDimension::EngineeringQuality` (Initiative 3,
/// Item 2, frozen mapping): this Rule is a general, content-neutral
/// observability check, not specific to any of the other five
/// dimensions.
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
            "Evidence collected",
            "Evidence was collected for this Assessment.",
            ModHealthDimension::EngineeringQuality,
            FindingStatus::Final,
            evidence.iter().map(Evidence::id).collect(),
            RuleReference::new("evidence-presence-rule"),
        )
        .expect("severity, title, summary, and rule reference are valid");

        let recommendation = Recommendation::new(
            "Review the collected evidence and address any issues found.",
            vec![finding.id()],
            None,
            Vec::new(),
        )
        .expect("action is valid");

        Some(RuleOutcome {
            finding,
            recommendation: Some(recommendation),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use modiq_runtime::assessment::EvidenceCategory;

    fn sample_evidence() -> Evidence {
        Evidence::new(
            EvidenceCategory::FileStructureAnalysis,
            "sample evidence",
            None,
            None,
            None,
        )
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
        assert!(!outcome.finding.title().is_empty());
        assert!(!outcome.finding.summary().is_empty());
        assert_eq!(
            outcome.finding.mod_health_dimension(),
            ModHealthDimension::EngineeringQuality
        );
        assert_eq!(outcome.finding.status(), FindingStatus::Final);
        assert_eq!(outcome.finding.evidence_ids(), &[evidence_id]);
        assert_eq!(
            outcome.finding.rule_reference().identifier(),
            "evidence-presence-rule"
        );
        let recommendation = outcome.recommendation.expect("this Rule always recommends");
        assert!(!recommendation.action().is_empty());
        assert_eq!(recommendation.finding_ids(), &[outcome.finding.id()]);
        assert_eq!(recommendation.repair_recipe_reference(), None);
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
        assert_eq!(first.finding.title(), second.finding.title());
        assert_eq!(first.finding.summary(), second.finding.summary());
        assert_eq!(first.finding.evidence_ids(), second.finding.evidence_ids());
        assert_eq!(
            first.finding.rule_reference(),
            second.finding.rule_reference()
        );
        let first_recommendation = first.recommendation.expect("this Rule always recommends");
        let second_recommendation = second.recommendation.expect("this Rule always recommends");
        assert_eq!(
            first_recommendation.action(),
            second_recommendation.action()
        );
        assert_eq!(
            first_recommendation.repair_recipe_reference(),
            second_recommendation.repair_recipe_reference()
        );
    }
}

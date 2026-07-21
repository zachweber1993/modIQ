use modiq_runtime::assessment::{
    Evidence, EvidenceCategory, Finding, FindingSeverity, Recommendation, RuleReference,
};

use super::engine::RuleOutcome;

/// The Structural Duplication Rule (GOV-012, Sprint 5 Phase 2).
///
/// Sprint 5's second concrete Rule: evaluates only
/// `EvidenceCategory::StructuralDuplication` Evidence — the fact,
/// recorded by `ArchiveCollector` (Sprint 4 Phase 3C, GOV-011), that an
/// archive's central directory could not fully resolve every entry due
/// to a duplicate name. Every other Evidence category is ignored by
/// this Rule; per GOV-012 (Question 3), Rules compose independently —
/// this Rule's applicability is judged solely on Evidence category
/// match, never on whether another Rule also matches the same or
/// different Evidence.
///
/// Not yet reachable from `RuleEngine::evaluate` — wiring multiple
/// Rules together is Sprint 5 Phase 3, not yet authorized. This Rule
/// is a complete, independently tested unit today, exactly as
/// `ArchiveReader`/`ArchiveEvidenceBuilder` were real and tested before
/// `AssessmentService` routing existed to reach them (Sprint 4 Phases
/// 3A/3B).
pub struct StructuralDuplicationRule;

impl StructuralDuplicationRule {
    /// Evaluates the given Evidence for `StructuralDuplication` items.
    ///
    /// Assigns `FindingSeverity::Warning` (`DataModel.md`: Finding
    /// Severity), not `Error`: a well-formed archive containing
    /// duplicate entry names is a genuine, Evidence-supported
    /// reliability concern (which physical entry a reader actually
    /// extracts is not guaranteed by the archive format itself) but is
    /// not, by itself, conclusive proof the mod is broken.
    ///
    /// Returns `None` when no `StructuralDuplication` Evidence is
    /// present. When present, references every matching Evidence item
    /// by id — not just the first — though `ArchiveCollector` today
    /// produces at most one such item per archive.
    pub fn evaluate(&self, evidence: &[Evidence]) -> Option<RuleOutcome> {
        let matching_ids: Vec<_> = evidence
            .iter()
            .filter(|item| item.category() == EvidenceCategory::StructuralDuplication)
            .map(Evidence::id)
            .collect();

        if matching_ids.is_empty() {
            return None;
        }

        let finding = Finding::new(
            FindingSeverity::Warning,
            "The archive contains duplicate entry names; collection could not fully resolve \
             every entry, so which content a reader actually extracts is not guaranteed.",
            matching_ids,
            RuleReference::new("structural-duplication-rule"),
        )
        .expect("severity, description, and rule reference are valid");

        let recommendation = Recommendation::new(
            "Repackage the archive without duplicate entry names to ensure deterministic, \
             unambiguous content extraction.",
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

    fn duplication_evidence() -> Evidence {
        Evidence::new(
            EvidenceCategory::StructuralDuplication,
            "duplicate entry names detected",
        )
        .expect("category and description are valid")
    }

    fn other_evidence() -> Evidence {
        Evidence::new(EvidenceCategory::FileStructureAnalysis, "sample evidence")
            .expect("category and description are valid")
    }

    #[test]
    fn returns_none_for_empty_evidence() {
        let rule = StructuralDuplicationRule;

        assert_eq!(rule.evaluate(&[]), None);
    }

    #[test]
    fn returns_none_when_no_matching_evidence_exists() {
        let rule = StructuralDuplicationRule;
        let evidence = [other_evidence()];

        assert_eq!(rule.evaluate(&evidence), None);
    }

    #[test]
    fn produces_a_warning_finding_for_matching_evidence() {
        let rule = StructuralDuplicationRule;
        let evidence = duplication_evidence();
        let evidence_id = evidence.id();

        let outcome = rule.evaluate(&[evidence]).expect("evidence matched");

        assert_eq!(outcome.finding.severity(), FindingSeverity::Warning);
        assert!(!outcome.finding.description().is_empty());
        assert_eq!(outcome.finding.evidence_ids(), &[evidence_id]);
        assert_eq!(
            outcome.finding.rule_reference().identifier(),
            "structural-duplication-rule"
        );
        assert!(!outcome.recommendation.action().is_empty());
        assert_eq!(
            outcome.recommendation.finding_ids(),
            &[outcome.finding.id()]
        );
        assert_eq!(outcome.recommendation.repair_recipe_reference(), None);
    }

    #[test]
    fn ignores_non_matching_evidence_alongside_matching_evidence() {
        let rule = StructuralDuplicationRule;
        let matching = duplication_evidence();
        let matching_id = matching.id();
        let non_matching = other_evidence();

        let outcome = rule
            .evaluate(&[non_matching, matching])
            .expect("one item matched");

        assert_eq!(outcome.finding.evidence_ids(), &[matching_id]);
    }

    #[test]
    fn references_all_matching_items_when_more_than_one_exists() {
        let rule = StructuralDuplicationRule;
        let first = duplication_evidence();
        let second = duplication_evidence();
        let first_id = first.id();
        let second_id = second.id();

        let outcome = rule.evaluate(&[first, second]).expect("both items matched");

        assert_eq!(outcome.finding.evidence_ids(), &[first_id, second_id]);
    }

    #[test]
    fn is_deterministic_for_identical_input() {
        let rule = StructuralDuplicationRule;
        let evidence = [duplication_evidence()];

        let first = rule.evaluate(&evidence).expect("evidence matched");
        let second = rule.evaluate(&evidence).expect("evidence matched");

        // Each evaluation freshly assigns Finding/Recommendation
        // identity by design (mirroring every other Runtime Domain
        // identity); determinism is judged by content, not by
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

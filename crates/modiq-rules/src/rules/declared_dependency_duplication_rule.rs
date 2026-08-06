use std::collections::HashMap;

use modiq_runtime::assessment::{
    Evidence, EvidenceCategory, EvidenceId, Finding, FindingSeverity, FindingStatus,
    ModHealthDimension, Recommendation, RuleReference,
};

use super::engine::RuleOutcome;

/// The exact prefix `XmlCollector` (`modiq-collection`,
/// `xml_collector.rs`) uses when reporting a mod's declared
/// `<dependency>` element as Evidence. This Rule and that Collector
/// remain architecturally independent (`EvidenceCollection.md`:
/// Collector Contract) — this is a data-format convention between
/// them, not a code dependency; duplicated here deliberately rather
/// than shared through a new crate dependency, mirroring
/// `VersionCompatibilityRule`'s own `DECLARED_DESC_VERSION_PREFIX`
/// precedent.
const DECLARED_DEPENDENCY_PREFIX: &str = "modDesc.xml declares dependency: ";

/// The Declared Dependency Duplication Rule (C2: Declared Dependency
/// Interpretation).
///
/// Evaluates `XmlInspection` Evidence declaring a mod's `<dependency>`
/// elements for a declared name that recurs more than once within one
/// manifest. `XmlCollector` produces one Evidence item per non-empty
/// `<dependency>` element, undeduplicated; this Rule is the first
/// place that repetition is judged
/// (`C2_IMPLEMENTATION_PLAN_DECLARED_DEPENDENCY_INTERPRETATION.md`
/// §2.1).
///
/// This is the first Rule in this crate to compare multiple Evidence
/// items against each other rather than filtering or mapping each
/// independently — the underlying comparison shape is adapted from
/// `ArchiveCollector::detect_duplicate_entry_names`'s own Collection-
/// layer precedent, not invented fresh (Implementation Plan §2.1).
///
/// Assigned `ModHealthDimension::Structure` (Implementation Plan
/// §2.1): the fact being judged is a defect in the manifest's own
/// declaration structure — a name listed twice — not a compatibility
/// relationship with another mod's actual content.
///
/// Not yet reachable from `RuleEngine::evaluate` — dispatch wiring is
/// Phase 2 of this capability's own Implementation Plan, not this
/// phase. This Rule is a complete, independently tested unit today,
/// exactly as `StructuralDuplicationRule` and `RuntimeLoadFailureRule`
/// were real and tested before their own dispatch wiring existed to
/// reach them.
pub struct DeclaredDependencyDuplicationRule;

impl DeclaredDependencyDuplicationRule {
    /// Evaluates the given Evidence for a declared dependency name
    /// that recurs more than once.
    ///
    /// Assigns `FindingSeverity::Warning` (`DataModel.md`: Finding
    /// Severity), not `Error`: the Evidence available establishes only
    /// that a declared name appears more than once in text, not any
    /// functional consequence of that repetition (Implementation Plan
    /// §2.1).
    ///
    /// Returns `None` when no declared-dependency Evidence exists, or
    /// when every declared name is distinct. References every Evidence
    /// item whose name recurs, not only the second and later
    /// occurrences, in a single Finding — mirroring
    /// `StructuralDuplicationRule`'s own "one Finding, referencing
    /// every matching item" shape.
    pub fn evaluate(&self, evidence: &[Evidence]) -> Option<RuleOutcome> {
        let declared: Vec<(EvidenceId, &str)> = evidence
            .iter()
            .filter(|item| item.category() == EvidenceCategory::XmlInspection)
            .filter_map(|item| {
                let name = item
                    .description()
                    .strip_prefix(DECLARED_DEPENDENCY_PREFIX)?;
                Some((item.id(), name))
            })
            .collect();

        let mut occurrences: HashMap<&str, usize> = HashMap::new();
        for &(_, name) in &declared {
            *occurrences.entry(name).or_insert(0) += 1;
        }

        // A second pass over `declared`, in its own original slice
        // order, keeps `evidence_ids` and the duplicate-name summary
        // stable across repeated evaluations of identical input — the
        // duplication count above is consulted only for O(1) lookup,
        // never iterated over directly (its own iteration order is
        // not deterministic).
        let mut evidence_ids: Vec<EvidenceId> = Vec::new();
        let mut duplicate_names: Vec<&str> = Vec::new();
        for &(id, name) in &declared {
            if occurrences[name] > 1 {
                evidence_ids.push(id);
                if !duplicate_names.contains(&name) {
                    duplicate_names.push(name);
                }
            }
        }

        if evidence_ids.is_empty() {
            return None;
        }

        let summary = format!(
            "modDesc.xml declares the dependency {} more than once.",
            duplicate_names.join(", ")
        );

        let finding = Finding::new(
            FindingSeverity::Warning,
            "Duplicate declared dependency",
            summary,
            ModHealthDimension::Structure,
            FindingStatus::Final,
            evidence_ids,
            RuleReference::new("declared-dependency-duplication-rule"),
        )
        .expect("severity, title, summary, and rule reference are valid");

        let recommendation = Recommendation::new(
            "Remove the duplicate <dependency> declaration from modDesc.xml so each required \
             mod is listed once.",
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

    fn declared_dependency_evidence(name: &str) -> Evidence {
        Evidence::with_location(
            EvidenceCategory::XmlInspection,
            format!("{DECLARED_DEPENDENCY_PREFIX}{name}"),
            "modDesc.xml",
            None,
            None,
            None,
        )
        .expect("description and location are valid")
    }

    fn other_evidence() -> Evidence {
        Evidence::new(
            EvidenceCategory::FileStructureAnalysis,
            "sample evidence",
            None,
            None,
            None,
        )
        .expect("category and description are valid")
    }

    fn declared_desc_version_evidence(desc_version: &str) -> Evidence {
        Evidence::with_location(
            EvidenceCategory::XmlInspection,
            format!("modDesc.xml declares descVersion: {desc_version}"),
            "modDesc.xml",
            None,
            None,
            None,
        )
        .expect("description and location are valid")
    }

    #[test]
    fn returns_none_for_empty_evidence() {
        let rule = DeclaredDependencyDuplicationRule;

        assert_eq!(rule.evaluate(&[]), None);
    }

    #[test]
    fn returns_none_when_no_declared_dependency_evidence_exists() {
        let rule = DeclaredDependencyDuplicationRule;
        let evidence = [other_evidence(), declared_desc_version_evidence("93")];

        assert_eq!(rule.evaluate(&evidence), None);
    }

    #[test]
    fn returns_none_when_every_declared_name_is_distinct() {
        let rule = DeclaredDependencyDuplicationRule;
        let evidence = [
            declared_dependency_evidence("FS25_exampleModOne"),
            declared_dependency_evidence("FS25_exampleModTwo"),
        ];

        assert_eq!(rule.evaluate(&evidence), None);
    }

    #[test]
    fn produces_a_warning_finding_for_a_duplicate_declared_dependency() {
        let rule = DeclaredDependencyDuplicationRule;
        let first = declared_dependency_evidence("FS25_exampleModOne");
        let second = declared_dependency_evidence("FS25_exampleModOne");
        let first_id = first.id();
        let second_id = second.id();

        let outcome = rule
            .evaluate(&[first, second])
            .expect("the declared name recurs");

        assert_eq!(outcome.finding.severity(), FindingSeverity::Warning);
        assert!(!outcome.finding.title().is_empty());
        assert!(!outcome.finding.summary().is_empty());
        assert_eq!(
            outcome.finding.mod_health_dimension(),
            ModHealthDimension::Structure
        );
        assert_eq!(outcome.finding.status(), FindingStatus::Final);
        assert_eq!(outcome.finding.evidence_ids(), &[first_id, second_id]);
        assert_eq!(
            outcome.finding.rule_reference().identifier(),
            "declared-dependency-duplication-rule"
        );
        let recommendation = outcome.recommendation.expect("this Rule always recommends");
        assert!(!recommendation.action().is_empty());
        assert_eq!(recommendation.finding_ids(), &[outcome.finding.id()]);
        assert_eq!(recommendation.repair_recipe_reference(), None);
    }

    #[test]
    fn references_all_matching_items_when_more_than_one_duplicate_name_exists() {
        let rule = DeclaredDependencyDuplicationRule;
        let first = declared_dependency_evidence("FS25_exampleModOne");
        let second = declared_dependency_evidence("FS25_exampleModOne");
        let third = declared_dependency_evidence("FS25_exampleModTwo");
        let fourth = declared_dependency_evidence("FS25_exampleModTwo");
        let ids = [first.id(), second.id(), third.id(), fourth.id()];

        let outcome = rule
            .evaluate(&[first, second, third, fourth])
            .expect("both declared names recur");

        assert_eq!(outcome.finding.evidence_ids(), &ids);
        assert!(outcome.finding.summary().contains("FS25_exampleModOne"));
        assert!(outcome.finding.summary().contains("FS25_exampleModTwo"));
    }

    #[test]
    fn ignores_non_matching_evidence_alongside_duplicate_evidence() {
        let rule = DeclaredDependencyDuplicationRule;
        let first = declared_dependency_evidence("FS25_exampleModOne");
        let second = declared_dependency_evidence("FS25_exampleModOne");
        let first_id = first.id();
        let second_id = second.id();
        let structural = other_evidence();
        let version = declared_desc_version_evidence("93");

        let outcome = rule
            .evaluate(&[structural, version, first, second])
            .expect("the declared name recurs");

        assert_eq!(outcome.finding.evidence_ids(), &[first_id, second_id]);
    }

    #[test]
    fn is_deterministic_for_identical_input() {
        let rule = DeclaredDependencyDuplicationRule;
        let evidence = [
            declared_dependency_evidence("FS25_exampleModOne"),
            declared_dependency_evidence("FS25_exampleModOne"),
        ];

        let first = rule.evaluate(&evidence).expect("the declared name recurs");
        let second = rule.evaluate(&evidence).expect("the declared name recurs");

        // Each evaluation freshly assigns Finding/Recommendation
        // identity by design (mirroring every other Runtime Domain
        // identity); determinism is judged by content, not by
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

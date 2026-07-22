use modiq_knowledge::knowledge::RepairRecipe;
use modiq_runtime::assessment::{
    Evidence, EvidenceCategory, Finding, FindingSeverity, Recommendation, RepairRecipeReference,
    RuleReference,
};
use modiq_versioning::versioning::VersionProfile;

use super::engine::RuleOutcome;

/// The exact prefix `XmlCollector` (`modiq-collection`,
/// `xml_collector.rs`) uses when reporting a mod's declared
/// `descVersion` as Evidence. This Rule and that Collector remain
/// architecturally independent (`EvidenceCollection.md`: Collector
/// Contract) — this is a data-format convention between them, not a
/// code dependency; duplicated here deliberately rather than shared
/// through a new crate dependency.
const DECLARED_DESC_VERSION_PREFIX: &str = "modDesc.xml declares descVersion: ";

/// The Version Compatibility Rule (Sprint 8: Version Profile-aware
/// compatibility checking).
///
/// Evaluates `XmlInspection` Evidence declaring a mod's `descVersion`
/// against the active `VersionProfile`, per `RuleEngine.md`'s Version
/// Aware execution principle. Version-aware interpretation begins
/// here, inside the Rule Engine — `XmlCollector` only ever reports the
/// raw declared value as a fact; this Rule is the first and only place
/// that value is judged against what the platform actually recognizes
/// (Sprint 8 Architectural Resolution, Decision 3).
pub struct VersionCompatibilityRule;

impl VersionCompatibilityRule {
    /// Evaluates the given Evidence for a declared `descVersion` the
    /// active `VersionProfile` does not recognize.
    ///
    /// Assigns `FindingSeverity::Warning` (`DataModel.md`: Finding
    /// Severity), not `Error`: an unrecognized declared version is a
    /// genuine, Evidence-supported compatibility concern, but this
    /// platform cannot conclusively establish from manifest content
    /// alone that the mod will not function — only that its declared
    /// version does not match a Version Profile this platform
    /// recognizes.
    ///
    /// Returns `None` when no declared `descVersion` Evidence exists,
    /// or when every declared value the active Version Profile
    /// recognizes is supported. An Evidence item whose declared value
    /// does not parse as a number is ignored, not treated as
    /// unsupported — this Rule judges only what it can evaluate.
    pub fn evaluate(
        &self,
        evidence: &[Evidence],
        version_profile: &VersionProfile,
    ) -> Option<RuleOutcome> {
        let unsupported: Vec<_> = evidence
            .iter()
            .filter(|item| item.category() == EvidenceCategory::XmlInspection)
            .filter_map(|item| {
                let raw = item
                    .description()
                    .strip_prefix(DECLARED_DESC_VERSION_PREFIX)?;
                let desc_version: u32 = raw.trim().parse().ok()?;
                (!version_profile.supports(desc_version)).then_some((item.id(), desc_version))
            })
            .collect();

        if unsupported.is_empty() {
            return None;
        }

        let evidence_ids: Vec<_> = unsupported.iter().map(|(id, _)| *id).collect();
        let declared_versions: Vec<String> = unsupported
            .iter()
            .map(|(_, version)| version.to_string())
            .collect();

        let finding = Finding::new(
            FindingSeverity::Warning,
            format!(
                "modDesc.xml declares descVersion {}, which the active Version Profile ({}) \
                 does not recognize.",
                declared_versions.join(", "),
                version_profile.game_version().name()
            ),
            evidence_ids,
            RuleReference::new("version-compatibility-rule"),
        )
        .expect("severity, description, and rule reference are valid");

        let recipe = RepairRecipe::version_compatibility_declared_version_mismatch();
        let recommendation = Recommendation::new(
            recipe.guidance(),
            vec![finding.id()],
            Some(RepairRecipeReference::new(recipe.identifier())),
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
    use modiq_versioning::versioning::GameVersion;

    use super::*;

    fn declared_desc_version_evidence(desc_version: &str) -> Evidence {
        Evidence::with_location(
            EvidenceCategory::XmlInspection,
            format!("{DECLARED_DESC_VERSION_PREFIX}{desc_version}"),
            "modDesc.xml",
        )
        .expect("description and location are valid")
    }

    fn other_evidence() -> Evidence {
        Evidence::new(EvidenceCategory::FileStructureAnalysis, "sample evidence")
            .expect("category and description are valid")
    }

    fn fs25_profile() -> VersionProfile {
        VersionProfile::fs25()
    }

    #[test]
    fn returns_none_for_empty_evidence() {
        let rule = VersionCompatibilityRule;

        assert_eq!(rule.evaluate(&[], &fs25_profile()), None);
    }

    #[test]
    fn returns_none_when_no_declared_version_evidence_exists() {
        let rule = VersionCompatibilityRule;
        let evidence = [other_evidence()];

        assert_eq!(rule.evaluate(&evidence, &fs25_profile()), None);
    }

    #[test]
    fn returns_none_when_the_declared_version_is_supported() {
        let rule = VersionCompatibilityRule;
        let evidence = [declared_desc_version_evidence("93")];

        assert_eq!(rule.evaluate(&evidence, &fs25_profile()), None);
    }

    #[test]
    fn produces_a_warning_finding_for_an_unrecognized_declared_version() {
        let rule = VersionCompatibilityRule;
        let evidence = declared_desc_version_evidence("42");
        let evidence_id = evidence.id();

        let outcome = rule
            .evaluate(&[evidence], &fs25_profile())
            .expect("declared version is unrecognized");

        assert_eq!(outcome.finding.severity(), FindingSeverity::Warning);
        assert!(outcome.finding.description().contains("42"));
        assert!(outcome.finding.description().contains("FS25"));
        assert_eq!(outcome.finding.evidence_ids(), &[evidence_id]);
        assert_eq!(
            outcome.finding.rule_reference().identifier(),
            "version-compatibility-rule"
        );
        assert!(!outcome.recommendation.action().is_empty());
        assert_eq!(
            outcome.recommendation.finding_ids(),
            &[outcome.finding.id()]
        );
        assert_eq!(
            outcome.recommendation.repair_recipe_reference(),
            Some(&RepairRecipeReference::new(
                "version-compatibility-declared-version-mismatch"
            ))
        );
    }

    #[test]
    fn ignores_an_unparseable_declared_version() {
        let rule = VersionCompatibilityRule;
        let evidence = [declared_desc_version_evidence("not-a-number")];

        assert_eq!(rule.evaluate(&evidence, &fs25_profile()), None);
    }

    #[test]
    fn ignores_non_matching_evidence_alongside_a_supported_declared_version() {
        let rule = VersionCompatibilityRule;
        let evidence = [other_evidence(), declared_desc_version_evidence("93")];

        assert_eq!(rule.evaluate(&evidence, &fs25_profile()), None);
    }

    #[test]
    fn is_deterministic_for_identical_input() {
        let rule = VersionCompatibilityRule;
        let evidence = [declared_desc_version_evidence("42")];

        let first = rule
            .evaluate(&evidence, &fs25_profile())
            .expect("declared version is unrecognized");
        let second = rule
            .evaluate(&evidence, &fs25_profile())
            .expect("declared version is unrecognized");

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

    #[test]
    fn a_different_version_profile_recognizes_a_different_desc_version() {
        let rule = VersionCompatibilityRule;
        let profile = VersionProfile::new(GameVersion::new("FS22"), vec![74]);
        let evidence = [declared_desc_version_evidence("74")];

        assert_eq!(rule.evaluate(&evidence, &profile), None);
    }
}

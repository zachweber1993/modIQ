use modiq_runtime::assessment::{
    Evidence, EvidenceCategory, EvidenceId, Finding, FindingSeverity, Recommendation, RuleReference,
};

use super::engine::RuleOutcome;

/// The exact prefix `RuntimeLogCollector` (`modiq-collection`,
/// `runtime_log_collector.rs`) uses when reporting the recognized
/// runtime load failure as Evidence. This Rule and that Collector
/// remain architecturally independent (`EvidenceCollection.md`:
/// Collector Contract) — this is a data-format convention between
/// them, not a code dependency; duplicated here deliberately rather
/// than shared through a new crate dependency, mirroring
/// `VersionCompatibilityRule`'s own `DECLARED_DESC_VERSION_PREFIX`
/// precedent.
const RUNTIME_LOG_FAILURE_PREFIX: &str =
    "Runtime log records: Unsupported mod description version in mod ";

/// The Runtime Load Failure Rule (Sprint 11: Runtime Evidence
/// Processing Architecture, Decision Matrix row 3).
///
/// Evaluates `RuntimeLogs` Evidence for the one recognized failure
/// signature this platform has real fixture evidence for
/// (`single-incompatible-mod`): a mod rejected at modDesc validation,
/// never reaching a loadable state. Recognition is exact against the
/// documented template — a category match alone is not sufficient — so
/// a `RuntimeLogs` Evidence item whose description does not match this
/// exact prefix is left as simply collected Evidence, producing no
/// Finding (`RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md`, Architectural
/// Invariants: "Unknown runtime signatures never produce inferred
/// Findings").
///
/// Not yet reachable from `RuleEngine::evaluate` — dispatch wiring is a
/// separate, later milestone. This Rule is a complete, independently
/// tested unit today, exactly as `StructuralDuplicationRule` was real
/// and tested before `RuleEngine` dispatch wiring existed to reach it
/// (Sprint 5 Phase 2/3).
pub struct RuntimeLoadFailureRule;

impl RuntimeLoadFailureRule {
    /// Evaluates the given Evidence for the recognized runtime load
    /// failure template.
    ///
    /// Assigns `FindingSeverity::Error` (`DataModel.md`: Finding
    /// Severity — "conclusively established by Evidence alone...
    /// blocking"): unlike a static manifest declaration, this Evidence
    /// is a direct observation of an actual failed load attempt — the
    /// mod's own enumeration entry is followed by rejection, with no
    /// subsequent load confirmation anywhere in the log
    /// (`RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md`, Section 3.2).
    ///
    /// Returns `None` when no Evidence matches the recognized
    /// template — including when `RuntimeLogs` Evidence exists but
    /// does not match it, which this Rule treats identically to no
    /// `RuntimeLogs` Evidence existing at all. References every
    /// matching Evidence item by id, not just the first.
    pub fn evaluate(&self, evidence: &[Evidence]) -> Option<RuleOutcome> {
        let failures: Vec<_> = evidence
            .iter()
            .filter(|item| item.category() == EvidenceCategory::RuntimeLogs)
            .filter_map(Self::recognized_failure)
            .collect();

        if failures.is_empty() {
            return None;
        }

        let evidence_ids: Vec<_> = failures.iter().map(|(id, _)| *id).collect();
        let mod_names: Vec<String> = failures.iter().map(|(_, name)| name.clone()).collect();

        let finding = Finding::new(
            FindingSeverity::Error,
            format!(
                "The runtime log shows {} was rejected at modDesc validation and never reached \
                 a loadable state.",
                mod_names.join(", ")
            ),
            evidence_ids,
            RuleReference::new("runtime-load-failure-rule"),
        )
        .expect("severity, description, and rule reference are valid");

        let recommendation = Recommendation::new(
            "This mod's declared modDesc version was rejected by the game engine before it \
             could load. Update the mod to declare a modDesc version supported by this \
             installation, or obtain an updated release from the mod's author.",
            vec![finding.id()],
            None,
        )
        .expect("action is valid");

        Some(RuleOutcome {
            finding,
            recommendation,
        })
    }

    /// Recognizes exactly the documented failure template, generalized
    /// over the named mod but not the message text itself
    /// (`RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md`, Section 2.2). An
    /// Evidence item that does not match this exact prefix, or whose
    /// remainder is empty after trimming, is not recognized.
    fn recognized_failure(item: &Evidence) -> Option<(EvidenceId, String)> {
        let mod_name = item
            .description()
            .strip_prefix(RUNTIME_LOG_FAILURE_PREFIX)?
            .trim();
        if mod_name.is_empty() {
            return None;
        }
        Some((item.id(), mod_name.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn recognized_failure_evidence(mod_name: &str) -> Evidence {
        Evidence::with_location(
            EvidenceCategory::RuntimeLogs,
            format!("{RUNTIME_LOG_FAILURE_PREFIX}{mod_name}"),
            "log.txt",
        )
        .expect("description and location are non-empty")
    }

    fn unrecognized_runtime_log_evidence() -> Evidence {
        Evidence::with_location(
            EvidenceCategory::RuntimeLogs,
            "Runtime log records: some other, unrecognized observation",
            "log.txt",
        )
        .expect("description and location are non-empty")
    }

    fn other_evidence() -> Evidence {
        Evidence::new(EvidenceCategory::FileStructureAnalysis, "sample evidence")
            .expect("category and description are valid")
    }

    #[test]
    fn returns_none_for_empty_evidence() {
        let rule = RuntimeLoadFailureRule;

        assert_eq!(rule.evaluate(&[]), None);
    }

    #[test]
    fn returns_none_when_no_runtime_logs_evidence_exists() {
        let rule = RuntimeLoadFailureRule;
        let evidence = [other_evidence()];

        assert_eq!(rule.evaluate(&evidence), None);
    }

    #[test]
    fn returns_none_for_unrecognized_runtime_logs_evidence() {
        // Matrix row 4: RuntimeLogs Evidence exists, but does not match
        // the recognized template — must not produce an inferred
        // Finding.
        let rule = RuntimeLoadFailureRule;
        let evidence = [unrecognized_runtime_log_evidence()];

        assert_eq!(rule.evaluate(&evidence), None);
    }

    #[test]
    fn produces_an_error_finding_for_the_recognized_failure_template() {
        let rule = RuntimeLoadFailureRule;
        let evidence = recognized_failure_evidence("FS25_DodgeChallengerHellcat");
        let evidence_id = evidence.id();

        let outcome = rule
            .evaluate(&[evidence])
            .expect("the recognized template matched");

        // FindingSeverity per Decision Matrix row 3 / Architecture
        // Section 3.2.
        assert_eq!(outcome.finding.severity(), FindingSeverity::Error);
        assert!(
            outcome
                .finding
                .description()
                .contains("FS25_DodgeChallengerHellcat")
        );
        assert_eq!(outcome.finding.evidence_ids(), &[evidence_id]);
        assert_eq!(
            outcome.finding.rule_reference().identifier(),
            "runtime-load-failure-rule"
        );

        // Recommendation generation per Architecture Section 3.4:
        // inline-authored, no Repair Recipe pairing.
        assert!(!outcome.recommendation.action().is_empty());
        assert_eq!(
            outcome.recommendation.finding_ids(),
            &[outcome.finding.id()]
        );
        assert_eq!(outcome.recommendation.repair_recipe_reference(), None);
    }

    #[test]
    fn generalizes_over_a_different_mod_name() {
        let rule = RuntimeLoadFailureRule;
        let evidence = recognized_failure_evidence("FS25_SomeOtherMod");

        let outcome = rule
            .evaluate(&[evidence])
            .expect("the recognized template matched");

        assert!(outcome.finding.description().contains("FS25_SomeOtherMod"));
    }

    #[test]
    fn ignores_non_matching_evidence_in_a_mixed_collection() {
        let rule = RuntimeLoadFailureRule;
        let matching = recognized_failure_evidence("FS25_DodgeChallengerHellcat");
        let matching_id = matching.id();
        let unrecognized = unrecognized_runtime_log_evidence();
        let structural = other_evidence();

        let outcome = rule
            .evaluate(&[structural, unrecognized, matching])
            .expect("exactly one item matched");

        assert_eq!(outcome.finding.evidence_ids(), &[matching_id]);
    }

    #[test]
    fn references_all_matching_items_when_more_than_one_exists() {
        let rule = RuntimeLoadFailureRule;
        let first = recognized_failure_evidence("FS25_ModOne");
        let second = recognized_failure_evidence("FS25_ModTwo");
        let first_id = first.id();
        let second_id = second.id();

        let outcome = rule.evaluate(&[first, second]).expect("both items matched");

        assert_eq!(outcome.finding.evidence_ids(), &[first_id, second_id]);
        assert!(outcome.finding.description().contains("FS25_ModOne"));
        assert!(outcome.finding.description().contains("FS25_ModTwo"));
    }

    #[test]
    fn is_deterministic_for_identical_input() {
        let rule = RuntimeLoadFailureRule;
        let evidence = [recognized_failure_evidence("FS25_DodgeChallengerHellcat")];

        let first = rule.evaluate(&evidence).expect("the template matched");
        let second = rule.evaluate(&evidence).expect("the template matched");

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

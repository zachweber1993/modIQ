use modiq_collection::collection::{ArchiveCollector, AssessmentInput, EvidenceCollector};
use modiq_report::report::AssessmentReport;
use modiq_rules::rules::RuleEngine;
use modiq_runtime::assessment::{Assessment, AssessmentContext, AssessmentSubject, Evidence};

use super::assessment_execution_error::AssessmentExecutionError;

/// Coordinates the lifecycle of an Assessment.
///
/// Owns orchestration only: it sequences calls into the Runtime Domain
/// (modiq-runtime), Rule Engine (modiq-rules), and Reporting
/// (modiq-report) crates without owning runtime state, rule logic, or
/// reporting logic itself. Each of those responsibilities remains
/// implemented, and enforced, entirely within its own crate.
pub struct AssessmentService;

impl AssessmentService {
    /// Executes one complete deterministic Assessment: creation, Evidence
    /// collection, Rule evaluation, Finding and Recommendation
    /// collection, Assessment Report generation, and completion.
    ///
    /// Returns the Assessment Report generated prior to completion.
    pub fn execute(
        &self,
        subject: AssessmentSubject,
        context: AssessmentContext,
        evidence: Vec<Evidence>,
    ) -> AssessmentReport {
        let mut assessment = Assessment::new(subject, context);

        assessment
            .begin_evidence_collection()
            .expect("a newly created Assessment is always in the Created state");

        for item in evidence {
            assessment.add_evidence(item).expect(
                "evidence collection is active immediately after begin_evidence_collection",
            );
        }

        assessment
            .begin_rule_evaluation()
            .expect("evidence collection was just entered and has not yet transitioned away");

        let rule_engine = RuleEngine;
        if let Some(outcome) = rule_engine.evaluate(assessment.evidence()) {
            assessment
                .add_finding(outcome.finding)
                .expect("rule evaluation is active immediately after begin_rule_evaluation");
            assessment
                .add_recommendation(outcome.recommendation)
                .expect("a Finding was just added, and rule evaluation is still active");
        }

        let report = AssessmentReport::generate(&assessment);

        assessment
            .complete()
            .expect("rule evaluation is still active and has not yet completed");

        report
    }

    /// Executes one complete deterministic Assessment, using Evidence
    /// Collection (`modiq-collection`) to produce its Evidence from an
    /// AssessmentInput, rather than accepting already-constructed
    /// Evidence directly (ADR-0008).
    ///
    /// Added alongside `execute` rather than changing its signature:
    /// whether `execute` itself should evolve to accept an
    /// AssessmentInput remains open (ADR-0009, GOV-008). This method
    /// constructs the AssessmentInput, routes it to the appropriate
    /// Collector (see `is_archive_location`), invokes Evidence
    /// Collection, and then delegates to the existing, unchanged
    /// `execute` for the rest of the pipeline — in that order, so that
    /// `execute` (and therefore Assessment creation) is only ever
    /// reached once collection has already succeeded (Collection
    /// Atomicity, `EvidenceCollection.md`): if any step fails, this
    /// method returns before any Assessment exists at all.
    pub fn execute_from_assessment_input(
        &self,
        subject: AssessmentSubject,
        context: AssessmentContext,
        input: impl Into<String>,
    ) -> Result<AssessmentReport, AssessmentExecutionError> {
        let assessment_input = AssessmentInput::new(input)?;

        let evidence = if Self::is_archive_location(assessment_input.value()) {
            ArchiveCollector.collect(assessment_input.value())?
        } else {
            EvidenceCollector.collect(&assessment_input)?
        };

        Ok(self.execute(subject, context, evidence))
    }

    /// The explicit archive-vs-filesystem routing decision (Sprint 4
    /// Phase 3D, `SPRINT4_IMPLEMENTATION_PLAN.md`: Approved Routing &
    /// Collector Shape). A location is routed to `ArchiveCollector`
    /// exactly when its value ends in `.zip`, case-insensitively;
    /// every other value routes to the filesystem `EvidenceCollector`,
    /// exactly as it did before this phase.
    ///
    /// Deliberately one direct, inline check rather than any lookup
    /// table, trait dispatch, or configuration-driven mechanism, per
    /// the Technical Director's explicit-routing decision: two
    /// collectors is not evidence a dispatch abstraction is
    /// justified. Neither collector shares a trait or common
    /// supertype with the other; both are invoked directly, by name,
    /// from this one decision point.
    ///
    /// A location merely named `*.zip` that is not actually a
    /// well-formed archive (a directory, a non-archive file) is not
    /// specially handled here — `ArchiveCollector` itself reports the
    /// appropriate Inaccessible/Unsupported outcome, the same
    /// discipline `EvidenceCollector` already applies for its own
    /// unsupported cases.
    fn is_archive_location(value: &str) -> bool {
        value.to_ascii_lowercase().ends_with(".zip")
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    use modiq_collection::collection::{AssessmentInputError, CollectionError};
    use modiq_runtime::assessment::{AssessmentStatus, EvidenceCategory};
    use zip::ZipWriter;
    use zip::write::SimpleFileOptions;

    use super::*;

    fn sample_evidence() -> Evidence {
        Evidence::new(EvidenceCategory::FileStructureAnalysis, "sample evidence")
            .expect("category and description are valid")
    }

    /// A real, unique, temporary directory for exercising the real
    /// filesystem collector end to end. Removed when dropped.
    struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        fn new(label: &str) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "modiq-engine-test-{}-{}-{}",
                std::process::id(),
                label,
                unique
            ));
            fs::create_dir_all(&path).expect("can create a temporary test directory");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    /// Writes a real, well-formed ZIP archive containing the given
    /// (name, content) file entries, for exercising the real archive
    /// routing path end to end. Mirrors `archive_reader.rs`'s own test
    /// helper of the same shape.
    fn write_archive(path: &Path, files: &[(&str, &str)]) {
        let file = fs::File::create(path).expect("can create a temporary archive file");
        let mut writer = ZipWriter::new(file);
        let options = SimpleFileOptions::default();

        for (name, content) in files {
            writer
                .start_file(*name, options)
                .expect("can start a file entry");
            std::io::Write::write_all(&mut writer, content.as_bytes())
                .expect("can write file entry content");
        }
        writer.finish().expect("can finalize the archive");
    }

    #[test]
    fn execute_with_evidence_produces_a_finding_and_recommendation() {
        let service = AssessmentService;
        let evidence = sample_evidence();

        let report = service.execute(AssessmentSubject, AssessmentContext, vec![evidence.clone()]);

        assert_eq!(report.evidence(), &[evidence]);
        assert_eq!(report.findings().len(), 1);
        assert_eq!(report.recommendations().len(), 1);
    }

    #[test]
    fn execute_without_evidence_produces_no_findings_or_recommendations() {
        let service = AssessmentService;

        let report = service.execute(AssessmentSubject, AssessmentContext, vec![]);

        assert!(report.evidence().is_empty());
        assert!(report.findings().is_empty());
        assert!(report.recommendations().is_empty());
    }

    #[test]
    fn execute_reflects_state_at_report_generation_prior_to_completion() {
        let service = AssessmentService;

        let report = service.execute(
            AssessmentSubject,
            AssessmentContext,
            vec![sample_evidence()],
        );

        assert_eq!(report.status(), AssessmentStatus::EvaluatingRules);
    }

    #[test]
    fn separate_executions_are_independent_and_deterministic() {
        let service = AssessmentService;
        let evidence = sample_evidence();

        let first = service.execute(AssessmentSubject, AssessmentContext, vec![evidence.clone()]);
        let second = service.execute(AssessmentSubject, AssessmentContext, vec![evidence]);

        assert_ne!(first.assessment_id(), second.assessment_id());
        assert_eq!(first.evidence(), second.evidence());
        // Each execution's Finding is freshly assigned its own identity by
        // RuleEngine (mirroring AssessmentId/EvidenceId); determinism is
        // judged by content, not by incidental identity.
        assert_eq!(first.findings().len(), second.findings().len());
        for (first_finding, second_finding) in first.findings().iter().zip(second.findings()) {
            assert_eq!(first_finding.severity(), second_finding.severity());
            assert_eq!(first_finding.description(), second_finding.description());
            assert_eq!(first_finding.evidence_ids(), second_finding.evidence_ids());
            assert_eq!(
                first_finding.rule_reference(),
                second_finding.rule_reference()
            );
        }
        // Same rationale for Recommendation: content is compared, not
        // identity, and finding_ids is compared structurally rather than
        // by value since it references each execution's own Finding.
        assert_eq!(
            first.recommendations().len(),
            second.recommendations().len()
        );
        for (first_recommendation, second_recommendation) in
            first.recommendations().iter().zip(second.recommendations())
        {
            assert_eq!(
                first_recommendation.action(),
                second_recommendation.action()
            );
            assert_eq!(
                first_recommendation.finding_ids().len(),
                second_recommendation.finding_ids().len()
            );
            assert_eq!(
                first_recommendation.repair_recipe_reference(),
                second_recommendation.repair_recipe_reference()
            );
        }
    }

    #[test]
    fn execute_from_assessment_input_produces_a_finding_and_recommendation_via_the_real_filesystem()
    {
        let dir = TempDir::new("execute-success");
        fs::write(dir.path().join("sample.txt"), "sample content").unwrap();
        let service = AssessmentService;

        let report = service
            .execute_from_assessment_input(
                AssessmentSubject,
                AssessmentContext,
                dir.path().display().to_string(),
            )
            .expect("directory is accessible");

        assert_eq!(report.evidence().len(), 1);
        assert_eq!(report.findings().len(), 1);
        assert_eq!(report.recommendations().len(), 1);
    }

    #[test]
    fn execute_from_assessment_input_rejects_an_empty_input() {
        let service = AssessmentService;

        let result =
            service.execute_from_assessment_input(AssessmentSubject, AssessmentContext, "");

        assert_eq!(
            result,
            Err(AssessmentExecutionError::InvalidInput(
                AssessmentInputError::EmptyValue
            ))
        );
    }

    #[test]
    fn execute_from_assessment_input_reports_inaccessible_for_a_nonexistent_path() {
        let dir = TempDir::new("execute-inaccessible");
        let missing = dir.path().join("does-not-exist");
        let service = AssessmentService;

        let result = service.execute_from_assessment_input(
            AssessmentSubject,
            AssessmentContext,
            missing.display().to_string(),
        );

        assert_eq!(
            result,
            Err(AssessmentExecutionError::Collection(
                CollectionError::Inaccessible {
                    path: missing.display().to_string()
                }
            ))
        );
    }

    #[test]
    fn execute_from_assessment_input_reflects_state_at_report_generation_prior_to_completion() {
        let dir = TempDir::new("execute-status");
        fs::write(dir.path().join("sample.txt"), "sample content").unwrap();
        let service = AssessmentService;

        let report = service
            .execute_from_assessment_input(
                AssessmentSubject,
                AssessmentContext,
                dir.path().display().to_string(),
            )
            .expect("directory is accessible");

        assert_eq!(report.status(), AssessmentStatus::EvaluatingRules);
    }

    #[test]
    fn execute_from_assessment_input_never_creates_an_assessment_when_collection_fails() {
        let service = AssessmentService;

        let result =
            service.execute_from_assessment_input(AssessmentSubject, AssessmentContext, "");

        // Atomicity is structural here: execute_from_assessment_input
        // returns before calling execute (and therefore before
        // Assessment::new) whenever AssessmentInput::new or
        // EvidenceCollector::collect fails, so there is no separate
        // runtime state to inspect — the absence of any AssessmentReport
        // at all is the assertion.
        assert!(result.is_err());
    }

    #[test]
    fn is_archive_location_matches_the_zip_extension_case_insensitively() {
        assert!(AssessmentService::is_archive_location("mod.zip"));
        assert!(AssessmentService::is_archive_location("MOD.ZIP"));
        assert!(AssessmentService::is_archive_location(
            "archive/nested/mod.Zip"
        ));
    }

    #[test]
    fn is_archive_location_rejects_non_zip_values() {
        assert!(!AssessmentService::is_archive_location("mod.rar"));
        assert!(!AssessmentService::is_archive_location("a/mod/directory"));
        assert!(!AssessmentService::is_archive_location("zip"));
        assert!(!AssessmentService::is_archive_location("mod.zip.bak"));
    }

    #[test]
    fn execute_from_assessment_input_routes_a_zip_extension_to_the_archive_collector() {
        let dir = TempDir::new("execute-archive-success");
        let archive_path = dir.path().join("mod.zip");
        write_archive(&archive_path, &[("notes.txt", "sample content")]);
        let service = AssessmentService;

        let report = service
            .execute_from_assessment_input(
                AssessmentSubject,
                AssessmentContext,
                archive_path.display().to_string(),
            )
            .expect("archive is well-formed and accessible");

        assert_eq!(report.evidence().len(), 1);
        assert_eq!(
            report.evidence()[0].category(),
            EvidenceCategory::FileStructureAnalysis
        );
        assert_eq!(
            report.evidence()[0].description(),
            "File discovered during archive collection."
        );
        assert_eq!(report.evidence()[0].location(), Some("notes.txt"));
        assert_eq!(report.findings().len(), 1);
        assert_eq!(report.recommendations().len(), 1);
    }

    #[test]
    fn execute_from_assessment_input_routing_is_case_insensitive_for_the_zip_extension() {
        let dir = TempDir::new("execute-archive-case-insensitive");
        let archive_path = dir.path().join("MOD.ZIP");
        write_archive(&archive_path, &[("notes.txt", "sample content")]);
        let service = AssessmentService;

        let report = service
            .execute_from_assessment_input(
                AssessmentSubject,
                AssessmentContext,
                archive_path.display().to_string(),
            )
            .expect("archive is well-formed and accessible");

        assert_eq!(report.evidence().len(), 1);
    }

    #[test]
    fn execute_from_assessment_input_reports_unsupported_for_a_malformed_archive() {
        let dir = TempDir::new("execute-archive-malformed");
        let archive_path = dir.path().join("mod.zip");
        fs::write(&archive_path, b"this is plain text, not a zip archive").unwrap();
        let service = AssessmentService;

        let result = service.execute_from_assessment_input(
            AssessmentSubject,
            AssessmentContext,
            archive_path.display().to_string(),
        );

        assert_eq!(
            result,
            Err(AssessmentExecutionError::Collection(
                CollectionError::Unsupported {
                    path: archive_path.display().to_string()
                }
            ))
        );
    }

    #[test]
    fn execute_from_assessment_input_still_routes_a_non_zip_file_to_the_filesystem_collector() {
        let dir = TempDir::new("execute-plain-file");
        let file_path = dir.path().join("notes.txt");
        fs::write(&file_path, "sample content").unwrap();
        let service = AssessmentService;

        let report = service
            .execute_from_assessment_input(
                AssessmentSubject,
                AssessmentContext,
                file_path.display().to_string(),
            )
            .expect("file is accessible");

        assert_eq!(report.evidence().len(), 1);
        assert_eq!(
            report.evidence()[0].description(),
            "File discovered during filesystem collection."
        );
    }

    #[test]
    fn execute_from_assessment_input_archive_routing_is_deterministic_across_repeated_calls() {
        let dir = TempDir::new("execute-archive-deterministic");
        let archive_path = dir.path().join("mod.zip");
        write_archive(
            &archive_path,
            &[("alpha.txt", "a"), ("nested/beta.txt", "b")],
        );
        let service = AssessmentService;

        let first = service
            .execute_from_assessment_input(
                AssessmentSubject,
                AssessmentContext,
                archive_path.display().to_string(),
            )
            .expect("archive is well-formed and accessible");
        let second = service
            .execute_from_assessment_input(
                AssessmentSubject,
                AssessmentContext,
                archive_path.display().to_string(),
            )
            .expect("archive is well-formed and accessible");

        let first_locations: Vec<Option<&str>> =
            first.evidence().iter().map(Evidence::location).collect();
        let second_locations: Vec<Option<&str>> =
            second.evidence().iter().map(Evidence::location).collect();
        assert_eq!(first_locations, second_locations);
        assert_eq!(first.findings().len(), second.findings().len());
    }
}

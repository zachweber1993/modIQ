use modiq_storage::storage::{PersistedAssessmentReport, ReportKey, ReportStore, ReportStoreError};

use crate::app::ExitCode;

/// Retrieves a previously-stored `AssessmentReport`, independent of
/// running a new Assessment.
///
/// Owns only: constructing the call into `ReportStore` and formatting
/// its result for display — the CLI-side proof that a report
/// survives beyond the process that produced it.
pub struct RetrieveCommand;

impl RetrieveCommand {
    pub fn run(key: &str, storage_root: &str) -> (String, ExitCode) {
        let store = ReportStore::new(storage_root);

        match store.retrieve(&ReportKey::from_raw(key)) {
            Ok(report) => (Self::format_report(key, &report), ExitCode::Success),
            Err(ReportStoreError::NotFound) => (
                format!("error: no report is stored under key `{key}`"),
                ExitCode::InvalidUsage,
            ),
            Err(error) => (
                format!("error: failed to retrieve the report: {error}"),
                ExitCode::ExecutionFailure,
            ),
        }
    }

    fn format_report(key: &str, report: &PersistedAssessmentReport) -> String {
        let mut output = format!("Report {key} — status: {:?}\n", report.status());

        output.push_str(&format!("\nEvidence ({}):\n", report.evidence().len()));
        for item in report.evidence() {
            output.push_str(&format!(
                "  - [{:?}] {}\n",
                item.category(),
                item.description()
            ));
            if let Some(location) = item.location() {
                output.push_str(&format!("    - Location: {location}\n"));
            }
        }

        output.push_str(&format!("\nFindings ({}):\n", report.findings().len()));
        for finding in report.findings() {
            output.push_str(&format!(
                "  - [{:?}] {}: {}\n",
                finding.severity(),
                finding.title(),
                finding.summary()
            ));
        }

        output.push_str(&format!(
            "\nRecommendations ({}):\n",
            report.recommendations().len()
        ));
        for recommendation in report.recommendations() {
            output.push_str(&format!("  - {}\n", recommendation.action()));
            for step in recommendation.repair_steps() {
                output.push_str(&format!(
                    "    - {:?}: {}\n",
                    step.kind(),
                    step.instruction()
                ));
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;
    use modiq_runtime::assessment::{
        Assessment, AssessmentContext, AssessmentSubject, Evidence, EvidenceCategory, Finding,
        FindingSeverity, FindingStatus, ModHealthDimension, Recommendation, RecommendationStep,
        RecommendationStepKind, RuleReference, VersionProfileReference,
    };

    /// A real, unique, temporary directory, mirroring this crate's own
    /// `assess` test helper of the same shape. Removed when dropped.
    struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        fn new(label: &str) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "modiq-cli-retrieve-test-{}-{}-{}",
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

    #[test]
    fn run_retrieves_a_previously_stored_report() {
        let storage = TempDir::new("retrieve-success");
        let store = ReportStore::new(storage.path());

        let mut assessment = Assessment::new(
            AssessmentSubject,
            AssessmentContext,
            VersionProfileReference::new("FS25"),
        );
        assessment.begin_evidence_collection().unwrap();
        assessment
            .add_evidence(
                Evidence::new(
                    EvidenceCategory::FileStructureAnalysis,
                    "sample evidence",
                    None,
                    None,
                    None,
                )
                .unwrap(),
            )
            .unwrap();
        let report = modiq_report::report::AssessmentReport::generate(&assessment);
        let key = store.store(&report).unwrap();

        let (message, exit_code) =
            RetrieveCommand::run(key.value(), &storage.path().display().to_string());

        assert_eq!(exit_code, ExitCode::Success);
        assert!(message.contains("Evidence (1)"));
    }

    #[test]
    fn run_against_an_unrecognized_key_is_invalid_usage() {
        let storage = TempDir::new("retrieve-not-found");

        let (message, exit_code) =
            RetrieveCommand::run("no-such-key", &storage.path().display().to_string());

        assert_eq!(exit_code, ExitCode::InvalidUsage);
        assert!(message.contains("no report is stored"));
    }

    #[test]
    fn run_shows_an_evidence_items_location_when_present() {
        let storage = TempDir::new("retrieve-location-present");
        let store = ReportStore::new(storage.path());

        let mut assessment = Assessment::new(
            AssessmentSubject,
            AssessmentContext,
            VersionProfileReference::new("FS25"),
        );
        assessment.begin_evidence_collection().unwrap();
        assessment
            .add_evidence(
                Evidence::with_location(
                    EvidenceCategory::FileStructureAnalysis,
                    "sample evidence",
                    "root",
                    None,
                    None,
                    None,
                )
                .unwrap(),
            )
            .unwrap();
        let report = modiq_report::report::AssessmentReport::generate(&assessment);
        let key = store.store(&report).unwrap();

        let (message, exit_code) =
            RetrieveCommand::run(key.value(), &storage.path().display().to_string());

        assert_eq!(exit_code, ExitCode::Success);
        assert!(message.contains("Location: root"));
    }

    #[test]
    fn run_omits_the_location_sub_line_when_absent() {
        let storage = TempDir::new("retrieve-location-absent");
        let store = ReportStore::new(storage.path());

        let mut assessment = Assessment::new(
            AssessmentSubject,
            AssessmentContext,
            VersionProfileReference::new("FS25"),
        );
        assessment.begin_evidence_collection().unwrap();
        assessment
            .add_evidence(
                Evidence::new(
                    EvidenceCategory::FileStructureAnalysis,
                    "sample evidence",
                    None,
                    None,
                    None,
                )
                .unwrap(),
            )
            .unwrap();
        let report = modiq_report::report::AssessmentReport::generate(&assessment);
        let key = store.store(&report).unwrap();

        let (message, exit_code) =
            RetrieveCommand::run(key.value(), &storage.path().display().to_string());

        assert_eq!(exit_code, ExitCode::Success);
        assert!(!message.contains("Location:"));
    }

    #[test]
    fn run_shows_each_repair_steps_kind_and_instruction() {
        let storage = TempDir::new("retrieve-repair-steps-present");
        let store = ReportStore::new(storage.path());

        let mut assessment = Assessment::new(
            AssessmentSubject,
            AssessmentContext,
            VersionProfileReference::new("FS25"),
        );
        assessment.begin_evidence_collection().unwrap();
        let evidence = Evidence::new(
            EvidenceCategory::FileStructureAnalysis,
            "sample evidence",
            None,
            None,
            None,
        )
        .unwrap();
        assessment.add_evidence(evidence.clone()).unwrap();
        assessment.begin_rule_evaluation().unwrap();
        let finding = Finding::new(
            FindingSeverity::Warning,
            "Sample finding",
            "sample finding summary",
            ModHealthDimension::Compatibility,
            FindingStatus::Final,
            vec![evidence.id()],
            RuleReference::new("sample-rule"),
        )
        .unwrap();
        let finding_id = finding.id();
        assessment.add_finding(finding).unwrap();
        let recommendation = Recommendation::new(
            "update the mod dependency",
            vec![finding_id],
            None,
            vec![RecommendationStep::new(
                RecommendationStepKind::VersionUpdate,
                "update to the required version",
            )],
        )
        .unwrap();
        assessment.add_recommendation(recommendation).unwrap();

        let report = modiq_report::report::AssessmentReport::generate(&assessment);
        let key = store.store(&report).unwrap();

        let (message, exit_code) =
            RetrieveCommand::run(key.value(), &storage.path().display().to_string());

        assert_eq!(exit_code, ExitCode::Success);
        assert!(message.contains("VersionUpdate: update to the required version"));
    }

    #[test]
    fn run_shows_no_repair_step_sub_line_when_repair_steps_is_empty() {
        let storage = TempDir::new("retrieve-repair-steps-empty");
        let store = ReportStore::new(storage.path());

        let mut assessment = Assessment::new(
            AssessmentSubject,
            AssessmentContext,
            VersionProfileReference::new("FS25"),
        );
        assessment.begin_evidence_collection().unwrap();
        let evidence = Evidence::new(
            EvidenceCategory::FileStructureAnalysis,
            "sample evidence",
            None,
            None,
            None,
        )
        .unwrap();
        assessment.add_evidence(evidence.clone()).unwrap();
        assessment.begin_rule_evaluation().unwrap();
        let finding = Finding::new(
            FindingSeverity::Warning,
            "Sample finding",
            "sample finding summary",
            ModHealthDimension::Compatibility,
            FindingStatus::Final,
            vec![evidence.id()],
            RuleReference::new("sample-rule"),
        )
        .unwrap();
        let finding_id = finding.id();
        assessment.add_finding(finding).unwrap();
        let recommendation = Recommendation::new(
            "update the mod dependency",
            vec![finding_id],
            None,
            Vec::new(),
        )
        .unwrap();
        assessment.add_recommendation(recommendation).unwrap();

        let report = modiq_report::report::AssessmentReport::generate(&assessment);
        let key = store.store(&report).unwrap();

        let (message, exit_code) =
            RetrieveCommand::run(key.value(), &storage.path().display().to_string());

        assert_eq!(exit_code, ExitCode::Success);
        assert!(message.contains("update the mod dependency"));
        assert!(!message.contains("VersionUpdate"));
        assert!(!message.contains("XmlChange"));
        assert!(!message.contains("LuaChange"));
        assert!(!message.contains("DependencyInstallation"));
        assert!(!message.contains("AssetReplacement"));
    }
}

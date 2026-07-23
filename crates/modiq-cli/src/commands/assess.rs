use modiq_engine::engine::{AssessmentExecutionError, AssessmentService};
use modiq_report::report::AssessmentReport;
use modiq_runtime::assessment::{AssessmentContext, AssessmentSubject};
use modiq_storage::storage::{ReportKey, ReportStore};

use crate::app::ExitCode;

/// Executes an assessment against a user-supplied path.
///
/// Owns only: constructing the call into `AssessmentService`,
/// formatting its result for display, and — once an Assessment
/// succeeds — handing the resulting `AssessmentReport` to Storage.
/// It never evaluates Evidence, generates Findings or
/// Recommendations, or reimplements any part of the pipeline
/// `AssessmentService::execute_from_assessment_input` already owns
/// (`GOVERNANCE.md`, CLI: "must never contain business logic").
/// Reuses the same entry point the Sandbox already calls
/// (`apps/sandbox/src-tauri/src/lib.rs`), against a real, user-supplied
/// path rather than a fixed fixture. Storage sits strictly downstream
/// of Reporting: a storage failure is reported alongside a successful
/// assessment's own output, never in place of it — the assessment
/// itself already completed by the time Storage is ever consulted.
pub struct AssessCommand;

impl AssessCommand {
    pub fn run(path: &str, storage_root: &str) -> (String, ExitCode) {
        let service = AssessmentService;

        match service.execute_from_assessment_input(AssessmentSubject, AssessmentContext, path) {
            Ok(report) => {
                let store = ReportStore::new(storage_root);
                let stored = store.store(&report);
                (
                    Self::format_report(&report, stored.as_ref()),
                    ExitCode::Success,
                )
            }
            // AssessmentInputError: the AssessmentInput itself was
            // malformed (currently only an empty value) — collection
            // never began, so this is invalid input, not an execution
            // failure.
            Err(AssessmentExecutionError::InvalidInput(error)) => {
                (format!("error: {error}"), ExitCode::InvalidUsage)
            }
            // CollectionError: a well-formed input was given, but
            // collection was attempted and aborted (inaccessible or
            // unsupported location) — an execution failure.
            Err(AssessmentExecutionError::Collection(error)) => {
                (format!("error: {error}"), ExitCode::ExecutionFailure)
            }
        }
    }

    fn format_report(
        report: &AssessmentReport,
        stored: Result<&ReportKey, &modiq_storage::storage::ReportStoreError>,
    ) -> String {
        let mut output = format!(
            "Assessment {:?} — status: {:?}\n",
            report.assessment_id(),
            report.status()
        );

        match stored {
            Ok(key) => output.push_str(&format!("Stored as: {}\n", key.value())),
            Err(error) => {
                output.push_str(&format!("Warning: this report was not stored: {error}\n"))
            }
        }

        output.push_str(&format!("\nEvidence ({}):\n", report.evidence().len()));
        for item in report.evidence() {
            output.push_str(&format!(
                "  - [{:?}] {}\n",
                item.category(),
                item.description()
            ));
        }

        output.push_str(&format!("\nFindings ({}):\n", report.findings().len()));
        for finding in report.findings() {
            output.push_str(&format!(
                "  - [{:?}] {}\n",
                finding.severity(),
                finding.description()
            ));
        }

        output.push_str(&format!(
            "\nRecommendations ({}):\n",
            report.recommendations().len()
        ));
        for recommendation in report.recommendations() {
            output.push_str(&format!("  - {}\n", recommendation.action()));
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

    /// A real, unique, temporary directory, mirroring
    /// `modiq-engine`'s own test helper of the same shape. Removed
    /// when dropped.
    struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        fn new(label: &str) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "modiq-cli-test-{}-{}-{}",
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
    fn run_against_a_real_directory_succeeds_and_reports_evidence() {
        let dir = TempDir::new("assess-success");
        fs::write(dir.path().join("sample.txt"), "sample content").unwrap();
        let storage = TempDir::new("assess-success-storage");

        let (message, exit_code) = AssessCommand::run(
            &dir.path().display().to_string(),
            &storage.path().display().to_string(),
        );

        assert_eq!(exit_code, ExitCode::Success);
        // One FileStructureAnalysis item (sample.txt) plus one
        // XmlInspection item (no modDesc.xml in this fixture) — Sprint
        // 7's Multi-Source Evidence Collection now always runs
        // alongside the structural Collector.
        assert!(message.contains("Evidence (2)"));
        assert!(message.contains("Findings (1)"));
        assert!(message.contains("Recommendations (1)"));
    }

    #[test]
    fn run_against_a_real_directory_stores_the_report() {
        let dir = TempDir::new("assess-stores");
        fs::write(dir.path().join("sample.txt"), "sample content").unwrap();
        let storage = TempDir::new("assess-stores-storage");

        let (message, _) = AssessCommand::run(
            &dir.path().display().to_string(),
            &storage.path().display().to_string(),
        );

        assert!(message.contains("Stored as:"));
        assert!(!message.contains("Warning: this report was not stored"));

        // The stored report is genuinely retrievable afterward — not
        // merely reported as stored.
        let key_line = message
            .lines()
            .find(|line| line.starts_with("Stored as:"))
            .expect("success output names the storage key");
        let key_value = key_line.trim_start_matches("Stored as:").trim();

        let store = ReportStore::new(storage.path());
        let retrieved = store.retrieve(&ReportKey::from_raw(key_value));

        assert!(retrieved.is_ok());
    }

    #[test]
    fn run_against_an_empty_path_is_invalid_usage() {
        let storage = TempDir::new("assess-empty-storage");

        let (message, exit_code) = AssessCommand::run("", &storage.path().display().to_string());

        assert_eq!(exit_code, ExitCode::InvalidUsage);
        assert!(message.contains("must not be empty"));
    }

    #[test]
    fn run_against_a_nonexistent_path_is_an_execution_failure() {
        let dir = TempDir::new("assess-inaccessible");
        let missing = dir.path().join("does-not-exist");
        let storage = TempDir::new("assess-inaccessible-storage");

        let (message, exit_code) = AssessCommand::run(
            &missing.display().to_string(),
            &storage.path().display().to_string(),
        );

        assert_eq!(exit_code, ExitCode::ExecutionFailure);
        assert!(message.contains("not accessible"));
    }
}

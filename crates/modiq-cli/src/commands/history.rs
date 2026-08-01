use modiq_storage::storage::{PatternFrequency, ReportStore, recurring_patterns};

use crate::app::ExitCode;

/// Displays recurring Rule/Severity patterns across every report
/// currently held by Storage.
///
/// Read-only, and entirely out-of-band from `AssessmentService`: this
/// command never runs an Assessment, never touches `modiq-knowledge`,
/// and never mutates anything Storage holds. See
/// `docs/engineering/CAPABILITY_DEFINITION_HISTORICAL_ASSESSMENT_ANALYSIS.md`.
pub struct HistoryCommand;

impl HistoryCommand {
    pub fn run(storage_root: &str) -> (String, ExitCode) {
        let store = ReportStore::new(storage_root);

        match recurring_patterns(&store) {
            Ok(patterns) if patterns.is_empty() => (
                "No recurring patterns found — no reports are stored yet, or no stored report contains a Finding.".to_string(),
                ExitCode::Success,
            ),
            Ok(patterns) => (Self::format_patterns(&patterns), ExitCode::Success),
            Err(error) => (
                format!("error: failed to analyze report history: {error}"),
                ExitCode::ExecutionFailure,
            ),
        }
    }

    fn format_patterns(patterns: &[PatternFrequency]) -> String {
        let mut output = format!("Recurring patterns ({}):\n", patterns.len());
        for pattern in patterns {
            output.push_str(&format!(
                "  - [{:?}] {} — {} occurrence(s)\n",
                pattern.severity(),
                pattern.rule_reference(),
                pattern.occurrences()
            ));
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
        FindingSeverity, RuleReference, VersionProfileReference,
    };

    /// A real, unique, temporary directory, mirroring this crate's own
    /// `retrieve` test helper of the same shape. Removed when dropped.
    struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        fn new(label: &str) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "modiq-cli-history-test-{}-{}-{}",
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
    fn run_against_an_empty_store_reports_no_patterns() {
        let storage = TempDir::new("history-empty");

        let (message, exit_code) = HistoryCommand::run(&storage.path().display().to_string());

        assert_eq!(exit_code, ExitCode::Success);
        assert!(message.contains("No recurring patterns found"));
    }

    #[test]
    fn run_reports_a_recurring_pattern_across_stored_reports() {
        let storage = TempDir::new("history-recurring");
        let store = ReportStore::new(storage.path());

        for _ in 0..2 {
            let mut assessment = Assessment::new(
                AssessmentSubject,
                AssessmentContext,
                VersionProfileReference::new("FS25"),
            );
            assessment.begin_evidence_collection().unwrap();
            let evidence =
                Evidence::new(EvidenceCategory::FileStructureAnalysis, "sample evidence").unwrap();
            assessment.add_evidence(evidence.clone()).unwrap();
            assessment.begin_rule_evaluation().unwrap();
            assessment
                .add_finding(
                    Finding::new(
                        FindingSeverity::Warning,
                        "declared version mismatch",
                        vec![evidence.id()],
                        RuleReference::new("version-compatibility-rule"),
                    )
                    .unwrap(),
                )
                .unwrap();
            let report = modiq_report::report::AssessmentReport::generate(&assessment);
            store.store(&report).unwrap();
        }

        let (message, exit_code) = HistoryCommand::run(&storage.path().display().to_string());

        assert_eq!(exit_code, ExitCode::Success);
        assert!(message.contains("version-compatibility-rule"));
        assert!(message.contains("2 occurrence(s)"));
    }
}

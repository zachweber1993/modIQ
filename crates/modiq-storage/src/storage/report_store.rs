//! Durable write and read capability for `PersistedAssessmentReport`.
//!
//! Real filesystem I/O — one file per stored report, keyed by
//! `ReportKey` — chosen as the smallest mechanism requiring no new
//! external dependency beyond what this workspace already declares.
//! The choice of mechanism is an implementation detail, not an
//! architectural one; nothing above this module depends on it being a
//! filesystem specifically.
use std::fs;
use std::io;
use std::path::PathBuf;

use modiq_report::report::AssessmentReport;

use super::persisted_report::PersistedAssessmentReport;
use super::report_key::ReportKey;
use super::report_store_error::ReportStoreError;

pub struct ReportStore {
    root: PathBuf,
}

impl ReportStore {
    /// Creates a store rooted at the given directory. The directory
    /// is created on first write if it does not already exist.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Persists an already-generated `AssessmentReport` durably,
    /// returning the key it can later be retrieved by.
    pub fn store(&self, report: &AssessmentReport) -> Result<ReportKey, ReportStoreError> {
        let key = ReportKey::generate();
        let persisted = PersistedAssessmentReport::from_report(report);
        let bytes = serde_json::to_vec_pretty(&persisted).map_err(ReportStoreError::Serialize)?;

        fs::create_dir_all(&self.root).map_err(ReportStoreError::Write)?;
        fs::write(self.path_for(&key), bytes).map_err(ReportStoreError::Write)?;

        Ok(key)
    }

    /// Returns a previously-stored report, given only the key it was
    /// stored under.
    pub fn retrieve(&self, key: &ReportKey) -> Result<PersistedAssessmentReport, ReportStoreError> {
        let bytes = fs::read(self.path_for(key)).map_err(|error| match error.kind() {
            io::ErrorKind::NotFound => ReportStoreError::NotFound,
            _ => ReportStoreError::Read(error),
        })?;

        serde_json::from_slice(&bytes).map_err(ReportStoreError::Deserialize)
    }

    fn path_for(&self, key: &ReportKey) -> PathBuf {
        self.root.join(format!("{}.json", key.value()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use modiq_runtime::assessment::{
        Assessment, AssessmentContext, AssessmentSubject, Evidence, EvidenceCategory, Finding,
        FindingSeverity, RuleReference, VersionProfileReference,
    };
    use std::path::Path;
    use std::sync::atomic::{AtomicU64, Ordering};

    /// A real, unique, temporary directory, mirroring `modiq-cli`'s
    /// and `modiq-engine`'s own test helper of the same shape. Removed
    /// when dropped.
    struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        fn new(label: &str) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "modiq-storage-test-{}-{}-{}",
                std::process::id(),
                label,
                unique
            ));
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

    fn sample_report() -> AssessmentReport {
        let mut assessment = Assessment::new(
            AssessmentSubject,
            AssessmentContext,
            VersionProfileReference::new("FS25"),
        );
        assessment.begin_evidence_collection().unwrap();
        let evidence = Evidence::new(
            EvidenceCategory::FileStructureAnalysis,
            "missing modDesc.xml",
        )
        .unwrap();
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

        AssessmentReport::generate(&assessment)
    }

    #[test]
    fn store_then_retrieve_within_the_same_process_returns_matching_content() {
        let dir = TempDir::new("round-trip");
        let store = ReportStore::new(dir.path());
        let report = sample_report();
        let expected = PersistedAssessmentReport::from_report(&report);

        let key = store
            .store(&report)
            .expect("store succeeds against a real directory");
        let retrieved = store
            .retrieve(&key)
            .expect("retrieve succeeds for a key just stored");

        assert_eq!(retrieved, expected);
    }

    #[test]
    fn retrieve_is_durable_across_a_new_store_instance_at_the_same_root() {
        // A new `ReportStore` value, rooted at the same directory,
        // stands in for a separate process invocation: nothing is
        // held in memory between store() and this second instance's
        // retrieve() — only the filesystem carries the content
        // across, exactly as it would across a real process boundary.
        let dir = TempDir::new("durable-across-instances");
        let report = sample_report();
        let expected = PersistedAssessmentReport::from_report(&report);

        let key = {
            let writer = ReportStore::new(dir.path());
            writer
                .store(&report)
                .expect("store succeeds against a real directory")
        };

        let reader = ReportStore::new(dir.path());
        let retrieved = reader
            .retrieve(&key)
            .expect("retrieve succeeds for a key stored by a prior instance");

        assert_eq!(retrieved, expected);
    }

    #[test]
    fn retrieve_reports_not_found_for_an_unrecognized_key() {
        let dir = TempDir::new("not-found");
        let store = ReportStore::new(dir.path());

        let result = store.retrieve(&ReportKey::from_raw("no-such-report"));

        assert!(matches!(result, Err(ReportStoreError::NotFound)));
    }

    #[test]
    fn each_store_call_returns_a_distinct_key() {
        let dir = TempDir::new("distinct-keys");
        let store = ReportStore::new(dir.path());
        let report = sample_report();

        let first = store.store(&report).unwrap();
        let second = store.store(&report).unwrap();

        assert_ne!(first, second);
    }
}

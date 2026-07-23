use modiq_engine::engine::AssessmentService;
use modiq_report::report::AssessmentReport;
use modiq_runtime::assessment::{
    AssessmentContext, AssessmentSubject, Evidence, Finding, Recommendation,
};
use modiq_storage::storage::{
    PersistedAssessmentReport, PersistedEvidence, PersistedFinding, PersistedRecommendation,
    ReportKey, ReportStore,
};

/// IPC-safe snapshot of a single Evidence item's existing public data.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct EvidenceEntry {
    id: String,
    category: String,
    description: String,
    location: Option<String>,
}

impl From<&Evidence> for EvidenceEntry {
    fn from(evidence: &Evidence) -> Self {
        Self {
            id: format!("{:?}", evidence.id()),
            category: format!("{:?}", evidence.category()),
            description: evidence.description().to_string(),
            location: evidence.location().map(str::to_string),
        }
    }
}

/// IPC-safe snapshot of a single Finding's existing public data.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct FindingEntry {
    id: String,
    severity: String,
    description: String,
}

impl From<&Finding> for FindingEntry {
    fn from(finding: &Finding) -> Self {
        Self {
            id: format!("{:?}", finding.id()),
            severity: format!("{:?}", finding.severity()),
            description: finding.description().to_string(),
        }
    }
}

/// IPC-safe snapshot of a single Recommendation's existing public data.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct RecommendationEntry {
    id: String,
    action: String,
}

impl From<&Recommendation> for RecommendationEntry {
    fn from(recommendation: &Recommendation) -> Self {
        Self {
            id: format!("{:?}", recommendation.id()),
            action: recommendation.action().to_string(),
        }
    }
}

/// A structured, IPC-safe snapshot of an executed Assessment's state.
///
/// Built from `AssessmentReport` — the same read-only snapshot type
/// `AssessmentService::execute` already returns — never from the
/// `Assessment` aggregate directly.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct AssessmentSummary {
    assessment_id: String,
    evidence_count: usize,
    finding_count: usize,
    recommendation_count: usize,
    evidence: Vec<EvidenceEntry>,
    findings: Vec<FindingEntry>,
    recommendations: Vec<RecommendationEntry>,
    /// `None` only if Storage failed to persist this report — the
    /// Assessment itself has already completed by that point, so a
    /// storage failure is reflected here, not by failing the command.
    stored_report_key: Option<String>,
}

impl From<&AssessmentReport> for AssessmentSummary {
    fn from(report: &AssessmentReport) -> Self {
        Self {
            assessment_id: format!("{:?}", report.assessment_id()),
            evidence_count: report.evidence().len(),
            finding_count: report.findings().len(),
            recommendation_count: report.recommendations().len(),
            evidence: report.evidence().iter().map(EvidenceEntry::from).collect(),
            findings: report.findings().iter().map(FindingEntry::from).collect(),
            recommendations: report
                .recommendations()
                .iter()
                .map(RecommendationEntry::from)
                .collect(),
            stored_report_key: None,
        }
    }
}

/// IPC-safe snapshot of a single persisted Evidence entry.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct PersistedEvidenceEntry {
    category: String,
    description: String,
    location: Option<String>,
}

impl From<&PersistedEvidence> for PersistedEvidenceEntry {
    fn from(evidence: &PersistedEvidence) -> Self {
        Self {
            category: format!("{:?}", evidence.category()),
            description: evidence.description().to_string(),
            location: evidence.location().map(str::to_string),
        }
    }
}

/// IPC-safe snapshot of a single persisted Finding entry.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct PersistedFindingEntry {
    severity: String,
    description: String,
}

impl From<&PersistedFinding> for PersistedFindingEntry {
    fn from(finding: &PersistedFinding) -> Self {
        Self {
            severity: format!("{:?}", finding.severity()),
            description: finding.description().to_string(),
        }
    }
}

/// IPC-safe snapshot of a single persisted Recommendation entry.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct PersistedRecommendationEntry {
    action: String,
}

impl From<&PersistedRecommendation> for PersistedRecommendationEntry {
    fn from(recommendation: &PersistedRecommendation) -> Self {
        Self {
            action: recommendation.action().to_string(),
        }
    }
}

/// A structured, IPC-safe snapshot of a previously-stored report,
/// retrieved independently of running a new Assessment.
///
/// Built from `PersistedAssessmentReport` — Storage's own
/// representation — never a reconstructed `AssessmentReport`; see
/// `docs/engineering/STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md`.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct PersistedReportSummary {
    status: String,
    evidence: Vec<PersistedEvidenceEntry>,
    findings: Vec<PersistedFindingEntry>,
    recommendations: Vec<PersistedRecommendationEntry>,
}

impl From<&PersistedAssessmentReport> for PersistedReportSummary {
    fn from(report: &PersistedAssessmentReport) -> Self {
        Self {
            status: format!("{:?}", report.status()),
            evidence: report
                .evidence()
                .iter()
                .map(PersistedEvidenceEntry::from)
                .collect(),
            findings: report
                .findings()
                .iter()
                .map(PersistedFindingEntry::from)
                .collect(),
            recommendations: report
                .recommendations()
                .iter()
                .map(PersistedRecommendationEntry::from)
                .collect(),
        }
    }
}

/// A fixed, checked-in fixture directory, used as the Sandbox's
/// AssessmentInput so it can exercise the real filesystem Evidence
/// Collector without any file-picker, drag-and-drop, or other UI
/// input workflow (Sprint 3 Phase 5 — those remain explicitly out of
/// scope; see `PROPOSAL_FILESYSTEM_COLLECTION.md`, Sandbox
/// Interaction). Resolved relative to the crate's own manifest
/// directory, not the process's current working directory, so it
/// resolves to the same absolute location regardless of how the
/// Sandbox is launched.
const FIXTURE_ASSESSMENT_INPUT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/fixtures/sample-assessment-input"
);

/// Default root directory Storage persists reports under. Resolved
/// relative to the crate's own manifest directory, mirroring
/// `FIXTURE_ASSESSMENT_INPUT`'s own convention, so it resolves to the
/// same absolute location regardless of how the Sandbox is launched.
/// Not configurable yet — the smallest slice Phase 3 authorizes; a
/// configurable location is a separate, later capability.
const STORAGE_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/.modiq-storage");

/// Executes the Assessment pipeline through `AssessmentService`'s
/// `execute_from_assessment_input` entry point — the same
/// orchestration `execute` performs, now including real Evidence
/// Collection (`modiq-collection`) over an actual filesystem fixture,
/// rather than sandbox-constructed Evidence — and returns the result
/// as a DTO.
///
/// No orchestration, Rule Engine, Evidence Collection, or Runtime
/// logic is reimplemented here: this command only supplies the
/// pipeline's input and maps its already-existing output to an
/// IPC-safe shape. `FIXTURE_ASSESSMENT_INPUT` is a fixed, checked-in
/// directory; it is not a claim about any real Assessment Subject.
#[tauri::command]
fn create_assessment() -> AssessmentSummary {
    create_assessment_with_storage(STORAGE_ROOT)
}

/// The storage-root-parameterized core of `create_assessment`, kept
/// separate so tests can exercise it against a real, hermetic
/// temporary directory rather than `STORAGE_ROOT`'s own fixed
/// location.
fn create_assessment_with_storage(storage_root: &str) -> AssessmentSummary {
    let service = AssessmentService;
    let report = service
        .execute_from_assessment_input(
            AssessmentSubject,
            AssessmentContext,
            FIXTURE_ASSESSMENT_INPUT,
        )
        .expect("the fixture assessment input exists and is accessible");

    let mut summary = AssessmentSummary::from(&report);
    let store = ReportStore::new(storage_root);
    summary.stored_report_key = store.store(&report).ok().map(|key| key.value().to_string());
    summary
}

/// Retrieves a previously-stored report, independent of running a new
/// Assessment — the Sandbox-side proof that a report survives beyond
/// the process that produced it, mirroring `modiq-cli`'s own
/// `retrieve` command.
#[tauri::command]
fn retrieve_report(key: String) -> Result<PersistedReportSummary, String> {
    retrieve_report_with_storage(&key, STORAGE_ROOT)
}

/// The storage-root-parameterized core of `retrieve_report`.
fn retrieve_report_with_storage(
    key: &str,
    storage_root: &str,
) -> Result<PersistedReportSummary, String> {
    let store = ReportStore::new(storage_root);
    store
        .retrieve(&ReportKey::from_raw(key))
        .map(|report| PersistedReportSummary::from(&report))
        .map_err(|error| error.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![create_assessment, retrieve_report])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    /// A real, unique, temporary directory, mirroring `modiq-cli`'s
    /// own test helper of the same shape. Removed when dropped. Used
    /// so these tests exercise Storage against a hermetic location,
    /// never `STORAGE_ROOT`'s own fixed, manifest-relative directory.
    struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        fn new(label: &str) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "modiq-sandbox-test-{}-{}-{}",
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
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }

    /// A fixed, checked-in ZIP archive fixture, used only by this test
    /// module to exercise `AssessmentService::execute_from_assessment_input`'s
    /// archive-routing path (Sprint 4 Phase 3D) through the exact same
    /// production entry point `create_assessment` itself uses.
    /// Deliberately not exposed as a second `#[tauri::command]` or any
    /// new IPC surface — this is validation code only, consistent with
    /// the Sandbox's standing no-file-picker, no-new-input-mechanism
    /// constraint (`PROPOSAL_FILESYSTEM_COLLECTION.md`, Sandbox
    /// Interaction). Mirrors `sample-assessment-input/`'s own
    /// structure (one top-level file, one subdirectory, one nested
    /// file) so the two fixtures are directly comparable.
    const FIXTURE_ARCHIVE_INPUT: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/fixtures/sample-archive-input.zip"
    );

    #[test]
    fn create_assessment_discovers_the_fixture_directory_contents_via_the_real_collector() {
        let storage = TempDir::new("discovers-fixture");
        let summary = create_assessment_with_storage(&storage.path().display().to_string());

        // fixtures/sample-assessment-input contains one top-level file
        // (notes.txt), one subdirectory (nested), and one file within
        // it (nested/detail.txt) — three structural facts for the real
        // filesystem collector to discover — plus one XmlInspection
        // item from XmlCollector (Sprint 7, Multi-Source Evidence
        // Collection), which always runs alongside the structural
        // Collector; this fixture has no modDesc.xml, so that item
        // records absence.
        assert_eq!(summary.evidence_count, 4);
        assert_eq!(summary.evidence.len(), 4);
        assert_eq!(summary.finding_count, 1);
        assert_eq!(summary.recommendation_count, 1);
        assert_eq!(summary.findings.len(), 1);
        assert_eq!(summary.recommendations.len(), 1);
    }

    #[test]
    fn each_invocation_produces_a_distinct_assessment_id() {
        let storage = TempDir::new("distinct-assessment-id");
        let first = create_assessment_with_storage(&storage.path().display().to_string());
        let second = create_assessment_with_storage(&storage.path().display().to_string());

        assert_ne!(first.assessment_id, second.assessment_id);
    }

    #[test]
    fn evidence_entries_reflect_the_fixture_directory_in_deterministic_order() {
        let storage = TempDir::new("deterministic-order");
        let summary = create_assessment_with_storage(&storage.path().display().to_string());

        // Filtered to the structural Collector's own output — Sprint
        // 7's XmlCollector also contributes an XmlInspection item to
        // this same evidence list, which this test (about filesystem
        // traversal order specifically) is not concerned with.
        let structural: Vec<_> = summary
            .evidence
            .iter()
            .filter(|entry| entry.category == "FileStructureAnalysis")
            .collect();

        for entry in &structural {
            assert!(!entry.description.is_empty());
        }

        let locations: Vec<String> = structural
            .iter()
            .map(|entry| entry.location.clone().unwrap_or_default())
            .collect();
        let nested_detail = std::path::Path::new("nested")
            .join("detail.txt")
            .display()
            .to_string();
        assert_eq!(
            locations,
            vec!["nested".to_string(), nested_detail, "notes.txt".to_string()]
        );
    }

    #[test]
    fn evidence_entries_also_include_xml_inspection_output() {
        // Sprint 7: XmlCollector always runs alongside the structural
        // Collector. The fixture has no modDesc.xml, so this records
        // absence — the same "always produces Evidence, never Empty
        // Collection" guarantee `XmlCollector`'s own tests verify
        // directly, exercised here through the real Sandbox pipeline.
        let storage = TempDir::new("xml-inspection-output");
        let summary = create_assessment_with_storage(&storage.path().display().to_string());

        let xml_entries: Vec<_> = summary
            .evidence
            .iter()
            .filter(|entry| entry.category == "XmlInspection")
            .collect();

        assert_eq!(xml_entries.len(), 1);
        assert!(xml_entries[0].description.contains("No modDesc.xml"));
    }

    #[test]
    fn execute_from_assessment_input_discovers_the_fixture_archive_via_the_real_archive_collector()
    {
        let service = AssessmentService;

        let report = service
            .execute_from_assessment_input(
                AssessmentSubject,
                AssessmentContext,
                FIXTURE_ARCHIVE_INPUT,
            )
            .expect("the fixture archive exists and is well-formed");
        let summary = AssessmentSummary::from(&report);

        // sample-archive-input.zip contains one top-level file
        // (notes.txt), one directory entry (nested/), and one nested
        // file (nested/detail.txt) — three structural facts, this
        // time discovered by ArchiveCollector via AssessmentService's
        // explicit .zip routing rather than the filesystem collector —
        // plus one XmlInspection item (Sprint 7): the archive has no
        // modDesc.xml, so XmlCollector records absence.
        assert_eq!(summary.evidence_count, 4);
        assert_eq!(summary.evidence.len(), 4);
        assert_eq!(summary.finding_count, 1);
        assert_eq!(summary.recommendation_count, 1);
    }

    #[test]
    fn archive_evidence_entries_are_categorized_and_described_as_archive_collection_output() {
        let service = AssessmentService;

        let report = service
            .execute_from_assessment_input(
                AssessmentSubject,
                AssessmentContext,
                FIXTURE_ARCHIVE_INPUT,
            )
            .expect("the fixture archive exists and is well-formed");
        let summary = AssessmentSummary::from(&report);

        // Filtered to the structural Collector's own output — see
        // `evidence_entries_reflect_the_fixture_directory_in_deterministic_order`
        // for why.
        for entry in summary
            .evidence
            .iter()
            .filter(|entry| entry.category == "FileStructureAnalysis")
        {
            assert!(entry.description.contains("archive collection"));
        }
    }

    #[test]
    fn the_directory_fixture_path_still_produces_filesystem_collection_output() {
        // Regression guard, specific to this Sprint 4 closeout's
        // Sandbox Archive Validation: confirms the pre-existing
        // create_assessment command's fixed directory target still
        // routes to the filesystem EvidenceCollector, unaffected by
        // the archive-routing addition, and still describes its
        // Evidence as filesystem — not archive — collection output.
        let storage = TempDir::new("still-filesystem-collection");
        let summary = create_assessment_with_storage(&storage.path().display().to_string());

        assert_eq!(summary.evidence_count, 4);
        for entry in summary
            .evidence
            .iter()
            .filter(|entry| entry.category == "FileStructureAnalysis")
        {
            assert!(entry.description.contains("filesystem collection"));
        }
    }

    #[test]
    fn create_assessment_with_storage_stores_a_retrievable_report() {
        let storage = TempDir::new("stores-retrievable-report");

        let summary = create_assessment_with_storage(&storage.path().display().to_string());

        let key = summary
            .stored_report_key
            .clone()
            .expect("storage succeeds against a real, writable directory");

        let retrieved = retrieve_report_with_storage(&key, &storage.path().display().to_string())
            .expect("the just-stored report is retrievable");

        assert_eq!(retrieved.evidence.len(), summary.evidence.len());
        assert_eq!(retrieved.findings.len(), summary.findings.len());
        assert_eq!(
            retrieved.recommendations.len(),
            summary.recommendations.len()
        );
    }

    #[test]
    fn retrieve_report_against_an_unrecognized_key_is_an_error() {
        let storage = TempDir::new("retrieve-not-found");

        let result =
            retrieve_report_with_storage("no-such-key", &storage.path().display().to_string());

        assert!(result.is_err());
    }
}

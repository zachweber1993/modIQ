use modiq_engine::engine::AssessmentService;
use modiq_report::report::AssessmentReport;
use modiq_runtime::assessment::{
    AssessmentContext, AssessmentSubject, Evidence, Finding, Recommendation,
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
    let service = AssessmentService;
    let report = service
        .execute_from_assessment_input(
            AssessmentSubject,
            AssessmentContext,
            FIXTURE_ASSESSMENT_INPUT,
        )
        .expect("the fixture assessment input exists and is accessible");

    AssessmentSummary::from(&report)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![create_assessment])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let summary = create_assessment();

        // fixtures/sample-assessment-input contains one top-level file
        // (notes.txt), one subdirectory (nested), and one file within
        // it (nested/detail.txt) — three structural facts for the real
        // filesystem collector to discover.
        assert_eq!(summary.evidence_count, 3);
        assert_eq!(summary.evidence.len(), 3);
        assert_eq!(summary.finding_count, 1);
        assert_eq!(summary.recommendation_count, 1);
        assert_eq!(summary.findings.len(), 1);
        assert_eq!(summary.recommendations.len(), 1);
    }

    #[test]
    fn each_invocation_produces_a_distinct_assessment_id() {
        let first = create_assessment();
        let second = create_assessment();

        assert_ne!(first.assessment_id, second.assessment_id);
    }

    #[test]
    fn evidence_entries_reflect_the_fixture_directory_in_deterministic_order() {
        let summary = create_assessment();

        for entry in &summary.evidence {
            assert_eq!(entry.category, "FileStructureAnalysis");
            assert!(!entry.description.is_empty());
        }

        let locations: Vec<String> = summary
            .evidence
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
        // explicit .zip routing rather than the filesystem collector.
        assert_eq!(summary.evidence_count, 3);
        assert_eq!(summary.evidence.len(), 3);
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

        for entry in &summary.evidence {
            assert_eq!(entry.category, "FileStructureAnalysis");
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
        let summary = create_assessment();

        assert_eq!(summary.evidence_count, 3);
        for entry in &summary.evidence {
            assert!(entry.description.contains("filesystem collection"));
        }
    }
}

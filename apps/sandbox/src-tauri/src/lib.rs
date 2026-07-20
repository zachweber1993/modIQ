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

/// Executes the Assessment pipeline through `AssessmentService`'s
/// `execute_from_descriptor` entry point — the same orchestration
/// `execute` performs, now including real Evidence Collection
/// (`modiq-collection`) driven by one deterministic Input Descriptor,
/// rather than sandbox-constructed Evidence — and returns the result
/// as a DTO.
///
/// No orchestration, Rule Engine, Evidence Collection, or Runtime
/// logic is reimplemented here: this command only supplies the
/// pipeline's input and maps its already-existing output to an
/// IPC-safe shape. The Input Descriptor is a fixed, deterministic
/// placeholder; it is not a claim about any real Assessment Subject.
#[tauri::command]
fn create_assessment() -> AssessmentSummary {
    let service = AssessmentService;
    let report = service
        .execute_from_descriptor(
            AssessmentSubject,
            AssessmentContext,
            "Deterministic bootstrap input for verifying the Evidence Collection pipeline end-to-end.",
        )
        .expect("the bootstrap input descriptor is non-empty");

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

    #[test]
    fn create_assessment_produces_one_of_each_via_the_real_pipeline() {
        let summary = create_assessment();

        assert_eq!(summary.evidence_count, 1);
        assert_eq!(summary.finding_count, 1);
        assert_eq!(summary.recommendation_count, 1);
        assert_eq!(summary.evidence.len(), 1);
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
    fn evidence_entry_reflects_the_deterministic_bootstrap_content() {
        let summary = create_assessment();

        assert_eq!(summary.evidence[0].category, "FileStructureAnalysis");
        assert!(!summary.evidence[0].description.is_empty());
    }
}

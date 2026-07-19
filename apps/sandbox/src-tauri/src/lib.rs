use modiq_runtime::assessment::{Assessment, AssessmentContext, AssessmentSubject};

/// A structured, IPC-safe snapshot of a newly created Assessment's
/// initial state.
///
/// This DTO exists specifically so the `Assessment` aggregate itself
/// is never exposed across the Tauri boundary: React only ever sees
/// this summary, never the Runtime type it was derived from.
///
/// `assessment_id` is currently the `Debug` representation of
/// `AssessmentId` (e.g. `"AssessmentId(1)"`) — `modiq-runtime` exposes
/// no accessor or `Display`/`Serialize` implementation for the value
/// it wraps, and this phase does not modify `modiq-runtime`. See the
/// accompanying engineering summary for this known limitation.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct AssessmentSummary {
    assessment_id: String,
    evidence_count: usize,
    finding_count: usize,
    recommendation_count: usize,
}

impl From<&Assessment> for AssessmentSummary {
    fn from(assessment: &Assessment) -> Self {
        Self {
            assessment_id: format!("{:?}", assessment.id()),
            evidence_count: assessment.evidence().len(),
            finding_count: assessment.findings().len(),
            recommendation_count: assessment.recommendations().len(),
        }
    }
}

/// Constructs a real Assessment via `modiq-runtime` and returns its
/// initial state as a DTO.
///
/// Performs no other business logic: no lifecycle transitions, no
/// Rule Engine evaluation, no Evidence/Finding/Recommendation
/// construction. The Assessment is not persisted or retained after
/// this call returns — each invocation creates and immediately
/// summarizes a fresh, ephemeral Assessment, matching the "no
/// application state" constraint for this phase.
#[tauri::command]
fn create_assessment() -> AssessmentSummary {
    let assessment = Assessment::new(AssessmentSubject, AssessmentContext);
    AssessmentSummary::from(&assessment)
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
    fn a_newly_created_assessment_summarizes_with_zero_counts() {
        let assessment = Assessment::new(AssessmentSubject, AssessmentContext);

        let summary = AssessmentSummary::from(&assessment);

        assert_eq!(summary.evidence_count, 0);
        assert_eq!(summary.finding_count, 0);
        assert_eq!(summary.recommendation_count, 0);
    }

    #[test]
    fn each_assessment_summary_reflects_a_distinct_assessment_id() {
        let first = Assessment::new(AssessmentSubject, AssessmentContext);
        let second = Assessment::new(AssessmentSubject, AssessmentContext);

        let first_summary = AssessmentSummary::from(&first);
        let second_summary = AssessmentSummary::from(&second);

        assert_ne!(first_summary.assessment_id, second_summary.assessment_id);
    }
}

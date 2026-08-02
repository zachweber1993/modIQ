//! The Request/Response Mechanism's engine-facing half (Phase 2/3, Sprint 21).
//!
//! This is the only module in this crate permitted to depend on a
//! `modiq-*` crate — the concrete realization of the Boundary
//! Enforcement `lib.rs` establishes structurally (see `Cargo.toml`).
//! It calls `AssessmentService::execute_from_assessment_input` exactly
//! as `modiq-cli` and `apps/sandbox` already do, and builds
//! `ReportSummary` from `AssessmentReport`'s already-public getters —
//! never a serialized Runtime type.
//!
//! `ReportSummary` is provisional, not final or stable — Authorization
//! §4 holds the concrete request/response payload shape open pending
//! GOV-008. Every field here has an immediate consumer in Phase 3's
//! Reviewing implementation; nothing is included because the Runtime
//! happens to expose it. Notably: `AssessmentReport` exposes evidence,
//! findings, and recommendations as three flat, parallel lists — a
//! `Finding` references its own Evidence by id (`evidence_ids`), and a
//! `Recommendation` references its own Finding(s) by id
//! (`finding_ids`). `ReportSummary` resolves both associations here,
//! at the boundary, so presentation code receives Evidence and
//! Recommendation already scoped to the Finding they belong to and
//! never needs to perform that lookup itself.

use std::collections::HashMap;

use modiq_engine::engine::AssessmentService;
use modiq_report::report::AssessmentReport;
use modiq_runtime::assessment::{AssessmentContext, AssessmentSubject, Evidence, FindingId};

/// IPC-safe snapshot of one Evidence item, scoped to the Finding that
/// references it. No `category` field: nothing in Phase 3 reads it —
/// `description` and `location` are Evidence's own always-visible
/// content (`EVIDENCE_EXPLORATION.md`); there is no separate raw
/// `Content` field on the Runtime type to gate behind a further
/// reveal.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceSummary {
    id: String,
    description: String,
    location: Option<String>,
}

impl From<&Evidence> for EvidenceSummary {
    fn from(evidence: &Evidence) -> Self {
        Self {
            id: format!("{:?}", evidence.id()),
            description: evidence.description().to_string(),
            location: evidence.location().map(str::to_string),
        }
    }
}

/// IPC-safe snapshot of one Finding, with its own Evidence resolved
/// and its own Recommendation (if any) resolved — never a bare
/// Finding requiring presentation code to perform either lookup.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FindingSummary {
    id: String,
    severity: String,
    description: String,
    recommendation: Option<String>,
    evidence: Vec<EvidenceSummary>,
}

/// The provisional Request/Response Mechanism transport. No top-level
/// counts, no recommendation-exists flag, no assessment identity:
/// every one of those is either fully derivable client-side from
/// `findings` (counts, recommendation existence) or already known
/// client-side from the selected path (identity) — see
/// `SPRINT21_PLAN.md` Phase 3 and this Sprint's own Overview-derivation
/// decision.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportSummary {
    pub findings: Vec<FindingSummary>,
}

impl From<&AssessmentReport> for ReportSummary {
    fn from(report: &AssessmentReport) -> Self {
        let evidence_by_id: HashMap<_, &Evidence> = report
            .evidence()
            .iter()
            .map(|item| (item.id(), item))
            .collect();

        // A Recommendation may reference more than one Finding
        // (`Recommendation::finding_ids`); this repository's current
        // Rule Engine produces at most one applicable Recommendation
        // per Finding in practice, so the first match is taken. This
        // is a real, present-tense fact about today's Rule Engine, not
        // an assumption about its future shape.
        let recommendation_for = |finding_id: FindingId| -> Option<String> {
            report
                .recommendations()
                .iter()
                .find(|recommendation| recommendation.finding_ids().contains(&finding_id))
                .map(|recommendation| recommendation.action().to_string())
        };

        let findings = report
            .findings()
            .iter()
            .map(|finding| FindingSummary {
                id: format!("{:?}", finding.id()),
                severity: format!("{:?}", finding.severity()),
                description: finding.description().to_string(),
                recommendation: recommendation_for(finding.id()),
                evidence: finding
                    .evidence_ids()
                    .iter()
                    .filter_map(|id| evidence_by_id.get(id).copied())
                    .map(EvidenceSummary::from)
                    .collect(),
            })
            .collect();

        Self { findings }
    }
}

/// Executes a real Assessment against `input_path` and returns a
/// provisional summary of its result — see this module's own doc
/// comment for what is deliberately not included.
#[tauri::command]
pub fn submit_assessment(input_path: String) -> Result<ReportSummary, String> {
    submit_assessment_from_path(&input_path)
}

/// The testable core of `submit_assessment`, kept separate so tests
/// exercise it directly rather than through Tauri's own command
/// dispatch — mirroring `apps/sandbox`'s own established pattern.
fn submit_assessment_from_path(input_path: &str) -> Result<ReportSummary, String> {
    let service = AssessmentService;
    service
        .execute_from_assessment_input(AssessmentSubject, AssessmentContext, input_path)
        .map(|report| ReportSummary::from(&report))
        .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE_SAMPLE_MOD: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/sample-mod");

    #[test]
    fn submit_assessment_succeeds_against_a_real_fixture_directory() {
        let result = submit_assessment_from_path(FIXTURE_SAMPLE_MOD);
        assert!(result.is_ok());
    }

    #[test]
    fn a_real_fixture_produces_at_least_one_finding() {
        let result = submit_assessment_from_path(FIXTURE_SAMPLE_MOD).unwrap();
        assert!(!result.findings.is_empty());
    }

    #[test]
    fn every_finding_carries_a_non_empty_severity_and_description() {
        let result = submit_assessment_from_path(FIXTURE_SAMPLE_MOD).unwrap();
        for finding in &result.findings {
            assert!(!finding.severity.is_empty());
            assert!(!finding.description.is_empty());
        }
    }

    #[test]
    fn submit_assessment_fails_against_a_nonexistent_path() {
        assert!(submit_assessment_from_path("/no/such/path/modiq-console-test").is_err());
    }
}

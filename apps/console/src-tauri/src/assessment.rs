//! The Request/Response Mechanism's engine-facing half (Phase 2, Sprint 21).
//!
//! This is the only module in this crate permitted to depend on a
//! `modiq-*` crate — the concrete realization of the Boundary
//! Enforcement `lib.rs` establishes structurally (see `Cargo.toml`).
//! It calls `AssessmentService::execute_from_assessment_input` exactly
//! as `modiq-cli` and `apps/sandbox` already do, and reports only
//! whether that call succeeded — no Evidence, Finding, or
//! Recommendation content crosses this boundary. Authorization §4
//! holds the concrete request/response payload shape open pending
//! GOV-008; a bare success/failure signal, rather than any decomposed
//! Report content, commits to nothing that decision has not yet
//! settled.

use modiq_engine::engine::AssessmentService;
use modiq_runtime::assessment::{AssessmentContext, AssessmentSubject};

/// Executes a real Assessment against `input_path` and reports only
/// whether it completed. The `AssessmentReport` this call necessarily
/// produces exists solely to prove completion; none of its content is
/// transported — see this module's own doc comment.
#[tauri::command]
pub fn submit_assessment(input_path: String) -> Result<(), String> {
    submit_assessment_from_path(&input_path)
}

/// The testable core of `submit_assessment`, kept separate so tests
/// exercise it directly rather than through Tauri's own command
/// dispatch — mirroring `apps/sandbox`'s own established pattern.
fn submit_assessment_from_path(input_path: &str) -> Result<(), String> {
    let service = AssessmentService;
    service
        .execute_from_assessment_input(AssessmentSubject, AssessmentContext, input_path)
        .map(|_report| ())
        .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE_SAMPLE_MOD: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/sample-mod");

    #[test]
    fn submit_assessment_succeeds_against_a_real_fixture_directory() {
        assert!(submit_assessment_from_path(FIXTURE_SAMPLE_MOD).is_ok());
    }

    #[test]
    fn submit_assessment_fails_against_a_nonexistent_path() {
        assert!(submit_assessment_from_path("/no/such/path/modiq-console-test").is_err());
    }
}

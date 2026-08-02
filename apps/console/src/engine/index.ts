import { invoke } from "@tauri-apps/api/core";

/**
 * The Request/Response Mechanism's only public entry point (Phase 2,
 * Sprint 21). This is the sole file permitted to call Tauri's
 * `invoke` anywhere in this application — the frontend half of the
 * Boundary Enforcement `src-tauri/src/assessment.rs` establishes on
 * the Rust side.
 *
 * Resolves once a real Assessment has completed; rejects with the
 * failure's message otherwise. Carries no Evidence, Finding, or
 * Recommendation content — Authorization §4 holds the concrete
 * request/response payload shape open pending GOV-008, and a bare
 * success/failure signal commits to nothing that decision has not
 * yet settled.
 */
export async function submitAssessment(inputPath: string): Promise<void> {
  await invoke("submit_assessment", { inputPath });
}

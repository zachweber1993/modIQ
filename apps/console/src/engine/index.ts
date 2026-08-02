import { invoke } from "@tauri-apps/api/core";
import type { ReportSummary } from "./types";

export type { ReportSummary, FindingSummary, EvidenceSummary } from "./types";

/**
 * The Request/Response Mechanism's only public entry point. This is
 * the sole file permitted to call Tauri's `invoke` anywhere in this
 * application — the frontend half of the Boundary Enforcement
 * `src-tauri/src/assessment.rs` establishes on the Rust side.
 *
 * Resolves once a real Assessment has completed with the provisional
 * `ReportSummary`; rejects with the failure's message otherwise.
 */
export async function submitAssessment(
  inputPath: string,
): Promise<ReportSummary> {
  return invoke<ReportSummary>("submit_assessment", { inputPath });
}

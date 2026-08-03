/**
 * The Request/Response Mechanism's provisional transport (Phase 3,
 * Sprint 21). Not final or stable — FRONTEND_IMPLEMENTATION_AUTHORIZATION.md
 * §4 holds the concrete payload shape open pending GOV-008.
 *
 * Every field here has an immediate consumer in the Reviewing
 * implementation (`src/workspace/`). Nothing is included because the
 * Runtime happens to expose it, and no field mirrors `apps/sandbox`'s
 * own DTO shape for that reason — see the Phase 3 transport review
 * for the field-by-field justification. Notably absent, and why:
 *
 * - No top-level counts or a recommendation-exists flag — both are
 *   fully derivable from `findings` (see `src/workspace/Overview.tsx`).
 * - No assessment identity — Overview's "what was assessed" uses the
 *   folder path already known client-side from the picker (Phase 2),
 *   not engine data.
 * - No Mod Health dimension, no Evidence `Content`/`Label`/`Source`
 *   field, no Confidence — Sprint 22 (Initiative 3) added these to
 *   the Runtime types, but nothing on this side of the boundary
 *   consumes them yet; Title/Summary are the one Sprint 22 addition
 *   this transport does carry, replacing the prior flat `description`.
 */

export interface EvidenceSummary {
  id: string;
  description: string;
  location: string | null;
}

export interface FindingSummary {
  id: string;
  severity: string;
  title: string;
  summary: string;
  recommendation: string | null;
  evidence: EvidenceSummary[];
}

export interface ReportSummary {
  findings: FindingSummary[];
}

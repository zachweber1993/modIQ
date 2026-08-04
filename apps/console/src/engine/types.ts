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
 * - No Confidence — Confidence does not exist on the Runtime at all
 *   (Initiative 4, unimplemented); nothing to expose. Mod Health
 *   dimension and Evidence `Label`/`Source`/`Content` were added to
 *   the Runtime by Sprint 22 (Initiative 3) and are carried by this
 *   transport as of Sprint 23 (`FindingSummary.modHealthDimension`/
 *   `status`, `EvidenceSummary.label`/`source`/`content`, below) —
 *   Title/Summary were the one Sprint 22 addition this transport
 *   already carried, replacing the prior flat `description`.
 */

export interface EvidenceSummary {
  id: string;
  description: string;
  location: string | null;
  label: string | null;
  source: string | null;
  content: string | null;
}

export interface RecommendationStepSummary {
  kind: string;
  instruction: string;
}

export interface RecommendationSummary {
  action: string;
  repairSteps: RecommendationStepSummary[];
}

export interface FindingSummary {
  id: string;
  severity: string;
  title: string;
  summary: string;
  modHealthDimension: string;
  status: string;
  recommendation: RecommendationSummary | null;
  evidence: EvidenceSummary[];
}

export interface ReportSummary {
  findings: FindingSummary[];
}

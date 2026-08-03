import type { FindingSummary } from "@/engine";

/**
 * Overview — a pure derived view, never a transported or separately
 * stored object (WORKSPACE_EVOLUTION.md §6, THE_ASSESSMENT_REPORT.md
 * §3). It reflects facts already present in `findings`; it introduces
 * none of its own — the same discipline `DashboardAndConsole.md`
 * already establishes for Console and Dashboard.
 *
 * Mod Health, by dimension, is not aggregated here. `ModHealthDimension`
 * exists on the Runtime as of Sprint 22 (Initiative 3) and is carried
 * by the transport as of Sprint 23, but is surfaced per-Finding in
 * `Reviewing.tsx`'s own expansion, not summarized at this level — a
 * per-dimension aggregate would be a form of grouping, outside Sprint
 * 23 Phase 3's authorized scope. Only what this Sprint authorizes is
 * presented here: severity counts and whether any Recommendation
 * exists, per Explainable Continuity (`FrontendArchitecture.md`):
 * nothing here is a fact this consumer invented.
 */

const SEVERITY_ORDER = ["Error", "Warning", "Informational", "BestPractice"];

export function Overview({ findings }: { findings: FindingSummary[] }) {
  const counts = SEVERITY_ORDER.map((severity) => ({
    severity,
    count: findings.filter((finding) => finding.severity === severity).length,
  })).filter((entry) => entry.count > 0);

  const hasRecommendation = findings.some(
    (finding) => finding.recommendation !== null,
  );

  return (
    <div className="flex flex-col gap-2 border-b border-border pb-4">
      {findings.length === 0 ? (
        <p className="text-sm text-muted-foreground">No Findings.</p>
      ) : (
        <div className="flex gap-4 text-sm text-foreground">
          {counts.map(({ severity, count }) => (
            <span key={severity}>
              {severity}: {count}
            </span>
          ))}
        </div>
      )}
      <p className="text-sm text-muted-foreground">
        {hasRecommendation
          ? "Recommendations are available."
          : "No Recommendations."}
      </p>
    </div>
  );
}

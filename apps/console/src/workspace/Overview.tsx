import type { FindingSummary } from "@/engine";

/**
 * Overview — a pure derived view, never a transported or separately
 * stored object (WORKSPACE_EVOLUTION.md §6, THE_ASSESSMENT_REPORT.md
 * §3). It reflects facts already present in `findings`; it introduces
 * none of its own — the same discipline `DashboardAndConsole.md`
 * already establishes for Console and Dashboard.
 *
 * Mod Health, by dimension, is presented here as of Report Anatomy
 * Alignment (WORKSPACE_EVOLUTION.md §6 item 3): each represented
 * dimension is shown independently, as a count, never collapsed into
 * one composite value — no score, formula, weighting, or ranking is
 * computed anywhere in this component. `ModHealthDimension` already
 * exists on every `FindingSummary` (Sprint 22/23); this component
 * only aggregates what is already there, per Explainable Continuity
 * (`FrontendArchitecture.md`) — nothing here is a fact this consumer
 * invented.
 */

const SEVERITY_ORDER = ["Error", "Warning", "Informational", "BestPractice"];

const DIMENSION_ORDER = [
  "Compatibility",
  "Stability",
  "Maintainability",
  "Performance",
  "Structure",
  "EngineeringQuality",
];

export function Overview({ findings }: { findings: FindingSummary[] }) {
  const counts = SEVERITY_ORDER.map((severity) => ({
    severity,
    count: findings.filter((finding) => finding.severity === severity).length,
  })).filter((entry) => entry.count > 0);

  const dimensionCounts = DIMENSION_ORDER.map((dimension) => ({
    dimension,
    count: findings.filter(
      (finding) => finding.modHealthDimension === dimension,
    ).length,
  })).filter((entry) => entry.count > 0);

  const hasRecommendation = findings.some(
    (finding) => finding.recommendation !== null,
  );

  return (
    <div className="flex flex-col gap-2 border-b border-border pb-4">
      {findings.length === 0 ? (
        <p className="text-sm text-muted-foreground">No Findings.</p>
      ) : (
        <>
          <div className="flex gap-4 text-sm text-foreground">
            {counts.map(({ severity, count }) => (
              <span key={severity}>
                {severity}: {count}
              </span>
            ))}
          </div>
          <div className="flex gap-4 text-sm text-foreground">
            {dimensionCounts.map(({ dimension, count }) => (
              <span key={dimension}>
                {dimension}: {count}
              </span>
            ))}
          </div>
        </>
      )}
      <p className="text-sm text-muted-foreground">
        {hasRecommendation
          ? "Recommendations are available."
          : "No Recommendations."}
      </p>
    </div>
  );
}

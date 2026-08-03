import { useState } from "react";
import type { FindingSummary } from "@/engine";
import { Overview } from "./Overview";

/**
 * Reviewing (Phase 3, Sprint 21) — Overview, Finding, Recommendation,
 * and Evidence as one continuous object at varying focus
 * (NAVIGATION_AND_WORKSPACE_BEHAVIOR.md), never separate destinations.
 *
 * `expandedFindingId` is the entire navigation state: which Finding,
 * if any, is currently examined more closely. Expanding is one step
 * from Overview (single-step locality); it reveals that Finding's own
 * Recommendation and Evidence together, in the same single expansion
 * layer THE_FINDING.md specifies — there is no further, separately
 * navigable Evidence level, because the Runtime carries no Evidence
 * `Content` field distinct from `description` for a further reveal to
 * disclose. Collapsing uses the identical setter, inverted (symmetric
 * reversal). The Overview above the list is never replaced while a
 * Finding is expanded (cumulative orientation).
 *
 * No affordance returns to Intake: Sprint 21 does not implement
 * reentrancy or supplementation (Initiative 2), so once Reviewing is
 * reached there is deliberately nowhere else for this phase to go.
 */
export function Reviewing({
  findings,
  subjectLabel,
}: {
  findings: FindingSummary[];
  subjectLabel: string;
}) {
  const [expandedFindingId, setExpandedFindingId] = useState<string | null>(
    null,
  );

  // Ordered by Severity, per WORKSPACE_EVOLUTION.md's residual rule
  // once Category (unavailable) is set aside. A stable sort preserves
  // the engine's own relative order among Findings of equal severity —
  // this reorders nothing the engine did not already determine.
  const ordered = [...findings].sort(
    (a, b) => severityRank(a.severity) - severityRank(b.severity),
  );

  return (
    <div className="flex h-full flex-col gap-4 overflow-auto p-4">
      <p className="text-sm text-muted-foreground">
        Reviewing: {subjectLabel}
      </p>
      <Overview findings={findings} />
      <ul className="flex flex-col gap-2">
        {ordered.map((finding) => {
          const expanded = expandedFindingId === finding.id;
          return (
            <li
              key={finding.id}
              className="rounded-lg border border-border p-3"
            >
              <button
                type="button"
                className="flex w-full items-start gap-2 text-left"
                onClick={() =>
                  setExpandedFindingId(expanded ? null : finding.id)
                }
              >
                <span className="shrink-0 text-xs font-medium uppercase text-muted-foreground">
                  {finding.severity}
                </span>
                <span className="text-foreground">{finding.summary}</span>
              </button>
              {expanded && (
                <div className="mt-3 flex flex-col gap-3 border-t border-border pt-3">
                  {finding.recommendation && (
                    <p className="text-sm text-foreground">
                      Recommendation: {finding.recommendation}
                    </p>
                  )}
                  {finding.evidence.length > 0 && (
                    <ul className="flex flex-col gap-1">
                      {finding.evidence.map((evidence) => (
                        <li
                          key={evidence.id}
                          className="text-sm text-muted-foreground"
                        >
                          {evidence.description}
                          {evidence.location ? ` (${evidence.location})` : ""}
                        </li>
                      ))}
                    </ul>
                  )}
                </div>
              )}
            </li>
          );
        })}
      </ul>
    </div>
  );
}

const SEVERITY_ORDER = ["Error", "Warning", "Informational", "BestPractice"];

function severityRank(severity: string): number {
  const index = SEVERITY_ORDER.indexOf(severity);
  return index === -1 ? SEVERITY_ORDER.length : index;
}

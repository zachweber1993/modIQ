import { useState } from "react";
import type { FindingSummary } from "@/engine";
import { Overview } from "./Overview";

/**
 * Reviewing (Phase 3, Sprint 21; fields extended Sprint 23 Phase 3;
 * anatomy aligned to the frozen specification by Report Anatomy
 * Alignment) — Overview, Finding, Recommendation, and Evidence as one
 * continuous object at varying focus
 * (NAVIGATION_AND_WORKSPACE_BEHAVIOR.md), never separate destinations.
 *
 * Two independent, symmetric disclosure toggles hold all local state:
 * `expandedFindingId` — which Finding, if any, is currently examined
 * more closely (single-step locality from Overview); and
 * `revealedEvidenceIds` — which Evidence items, if any, have had
 * their own substantial `content` explicitly revealed (THE_FINDING.md
 * §4, EVIDENCE.md §4's second, nested layer of progressive
 * disclosure). Neither is navigation — both are disclosure state
 * within the single Reviewing surface. Expanding a Finding reveals
 * its own Mod Health dimension, Status, Recommendation headline (now
 * also visible collapsed, per THE_FINDING.md §4), full Recommendation
 * detail, and Evidence together, in the same single expansion layer
 * THE_FINDING.md specifies. Evidence's `content` field (Sprint 22) is
 * shown inline within that same single expansion layer when brief,
 * and behind one further, explicit reveal when substantial
 * (EVIDENCE.md §4) — this is a second disclosure step nested inside
 * the one Finding-expansion layer, not a new expansion level.
 * Collapsing uses the identical setters, inverted (symmetric
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
  const [revealedEvidenceIds, setRevealedEvidenceIds] = useState<Set<string>>(
    new Set(),
  );

  const revealEvidence = (evidenceId: string) =>
    setRevealedEvidenceIds((prev) => {
      const next = new Set(prev);
      next.add(evidenceId);
      return next;
    });

  // Grouped by Mod Health dimension, per THE_FINDING.md §5 — the same
  // canonical dimension order Overview.tsx uses for its own per-
  // dimension presentation. Within each dimension group, ordered by
  // Severity, unchanged from the prior flat sort (severityRank is
  // reused verbatim, only re-scoped to apply per group rather than
  // once over the whole list) — a stable sort preserves the engine's
  // own relative order among Findings of equal severity within a
  // dimension, reordering nothing the engine did not already
  // determine. Dimensions with no represented Finding are omitted,
  // not shown as empty groups.
  const grouped = DIMENSION_ORDER.map((dimension) => ({
    dimension,
    findings: findings
      .filter((finding) => finding.modHealthDimension === dimension)
      .sort((a, b) => severityRank(a.severity) - severityRank(b.severity)),
  })).filter((group) => group.findings.length > 0);

  return (
    <div className="flex h-full flex-col gap-4 overflow-auto p-4">
      <p className="text-sm text-muted-foreground">
        Reviewing: {subjectLabel}
      </p>
      <Overview findings={findings} />
      <div className="flex flex-col gap-4">
        {grouped.map(({ dimension, findings: dimensionFindings }) => (
          <div key={dimension} className="flex flex-col gap-2">
            <h3 className="text-xs font-medium uppercase text-muted-foreground">
              {dimension}
            </h3>
            <ul className="flex flex-col gap-2">
              {dimensionFindings.map((finding) => {
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
                      <span className="flex flex-col gap-0.5">
                        <span className="text-foreground">
                          {finding.title}
                        </span>
                        <span className="text-sm text-muted-foreground">
                          {finding.summary}
                        </span>
                        <span className="text-xs uppercase text-muted-foreground">
                          {finding.modHealthDimension} · {finding.status}
                        </span>
                        {finding.recommendation && (
                          <span className="text-sm text-foreground">
                            Recommendation: {finding.recommendation.action}
                          </span>
                        )}
                        <span className="text-xs text-muted-foreground">
                          {finding.evidence.length > 0
                            ? `Evidence available (${finding.evidence.length})`
                            : "No Evidence."}
                        </span>
                      </span>
                    </button>
                    {expanded && (
                      <div className="mt-3 flex flex-col gap-3 border-t border-border pt-3">
                        {finding.recommendation &&
                          finding.recommendation.repairSteps.length > 0 && (
                            <ul className="flex flex-col gap-1 pl-2">
                              {finding.recommendation.repairSteps.map(
                                (step, index) => (
                                  <li
                                    key={index}
                                    className="text-sm text-muted-foreground"
                                  >
                                    <span className="font-medium text-foreground">
                                      {step.kind}:{" "}
                                    </span>
                                    {step.instruction}
                                  </li>
                                ),
                              )}
                            </ul>
                          )}
                        {finding.evidence.length > 0 && (
                          <ul className="flex flex-col gap-1">
                            {finding.evidence.map((evidence) => {
                              const isSubstantial =
                                !!evidence.content &&
                                evidence.content.length >
                                  EVIDENCE_CONTENT_REVEAL_THRESHOLD;
                              const isRevealed = revealedEvidenceIds.has(
                                evidence.id,
                              );
                              return (
                                <li
                                  key={evidence.id}
                                  className="text-sm text-muted-foreground"
                                >
                                  {evidence.label && (
                                    <span className="font-medium text-foreground">
                                      {evidence.label}:{" "}
                                    </span>
                                  )}
                                  {evidence.description}
                                  {evidence.location
                                    ? ` (${evidence.location})`
                                    : ""}
                                  {evidence.source
                                    ? ` — ${evidence.source}`
                                    : ""}
                                  {evidence.content &&
                                    (!isSubstantial || isRevealed) && (
                                      <div className="mt-0.5 pl-2 text-xs italic">
                                        {evidence.content}
                                      </div>
                                    )}
                                  {evidence.content &&
                                    isSubstantial &&
                                    !isRevealed && (
                                      <button
                                        type="button"
                                        className="mt-0.5 block pl-2 text-xs text-muted-foreground underline"
                                        onClick={() =>
                                          revealEvidence(evidence.id)
                                        }
                                      >
                                        Show content
                                      </button>
                                    )}
                                </li>
                              );
                            })}
                          </ul>
                        )}
                      </div>
                    )}
                  </li>
                );
              })}
            </ul>
          </div>
        ))}
      </div>
    </div>
  );
}

const SEVERITY_ORDER = ["Error", "Warning", "Informational", "BestPractice"];

const DIMENSION_ORDER = [
  "Compatibility",
  "Stability",
  "Maintainability",
  "Performance",
  "Structure",
  "EngineeringQuality",
];

// Presentation-only threshold distinguishing brief Evidence content
// (shown inline) from substantial content (behind one further reveal),
// per EVIDENCE.md §4. Not an architectural or data-model distinction —
// purely a local rendering decision over the already-delivered
// `content` string.
const EVIDENCE_CONTENT_REVEAL_THRESHOLD = 120;

function severityRank(severity: string): number {
  const index = SEVERITY_ORDER.indexOf(severity);
  return index === -1 ? SEVERITY_ORDER.length : index;
}

/**
 * Workspace — the per-Assessment-Subject region (WORKSPACE_EVOLUTION.md,
 * NAVIGATION_AND_WORKSPACE_BEHAVIOR.md). Phase 1 mounts this region
 * with no content: Assessment Input acquisition, submission, and the
 * Reviewing experience are Phase 2/3 work (SPRINT21_PLAN.md §3), not
 * this phase's. This is an honest empty state, not a placeholder for
 * fabricated Assessment content.
 */
export function Workspace() {
  return (
    <div className="flex h-full items-center justify-center text-muted-foreground">
      <p>Workspace — Intake arrives in Phase 2.</p>
    </div>
  );
}

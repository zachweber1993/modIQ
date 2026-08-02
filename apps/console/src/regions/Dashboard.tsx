import { useSession } from "@/session/SessionContext";

/**
 * Dashboard — the default interaction region Console presents
 * (DashboardAndConsole.md). Presents Platform state; introduces none
 * of its own. Every fact here is read from the session's own real
 * Organization/Project representation (Authorization §6) — never
 * fabricated. No Upload exists yet in Phase 1, so the Upload list is
 * genuinely empty, not a placeholder.
 */
export function Dashboard() {
  const { session } = useSession();
  if (!session) return null;

  return (
    <div className="flex flex-col gap-6">
      <section>
        <h2 className="text-sm font-medium text-muted-foreground">
          Organization
        </h2>
        <p className="text-lg text-foreground">{session.organization.name}</p>
      </section>
      <section>
        <h2 className="text-sm font-medium text-muted-foreground">
          Projects
        </h2>
        <div className="rounded-lg border border-border p-4">
          <p className="text-foreground">{session.project.name}</p>
          <p className="mt-1 text-sm text-muted-foreground">
            No Uploads yet.
          </p>
        </div>
      </section>
    </div>
  );
}

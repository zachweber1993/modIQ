import { useState } from "react";
import { Button } from "@/components/ui/button";
import { useSession } from "@/session/SessionContext";
import { Dashboard } from "./Dashboard";
import { Workspace } from "./Workspace";

type Region = "dashboard" | "workspace";

/**
 * Console — the platform's persistent interaction shell
 * (DashboardAndConsole.md). Owns no Platform concept and has no
 * independent lifecycle; it only hosts regions and reflects which one
 * is active. Region content ownership stays with Dashboard and
 * Workspace themselves.
 */
export function Console() {
  const [active, setActive] = useState<Region>("dashboard");
  const { session, signOut } = useSession();

  if (!session) return null;

  return (
    <div className="flex h-screen flex-col bg-background text-foreground">
      <header className="flex items-center justify-between border-b border-border px-4 py-2">
        <nav className="flex gap-1">
          <Button
            variant={active === "dashboard" ? "secondary" : "ghost"}
            size="sm"
            onClick={() => setActive("dashboard")}
          >
            Dashboard
          </Button>
          <Button
            variant={active === "workspace" ? "secondary" : "ghost"}
            size="sm"
            onClick={() => setActive("workspace")}
          >
            Workspace
          </Button>
        </nav>
        <div className="flex items-center gap-2 text-sm text-muted-foreground">
          <span>{session.organization.name}</span>
          <Button variant="ghost" size="sm" onClick={signOut}>
            Sign out
          </Button>
        </div>
      </header>
      <main className="flex-1 overflow-auto p-6">
        {active === "dashboard" ? <Dashboard /> : <Workspace />}
      </main>
    </div>
  );
}

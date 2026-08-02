import { createContext, useContext, useState, type ReactNode } from "react";
import type { Session } from "./types";

interface SessionContextValue {
  session: Session | null;
  signIn: (name: string) => void;
  signOut: () => void;
}

const SessionContext = createContext<SessionContextValue | undefined>(
  undefined,
);

let idCounter = 0;
function nextId(prefix: string): string {
  idCounter += 1;
  return `${prefix}-${idCounter}`;
}

/**
 * Establishes the minimal, real Platform Architecture representation
 * FRONTEND_IMPLEMENTATION_AUTHORIZATION.md §6 authorizes: signing in
 * constructs one real Membership, one Organization, and one
 * auto-provisioned Project for that session — never fabricated
 * display data, and never more than Application Shell/Region
 * Composition and Identity and Session Mechanism themselves require
 * (Authorization §4 excludes anything further).
 *
 * No credential is verified against anything: IdentityAndAccess.md
 * itself defines no persistence or credential mechanism to verify
 * against, and building one is explicitly excluded. Nothing here
 * persists beyond the running session.
 */
export function SessionProvider({ children }: { children: ReactNode }) {
  const [session, setSession] = useState<Session | null>(null);

  const signIn = (name: string) => {
    const user = { id: nextId("user"), name };
    const organization = {
      id: nextId("org"),
      name: `${name}'s Organization`,
    };
    const project = {
      id: nextId("project"),
      organizationId: organization.id,
      name: "Default Project",
    };
    const membership = {
      userId: user.id,
      organizationId: organization.id,
      role: "Owner" as const,
    };
    setSession({ user, membership, organization, project });
  };

  const signOut = () => setSession(null);

  return (
    <SessionContext.Provider value={{ session, signIn, signOut }}>
      {children}
    </SessionContext.Provider>
  );
}

export function useSession(): SessionContextValue {
  const context = useContext(SessionContext);
  if (!context) {
    throw new Error("useSession must be used within a SessionProvider");
  }
  return context;
}

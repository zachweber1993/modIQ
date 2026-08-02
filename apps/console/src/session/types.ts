/**
 * Minimal Platform Architecture representation, scoped exactly to what
 * FRONTEND_IMPLEMENTATION_AUTHORIZATION.md §6 authorizes: only what
 * Application Shell/Region Composition and Identity and Session
 * Mechanism themselves require to function. This is not a claim about
 * a real, multi-user Platform Architecture implementation — see
 * FRONTEND_IMPLEMENTATION_AUTHORIZATION.md §4 for what remains
 * explicitly excluded (full Membership/Role/Organization
 * implementation, region richness across multiple Projects).
 */

export type Role = "Owner" | "Member";

export interface User {
  id: string;
  name: string;
}

export interface Organization {
  id: string;
  name: string;
}

export interface Membership {
  userId: string;
  organizationId: string;
  role: Role;
}

/**
 * Auto-provisioned per ProjectsAndUploads.md's own resolved
 * Architectural Decision Deferred: a Project is "auto-created,
 * provisioned automatically when none is designated" — never a
 * mandatory, user-authored setup step.
 */
export interface Project {
  id: string;
  organizationId: string;
  name: string;
}

export interface Session {
  user: User;
  membership: Membership;
  organization: Organization;
  project: Project;
}

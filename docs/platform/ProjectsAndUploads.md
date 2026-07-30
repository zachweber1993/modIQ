# Projects & Uploads

| Property | Value |
|---|---|
| **Document** | ProjectsAndUploads.md |
| **Status** | Frozen — Platform Architecture 1.0 (2026-07-30) |
| **Project** | modIQ |
| **Owner** | Zach Weber |
| **Created** | 2026-07-29 |
| **Last Updated** | 2026-07-30 |

---

## Specification Authority

**Authority:**

- `docs/platform/PlatformSpecification.md`

This document introduces no first-class Platform Architecture concept. It refines Project and Upload, both defined in `PlatformSpecification.md`, remains consistent with `IdentityAndAccess.md`'s treatment of Organization, User, Membership, and API Client without redefining any of them, and governs no further document.

If a conflict exists between this document and `PlatformSpecification.md`, `PlatformSpecification.md` takes precedence.

---

## Purpose

This document refines Project and Upload — what each represents in more concrete terms than `PlatformSpecification.md`'s deliberately terse definitions, how they relate to their immediate neighbors (Organization above, Assessment below), and their lifecycles — and resolves the one Architectural Decision Deferred assigned to it: whether a Project is mandatory or automatically created.

It assumes the reader has `PlatformSpecification.md`'s and `IdentityAndAccess.md`'s definitions in hand and does not restate them.

---

## Platform Principles

Consistent with `IdentityAndAccess.md`'s Identity Principles:

- **Project and Upload are refined with the smallest sufficient model.** No intermediate ownership concept is introduced between Organization and Project, or between Project and Assessment.
- **Upload's immutability, once submitted, is preserved exactly as `PlatformSpecification.md` states it — never reinterpreted or loosened.**
- **Where the platform must choose between requiring manual setup and provisioning automatically, this document prefers the posture `IdentityAndAccess.md` already established for Organization** — provisioned automatically rather than required in advance — **over inventing a second, inconsistent posture for Project.**
- **No new first-class Platform Architecture concept was found necessary.** Every question this document resolves is answered using Project, Upload, Organization, and Assessment alone.

---

## Project

**Relationship to Organization.** Unchanged from `PlatformSpecification.md`: a Project belongs to exactly one Organization. Any Membership granting standing in that Organization grants standing over all of its Projects, per `IdentityAndAccess.md`'s Access Relationships.

**Relationship to Upload.** A Project is the exclusive scope an Upload is submitted into. An Upload belonging to no Project cannot exist — see *Resolved Architectural Decisions* for how a Project comes to exist to receive one.

**Relationship to Assessment.** A Project contains every Assessment triggered by the Uploads submitted into it. Unchanged from `PlatformSpecification.md`.

**Archival.** Archiving a Project requires Owner standing, applying `IdentityAndAccess.md`'s Role definition rather than introducing a new one. An archived Project's existing Assessment record remains exactly as visible and traceable as before archival — archival changes whether new Uploads may be submitted into it, not whether its past is retained.

**Deletion.** Not a Platform Architecture concept. `PlatformSpecification.md`'s Lifecycle Model for Project has exactly two reachable states beyond Created — Active and Archived. Whether archived Project data is ever physically removed at a later layer is an implementation and retention question, outside this lineage.

**Lifecycle.** Unchanged from `PlatformSpecification.md`: **Created → Active → (Archived)**. A Project may come to exist through a deliberate act or through automatic provisioning (see *Resolved Architectural Decisions*); the Lifecycle Model does not distinguish between the two.

---

## Upload

**What Upload represents.** The platform-level fact that specific material was submitted into a specific Project, by a specific User or API Client, at a specific time, and which Assessment it triggered. Upload does not represent the material's own nature or content — what kind of Assessment Subject it became, or what it contains — which remains Runtime Architecture's concern, referenced here only as "the Assessment it triggered."

**When it becomes part of a Project.** Immediately and permanently. An Upload record never exists independent of a Project, and is never reassigned to a different one afterward, consistent with its constitutional immutability.

**Relationship to Assessment.** Each Upload triggers exactly one Assessment. Unchanged from `PlatformSpecification.md`, and not a new constraint this document imposes on Runtime Architecture — the relationship was already fixed at the constitutional layer; this document only inherits it. A desire to reassess material already submitted is satisfied by a new Upload, triggering its own Assessment; the original Upload and the Assessment it triggered are never retargeted or reopened. This keeps the Platform Architecture model neutral on whether that new Assessment continues or restarts for the same Assessment Subject — an open Runtime Architecture question this document does not take a position on.

**Persistence.** Unchanged from `PlatformSpecification.md`: Upload is a persistent Platform record, not a transient file reference.

**Lifecycle.** Unchanged from `PlatformSpecification.md`: **Submitted → Processed → Retained, permanently and immutably.** No further state is adopted.

---

## Resolved Architectural Decisions

This document resolves the one item `PlatformSpecification.md`'s Architectural Decisions Deferred table assigned to it:

| Decision | Resolution |
|---|---|
| Is a Project mandatory, or auto-created implicitly? | Auto-created. A Project always exists to receive an Upload — created explicitly in advance, or provisioned automatically when none has been designated. Explicit, deliberate Project creation remains available and is expected for Organizations coordinating multiple Assessment Subjects together. |

**Rationale.** Two considerations point the same direction. First, `IdentityAndAccess.md` already resolved an analogous question for Organization the same way: registration provisions the container a User needs rather than requiring manual setup first. Treating Project differently — requiring explicit creation before a first Upload — would apply two inconsistent postures to structurally similar problems without a reason for the difference. Second, `ASSESSMENT_INTAKE_AND_UPLOAD.md`'s frozen design principle states that beginning an Assessment "costs the user nothing to try" and is "a single, intentionally simple action." A mandatory Project-creation step interposed before a first Upload would add exactly the kind of friction that document was certified as deliberately excluding.

This specification introduces no Platform Architecture decision beyond this one, delegated to it by `PlatformSpecification.md`.

---

## Boundaries

Unchanged from `PlatformSpecification.md`: Assessment is the floor. This document describes containment up to that point only — "Upload triggers an Assessment," "a Project contains Assessments" — and does not describe what an Assessment does once triggered.

This document does not define Runtime Architecture, assessment execution, persistence mechanisms, API contracts, storage implementation, queueing, file systems, or report generation. It does not redefine Organization, User, Membership, API Client, or Assessment — all remain exactly as `PlatformSpecification.md` and `IdentityAndAccess.md` already define them.

---

## Document Status

**Approved** — the second governed specification of the Platform Architecture lineage. Ratified per `PlatformSpecification.md`'s Platform Architecture Governance section, which permits this document to refine and apply the concepts it delegates without expanding the Platform Domain Model.

**Frozen 2026-07-30**, as part of Platform Architecture 1.0, per `PlatformSpecification.md`'s Document Status. Any future change to this document is a governed amendment to Platform Architecture, not ordinary editing.

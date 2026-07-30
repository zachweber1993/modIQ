# Dashboard & Console

| Property | Value |
|---|---|
| **Document** | DashboardAndConsole.md |
| **Status** | Approved |
| **Project** | modIQ |
| **Owner** | Zach Weber |
| **Created** | 2026-07-30 |
| **Last Updated** | 2026-07-30 |

---

## Specification Authority

**Authority:**

- `docs/platform/PlatformSpecification.md`

Consistent with `IdentityAndAccess.md` and `ProjectsAndUploads.md` — neither is redefined by this document. This document introduces no first-class Platform Architecture concept and governs no further document.

If a conflict exists between this document and `PlatformSpecification.md`, `PlatformSpecification.md` takes precedence.

---

## Purpose

This document refines Console and Dashboard — the two experience-layer elements `PlatformSpecification.md`'s Scope names but explicitly excludes from the Platform Domain Model. It establishes Console as the persistent interaction shell and Dashboard as its default region, references Workspace only to establish its containment relationship to Console, and defines no behavior belonging to `WayfindingAndSearch.md` or `Notifications.md`.

It assumes the reader has `PlatformSpecification.md`, `IdentityAndAccess.md`, and `ProjectsAndUploads.md`'s definitions in hand and does not restate them.

---

## Interaction Principles

- **Console is the persistent interaction shell.** It reflects Platform state, derives visibility and organization from already-approved Platform concepts, contains interaction regions, and introduces no Platform state of its own.
- **Dashboard is the default interaction region contained within Console.** It presents Platform state, deriving everything it presents entirely from the Platform Domain Model. It owns no Platform concept and introduces no Platform state.
- **Workspace is referenced, not redefined.** It is a contained interaction region already owned by Product Design; this document establishes only its containment relationship to Console.
- **Neither Dashboard nor Workspace owns Platform concepts.** They exist solely to organize interaction with Platform concepts already defined elsewhere.
- **The interaction layer reflects Platform Architecture. It never infers, computes, reinterprets, or owns Platform state.** This is the same discipline `ProjectsAndUploads.md` applied to Upload's relationship to Assessment, extended one layer up: presentation follows from what already exists; it never decides what exists.

---

## Platform Vocabulary

One verb per responsibility, maintained consistently across the lineage:

| Platform Element | Responsibility |
|---|---|
| Membership | determines visibility |
| Project | organizes work |
| Upload | records submission |
| Assessment | exists |
| Console | reflects |
| Dashboard | presents |
| Workspace | focuses assessment-specific interaction |

---

## Console

Console is the platform's persistent interaction shell — present regardless of which region is currently active.

Console reflects Platform state; it does not create it. Which Organization is active is derived from the User's Memberships (`IdentityAndAccess.md`); what that Organization contains is derived from its Projects (`ProjectsAndUploads.md`). Every fact Console reflects already exists in the Platform Domain Model before Console reflects it.

Console contains interaction regions. Two are established here: Dashboard, the default region, and Workspace, a region already defined by Product Design and referenced, not redefined, below. Console does not determine what a region contains — that remains each region's own responsibility — only that a region is active within the shell.

Console owns no Platform concept and has no independent lifecycle, exactly as `PlatformSpecification.md`'s Domain Model already states of the platform experience layer generally: no independent ownership, no independent lifecycle.

---

## Dashboard

Dashboard is the default interaction region Console presents — the region shown before a User has navigated into anything more specific.

Dashboard presents Platform state; it does not reflect the shell-level facts Console reflects, and it introduces no state of its own. What it presents is derived entirely from the Platform Domain Model: the Projects belonging to the active Organization, the Uploads submitted into those Projects, and the fact that an Assessment exists for a given Upload — never an Assessment's internal state, only that one exists, per the Architectural Boundary `PlatformSpecification.md` already establishes.

Dashboard owns no Platform concept and has no independent lifecycle, for the same reason Console has none: it is a region of the shell, not a Platform Architecture concept in its own right.

---

## Workspace (Referenced, Not Redefined)

Workspace is a second interaction region contained within Console, already fully defined by Product Design (`WORKSPACE_EVOLUTION.md`) and Interaction Design (`NAVIGATION_AND_WORKSPACE_BEHAVIOR.md`). This document's only claim about Workspace is structural: it is a region Console contains, alongside Dashboard. Its own states, its own behavior, and its own relationship to a single Assessment Subject remain entirely Product Design's and Interaction Design's, unrepeated and unredefined here.

---

## Organization Switching

Console may contain an Organization selector as persistent shell chrome — a structural fact about what Console contains, not a behavior this document defines. A User's Memberships determine which Organizations are available to select; Console reflects that set. What happens when a selection is made — the switching behavior itself, any visibility change, routing, or transition — belongs entirely to `WayfindingAndSearch.md` and is not defined here.

---

## Architectural Decisions Deferred

`PlatformSpecification.md`'s Architectural Decisions Deferred table assigns no item to this document. Its one remaining open item — Notification addressing scope — belongs to `Notifications.md`. This specification resolves nothing beyond what `PlatformSpecification.md`, `IdentityAndAccess.md`, and `ProjectsAndUploads.md` have already settled.

---

## Boundaries

This document does not define: navigation, movement between Organizations, movement between Projects, movement between Assessments, search, notification behavior, notification delivery, runtime interaction, or Workspace behavior.

It does not redefine Organization, User, Membership, Project, Upload, Assessment, Notification, Subscription, API Client, or Public Visitor — all remain exactly as `PlatformSpecification.md`, `IdentityAndAccess.md`, and `ProjectsAndUploads.md` already define them.

---

## Future Specification Boundaries

`WayfindingAndSearch.md` owns navigation semantics, movement, search, transition behavior, and organization-switching behavior — the behavior itself, not the structural fact that Console contains a selector (see *Organization Switching*, above).

`Notifications.md` owns notification behavior, delivery, addressing, and lifecycle.

This document acknowledges that Console contains shell elements associated with a future capability only where one was established above (the Organization selector) — it does not anticipate or acknowledge any other.

---

## Architectural Boundary

Dashboard and Console are interaction surfaces over the Platform Domain Model. They do not own Organization, Membership, Project, Upload, Assessment, or Notification — they organize and present those concepts without redefining them.

Any interaction beyond the boundaries of a Console region — for example, movement between Organizations, Projects, or Assessments; search; or notification delivery — is governed by the Platform specifications responsible for Wayfinding, Search, and Notifications, not by this one.

---

## Document Status

**Approved** — the third governed specification of the Platform Architecture lineage. Ratified per `PlatformSpecification.md`'s Platform Architecture Governance section, which permits this document to refine and apply the concepts it delegates without expanding the Platform Domain Model.

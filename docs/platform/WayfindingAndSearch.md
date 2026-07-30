# Wayfinding & Search

| Property | Value |
|---|---|
| **Document** | WayfindingAndSearch.md |
| **Status** | Frozen — Platform Architecture 1.0 (2026-07-30) |
| **Project** | modIQ |
| **Owner** | Zach Weber |
| **Created** | 2026-07-30 |
| **Last Updated** | 2026-07-30 |

---

## Specification Authority

**Authority:**

- `docs/platform/PlatformSpecification.md`

Consistent with `IdentityAndAccess.md`, `ProjectsAndUploads.md`, and `DashboardAndConsole.md` — none is redefined by this document. This document introduces no first-class Platform Architecture concept and governs no further document.

If a conflict exists between this document and `PlatformSpecification.md`, `PlatformSpecification.md` takes precedence.

---

## Purpose

This document refines how Users move among already-existing Platform resources and how existing Platform resources are located. It establishes movement and discovery as interaction-layer responsibilities over an already-established Platform Domain Model. It defines neither Platform state nor interaction behavior within a region.

It assumes the reader has `PlatformSpecification.md`, `IdentityAndAccess.md`, `ProjectsAndUploads.md`, and `DashboardAndConsole.md`'s definitions in hand and does not restate them.

---

## Governing Principles

**Wayfinding changes which Platform resource and interaction region are active.** It does not alter the internal state of an interaction region, create Platform state, mutate Platform state, or participate in Platform lifecycle actions.

**Identity & Access determines what a User may see.** Wayfinding operates only over the Platform resources `IdentityAndAccess.md`'s Membership already makes visible. It never evaluates, duplicates, or extends authorization — a resource outside a User's visible set is not withheld by Wayfinding, it is simply not navigable.

**Search locates existing Platform resources.** It never creates new Platform state and never changes navigation by itself. Locating and moving remain separate responsibilities, held by separate elements of this document.

---

## Platform Vocabulary

One primary verb per architectural responsibility:

| Element | Responsibility |
|---|---|
| Membership | determines visibility |
| Project | organizes work |
| Upload | records submission |
| Assessment | exists |
| Console | reflects |
| Dashboard | presents |
| Wayfinding | moves |
| Search | locates |
| Workspace | focuses assessment-specific interaction |

---

## Navigable Platform Resources

The only navigable Platform resources are Organization, Project, and Assessment — up to the Workspace boundary. No other Platform concept is independently navigable.

Upload and Notification are presented, not navigated to (`DashboardAndConsole.md`). Interaction regions — Dashboard, Workspace, and any future region — are destinations within the interaction layer, not Platform resources in their own right. Wayfinding governs movement to an interaction region only as necessary to activate the already-selected Platform resource represented there.

---

## Wayfinding

Wayfinding moves. It changes which Platform resource and interaction region are active — never what exists, never what a region does once active.

Wayfinding operates only over the Organizations, Projects, and Assessments a User's Membership already makes visible. It performs no independent authorization of its own.

Movement into a Project, or to an Assessment's Workspace threshold, is Wayfinding's. So is movement triggered from within Dashboard's own presented content — clicking a Project Dashboard lists is a transition, and transitions are Wayfinding's regardless of where the control that starts them happens to be displayed, exactly as `DashboardAndConsole.md` already established for Console's Organization selector.

---

## Search

Search locates. It exposes existing Platform resources as navigable targets — it creates nothing and moves nothing itself.

Search and Wayfinding remain distinct even where their outputs meet: Search may surface a Project as a result; moving to it is Wayfinding's act, not Search's. Neither owns the other.

---

## Architectural Boundaries

**Wayfinding owns:** movement between Organizations, movement between Projects, movement to an Assessment Workspace, activation of interaction regions.

**Wayfinding does not own:** Workspace behavior, Dashboard presentation, authorization, Platform lifecycle, Platform state, notification behavior, runtime behavior.

**Search owns:** locating existing Platform resources, exposing existing navigable targets.

**Search does not own:** authorization, navigation, presentation, filtering Platform state, ranking implementation, runtime search behavior.

---

## Interaction Layer Boundary

Wayfinding changes which Platform resource and interaction region are active. It never alters the internal state of an interaction region. Dashboard presents; Wayfinding moves; Workspace behaves according to Product Design and Interaction Design. Those responsibilities remain independent of one another.

---

## Identity Boundary

Visibility is entirely determined by Membership. Wayfinding never determines whether a Platform resource is visible — resources outside a User's Membership-derived visible set are simply not navigable. Wayfinding therefore performs no independent authorization.

---

## Assessment Boundary

Assessment is navigable only up to the Workspace boundary. Wayfinding is responsible for reaching the Assessment Workspace. Everything that occurs once inside it remains governed by Product Design and Interaction Design.

---

## Search Boundary

Search locates. Wayfinding moves. Search may expose a Platform resource as a navigable destination; Wayfinding governs movement to that destination. Neither owns the other.

---

## Future Specification Boundaries

`Notifications.md` owns notification communication and Platform-level notification lifecycle — not runtime transport (email, push delivery, queues, polling, or infrastructure), which remains Runtime and implementation's concern.

Runtime search implementation — indexing, ranking, query execution — is an implementation concern, outside Platform Architecture entirely, not delegated to any governed document.

Workspace behavior, product workflows, assessment workflows, and interaction inside any region remain Product Design's and Interaction Design's, unrepeated and unredefined here.

---

## Architectural Decisions Deferred

`PlatformSpecification.md`'s Architectural Decisions Deferred table assigns no item to this document. Its one remaining open item — Notification addressing scope — belongs to `Notifications.md`. This specification resolves nothing beyond what `PlatformSpecification.md`, `IdentityAndAccess.md`, `ProjectsAndUploads.md`, and `DashboardAndConsole.md` have already settled.

---

## Document Status

**Approved** — the fourth governed specification of the Platform Architecture lineage. Ratified per `PlatformSpecification.md`'s Platform Architecture Governance section, which permits this document to refine and apply the concepts it delegates without expanding the Platform Domain Model.

**Amended 2026-07-30:** revised the Future Specification Boundaries' forward reference to `Notifications.md` to distinguish Platform-level notification communication and lifecycle (owned) from runtime transport mechanisms (not owned). Wording synchronization only — no boundary, concept, or responsibility changed.

**Frozen 2026-07-30**, as part of Platform Architecture 1.0, per `PlatformSpecification.md`'s Document Status. Any future change to this document is a governed amendment to Platform Architecture, not ordinary editing.

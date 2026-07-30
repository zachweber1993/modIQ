# Notifications

| Property | Value |
|---|---|
| **Document** | Notifications.md |
| **Status** | Approved |
| **Project** | modIQ |
| **Owner** | Zach Weber |
| **Created** | 2026-07-30 |
| **Last Updated** | 2026-07-30 |

---

## Specification Authority

**Authority:**

- `docs/platform/PlatformSpecification.md`

Consistent with `IdentityAndAccess.md`, `ProjectsAndUploads.md`, `DashboardAndConsole.md`, and `WayfindingAndSearch.md` — none is redefined by this document. This document introduces no first-class Platform Architecture concept and governs no further document.

If a conflict exists between this document and `PlatformSpecification.md`, `PlatformSpecification.md` takes precedence.

---

## Purpose

This document refines how already-existing Platform state is communicated. It defines neither Platform state, presentation, navigation, authorization, nor runtime delivery.

It assumes the reader has `PlatformSpecification.md`, `IdentityAndAccess.md`, `ProjectsAndUploads.md`, `DashboardAndConsole.md`, and `WayfindingAndSearch.md`'s definitions in hand and does not restate them.

---

## Architectural Inheritance

Notifications operate over Platform state the prior four governed specifications have already established, and redefine none of it:

- `IdentityAndAccess.md` — visibility.
- `ProjectsAndUploads.md` — Platform existence.
- `DashboardAndConsole.md` — presentation.
- `WayfindingAndSearch.md` — movement and discovery.

---

## Governing Principles

**Notifications communicate already-existing Platform state.** They do not create Platform state, mutate Platform state, authorize Platform state, present Platform state, navigate Platform state, or participate in Platform lifecycle actions.

**Identity & Access determines visibility.** Notifications communicate only Platform state already visible through `IdentityAndAccess.md`. They never determine visibility or perform independent authorization.

**Projects, Uploads, and Assessments determine what exists.** Notifications communicate those existing facts. They never establish them.

**Dashboard presents. Wayfinding moves. Search locates. Notifications communicate.** These remain independent architectural concerns.

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
| Notification | communicates |
| Workspace | focuses assessment-specific interaction |

---

## Notification Responsibility

Notifications communicate facts. They never become the source of those facts. The existence of a Notification must never imply ownership of the underlying Platform state — a Notification about a Project is evidence that something happened, never the thing that happened.

Notifications are generated in response to an event on another concept, exactly as `PlatformSpecification.md` already states: "Generated in response to an event on any other concept." This document does not expand what counts as such an event; it refines only how the resulting Notification behaves once generated.

---

## Notification Lifecycle at the Platform Architecture Level

`PlatformSpecification.md`'s Lifecycle Model is unchanged: Generated → Delivered → Read/Dismissed → (Retained or Expired).

"Delivered," at this layer, means only that a Notification exists and is associated with its recipient — a Platform Architecture fact, not a transport claim. Whether any particular delivery mechanism actually reached the User is a Runtime and implementation concern (see *Runtime Boundary*) and is not what this Lifecycle state asserts.

Read/Dismissed is a fact about the Notification record itself, not about the underlying Platform state — dismissing a Notification never changes the Project, Upload, or Assessment it was about.

---

## Architectural Boundaries

**Notifications own:** communication of Platform state, notification relationships (which event generated a Notification, which User it's addressed to), notification visibility (whether a given Notification record is pending or already Read/Dismissed — not whether the underlying Platform state is visible, which remains Identity & Access's), and notification lifecycle at the Platform Architecture level.

**Notifications do not own:** authorization, presentation, navigation, search, Platform state, runtime delivery, transport mechanisms, user interface behavior, product workflows, assessment workflows.

---

## Identity Boundary

Visibility is entirely determined by Membership. Notifications never determine whether Platform state is visible. Notifications communicate only Platform state already visible to the User through `IdentityAndAccess.md`.

---

## Runtime Boundary

This specification does not define delivery mechanisms, email, push notifications, webhooks, polling, subscriptions, queues, indexing, transport, timing, or infrastructure. Those belong to Runtime and implementation.

This boundary is not new caution introduced here — `PlatformSpecification.md`'s own Architectural Dependencies section already flags that Notification behavior tied to in-progress Assessment activity depends on an intermediate observation point the engine does not yet confirmably expose (`ENGINEERING_ALIGNMENT_PROGRAM.md` §1, still open as of Initiative 1's Architectural Resolution). This document does not design around that capability; it defines Notification's architecture independent of whether or when it becomes available.

---

## Future Boundaries

This specification does not anticipate Product Design or implementation decisions. It does not define notification rendering, notification placement, notification UI, notification prioritization algorithms, or delivery infrastructure. It establishes architectural responsibility only.

---

## Architectural Decisions Deferred

This document resolves the one remaining item in `PlatformSpecification.md`'s Architectural Decisions Deferred table:

| Decision | Resolution |
|---|---|
| Can a Notification target an entire Organization, or only a User? | User-only, confirmed. |

**Rationale.** Notification remains constitutionally User-owned — `PlatformSpecification.md`'s Ownership Model places it there directly, not as an inference from how Notifications happen to be generated. Organization-wide addressing would require either redefining that ownership relationship or introducing a new mediating Platform concept to hold it; neither is permitted without a constitutional amendment to `PlatformSpecification.md`. An Organization-wide occurrence is therefore represented by distinct, User-owned Notifications — one for each Member `IdentityAndAccess.md` determines holds standing — rather than by an Organization-addressed Notification that the Domain Model does not provide for.

With this resolved, all four items in `PlatformSpecification.md`'s Architectural Decisions Deferred table are now settled. This specification introduces no Platform Architecture decision beyond this one.

---

## Document Status

**Approved** — the fifth governed specification of the Platform Architecture lineage. Ratified per `PlatformSpecification.md`'s Platform Architecture Governance section, which permits this document to refine and apply the concepts it delegates without expanding the Platform Domain Model.

# Identity & Access

| Property | Value |
|---|---|
| **Document** | IdentityAndAccess.md |
| **Status** | Frozen — Platform Architecture 1.0 (2026-07-30) |
| **Project** | modIQ |
| **Owner** | Zach Weber |
| **Created** | 2026-07-29 |
| **Last Updated** | 2026-07-29 |

---

## Specification Authority

**Authority:**

- `docs/platform/PlatformSpecification.md`

This document introduces no first-class Platform Architecture concept. It refines Organization, User, Membership, and API Client — all defined in `PlatformSpecification.md` — and does not govern any further document.

If a conflict exists between this document and `PlatformSpecification.md`, `PlatformSpecification.md` takes precedence.

---

## Purpose

This document refines four concepts `PlatformSpecification.md` establishes but deliberately leaves under-specified: how a User comes to hold standing in an Organization, what that standing permits, how an API Client's grant differs from a User's, and how two of `PlatformSpecification.md`'s Architectural Decisions Deferred are resolved.

It assumes the reader has `PlatformSpecification.md`'s definitions of Organization, User, Membership, and API Client in hand and does not restate them here.

---

## Identity Principles

This document extends `PlatformSpecification.md`'s Platform Architecture Filter with the discipline specific to identity and access:

- **Identity and access are modeled with the smallest set of concepts necessary.** Organization, User, Membership, and API Client — four concepts, unchanged from `PlatformSpecification.md` — are sufficient for every relationship this document defines.
- **Membership remains the sole relationship granting standing within an Organization.** No second relationship concept is introduced for Role-scoping, Project-scoping, or any other refinement below.
- **Role is an attribute of Membership rather than a first-class Platform Architecture concept.** This specification refines the values of that attribute without expanding the Platform Domain Model established by `PlatformSpecification.md`.
- **Additional structural complexity is introduced only through demonstrated need and governed amendment**, never anticipated in advance of a real requirement.

These principles are not new — they follow directly from `PlatformSpecification.md`'s Platform Architecture Governance section and its preference for the smallest sufficient model. Stating them here makes that discipline explicit for a document whose entire purpose is to add detail without adding concepts.

---

## Access Relationships

A Membership's standing is Organization-wide, not Project-scoped. A User holding any Membership in an Organization can see every Project that Organization owns; what differs by Role (below) is what the User may *do* within them, never which Projects are visible.

**Rationale:** a per-Project grant would require a second relationship concept sitting between Membership and Project, duplicating what Membership already does at a coarser grain. A future need for per-Project access control would require an approved amendment to `PlatformSpecification.md`, not an extension made here.

---

## Role

**Role is an attribute of Membership** — the specific value of the "standing" `PlatformSpecification.md` already says a Membership carries — not a concept of its own.

Two Role values are defined:

**Owner.** Full standing. May grant and revoke other Memberships, manage the Organization's Subscription, and create or archive Projects.

**Member.** Participating standing. May view and act within the Organization's Projects. May not grant or revoke Memberships, may not manage the Subscription.

**Owner-only Project creation does not extend to automatic Project provisioning.** `ProjectsAndUploads.md` resolves that a Project is provisioned automatically when an Upload arrives with none designated to receive it. That provisioning is not the deliberate, administrative act the Owner-only restriction above governs — it is a consequence of submitting an Upload, which any Member may already do under their standing to "act within the Organization's Projects." Requiring Owner standing for automatic provisioning would gate a new Member's first submission behind a capability they may not hold, defeating the reason automatic provisioning exists. Any Member or API Client may trigger it; only deliberate, explicit Project creation remains Owner-only.

These two values are the minimum Role vocabulary the platform currently requires — not a closed or complete constitutional set. Additional Role values require an approved amendment to this specification, not to `PlatformSpecification.md`.

Every Organization always has at least one Membership whose Role is Owner — a direct consequence of Organization's own Lifecycle Model (`PlatformSpecification.md`): an Organization with no Membership carrying that Role would have no path back to Active management, which nothing in the constitutional Lifecycle Model provides for.

---

## API Client Access

An API Client's grant is Organization-wide, not Role- or Project-scoped.

**Rationale:** Membership exists specifically to mediate a User's standing because a single User may hold Memberships in more than one Organization — the relationship object is necessary because the same identity can have multiple, independent standings. An API Client has no analogous multiplicity: `PlatformSpecification.md`'s own Ownership Model already places API Client as directly Organization-owned, not reached through any relationship object. Introducing Role-scoping for API Client would mean inventing a second mediating concept to solve a problem Membership exists to solve for User — unjustified without evidence that an Organization needs to grant an API Client less than full standing, which nothing available today demonstrates.

---

## Resolved Architectural Decisions

This document resolves two of the four items in `PlatformSpecification.md`'s Architectural Decisions Deferred table:

| Decision | Resolution |
|---|---|
| Does every Project require an Organization, including a solo User's? | Yes. See *Project Ownership Root*. |
| Does an API Client's grant carry Role/Project scope, or is it Organization-wide? | Organization-wide. See *API Client Access*. |

This specification introduces no Platform Architecture decision beyond those two, delegated to it by `PlatformSpecification.md`. The remaining two items in that table — Notification addressing scope, and whether a Project is mandatory or auto-created — belong to `Notifications.md` and `ProjectsAndUploads.md` respectively and are not addressed here.

---

## Project Ownership Root

Every Project belongs to an Organization; there is no path for a Project to exist without one. This includes a solo User acting alone: registration creates exactly one Organization together with that User's own Membership as its Owner, before any Project exists.

An Organization with exactly one Owner and no other Members is not a different kind of Organization — it is the same concept `PlatformSpecification.md` defines, simply with one Membership instead of several. No new concept, status, or Lifecycle state is introduced to distinguish it.

---

## Boundaries

This document does not define Project or Upload behavior (`ProjectsAndUploads.md`), the platform shell (`DashboardAndConsole.md`), or anything at or beneath the Assessment (Runtime Architecture, per `PlatformSpecification.md`'s Architectural Boundary — unchanged and not revisited here).

It defines no persistence, no credential mechanism, no session model, and no API contract. How a Membership or an API Client's standing is actually verified at request time is an implementation concern, out of scope for this lineage entirely.

---

## Document Status

**Approved** — the first governed specification of the Platform Architecture lineage. Ratified per `PlatformSpecification.md`'s Platform Architecture Governance section, which permits this document to refine and apply the concepts it delegates without expanding the Platform Domain Model.

**Amended 2026-07-29:** added a clarification to the Role section stating that the Owner-only restriction on Project creation applies to deliberate, administrative creation and does not extend to automatic Project provisioning under `ProjectsAndUploads.md`'s resolution — that provisioning follows from any Member's or API Client's standing to submit an Upload. No Role value was added or changed; no Platform Domain Model concept was introduced.

**Frozen 2026-07-30**, as part of Platform Architecture 1.0, per `PlatformSpecification.md`'s Document Status. Any future change to this document is a governed amendment to Platform Architecture, not ordinary editing.

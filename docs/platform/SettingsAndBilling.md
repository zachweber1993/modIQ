# Settings & Billing

| Property | Value |
|---|---|
| **Document** | SettingsAndBilling.md |
| **Status** | Approved |
| **Project** | modIQ |
| **Owner** | Zach Weber |
| **Created** | 2026-07-31 |
| **Last Updated** | 2026-07-31 |

---

## Specification Authority

**Authority:**

- `docs/platform/PlatformSpecification.md`

Consistent with `IdentityAndAccess.md`, `ProjectsAndUploads.md`, `DashboardAndConsole.md`, `WayfindingAndSearch.md`, and `Notifications.md` — none is redefined by this document. This document introduces no first-class Platform Architecture concept and governs no further document.

If a conflict exists between this document and `PlatformSpecification.md`, `PlatformSpecification.md` takes precedence.

---

## Purpose

This document refines Settings — the platform experience-layer element `PlatformSpecification.md`'s Scope names but explicitly excludes from the Platform Domain Model — and resolves the specific responsibility `PlatformSpecification.md` assigns this document by name: "the experience of managing a Subscription — plan selection, invoice visibility — never what a plan costs or contains."

It establishes Settings as a non-owning interaction region presenting, and enabling management action over, Platform state already defined elsewhere — Organization, User, Membership, Subscription, and API Client — exactly as `DashboardAndConsole.md` established for Dashboard's relationship to Project, Upload, and Assessment existence.

It assumes the reader has `PlatformSpecification.md`, `IdentityAndAccess.md`, `ProjectsAndUploads.md`, `DashboardAndConsole.md`, `WayfindingAndSearch.md`, and `Notifications.md`'s definitions in hand and does not restate them.

---

## Platform Principles

- **Settings is a non-owning interaction region.** It reflects Platform state and surfaces action on it; it never infers, computes, reinterprets, or owns that state — the same discipline `DashboardAndConsole.md` established for Console and Dashboard, applied here without modification.
- **Settings presents and enables management of Organization, User, Membership, Subscription, and API Client — never redefining any of them.** Every fact Settings presents already exists in the Platform Domain Model before Settings presents it.
- **Mutating actions available through Settings invoke lifecycle transitions this lineage already owns; Settings does not own the transition itself.** Revoking a Membership, for instance, is `IdentityAndAccess.md`'s Lifecycle Model acting — Settings owns only the surface that initiates it, the same relationship Console holds to Organization switching (a structural fact about what Console contains, never the switching behavior itself).
- **Commercial content is outside this document, and outside Platform Architecture entirely.** What a plan costs or contains is explicitly excluded by `PlatformSpecification.md`'s own text; this document does not narrow or reinterpret that exclusion.

---

## Platform Vocabulary

One primary responsibility per architectural element, unchanged from the prior five documents, extended by one:

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
| Settings | manages |

"Manages," here, means presents and surfaces action — never owns. The same restraint every other verb in this table already carries.

---

## Account Settings

Account Settings presents a User's own Memberships, already defined by `IdentityAndAccess.md`, and surfaces the Role-gated actions those Memberships already permit — granting or revoking another Membership, for a User whose own Role is Owner. Account Settings does not define Role, does not introduce a new Role value, and does not perform authorization itself: whether an action is available follows entirely from `IdentityAndAccess.md`'s existing Role definition, checked elsewhere, reflected here.

Account Settings does not present or manage API Client standing — that remains Organization-scoped and is presented under Subscription & Billing below, consistent with `IdentityAndAccess.md`'s own placement of API Client as directly Organization-owned.

---

## Subscription & Billing

Subscription & Billing presents an Organization's Subscription — its active plan selection and invoice visibility, exactly as `PlatformSpecification.md` names them — and surfaces the action of changing that selection, which invokes `PlatformSpecification.md`'s own Subscription Lifecycle Model (Initiated → Active → (Past Due) → (Canceled)). This document does not own that lifecycle; it owns only the surface that initiates a transition within it.

"Invoice visibility" is an experience commitment: that invoices are visible to the User. It is not a claim that Invoice is a Platform Domain Model concept — no such concept is introduced here or by `PlatformSpecification.md`. The contents of an invoice are commercial and billing-system content, outside this document and outside Platform Architecture entirely, exactly as "what a plan costs or contains" already is.

Subscription & Billing also presents Organization-owned API Client standing (`IdentityAndAccess.md`), since API Client management is Organization-scoped exactly as Subscription is — grouped here rather than under Account Settings for that reason, not because API Client and Subscription share any Domain Model relationship beyond common Organization ownership.

---

## Architectural Decisions Deferred

All four items in `PlatformSpecification.md`'s Architectural Decisions Deferred table were already resolved before this document existed. This document introduces no new deferred decisions.

---

## Boundaries

This document does not define: subscription pricing, commercial policy, licensing, payment providers, billing implementation, invoice content or generation, entitlement logic, backend services, runtime behavior, persistence, credential or session mechanisms, or API design.

It does not redefine Organization, User, Membership, Project, Upload, Assessment, Notification, Subscription, or API Client — all remain exactly as `PlatformSpecification.md`, `IdentityAndAccess.md`, and `ProjectsAndUploads.md` already define them.

`PlatformSurfaces.md`'s own scope is not yet determined. This document does not anticipate, assume, or reserve anything on its behalf.

---

## Architectural Boundary

Settings is an interaction surface over the Platform Domain Model. It does not own Organization, User, Membership, Subscription, or API Client — it presents and surfaces management action over those concepts without redefining them or their lifecycles.

Any interaction beyond Settings' own surfaces — movement to reach it, notification of events affecting what it presents, or the underlying lifecycle transitions its actions invoke — is governed by the Platform specifications already responsible for those concerns, not by this one.

---

## Document Status

**Approved** — a governed specification of the Platform Architecture lineage. Ratified per `PlatformSpecification.md`'s Platform Architecture Governance section, which permits this document to refine and apply the concepts it delegates without expanding the Platform Domain Model.

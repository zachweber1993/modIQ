# Platform Specification

> **The authoritative specification defining the platform-level concepts a modIQ user, organization, and external consumer exist within — the identity, tenancy, and containment model surrounding the Assessment.**

| Property | Value |
|---|---|
| **Document** | PlatformSpecification.md |
| **Status** | Frozen — Platform Architecture 1.0 (2026-07-30), amended 2026-08-01 (`PlatformSurfaces.md` retired from the Governs list) |
| **Project** | modIQ |
| **Owner** | Zach Weber |
| **Created** | 2026-07-29 |
| **Last Updated** | 2026-07-29 |

---

## Specification Authority

**Authority** (referenced; none of the following are amended by this document):

- `Vision.md`
- `Principles.md`
- `Glossary.md`
- `ProductSpecification.md`

Authority runs outward from the frozen constitutional tier, never back into it. This document does not appear in any of the four documents above's own governed-specification lists.

**This document governs:**

- `docs/platform/IdentityAndAccess.md`
- `docs/platform/ProjectsAndUploads.md`
- `docs/platform/DashboardAndConsole.md`
- `docs/platform/WayfindingAndSearch.md`
- `docs/platform/Notifications.md`
- `docs/platform/SettingsAndBilling.md`

If a conflict exists between this document and a higher-level specification, the higher-level specification takes precedence.

---

## Purpose

This document establishes the platform-level conceptual domain of modIQ: the identities, containers, and relationships that give an Assessment an owner, a home, a history, and an audience.

It defines the Platform Domain Model, the terminology this lineage introduces, the boundary between this lineage and Runtime Architecture, and the rules every subordinate Platform Architecture document must satisfy.

It does not define UI behavior, persistence, APIs, authentication mechanisms, deployment technology, or pricing. It does not define anything at or beneath the Assessment.

---

## Scope

Platform Architecture owns Organization, User, Membership, Project, Upload (the record aspect), Notification, Subscription, API Client, and Public Visitor — defined completely in *The Platform Domain Model* — together with a platform experience layer (Console, Wayfinding, Search, Dashboard, Settings) governed by this specification but not modeled as part of the Domain Model.

Platform Architecture does not own the Rule Engine, Evidence Collection, or Assessment Engine internals; documentation governance itself (the Governance Register, ADR process, Documentation Authority order); or business-model decisions. `SettingsAndBilling.md` specifies the experience of managing a Subscription — plan selection, invoice visibility — never what a plan costs or contains.

---

## Architectural Boundary

Platform Architecture and Runtime Architecture meet at a single, fixed boundary: the Assessment.

Platform Architecture owns everything above that boundary — the identity, tenancy, and containment through which an Organization owns Projects, a Project contains Assessments, and an Upload triggers one. Runtime Architecture owns everything at or beneath it — Assessment Subject, Evidence, Finding, Recommendation, and Assessment Report — defined completely by `Glossary.md` and `DataModel.md`.

Platform Architecture references the Assessment to describe containment. It does not redefine it, model its internal structure, or name what lies beneath it beyond this boundary statement.

This division keeps both lineages independent. Runtime Architecture owns the evolution of everything beneath the boundary; Platform Architecture owns the evolution of everything above it. Ownership on one side carries no authority on the other.

---

## Architectural Responsibilities

```
Vision → Principles → Glossary → ProductSpecification    (frozen constitutional tier)
                    │
        ┌───────────┴───────────┐
        │                       │
  Product Design           Platform Architecture   (this document)
        │                       │
Interaction Design         Platform topic documents
        │                       │
        └──────────┬────────────┘
                    │
        Runtime Architecture (docs/architecture/)
                    │
        Engineering Alignment Program
```

| Lineage | Owns |
|---|---|
| **Platform Architecture** | Identity, tenancy, and containment above the Assessment: Organization, User, Membership, Project, Upload (record), Notification, Subscription, API Client, Public Visitor, and the platform experience layer. |
| **Product Design** | The conceptual model of the Assessment experience: Finding, Evidence, Recommendation, Assessment Subject (experientially), the Workspace. |
| **Interaction Design** | The behavioral specification of that experience: movement through Intake, Assessing, Reviewing; the Upload experience. |
| **Runtime Architecture** | Everything at or beneath the Assessment: Assessment Subject (structurally), Evidence, Finding, Recommendation, Assessment Report, the Rule Engine, Evidence Collection, Version Profiles. |
| **Engineering Alignment** | Owns no platform or runtime domain concept. Owns the process reconciling Runtime Architecture as implemented against the frozen Product Design and Interaction Design corpus. |

Platform Architecture and Interaction Design are siblings, not a hierarchy: Interaction Design is authoritative inside the Assessment experience; Platform Architecture is authoritative outside it.

---

## The Platform Domain Model

### Overview

The platform domain answers who is acting, what durable container their work lives in, how material entered that container, and who is told what happened.

### Concepts

**Organization.** The tenancy boundary of the platform. Owns Projects, holds one Subscription, is the scope a Membership grants standing within.

**User.** An individual human identity. Holds Memberships; originates Uploads; receives Notifications. Has no capability by itself — every capability is reached through a Membership.

**Membership.** The relationship connecting exactly one User to exactly one Organization, carrying that User's standing within it — the sole path by which a User gains capability inside an Organization. A Membership grants standing, not shared interaction: multiple Users holding Memberships into the same Organization each receive an independent view of its Projects. This does not create, and must never be implemented as, a collaborative Workspace session, which remains outside Product Design's approved scope.

**Project.** An Organization-owned container that groups the Assessments performed within it under a single organizing purpose, accumulated over time. Belongs to exactly one Organization. Contains the Assessments performed within it and the Upload history that triggered them. What each Assessment evaluates internally is Runtime Architecture's exclusive concern.

**Upload** (record aspect). The platform-level record of material submitted to a Project. Belongs to exactly one Project. Triggers one Assessment. Originates from exactly one User or API Client. Immutable once submitted.

**Assessment** [the Architectural Boundary]. Referenced exactly twice — a Project contains Assessments; an Upload triggers one — never structurally.

**Notification.** A system-generated record informing a User that a platform event occurred. Generated in response to an event on any other concept. Addressed to a User.

**Subscription.** The record of an Organization's commercial standing and the capability level it grants. Belongs to exactly one Organization.

**API Client.** A non-human actor, owned by an Organization, capable of the same platform-facing actions a User can take — principally, originating Uploads.

**Public Visitor.** The absence of identity — the platform's unauthenticated actor. Owns nothing; is owned by nothing. Transitions into a User upon registration.

The platform experience layer — Console, Wayfinding, Search, Dashboard, Settings — has no independent ownership or lifecycle and is therefore not part of this Domain Model. See *Terminology*.

### Supporting Views of the Domain Model

The Relationship Model, Ownership Model, and Lifecycle Model below are not separate architectures. Each is the same Platform Domain Model viewed through a different lens — containment, ownership, and existence, respectively.

#### Relationship Model

```text
Public Visitor
   │  registers, becomes
   ▼
User ──────────────┐
   │  holds many    │  originates
   ▼                ▼
Membership      Upload ──── triggers ──▶ Assessment [the boundary]
   │                │
   │  grants standing│  belongs to
   │  within          ▼
   └──────────▶ Organization
                    │  owns
                    ├── many Projects ──── contains ──▶ Assessment
                    ├── one Subscription
                    └── many API Clients ── originates ──▶ Upload (symmetric with User)

Notification ── addressed to ──▶ User
             ── generated by ──▶ event on any concept above
```

#### Ownership Model

| Concept | Organization-owned | User-owned | Shared | System-owned | External |
|---|---|---|---|---|---|
| Organization | — (root) | | | | |
| User | | ✓ | | | |
| Membership | | | ✓ | | |
| Project | ✓ | | | | |
| Upload | | | | ✓ | |
| Assessment | *Inherited — `DataModel.md`* | | | | |
| Notification | | ✓ | | | |
| Subscription | ✓ | | | | |
| API Client | ✓ | | | | |
| Public Visitor | | | | | ✓ |

#### Lifecycle Model

- **Organization** — Created → Active → (Suspended) → (Closed).
- **User** — Registered → Active → (Deactivated).
- **Membership** — Granted → Active → (Revoked).
- **Project** — Created → Active → (Archived). Persists across the full history of Assessments performed within it.
- **Upload** — Submitted → Processed → Retained, permanently and immutably.
- **Assessment** — *Inherited entirely from `DataModel.md`'s Runtime Lifecycle.*
- **Notification** — Generated → Delivered → Read/Dismissed → (Retained or Expired).
- **Subscription** — Initiated → Active → (Past Due) → (Canceled).
- **API Client** — Created → Active → (Revoked).
- **Public Visitor** — No persistent lifecycle; exists for one anonymous interaction, then departs or transitions into a User.

---

## Terminology

**Terminology qualified for Glossary proposal.**

*Upload.* One term, two aspects — the Upload Experience (`ASSESSMENT_INTAKE_AND_UPLOAD.md`'s exclusive territory: selection, feedback, error recovery) and the Upload Record (this lineage's exclusive territory: Project membership, immutability, the Assessment it triggered). Neither redefines the other; one Glossary entry covers both. `Glossary.md`'s own rule requires new terminology to be added before use elsewhere; Upload is used within this specification's own Domain Model and therefore qualifies.

**Terminology reserved, pending use.**

*Console.* The platform-level persistent shell containing the already-defined per-Assessment Workspace as one region within it. Coined to avoid reusing Workspace, which keeps its existing meaning exactly as `WORKSPACE_EVOLUTION.md` and `NAVIGATION_AND_WORKSPACE_BEHAVIOR.md` define it. Held as internal Platform Architecture terminology; becomes eligible for Glossary proposal once a governed document exercises it.

*Wayfinding.* Movement across Organizations, Projects, and Assessments. Coined to avoid reusing Navigation, which keeps its existing meaning exactly as `NAVIGATION_AND_WORKSPACE_BEHAVIOR.md` defines it — movement within a single Assessment. Reserved under the same rule as Console.

**Existing words, reused without new definition.**

*User.* The capitalized platform identity formalizes, and coexists with, the lowercase "user" already used throughout Product Design and Interaction Design to mean the same person. Neither usage redefines the other.

*API.* This lineage's "API Client" is unrelated to `EngineAPI.md`'s "API," which names internal subsystem interfaces. The words overlap; the territories do not.

---

## The Platform Architecture Filter

Every document this specification governs, and this specification itself, satisfies all five:

**Architecture-first.** Concepts and relationships are established before any topic document proposes behavior built on them.

**Platform-first.** Every concept generalizes across Organizations, Projects, and Assessment Subject types without redesign.

**Technology-agnostic.** No database, table, identifier scheme, API contract, or authentication mechanism appears in this lineage's architecture documents.

**Deterministic where applicable.** Where a platform concept has traceability behavior — Upload provenance, chiefly — it is held to the same reproducibility standard `Principles.md` sets for the runtime domain.

**Documentation-first.** No topic document is drafted before the concepts it depends on exist here, ratified.

---

## Architectural Decisions Deferred

Each item is ratified in the topic document named, not adopted here. All four have since been ratified.

| Decision | Status | Resolved by |
|---|---|---|
| Does every Project require an Organization, including a solo User's? | Resolved — yes, one ownership rule, not two. | `IdentityAndAccess.md` |
| Is a Project mandatory, or auto-created implicitly? | Resolved — auto-created, provisioned automatically when none is designated. | `ProjectsAndUploads.md` |
| Can a Notification target an entire Organization, or only a User? | Resolved — User-only, confirmed. | `Notifications.md` |
| Does an API Client's grant carry Role/Project scope, like a Membership, or is it always Organization-wide? | Resolved — Organization-wide. | `IdentityAndAccess.md` |

---

## Platform Architecture Governance

PlatformSpecification.md is the sole constitutional authority for first-class concepts within the Platform Architecture lineage. No other document — governed or otherwise — may define one.

A governed document — `IdentityAndAccess.md`, `ProjectsAndUploads.md`, and the rest — may refine, constrain, explain, or apply a concept this specification defines. It may not independently introduce an additional first-class concept, regardless of how narrowly scoped or self-evident that concept may seem.

A new first-class Platform Architecture concept comes into existence only through an approved amendment to this specification. It may not appear in any governed document — not even provisionally — before that amendment is approved.

This rule preserves the integrity of the Platform Domain Model as the lineage grows. Without it, the Domain Model fragments across topic documents instead of remaining the single conceptual foundation this specification establishes.

---

## Architectural Dependencies

This section describes the current state of adjacent work. Unlike the rest of this document, it is expected to become outdated and should be reviewed, not treated as binding, once any of the following changes.

- Notification behavior tied to in-progress Assessment activity depends on the engine exposing an intermediate observation point during execution. As of this writing, that capability is one of the divergences `ENGINEERING_ALIGNMENT_PROGRAM.md` §1 has not resolved. Topic documents should not design around it until it is confirmed.
- Platform Architecture is not one of the bodies of work the current Engineering Alignment Program reconciles. A future reconciliation program, once this lineage reaches comparable maturity to Product Design and Interaction Design, is anticipated but not authorized or scheduled by this document.
- The Architectural Boundary has one structural consequence worth naming here: because this lineage's model terminates at Assessment, it has no surface area that could take an implicit position on Engineering Alignment's open "terminal completion state" question. That question remains entirely Runtime Architecture's and Engineering Alignment's to resolve.

---

## Closing Constitutional Statement

Platform Architecture exists so that no Assessment is ever orphaned. Every Assessment belongs to someone, lives inside something durable, arrived through a record that can be traced, and is seen only by those entitled to see it.

Product Design and Interaction Design define what an Assessment means and how it is experienced. Runtime Architecture defines how it is computed. Platform Architecture defines none of these — only the identity, ownership, and containment that make an Assessment belong to a real Organization and a real User in the first place. The other lineages ask what modIQ understands. Platform Architecture asks only whose understanding it is.

---

## Related Documents

- `docs/constitutional/Vision.md`, `Principles.md`, `Glossary.md`, `ProductSpecification.md` — constitutional authority.
- `docs/product-design/README.md`, `docs/interaction-design/README.md` — the Assessment experience this lineage contains.
- `docs/architecture/DataModel.md` — the Architectural Boundary's authoritative source.
- `docs/DOCUMENTATION_MAP.md` — how this lineage fits the repository as a whole.

---

## Document Status

**Frozen** — Platform Architecture 1.0, effective 2026-07-30. The status values applicable to this document across its lifecycle are: **Draft** (not yet ratified), **Approved** (ratified, not yet frozen), **Frozen** (permanent; changes only through a governed amendment), **Superseded** (replaced by a later document).

Frozen means this specification, and the five governed specifications built on it (`IdentityAndAccess.md`, `ProjectsAndUploads.md`, `DashboardAndConsole.md`, `WayfindingAndSearch.md`, `Notifications.md`), are now permanent. Any future change to any of the six is a governed amendment to Platform Architecture, not ordinary editing — it must state its own rationale and impact, following the same pattern already used for the amendments recorded below. New architectural work (`SettingsAndBilling.md` and beyond) inherits this baseline; it does not revisit it.

**Amended 2026-07-29:** the Architectural Decisions Deferred table was updated to record that three of its four items have since been ratified in `IdentityAndAccess.md` and `ProjectsAndUploads.md`. This is a documentation synchronization, not a new architectural decision — no concept, boundary, or governance rule changed; the table previously showed each item's original working assumption and now shows its resolution instead.

**Amended 2026-07-30:** the Architectural Decisions Deferred table was updated to record that the fourth and final item — Notification addressing scope — has been ratified in `Notifications.md` (User-only, confirmed). All four items are now resolved. This is a documentation synchronization, not a new architectural decision.

**Amended 2026-08-01:** `PlatformSurfaces.md` is retired from the Governs list. A dedicated Scope Determination and Historical Determination found, from repository evidence alone, that it was never assigned an architectural responsibility — it appeared in this document's original commit with no accompanying scope, responsibility, or explanatory text, and received none in any commit since; the platform experience layer's own enumeration (Console, Wayfinding, Search, Dashboard, Settings), the only textually plausible reading of its name, has never changed and is now completely governed by `DashboardAndConsole.md`, `WayfindingAndSearch.md`, and `SettingsAndBilling.md`. No architectural responsibility, documented scope, or architectural gap corresponds to it. This retirement is not based on any assumption about original authorial intent. It removes an unexplained placeholder; it does not remove, narrow, or reassign any existing responsibility, and introduces no new concept. A separate, independent observation from the same Scope Determination — that Public Visitor remains the only Platform Domain Model concept without its own refinement document — is explicitly not part of this amendment's rationale and authorizes no action here.

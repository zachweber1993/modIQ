# Documentation Map

| Property | Value |
|----------|-------|
| **Document** | DOCUMENTATION_MAP.md |
| **Project** | modIQ |
| **Purpose** | Architectural overview of the repository's documentation — orientation, not content. |
| **Status** | Living. Reflects repository structure, not any single specification's content. |
| **Last Updated** | 2026-07-30 (Platform Architecture 1.0 frozen) |

---

## Purpose

This document answers one question: *given a topic, which part of the repository's documentation should I open first?* It does not restate what any document says — `docs/README.md`, `docs/00-Governance.md`, and each group's own index already do that. It exists one level above them, mapping how the groups relate to each other and to the two lineages the repository actually contains.

---

## The Three Lineages

The repository's documentation is not one linear sequence. It is three related but independently-paced lineages, sharing a common constitutional foundation:

**The Engineering Specification Lineage** — Constitutional → Architecture → Implementation/Engineering. Frozen, versioned by Documentation Release, tracked Sprint by Sprint in `docs/governance/PROJECT_STATUS.md`, governed by `docs/engineering/GOVERNANCE.md`'s Governance Register and the ADR process. This is the lineage `docs/README.md`'s Reading Order describes.

**The Product Track** — Product Definition → Product Design → Interaction Design. Defines the Version 1 assessment *experience*, ahead of and independent from implementation. Carries no Documentation Authority ranking and is not governed by the Governance Register — its own convention is a lightweight closeout record at the end of each phase.

**The Platform Architecture Lineage** — a constitutional document, `docs/platform/PlatformSpecification.md`, and six governed topic documents built on it: `IdentityAndAccess.md`, `ProjectsAndUploads.md`, `DashboardAndConsole.md`, `WayfindingAndSearch.md`, `Notifications.md`, and `SettingsAndBilling.md`. **The specification and the first five topic documents are Frozen as Platform Architecture 1.0, effective 2026-07-30** — permanent; any future change to any of those six is a governed amendment, not ordinary editing. `SettingsAndBilling.md`, drafted after the freeze, is Approved and unaffected by it. Defines the platform-level identity, tenancy, and containment model surrounding the Assessment — Organization, User, Project, and the rest — distinct from the Assessment experience the Product Track defines and from the Runtime Architecture the Engineering Specification lineage defines. Carries no Documentation Authority ranking and is not governed by the Governance Register; its own governance is internal to `PlatformSpecification.md`, which is the sole authority for introducing new first-class concepts within it.

**As of this writing, none of the three lineages are reconciled with one another**: no Sprint has been scoped against any Product Design or Interaction Design artifact, and the Engineering Alignment Program governing that reconciliation does not yet include the Platform Architecture lineage as one of the bodies of work it reconciles. `docs/governance/PROJECT_STATUS.md`'s "Product & Interaction Design Track" and "Platform Architecture Track" sections state this explicitly. Future work bringing all three into alignment is anticipated but not yet authorized or scheduled.

---

## Documentation Groups

| Group | Location | Purpose | Entry Point |
|---|---|---|---|
| Constitutional | `docs/constitutional/` | Frozen product philosophy and vocabulary: Vision, Principles, Glossary, Product Specification. | `docs/README.md` Reading Order |
| Architecture | `docs/architecture/` | Frozen technical specifications: platform architecture, data model, rule engine, evidence collection, engine API, version profiles. | `docs/README.md` Reading Order |
| Implementation | `docs/implementation/` | Sprint-level implementation plans, crate roadmap, dependency map, runtime invariants. | `docs/README.md` Reading Order (partial) |
| Engineering | `docs/engineering/` | The working engineering record: capability proposals, architecture evaluations, sprint reports, engineering releases, handoff documents, and the product-track closeout records. The largest and least indexed group in the repository — see Known Navigation Gaps below. | `docs/governance/PROJECT_STATUS.md` (narrates and links the relevant documents chronologically) |
| Governance (process) | `docs/governance/` | Project tracking and process documents: current status, changelog, roadmap, engineering workflow guide, code review checklist, and the Repository Synchronization Policy governing how approved artifacts enter the repository. Distinct from architectural governance — see the naming note below. | `docs/governance/README.md` |
| Architectural Governance | `docs/engineering/GOVERNANCE.md` | The Governance Register, Crate Boundary Rules, Documentation Authority order, and change-category policy. | `docs/engineering/GOVERNANCE.md` itself |
| ADRs | `docs/adrs/` | Historical Architecture Decision Records — why, not what. | `docs/adrs/README.md` |
| Releases | `docs/releases/` and `docs/engineering/ENGINEERING_RELEASE_*.md` | Per-Sprint/Documentation-Release engineering records. Split across two locations — see Known Navigation Gaps below. | `docs/governance/PROJECT_STATUS.md` |
| Product Design | `docs/product-design/` | The Version 1 conceptual product model: Workspace Evolution, The Finding, The Assessment Experience, Evidence, The Assessment Report. | `docs/product-design/README.md` |
| Interaction Design | `docs/interaction-design/` | Version 1 behavioral specification, built on the Product Design model. | `docs/interaction-design/README.md` |
| Platform Architecture | `docs/platform/` | The platform-level identity, tenancy, and containment model surrounding the Assessment. **Frozen as Platform Architecture 1.0** (constitutional foundation plus five governed topic documents: `IdentityAndAccess.md`, `ProjectsAndUploads.md`, `DashboardAndConsole.md`, `WayfindingAndSearch.md`, `Notifications.md`); all four Architectural Decisions Deferred resolved. `SettingsAndBilling.md`, drafted after the freeze, is Approved and unaffected by it. One topic document (`PlatformSurfaces.md`) remains undrafted; its own architectural scope has not yet been determined. | `docs/platform/PlatformSpecification.md` |

---

## A Naming Note Worth Knowing Up Front

"Governance" refers to two different things in this repository, and the split is easy to trip over:

- `docs/governance/` (the folder) holds **project tracking and process** documents — status, changelog, roadmap, workflow guide.
- `docs/engineering/GOVERNANCE.md` (a single file, in a different folder) holds **architectural governance** — the Governance Register, Crate Boundary Rules, and Documentation Authority order.

Neither is misnamed; they simply answer different questions ("where do things stand" versus "what may change and how"). This map exists partly so a new contributor learns this in one place rather than by trial and error.

---

## Quick Orientation

| If you want to understand... | Start here |
|---|---|
| What modIQ is and why it exists | `docs/constitutional/Vision.md` → `docs/constitutional/ProductSpecification.md` |
| The authoritative vocabulary | `docs/constitutional/Glossary.md` |
| The current engineering state (what Sprint, what's built) | `docs/governance/PROJECT_STATUS.md` |
| What may change and how (architectural governance) | `docs/engineering/GOVERNANCE.md` |
| Why a specific architectural decision was made | `docs/adrs/README.md` |
| The Version 1 product experience (concepts) | `docs/product-design/README.md` |
| The Version 1 product experience (behavior) | `docs/interaction-design/README.md` |
| Whether the product/engineering lineages are reconciled yet | `docs/governance/PROJECT_STATUS.md`, "Product & Interaction Design Track" |
| How an approved artifact gets into the repository | `docs/governance/RepositorySynchronizationPolicy.md` |
| The platform-level identity, tenancy, and containment model surrounding the Assessment | `docs/platform/PlatformSpecification.md` |

---

## Known Navigation Gaps (Named, Not Resolved Here)

Identified during the 2026-07-28 documentation architecture review. None of these represent inconsistent or incorrect documentation — they are discoverability gaps, left for separately-scoped future work rather than addressed in this pass:

- **`docs/engineering/` has no index.** It holds roughly 85 documents (proposals, architecture evaluations, sprint reports, releases, handoffs) with no README grouping them by kind or Sprint. `docs/governance/PROJECT_STATUS.md`'s own Sprint-by-Sprint narrative is the practical way to navigate it today. Building a real index is a substantive undertaking, not a minor edit, and is recommended as its own future initiative.
- **Engineering Release records are split across two locations.** `docs/releases/ENGINEERING_RELEASE_0.2.md` is the only release record in `docs/releases/`; every release from 0.3 onward lives in `docs/engineering/` instead. This is historical, not a defect to silently correct — moving `0.2` now would rewrite established file history for a cosmetic gain.
- **`docs/governance/ROADMAP.md` is significantly stale** (last updated 2026-07-16; still describes Sprint 1 as "In Progress" against a repository now at Sprint 21, per `PROJECT_STATUS.md`). This predates and is unrelated to the Product/Interaction Design work this map was written alongside. Reconciling it is a separate, larger effort outside this review's conservative scope.
- **`docs/platform/` has no `README.md` of its own yet**, unlike `docs/product-design/` and `docs/interaction-design/`. `PlatformSpecification.md` (constitutional), `IdentityAndAccess.md`, `ProjectsAndUploads.md`, `DashboardAndConsole.md`, `WayfindingAndSearch.md`, and `Notifications.md` (the five approved governed topic documents) are all synchronized; two governed topic documents (`SettingsAndBilling.md`, `PlatformSurfaces.md`) remain undrafted. Not a defect — named here so a reader isn't surprised by the directory's current thinness.

---

## Relationship to Other Top-Level Documents

This map supplements, and does not replace:

- **`docs/README.md`** — the authoritative Reading Order for the frozen Engineering Specification lineage.
- **`docs/00-Governance.md`** — a short constitutional-through-architecture reading-order pointer.

Where those documents describe *what to read and in what order* within their own lineage, this map describes *how the whole repository's documentation is organized* across both lineages.

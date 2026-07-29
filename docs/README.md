# modIQ Design Specification

Welcome to the official design specification for modIQ.

> For an architectural overview of how this Reading Order relates to the rest of the repository's documentation (engineering, governance, ADRs, the Product/Interaction Design track, and the Platform Architecture lineage), see [`DOCUMENTATION_MAP.md`](DOCUMENTATION_MAP.md).

## Reading Order

1. Vision.md
2. Principles.md
3. Glossary.md
4. ProductSpecification.md
5. Architecture.md
6. DataModel.md
7. KnowledgeModel.md
8. VersionProfile.md
9. RuleEngine.md
10. EvidenceCollection.md
11. EngineAPI.md
12. Sprint0.md
13. Sprint1.md

## Purpose

These documents define the product before implementation begins.

All contributors should read them in order before making architectural or implementation decisions.

Approved RFCs supplement these documents.

## Specification Hierarchy

The documentation is hierarchical.

Each specification derives its authority from the documents that precede it in the Reading Order.

Higher-level specifications define architectural intent.

Lower-level specifications implement that intent without redefining it.

If a conflict exists between specifications, the higher-level specification takes precedence.

## Current Documentation Status

The constitutional specifications, platform architecture, technical specifications, and implementation planning documents have completed the Documentation Release 1.0 Final Review and are considered **Frozen**.

The review verified:

- Architectural consistency
- Cross-specification reconciliation
- Terminology consistency
- Documentation quality
- Implementation readiness

The review concluded with:

- Documentation Release 1.0 frozen
- Git tag `v0.1.0`
- Transition from documentation to implementation

Documentation Release 2.0 (governance and terminology reconciliation) and Documentation Release 2.1 (Evidence Collection subsystem boundary — new specification `EvidenceCollection.md`) have since been completed; Documentation Release 2.1 remains current — none of Sprints 8 through 13 required a Documentation Release amendment. The current Engineering Release is **1.3** (Sprint 13: Storage Architectural Activation — `modiq-storage` gave the Storage subsystem its first real content, following an evidence-acquisition investigation (INV-002), a governance reconciliation restoring Sprint 8's own "Architectural Activation" precedent alongside Sprint 12's Capability Identity procedure, Architecture Evaluation, Architectural Resolution, Implementation Authorization, and a three-phase Sprint Plan wiring Storage through `modiq-cli` and `apps/sandbox`), produced at its own Repository Closeout. Engineering Releases 0.6 (Sprint 6: CLI wiring, `modiq-report` scaffold retirement) and 0.7 (Sprint 7: Multi-Source Evidence Collection) were both produced retroactively, after a two-Sprint documentation gap (`docs/engineering/HISTORICAL_RELEASE_COMPLETION_REPORT.md`); every Sprint since Sprint 8 has produced its own release at Sprint close, not repeating that gap. See `docs/engineering/ENGINEERING_RELEASE_1.3.md` and `docs/governance/PROJECT_STATUS.md` for current status. The engineering methodology itself is designated Version 1.0, unchanged since Sprint 7 — see `docs/engineering/PROJECT_HANDOFF_v1.0.md`, Section 5.

## Product & Interaction Design Track

Separately from the engineering specification and Documentation Release lineage above, a product-track body of work defines the Version 1 assessment *experience*, following `docs/constitutional/ProductSpecification.md` but preceding any UI or implementation decision. This track is not part of the Reading Order above and carries no Documentation Authority ranking — it is product-track continuity material, not an engineering, architecture, or governance specification.

- `docs/engineering/PRODUCT_DEFINITION_CLOSEOUT.md` — closes Product Definition, opens Product Design.
- `docs/product-design/` — the five approved Version 1 Product Design artifacts (Workspace Evolution, The Finding, The Assessment Experience, Evidence, The Assessment Report).
- `docs/engineering/PRODUCT_DESIGN_CLOSEOUT.md` — closes Product Design, reflects on the body of work as a whole, opens Interaction Design.
- `docs/interaction-design/` — Interaction Design artifacts specifying user-facing behavior around the Product Design model. Currently: Assessment Intake & Upload (approved).

## Platform Architecture Track

Separately again from the engineering specification lineage above and the Product & Interaction Design Track above, a third body of work defines the platform-level identity, tenancy, and containment model surrounding the Assessment. This track is not part of the Reading Order above and carries no Documentation Authority ranking.

- `docs/platform/PlatformSpecification.md` — the constitutional foundation of the Platform Architecture lineage. Approved.
- `docs/platform/IdentityAndAccess.md` — the first approved governed topic document. Refines Organization, User, Membership, and API Client; defines the Role attribute of Membership (Owner, Member); resolves the Project ownership root and API Client access scope items from `PlatformSpecification.md`'s Architectural Decisions Deferred. Approved.
- `docs/platform/ProjectsAndUploads.md` — the second approved governed topic document. Refines Project and Upload; resolves the Project mandatory-or-auto-created item from `PlatformSpecification.md`'s Architectural Decisions Deferred (auto-created). Approved.
- Remaining governed topic documents (`DashboardAndConsole.md`, `WayfindingAndSearch.md`, `Notifications.md`, `SettingsAndBilling.md`, `PlatformSurfaces.md`) have not yet been drafted.
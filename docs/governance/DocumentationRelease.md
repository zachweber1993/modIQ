# Documentation Release

| Property | Value |
|----------|-------|
| **Document** | DocumentationRelease.md |
| **Project** | modIQ |
| **Purpose** | Documentation Release Governance |
| **Maintained By** | Project Maintainers |
| **Last Updated** | 2026-07-17 |

---

# Purpose

This document defines the governance process for Documentation Releases within the modIQ project.

A Documentation Release represents a stable, internally consistent engineering specification that is considered suitable for implementation.

Documentation Releases establish formal milestones in the evolution of the platform and provide controlled transition points between architecture, implementation, and future specification revisions.

---

# Documentation Philosophy

Documentation is the authoritative source of truth for the modIQ platform.

Implementation should derive from documentation rather than documentation being written after implementation.

Architectural intent should be established, reviewed, reconciled, and frozen before production code is introduced.

---

# Documentation Hierarchy

Documentation is organized into four layers.

## Constitutional Layer

Defines why the platform exists.

Documents include:

- Vision.md
- Principles.md
- Glossary.md
- ProductSpecification.md

---

## Platform Layer

Defines how the platform is organized.

Documents include:

- Architecture.md

---

## Technical Layer

Defines the conceptual implementation model.

Documents include:

- DataModel.md
- KnowledgeModel.md
- RuleEngine.md
- EngineAPI.md

---

## Execution Layer

Defines implementation planning.

Documents include:

- Sprint0.md
- Sprint1.md

---

# Documentation Release Lifecycle

Every Documentation Release follows the same governance process.

## Phase 1 — Draft

Specifications are authored and refined.

Terminology, architectural direction, and document relationships may evolve.

---

## Phase 2 — Foundation Review

The constitutional specifications are reviewed for:

- Vision alignment
- Terminology consistency
- Product scope
- Architectural intent

Successful completion results in a Foundation Freeze.

---

## Phase 3 — Technical Review

Technical specifications are reviewed for:

- Cross-specification consistency
- Architectural boundaries
- Runtime and knowledge separation
- Explainability
- Determinism
- Platform independence

Successful completion results in a Technical Freeze.

---

## Phase 4 — Repository Audit

The repository is reviewed as a complete engineering specification.

The audit verifies:

- Governance documents
- Repository organization
- Specification hierarchy
- Documentation quality
- Cross-specification reconciliation
- Implementation readiness

---

## Phase 5 — Documentation Freeze

Following a successful audit:

- Specifications are designated Frozen Drafts.
- Documentation Release is considered complete.
- Architectural changes require deliberate review.

---

## Phase 6 — Release Tag

The completed Documentation Release is tagged within source control.

Documentation Release 1.0 was tagged:

```
v0.1.0
```

---

## Phase 7 — Implementation Authorization

Production implementation may begin only after the Documentation Release has been frozen.

Implementation should reconcile all engineering decisions against the frozen documentation.

---

# Frozen Specifications

Frozen specifications remain authoritative.

Changes should occur only when:

- A critical architectural defect is identified.
- A documented contradiction exists.
- An accepted Architecture Decision Record requires modification.
- A future Documentation Release supersedes the specification.

Routine implementation work should not modify frozen specifications.

---

# Architecture Decision Records

Significant architectural changes should be documented through Architecture Decision Records (ADRs).

ADRs preserve the reasoning behind architectural evolution without rewriting historical specifications.

---

# Release Deliverables

A Documentation Release should include:

- Repository governance
- Constitutional specifications
- Platform architecture
- Technical architecture
- Implementation planning
- Cross-specification review
- Repository audit
- Documentation freeze
- Release tag

---

# Definition of Done

A Documentation Release is considered complete when:

- All planned specifications have been completed.
- Cross-specification consistency has been verified.
- Terminology is reconciled.
- Repository governance is current.
- Architectural boundaries are preserved.
- Documentation has successfully passed the Repository Audit.
- The repository is considered implementation-ready.

---

# Relationship to Implementation

Documentation Releases define architectural intent.

Implementation realizes that intent.

Implementation should remain consistent with the authoritative specifications unless superseded by a future Documentation Release.

---

# Release History

## Documentation Release 1.0

**Status:** Complete

Completed the constitutional, platform, technical, and implementation planning specifications. Passed the Repository Audit and Documentation Freeze (Phase 5).

Tagged: `v0.1.0`

---

## Documentation Release 2.0

**Status:** Complete

A governance and terminology reconciliation release. Reconciled the "modIQ Assessment Framework (MAF)" terminology, Version Profile ownership wording, and the constitutional Assessment definition. Corrected governance tracking documents (PROJECT_STATUS.md, CrateRoadmap.md, DependencyMap.md, ROADMAP.md, CHANGELOG.md, EngineeringGuide.md) to reflect actual repository state, including removal of stale references to Documentation Release 1.0 planning language.

Not yet tagged in source control as of this record.

---

## Documentation Release 2.1

**Status:** Complete

Establishes the architectural and governance foundation for Evidence Collection, following Technical Director approval of `PROPOSAL_EVIDENCE_COLLECTION_BOUNDARY.md`'s architectural direction. This release is architecture/documentation only — no production code changed, and no implementation is authorized by it.

Scope:

- ADR-0008 (Evidence Collection Subsystem Boundary) and ADR-0009 (AssessmentService Public API Evolution), both new.
- `docs/architecture/EvidenceCollection.md`, a new Technical Layer specification defining the Evidence Collection subsystem boundary, its relationship to the four existing subsystems, and the Collector Contract.
- Amendments to `Architecture.md` (Evidence Collection named as a Core Platform Component across the System Overview, Assessment Lifecycle, Dependency Rules, and Extensibility sections; Assessment Lifecycle diagram corrected to place Evidence Collection before Rule Engine, resolving a documented inconsistency with `DataModel.md` identified by the proposal) and cross-reference additions to `DataModel.md` and `RuleEngine.md`.
- `docs/README.md`: Reading Order updated to include `EvidenceCollection.md`.
- `GOVERNANCE.md`: a new Evidence Collection Crate Boundary Rules entry, and four new Governance Register items (GOV-007 through GOV-010), all left open pending implementation review.
- `CrateRoadmap.md`: `modiq-collection` recorded as a planned (not yet created) crate.

Phase 1 (Draft) through Phase 6 (Release Tag) of the Documentation Release Lifecycle above are complete for this release's actual scope — a Technical Director review served as Foundation Review, Technical Review, and Repository Audit together, given the release's narrow, single-subsystem scope. `EvidenceCollection.md` and the amended sections of `Architecture.md`/`DataModel.md`/`RuleEngine.md` are now marked Frozen. Not yet tagged in source control as of this record, consistent with Documentation Release 2.0's own precedent.

Per Phase 7 (Implementation Authorization): this release establishes architectural intent only. Evidence Collection implementation remains gated on GOV-007 and is not authorized by this Documentation Release alone.

---

# Future Evolution

Future Documentation Releases should build upon previous releases through refinement rather than unnecessary replacement.

Stable concepts should evolve deliberately while preserving architectural continuity whenever practical.
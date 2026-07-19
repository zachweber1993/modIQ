# CHANGELOG

| Property | Value |
|----------|-------|
| **Document** | CHANGELOG.md |
| **Project** | modIQ |
| **Purpose** | Repository History |
| **Maintained By** | Project Maintainers |
| **Last Updated** | 2026-07-19 |

---

All notable changes to the modIQ project will be documented in this file.

The format follows the principles of Keep a Changelog and Semantic Versioning where applicable.

---

# [Documentation Release 1.0]

## Added

### Repository Foundation

- Established the initial repository structure.
- Established documentation standards and governance.
- Defined repository organization and specification hierarchy.

---

### Constitutional Layer

#### Vision

- Defined the long-term vision and purpose of the modIQ platform.
- Established the platform's educational, evidence-based philosophy.
- Defined long-term product direction and design directives.

#### Principles

- Defined the engineering principles governing all architectural decisions.
- Established deterministic, explainable, and platform-first design philosophy.

#### Glossary

- Established the authoritative terminology for the project.
- Standardized platform vocabulary across all specifications.

#### Product Specification

- Defined the conceptual product requirements.
- Established product scope, capabilities, objectives, and non-goals.

---

### Platform Layer

#### Architecture

- Defined the conceptual architecture of the modIQ platform.
- Established subsystem responsibilities and architectural boundaries.
- Formalized the separation between runtime and knowledge domains.

---

### Technical Layer

#### Data Model

- Defined the conceptual runtime domain model.
- Established Assessment as the aggregate root of the runtime domain.
- Defined runtime entities, ownership, relationships, and lifecycle.

#### Knowledge Model

- Defined the conceptual engineering knowledge domain.
- Established the modIQ Knowledge Base (MKB).
- Defined reusable engineering knowledge independent of Assessment execution.

#### Rule Engine

- Defined the conceptual execution model for deterministic assessment.
- Established evidence-based rule evaluation.
- Formalized traceability and explainability throughout rule execution.

#### Engine API

- Defined the conceptual service contract of the assessment engine.
- Established capability-oriented services independent of implementation technology.
- Defined Assessment, Knowledge, Rule Evaluation, Reporting, and Version Profile services.

---

### Execution Layer

#### Sprint 0

- Defined the implementation readiness sprint.
- Established engineering objectives, deliverables, dependencies, and success criteria required before production development.

#### Sprint 1

- Defined the first production implementation sprint.
- Established the initial end-to-end Assessment implementation plan.
- Defined implementation priorities centered on determinism, explainability, and architectural integrity.

---

## Milestones

- Completed the constitutional architecture.
- Completed the platform architecture.
- Completed the technical architecture.
- Completed the implementation planning layer.
- Completed the Documentation Release 1.0 specification set.
- Completed the Documentation Release 1.0 Final Review and Documentation Freeze.

---

## Final Review Outcomes

The Documentation Release 1.0 Final Review concluded with:

- Repository audit passed
- Cross-specification reconciliation completed
- Documentation Freeze complete
- Git tag `v0.1.0`
- Authorization to begin implementation

---

# [Sprint 0]

## Added

### Milestone 1

- Established Cargo workspace
- Configured Rust toolchain
- Implemented runtime domain skeleton
- Implemented Assessment aggregate
- Structured runtime entities according to DataModel.md

### Runtime Domain

- Established `modiq-runtime` crate
- Implemented Assessment aggregate skeleton
- Implemented runtime domain module hierarchy
- Aligned runtime model with DataModel.md
- Established aggregate ownership

### Common Platform

- Established `modiq-common` crate
- Added shared platform documentation
- Defined architectural ownership rules

### Knowledge Domain

- Established `modiq-knowledge` crate
- Implemented knowledge domain module hierarchy
- Implemented reusable engineering knowledge entities
- Aligned implementation with KnowledgeModel.md
- Preserved runtime / knowledge separation

---

# [Sprint 1]

**Status:** Complete

## Added

- Implemented the Assessment aggregate in `modiq-runtime`: creation, unique identity, and initialization of empty Evidence, Finding, and Recommendation collections.
- Implemented and enforced Assessment lifecycle transitions (`begin_evidence_collection`, `begin_rule_evaluation`, `complete`).
- Added `AssessmentError` for invalid lifecycle transitions.
- Added RuntimeInvariants.md INV-010, INV-011, and INV-012 governing lifecycle sequencing.
- Implemented Evidence collection (`Assessment::add_evidence`), gated to the evidence-collection lifecycle phase.
- Implemented the first deterministic Rule Engine (`modiq-rules::RuleEngine`), evaluating Evidence into a Finding and Recommendation.
- Implemented immutable Assessment Report snapshot generation (`modiq-report::AssessmentReport`).
- Implemented orchestration of the complete pipeline (`modiq-engine::AssessmentService::execute`), composing the Runtime Domain, Rule Engine, and Reporting into one executable operation, demonstrated end to end by integration tests.

## Released

- Repository tagged `v0.1.0-alpha`.
- Recorded in `ENGINEERING_RELEASE_v0.1.0-alpha.md` and `HANDOFF_SPRINT1.md`.

---

# [Sprint 2]

**Status:** Complete

## Added

- Gave `Evidence`, `Finding`, and `Recommendation` real field content in `modiq-runtime`, replacing the content-free marker types used to validate the Sprint 1 pipeline.
- Added `EvidenceId`, `FindingId`, and `RecommendationId` — process-local, monotonic identity types following the existing `AssessmentId` pattern.
- Added `EvidenceCategory` and `FindingSeverity`, closed classification enums drawn from Glossary.md.
- Added `RuleReference` and `RepairRecipeReference` — opaque Runtime-owned reference types identifying the Rule and, optionally, the Repair Recipe informing a Finding or Recommendation, without Runtime owning or evaluating Knowledge.
- Added constructor-level validation (`EvidenceError`, `FindingError`, `RecommendationError`) rejecting empty or whitespace-only descriptive content.
- Established identity-based equality for all three entities, consistent with `AssessmentId`.
- Extended `Assessment` with relationship-resolution methods (`evidence_by_id`, `finding_by_id`, `evidence_for_finding`, `findings_for_recommendation`), making the aggregate root responsible for resolving cross-entity relationships that were previously stored but never consulted.
- Updated `modiq-rules::RuleEngine` to construct real Finding/Recommendation content; no Rule Engine decision logic changed.
- Expanded the workspace test suite from 55 to 97 tests.

## Deferred (Governance-Pending)

- A new Finding invariant (every Finding must reference at least one Evidence item) and a refinement of INV-005 (a Recommendation must reference specific, existing Finding(s) rather than merely coexist with some Finding) were identified during implementation and recorded as GOV-005 and GOV-006 in `GOVERNANCE.md`. Neither is enforced.

## Released

- Documented in `SPRINT2_IMPLEMENTATION_PLAN.md` (implementation blueprint) and `docs/releases/ENGINEERING_RELEASE_0.2.md` (release record).

---

# [Engineering Release 0.2]

**Status:** Complete

## Added

- Published `docs/releases/ENGINEERING_RELEASE_0.2.md`, the architectural and implementation record for Sprint 2.
- Published ADR-0007 (Runtime Entity Design Pattern), recording the Runtime entity modeling pattern established across Sprint 2: aggregate ownership, entity identity, value objects, opaque references, constructor validation, identity-based equality, aggregate-owned relationship resolution, governance-controlled invariants, and deterministic behavior.
- Added Governance Register items GOV-005 and GOV-006 to `GOVERNANCE.md`.

## Changed

- Updated `PROJECT_STATUS.md` to reflect Sprint 2 completion and Engineering Release 0.2 as the current milestone.
- Updated `CrateRoadmap.md`'s Current Sprint section and revision history to record Sprint 2.
- Completed this file's Sprint 1 record (previously left "In Progress" despite Sprint 1 having concluded at Engineering Release v0.1.0-alpha) and added the Sprint 2 record.

---

# [Documentation Release 2.0]

**Status:** Complete

## Changed

- Reconciled "modIQ Assessment Framework (MAF)" terminology across constitutional and architecture documents to a single canonical expansion and a single meaning (assessment methodology, not a runtime subsystem).
- Updated the constitutional Assessment definition in Glossary.md and propagated it to DataModel.md.
- Retargeted Architecture.md's orchestration-subsystem description from MAF to the existing Assessment Service (EngineAPI.md), with diagrams updated to match.
- Reconciled Version Profile ownership wording in DataModel.md with the frozen architecture (referenced, not owned, by the Assessment).
- Clarified Recommendation provenance wording in RuleEngine.md: Findings trigger Recommendations; Repair Recipes inform their content.
- Updated all frozen specification documents' status metadata from Draft to Frozen.
- Rewrote PROJECT_STATUS.md to reflect current Sprint 1 status and removed obsolete Sprint 0 planning language.
- Updated CrateRoadmap.md and DependencyMap.md so both describe the same crate dependency hierarchy.
- Added VersionProfile.md to the documentation reading order in README.md and EngineeringGuide.md.
- Corrected git tag references from `v0.1.0-docs` (never created) to `v0.1.0` (the actual tag) in README.md and PROJECT_STATUS.md.

## Finalized

- Completed final repository consistency review across constitutional, architecture, governance, and implementation documentation.
- Updated PROJECT_STATUS.md as the authoritative repository dashboard, reflecting Documentation Release 2.0 completion and Sprint 1 as active.
- Converted DocumentationRelease.md from a generic in-progress planning document into a record reflecting completed Documentation Release 1.0 and 2.0 history.
- Confirmed the repository ready to resume Sprint 1 implementation, with Evidence Collection as the next implementation milestone.

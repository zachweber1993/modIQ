# Crate Roadmap

> **The authoritative implementation roadmap for all modIQ platform crates.**

---

| Property | Value |
|----------|-------|
| **Document** | CrateRoadmap.md |
| **Version** | 1.0.0 |
| **Status** | Active |
| **Project** | modIQ |
| **Owner** | Zach Weber |
| **Maintained By** | Technical Director |

---

# Purpose

This document tracks the engineering maturity of every crate within the modIQ platform.

It provides implementation visibility while preserving the architectural boundaries established by the Engineering Specification.

Each crate progresses independently through defined implementation stages while respecting platform dependencies.

---

# Crate Dependency Hierarchy

Implementation proceeds from the lowest-level platform crates upward.

```text
                 modiq-cli
                      │
                      ▼
                modiq-engine
          ┌─────────┼──────────┬──────────┬──────────┐
          ▼         ▼          ▼          ▼          ▼
   modiq-runtime  modiq-knowledge  modiq-rules  modiq-versioning  modiq-report
          │                                 │
          └───────────────┬─────────────────┘
                          ▼
                    modiq-common
```

Additional direct dependencies:

- modiq-rules depends on modiq-runtime (consumes Evidence, Finding, Recommendation)
- modiq-report depends on modiq-runtime (consumes Assessment, Evidence, Finding, Recommendation)

No crate should depend on a crate above it in this hierarchy.

**`modiq-collection`** (ADR-0008): created in Sprint 3 Phase 4, depending on `modiq-runtime` only, with `modiq-engine` depending on it — the same shape as `modiq-rules` and `modiq-report`. Sprint 3 Phase 5 gave it its first real capability: filesystem discovery. See `docs/architecture/EvidenceCollection.md`.

---

# Crate Maturity Levels

| Level | Description |
|--------|-------------|
| L0 | Crate created |
| L1 | Architecture translated into module structure |
| L2 | Domain model implemented |
| L3 | Business logic implemented |
| L4 | Fully tested |
| L5 | Stable |

---

# Implementation Status

| Crate | Responsibility | Current Level | Status |
|--------|----------------|--------------|--------|
| modiq-common | Shared platform types | L1 | ✅ Scaffolded |
| modiq-runtime | Runtime domain | L3 | 🔧 In Progress |
| modiq-knowledge | Engineering knowledge | L1 | ✅ Scaffolded |
| modiq-versioning | Version Profile management | L1 | ✅ Scaffolded |
| modiq-rules | Deterministic rule evaluation | L3 | 🔧 In Progress — internal service-oriented scaffolding (RuleSelector, EvidenceEvaluator, Explainability, Traceability) retired under GOV-004; RuleEngine::evaluate remains the sole implementation |
| modiq-engine | Assessment orchestration | L3 | 🔧 In Progress — internal EngineAPI service scaffolding (KnowledgeService, ReportingService, RuleEvaluationService, VersionProfileService) retired under GOV-004; AssessmentService composes subsystems directly |
| modiq-report | Report representation and formatting | L3 | 🔧 In Progress |
| modiq-cli | Command-line interface | L1 | ✅ Scaffolded |
| modiq-collection | Evidence Collection (produces Evidence from Assessment Subject content) | L2 | 🔧 In Progress — real filesystem discovery (Sprint 3 Phase 5) and real archive discovery (Sprint 4 Phases 3A–3D), both reachable through `AssessmentService`'s explicit routing; XML/Lua collectors not yet implemented |

---

# Crate Completion Workflow

Every crate progresses through the same engineering lifecycle.

## Phase 0 — Scaffold

- Crate created
- Cargo.toml configured
- README.md created
- Module structure established

---

## Phase 1 — Architecture

- Engineering Specification reviewed
- Responsibilities confirmed
- Architectural boundaries verified
- Public module structure defined

---

## Phase 2 — Domain

- Domain entities implemented
- Traits established
- Public interfaces reviewed
- Documentation updated

---

## Phase 3 — Implementation

- Business logic implemented
- Error handling completed
- Integration completed

---

## Phase 4 — Validation

- Unit tests
- Integration tests
- Documentation review
- Technical review

---

## Phase 5 — Stable

The crate is considered complete for the current release.

Future enhancements should occur through new implementation work rather than architectural redesign.

---

# Engineering Rules

A crate may only begin implementation when all lower-level dependencies required by that crate have reached an appropriate maturity level.

Every implementation decision must remain traceable to the Engineering Specification.

No implementation may redefine architectural ownership.

Architectural conflicts must be resolved through the ADR process before implementation continues.

---

# Current Sprint

## Sprint 1 — Complete

Sprint 0 completed the engineering foundation for the modIQ platform: workspace scaffolding, repository organization, crate hierarchy, module hierarchy, documentation hierarchy, governance documentation, dependency map, clean workspace verification, and engineering audit.

Sprint 1 implemented the Assessment lifecycle, deterministic Rule evaluation, and Assessment Report generation across `modiq-runtime`, `modiq-rules`, and `modiq-report`, composed into one executable pipeline by `AssessmentService` in `modiq-engine` and demonstrated end-to-end by an integration test. Tagged `v0.1.0-alpha`.

## Sprint 2 — Complete

Sprint 2 gave `Evidence`, `Finding`, and `Recommendation` — content-free through Sprint 1 — real field content, process-local identity (`EvidenceId`, `FindingId`, `RecommendationId`), and constructor validation, and extended `Assessment` with relationship-resolution methods (`evidence_by_id`, `finding_by_id`, `evidence_for_finding`, `findings_for_recommendation`). The Runtime entity design pattern established across all three is recorded in ADR-0007. No Rule Engine, Reporting, or Engine orchestration behavior changed. Two content-level invariants were identified and deliberately left unenforced, recorded as GOV-005 and GOV-006 in `GOVERNANCE.md`. Frozen as Engineering Release 0.2; see `docs/releases/ENGINEERING_RELEASE_0.2.md`.

## Sprint 3 — Complete

Sprint 3 Phase 1 proved the sandbox could drive the real Evidence → Rule Engine → Assessment → Report pipeline end to end. Sprint 3 Phase 2 resolved GOV-005 and GOV-006 (Finding/Recommendation minimum reference cardinality, INV-013/INV-014). Sprint 3 Phase 3 was an architecture-and-governance-only phase: `PROPOSAL_EVIDENCE_COLLECTION_BOUNDARY.md` was approved at the architectural level, producing ADR-0008 (Evidence Collection Subsystem Boundary), ADR-0009 (AssessmentService Public API Evolution), the new `EvidenceCollection.md` specification, corresponding amendments to `Architecture.md`/`DataModel.md`/`RuleEngine.md`, and four new Governance Register items (GOV-007 through GOV-010); Documentation Release 2.1 was Frozen. Sprint 3 Phase 4 implemented the minimal `modiq-collection` crate proving the boundary (GOV-007 resolved), wired into `modiq-engine` via an additive entry point. A follow-up governance-resolution session then resolved GOV-009 and GOV-010 for the filesystem case, following `PROPOSAL_FILESYSTEM_COLLECTION.md`. Sprint 3 Phase 5 implemented the first real collector against that resolved architecture: real filesystem discovery (files and directories), the approved four-outcome Collection Error Model, Collection Atomicity, and the Phase 5 Symbolic Link Policy, renaming `InputDescriptor`/`InputDescriptorError` to `AssessmentInput`/`AssessmentInputError` throughout. ZIP, XML, and Lua collectors remain future capabilities. Sprint 3 is now frozen as Engineering Release 0.3; see `docs/engineering/ENGINEERING_RELEASE_0.3.md` for the full record, including its recommendation that a second real collector (most likely ZIP traversal) or CLI wiring should follow.

## Platform Validation Phase 1 — Complete

Following Engineering Release 0.3, a Platform Validation cycle reviewed accumulated architectural assumptions against three Sprints of implementation evidence (`PROPOSAL_PLATFORM_VALIDATION_REVIEW.md`). GOV-004 (Engine Service Granularity) was evaluated (`PLATFORM_VALIDATION_GOV-004.md`), approved, and implemented: `AssessmentService` direct subsystem composition is the confirmed engine architecture, and the unused internal `EngineAPI` service model and its `modiq-rules` mirror were removed (`EngineAPI.md` amended to v1.1.0; ADR-0010). GOV-008 (AssessmentService Public API Evolution) was evaluated (`PLATFORM_VALIDATION_GOV-008.md`, `PLATFORM_VALIDATION_EXECUTION_CONTRACT.md`) and deferred: implementation evidence was found insufficient to resolve it, no architectural change was authorized, and the current `AssessmentService` execution contract (both entry points, `AssessmentInput`, `AssessmentReport`, and the public error model) remains the approved platform boundary pending future implementation evidence. Architecture validated for continued roadmap execution.

**Next implementation milestone:** ZIP / Archive Evidence Collection, the platform's second real Evidence Collector — see `docs/engineering/PROPOSAL_ZIP_EVIDENCE_COLLECTION.md`.

---

# Exit Criteria

## Sprint 1 Exit Criteria (met)

Sprint 1 was considered complete when:

- Assessment lifecycle implemented
- Runtime domain operational
- Rule Engine executes deterministic evaluation
- Findings generated from objective evidence
- Recommendations generated from findings
- Assessment reports produced through the Engine API
- Unit tests passing
- Integration tests passing
- Workspace builds without warnings

## Sprint 2 Exit Criteria (met)

Sprint 2 was considered complete when:

- Evidence, Finding, and Recommendation carry real field content, identity, and constructor validation
- Identity-based equality is implemented and tested for all three types
- Assessment resolves Finding→Evidence and Recommendation→Finding relationships as the aggregate root
- Content-level invariants discovered during implementation (GOV-005, GOV-006) are recorded in the Governance Register rather than resolved informally
- Unit tests passing across the full workspace (97 tests, 0 failures)
- Workspace builds without warnings

---

# Revision History

| Version | Date | Description |
|----------|------|-------------|
| 1.0.0 | 2026-07-16 | Initial crate implementation roadmap established. |
| 1.1.0 | 2026-07-16 | Sprint 0 completed. All workspace crates scaffolded and engineering foundation established. |
| 1.2.0 | 2026-07-16 | modiq-runtime advanced to L3. Assessment aggregate, lifecycle transitions, and AssessmentError implemented. |
| 1.3.0 | 2026-07-18 | modiq-rules and modiq-report advanced to L3. Deterministic Rule evaluation, Assessment Report generation, and Recommendation ownership in modiq-runtime implemented; end-to-end pipeline demonstrated by integration test. |
| 1.4.0 | 2026-07-18 | modiq-engine advanced to L3. AssessmentService orchestrates the full pipeline (Runtime Domain, Rule Engine, Reporting); end-to-end integration test moved from modiq-report to modiq-engine to exercise the orchestration layer directly. |
| 1.5.0 | 2026-07-19 | Sprint 2 completed. Evidence, Finding, and Recommendation carry real field content, identity, and constructor validation in modiq-runtime; Assessment extended with relationship-resolution methods (evidence_by_id, finding_by_id, evidence_for_finding, findings_for_recommendation). Runtime entity design pattern recorded in ADR-0007. Workspace test suite expanded to 97 tests. Frozen as Engineering Release 0.2. |
| 1.6.0 | 2026-07-19 | Sprint 3 Phase 3 (architecture/documentation only, no runtime code changed): Evidence Collection Subsystem Boundary approved architecturally (ADR-0008), with AssessmentService's anticipated public API evolution recorded separately (ADR-0009). Added `modiq-collection` as a planned crate (not yet created). Four new Governance Register items opened (GOV-007–GOV-010). Documentation Release 2.1 Frozen. |
| 1.7.0 | 2026-07-19 | Sprint 3 Phase 4: `modiq-collection` created, proving the Evidence Collection boundary with a minimal synthetic collector. GOV-007 resolved. GOV-009/GOV-010 subsequently resolved for the filesystem case (`PROPOSAL_FILESYSTEM_COLLECTION.md`). Sprint 3 Phase 5 implemented the first real collector: filesystem discovery, the four-outcome Collection Error Model, Collection Atomicity, and the Phase 5 Symbolic Link Policy; `InputDescriptor`/`InputDescriptorError` renamed to `AssessmentInput`/`AssessmentInputError`. `modiq-collection` advanced to L2. |
| 1.8.0 | 2026-07-19 | Engineering Release 0.3: Sprint 3 (Phases 1–5) frozen. Living-document reconciliation pass across `PROJECT_STATUS.md`, `CHANGELOG.md`, `docs/README.md`, and this document. Four crates (`modiq-knowledge`, `modiq-versioning`, `modiq-cli`, `modiq-common`) flagged as untouched since Sprint 0 — deliberately, not neglectfully, deferred; see `ENGINEERING_RELEASE_0.3.md`'s Crate Maturity Review. |
| 1.9.0 | 2026-07-20 | Platform Validation cycle: GOV-004 (Engine Service Granularity) resolved. `AssessmentService` direct subsystem composition formalized as the approved engine architecture; the internal `EngineAPI` service model (`KnowledgeService`, `ReportingService`, `RuleEvaluationService`, `VersionProfileService`) and the mirrored `modiq-rules` scaffolding (`RuleSelector`, `EvidenceEvaluator`, `Explainability`, `Traceability`) retired as an implementation target, pending deletion in a future implementation phase. `EngineAPI.md` amended (v1.1.0); ADR-0010 recorded. Governance decision only — no crate advanced or regressed in maturity level. |
| 1.10.0 | 2026-07-20 | GOV-004 implementation: the eight retired stub types named in the 1.9.0 entry deleted from `modiq-engine` and `modiq-rules`, along with their `mod`/`use` declarations. No behavioral change — root workspace tests unchanged at 112/112 passing, Sandbox unchanged at 3/3 passing, zero warnings in both workspaces. No crate's maturity level changed; both were already L3 for the code that remains. |
| 1.11.0 | 2026-07-20 | Platform Validation Phase 1 complete. GOV-008 reviewed against implementation evidence and deferred — insufficient evidence to resolve; no architectural change authorized; the current `AssessmentService` execution contract remains the approved platform boundary. Architecture validated for continued roadmap execution. Next implementation milestone selected: ZIP / Archive Evidence Collection (`PROPOSAL_ZIP_EVIDENCE_COLLECTION.md`). |
| 1.12.0 | 2026-07-20 | GOV-011 (Archive Collection Model) resolved in its entirety following Sprint 4 Phase 1 (Governance Preparation) and Phase 2 (Boundary-Proving): malformed/corrupt archives and resource-limit violations categorized as Unsupported Input; a Duplicate Archive Entry Policy adopted (detection recorded as an observable fact, no fabricated per-entry Evidence, no silent last-write-wins); an Archive Traversal Boundary Policy adopted, covering both relative traversal and absolute-path entries independent of dependency sanitization. `EvidenceCollection.md` amended to v1.2.0. Governance and documentation only — no crate implementation began; Sprint 4 Phase 3 (Real Collector Implementation) is now unblocked. |
| 1.13.0 | 2026-07-20 | Sprint 4 Phase 3 (Real Collector Implementation) completed, across Phases 3A–3C: `ArchiveReader`/`ArchiveEntry`/`ArchiveReadError` (3A) and `ArchiveEvidenceBuilder` (3B) preceded this entry without their own roadmap revision; this entry records both alongside 3C for completeness. Phase 3C assembled them with GOV-011's three remaining policies into `ArchiveCollector`: resource limits and the Archive Traversal Boundary Policy enforced; the Duplicate Archive Entry Policy represented via a new closed-set category, `EvidenceCategory::StructuralDuplication` (`modiq-runtime`), per Technical Director approval of `PROPOSAL_GOV-011_DUPLICATE_REPRESENTATION.md`. `ArchiveCollector` is a real, fully tested Collector Contract implementation, not yet reachable from `AssessmentService` — routing is Phase 3D, not yet authorized. `EvidenceCollection.md` amended to v1.3.0; `Glossary.md`'s Evidence enumeration extended. Workspace test suite grew to 143 tests. |
| 1.14.0 | 2026-07-20 | Sprint 4 Phase 3D: `ArchiveCollector` wired into `AssessmentService::execute_from_assessment_input` via one explicit, inline, case-insensitive `.zip`-suffix routing check (`is_archive_location`) — no dispatcher, registry, trait, or common supertype introduced, per the Technical Director's standing explicit-routing decision. The filesystem `EvidenceCollector` path is unchanged; both collectors converge on the same `execute` pipeline (Evidence → Rule Engine → Report), completing the platform's first end-to-end archive assessment path. `modiq-engine` gained `zip` as a dev-dependency (already an approved workspace dependency, now also used to build real archive fixtures in `AssessmentService`'s own tests). `EvidenceCollection.md`'s Collector Contract amended to record that Collector composition is resolved for a two-collector platform (explicit routing, no abstraction), no longer an open question. Workspace test suite grew to 150 tests; Sandbox reverified unchanged at 3/3. |
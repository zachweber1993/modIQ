# PROJECT_STATUS

| Property | Value |
|----------|-------|
| **Project** | modIQ |
| **Current Release** | Engineering Release 0.5 (Sprints 6 and 7 both implemented, reviewed, committed, and pushed; formal Engineering Release 0.6/0.7 records have not yet been produced) |
| **Repository Status** | Implementation Ready |
| **Current Milestone** | Sprint 7 — Complete (Multi-Source Evidence Collection: XML inspection); Repository Closeout complete |
| **Overall Status** | Active Implementation |
| **Current Phase** | Post-Sprint 7 Repository Closeout complete; Sprint 8 not yet scoped |
| **Engineering Methodology Version** | 1.0 — declared following Sprint 7 and the Engineering Workflow Consolidation (`PROJECT_HANDOFF_v1.0.md`, Section 5) |
| **Last Updated** | 2026-07-21 |

---

# Current Milestone

## Sprint 1 — Complete

Sprint 0 is complete. The engineering foundation was established, including workspace scaffolding, repository governance, crate hierarchy, module hierarchy, documentation hierarchy, architecture verification, and a clean workspace build.

Sprint 1 delivered the first complete, deterministic Assessment pipeline: Assessment creation, Evidence collection, deterministic Rule evaluation, Finding and Recommendation generation, Assessment Report generation, and Assessment completion, composed into one executable operation (`AssessmentService::execute`) and exercised end to end by integration tests. The repository was tagged `v0.1.0-alpha`. Full record: `ENGINEERING_RELEASE_v0.1.0-alpha.md`, `HANDOFF_SPRINT1.md`.

## Sprint 2 — Complete

Sprint 2 gave Evidence, Finding, and Recommendation — content-free in Sprint 1 — real field content, identity, and constructor validation, and extended `Assessment` with relationship-resolution methods (`evidence_by_id`, `finding_by_id`, `evidence_for_finding`, `findings_for_recommendation`). No Rule Engine, Reporting, or Engine orchestration behavior changed; no Knowledge Domain integration began. Two content-level invariants were identified and deliberately left unenforced, recorded as Governance Register items GOV-005 and GOV-006. The workspace test suite grew from 55 to 97 tests. Full record: `docs/releases/ENGINEERING_RELEASE_0.2.md`, `SPRINT2_IMPLEMENTATION_PLAN.md`, ADR-0007.

## Sprint 3 — Complete

Sprint 3 delivered the Evidence Collection subsystem, from architectural boundary through its first real capability, across five phases: Sandbox real-pipeline integration (Phase 1); GOV-005/GOV-006 minimum reference cardinality (Phase 2, INV-013/INV-014); the Evidence Collection Subsystem Boundary architecture (Phase 3, ADR-0008, ADR-0009, Documentation Release 2.1); a minimal `modiq-collection` crate proving the boundary (Phase 4, GOV-007); and the first real, filesystem-backed collector (Phase 5, GOV-009/GOV-010, deterministic traversal, the four-outcome Collection Error Model, Collection Atomicity, the Symbolic Link Policy). The workspace grew to nine crates; the root test suite grew from 97 to 112 tests. Full record: `ENGINEERING_RELEASE_0.3.md`.

## Platform Validation Phase 1 — Complete

Following Engineering Release 0.3, a Platform Validation cycle reviewed accumulated architectural assumptions against three Sprints of implementation evidence (`PROPOSAL_PLATFORM_VALIDATION_REVIEW.md`). GOV-004 (Engine Service Granularity) was evaluated, approved, and implemented: `AssessmentService` direct subsystem composition is the confirmed engine architecture, and the unused internal `EngineAPI` service model (and its `modiq-rules` mirror) was removed. GOV-008 (AssessmentService Public API Evolution) was evaluated and deferred: implementation evidence was found insufficient to resolve it, no architectural change was authorized, and the current `AssessmentService` execution contract remains the approved platform boundary pending future implementation evidence. Architecture validated for continued roadmap execution. Full record: `docs/engineering/ENGINEERING_LOG.md`.

**Next implementation milestone (superseded by Sprint 4, below):** ZIP / Archive Evidence Collection, the platform's second real Evidence Collector — see `docs/engineering/PROPOSAL_ZIP_EVIDENCE_COLLECTION.md`. CLI wiring remains a separately available, low-risk parallel track, not yet scoped in detail.

## Sprint 4 — Complete

Sprint 4 delivered the platform's second real Evidence Collector, archive-based, across four implementation phases plus a governance-resolution and boundary-proving cycle. GOV-011 (Archive Collection Model) was resolved in its entirety (`PROPOSAL_GOV-011.md`), informed by Phase 2's empirical investigation of the `zip` crate (v8.6.0) — the platform's first archive-parsing dependency and first genuinely adversarial input surface. Phase 3A implemented deterministic archive structural enumeration (`ArchiveReader`); Phase 3B transformed that structure into real Evidence (`ArchiveEvidenceBuilder`), reusing `EvidenceCategory::FileStructureAnalysis` unchanged; Phase 3C assembled both with GOV-011's three remaining policies (resource limits, the Archive Traversal Boundary Policy, and the Duplicate Archive Entry Policy) into one real Collector (`ArchiveCollector`), introducing one new closed-set category, `EvidenceCategory::StructuralDuplication`, following a dedicated Architecture Review (`PROPOSAL_GOV-011_DUPLICATE_REPRESENTATION.md`); Phase 3D wired `ArchiveCollector` into `AssessmentService` via one explicit, inline routing check, completing the platform's first end-to-end archive assessment path. No collector-dispatch abstraction was introduced at any phase. A Sprint 4 Closeout reconciled repository documentation, validated the archive path through the Sandbox application against a checked-in fixture, and produced an engineering retrospective (recorded inside `ENGINEERING_RELEASE_0.4.md` itself, per this project's established pattern of housing a release's retrospective in its own release document rather than a separate file). The workspace grew from 112 to 150 root tests (Sandbox: 3 to 6); nine workspace crates, unchanged in count. Full record: `docs/engineering/ENGINEERING_RELEASE_0.4.md`.

## Sprint 5 — Complete

Sprint 5 deliberately did not add a third Evidence Collector; per Technical Director direction, it strengthened the platform vertically instead, building the assessment intelligence layer — the Rule Engine's own depth — before widening Evidence Collection further. GOV-012 (Rule Evaluation Model) was resolved: `RuleEngine::evaluate` returns `Vec<RuleOutcome>`, Rules are dispatched in fixed, explicit declaration order, and compose independently with no suppression model. GOV-013 (FindingSeverity Severity/Kind Conflation) was opened, deliberately Open, not Resolved — the Technical Director accepted a real architectural tension (`BestPractice` classifies Finding *kind*, not severity, unlike `Error`/`Warning`/`Informational`) as provisionally accepted rather than deciding a Runtime model change from two Rules alone, to be revisited once more concrete Rules exist. `DataModel.md` was amended to v1.1.0 with the platform's first specification-level Finding Severity definitions. `modiq-rules` gained its second concrete Rule, `StructuralDuplicationRule` (Phase 2), and the original Sprint 1 Rule was extracted into its own unit, `EvidencePresenceRule`, both dispatched by the new multi-Rule `RuleEngine::evaluate` (Phase 3) — no trait, registry, or dispatch abstraction introduced. A dedicated investigation (Phase 4) recommended retiring `modiq-report`'s four unused scaffold types (`FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, `ReportFormatter`), using the same evidentiary method GOV-004 used, but no action was taken this Sprint. Phase 5 closed one determinism-testing gap (Rule outcome order confirmed independent of Evidence arrival order, not just repeated-identical-input order). A Sprint 5 Closeout reconciled repository documentation and produced an engineering retrospective (inside `ENGINEERING_RELEASE_0.5.md`). The workspace grew from 150 to 162 root tests (Sandbox unchanged at 6); nine workspace crates, unchanged in count. Full record: `docs/engineering/ENGINEERING_RELEASE_0.5.md`.

## Sprint 6 — Complete (Implementation and Merge)

Sprint 6 closed two of the three roadmap candidates named at Sprint 5 Closeout. `modiq-cli` was wired to `modiq-engine` for the first time since Sprint 0: `Application` dispatches `assess`/`help`/`version` by one direct match (no command trait or registry); `AssessCommand` calls `AssessmentService::execute_from_assessment_input` against a real, user-supplied path — the same entry point the Sandbox already calls, reused rather than reinvented — mapping the result to a three-tier exit-code convention (0 success, 1 execution failure, 2 invalid usage). `modiq-report`'s four scaffold types recommended for retirement at Sprint 5 Phase 4 (`FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, `ReportFormatter`) were deleted under this Sprint's explicit Chief Architect authorization; `AssessmentReport`, the crate's real, tested content, is unchanged. No change to `AssessmentService`'s public entry points, `AssessmentInput`, or the public error model. `modiq-cli` advanced from L1 to L2. The workspace grew from 162 to 172 root tests (`modiq-cli` 0 → 10; `modiq-report` unchanged at 3, confirming the deletion had zero test-coverage impact); Sandbox reverified unchanged at 6/6. Implementation was committed (`397707f`) on `feature/sprint6-cli`, reviewed, and merged into `feature/runtime-implementation` (`29657df`) with a full post-merge revalidation. Full record: `docs/engineering/SPRINT6_IMPLEMENTATION_PLAN.md` (including its Authorization Record) and `docs/engineering/POST_SPRINT6_REPOSITORY_ASSESSMENT.md`. A formal `ENGINEERING_RELEASE_0.6.md` record, matching every prior Sprint's own convention, has not yet been produced — noted here as outstanding, not silently assumed complete.

## Sprint 7 — Complete (Multi-Source Evidence Collection: XML Inspection)

Sprint 7 closed the sole remaining candidate from the original three-item Sprint 6 roadmap. Rather than beginning with implementation, Sprint 7 began with a capability question — "what new capability should XML inspection provide?" — and, when that surfaced the platform's first need for more than one Collector to run per Assessment, was preceded by a dedicated Architecture Evaluation (`COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`) before any code was written. The approved architecture: Collection is a phase of an Assessment, not a standalone component; Collectors remain fully independent and never consume one another's output; composition is direct and inline inside `AssessmentService`, with no `CollectionCoordinator` introduced. `XmlCollector` now runs alongside the existing structural Collector for every Assessment, producing `XmlInspection` Evidence — manifest presence, well-formedness, and declared dependencies — with a missing manifest itself recorded as Evidence rather than silently treated as Empty Collection. No Rule Engine change; `modiq-rules` is untouched. New dependency: `roxmltree`. The workspace grew from 172 to 187 root tests (Sandbox 6 → 7). Implementation committed and pushed directly to `feature/runtime-implementation` (`277aefd`) — no separate Sprint branch was created this cycle, unlike Sprint 6.

Sprint 7 also consolidated the repository's engineering workflow: nine independent, drifted workflow descriptions (`ENGINEERING_WORKFLOW_CONSOLIDATION_STUDY.md`) reduced to one canonical source (`PROJECT_HANDOFF_v1.0.md`, Section 5), terminology unified project-wide, `ImplementationWorkflow.md` archived as non-normative history, and the Implementation Report template reconciled with demonstrated Sprint 6/7 practice (`ENGINEERING_WORKFLOW_CONSOLIDATION_REPORT.md`). The engineering methodology is now designated **Version 1.0** — exercised across multiple completed Sprints, consolidated into one canonical process, and expected to evolve only through the same evidence-based process used throughout this repository, not routine amendment.

Full record: `SPRINT7_CAPABILITY_AND_IMPLEMENTATION_PLAN.md`, `COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`, `SPRINT7_IMPLEMENTATION_AUTHORIZATION.md`, `SPRINT7_IMPLEMENTATION_REPORT.md`, `ENGINEERING_WORKFLOW_CONSOLIDATION_STUDY.md`, `ENGINEERING_WORKFLOW_CONSOLIDATION_REPORT.md`. Formal `ENGINEERING_RELEASE_0.6.md` and `_0.7.md` records, matching every earlier Sprint's own convention, remain outstanding — noted here as outstanding, not silently assumed complete.

**Next implementation milestone:** not yet scoped. Sprint 8 has not begun as of this record.

---

# Documentation Progress

## Phase 1 — Foundation

### Repository

| Item | Status |
|------|--------|
| Repository Foundation | ✅ Complete |
| Documentation Standards | ✅ Complete |
| Governance | ✅ Complete |

### Constitutional Specifications

| Document | Status |
|----------|--------|
| Vision.md | ✅ Frozen |
| Principles.md | ✅ Frozen |
| Glossary.md | ✅ Frozen |
| ProductSpecification.md | ✅ Frozen |

### Platform Architecture

| Document | Status |
|----------|--------|
| Architecture.md | ✅ Frozen |

---

## Phase 2 — Technical Specifications

| Document | Status |
|----------|--------|
| DataModel.md | ✅ Frozen |
| KnowledgeModel.md | ✅ Frozen |
| RuleEngine.md | ✅ Frozen |
| EngineAPI.md | ✅ Frozen |

---

## Phase 3 — Implementation Planning

| Document | Status |
|----------|--------|
| Sprint0.md | ✅ Frozen |
| Sprint1.md | ✅ Frozen |

---

# Documentation Release Status

| Property | Value |
|----------|-------|
| **Current Release** | Documentation Release 2.1 |
| **Status** | ✅ Complete (Frozen) |
| **Constitutional Layer** | ✅ Frozen |
| **Architecture Layer** | ✅ Frozen (amended under 2.1 — Evidence Collection named a Core Platform Component; Assessment Lifecycle diagram ordering corrected) |
| **Technical Layer** | ✅ Frozen (new: `EvidenceCollection.md`) |
| **Governance Documentation** | ✅ Reconciled |
| **Repository Audit** | ✅ Passed |
| **Implementation Readiness** | ✅ Verified |
| **Previous Release** | Documentation Release 2.0 |

Governance documentation (EngineeringGuide.md, CHANGELOG.md, ROADMAP.md, PROJECT_STATUS.md, and related tracking documents) is reconciled with current repository state as of this release. Unlike the constitutional and architecture layers, governance documentation remains a living body of documentation per EngineeringGuide.md and is not designated Frozen — and, as `ENGINEERING_RELEASE_0.3.md`'s Repository Health section notes directly, has a documented history of falling behind between releases despite that.

---

# Current Focus

Documentation Releases 1.0, 2.0, and 2.1 have all concluded; Documentation Release 2.1 (Evidence Collection subsystem boundary) is the current one, amended three times since its own freeze to record GOV-011's resolution and its Sprint 4 Phase 3C/3D implementation (`EvidenceCollection.md` v1.2.0 → v1.4.0). `DataModel.md` was separately amended to v1.1.0 during Sprint 5 to add Finding Severity definitions (GOV-012/GOV-013). Engineering Release 0.4 froze Sprint 4 (Phases 1–3D, plus Closeout); Engineering Release 0.5 froze Sprint 5 (Phases 1–5, plus Closeout). Platform Validation Phase 1 closed before Sprint 4 began, confirming the engine architecture (GOV-004) and deferring GOV-008 pending future evidence — GOV-008 remains open, untouched by Sprints 4, 5, and 6 (Sprint 6 reused `AssessmentService`'s existing entry points exactly as designed and generated no new evidence toward it). Sprint 6 is complete: `modiq-cli` is wired to `modiq-engine`, and `modiq-report`'s four scaffold types are retired. XML inspection is the sole remaining candidate from the original three-item Sprint 6 roadmap, not yet scoped for Sprint 7.

Implementation should remain consistent with the frozen engineering specification.

---

# Implementation Policy

The documentation contained within this repository is the authoritative engineering specification for modIQ.

Implementation should reconcile all engineering decisions against the frozen specifications.

Architectural changes should be introduced through Architecture Decision Records (ADRs) and incorporated into future Documentation Releases rather than modifying Documentation Release 1.0.

---

## Governance Status

Status: Frozen for Sprint 6, carried into Sprint 7 (both complete); baseline carries forward unchanged into Sprint 8. Engineering Methodology Version 1.0 declared following Sprint 7 (`PROJECT_HANDOFF_v1.0.md`, Section 5) — a versioning of the workflow itself, distinct from this governance baseline.

The project's governance baseline is established.

Authoritative governance documents:

- PROJECT_HANDOFF_v1.0.md
- CHIEF_ARCHITECT_HANDOFF_v1.0.md
- LEAD_ENGINEER_HANDOFF_v3.0.md

Future governance changes must be justified by implementation evidence,
architectural evolution, or approved governance decisions.

Sprint 6 begins under this governance baseline.

---

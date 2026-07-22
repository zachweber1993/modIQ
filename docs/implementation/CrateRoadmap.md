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
- modiq-cli depends directly on modiq-runtime and modiq-report, in addition to modiq-engine (Sprint 6): it constructs `AssessmentSubject`/`AssessmentContext` (modiq-runtime) and names `AssessmentReport` (modiq-report) in its own formatting code, since modiq-engine does not re-export either type. The diagram above shows modiq-cli's original, single edge to modiq-engine only; both additional edges are real and unchanged since Sprint 6, not yet reflected in the diagram itself.
- **modiq-engine depends on modiq-versioning (Sprint 8):** `AssessmentService` constructs `VersionProfile::fs25()` directly and passes it to the Rule Engine — the crate's first real consumer. `modiq-runtime` does **not** depend on `modiq-versioning`: `Assessment` records only an opaque `VersionProfileReference` (mirroring `RuleReference`/`RepairRecipeReference`, ADR-0007), preserving `modiq-runtime` as the platform's sole dependency-free leaf, unbroken since Sprint 0.
- **modiq-rules depends on modiq-versioning (Sprint 8):** `VersionCompatibilityRule` consults the real `VersionProfile` type directly to evaluate a mod's declared `descVersion`. Both new edges are real and confirmed via `Cargo.lock`; the diagram above does not yet show either.

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
| modiq-versioning | Version Profile management | L2 | 🔧 In Progress — first real content since Sprint 0 (Sprint 8): `GameVersion`/`VersionProfile` with real fields, `supports()`, and a single hardcoded `VersionProfile::fs25()`; `Capability`/`Compatibility` remain unimplemented, deliberately deferred pending a forcing function |
| modiq-rules | Deterministic rule evaluation | L3 | 🔧 In Progress — internal service-oriented scaffolding (RuleSelector, EvidenceEvaluator, Explainability, Traceability) retired under GOV-004; `RuleEngine::evaluate` dispatches three real Rules in fixed declaration order (`EvidencePresenceRule`, `StructuralDuplicationRule`, `VersionCompatibilityRule` — Sprint 8), the last consuming `modiq-versioning` directly |
| modiq-engine | Assessment orchestration | L3 | 🔧 In Progress — internal EngineAPI service scaffolding (KnowledgeService, ReportingService, RuleEvaluationService, VersionProfileService) retired under GOV-004; AssessmentService composes subsystems directly, including `modiq-versioning` since Sprint 8 |
| modiq-report | Report representation and formatting | L3 | 🔧 In Progress — the four unused scaffold types recommended for retirement at Sprint 5 Phase 4 (`FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, `ReportFormatter`) were retired Sprint 6; the crate now contains only `AssessmentReport`, its real, tested content |
| modiq-cli | Command-line interface | L2 | 🔧 In Progress — wired to `modiq-engine` Sprint 6: `Application` dispatches `assess`/`help`/`version`, `AssessCommand` calls `AssessmentService::execute_from_assessment_input` against a user-supplied path, reusing the same entry point the Sandbox already calls |
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

**Next implementation milestone (superseded by Sprint 4, below):** ZIP / Archive Evidence Collection, the platform's second real Evidence Collector — see `docs/engineering/PROPOSAL_ZIP_EVIDENCE_COLLECTION.md`.

## Sprint 4 — Complete

Sprint 4 gave `modiq-collection` its second real Collector, archive-based, resolving GOV-011 (Archive Collection Model) in full and implementing against it across four phases: `ArchiveReader` (structural enumeration, Phase 3A), `ArchiveEvidenceBuilder` (Evidence generation reusing `FileStructureAnalysis`, Phase 3B), `ArchiveCollector` (resource limits, the Archive Traversal Boundary Policy, and the Duplicate Archive Entry Policy — represented via a new closed-set category, `EvidenceCategory::StructuralDuplication` in `modiq-runtime` — assembled into one real Collector, Phase 3C), and explicit routing wired into `AssessmentService` (Phase 3D), completing the platform's first end-to-end archive assessment path. No collector-dispatch abstraction was introduced. A Sprint 4 Closeout followed: repository documentation reconciled, the archive path validated through the Sandbox application against a checked-in fixture, and an engineering retrospective produced. `modiq-collection` remains L2; the root workspace test suite grew from 112 to 150 tests. Full record: `docs/engineering/ENGINEERING_RELEASE_0.4.md`.

## Sprint 5 — Complete

Sprint 5 deliberately did not extend Evidence Collection; per Technical Director direction, it strengthened the assessment intelligence layer instead — the Rule Engine's own depth — before a third Collector arrives. GOV-012 (Rule Evaluation Model) resolved `RuleEngine::evaluate`'s multi-Rule shape (`Vec<RuleOutcome>`, explicit declaration order, independent composition); GOV-013 (FindingSeverity Severity/Kind Conflation) was opened, deliberately Open rather than resolved, accepting the current `FindingSeverity` model as provisional pending more concrete Rules. `DataModel.md` gained the platform's first specification-level Finding Severity definitions (v1.1.0). `modiq-rules` gained its second concrete Rule (`StructuralDuplicationRule`, Phase 2) and its original Rule extracted into its own unit (`EvidencePresenceRule`), both dispatched by the new multi-Rule `RuleEngine::evaluate` (Phase 3) — no trait, registry, or dispatch abstraction introduced. A dedicated investigation (Phase 4) recommended retiring `modiq-report`'s four unused scaffold types, using the same evidentiary method GOV-004 used, but no action was taken this Sprint. A Sprint 5 Closeout followed: repository documentation reconciled (including correcting a broken cross-reference to a `SPRINT4_RETROSPECTIVE.md` file that was never created) and an engineering retrospective produced. `modiq-rules` advanced with real multi-Rule content; the root workspace test suite grew from 150 to 162 tests. Full record: `docs/engineering/ENGINEERING_RELEASE_0.5.md`.

## Sprint 6 — Complete

Sprint 6 closed two of the three roadmap candidates named at Sprint 5 Closeout, deliberately not touching the third (XML inspection). `modiq-cli` was wired to `modiq-engine` for the first time since Sprint 0: `Application` dispatches `assess`/`help`/`version` by one direct match, `AssessCommand` calls `AssessmentService::execute_from_assessment_input` against a real, user-supplied path, mapping the result to a three-tier exit-code convention — no command trait, registry, or new external dependency introduced. `modiq-report`'s four scaffold types recommended for retirement at Sprint 5 Phase 4 were deleted under this Sprint's explicit Chief Architect authorization; `AssessmentReport` is unchanged. No change to `AssessmentService`'s public entry points. `modiq-cli` advanced from L1 to L2; the root workspace test suite grew from 162 to 172 tests. Implementation committed and merged into `feature/runtime-implementation`; a formal `ENGINEERING_RELEASE_0.6.md` record has not yet been produced. Full record: `docs/engineering/SPRINT6_IMPLEMENTATION_PLAN.md` and `docs/engineering/POST_SPRINT6_REPOSITORY_ASSESSMENT.md`.

## Sprint 7 — Complete

Sprint 7 closed the sole remaining Sprint 6 roadmap candidate: XML inspection. `modiq-collection` gained its third real Collector, `XmlCollector` — the platform's first content-inspecting Collector and first multi-Collector Assessment, composed inline alongside the existing structural Collector per a dedicated Architecture Evaluation (`COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`). No Rule Engine change. The root workspace test suite grew from 172 to 187 tests. Sprint 7 also consolidated the repository's engineering workflow into one canonical source (`PROJECT_HANDOFF_v1.0.md`, Section 5) and declared Engineering Methodology Version 1.0. Full record: `docs/engineering/ENGINEERING_RELEASE_0.7.md`.

## Sprint 8 — Complete

Sprint 8 activated Version Profile-aware compatibility checking — the architectural dimension specified since Documentation Release 1.0 (`VersionProfile.md`, ADR-0004) but unimplemented through seven Sprints. `modiq-versioning` gained its first real content since Sprint 0 (`GameVersion`, `VersionProfile::fs25()`); `XmlCollector` was extended to extract a mod's declared `descVersion` as factual Evidence; `modiq-rules` gained its third Rule, `VersionCompatibilityRule`, evaluating that declared value against the active Version Profile inside the Rule Engine. `Assessment` records the active profile through a new opaque `VersionProfileReference` (extending ADR-0007's Opaque Runtime References pattern), preserving `modiq-runtime` as the platform's dependency-free leaf. `AssessmentService`'s two public entry points required zero signature change. The root workspace test suite grew from 187 to 205 tests; Sandbox unchanged at 7/7. Full record: `docs/engineering/ENGINEERING_RELEASE_0.8.md`, `docs/engineering/SPRINT8_IMPLEMENTATION_REPORT.md`, `docs/engineering/SPRINT8_IMPLEMENTATION_DEVIATIONS.md`.

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
| 1.15.0 | 2026-07-21 | Sprint 5 Phases 1–3 (Assessment Intelligence Layer): GOV-012 (Rule Evaluation Model) and GOV-013 (FindingSeverity Severity/Kind Conflation, Open by design) resolved into the Governance Register; `DataModel.md` amended to v1.1.0 with the platform's first specification-level Finding Severity definitions. `modiq-rules` gained its second concrete Rule, `StructuralDuplicationRule` (evaluates `EvidenceCategory::StructuralDuplication`, assigns `FindingSeverity::Warning`), and the original Sprint 1 Rule was extracted into its own unit, `EvidencePresenceRule`, both now dispatched by `RuleEngine::evaluate`'s new `Vec<RuleOutcome>` return shape in fixed, explicit declaration order, composing independently with no suppression model — no trait, registry, or dispatch abstraction introduced. `modiq-engine`'s `AssessmentService::execute` internal handling updated to loop over the new return shape; its public signature is unchanged. Workspace test suite grew to 161 tests (`modiq-rules` 3 → 14); Sandbox reverified unchanged at 6/6. |
| 1.16.0 | 2026-07-21 | Sprint 5 Phases 4–5: investigated `modiq-report`'s four unused scaffold types (`FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, `ReportFormatter`) against Sprint 5's own new severity differentiation; recommended retirement (same evidentiary method as GOV-004), eligible pending formal governance approval, not acted on this Sprint. Closed one test-coverage gap: `RuleEngine::evaluate`'s outcome order confirmed independent of Evidence arrival order, not just repeated-identical-input order. All five Sprint 5 implementation phases now complete. Workspace test suite grew to 162 tests (`modiq-rules` 14 → 15); Sandbox unchanged at 6/6. |
| 1.17.0 | 2026-07-21 | Sprint 5 Closeout: repository reconciled against `SPRINT5_IMPLEMENTATION_PLAN.md`'s Completion Checklist (all items satisfied). `PROJECT_STATUS.md` and `CHANGELOG.md`, found stale again despite Sprint 4 Closeout's own process-improvement directive, corrected — the same recurring pattern named at every closeout to date. A genuine documentation error found and corrected: both files referenced a `SPRINT4_RETROSPECTIVE.md` that was never created (Sprint 4's retrospective lives inside `ENGINEERING_RELEASE_0.4.md` itself). `docs/README.md`'s Engineering Release cross-reference updated to 0.5. `modiq-report`'s crate-table row updated to record Phase 4's retirement recommendation. `ENGINEERING_RELEASE_0.5.md` published as the full Sprint 5 record, retrospective, and completion report, mirroring `ENGINEERING_RELEASE_0.4.md`'s own structure. No production code changed. |
| 1.18.0 | 2026-07-21 | Sprint 6: `modiq-cli` wired to `modiq-engine` for the first time — `Application` dispatches `assess`/`help`/`version` by one direct match (no command trait or registry), `AssessCommand` calls `AssessmentService::execute_from_assessment_input` against a real, user-supplied path (not a fixed fixture, unlike the Sandbox's own use of the same entry point), mapping the result to a three-tier exit-code convention (0 success, 1 execution failure, 2 invalid usage). `modiq-report`'s four scaffold types (`FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, `ReportFormatter`), recommended for retirement at Sprint 5 Phase 4, deleted under this Sprint's explicit authorization; the crate now contains only `AssessmentReport`. No change to `AssessmentService`'s public entry points, `AssessmentInput`, or the public error model. `modiq-cli` advanced from L1 to L2. Workspace test suite grew from 162 to 172 tests (`modiq-cli` 0 → 10; `modiq-report` unchanged at 3, confirming the deletion had zero test-coverage impact); Sandbox reverified unchanged at 6/6. |
| 1.19.0 | 2026-07-21 | Sprint 7: `modiq-collection` gained its third real Collector, `XmlCollector` — the platform's first content-inspecting Collector and first multi-Collector Assessment, composed inline in `AssessmentService::execute_from_assessment_input` per `COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`'s approved architecture. No Rule Engine change. Workspace test suite grew from 172 to 187 tests; Sandbox 6 → 7. Engineering Workflow Consolidation: nine drifted workflow descriptions reduced to one canonical source (`PROJECT_HANDOFF_v1.0.md`, Section 5); Engineering Methodology Version 1.0 declared. |
| 1.20.0 | 2026-07-22 | Sprint 8: Version Profile-aware compatibility checking. `modiq-versioning` advanced from L1 to L2 — its first real content since Sprint 0 (`GameVersion`, `VersionProfile::fs25()`). `XmlCollector` extended to extract a mod's declared `descVersion` as factual Evidence. `modiq-rules` gained its third Rule, `VersionCompatibilityRule`, dispatched third in `RuleEngine::evaluate`'s fixed declaration order, now accepting a `VersionProfile` parameter; `modiq-rules` and `modiq-engine` each gained their first dependency on `modiq-versioning`. `Assessment` gained a new opaque `VersionProfileReference` field (ADR-0007's Opaque Runtime References pattern, extended for the first time to a domain relationship); `modiq-runtime` gained no new dependency. `AssessmentService`'s two public entry points required zero signature change. Workspace test suite grew from 187 to 205 tests; Sandbox unchanged at 7/7. |
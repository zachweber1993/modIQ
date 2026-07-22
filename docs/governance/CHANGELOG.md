# CHANGELOG

| Property | Value |
|----------|-------|
| **Document** | CHANGELOG.md |
| **Project** | modIQ |
| **Purpose** | Repository History |
| **Maintained By** | Project Maintainers |
| **Last Updated** | 2026-07-21 |

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

---

# [Sprint 3]

**Status:** Complete (Phases 1–5)

## Added

- **Phase 1** — Wired `apps/sandbox`'s `create_assessment` command to the real `AssessmentService::execute` pipeline, replacing the empty-Assessment DTO used since Sandbox Phase 2.
- **Phase 2** — Enforced minimum reference cardinality on `Finding` (`evidence_ids`) and `Recommendation` (`finding_ids`), recorded as new invariants INV-013 and INV-014. Resolved GOV-005 and GOV-006 (cardinality only; referential integrity deliberately left open).
- **Phase 3** — Designed and approved the Evidence Collection Subsystem Boundary (`PROPOSAL_EVIDENCE_COLLECTION_BOUNDARY.md`). Added ADR-0008 and ADR-0009. Added `docs/architecture/EvidenceCollection.md`. Amended `Architecture.md` (Evidence Collection named a Core Platform Component; Assessment Lifecycle diagram ordering corrected to match `DataModel.md`) and added cross-references in `DataModel.md`/`RuleEngine.md`. Added Governance Register items GOV-007 through GOV-010. Froze Documentation Release 2.1.
- **Phase 4** — Created `modiq-collection`, the platform's ninth workspace crate, with a minimal synthetic collector proving the Evidence Collection boundary. Added `AssessmentService::execute_from_descriptor` (later renamed, see Phase 5) as a new, additive Engine entry point, leaving `execute` unchanged. Resolved GOV-007.
- **Governance resolution (between Phases 4 and 5)** — Designed and approved Filesystem Collection's architecture (`PROPOSAL_FILESYSTEM_COLLECTION.md`). Resolved GOV-009 (Assessment Input Ownership) and GOV-010 (Collection Error Model) for the filesystem case, including the Collection Atomicity and Symbolic Link Policy decisions.
- **Phase 5** — Implemented the first real collector: deterministic filesystem discovery of files and directories, the approved four-outcome Collection Error Model (`CollectionError`), Collection Atomicity, and the Symbolic Link Policy. Renamed `InputDescriptor`/`InputDescriptorError` to `AssessmentInput`/`AssessmentInputError` throughout, and `execute_from_descriptor` to `execute_from_assessment_input`. Updated the Sandbox to exercise the real collector against a fixed, checked-in fixture directory.
- Expanded the root workspace test suite from 97 to 112 tests.

## Deferred (Governance-Pending)

- GOV-001, GOV-002, GOV-003, GOV-004, and GOV-008 remain open; none were addressed this Sprint.
- Referential integrity for Finding/Recommendation references (the GOV-005/GOV-006 follow-up) remains unassigned to a Governance Register item, though Phase 5's real collector is noted in `ENGINEERING_RELEASE_0.3.md` as a plausible forcing function that did not exist when the follow-up was first deferred.

## Released

- Documented in `ENGINEERING_RELEASE_0.3.md` (release record), `ROADMAP_REVIEW_2026.md` (the reassessment that recommended this Sprint's later phases), and the `PROPOSAL_EVIDENCE_COLLECTION_BOUNDARY.md` / `PROPOSAL_FILESYSTEM_COLLECTION.md` proposals that preceded Phases 3 and 5 respectively.

---

# [Documentation Release 2.1]

**Status:** Complete (Frozen)

## Added

- `docs/architecture/EvidenceCollection.md` — a new Technical Layer specification: Evidence Collection subsystem ownership, orchestration flow, responsibilities/non-responsibilities, Assessment Input, the Collector Contract, Collection Outcomes, Collection Atomicity, and the Symbolic Link Policy.
- ADR-0008 (Evidence Collection Subsystem Boundary) and ADR-0009 (AssessmentService Public API Evolution).

## Changed

- `Architecture.md` amended to name Evidence Collection as a Core Platform Component (System Overview, Assessment Lifecycle, Dependency Rules, Extensibility) and to correct the Assessment Lifecycle diagram's ordering, resolving a documented inconsistency with `DataModel.md`. The amendment is recorded explicitly within the document, not silently made.
- Minor cross-reference additions to `DataModel.md` and `RuleEngine.md`.
- `docs/README.md`'s Reading Order updated to include `EvidenceCollection.md`.

---

# [Engineering Release 0.3]

**Status:** Complete

## Added

- Published `docs/engineering/ENGINEERING_RELEASE_0.3.md`, the architectural and implementation record for Sprint 3 (Phases 1–5).

## Changed

- Updated `PROJECT_STATUS.md` to reflect Sprint 3 completion, Engineering Release 0.3, and Documentation Release 2.1 as current.
- Updated `CrateRoadmap.md`'s crate maturity table and Current Sprint section to reflect `modiq-collection`'s creation and its Phase 5 capability.
- Completed this file's Sprint 3, Documentation Release 2.1, and Engineering Release 0.3 records.

## Noted, Not Resolved

- As of this release, the work it records (Sprint 3 Phase 5, the preceding governance-resolution session, and the Roadmap Review and Filesystem Collection proposal before that) remains uncommitted in the repository's working tree. See `ENGINEERING_RELEASE_0.3.md`, Repository Health.
- `v0.3.0` already exists as a git tag from unrelated, earlier repository history (pre-existing git tag hygiene debt); this Engineering Release is not tagged, to avoid colliding with it.

---

# [Platform Validation Phase 1]

**Status:** Complete

## Reviewed

- GOV-004 (Engine Service Granularity) — evaluated in `PLATFORM_VALIDATION_GOV-004.md`, approved via `PROPOSAL_GOV-004.md`, and implemented: `AssessmentService` direct subsystem composition confirmed as the engine architecture; the unused internal `EngineAPI` service model and its `modiq-rules` mirror removed. `EngineAPI.md` amended to v1.1.0. ADR-0010 recorded.
- GOV-008 (AssessmentService Public API Evolution) — evaluated in `PLATFORM_VALIDATION_GOV-008.md` and `PLATFORM_VALIDATION_EXECUTION_CONTRACT.md`. Implementation evidence found insufficient to resolve it; no architectural change authorized. The current `AssessmentService` execution contract (both entry points, `AssessmentInput`, `AssessmentReport`, and the public error model) remains the approved platform boundary pending future implementation evidence. GOV-008 remains Open.

## Changed

- `GOVERNANCE.md`: GOV-008's Resolution field gained one paragraph recording the review outcome; Status unchanged (Open).
- `CrateRoadmap.md`: new "Platform Validation Phase 1 — Complete" subsection and revision history entry.
- `PROJECT_STATUS.md`: Current Milestone, Current Phase, and Current Focus updated to reflect Platform Validation Phase 1 as complete and to name the next milestone; a stale "pending commit" Repository Status note, accurate before the Sprint 3 backlog was committed, corrected in the same pass.

## Not Resolved

- GOV-008 remains open, deliberately, pending future implementation evidence.

## Next

- ZIP / Archive Evidence Collection proposed (`PROPOSAL_ZIP_EVIDENCE_COLLECTION.md`), the platform's second real Evidence Collector — awaiting Technical Director review. CLI wiring remains an independent, unscoped parallel track.

---

# [Sprint 4]

**Status:** Complete (Phases 1–3D, plus Closeout)

## Added

- **Phase 1 (Governance Preparation)** and **Phase 2 (Boundary-Proving)** — drafted and empirically validated candidate answers to GOV-011's four questions against the `zip` crate (v8.6.0) in a standalone investigation: deterministic entry enumeration (explicit sort required), clean malformed-archive failure, metadata-only resource-limit quantities (~1,270× faster than full decompression), duplicate entry names only partially observable through the dependency's ordinary API, and `enclosed_name()` sanitizing rather than rejecting absolute-path entries.
- **GOV-011 (Archive Collection Model)** resolved in its entirety (`PROPOSAL_GOV-011.md`): malformed/corrupt archives and resource-limit violations categorized as Unsupported Input; the Duplicate Archive Entry Policy adopted (detection recorded as an observable fact, no fabricated per-entry Evidence, no silent last-write-wins); the Archive Traversal Boundary Policy adopted, covering relative traversal and absolute-path entries independent of dependency sanitization; the Archive Metadata Policy adopted. `EvidenceCollection.md` amended.
- **Phase 3A** — `ArchiveReader`/`ArchiveEntry`/`ArchiveReadError`: deterministic, sorted structural enumeration of a ZIP archive's entries. `zip` v8.6.0 added to `modiq-collection`, the platform's first archive-parsing dependency.
- **Phase 3B** — `ArchiveEvidenceBuilder`: transforms `ArchiveEntry` values into real `Evidence`, reusing `EvidenceCategory::FileStructureAnalysis` unchanged.
- **Phase 3C** — `ArchiveCollector`: assembled Phases 3A/3B with GOV-011's three remaining policies into one real Collector. Resource limits and the Archive Traversal Boundary Policy enforced. The Duplicate Archive Entry Policy represented via a new closed-set category, `EvidenceCategory::StructuralDuplication` (`modiq-runtime`), following a dedicated Architecture Review (`PROPOSAL_GOV-011_DUPLICATE_REPRESENTATION.md`) and Technical Director approval, named for the observation's semantic class rather than the collection mechanism. Duplicate detection performs sequential local-file-header inspection, since the dependency's central directory is name-indexed and cannot represent a collision.
- **Phase 3D** — `ArchiveCollector` wired into `AssessmentService::execute_from_assessment_input` via one explicit, inline, case-insensitive `.zip`-suffix routing check; no dispatcher, registry, trait, or plugin mechanism. The filesystem `EvidenceCollector` path is unchanged. Completed the platform's first end-to-end archive assessment path.
- **Closeout** — a checked-in archive fixture (`apps/sandbox/src-tauri/fixtures/sample-archive-input.zip`) and dedicated Sandbox-workspace tests exercise `AssessmentService::execute_from_assessment_input`'s archive-routing path through the exact production entry point `create_assessment` uses, alongside a regression guard confirming the pre-existing filesystem fixture path is unaffected. No new `#[tauri::command]` or UI surface added.
- The root workspace test suite grew from 112 to 150 tests across the sprint (`modiq-collection` 12→43, `modiq-engine` 9→16, plus `modiq-runtime`'s `EvidenceCategory::StructuralDuplication` coverage); the Sandbox's own separate suite grew from 3 to 6.

## Deferred (Governance-Pending)

- GOV-001, GOV-002, GOV-003, and GOV-008 remain open; none were addressed this Sprint.
- Numeric resource-limit thresholds (50,000 entries; 10,000:1 compression ratio) remain provisional, chosen with headroom above Phase 2's measured baseline rather than calibrated against production data.
- Nested archive traversal (an archive within an archive) and any archive format other than ZIP remain out of scope, as originally planned.

## Released

- Documented in `docs/engineering/ENGINEERING_RELEASE_0.4.md` (release record, including its own retrospective section — no separate `SPRINT4_RETROSPECTIVE.md` file was ever created; a prior reference to one in this entry was a documentation error, found and corrected during Sprint 5 Closeout), and the `PROPOSAL_ZIP_EVIDENCE_COLLECTION.md` / `PROPOSAL_GOV-011.md` / `PROPOSAL_GOV-011_DUPLICATE_REPRESENTATION.md` proposals that preceded and shaped implementation.

---

# [Sprint 5]

**Status:** Complete (Phases 1–5, plus Closeout)

## Added

- **Phase 1 (Design Preparation)** — `SPRINT5_IMPLEMENTATION_PLAN.md` drafted, naming five Design Questions and one candidate governance item rather than presuming answers. GOV-012 (Rule Evaluation Model) approved by Technical Director in full: `RuleEngine::evaluate` returns `Vec<RuleOutcome>`; Rules dispatch in fixed, explicit declaration order; Rules compose independently, no suppression model. `FindingSeverity` semantic definitions drafted — the first time this project defined what `Error`/`Warning`/`Informational`/`BestPractice` actually mean. Drafting those definitions surfaced a real architectural tension (`BestPractice` classifies Finding *kind*, not severity), recorded as GOV-013, deliberately Open — provisionally accepted, not resolved, revisited once more concrete Rules exist. Both governance items formally inserted into `GOVERNANCE.md`; the `FindingSeverity` definitions inserted into `DataModel.md` (amended to v1.1.0).
- **Phase 2 (Second Real Rule)** — `StructuralDuplicationRule` implemented (`modiq-rules`), evaluating `EvidenceCategory::StructuralDuplication` Evidence and assigning `FindingSeverity::Warning` per the newly-recorded definitions.
- **Phase 3 (Multi-Rule Evaluation Assembly)** — the original Sprint 1 Rule extracted into its own unit, `EvidencePresenceRule`. `RuleEngine::evaluate` rewritten as a two-Rule dispatcher per GOV-012's resolved shape/ordering/composition. `modiq-engine`'s `AssessmentService::execute` updated internally to loop over the new return shape; its public signature is unchanged. No trait, registry, or dispatch abstraction introduced.
- **Phase 4 (Reporting Scaffold Investigation)** — investigated whether `modiq-report`'s four unused scaffold types (`FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, `ReportFormatter`) should be built out or retired, specifically checking whether Sprint 5's own new severity differentiation created a real need (it did not). Recommended retirement, using the same evidentiary method GOV-004 used. No `modiq-report` code changed.
- **Phase 5 (Testing & Verification)** — closed a real determinism-testing gap: confirmed `RuleEngine::evaluate`'s outcome order is independent of Evidence arrival order, not just repeated-identical-input order.
- The root workspace test suite grew from 150 to 162 tests across the sprint (`modiq-rules` 3 → 15); the Sandbox's own separate suite was unaffected (unchanged at 6), since no Sandbox fixture produces `StructuralDuplication` Evidence.

## Deferred (Governance-Pending)

- GOV-001, GOV-002, GOV-003, and GOV-008 remain open; none were addressed this Sprint.
- GOV-013 (FindingSeverity Severity/Kind Conflation) remains deliberately Open — provisionally accepted, to be revisited once the Rule Engine has multiple concrete Rules operating in practice with real evidence bearing on the question, not decided from two Rules alone.
- Retiring (or building out) `modiq-report`'s four scaffold types — recommended by Phase 4's investigation, eligible pending formal governance approval, but not acted on this Sprint.
- XML inspection (the next Evidence Collector) and CLI wiring both remain out of scope, as originally planned — Sprint 5's own charter was to strengthen the assessment model before widening Evidence Collection further.

## Released

- Documented in `docs/engineering/ENGINEERING_RELEASE_0.5.md` (release record, including its own retrospective section), `docs/engineering/SPRINT5_IMPLEMENTATION_PLAN.md`, `docs/engineering/GOV-012_AND_FINDINGSEVERITY_PREPARATION.md`, and `docs/engineering/SPRINT5_PHASE4_REPORTING_INVESTIGATION.md`.

---

# [Sprint 6]

**Status:** Complete (implementation, review, and merge into `feature/runtime-implementation`)

## Added

- **CLI wiring** — `modiq-cli` wired to `modiq-engine` for the first time since Sprint 0. `Application` dispatches `assess`/`help`/`version` by one direct match, no command trait or registry. `AssessCommand` calls `AssessmentService::execute_from_assessment_input` against a real, user-supplied path (not a fixed fixture, unlike the Sandbox's own use of the same entry point), formatting Evidence/Findings/Recommendations for display. Exit-code convention: 0 success, 1 execution failure (`CollectionError` — a well-formed input, execution attempted and aborted), 2 invalid usage (CLI-level usage errors and `AssessmentInputError` alike — invalid before execution begins). No new external dependency; argument parsing is manual, per explicit Chief Architect direction.
- **`modiq-report` scaffold retirement** — the four unused types recommended for retirement at Sprint 5 Phase 4 (`FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, `ReportFormatter`) deleted under this Sprint's explicit, separate authorization. `AssessmentReport`, the crate's real, tested content, is unchanged and remains the canonical report model.
- The root workspace test suite grew from 162 to 172 tests across the sprint (`modiq-cli` 0 → 10; every other crate's count unchanged, including `modiq-report`'s 3, confirming the deletion had zero test-coverage impact); the Sandbox's own separate suite was reverified unchanged at 6/6.

## Deferred (Governance-Pending)

- GOV-001, GOV-002, GOV-003, GOV-008, and GOV-013 remain open; none were addressed this Sprint. GOV-008 specifically was not advanced: CLI wiring reuses `AssessmentService`'s existing entry point exactly as designed and was not expected to, and did not, generate new evidence toward it.
- `Display`/`Serialize` for Runtime identity/enum types remains explicitly out of scope, per direct Chief Architect authorization — `modiq-cli` formats results with `{:?}`, matching the Sandbox's own established approach.
- XML inspection (the next Evidence Collector) remains out of scope — the sole remaining candidate from the original three-item Sprint 6 roadmap, deferred to a future Sprint of its own rather than bundled into this one.
- A minor, twice-observed architectural gap was named, not acted on: `modiq-engine` does not re-export `AssessmentReport`, so both real consumers of `AssessmentService` (the Sandbox and now `modiq-cli`) independently needed a direct `modiq-report` dependency just to name the type. Two data points, below this project's own usual three-point convergent-evidence bar — tracked, not yet a Governance Register item.
- ~~A formal `ENGINEERING_RELEASE_0.6.md` record, matching every prior Sprint's own convention, has not yet been produced.~~ **Resolved, retroactively:** `ENGINEERING_RELEASE_0.6.md` was produced after Sprint 7's own close, alongside `_0.7.md` — see `HISTORICAL_RELEASE_COMPLETION_REPORT.md`.

## Released

- Documented in `docs/engineering/SPRINT6_IMPLEMENTATION_PLAN.md` (including its Authorization Record), `docs/engineering/POST_SPRINT6_REPOSITORY_ASSESSMENT.md`, and `docs/engineering/ENGINEERING_RELEASE_0.6.md` (produced retroactively after Sprint 7's own close). Implementation committed as `397707f` on `feature/sprint6-cli`, merged into `feature/runtime-implementation` as `29657df`.

---

# [Sprint 7]

**Status:** Complete (implementation, review, and Repository Closeout on `feature/runtime-implementation` — no separate Sprint branch this cycle)

## Added

- **Multi-Source Evidence Collection: XML inspection** — `XmlCollector` runs independently alongside the existing structural Collector (`EvidenceCollector`/`ArchiveCollector`) for every Assessment, composed inline in `AssessmentService::execute_from_assessment_input`. Locates `modDesc.xml` at an Assessment Input's root (directory, bare file, or archive root) and produces `XmlInspection` Evidence: well-formedness, declared `<dependency>` element extraction, or absence — a missing manifest is itself recorded as Evidence, never silently Empty Collection. No Rule Engine change; `modiq-rules` untouched. New dependency: `roxmltree` (read-only, no DTD/entity support — a deliberate safety property for untrusted, community-submitted content).
- Preceded by a dedicated Architecture Evaluation (`COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`) once Sprint 7's own capability-first planning surfaced the platform's first need for more than one Collector per Assessment. Approved architecture: Collectors remain fully independent, never consume each other's output; composition is direct and inline; no `CollectionCoordinator` introduced, with a five-condition threshold recorded for when that should be revisited.
- The root workspace test suite grew from 172 to 187 tests (`modiq-collection` +13, `modiq-engine` +2); the Sandbox's own separate suite grew from 6 to 7.

## Engineering Workflow Consolidation

- Nine independent, drifted workflow descriptions found across the repository (`ENGINEERING_WORKFLOW_CONSOLIDATION_STUDY.md`) reduced to one canonical source: `PROJECT_HANDOFF_v1.0.md`, Section 5. Terminology unified project-wide (Capability Definition, Architecture Evaluation, Architectural Resolution, Implementation Authorization, Implementation, Validation, Implementation Report, Architectural Conformance Review, Commit, Merge, Repository Closeout) — resolving, among other drift, an ambiguous pair ("Chief Architect Review"/"Architecture Review") previously used for two different stages.
- `docs/implementation/ImplementationWorkflow.md` archived as non-normative history rather than deleted. Implementation Report template reconciled with practice actually demonstrated in Sprints 6 and 7, six sections added, none removed.
- **The engineering methodology is now designated Version 1.0** — exercised across multiple completed Sprints, consolidated into one canonical process, expected to evolve only through implementation evidence going forward, not routine amendment.

## Deferred (Governance-Pending)

- GOV-001, GOV-002, GOV-003, GOV-008, and GOV-013 remain open; none were addressed this Sprint.
- Dependency extraction (`XmlCollector`'s `<dependency>` element interpretation) is implemented per a documented interpretation of `modDesc.xml` convention, not yet validated against a real Farming Simulator mod manifest — no such sample exists in this repository.
- The Collector Composition Governance item remains deferred, per explicit Chief Architect direction: Sprint 7 is the first implementation evidence for the approved architecture, not the final governance evidence: it should wait for at least one more independent content Collector.
- Two smaller items from the workflow consolidation remain open: whether `ImplementationWorkflow.md` should be deleted outright rather than archived, and whether `EngineeringGuide.md`'s remaining "Technical Director" reference and non-workflow content should be reconciled or relocated.
- ~~Formal `ENGINEERING_RELEASE_0.6.md` and `_0.7.md` records, matching every earlier Sprint's own convention, have not yet been produced.~~ **Resolved, retroactively:** both produced after Sprint 7's own close — see `HISTORICAL_RELEASE_COMPLETION_REPORT.md`. The gap itself (two Sprints before either record existed) is named in both new records' own Lessons Learned as a process risk not to repeat.

## Released

- Documented in `docs/engineering/SPRINT7_CAPABILITY_AND_IMPLEMENTATION_PLAN.md`, `docs/engineering/COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`, `docs/engineering/SPRINT7_IMPLEMENTATION_AUTHORIZATION.md`, `docs/engineering/SPRINT7_IMPLEMENTATION_REPORT.md`, `docs/engineering/ENGINEERING_WORKFLOW_CONSOLIDATION_STUDY.md`, `docs/engineering/ENGINEERING_WORKFLOW_CONSOLIDATION_REPORT.md`, and `docs/engineering/ENGINEERING_RELEASE_0.7.md` (produced retroactively, alongside `_0.6.md`, after this Sprint's own close). Implementation, refinement, and consolidation committed together as `277aefd` directly on `feature/runtime-implementation` and pushed — no separate Sprint branch existed this cycle, unlike Sprint 6.

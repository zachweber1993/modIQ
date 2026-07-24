# CHANGELOG

| Property | Value |
|----------|-------|
| **Document** | CHANGELOG.md |
| **Project** | modIQ |
| **Purpose** | Repository History |
| **Maintained By** | Project Maintainers |
| **Last Updated** | 2026-07-23 |

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

---

# [Sprint 8]

**Status:** Complete (implementation, review, and Repository Closeout on `feature/runtime-implementation` — no separate Sprint branch this cycle, matching Sprint 7's own precedent)

## Added

- **Version Profile-aware compatibility checking** — `modiq-versioning` gained its first real content since Sprint 0: a minimal `GameVersion`/`VersionProfile` pair, with a single hardcoded `VersionProfile::fs25()` recognizing `descVersion` 93. `XmlCollector` (Sprint 7) extended to extract a mod's declared `descVersion` as a purely factual `XmlInspection` Evidence item — no interpretation, preserving the Collector Contract's observational boundary exactly. A new Rule, `VersionCompatibilityRule`, evaluates that declared value against the active Version Profile inside the Rule Engine, producing a `Warning` Finding when unrecognized; `RuleEngine::evaluate` gained a `VersionProfile` parameter, dispatching the new Rule third, after `StructuralDuplicationRule` (GOV-012's fixed declaration order, extended, never reordered). `Assessment` records which Version Profile governed it through a new opaque `VersionProfileReference`, extending ADR-0007's Opaque Runtime References pattern (`RuleReference`, `RepairRecipeReference`) to a domain relationship for the first time; `modiq-runtime` gained no new dependency and remains the platform's sole dependency-free leaf, unbroken since Sprint 0. `AssessmentService`'s two public entry points (`execute`, `execute_from_assessment_input`) required zero signature change — both execute every Assessment against `VersionProfile::fs25()` internally, an implementation simplification accepted in place of the originally anticipated new additive entry point, since no second Version Profile yet exists for a caller to select between.
- Preceded by the full Capability Definition → Architecture Evaluation → Architectural Resolution → Implementation Authorization sequence, each producing its own reviewed document: `SPRINT8_INITIALIZATION_REPORT.md`, `SPRINT8_CAPABILITY_AND_IMPLEMENTATION_PLAN.md`, `SPRINT8_ARCHITECTURAL_RESOLUTION.md`, `SPRINT8_IMPLEMENTATION_AUTHORIZATION.md`. Six architectural decisions (Version Profile ownership, extraction location, version-aware evaluation location, Assessment construction, crate dependencies, governance timing) were evaluated with alternatives and a recommendation each, then explicitly decided by the Chief Architect before implementation began — zero implementation-before-decision, consistent with this project's unbroken discipline.
- The root workspace test suite grew from 187 to 205 tests (`modiq-versioning` +4, its first tests ever; `modiq-rules` +10; `modiq-runtime` +2; `modiq-collection` +1; `modiq-engine` +1); the Sandbox's own separate suite was unaffected (unchanged at 7/7, requiring zero source modification — neither entry point it calls changed signature).
- Exactly two new internal dependency edges (`modiq-engine` → `modiq-versioning`, `modiq-rules` → `modiq-versioning`), both directly justified; no new external dependency; `modiq-versioning` → `modiq-common` deliberately not added.

## Deferred (Governance-Pending)

- GOV-001, GOV-002, GOV-003, GOV-008, and GOV-013 remain open; none were addressed this Sprint. GOV-008 specifically was not advanced: both `AssessmentService` entry points required no signature change, generating no new evidence toward it — the same non-outcome Sprint 6 and 7 each already produced.
- No new Governance Register item was opened for the Version Profile Ownership, Version-aware Evaluation, or Assessment Construction decisions, by explicit Chief Architect decision — mirroring Sprint 7's own treatment of the Collector Composition Model (implementation evidence to precede formal governance codification, not the reverse).
- The `modiq-versioning` Crate Boundary Rules gap in `GOVERNANCE.md`, named during Sprint 8 planning, remains open, deliberately.
- Exhaustive Farming Simulator version knowledge, a profile-selection mechanism, Knowledge Domain integration, and general-purpose Rule Selection filtering all remain out of scope, as originally planned — Sprint 8's own charter was to establish the architecture, not complete the ecosystem.

## Released

- Documented in `docs/engineering/SPRINT8_INITIALIZATION_REPORT.md`, `docs/engineering/SPRINT8_CAPABILITY_AND_IMPLEMENTATION_PLAN.md`, `docs/engineering/SPRINT8_ARCHITECTURAL_RESOLUTION.md`, `docs/engineering/SPRINT8_IMPLEMENTATION_AUTHORIZATION.md`, `docs/engineering/SPRINT8_IMPLEMENTATION_REPORT.md`, `docs/engineering/SPRINT8_IMPLEMENTATION_DEVIATIONS.md`, and `docs/engineering/ENGINEERING_RELEASE_0.8.md` (produced at this Sprint's own Closeout, not retroactively — correcting the two-Sprint-running pattern Engineering Release 0.7 itself named as a risk not to repeat a third time).

---

# [Sprint 9]

**Status:** Complete (Capability Definition, Architectural Resolution, Implementation, Repository Review, Closeout, on `feature/runtime-implementation` — no separate Sprint branch this cycle, matching Sprint 7's and Sprint 8's own precedent)

## Added

- **Repair Guidance (minimum viable `modiq-knowledge` activation)** — `RepairRecipe` gained real content for the first time since Sprint 0: `identifier`/`guidance` fields, an infallible constructor, and one named, authored recipe, `RepairRecipe::version_compatibility_declared_version_mismatch()`, mirroring `VersionProfile::fs25()`'s exact precedent (a specific minimum-viable value authored inside the domain crate itself, not inlined by its caller). `VersionCompatibilityRule` now calls this recipe rather than authoring guidance text inline, wiring a real `Some(RepairRecipeReference)` into its `Recommendation` in place of `None`. `RepairRecipeReference` (real and tested since Sprint 2, always `None` in practice until now) required no modification — its third proven instance of ADR-0007's Opaque Runtime References pattern. `RuleEngine::evaluate` and `AssessmentService`'s two public entry points required zero signature change; the capability's entire footprint is one new dependency edge, `modiq-rules` → `modiq-knowledge`, mirroring Sprint 8's own `modiq-rules` → `modiq-versioning` edge.
- Preceded by the full Capability Definition → Architectural Resolution → Implementation → Repository Review sequence, each producing its own reviewed document: `SPRINT9_CAPABILITY_DEFINITION.md`, `SPRINT9_ARCHITECTURAL_RESOLUTION.md`, `SPRINT9_REPOSITORY_REVIEW.md`. During Architectural Resolution review, the Chief Architect identified a conflation in the initial draft's Question 2 — where a `RepairRecipe` is retrieved (correctly resolved: no new `RuleEngine::evaluate` parameter) versus where its content is authored (initially resolved incorrectly: inline inside the consuming Rule). The resolution was revised before implementation began so that `modiq-knowledge`, not `modiq-rules`, authors engineering knowledge, consistent with `modiq-knowledge`'s own README boundary.
- The root workspace test suite grew from 205 to 210 tests (`modiq-knowledge` 0 → 5, its first tests ever; `modiq-rules` and `modiq-engine` unchanged in count, both extended in place — a stale `None` assertion updated to the real `Some(...)`, and an end-to-end pipeline assertion added); the Sandbox's own separate suite was unaffected (unchanged at 7/7, requiring zero source modification).

## Deferred (Governance-Pending)

- GOV-001, GOV-002, GOV-003, GOV-008, and GOV-013 remain open; none were addressed this Sprint. GOV-008 specifically was not advanced: `AssessmentService`'s two public entry points required no signature change this Sprint either, generating no new evidence toward it.
- No new Governance Register item or ADR was opened — the Knowledge Domain boundary section in `GOVERNANCE.md` already named Repair Recipes explicitly before this Sprint began, so (unlike Sprint 8's `modiq-versioning` gap) no amendment was needed.
- `modiq-knowledge`'s remaining six Knowledge Model categories (`Rule`, `Engine Behavior`, `Compatibility Pattern`, `Best Practice`, `Known Issue`, `Knowledge Reference`) remain unimplemented, as originally scoped — this Sprint's charter was one minimum-viable `RepairRecipe`, not the full Knowledge Model.
- `StructuralDuplicationRule` remains without its own Repair Recipe — `VersionCompatibilityRule` was the sole Sprint 9 consumer, by explicit scope.
- The `modiq-versioning` Crate Boundary Rules gap named during Sprint 8 planning remains open, unaffected by Sprint 9.

## Released

- Documented in `docs/engineering/SPRINT9_CAPABILITY_DEFINITION.md`, `docs/engineering/SPRINT9_ARCHITECTURAL_RESOLUTION.md`, `docs/engineering/SPRINT9_REPOSITORY_REVIEW.md`, and `docs/engineering/ENGINEERING_RELEASE_0.9.md` (produced at this Sprint's own Closeout, not retroactively).

---

# [Sprint 10]

**Status:** Complete (Capability Definition and Runtime Fixture Corpus Acquisition only — Runtime Log Interpretation's own implementation intentionally deferred; no separate Sprint branch this cycle)

## Added

- **Runtime Fixture Corpus** — a new, permanent, top-level `fixtures/runtime-logs/` corpus: real, provenance-tracked Farming Simulator runtime log fixtures, structurally and documentation-wise separate from `apps/sandbox/src-tauri/fixtures/`'s own unrelated synthetic-fixture convention. A consistent per-fixture metadata schema (`TEMPLATE.md`) and all three of the corpus's initial fixtures were captured, normalized, and integrated: `clean-base-game` (a verified mod-free baseline), `single-compatible-mod` (a successful third-party mod load, the negative control), and `single-incompatible-mod` (a real, fully evidenced failure — a mod's declared `descVersion` exceeding the runtime's own recognized version, rejected at modDesc validation, before registration or asset loading ever began). No mod archive was stored in the repository at any point — only runtime logs and independently verified factual metadata about the mods that produced them.
- **Runtime Log Normalization** policy — every fixture must have personally identifying or machine-specific information (e.g., a username embedded in a filesystem path) replaced with a fixed, standardized placeholder before entering the corpus, verified deterministic (the same raw source, normalized the same documented way, always produces the same fixture byte for byte), and strictly substitutive (never altering parser-relevant content, runtime semantics, ordering, line counts, or formatting beyond the exact documented substitution).
- **Installation State versus Savegame State** — a real acquisition finding (a runtime log enumerates the global mods directory regardless of which savegame is active; a new savegame does not clear it) split `TEMPLATE.md`'s schema into two independent fields, `Installed Mods (global)` and `Savegame Mod State`, closing a conflation the corpus's own first fixture (`clean-base-game`) was originally, and incorrectly, captured against.
- **Warning Categorization** policy — every warning a fixture's log contains is classified as a `Base-game warning` (verified by direct cross-reference against `clean-base-game`'s own mod-free content), a `Fixture warning` (attributable to the fixture's own mod, but not a failure), or a `Fixture-affecting warning` (calls the fixture's own validity into question) — attribution-based categories, deliberately, not a severity scale.
- Preceded by Capability Definition (`SPRINT10_CAPABILITY_DEFINITION.md`), which scoped the capability to recognizing one class of signal (a mod failed to load) and established, as an engineering requirement rather than an observation, that no architectural decision may assume a real log's structure, wording, stability, or formatting — acquiring and validating representative logs was named as the first engineering activity that requirement demands, and this Sprint completed exactly that activity, recorded in full in `SPRINT10_RUNTIME_LOG_FIXTURE_PREPARATION.md`.
- The root workspace test suite is **unchanged at 210/210**; Sandbox unchanged at 7/7. No Rust source file was modified this Sprint.

## Deferred (Governance-Pending)

- GOV-001, GOV-002, GOV-003, GOV-008, and GOV-013 remain open; none were addressed this Sprint — this Sprint touched no Rust source at all, generating no evidence toward any of them either way.
- No new Governance Register item or ADR was opened. Installation State vs. Savegame State and Warning Categorization are fixture-corpus documentation policies, not Governance Register items — neither touches a crate boundary, a public API, or an architectural principle.
- The `modiq-versioning` Crate Boundary Rules gap named during Sprint 8 planning remains open, unaffected by Sprint 10.
- **Runtime Log Interpretation's own implementation — a Collector activating `EvidenceCategory::RuntimeLogs`, and a Rule interpreting it — remains intentionally deferred.** This Sprint's own charter was acquiring and validating real evidence, not building against it; Architectural Resolution and implementation are explicitly future work, not begun here.
- A fourth fixture (`modded-map-only`, testing whether the recognized signal generalizes across Assessment Subject content types) and a fifth (`real-world-mod-profile`) both remain named, deferred candidates for a future corpus expansion — not built this Sprint, per the same minimum-viable-first discipline applied throughout.

## Released

- Documented in `docs/engineering/SPRINT10_CAPABILITY_DEFINITION.md`, `docs/engineering/SPRINT10_RUNTIME_LOG_FIXTURE_PREPARATION.md`, and `docs/engineering/ENGINEERING_RELEASE_1.0.md` (produced at this Sprint's own Closeout, not retroactively).

---

# [Sprint 11]

**Status:** Complete (Architectural Resolution, Implementation, Architectural Reconciliation, Repository Closeout, on `feature/runtime-implementation` — no separate Sprint branch this cycle, matching Sprints 7–10's own precedent)

## Added

- **Runtime Evidence Processing Architecture** — `RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md` resolved, grounded in Sprint 10's three real fixtures, the four questions `SPRINT11.md`'s Scope named: where a runtime observation enters the pipeline, `EvidenceCategory::RuntimeLogs`'s content shape, how the evidence is interpreted (including `FindingSeverity`), and fixture-corpus sufficiency. Included the Runtime Interpretation Decision Matrix (four rows, each citing real fixture evidence or an explicit generalization boundary) as a first-class deliverable, and an Architectural Invariants section restating Collector/Rule boundaries specifically for runtime evidence.
- **`RuntimeLogCollector`** (`modiq-collection`) — locates `log.txt` at the Assessment Input's root (filesystem or archive, mirroring `XmlCollector`'s identical symmetry, justified by the already-approved Assessment Input model rather than fixture evidence specific to this capability), recognizes the one documented failure template (generalized over the mod name, not the message text), and produces `RuntimeLogs` Evidence only for a recognized match. A missing or unrecognized log is Legitimate Absence, never a recorded fact — a deliberate divergence from `XmlCollector`'s own missing-manifest-as-Evidence precedent, reasoned from the two content types' different expected-presence baselines. Composed inline into `AssessmentService::execute_from_assessment_input`, appended after `XmlCollector` — the second content-Collector under the Collector Composition Architecture's own extraction axis, not yet crossing its five-condition threshold for a dedicated coordinator.
- **`RuntimeLoadFailureRule`** (`modiq-rules`) — filters `RuntimeLogs` Evidence, independently re-matching the same recognized template as a defense-in-depth safeguard against the Collector's own boundary, and assigns `FindingSeverity::Error` — the platform's first real use of that variant, reasoned directly from `DataModel.md`'s own Finding Severity definitions (a direct, conclusive observation of an actual failed load attempt, unlike `VersionCompatibilityRule`'s own static-declaration basis for `Warning`) rather than deferred to the game engine's own log wording. Dispatched fourth in `RuleEngine::evaluate`'s fixed declaration order (GOV-012), appended after `VersionCompatibilityRule`, never reordering the existing three. Recommendation is inline-authored with `repair_recipe_reference: None`, mirroring `VersionCompatibilityRule`'s own pre-Sprint-9 shape — no Knowledge Domain involvement.
- Neither `AssessmentService`'s two public entry points nor `RuleEngine::evaluate`'s parameter shape required any change.
- The root workspace test suite grew from 210 to 238 tests across the sprint (`modiq-collection` 57 → 70; `modiq-rules` 25 → 36; `modiq-engine` 19 → 23 unit); the Sandbox's own separate suite was unaffected (unchanged at 7/7).

## Architectural Reconciliation

- A dedicated, adversarial engineering verification pass — conducted specifically to attempt to disprove architectural consistency rather than confirm it, after implementation but before repository closeout — found a genuine internal contradiction: the architecture document's own Architectural Invariants section (v1.1.0) asserted that every unrecognized runtime observation remains Evidence until a Rule declines to interpret it, while `RuntimeLogCollector`, built against an earlier section of the same document, performs recognition *before* Evidence is created, so an unrecognized log line never becomes Evidence at all.
- Per this project's standing discipline, implementation was halted and the contradiction reported rather than resolved unilaterally in either direction. Chief Architect review determined the implementation was correct and the invariant's wording was the inconsistency; `RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md` was revised to v1.2.0 to describe the Collector-recognizes-then-Evidence-exists model the implementation already realized, adding an explicit statement of the Collector's own deterministic recognition contract, a forward-looking note on how that contract may be extended by future, separately approved, fixture-grounded work, and a resolution of a secondary observation (archive-location support, justified by the already-approved Assessment Input model, not by fixture evidence specific to this capability).
- **No Rust source, test, fixture, ADR, or Governance Register item was touched in this reconciliation** — only the architecture document's own wording.

## Deferred (Governance-Pending)

- GOV-001, GOV-002, GOV-003, GOV-008, and GOV-013 remain open; none were addressed this Sprint. GOV-013 specifically was documented as newly relevant (this Sprint's own `FindingSeverity::Error` assignment is offered as evidence for a future GOV-013 review) but deliberately not reopened or decided.
- No new Governance Register item or ADR was opened — Rule dispatch extension is already covered by GOV-012's own general resolution; Collector composition extension is already covered by the Sprint 7 Collector Composition Architecture's own extraction threshold, not crossed by a second content-Collector.
- Broader runtime log signature coverage (a second failure class), a non-macOS fixture capture, and any Knowledge Domain pairing (a Repair Recipe) for `RuntimeLoadFailureRule`'s own Finding all remain out of scope, as originally planned — named explicitly so they are not later rediscovered as surprises.
- The `modiq-versioning` Crate Boundary Rules gap named during Sprint 8 planning remains open, unaffected by Sprint 11.

## Released

- Documented in `docs/implementation/SPRINT11.md`, `docs/engineering/RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md` (v1.2.0), and `docs/engineering/ENGINEERING_RELEASE_1.1.md` (produced at this Sprint's own Closeout, not retroactively).

---

# [Sprint 12]

**Status:** Complete (Architectural Resolution, Adversarial Verification, Reconciliation, Repository Closeout, on `feature/runtime-implementation` — no separate Sprint branch this cycle, matching Sprints 7–11's own precedent; architecture-only, no Rust source touched at any point)

## Added

- **Capability Identity procedure** — an explicit architectural decision procedure, derived from seven completed engineering decisions (Sprint 3, Sprint 4, Sprint 4 Phase 3C, Sprint 7, Sprint 8, Sprint 9, Sprint 11), distinguishing Capability Enrichment, Capability Expansion, and Capability Introduction across three independent axes: the **Collection Axis** (does producing the fact need a new inspection mechanism), the **Evidence Axis** (does it represent a kind of observation no existing `EvidenceCategory` covers), and the **Interpretation Axis** (does interpreting it need a judgment no existing Rule already makes) — plus one orthogonal check reserving "Capability Introduction" for capabilities requiring genuinely new composition or dispatch machinery, independent of how novel any single axis is.
- **Collector Guidance** — mutually-exclusive routing versus additive composition, discriminated by whether two Collectors' own applicability conditions can co-occur on the same Assessment Input, recorded as guidance independent of, and not a replacement for, the Collector Composition Architecture's own extraction threshold (unchanged).
- **Rule Guidance** — the Interpretation Axis's judgment test, now confirmed by four independent historical instances (Sprint 4 Phase 3C, Sprint 8, Sprint 9, Sprint 11), with an explicit, unchanged disclosure that no historical instance yet tests two Rules sharing one already-interpreted `EvidenceCategory`.
- Neither `AssessmentService`'s two public entry points, `RuleEngine::evaluate`'s parameter shape, any Collector, any Rule, any `EvidenceCategory` variant, nor any test was touched — this Sprint produced documentation only.

## Architectural Contradiction and Reconciliation

- This Sprint's own first draft of the Capability Identity procedure (`SPRINT12.md` v1.0.0) was found, during its own adversarial verification pass, to contain a genuine contradiction: it conflated `EvidenceCategory` novelty with Collector novelty into a single cascading test, implicitly assuming a new kind of fact always implies a new Collector.
- **Sprint 4 Phase 3C** (`EvidenceCategory::StructuralDuplication`, GOV-011) falsifies this directly: a genuinely new category, produced by *extending* the already-existing `ArchiveCollector`, not by adding a new one.
- Per this project's standing discipline, the contradiction was reported rather than resolved unilaterally. On Chief Architect confirmation, the procedure was corrected to three independent axes (above), and `SPRINT12.md` was amended to v1.1.0 — the original model preserved in a Reconciliation Record, not deleted, with the full evidence and reasoning recorded in the new `docs/engineering/SPRINT12_ARCHITECTURAL_RESOLUTION.md`.
- **Direct consequence: Sprint 11 is reclassified from Capability Introduction to Capability Expansion**, because it reused Sprint 7's own additive-composition machinery and GOV-012's own fixed-order dispatch model, rather than inventing new machinery. This reclassification is architectural only — Sprint 11 remains the platform's first event-based Evidence source and first use of `FindingSeverity::Error`, and this document records both facts side by side rather than letting one imply the other has diminished.
- **No Rust source, test, fixture, ADR, or Governance Register item was touched in this reconciliation** — only the planning document's own wording, and a new Resolution document recording the full history.

## Deferred (Governance-Pending)

- GOV-001, GOV-002, GOV-003, GOV-008, and GOV-013 remain open; none were addressed this Sprint.
- No new Governance Register item or ADR was opened — this Sprint's own conclusions apply and extend already-approved architecture (the Collector Composition Architecture, GOV-012) rather than establishing a new durable principle requiring one.
- The untested "second Rule, same already-interpreted category" case remains unresolved by direct precedent, disclosed explicitly, left for whichever future capability first encounters it.
- The Introduction test's own single confirmed non-degenerate instance (Sprint 7) remains a real but thin evidentiary base — named explicitly as a limitation, not overstated.
- Any second runtime-log signature, Lua analysis, asset validation, dependency resolution, or performance observations all remain unscoped, each awaiting its own future Capability Proposal informed by this Sprint's procedure.

## Released

- Documented in `docs/implementation/SPRINT12.md` (v1.1.0), `docs/engineering/SPRINT12_ARCHITECTURAL_RESOLUTION.md`, and `docs/engineering/ENGINEERING_RELEASE_1.2.md` (produced at this Sprint's own Closeout, not retroactively).

---

# [Sprint 13]

**Status:** Complete (Investigation, Governance Reconciliation, Architecture Evaluation, Architectural Resolution, Implementation Authorization, Sprint Planning, three-phase Implementation, Repository Closeout, on `feature/runtime-implementation` — no separate Sprint branch this cycle, matching Sprints 7–12's own precedent)

## Added

- **`modiq-storage`** — a new workspace crate, giving the Storage subsystem (named in `Architecture.md`'s System Overview since the document's own baseline) its first real content. `PersistedAssessmentReport` and its nested `Persisted*` types are Storage's own representation of a report's content, built from `AssessmentReport`'s already-public getters only — never a reconstructed `AssessmentReport`, and never any Runtime entity's own process-local identity, which cannot be reconstructed to a specific value and is not meaningful across a process boundary in the first place (`AssessmentId`'s own generator restarts at 1 on every process invocation). Finding→Evidence and Recommendation→Finding cross-references are preserved as positions within the persisted report itself. `ReportKey` is an opaque identifier Storage mints itself at write time. `ReportStore` provides real, filesystem-backed write and read, using only `std` and the already-declared workspace `serde`/`serde_json` — no new external dependency.
- **`modiq-cli` integration** — `AssessCommand` hands a successful assessment's report to `ReportStore::store`, reporting the resulting key (`Stored as: <key>`); a storage failure is reported as a warning, never as a change to the assessment's own exit code. A new `retrieve <key>` command reads a previously-stored report back, independent of running a new Assessment.
- **`apps/sandbox` integration** — `create_assessment` gains a `stored_report_key` field on its existing IPC summary DTO; a new `retrieve_report` Tauri command mirrors `modiq-cli`'s own `retrieve`, returning a new `PersistedReportSummary` IPC DTO built from `PersistedAssessmentReport`.
- Both integration phases were verified with a genuine cross-process round trip against the real, built `modiq-cli` binary (run as two separate process invocations), not only in-test coverage.
- The root workspace test suite grew from 238 to 253 tests (`modiq-storage` 0 → 10, its first tests ever; `modiq-cli` 10 → 15); Sandbox grew from 7 to 9.
- `modiq-runtime`, `modiq-report`, `modiq-engine`, `modiq-rules`, `modiq-versioning`, and `modiq-knowledge` are unmodified. `AssessmentService`'s two public entry points required zero signature change.
- `.gitignore` gained `.modiq-storage/`, since both `modiq-cli`'s and the Sandbox's own default storage locations can now write real files into the working tree during normal use.

## Investigation and Governance Reconciliation

- **`INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md`** established Storage as a valid, well-evidenced capability candidate, then found the Sprint 12 Capability Identity procedure could not classify it — its three axes and Introduction test are scoped to Collector/Rule-shaped candidates, and a subsystem-level candidate is a different shape of question entirely.
- **`GOVERNANCE_OBSERVATION_SUBSYSTEM_ACTIVATION.md`** generalized this finding, then — on Chief Architect review surfacing that `SPRINT8_ARCHITECTURAL_RESOLUTION.md` §8 had already named and applied "Architectural Activation" to Version Profiles' own first real content — was revised to record that the repository already contained an answer Sprint 12's own historical derivation never checked itself against, rather than an absence of any governing concept.
- **`PROJECT_HANDOFF_v1.1.md` §5** was amended with a two-sentence scope clarification (commit `fd2db36`): the Capability Identity gate applies to Collector/Rule-shaped candidates specifically; an already-specified-but-dormant subsystem receiving its first real content follows Architectural Activation directly to Architecture Evaluation instead. This introduces no new procedure, taxonomy, or architectural decision — it reconnects two already-existing ones, and preserves both Sprint 8's and Sprint 12's own documents unchanged.

## Design Resolution

- Preparing implementation surfaced that `AssessmentReport` and its nested Runtime types have no `Serialize`/`Deserialize` derive, and that their identifier types (`AssessmentId`, `EvidenceId`, `FindingId`) expose no accessor and cannot be reconstructed to a specific value — a real conflict with the Sprint Plan's own "`modiq-runtime`/`modiq-report` unmodified" constraint, reported rather than resolved unilaterally.
- **`STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md`** resolved it within `modiq-storage`'s own boundary: Storage defines and owns its own persisted representation — the fourth instance of ADR-0007's Opaque Runtime References pattern — populated from `AssessmentReport`'s already-public API, with faithfulness judged by content and order, never by Runtime identity, consistent with this platform's own pre-existing determinism convention. No modification to `modiq-runtime` or `modiq-report` was required.

## Deferred (Governance-Pending)

- GOV-001, GOV-002, GOV-003, GOV-008, and GOV-013 remain open; none were addressed this Sprint.
- No new Governance Register item or ADR was opened — this Sprint's own governance work clarified an existing standing rule's scope rather than establishing a new one.
- Two product-forcing-functions `INV-002` left explicitly undecided — cross-mod collection validation, and MKB accumulation from real Assessments — remain unresolved; Storage's own minimum-viable slice (single-report write/read) does not presuppose either.
- The `modiq-versioning` Crate Boundary Rules gap named during Sprint 8 planning remains open, unaffected by Sprint 13. `modiq-storage` gained its own Crate Boundary Rule pair this Sprint, recorded directly.
- Extension Layer — the platform's other dormant System Overview subsystem — remains untouched and unscoped.

## Released

- Documented in `docs/engineering/INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md`, `docs/engineering/GOVERNANCE_OBSERVATION_SUBSYSTEM_ACTIVATION.md`, `docs/engineering/STORAGE_ARCHITECTURE_EVALUATION.md`, `docs/engineering/STORAGE_IMPLEMENTATION_AUTHORIZATION.md`, `docs/engineering/STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md`, `docs/engineering/STORAGE_SPRINT_PLAN.md`, and `docs/engineering/ENGINEERING_RELEASE_1.3.md` (produced at this Sprint's own Closeout, not retroactively).

---

# [Sprint 14]

**Status:** In Progress (GOV-001 Architecture Evaluation complete; GOV-003 Architecture Evaluation, Architectural Resolution, Implementation Authorization, Sprint Planning, and Implementation complete)

## Investigated (No Action Taken)

- **GOV-001 (Assessment Report Generation Timing):** Architecture Evaluation, directed by the Chief Architect following the forcing function `modiq-storage`'s Sprint 13 activation created (a durably persisted report observed holding `status: EvaluatingRules`, not `Completed`). Found no inconsistency between `DataModel.md`'s specified Runtime Lifecycle and the implementation. No Architectural Resolution performed; GOV-001 narrowed and returned to Open. See `docs/engineering/ENGINEERING_LOG.md`.

## Removed

- **`modiq-common`** — retired in its entirety (GOV-003). The crate produced zero real content and zero consumers across all 14 Sprints, confirmed directly (every crate's `Cargo.toml` grepped workspace-wide; `crates/modiq-common/src/` inspected directly — four files, each a single doc comment). Architecture Evaluation (`docs/engineering/GOV003_ARCHITECTURE_EVALUATION.md`) recommended retirement over retaining the crate as-is, citing the platform's own demonstrated pattern of creating a crate on demand (`modiq-collection` at Sprint 3, `modiq-storage` at Sprint 13) rather than pre-provisioning an empty one. The Chief Architect accepted this recommendation. Removed from the root `Cargo.toml` workspace `members` list; `crates/modiq-common/` deleted entirely. No other crate required any change — zero consumers existed to update.
- `README.md`'s crate table, `docs/implementation/CrateRoadmap.md`'s crate table and dependency diagram, and `docs/implementation/DependencyMap.md`'s dependency diagram all had their `modiq-common` entries removed.

## Verified

- Root workspace: `cargo fmt --check`, `cargo check --workspace` clean, zero warnings; `cargo test --workspace` — **253 passed, 0 failed**, identical in count to the pre-retirement baseline (`modiq-cli` 15, `modiq-collection` 70, `modiq-engine` 23 unit + 3 integration, `modiq-knowledge` 5, `modiq-report` 3, `modiq-rules` 36, `modiq-runtime` 84, `modiq-storage` 10, `modiq-versioning` 4), confirming zero behavioral footprint from the removal.
- `apps/sandbox/src-tauri` (its own, separate workspace): `cargo fmt --check`, `cargo check` clean, zero warnings; `cargo test` — **9 passed, 0 failed**, unchanged from the pre-retirement baseline.
- A residual-reference grep (`modiq-common`/`modiq_common`) across the entire repository, outside historical Sprint/Release documents and this Sprint's own governance record, confirmed no remaining reference. `docs/architecture/` and `docs/adrs/` — this project's own architecture and ADR authorities — contain zero references, confirmed directly; `modiq-common` never had a Crate Boundary Rule pair in `GOVERNANCE.md` to remove.

## Deferred (Governance-Pending)

- GOV-002, GOV-008, GOV-013, and GOV-014 remain open; none were addressed this Sprint.
- GOV-001 remains open under a narrower question (see Investigated, above); no forcing function currently requires deciding it.
- The Governance Register moved from 14 items, 8 Resolved, 6 Open (post-Sprint-13) to **14 items, 9 Resolved, 5 Open** (GOV-003 resolved).
- The `modiq-versioning` Crate Boundary Rules gap named during Sprint 8 planning remains open, unaffected by this Sprint.

## Released

- Documented in `docs/engineering/GOV003_ARCHITECTURE_EVALUATION.md`, `docs/engineering/GOV003_IMPLEMENTATION_AUTHORIZATION.md`, `docs/engineering/GOV003_SPRINT_PLAN.md`, and `docs/engineering/ENGINEERING_LOG.md`.

---

# [Sprint 15]

**Status:** Complete (Architecture Evaluation and Architectural Resolution only — no implementation authorized, none found necessary; no Rust source touched)

## Resolved

- **GOV-002 (Runtime Invariant Reconciliation)** — Open since Engineering Release v0.1.0-alpha, never previously evaluated across 14 Sprints. Architecture Evaluation (`docs/engineering/GOV002_ARCHITECTURE_EVALUATION.md`) checked all fourteen Runtime Invariants directly against `modiq-runtime`'s implementation and test suite, and against the broader Runtime documentation set (`DataModel.md`, `Architecture.md`, ADR-0002/0003/0007), not `RuntimeInvariants.md` in isolation. Found: `RuntimeInvariants.md` conforms — all fourteen invariants enforced and valid, none violated. INV-002's wording is looser than the actual precondition it describes, noted as a documentation clarity opportunity, not a conflict. **Resolved**, cleanly, with no residual sub-question left open. The Governance Register moves from 14 items, 9 Resolved, 5 Open (post-Sprint-14) to **14 items, 10 Resolved, 4 Open**.

## Findings Outside GOV-002's Scope (Recommended, Not Acted On)

- **ADR-0003's `AssessmentReport` ownership claim conflicts with the current Reporting Crate Boundary Rule** — `AssessmentReport` is owned and generated by `modiq-report`, a separate crate, not by the `Assessment` aggregate ADR-0003 still names it under. Recommended as a **new Governance Register item**, per this project's standing discipline that a new architectural question surfaced during an evaluation is not automatically absorbed into the item that found it, and that a conflict with an Accepted ADR is resolved by governance decision, never silent reinterpretation.
- **Three lifecycle vocabularies** (`RuntimeInvariants.md`, `DataModel.md`'s seven-step conceptual lifecycle, `Architecture.md`'s eight-step pipeline diagram) have never been cross-reconciled, and `Architecture.md`'s own diagram omits Recommendation Generation as a stage. No invariant is violated; each document operates at a legitimately different abstraction level. Recommended as a **documentation maintenance task**, not governance work.
- `AssessmentSubject`/`AssessmentContext`'s minimal content (matches established precedent) and ADR-0007's own non-duplicated documentation of constructor validation and identity-based equality both require no action.

## Deferred (Governance-Pending)

- GOV-001, GOV-008, GOV-013, and GOV-014 remain open; none were addressed this Sprint.
- No new Governance Register item was opened this Sprint — the ADR-0003 conflict and lifecycle-vocabulary observation are recorded as recommendations only, not yet acted on.
- The `modiq-versioning` Crate Boundary Rules gap named during Sprint 8 planning remains open, unaffected by this Sprint.

## Released

- Documented in `docs/engineering/GOV002_ARCHITECTURE_EVALUATION.md` and `docs/engineering/ENGINEERING_LOG.md`.

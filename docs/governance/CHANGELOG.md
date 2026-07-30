# CHANGELOG

| Property | Value |
|----------|-------|
| **Document** | CHANGELOG.md |
| **Project** | modIQ |
| **Purpose** | Repository History |
| **Maintained By** | Project Maintainers |
| **Last Updated** | 2026-07-30 (WayfindingAndSearch.md approval) |

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

---

# [Sprint 16]

**Status:** Complete (Governance Initiation only — no Architecture Evaluation, Architectural Resolution, or Implementation Authorization; no Rust source, ADR, or Architecture.md touched)

## Added

- **GOV-015 (Role of ADR-0003 in Describing `AssessmentReport` Ownership)** — opened as a new Governance Register item, Open. Addresses the finding Sprint 15's own GOV-002 Architecture Evaluation surfaced outside its scope: ADR-0003 lists `AssessmentReport` as an Assessment-owned entity, contradicting the current Reporting Crate Boundary Rule (`AssessmentReport` is owned and generated by `modiq-report`, a separate crate, confirmed directly against `modiq-runtime`'s source). `PROPOSAL_GOV-015.md` defined the architectural question — whether ADR-0003 needs an in-place amendment or a new superseding ADR — using only evidence already established at Sprint 15; no new investigation was performed. ADR-0003's still-valid core decision (`Assessment` as aggregate root) is explicitly out of scope. The Governance Register moves from 14 items, 10 Resolved, 4 Open (post-Sprint-15) to **15 items, 10 Resolved, 5 Open**.

## Deferred (Governance-Pending)

- GOV-001, GOV-008, GOV-013, and GOV-014 remain open; none were addressed this Sprint.
- GOV-015 itself remains open — this Sprint only initiated it; Architecture Evaluation has not begun.
- The `modiq-versioning` Crate Boundary Rules gap named during Sprint 8 planning remains open, unaffected by this Sprint.

## Released

- Documented in `docs/engineering/PROPOSAL_GOV-015.md` and `docs/engineering/ENGINEERING_LOG.md`.

---

# [Sprint 17]

**Status:** Complete (Architecture Evaluation and Architectural Resolution only — no ADR created, no ADR proposal created, no implementation authorized; no Rust source, ADR, or Architecture.md touched)

## Resolved

- **GOV-015 (Role of ADR-0003 in Describing `AssessmentReport` Ownership)** — Architecture Evaluation (`docs/engineering/GOV015_ARCHITECTURE_EVALUATION.md`) corrected an assumption from Sprint 15's own GOV-002 evaluation: ADR-0003 did not "predate" the platform's crate architecture — `modiq-report` existed as its own crate concurrent with ADR-0003's own date (2026-07-16), confirmed against `CrateRoadmap.md`'s revision history and this Changelog's own Sprint 1 entry. The actual cause: ADR-0007, accepted one ADR later, established a precise technical definition of Runtime-entity "ownership" that ADR-0003's own earlier, looser example list was never checked against. **Resolved**: the discrepancy is an editorial imprecision, not an architectural inconsistency; `docs/adrs/README.md`'s own stated policy (already applied once, to ADR-0002's uncorrected numbering defect) requires a new superseding ADR rather than in-place amendment; the remaining discrepancy is purely documentary. `PROJECT_HANDOFF_v1.1.md`'s current text was found to trace to `DataModel.md`'s own independent framing, not a repetition of ADR-0003, and likely needs no substantive change. The Governance Register moves from 15 items, 10 Resolved, 5 Open (post-Sprint-16) to **15 items, 11 Resolved, 4 Open**.

## Recommended Follow-up (Described Only, Not Authorized)

- Prepare a proposal for a superseding ADR (`PROPOSAL_ADR-0011.md` or the next available number), addressing only ADR-0003's `AssessmentReport` inclusion and preserving its still-valid core decision. Per this project's governance discipline, the proposal is not itself authoritative — Chief Architect approval is required before it becomes the ADR, and each of proposal preparation, ADR creation, and reconciliation requires its own separate authorization.
- Re-check `PROJECT_HANDOFF_v1.1.md`'s own citation once that ADR's text exists.

## Deferred (Governance-Pending)

- GOV-001, GOV-008, GOV-013, and GOV-014 remain open; none were addressed this Sprint.
- No ADR or ADR proposal was created this Sprint — both remain future, separately-authorized work.
- The `modiq-versioning` Crate Boundary Rules gap named during Sprint 8 planning remains open, unaffected by this Sprint.

## Released

- Documented in `docs/engineering/GOV015_ARCHITECTURE_EVALUATION.md` and `docs/engineering/ENGINEERING_LOG.md`.

---

# [Sprint 18]

**Status:** Complete (Proposal preparation only — no ADR created, no ADR-0003 modification, no repository reconciliation; no Rust source touched)

## Added

- `docs/engineering/PROPOSAL_ADR-0011.md` — a proposal for a superseding ADR correcting ADR-0003's inclusion of `Assessment Report` among Assessment-owned entities, per GOV-015's own recommended follow-up. Stages the full proposed ADR-0011 text for Chief Architect review, per the project's two-step drafting discipline. Revised once, following review, to replace implementation-level detail with architecture-level language, condense the historical narrative (GOV-015 remains the authoritative evidence record), remove repository-relative framing from the draft ADR text, and remove forward-looking reconciliation guidance from the draft ADR's own Consequences section. No architectural conclusion changed by the revision.

## Deferred (Governance-Pending)

- GOV-001, GOV-008, GOV-013, and GOV-014 remain open; none were addressed this Sprint.
- ADR-0011 itself was not created this Sprint — only its proposal.

## Released

- Documented in `docs/engineering/PROPOSAL_ADR-0011.md` and `docs/engineering/ENGINEERING_LOG.md`.

---

# [Sprint 19]

**Status:** Complete (ADR creation only — no repository reconciliation, no governance-tracking-document updates, no ADR-0003 modification; no Rust source touched)

## Added

- **ADR-0011 (`AssessmentReport` Ownership Correction)** — `docs/adrs/0011-assessmentreport-ownership-correction.md`, Accepted. Supersedes ADR-0003 solely with respect to its `Assessment Report` example: `Assessment Report` is not an Assessment-owned entity, produced instead by the platform's Reporting subsystem from Assessment's already-public state. ADR-0003's aggregate-root decision and its other five owned-entity examples are unaffected and unmodified.

## Deferred (Governance-Pending)

- Repository reconciliation (the ADR index, `GOVERNANCE.md`'s GOV-015/GOV-002 cross-references, and this Changelog's own entries) was explicitly left for a separately authorized Sprint.
- The two living handoff documents (`PROJECT_HANDOFF_v1.1.md`, `CHIEF_ARCHITECT_HANDOFF_v1.1.md`) remain unaffected by this Sprint.
- GOV-001, GOV-008, GOV-013, and GOV-014 remain open; none were addressed this Sprint.

## Released

- Documented in `docs/adrs/0011-assessmentreport-ownership-correction.md` and `docs/engineering/ENGINEERING_LOG.md`.

---

# [Product Design Phase]

**Status:** Complete (six sessions; independent of, and not sequenced against, the Sprint/Engineering Release lineage above)

## Added

- `docs/product-design/WORKSPACE_EVOLUTION.md` — the workspace state model (Intake, Assessing, Reviewing), progressive disclosure, the Provisional/Final Finding model, mid-assessment uploads, and the Assessment Overview. Approved in principle.
- `docs/product-design/THE_FINDING.md` — the Finding as a product object: purpose, anatomy, information hierarchy, progressive disclosure, multi-Finding presentation, and its relationships to Recommendations and Evidence. Approved in principle.
- `docs/product-design/THE_ASSESSMENT_EXPERIENCE.md` — the complete Version 1 Assessment journey connecting the Workspace and Finding product objects. Approved in principle.
- `docs/product-design/EVIDENCE.md` — the Evidence experience: anatomy, disclosure, and its relationship to Findings and Recommendations. Approved in principle.
- `docs/product-design/THE_ASSESSMENT_REPORT.md` — the Assessment Report as the Assessment itself, viewed in its Reviewing state, rather than a separately generated artifact. Approved in principle.
- `docs/product-design/README.md` — reading order and authority pointer for the Product Design collection.
- `docs/engineering/PRODUCT_DESIGN_CLOSEOUT.md` — closes the Product Design phase, reflects on the five artifacts as a coherent body of work (recurring product patterns, design language, interaction philosophy, information architecture, and trust model), and names four conceptual gaps carried forward ungoverned: Recommendation glossary definition, Multi-Subject Assessment, Knowledge Base integration, and Community Contributions.

## Notes

- This phase and its artifacts carry no Documentation Authority ranking and do not modify `Vision.md`, `Principles.md`, `Glossary.md`, `ProductSpecification.md`, or `Architecture.md`.
- No Rust source, crate, test, ADR, or Governance Register item was touched by this phase.

---

# [Interaction Design Phase — Session 1]

**Status:** Complete (Assessment Intake & Upload; Chief Architect approved)

## Added

- `docs/interaction-design/ASSESSMENT_INTAKE_AND_UPLOAD.md` — behavior of the Intake workspace state: the empty workspace, upload interaction (selection, drag, multiple files, replacement, optional supporting material, unsupported files, duplicates), transition into Assessing, immediate feedback, error and recovery, progressive understanding, accessibility/interaction principles, and emotional experience. Names, without introducing new mechanics, a behavioral trust sequence (submission → recognition → Assessment → Finding → Evidence) distinct from and layered beneath the Product Design trust chain (Evidence → Finding → Recommendation). Approved.
- `docs/interaction-design/README.md` — reading order and authority pointer for the Interaction Design collection.

## Notes

- This document treats every Product Design artifact as a frozen constraint and introduces no product concept, implementation detail, runtime behavior, or data model decision.
- Further Interaction Design sessions (e.g., the Assessing/streaming experience, Finding/Evidence interaction, Recommendation interaction) are not yet scoped.

---

# [Repository Synchronization]

**Status:** Complete (working tree only — not committed)

## Added

- `docs/product-design/` and `docs/interaction-design/` directories, each with a `README.md` index, created to hold the Product Design and Interaction Design tracks respectively — previously approved only in conversation history, not represented in the repository.

## Changed

- `docs/README.md` — added a "Product & Interaction Design Track" section pointing to the product-track continuity documents and the new `docs/product-design/`/`docs/interaction-design/` collections, without altering the existing Reading Order or Documentation Release status.
- `docs/governance/PROJECT_STATUS.md` — added a "Product & Interaction Design Track" section recording the same state, explicitly noting this track is independent of, and not yet sequenced against, the Sprint/Engineering Release lineage.
- `docs/governance/CHANGELOG.md` (this file) — added the two phase entries above.

## Notes

- No Rust source, crate, test, ADR, Governance Register item, or frozen constitutional/architecture specification was touched.
- Changes are prepared in the working tree only; no commit was made.

---

# [Documentation Architecture Refinement]

**Status:** Complete (working tree only — not committed)

## Added

- `docs/DOCUMENTATION_MAP.md` — an architectural overview of the repository's documentation: the two lineages (Engineering Specification and Product Track), a documentation-groups table, a "governance means two things" naming note, a quick-orientation lookup table, and a named-but-deferred Known Navigation Gaps list (no index for `docs/engineering/`'s ~85 documents; Engineering Release records split across `docs/releases/` and `docs/engineering/`; `docs/governance/ROADMAP.md` stale since 2026-07-16).
- `docs/governance/README.md` — a short index for the project-tracking/process folder, distinguishing it from `docs/engineering/GOVERNANCE.md`'s architectural governance.

## Changed

- `docs/README.md` — added a pointer to `DOCUMENTATION_MAP.md`.
- `docs/product-design/README.md`, `docs/interaction-design/README.md`, `docs/adrs/README.md` — added "See Also" cross-references. No content rewritten.

## Notes

- No approved artifact was redesigned; no product concept, behavior, or specification content changed.
- `docs/governance/ROADMAP.md` was found significantly stale and is named in `DOCUMENTATION_MAP.md` rather than corrected here, as reconciling 20 Sprints of drift is outside this review's conservative scope.
- Changes are prepared in the working tree only; no commit was made.

---

# [Governance Refinement — Repository Synchronization Policy]

**Status:** Complete (working tree only — not committed)

## Added

- `docs/governance/RepositorySynchronizationPolicy.md` — formalizes the workflow established across the Product Design and Interaction Design sessions: conversation is a drafting environment; Product Owner approval establishes acceptance; approved artifacts are synchronized into the repository working tree promptly; the repository is the canonical source of truth; synchronization does not imply commit or push; synchronization is transcription, not design. Introduces the **Repository Synchronization** / **Repository Evolution** classification — the former requires no additional approval beyond the artifact's own; the latter (new directory hierarchies, documentation architecture changes, naming convention changes, major reorganization, new governance mechanisms) requires explicit Product Owner approval before implementation, not merely before commit. Includes an explicit relationship note distinguishing this classification from `GOVERNANCE.md`'s existing Level 1–4 Change Categories (a different axis: repository/documentation placement versus specification/architecture content) and from `DocumentationRelease.md`'s Documentation Release lifecycle (which this policy does not extend to the Product Track). Also includes a Retroactive Note: applying this classification to the immediately preceding two sessions, creating `docs/product-design/`, `docs/interaction-design/`, and `docs/DOCUMENTATION_MAP.md` would today register as Repository Evolution, satisfied retroactively by that session's own explicit "Repository Modification Responsibilities" grant rather than in violation of it.

## Changed

- `docs/governance/README.md` — added an index entry for the new policy.
- `docs/DOCUMENTATION_MAP.md` — added the policy to the Governance (process) group description and the Quick Orientation table.

## Notes

- `docs/engineering/GOVERNANCE.md` and `docs/governance/DocumentationRelease.md` were deliberately left unmodified — this policy governs a distinct concern (repository entry, across both lineages) that neither document currently addresses, rather than a revision to either.
- `docs/governance/PROJECT_STATUS.md` was deliberately not touched this session, to keep this a small, targeted governance addition rather than another full synchronization pass.
- No approved specification, product design artifact, or architectural decision was modified.
- Changes are prepared in the working tree only; no commit was made.

---

# [Interaction Design Phase — Session 2]

**Status:** Complete (Assessing & Progressive Discovery; Chief Architect approved; synchronized per `RepositorySynchronizationPolicy.md`)

## Added

- `docs/interaction-design/ASSESSING_AND_PROGRESSIVE_DISCOVERY.md` — behavior of the Assessing state: beginning assessment, progressive discovery (reconciling fixed Category/Severity presentation order against incremental Finding arrival), living assessment, user attention, trust during progress (continuing the behavioral trust sequence named in Session 1), interruption & recovery, and the transition into a completed Assessment. Names two deliberate non-decisions rather than inventing product behavior for them: cancellation of an in-progress Assessment, and Assessment Subject replacement mid-Assessing. Approved without revision; one editorial rename applied during synchronization (final section retitled "Chief Architect Reflection").

## Changed

- `docs/interaction-design/README.md` — added Session 2 to the Reading Order; added a "Version 1 Interaction Design Program" table (the full seven-session plan); updated Status.
- `docs/governance/PROJECT_STATUS.md` — updated the Product & Interaction Design Track section to reflect the seven-session program and Session 2's approval.

## Notes

- No Product Design artifact was modified; no Product Design concept was changed or redefined.
- Synchronization applied only the one authorized editorial change (heading rename) plus the removal of a conversational process note from the draft (an in-progress "not yet synchronized" status line that would have been inaccurate once the file existed in the repository) — no other wording was altered.
- Changes are prepared in the working tree only; no commit was made.

---

# [Interaction Design Phase — Session 3]

**Status:** Complete (Finding Presentation; approved without revision; synchronized per `RepositorySynchronizationPolicy.md`)

## Added

- `docs/interaction-design/FINDING_PRESENTATION.md` — the behavioral semantics of the Finding as the product's primary interaction object: first encounter, understanding at a glance, progressive exploration (Recognition → Summary → Understanding → Recommendation → Evidence, deliberately preserving The Finding's already-resolved Recommendation-before-Evidence ordering rather than adopting this session's own differently-ordered example language), relationship to Evidence and Recommendations, attention bounded by declared Severity, trust, revisiting, and mental navigation across multiple Findings without introducing a navigation system. Approved exactly as drafted, with no substantive revision requested.

## Changed

- `docs/interaction-design/README.md` — added Session 3 to the Reading Order; updated the Version 1 Interaction Design Program table (Session 3 Approved, Session 4 Next); updated Status.
- `docs/governance/PROJECT_STATUS.md` — updated the Product & Interaction Design Track section to reflect Session 3's approval and Session 4 as next.

## Notes

- No Product Design artifact was modified; no Product Design concept was changed or redefined.
- Synchronization removed the same kind of conversational process note this file's Session 2 entry already describes (a closing line confirming the document had not yet been synchronized) — an accuracy correction, not a content edit, consistent with prior sessions' synchronization notes.
- Changes are prepared in the working tree only; no commit was made.

---

# [Interaction Design Phase — Session 4]

**Status:** Complete (Evidence Exploration; approved without revision; synchronized per `RepositorySynchronizationPolicy.md`)

## Added

- `docs/interaction-design/EVIDENCE_EXPLORATION.md` — the interaction experience of Evidence as a supporting, invited object: choosing to explore (motivation-agnostic, single existing expansion action), first encounter (oriented to the Finding throughout, nothing left behind), progressive understanding (Label → Source → Explanation → Content), relationship to the Finding (a distinct "showing its work" metaphor, deliberately not reusing Recommendation's "continuing to speak" framing), relationship to Recommendations (strengthens confidence, never a prerequisite to meaning), trust, attention/cognitive load, leaving (no designed exit, because no designed destination), and revisiting (Evidence inherits currency from its Finding rather than carrying its own "updated" marker). Approved exactly as drafted, with no substantive revision requested.

## Changed

- `docs/interaction-design/README.md` — added Session 4 to the Reading Order; updated the Version 1 Interaction Design Program table (Session 4 Approved, Session 5 Next); updated Status.
- `docs/governance/PROJECT_STATUS.md` — updated the Product & Interaction Design Track section to reflect Session 4's approval and Session 5 as next.

## Notes

- No Product Design artifact was modified; no Product Design concept was changed or redefined.
- No previously approved Interaction Design artifact (Sessions 1–3) was modified.
- Synchronization removed the same kind of conversational process note prior sessions' entries already describe (a closing line confirming the document had not yet been synchronized) — an accuracy correction, not a content edit.
- Changes are prepared in the working tree only; no commit was made.

---

# [Interaction Design Phase — Session 5]

**Status:** Complete (Assessment Report Experience; approved without revision; synchronized per `RepositorySynchronizationPolicy.md`)

## Added

- `docs/interaction-design/ASSESSMENT_REPORT_EXPERIENCE.md` — the interaction experience of a completed Assessment Report as a coherent whole: first encounter (same workspace surface, absence of Provisional as the completion signal), recognition (Report Identity precedes exploration), orientation (one time-invariant mechanism serving same-day, days-later, and weeks-later returns alike, deliberately not scaled by elapsed time), experiencing the whole (located in the user's own act of comparison across consistently-presented parts — the Overview's per-dimension breakdown and Category grouping — rather than in any product-authored synthesis, checked explicitly against the Non-Goal barring subjective judgment and opaque scoring), the Assessment→Finding→Recommendation→Evidence chain as one continuous act of understanding, supporting decisions (via existing severity-ordering, Confidence, and reversibility, with no new planning/task feature introduced), returning, assessment-level trust (consistency at scale as its own trust signal), and completion (a personal realization enabled by trusting Severity order, not a product-announced event). Approved exactly as drafted, with no substantive revision requested. Chief Architect review additionally recorded that the Interaction Design corpus (Sessions 1–5) now forms a coherent behavioral model spanning assessment formation, understanding, justification, and long-term use — noted here for review history; this observation did not alter the synchronized artifact.

## Changed

- `docs/interaction-design/README.md` — added Session 5 to the Reading Order; updated the Version 1 Interaction Design Program table (Session 5 Approved, Session 6 Next); updated Status.
- `docs/governance/PROJECT_STATUS.md` — updated the Product & Interaction Design Track section to reflect Session 5's approval and Session 6 as next.

## Notes

- No Product Design artifact was modified; no Product Design concept was changed or redefined.
- No previously approved Interaction Design artifact (Sessions 1–4) was modified.
- Synchronization removed the same kind of conversational process note prior sessions' entries already describe (a closing line confirming the document had not yet been synchronized) — an accuracy correction, not a content edit.
- Changes are prepared in the working tree only; no commit was made.

---

# [Interaction Design Phase — Session 6]

**Status:** Complete (Navigation & Workspace Behavior; approved without revision; synchronized per `RepositorySynchronizationPolicy.md`; committed and pushed)

## Added

- `docs/interaction-design/NAVIGATION_AND_WORKSPACE_BEHAVIOR.md` — the behavioral semantics of movement throughout the workspace, synthesizing rather than introducing new mechanics: workspace continuity as one law applied uniformly across all three states and all four scopes (Overview/Finding/Recommendation/Evidence); orientation as cumulative, never re-derived; movement as changing scope rather than identity (extending The Assessment Report's "not a separate artifact" claim down through Recommendation and Evidence); focus as single-step, incremental depth change; returning as symmetric reversal of the same mechanism used to go deeper, not a separate behavior; reversibility as a direct consequence of orientation rather than an independent guarantee; stability of terminology/ordering/relationships/expectations as the actual mechanism underneath continuity; predictable movement itself as a trust mechanism; and completion recognized as the third instance of a cross-document pattern (no product-announced completion of Assessing, of reading a Report, or now of movement itself). Approved exactly as drafted, with no substantive revision requested.

## Changed

- `docs/interaction-design/README.md` — added Session 6 to the Reading Order; updated the Version 1 Interaction Design Program table (Session 6 Approved, Session 7 Next); updated Status.
- `docs/governance/PROJECT_STATUS.md` — updated the Product & Interaction Design Track section to reflect Session 6's approval and Session 7 as next.

## Notes

- No Product Design artifact was modified; no Product Design concept was changed or redefined.
- No previously approved Interaction Design artifact (Sessions 1–5) was modified.
- No implementation, interface component, or navigation mechanism was introduced — this document is explicitly a behavioral synthesis of already-established rules, named as such in its own Chief Architect Reflection.
- Synchronization removed the same kind of conversational process note prior sessions' entries already describe (a closing line confirming the document had not yet been synchronized) — an accuracy correction, not a content edit.
- This session's changes were committed and pushed, per explicit Product Owner instruction — the first Interaction Design session to be committed and pushed within this conversation rather than left for separate action.

---

# [Interaction Design Phase — Session 7: Closeout & Certification]

**Status:** Complete (capstone certification; approved with one targeted revision; synchronized per `RepositorySynchronizationPolicy.md`)

## Added

- `docs/interaction-design/INTERACTION_DESIGN_CLOSEOUT_AND_CERTIFICATION.md` — the formal architectural closeout of the complete Version 1 Interaction Design corpus (Sessions 1–6). Certifies: behavioral completeness (confirming Recommendation was correctly never given its own session, consistent with Product Design's two-level Finding→Recommendation hierarchy), internal consistency (citing the one real tension the corpus caught and resolved during authoring — Finding Presentation's Recommendation-before-Evidence ordering, held consistently through every later session), terminology consistency, architectural alignment (no Interaction Design document redefines a Product Design concept or constitutional term), behavioral coverage across all named user-journey transitions, six genuine cross-document patterns, and implementation readiness. **Version 1 Interaction Design is certified and approved for freeze as the implementation specification.** One Version 1 scope decision — cancellation of an in-progress Assessment, self-disclosed as undecided in Assessing & Progressive Discovery §7 and never formally dispositioned — is recorded as a Product Owner Disposition item, explicitly classified as a Version 1 scope decision rather than a corpus deficiency, since engineering can proceed without ambiguity under either resolution (defer or omit). A second, minor item (supplement upload mechanics never explicitly restated outside Intake's own scope) is noted as a non-blocking documentation observation. The document was revised once before approval, at Product Owner request, to reclassify the cancellation item from a conditional-certification caveat to the Product Owner Disposition framing above — propagated consistently through the Completeness Audit, Implementation Readiness, Final Verdict, and Chief Architect Reflection sections.

## Changed

- `docs/interaction-design/README.md` — added Session 7 to the Reading Order; completed the Version 1 Interaction Design Program table (all seven sessions Approved); updated Status to record the program as complete and Version 1 as frozen.
- `docs/governance/PROJECT_STATUS.md` — updated the Product & Interaction Design Track section: Interaction Design marked closed, Version 1 frozen as the implementation specification, with the cancellation scope decision noted as open and independently tracked.

## Notes

- No Product Design artifact was modified; no Product Design concept was changed or redefined.
- No previously approved Interaction Design artifact (Sessions 1–6) was modified.
- Synchronization removed the same kind of conversational process note prior sessions' entries already describe — an accuracy correction, not a content edit.
- Changes are prepared in the working tree only; no commit was made.

---

# [Engineering — Engineering Alignment Program Establishment]

**Status:** Complete (Engineering Alignment Program established; drafted, reviewed, and revised across three rounds of Chief Architect/Product Owner review before approval; synchronized per `RepositorySynchronizationPolicy.md`; not yet committed)

## Added

- `docs/engineering/ENGINEERING_ALIGNMENT_PROGRAM.md` — the governing engineering program for reconciling the existing deterministic platform with the frozen Version 1 Product Design and Interaction Design corpus, following an Engineering Reconciliation that checked the complete corpus directly against the platform's real source and identified five independent architectural evolution initiatives: Progressive Execution Observability, Reentrant Assessment Lifecycle, Domain Model Anatomy Extension, Confidence as a First-Class Concept, and Production Interaction Layer Definition. Establishes an Architectural Philosophy (existing implementation and the frozen corpus each presumed correct until evidence says otherwise; smallest sufficient architectural evolution; architectural integrity prioritized over implementation speed), Guiding Principles, per-initiative purpose/scope/dependencies/governance touchpoints, a recommended Architecture Evaluation order, ADR expectations (no ADR precedes an Architectural Resolution; ADR-0009 already anticipates Initiative 1's evidence), Sprint planning strategy, repository discipline, and Success Criteria defining program completion as a governance and architectural readiness state — architecture reconciled, governance and ADR record updated, implementation authorized where warranted, Sprint planning able to proceed against a stable specification — not merely a code-readiness state.
- `docs/engineering/README.md` — a new living index for the `docs/engineering/` directory, organizing its large and continually growing body of records by category (Handoffs, Governance, Alignment Programs, Architecture Evaluations & Resolutions, Implementation Authorizations & Sprint Plans, Sprint Records, Engineering Releases, Investigations, Proposals, Reviews/Studies/Assessments, Product-Track Continuity Records) rather than enumerating every document, and naming the specific documents currently authoritative in each area (`GOVERNANCE.md`, `ENGINEERING_LOG.md`, the current handoff trio, and `ENGINEERING_ALIGNMENT_PROGRAM.md`). No equivalent index previously existed for this directory.

## Changed

- `docs/governance/PROJECT_STATUS.md` — added a new Engineering Alignment Program section recording the program's establishment and recommended evaluation order (Production Interaction Layer Definition first, then Progressive Execution Observability and Reentrant Assessment Lifecycle concurrently, then Domain Model Anatomy Extension, then Confidence as a First-Class Concept); revised the Product & Interaction Design Track section's closing sentence to reflect that the Alignment Program is now the first engineering decision made in reference to that track, distinct from — and not itself — a Sprint scoped against it; updated Last Updated to 2026-07-28.

## Notes

- No Product Design artifact was modified; no Product Design concept was changed or redefined.
- No Interaction Design artifact was modified.
- No ADR was created or modified — Initiative 1's relationship to the existing, anticipatory ADR-0009 is noted for a future Architecture Evaluation to determine, not decided here.
- No Governance Register entry (`docs/engineering/GOVERNANCE.md`) was modified — this program identifies GOV-008 and GOV-013 as relevant existing items and anticipates new items for Initiatives 2, 4, and 5, but opens none itself; opening a Governance Register item remains an action reserved for each initiative's own future Architecture Evaluation.
- No Architecture Evaluation was performed and no initiative was evaluated, resolved, or implemented — this entry records only the establishment of the program that will govern that future work.
- The document was drafted, reviewed, and revised across three rounds before approval: an Architectural Philosophy section was added; the evaluation order was reconsidered and reordered with full reasoning recorded rather than silently changed (Production Interaction Layer Definition moved first, after review found the original producer-first ordering underweighted the consumer boundary's total absence of real existence); a dedicated `docs/engineering/README.md` index was created (a gap identified during review — no such file previously existed); reconciliation-finding language was revised to avoid implying the pre-existing implementation was defective rather than architecturally superseded; and two final editorial refinements were applied — removing a specific, driftable document count from `docs/engineering/README.md` in favor of growth-agnostic language, and strengthening the Success Criteria's concluding statement to name governance and architectural readiness explicitly, not implementation readiness alone.
- One item was initially identified but left uncorrected pending explicit authorization, per this repository's own standing practice of naming rather than silently fixing out-of-scope staleness: `ENGINEERING_ALIGNMENT_PROGRAM.md`'s own header property table still read "Status: Draft — pending review and explicit approval" after the two named editorial refinements were applied. On explicit, separate Product Owner instruction, a third, metadata-only update was then applied: `Status` changed to `Approved`, matching this repository's existing convention for accepted governance and design artifacts (e.g. Interaction Design sessions).
- Changes are prepared in the working tree only; no commit was made, per explicit instruction.

---

# [Engineering — Initiative 5 Architecture Evaluation: Production Interaction Layer Definition]

**Status:** Complete (Architecture Evaluation only; drafted, reviewed, and revised across two rounds before approval; synchronized per `RepositorySynchronizationPolicy.md`; not yet committed)

## Added

- `docs/engineering/INITIATIVE_5_ARCHITECTURE_EVALUATION.md` — the first Architecture Evaluation conducted under the Engineering Alignment Program, per its own recommended evaluation order. Determines: the production interaction layer is a consumer, structurally analogous to the already-governed `modiq-cli` Crate Boundary Rule, not an existing System Overview subsystem and not a candidate for the Sprint 8/13 Architectural Activation precedent (Decision 1); the engine/consumer responsibility split, extending the CLI entry's "owns user interaction, must never contain business logic" shape (Decision 2); that Assessment state ownership is already fixed by existing Runtime Invariants (INV-006, INV-009), not newly decided (Decision 3); that workspace, navigation, interaction, and presentation state are consumer-owned, with workspace state specifically derived from — not independent of — whatever execution-phase signal Initiative 1 eventually exposes (Decision 4); a request/response baseline for the boundary-crossing mechanism, with a supplementary-signal sub-question explicitly left open and dependent on Initiative 1's own resolution rather than guessed at here (Decision 5); and that `apps/sandbox`, as it exists today, cannot serve as the production interaction layer, though its request/response and getter-based DTO patterns are named as validated precedent (Decision 6). A standalone **Governance Observation** — modeled on `GOVERNANCE_OBSERVATION_SUBSYSTEM_ACTIVATION.md`'s own precedent, elevated out of the Decision sequence during review — records that the repository's existing governance vocabulary has no classification procedure for formalizing an external architectural boundary; neither Capability Identity (Sprint 12) nor Architectural Activation (Sprint 8/13) applies, and the gap is generalized beyond this initiative rather than treated as Initiative-5-specific. No Governance Register item was opened by that observation.

## Changed

- `docs/governance/PROJECT_STATUS.md` — added a paragraph to the Engineering Alignment Program section recording Initiative 5's Architecture Evaluation outcome and the Governance Observation; updated Last Updated to 2026-07-29.

## Notes

- No Product Design artifact was modified; no Product Design concept was changed or redefined.
- No Interaction Design artifact was modified.
- No ADR was created or modified.
- No Governance Register entry (`docs/engineering/GOVERNANCE.md`) was modified — the Governance Observation records a vocabulary gap without opening a Governance Register item, per explicit instruction.
- No Engineering Alignment Program document (`ENGINEERING_ALIGNMENT_PROGRAM.md`, `docs/engineering/README.md`) was modified — Initiative 5's evaluation was conducted entirely within its own new document.
- No existing crate, Crate Boundary Rule, or dependency edge was modified.
- No implementation technology, API, payload, or event was chosen, named, or implied as a choice.
- No Architectural Resolution was performed — this entry records an Architecture Evaluation only, per the document's own Status field.
- The document was drafted, reviewed, and revised across two rounds before approval: the initial draft's Section 10 ("Cross-Cutting Classification") was restructured, at explicit review request, into the standalone Governance Observation described above — generalized from an initiative-specific finding into a standing repository-wide gap, given its own header block and internal Observation/Evidence/Not-Resolved-Here/Non-Scope structure, and removed from the Decision 1–6 numbering entirely. This did not alter any Decision's own conclusions, recommendations, or evidence.
- Changes are prepared in the working tree only; no commit was made, per explicit instruction.

---

# [Engineering — Initiative 5 Architectural Resolution: Production Interaction Layer Definition]

**Status:** Complete (Architectural Resolution; drafted and revised across one round before approval; synchronized per `RepositorySynchronizationPolicy.md`; not yet committed)

## Added

- `docs/engineering/INITIATIVE_5_ARCHITECTURAL_RESOLUTION.md` — dispositions every recommendation from `docs/engineering/INITIATIVE_5_ARCHITECTURE_EVALUATION.md`, treated throughout as fixed, verified evidence rather than reopened. Adopted: the consumer classification (Decision 1); the engine/consumer responsibility split (Decision 2); confirmation that Assessment state ownership is already fixed by INV-006/INV-009 (Decision 3); consumer ownership of navigation, interaction, and presentation state (Decision 4a); consumer ownership of workspace state in principle (Decision 4b, ownership half); and the request/response mechanism baseline (Decision 5a). Deferred, pending Initiative 1's own resolution: workspace state's derivation mechanism (Decision 4b, remainder) and a supplementary notification/poll-trigger mechanism (Decision 5b). Adopted: the finding that `apps/sandbox`, as it exists today, cannot serve as the production interaction layer, with its request/response and getter-based DTO pattern noted as validated precedent, not a technology endorsement (Decision 6). No recommendation was Rejected; none required further investigation. The Governance Observation is addressed separately from the architectural decisions, per instruction: acknowledged as valid and standing, not itself dispositioned, and not made a Governance Register item by this Resolution. Section 4 (Consequences) acknowledges only that the adopted decisions may eventually warrant an ADR, a Governance Register item, or both, under the repository's existing governance process — without proposing, characterizing, or recommending either.

## Changed

- `docs/governance/PROJECT_STATUS.md` — added a paragraph to the Engineering Alignment Program section recording Initiative 5's Architectural Resolution and its full disposition summary.

## Notes

- No Product Design artifact was modified; no Product Design concept was changed or redefined.
- No Interaction Design artifact was modified.
- No Engineering Alignment Program document (`ENGINEERING_ALIGNMENT_PROGRAM.md`, `docs/engineering/README.md`) was modified.
- `docs/engineering/INITIATIVE_5_ARCHITECTURE_EVALUATION.md` was not modified — referenced only, as this Resolution's primary evidence source, per explicit instruction that the Evaluation not be reopened.
- No ADR was created or modified.
- No Governance Register entry (`docs/engineering/GOVERNANCE.md`) was modified.
- No crate, Crate Boundary Rule, or dependency edge was modified.
- No implementation technology, API, payload, or event was chosen, named, or implied as a choice. No implementation was begun; no Sprint was scoped.
- The document was drafted, reviewed, and revised once before approval: Section 4 ("Consequences") was softened at explicit review request, from naming specific candidate governance artifacts (an ADR analogous to ADR-0009; a Governance Register item analogous to GOV-009/GOV-010) to a general acknowledgment that governance work may eventually be warranted, without characterizing its form — consistent with this Resolution's own purpose being to adopt architecture, not to recommend future governance work. No other section was altered by that revision.
- Changes are prepared in the working tree only; no commit was made, per explicit instruction.

---

# [Engineering — Initiative 1 Architecture Evaluation: Progressive Execution Observability]

**Status:** Complete (Architecture Evaluation only; drafted and revised across one round before approval; synchronized per `RepositorySynchronizationPolicy.md`; not yet committed)

## Added

- `docs/engineering/INITIATIVE_1_ARCHITECTURE_EVALUATION.md` — the second Architecture Evaluation conducted under the Engineering Alignment Program, treating Initiative 5's adopted Architectural Resolution as fixed, unreopened precedent throughout. Finds: current architecture does not support exposing assessment progress before completion, with the barrier traced precisely to `AssessmentService::execute`'s single-synchronous-call control flow rather than to any Runtime invariant — INV-002/003/004/010–012 govern mutation only, never observation (Question 1). Repository evidence points toward exposing the growing, not-yet-final Finding set at Evidence-Collector and per-Rule-dispatch boundaries, not a percentage- or time-based signal, subject to preserving GOV-012's already-adopted fixed dispatch order (Question 2). At least one observable execution-phase signal is architecturally required, confirming rather than merely repeating Initiative 5's own Decision 4b working assumption, though its exact granularity remains an open question (Question 3). Findings today are immutable once added and produced in a single atomic batch, with no incremental or provisional concept expressed anywhere — narrowing the real architectural gap to observing an accumulating set rather than inventing per-Finding incremental finalization, since Interaction Design itself requires Findings to finalize "together, as a set" (Question 4). A genuine tension exists between the current one-shot Report generation model and Interaction Design's progressive-Overview requirement; the tension is identified and found to require future architectural resolution, without comparing or selecting among candidate resolution models (Question 5). Interaction Design's own requirements for observable engine progress are catalogued independent of any consumer implementation (Question 6). Questions belonging to Initiative 2 (reentrant lifecycle, post-completion Report modification), Initiative 3 (field-level state representation), and Initiative 4 (Confidence, unaffected throughout) are explicitly named and left unresolved. A closing Evaluation Boundaries section states which questions were answered, which are left to later initiatives, and which adopted Initiative 5 decisions were relied upon without reopening.

## Changed

- `docs/governance/PROJECT_STATUS.md` — added a paragraph to the Engineering Alignment Program section recording Initiative 1's Architecture Evaluation outcome.

## Notes

- No Product Design artifact was modified; no Product Design concept was changed or redefined.
- No Interaction Design artifact was modified.
- No Initiative 5 document (`INITIATIVE_5_ARCHITECTURE_EVALUATION.md`, `INITIATIVE_5_ARCHITECTURAL_RESOLUTION.md`) was modified — both were consulted only as adopted precedent, per explicit instruction not to reopen or re-evaluate any question Initiative 5 already adopted.
- No ADR was created or modified.
- No Governance Register entry (`docs/engineering/GOVERNANCE.md`) was modified.
- No crate, Crate Boundary Rule, or dependency edge was modified.
- No implementation technology, API, transport, payload, or event was chosen, named, or implied as a choice. No implementation was begun; no Sprint was scoped.
- No Architectural Resolution was performed — this entry records an Architecture Evaluation only, per the document's own Status field.
- The document was drafted, reviewed, and revised once before approval: Question 5 ("Report Evolution") was revised at explicit review request to identify the tension between the current one-shot generation model and Interaction Design's progressive-Overview requirement, and to conclude that the tension requires future resolution, with the prior draft's description and comparison of candidate report-evolution models (immutable/append-only/living/regenerated) removed. No other section was altered by that revision.
- Changes are prepared in the working tree only; no commit was made, per explicit instruction.

---

# [Engineering — Initiative 1 Architectural Resolution: Progressive Execution Observability]

**Status:** Complete (Architectural Resolution; drafted and revised across one round before approval; synchronized per `RepositorySynchronizationPolicy.md`; not yet committed)

## Added

- `docs/engineering/INITIATIVE_1_ARCHITECTURAL_RESOLUTION.md` — dispositions every recommendation from `docs/engineering/INITIATIVE_1_ARCHITECTURE_EVALUATION.md`, treated throughout as fixed, verified evidence rather than reopened. Adopts three confirmed facts: progressive visibility is not currently supported (F-1); the growing, not-yet-final Finding set is the correct conceptual target state (F-2); a genuine tension exists between the current one-shot Report generation model and Interaction Design's progressive-Overview requirement (F-3). Adopts five binding **Adopted Architectural Constraints**, a disposition category introduced in this Resolution for findings that express a durable "shall" statement rather than a settled classification: progressive visibility must be established within the Engine orchestration layer without weakening or reinterpreting any Runtime domain invariant (AC-1); observation shall occur only at architecturally coherent, already-deterministic dispatch boundaries — per-Evidence-Collector completion and GOV-012's own fixed per-Rule dispatch order — never an arbitrary, percentage-based, or time-based cadence (AC-2); at least one observable execution-phase signal shall exist (AC-3); any Finding-visibility mechanism shall preserve set-wide finalization, Findings becoming Final together rather than individually (AC-4); and observable progress shall support recognition by absence, never by explicit announcement (AC-5). Two items are dispositioned Deferred and left intentionally unresolved: execution-phase signal granularity (D-1) and the Report-evolution resolution model (D-2). Explicitly restates, without re-deciding, which questions belong to Initiative 2 (reentrant lifecycle, post-completion Report modification), Initiative 3 (concrete field-level representation of the state the adopted constraints require), and Initiative 4 (Confidence, unaffected). No item was Rejected; none required further investigation.

## Changed

- `docs/governance/PROJECT_STATUS.md` — added a paragraph to the Engineering Alignment Program section recording Initiative 1's Architectural Resolution and its full disposition summary.

## Notes

- No Product Design artifact was modified; no Product Design concept was changed or redefined.
- No Interaction Design artifact was modified.
- `docs/engineering/INITIATIVE_1_ARCHITECTURE_EVALUATION.md` was not modified — referenced only, as this Resolution's primary evidence source, per explicit instruction that the Evaluation not be reopened.
- No Initiative 5 document (`INITIATIVE_5_ARCHITECTURE_EVALUATION.md`, `INITIATIVE_5_ARCHITECTURAL_RESOLUTION.md`) was modified.
- No Engineering Alignment Program document (`ENGINEERING_ALIGNMENT_PROGRAM.md`, `docs/engineering/README.md`) was modified.
- No ADR was created or modified.
- No Governance Register entry (`docs/engineering/GOVERNANCE.md`) was modified.
- No crate, Crate Boundary Rule, or dependency edge was modified.
- No implementation mechanism, API, transport, payload, or event was chosen, named, or implied as a choice. No implementation was begun; no Sprint was scoped.
- The document was drafted, reviewed, and revised once before approval: D-1 and D-2 were revised at explicit review request to state simply that each remains intentionally unresolved, removing references to future design work, Implementation Authorization, or any future governance artifact; Section 8 ("Readiness") was simplified to conclude Initiative 1 is architecturally resolved except for D-1 and D-2, without referring to future Implementation Authorization. No other section was altered by that revision.
- Changes are prepared in the working tree only; no commit was made, per explicit instruction.

---

# [Engineering — Initiative 2 Architecture Evaluation: Reentrant Assessment Lifecycle]

**Status:** Complete (Architecture Evaluation only; drafted and revised across one round before approval; synchronized per `RepositorySynchronizationPolicy.md`; not yet committed)

## Added

- `docs/engineering/INITIATIVE_2_ARCHITECTURE_EVALUATION.md` — the third Architecture Evaluation conducted under the Engineering Alignment Program, treating both Initiative 5's and Initiative 1's adopted Architectural Resolutions as fixed, unreopened precedent throughout. Finds: `AssessmentStatus::Completed` is unconditionally terminal, checked first on every mutation path, with no supplementation exception anywhere in the implementation (Question 1). Two independent barriers block reentrancy — `INV-012` in-memory, and a separate, independently-evidenced cross-process barrier, since Storage's own read path never reconstructs a live `Assessment` or `AssessmentReport` from persisted data, by explicit design (Question 2). `DataModel.md`'s Immutability principle is genuinely ambiguous as applied to same-Assessment supplementation — its own illustrative scope (engine/rule drift) is narrower than its general wording, and neither reading has ever been checked against Workspace Evolution's own requirement (Question 3). The Updated-marker mechanism is confirmed as a field-level question for Initiative 3, dependent on Question 1's own resolution as a precondition (Question 4). A completed, historical Assessment Report cannot currently reflect new material at any layer the evidence covers — Runtime, Reporting, or Storage — as a direct, unavoidable consequence of Question 1's finding, completing the second half of the Report-evolution question Initiative 1 explicitly left to this initiative (Question 5). Initiative 2 is identified as a plausible candidate forcing function for GOV-001's own still-open question, without resolving GOV-001 itself (Question 6). Initiative Boundaries and a closing Evaluation Boundaries section explicitly distinguish this initiative's own central question (whether INV-012's terminality should itself evolve) from Initiative 1's AC-1 (which found no Runtime invariant needed to change for its own, different purpose), and name what remains for Initiative 3 (field-level representation) and Initiative 4 (unaffected).

## Changed

- `docs/governance/PROJECT_STATUS.md` — added a paragraph to the Engineering Alignment Program section recording Initiative 2's Architecture Evaluation outcome.

## Notes

- No Product Design artifact was modified; no Product Design concept was changed or redefined.
- No Interaction Design artifact was modified.
- No Initiative 1 or Initiative 5 document (`INITIATIVE_1_ARCHITECTURE_EVALUATION.md`, `INITIATIVE_1_ARCHITECTURAL_RESOLUTION.md`, `INITIATIVE_5_ARCHITECTURE_EVALUATION.md`, `INITIATIVE_5_ARCHITECTURAL_RESOLUTION.md`) was modified — all four were consulted only as adopted precedent.
- No Engineering Alignment Program document (`ENGINEERING_ALIGNMENT_PROGRAM.md`, `docs/engineering/README.md`) was modified.
- No ADR was created or modified.
- No Governance Register entry (`docs/engineering/GOVERNANCE.md`) was modified.
- No crate, Crate Boundary Rule, or dependency edge was modified.
- No implementation mechanism, API, transport, payload, or event was chosen, named, or implied as a choice. No implementation was begun; no Sprint was scoped.
- No Architectural Resolution was performed — this entry records an Architecture Evaluation only, per the document's own Status field.
- The document was drafted, reviewed, and revised once before approval: Question 5's Conclusion and the closing Evaluation Boundaries summary were revised at explicit review request — "every layer it touches" narrowed to name the specific layers the evidence covers (Runtime, Reporting, and Storage), and "inventing a reading" replaced with the more neutral "adopting one interpretation over the other" in Question 3's Conclusion. No other section was altered by that revision.
- Changes are prepared in the working tree only; no commit was made, per explicit instruction.

---

# [Engineering — Initiative 2 Architectural Resolution: Reentrant Assessment Lifecycle]

**Status:** Complete (Architectural Resolution; drafted, stress-tested, and revised across four rounds before approval; synchronized per `RepositorySynchronizationPolicy.md`)

## Added

- `docs/engineering/INITIATIVE_2_ARCHITECTURAL_RESOLUTION.md` — dispositions every question raised by `docs/engineering/INITIATIVE_2_ARCHITECTURE_EVALUATION.md`, and, uniquely among the Alignment Program's Resolutions so far, does not adopt a lifecycle direction for its own central question. Adopted: both barriers to reentrancy are real and independent (Question 2); GOV-001 is unaffected regardless of outcome (Question 6); the Updated-marker mechanism's precondition, without a directional grounding (Question 4); and, decisively, that Product Design's own stated justification for its position ("no new mechanism is needed" for long-absence returns) is not satisfiable under current Runtime and Storage architecture, independent of which direction is eventually chosen — a finding not in the original Evaluation. Question 1 (terminal completion vs. reentrancy) is dispositioned **Requires Governance Reconciliation**, a new disposition this Resolution introduces alongside the four already established by Initiatives 5 and 1, and explicitly distinguished from "Requires Additional Investigation": the evidence-gathering stage is complete — every relevant document has been identified and read — and what remains is an authority decision between two frozen documents making incompatible claims (`DataModel.md`'s Immutability principle versus `THE_ASSESSMENT_REPORT.md` §2's explicit "not a second Report or a new version of one... no new mechanism is needed"), not further reading. An earlier draft's "Assessment Sequence" architecture (preserving `INV-012`, resolving supplementation via a new, related Assessment) is explicitly **not adopted** — reached by reasoning that read `DataModel.md` as decisive and Product Design's language as merely experiential without first checking whether Product Design's own text supported that reading; it did not, once checked, and the finding is withdrawn rather than carried forward. GOV-015 was considered as precedent for this reconciliation and found not structurally equivalent — GOV-015 had a tiebreaker external to both conflicting documents (already-built architecture matching one of them); no equivalent tiebreaker exists here, since neither candidate direction is implemented anywhere in the current codebase. A standalone **Governance Observation** further finds that `GOVERNANCE.md`'s Documentation Authority hierarchy defines no precedence relationship covering the Product & Interaction Design track at all — Product Design and Interaction Design appear in none of its ten ranked tiers — which is part of why Question 1 cannot currently be reconciled by any existing repository mechanism, independent of this specific conflict's own substance.

## Changed

- `docs/governance/PROJECT_STATUS.md` — added a paragraph to the Engineering Alignment Program section recording Initiative 2's Architectural Resolution outcome and its full disposition summary.

## Notes

- No Product Design artifact was modified; no Product Design concept was changed or redefined. `THE_ASSESSMENT_REPORT.md` and `DataModel.md` were read and quoted, not altered.
- No Interaction Design artifact was modified.
- `docs/engineering/INITIATIVE_2_ARCHITECTURE_EVALUATION.md` was not modified — referenced only, as this Resolution's primary evidence source.
- No Initiative 1 or Initiative 5 document was modified.
- No Engineering Alignment Program document (`ENGINEERING_ALIGNMENT_PROGRAM.md`, `docs/engineering/README.md`) was modified.
- No ADR was created or modified.
- No Governance Register entry (`docs/engineering/GOVERNANCE.md`) was modified — including no amendment to its Documentation Authority section, despite the Governance Observation naming a gap in it; per that observation's own explicit non-scope, no fix is proposed or performed.
- No crate, Crate Boundary Rule, or dependency edge was modified.
- No implementation mechanism, API, transport, or persistence mechanism was designed. No implementation was begun; no Sprint was scoped.
- This document went through four rounds of review before approval, more than any prior Alignment Program artifact, each substantively changing the document rather than only its wording: (1) an initial draft reached a decisive lifecycle-direction conclusion; (2) a requested stress-test found `THE_ASSESSMENT_REPORT.md` §2 directly contradicted that conclusion's central premise, and the draft was rewritten to withdraw the decision rather than defend it; (3) "Requires Additional Investigation" was replaced with the new "Requires Governance Reconciliation" disposition, and the GOV-015 comparison was checked for structural equivalence and removed as unsupported rather than kept as loose precedent; (4) a further review identified that the conflict exposes a distinct governance capability gap (Documentation Authority's silence on the Product & Interaction Design track), recorded as its own Governance Observation without proposing a fix; (5) a final editorial pass replaced correctness/blame language ("false," "incorrect," documents "built to accept" or "presupposing" intent) with compatibility and observable-behavior language throughout, changing no disposition, evidence, or conclusion.
- Changes are prepared in the working tree only; commit and push follow immediately per explicit Product Owner-directed workflow for this cycle.

---

# [Platform Architecture — Constitutional Foundation]

**Status:** Complete (`PlatformSpecification.md` approved as the constitutional foundation of a new Platform Architecture lineage; drafted and revised across multiple constitutional review passes before approval; synchronized per `RepositorySynchronizationPolicy.md`; not yet committed)

## Added

- `docs/platform/PlatformSpecification.md` — the constitutional foundation of a new Platform Architecture lineage, distinct from the Engineering Specification lineage and the Product & Interaction Design Track. Establishes the Platform Domain Model (Organization, User, Membership, Project, Upload, Notification, Subscription, API Client, Public Visitor, with Assessment referenced only as a fixed boundary); an Architectural Boundary dividing this lineage from Runtime Architecture exactly at the Assessment (Platform Architecture owns everything above it; Runtime Architecture owns Assessment Subject, Evidence, Finding, Recommendation, and Assessment Report, unmodified); an Architectural Responsibilities table naming what each of the five relevant lineages owns, including that Engineering Alignment owns a reconciliation process rather than a domain concept; terminology resolutions distinguishing Console from the existing Workspace and Wayfinding from the existing Navigation (both reserved, pending use by a governed document); a shared-vocabulary treatment of Upload across its Interaction Design experience aspect and its Platform Architecture record aspect, proposed for Glossary addition; a Platform Architecture Filter (five criteria every governed document must satisfy); four Architectural Decisions Deferred to future governed documents; and a Platform Architecture Governance rule making this specification the sole authority for first-class concepts in the lineage. Approved by Product Owner disposition.

## Changed

- `docs/DOCUMENTATION_MAP.md` — added Platform Architecture as a third lineage alongside the Engineering Specification lineage and the Product Track; added a Documentation Groups row and a Quick Orientation row; added a Known Navigation Gap noting `docs/platform/` currently contains only `PlatformSpecification.md`, with no README and no governed topic document yet drafted.
- `docs/README.md` — added a "Platform Architecture Track" section, mirroring the existing "Product & Interaction Design Track" section; updated the top pointer note to reference it.
- `docs/governance/PROJECT_STATUS.md` — added a "Platform Architecture Track" section recording `PlatformSpecification.md`'s approval; renamed the unrelated "### Platform Architecture" subsection under Phase 1 — Foundation (which listed only `Architecture.md`'s Frozen status) to "### Runtime Architecture" to remove a naming collision with the new lineage. No status or content under that subsection changed — only its heading.

## Notes

- No constitutional document (`Vision.md`, `Principles.md`, `Glossary.md`, `ProductSpecification.md`) was modified.
- No Product Design or Interaction Design artifact was modified.
- No Runtime Architecture document (`Architecture.md`, `DataModel.md`, and the rest) was modified.
- No Engineering Alignment Program document was modified.
- The same "Platform Architecture" phrase, used with an unrelated meaning, was also found in three historical records — `docs/engineering/TECHNICAL_DIRECTOR_HANDOFF_v2.0.md`, `v2.1.md`, `v2.2.md`. These were left unmodified, as dated historical snapshots, distinct from `PROJECT_STATUS.md`'s live-dashboard status, which this pass keeps current.
- No governed Platform Architecture topic document (`IdentityAndAccess.md`, and the rest) was drafted.
- No ADR was created. No Governance Register entry was opened or modified. No crate boundary was modified.
- Changes are prepared in the working tree only; no commit was made.

---

# [Platform Architecture — IdentityAndAccess.md]

**Status:** Complete (`IdentityAndAccess.md` approved as the first governed specification of the Platform Architecture lineage; drafted and revised across two refinement passes before approval; synchronized per `RepositorySynchronizationPolicy.md`)

## Added

- `docs/platform/IdentityAndAccess.md` — refines Organization, User, Membership, and API Client, all defined in `PlatformSpecification.md`, without introducing any first-class Platform Architecture concept. States Identity Principles (smallest sufficient concept set; Membership as the sole standing-granting relationship; Role as an attribute of Membership rather than a concept; additional complexity only through demonstrated need and governed amendment). Establishes Access Relationships as Organization-wide rather than Project-scoped. Defines Role as an attribute of Membership with two values (Owner, Member) — stated explicitly as the current minimum vocabulary, not a closed constitutional set; future values amend this document, not `PlatformSpecification.md`. Resolves two of `PlatformSpecification.md`'s four Architectural Decisions Deferred: every Project requires an Organization, with registration automatically establishing one and an Owner Membership for a solo User; and API Client access is Organization-wide, not Role- or Project-scoped, since Membership's mediating role exists specifically to handle a User's potential multi-Organization standing, which an API Client — already directly Organization-owned per `PlatformSpecification.md`'s own Ownership Model — has no analogous need for. Approved by Product Owner disposition.

## Changed

- `docs/DOCUMENTATION_MAP.md` — updated the Platform Architecture Documentation Groups row and the Known Navigation Gap entry to record `IdentityAndAccess.md`'s approval; six governed topic documents remain undrafted.
- `docs/README.md` — added `IdentityAndAccess.md` to the Platform Architecture Track section.
- `docs/governance/PROJECT_STATUS.md` — added `IdentityAndAccess.md` to the Platform Architecture Track section, recording its two resolved Architectural Decisions Deferred.

## Notes

- No constitutional document (`Vision.md`, `Principles.md`, `Glossary.md`, `ProductSpecification.md`, `PlatformSpecification.md`) was modified.
- No Product Design, Interaction Design, or Runtime Architecture artifact was modified.
- No first-class Platform Architecture concept was introduced — Role is refined strictly as an attribute of the already-established Membership concept.
- No `docs/platform/README.md` was created — flagged as a reasonable next step once more governed topic documents exist, not created preemptively in this pass.
- No implementation, persistence, credential, or API-contract detail was introduced; `IdentityAndAccess.md`'s own Boundaries section states these are explicitly out of scope.
- No further governed Platform Architecture topic document (`ProjectsAndUploads.md`, and the rest) was drafted.
- Committed and pushed to `feature/runtime-implementation` as a standalone milestone, per the Repository Workflow Standard established alongside this approval.

# [Engineering — Initiative 3 Architecture Evaluation: Domain Model Anatomy Extension]

**Status:** Architecture Evaluation complete. Approved by Chief Architect disposition. No Architectural Resolution performed.

**Affected Crates:** None. No source code was modified.

**Affected Documents:**

- Added: `docs/engineering/INITIATIVE_3_ARCHITECTURE_EVALUATION.md`.
- Updated: `docs/governance/PROJECT_STATUS.md` — appended an Initiative 3 Architecture Evaluation summary paragraph to the Engineering Alignment Program section.

**Notes:**

- Executed under the execution structure (3A independent items, 3B Item 10 dependency determination, 3C deferred watch list) approved prior to this evaluation.
- Evaluated all nine 3A field-shape items against direct repository evidence, including a first-time read of `Glossary.md`, `KnowledgeModel.md`, and `EvidenceCollection.md`'s Collector Contract for this specific scope.
- Item 4 (Evidence anatomy) surfaced an **Architectural Observation** — Evidence's Explanation field conflates field anatomy, subsystem responsibility, construction timing, and entity ownership, the last two of which exceed ordinary field-shape evaluation and are not resolved here.
- Item 6a (Report Identity) was found to connect to GOV-002's own prior "require no action" disposition on `AssessmentSubject`'s zero-field state; a concrete requirement for that content, absent when GOV-002 was evaluated, is named as a candidate for reconsideration, not reopened here.
- 3B's own narrow determination finds Item 10 (Provisional/Final) Independent of Initiative 2's unresolved Question 1, grounded in ADR-0007's own scope boundary and the confirmed absence of any mutation method on `Finding`, without reasoning about either candidate lifecycle model.
- 3C's two items (Report currency, the Updated marker) remain deferred pending Initiative 2's Governance Reconciliation; not evaluated here.
- Initiative 2's Question 1 disposition (Requires Governance Reconciliation) is treated as fixed precedent and is not reopened.
- `docs/platform/PlatformSpecification.md` (Platform Architecture lineage, committed separately as `1191a4b`) is noted in the Evaluation's own repository-state section as outside Initiative 3's scope, per its own stated Architectural Boundary; not treated as evidence.
- No ADR was created. No Governance Register entry was opened or modified. No crate boundary was modified. No implementation, API, or field representation was adopted.
- Changes are prepared in the working tree only; no commit was made.

# [Engineering — Initiative 3 Architectural Resolution: Domain Model Anatomy Extension]

**Status:** Architectural Resolution complete. Approved by Chief Architect disposition.

**Affected Crates:** None. No source code was modified.

**Affected Documents:**

- Added: `docs/engineering/INITIATIVE_3_ARCHITECTURAL_RESOLUTION.md`.
- Updated: `docs/governance/PROJECT_STATUS.md` — appended an Initiative 3 Architectural Resolution summary paragraph to the Engineering Alignment Program section.

**Notes:**

- Dispositions every item from `INITIATIVE_3_ARCHITECTURE_EVALUATION.md` individually rather than as a single vote, following Initiative 2's precedent. No new disposition vocabulary introduced.
- **Adopted:** Items 1 (Title/Summary), 3 (optional Recommendation cardinality), 4 (Evidence's Label/Source/Content), 5 (RepairRecipe structured steps), 6a (Report Identity field, as part of `AssessmentReport`'s domain model), and 10 (Provisional/Final, confirmed Independent of Initiative 2's Question 1). Item 2 (Category) adopted contingent on Item 9's naming constraint.
- **Adopted Architectural Constraint (Item 9 — naming):** the Finding-level Mod Health concept shall not be named `Category`. No specific replacement identifier is adopted — the Evaluation confirmed the collision but did not evaluate candidate identifiers; selection is left to Implementation Authorization. This narrows an earlier draft that had adopted the specific identifier `HealthDimension`, which was found on review to exceed the Evaluation's own evidence.
- **Adopted Architectural Constraint (Item 6a — implementation prerequisite):** the Subject-identity field shall not be implemented until `AssessmentSubject` carries sufficient content. Separated, on review, from the adopted field addition itself, to keep the domain-model decision and its implementation dependency distinct. Named as connected to GOV-002's prior "require no action" disposition; GOV-002 is not reopened by this Resolution.
- Item 4's Explanation field remains **Requires Additional Investigation** — the Architectural Observation names the question without answering it; evidence-incomplete, not an authority conflict, so "Requires Governance Reconciliation" does not apply.
- Items 7 (GOV-013) and 8 (Glossary's missing "Recommendation" entry) are named, not dispositioned — outside this Resolution's authority.
- 3C's two items (Report currency, the Updated marker) remain deferred pending Initiative 2's Governance Reconciliation.
- No ADR was created. No Governance Register entry was opened or modified. No crate boundary was modified. No implementation was authorized.
- Changes are prepared in the working tree only; no commit was made.

---

# [Platform Architecture — ProjectsAndUploads.md]

**Status:** Complete (`ProjectsAndUploads.md` approved as the second governed specification of the Platform Architecture lineage; drafted and revised across one refinement pass before approval; synchronized per `RepositorySynchronizationPolicy.md`)

## Added

- `docs/platform/ProjectsAndUploads.md` — refines Project and Upload, both defined in `PlatformSpecification.md`, without introducing any first-class Platform Architecture concept. States Platform Principles consistent with `IdentityAndAccess.md`'s Identity Principles. Refines Project's relationships to Organization, Upload, and Assessment; states that archival requires Owner standing per `IdentityAndAccess.md`'s Role definition; confirms deletion is not a Platform Architecture concept, since the constitutional Lifecycle Model has no third state beyond Created and Active/Archived. Refines Upload as the platform-level record of a submission's provenance, distinct from the material's own nature; confirms Upload's one-to-one relationship to the Assessment it triggers is inherited directly from `PlatformSpecification.md`, not a new constraint imposed on Runtime Architecture; keeps the model neutral on whether a later Assessment continues or restarts for the same Assessment Subject, an open Runtime Architecture question. Resolves the one remaining Project-related item of `PlatformSpecification.md`'s Architectural Decisions Deferred: Project is auto-created, provisioned automatically when none has been designated to receive an Upload, rationale grounded in `IdentityAndAccess.md`'s own auto-provisioning precedent for Organization and in `ASSESSMENT_INTAKE_AND_UPLOAD.md`'s frozen "costs the user nothing to try" design principle. Approved by Product Owner disposition.

## Changed

- `docs/DOCUMENTATION_MAP.md` — updated the Platform Architecture Documentation Groups row and the Known Navigation Gap entry to record `ProjectsAndUploads.md`'s approval; five governed topic documents remain undrafted.
- `docs/README.md` — added `ProjectsAndUploads.md` to the Platform Architecture Track section.
- `docs/governance/PROJECT_STATUS.md` — added `ProjectsAndUploads.md` to the Platform Architecture Track section, recording its resolved Architectural Decision Deferred.

## Notes

- No constitutional document (`Vision.md`, `Principles.md`, `Glossary.md`, `ProductSpecification.md`, `PlatformSpecification.md`) was modified.
- No Product Design, Interaction Design, or Runtime Architecture artifact was modified.
- `docs/platform/IdentityAndAccess.md` was not modified — referenced only, as precedent for the auto-provisioning rationale.
- No first-class Platform Architecture concept was introduced.
- No `docs/platform/README.md` was created — same reasoning as `IdentityAndAccess.md`'s milestone: a reasonable next step once more governed topic documents exist, not created preemptively.
- No implementation, persistence, or API-contract detail was introduced; `ProjectsAndUploads.md`'s own Boundaries section states these are explicitly out of scope.
- No further governed Platform Architecture topic document (`DashboardAndConsole.md`, and the rest) was drafted.
- Committed and pushed to `feature/runtime-implementation` as a standalone milestone, per the Repository Operating Charter.

---

# [Platform Architecture — Post-Verification Synchronization and Amendment]

**Status:** Complete (documentation synchronization plus one small governed amendment, following an architectural verification pass across `PlatformSpecification.md`, `IdentityAndAccess.md`, and `ProjectsAndUploads.md`)

## Changed

- `docs/platform/PlatformSpecification.md` — synchronized the Architectural Decisions Deferred table to record that three of its four items have since been ratified in `IdentityAndAccess.md` and `ProjectsAndUploads.md`; the fourth (Notification addressing scope) remains open. Documentation synchronization only — no concept, boundary, or governance rule changed.
- `docs/platform/IdentityAndAccess.md` — amended the Role section to clarify that the Owner-only restriction on Project creation governs deliberate, administrative creation only and does not extend to `ProjectsAndUploads.md`'s automatic Project provisioning, which follows from any Member's or API Client's existing standing to submit an Upload. No Role value was added or changed; no Platform Domain Model concept was introduced.

## Notes

- Both changes originated from an explicit architectural verification pass (chat-recorded, not a separate document) that checked all three Platform Architecture documents for internal consistency, boundary integrity, and cross-document gaps.
- `docs/platform/ProjectsAndUploads.md` was not modified — its "Assessment Subject" wording, flagged during verification as a borderline but compliant boundary reference, was explicitly left unchanged.
- No new first-class Platform Architecture concept was introduced by either change.
- No Product Design, Interaction Design, or Runtime Architecture artifact was modified.
- Navigation/index files (`DOCUMENTATION_MAP.md`, `README.md`, `PROJECT_STATUS.md`) were not updated — neither change alters what documents exist or their existing summaries there.
- Committed and pushed to `feature/runtime-implementation` as a standalone milestone, per the Repository Operating Charter.

# [Engineering — Initiative 2 Governance Reconciliation: Question 1]

**Status:** Governance Reconciliation complete. Approved by Chief Architect disposition. Initiative 2 closed.

**Affected Crates:** None. No source code was modified.

**Affected Documents:**

- Added: `docs/engineering/INITIATIVE_2_GOVERNANCE_RECONCILIATION.md`.
- Updated: `docs/governance/PROJECT_STATUS.md` — appended an Initiative 2 Governance Reconciliation summary paragraph to the Engineering Alignment Program section.

**Notes:**

- Completes the reconciliation `INITIATIVE_2_ARCHITECTURAL_RESOLUTION.md` deferred under the "Requires Governance Reconciliation" disposition. That Resolution is not amended — it accurately recorded what was known when written.
- Resolved GQ-A1 from `GOVERNANCE.md`'s Documentation Authority text alone: omission of an entire corpus (Product Design, Interaction Design, Platform Architecture) from the ten-tier hierarchy means the hierarchy provides no rule for that comparison, not that the omitted corpus is subordinate.
- Resolved GQ-B from `ProjectsAndUploads.md`'s adopted text alone: its explicit neutrality on "whether that new Assessment continues or restarts for the same Assessment Subject" excludes Platform Architecture's Upload→Assessment relationship from bearing on Question 1.
- With both resolved and neither supplying a tiebreaker, `DataModel.md`'s Immutability principle and `THE_ASSESSMENT_REPORT.md` §2 remain in direct, unmediated conflict; no other adopted architecture (ADR-0007, GOV-015, GOV-001) independently resolves it.
- **Determination: Question 1 cannot presently be reconciled under the repository's current governance model** — an evidenced negative finding, not a lack of effort or evidence.
- A governance finding, not a new disposition, records that the existing six-value disposition vocabulary no longer accurately describes this state. No new disposition is defined or adopted.
- Two governance recommendations are named for separate consideration, not acted on here: whether Documentation Authority should be amended to rank Product Design, Interaction Design, and Platform Architecture; and whether the disposition-vocabulary gap should be addressed, and how.
- Initiative 2 is complete on this basis, having exhausted the repository's currently documented authority framework for Question 1.
- The Engineering Alignment Program is **not** complete: Initiative 4 (Confidence as a First-Class Concept) remains unevaluated.
- No ADR was created. No Governance Register entry was opened or modified. No crate boundary was modified. No Documentation Authority or other governance document was amended.
- Changes are prepared in the working tree only; no commit was made.

---

# [Platform Architecture — DashboardAndConsole.md]

**Status:** Complete (`DashboardAndConsole.md` approved as the third governed specification of the Platform Architecture lineage; drafted and revised across one editorial pass before approval; synchronized per `RepositorySynchronizationPolicy.md`)

## Added

- `docs/platform/DashboardAndConsole.md` — refines Console and Dashboard, the two experience-layer elements `PlatformSpecification.md`'s Scope names but excludes from the Platform Domain Model, without introducing any first-class Platform Architecture concept. Establishes Console as the persistent interaction shell (reflects Platform state, contains interaction regions, owns no Platform concept, no independent lifecycle) and Dashboard as its default region (presents Platform state derived entirely from the Domain Model). References Workspace only to establish its containment relationship to Console, without redefining it — Workspace remains Product Design's and Interaction Design's. Establishes a Platform Vocabulary table giving each Platform element and each interaction-layer element exactly one verb. Explicitly excludes navigation, movement between Organizations/Projects/Assessments, search, and notification behavior/delivery, naming `WayfindingAndSearch.md` and `Notifications.md` as their respective owners; acknowledges only that Console may contain an Organization selector as shell chrome, without defining switching behavior. Resolves no item from `PlatformSpecification.md`'s Architectural Decisions Deferred table. Approved by Product Owner disposition.

## Changed

- `docs/DOCUMENTATION_MAP.md` — updated the Platform Architecture Documentation Groups row and the Known Navigation Gap entry to record `DashboardAndConsole.md`'s approval; four governed topic documents remain undrafted.
- `docs/README.md` — added `DashboardAndConsole.md` to the Platform Architecture Track section.
- `docs/governance/PROJECT_STATUS.md` — added `DashboardAndConsole.md` to the Platform Architecture Track section.

## Notes

- No constitutional document (`Vision.md`, `Principles.md`, `Glossary.md`, `ProductSpecification.md`, `PlatformSpecification.md`) was modified.
- No Product Design or Interaction Design artifact was modified — `WORKSPACE_EVOLUTION.md` and `NAVIGATION_AND_WORKSPACE_BEHAVIOR.md` were referenced only, as the authority Workspace remains under.
- No Runtime Architecture artifact was modified.
- No first-class Platform Architecture concept was introduced.
- This document's own drafting surfaced a scope question (whether Navigation, Search, Organization switching, and Notifications belonged here) that was resolved by deferring all four to `WayfindingAndSearch.md` and `Notifications.md`, consistent with `PlatformSpecification.md`'s existing Governs list and Terminology entries — not a new decision, a confirmation of existing delegation.
- No `docs/platform/README.md` was created — same reasoning as the prior two milestones.
- No implementation, persistence, routing, or API-contract detail was introduced; this document's own Boundaries section states these are explicitly out of scope.
- No further governed Platform Architecture topic document (`WayfindingAndSearch.md`, and the rest) was drafted.
- Committed and pushed to `feature/runtime-implementation` as a standalone milestone, per the Repository Operating Charter.

# [Engineering — Initiative 4 Architecture Evaluation: Confidence as a First-Class Concept]

**Status:** Architecture Evaluation complete. Approved by Chief Architect disposition. No Architectural Resolution performed.

**Affected Crates:** None. No source code was modified.

**Affected Documents:**

- Added: `docs/engineering/INITIATIVE_4_ARCHITECTURE_EVALUATION.md`.
- Updated: `docs/governance/PROJECT_STATUS.md` — appended an Initiative 4 Architecture Evaluation summary paragraph to the Engineering Alignment Program section.

**Notes:**

- Confirmed unblocked by Initiative 3 (`Recommendation`'s field shape unchanged by its adopted items) and independent of Initiative 2's Question 1 (Confidence attaches within a single Assessment's Recommendation, orthogonal to reentrancy).
- Found Confidence's placement settled and convergent across the full Product Design and Interaction Design corpus, searched in full: scoped exclusively to a Recommendation, never shown on a Provisional Finding, measures evidence quality rather than correctness.
- Found Confidence's concrete representation (scale, type, range) and its computation mechanism both entirely unspecified anywhere in the frozen corpus, confirmed against `RuleEngine.md`'s six responsibilities and all of Runtime Architecture (`DataModel.md`, `VersionProfile.md`, `KnowledgeModel.md`, `EvidenceCollection.md`).
- Applied `SPRINT8_ARCHITECTURAL_RESOLUTION.md` §8's own Architectural Activation test directly: Confidence matches one distinguishing trait of that precedent (forces a previously-stable `Recommendation::new` signature change) but not the other (no Runtime-Architecture-tier dormant placeholder was found in `DataModel.md`, `RuleEngine.md`, or `Architecture.md`'s System Overview, unlike Version Profiles or Storage) — a genuine, unresolved classification tension, not decided here.
- Checked GOV-013 (`FindingSeverity` Severity/Kind Conflation) directly for a mechanistic dependency with Confidence and found none — only shape-proximity, contrary to a coupling the Alignment Program's own scope note had flagged for checking.
- Re-flagged, not newly found: the Glossary's missing "Recommendation" entry (Initiative 3's Item 8) remains directly relevant, since Confidence's placement rules depend throughout on that concept.
- No ADR was created. No Governance Register entry was opened or modified. No crate boundary was modified. No data type, computation mechanism, or classification was adopted.
- Changes are prepared in the working tree only; no commit was made.

---

# [Platform Architecture — WayfindingAndSearch.md]

**Status:** Complete (`WayfindingAndSearch.md` approved as the fourth governed specification of the Platform Architecture lineage; drafted and revised across one editorial pass before approval; synchronized per `RepositorySynchronizationPolicy.md`)

## Added

- `docs/platform/WayfindingAndSearch.md` — refines Wayfinding and Search, the two remaining unaddressed elements of `PlatformSpecification.md`'s platform experience layer, without introducing any first-class Platform Architecture concept. Establishes the Governing Principle "Wayfinding changes which Platform resource and interaction region are active. It does not alter the internal state of an interaction region, create Platform state, mutate Platform state, or participate in Platform lifecycle actions." Confirms the three navigable Platform resources (Organization, Project, Assessment up to the Workspace boundary) directly from `PlatformSpecification.md`'s own Wayfinding terminology entry. Establishes that Wayfinding operates only over what `IdentityAndAccess.md`'s Membership already makes visible, performing no independent authorization. Extends the Platform Vocabulary table with Wayfinding (moves) and Search (locates), keeping one verb per responsibility across nine elements. Explicitly excludes notification behavior, runtime search implementation, and all Workspace/Product Design/Interaction Design territory. Resolves no item from `PlatformSpecification.md`'s Architectural Decisions Deferred table. Approved by Product Owner disposition.

## Changed

- `docs/DOCUMENTATION_MAP.md` — updated the Platform Architecture Documentation Groups row and the Known Navigation Gap entry to record `WayfindingAndSearch.md`'s approval; three governed topic documents remain undrafted.
- `docs/README.md` — added `WayfindingAndSearch.md` to the Platform Architecture Track section.
- `docs/governance/PROJECT_STATUS.md` — added `WayfindingAndSearch.md` to the Platform Architecture Track section.

## Notes

- No constitutional document (`Vision.md`, `Principles.md`, `Glossary.md`, `ProductSpecification.md`, `PlatformSpecification.md`) was modified.
- No Product Design or Interaction Design artifact was modified.
- No Runtime Architecture artifact was modified.
- No first-class Platform Architecture concept was introduced.
- This document's own framing, developed across several prior chat turns before drafting, resolved three scope boundaries later confirmed unchanged during drafting: Dashboard vs. Wayfinding (presents vs. moves, including that a transition triggered from Dashboard's own content is still Wayfinding's), Identity & Access vs. Wayfinding (visibility vs. movement over what's visible), and Wayfinding vs. Search (moves vs. locates).
- No `docs/platform/README.md` was created — same reasoning as the prior three milestones.
- No implementation, indexing, ranking, or routing detail was introduced; this document's own Boundaries sections state these are explicitly out of scope.
- No further governed Platform Architecture topic document (`Notifications.md`, and the rest) was drafted.
- Committed and pushed to `feature/runtime-implementation` as a standalone milestone, per the Repository Operating Charter.

# Engineering Log

## Entry Format

Each engineering log entry should use the following structure:

- Status
- Affected Crates
- Affected Documents
- Notes

This convention applies to all future entries. Existing historical entries are preserved as originally written.

---

## 2026-07-15

### Documentation Release 1.0 Frozen

Completed the first architectural specification set.

Established the platform architecture, runtime domain, rule engine model, engineering knowledge model, and implementation roadmap.

Documentation became the authoritative source for implementation.

---

## 2026-07-16

### Sprint 0 Completed

Initialized repository.

Established crate hierarchy.

Created foundational documentation.

Implemented workspace structure.

---

## 2026-07-17

### Assessment Aggregate Implemented

Implemented:

- lifecycle
- Evidence ownership
- Finding ownership
- Recommendation ownership

Assessment became the aggregate root of the runtime domain.

---

### Rule Engine Prototype

Implemented the first deterministic Rule.

Purpose is architectural validation rather than assessment capability.

No Rule abstraction introduced.

---

### Reporting System

Implemented immutable AssessmentReport snapshot generation.

Reporting performs no analysis.

---

### Engine Orchestration

Implemented AssessmentService::execute().

This completed the first executable Assessment pipeline.

Architecture validated end-to-end.

---

### Release

Repository tagged:

v0.1.0-alpha

Sprint 1 completed.

---

## 2026-07-18

### Removed Orphaned Sprint 0 Scaffold Files from modiq-runtime

Status:
Completed

Affected Crates:
- modiq-runtime

Affected Documents:
- HANDOFF_SPRINT1.md

Notes:
While reviewing the repository ahead of Sprint 2, found four files in `modiq-runtime/src/assessment/` left over from the Sprint 0 skeleton commit that were never wired into `mod.rs` and never adopted during Sprint 1: `assessment_builder.rs` (empty), `assessment_metadata.rs` and `assessment_summary.rs` (informal field-name notes, not valid Rust), and `report.rs` (an empty `pub struct AssessmentReport;`, re-exported from `modiq-runtime`'s public API).

The last of these was the more significant discrepancy: `GOVERNANCE.md`'s crate boundary rules assign `AssessmentReport` to Reporting (`modiq-report`), which already has a fully implemented `AssessmentReport`. The `modiq-runtime` copy was an unused duplicate of a name Governance places in a different crate — dead code rather than an active violation, since nothing consumed it, but a discrepancy worth recording rather than silently carrying forward.

Confirmed via workspace-wide grep that no code referenced any of the four files, then removed them and their `mod`/`use` declarations from `modiq-runtime/src/assessment/mod.rs`. `cargo fmt`, `cargo check --workspace`, and `cargo test --workspace` all pass with no change to the 55-test count.

---

## 2026-07-19

### Sprint 2: Runtime Domain Content Implemented

Status:
Completed

Affected Crates:
- modiq-runtime
- modiq-rules
- modiq-report
- modiq-engine

Affected Documents:
- SPRINT2_IMPLEMENTATION_PLAN.md

Notes:
Implemented in three reviewed phases: Evidence (id, category, description, location), Finding (id, severity, description, evidence_ids, rule_reference), and Recommendation (id, action, finding_ids, repair_recipe_reference). Each phase added a process-local monotonic identity type (`EvidenceId`, `FindingId`, `RecommendationId`) following the existing `AssessmentId` pattern, a dedicated validation error type, and a fallible constructor rejecting empty or whitespace-only descriptive content.

`modiq-rules::RuleEngine::evaluate` required a minimal, non-behavioral production-code change in each of the Finding and Recommendation phases, since it is the sole out-of-crate producer of those values — unlike Evidence, which was only ever constructed in other crates' test code. Decision logic did not change (evidence present → one Finding and one Recommendation; absent → neither); only the content of the values produced changed, using real data already available to the function (e.g. `evidence_ids` populated from the actual evidence evaluated).

Two content-level invariants were identified and deliberately left unenforced pending governance approval: a new Finding invariant requiring at least one Evidence reference, and a refinement of INV-005 requiring a Recommendation to reference specific, existing Finding(s) rather than merely coexist with some Finding. Both are recorded as GOV-005 and GOV-006 in `GOVERNANCE.md`.

`cargo fmt`, `cargo check --workspace`, and `cargo test --workspace` passed at each phase gate. Workspace test count: 55 → 86.

---

### Sprint 2: Assessment Aggregate Integration

Status:
Completed

Affected Crates:
- modiq-runtime

Affected Documents:
- SPRINT2_IMPLEMENTATION_PLAN.md

Notes:
Extended `Assessment` with four read-only methods: `evidence_by_id`, `finding_by_id`, `evidence_for_finding`, and `findings_for_recommendation`. These make the aggregate root responsible for resolving the cross-entity relationships Finding and Recommendation carry as stored data, but which nothing previously consulted.

No rejection-based validation was added. `SPRINT2_IMPLEMENTATION_PLAN.md`'s Governance Prerequisites bundle both the cardinality question (must a Finding/Recommendation reference at least one Evidence/Finding) and the referential-integrity question (must every reference resolve) into the same governance-gated deliverable; since neither GOV-005 nor GOV-006 has been resolved, both aspects were treated as pending. All four new methods are permissive: an unresolvable reference is omitted from the result, never rejected. `add_evidence`, `add_finding`, and `add_recommendation` are unchanged from the prior phase.

No changes were required outside `modiq-runtime` — a first for this sprint, confirming the new methods are purely additive.

`cargo fmt`, `cargo check --workspace`, and `cargo test --workspace` passed. Workspace test count: 86 → 97.

---

### Engineering Release 0.2

Status:
Completed

Affected Crates:
- (none — documentation only)

Affected Documents:
- PROJECT_STATUS.md
- CHANGELOG.md
- GOVERNANCE.md
- CrateRoadmap.md
- docs/adrs/README.md
- docs/adrs/0007-runtime-entity-design-pattern.md
- docs/releases/ENGINEERING_RELEASE_0.2.md

Notes:
Froze Sprint 2 into a formal Engineering Release. Published ADR-0007 (Runtime Entity Design Pattern), recording the aggregate-root, identity, value-object, opaque-reference, constructor-validation, identity-based-equality, aggregate-owned-resolution, governance-controlled-invariant, and determinism conventions established across the sprint. Published `docs/releases/ENGINEERING_RELEASE_0.2.md` as the full architectural record, including Sprint 3 entry criteria. Added GOV-005 and GOV-006 to the Governance Register. Corrected `PROJECT_STATUS.md` and `CHANGELOG.md`, both of which had remained unreconciled since 2026-07-17 (describing Sprint 1 as active despite Sprint 1 having concluded at Engineering Release v0.1.0-alpha).

Two pre-existing documentation defects were identified during this review and flagged rather than corrected, since accepted ADRs are not to be modified per `docs/adrs/README.md`: `0002-domain-model-boundaries.md` internally mislabels itself "ADR-0001," and `0006-documentation-release-1.0-freeze.md` references a git tag (`v0.1.0-docs`) that was never created. Both are noted in `docs/releases/ENGINEERING_RELEASE_0.2.md`'s Known Deferred Work; the first is also flagged directly in `docs/adrs/README.md`.

No Documentation Release was required — no frozen specification changed during Sprint 2.

`cargo fmt`, `cargo check --workspace`, and `cargo test --workspace` confirmed green (97 tests, 0 failures) as the closing verification for this release.

---

### Sprint 3 Phase 1: Sandbox Pipeline Integration

Status:
Completed

Affected Crates:
- (none — `apps/sandbox` only, not a workspace member)

Affected Documents:
- (none)

Notes:
Rewrote `apps/sandbox`'s `create_assessment` Tauri command to construct one deterministic Evidence item and execute it through the real pipeline — `modiq-engine::AssessmentService::execute`, the same orchestration entry point `modiq-engine`'s own integration tests use — rather than returning an empty Assessment DTO. Added `modiq-engine` and `modiq-report` as sandbox dependencies. The resulting `AssessmentReport` is mapped into a serializable `AssessmentSummary` DTO (with nested Evidence/Finding/Recommendation entries); the `Assessment` aggregate itself is never exposed over IPC. `App.tsx` updated to render the DTO's counts and lists. No domain logic was added to the sandbox and no crate boundary was crossed.

The original phase framing had asked the Rule Engine to construct Evidence itself; an architectural review (recorded in `TECHNICAL_DIRECTOR_HANDOFF_v2.0.md`) found this would reverse `RuleEngine.md`'s frozen "Evidence is consumed, not produced" specification and identified real Evidence Collection as a genuine, currently unowned architectural gap rather than something to improvise into `modiq-rules`. Implementation proceeded instead using Evidence supplied as ordinary external input, exactly as existing integration tests already did.

`cargo fmt`, `cargo check`, and `cargo test` passed in both the root workspace (unaffected, 97 tests) and the sandbox's own workspace (3 tests).

---

### Sprint 3 Phase 2: GOV-005/GOV-006 Cardinality Enforcement

Status:
Completed

Affected Crates:
- modiq-runtime
- modiq-report (test fixtures only)

Affected Documents:
- RuntimeInvariants.md
- GOVERNANCE.md
- PROPOSAL_GOV-005_GOV-006.md (new)

Notes:
Resolved GOV-005 and GOV-006, both open since Sprint 2, by Technical Director decision following an Architecture Review proposal. `Finding::new` now rejects empty `evidence_ids` and `Recommendation::new` now rejects empty `finding_ids`, recorded as new invariants INV-013 and INV-014. GOV-006 was resolved as a new invariant rather than a rewording of INV-005, since INV-005's existing text was not incorrect, only under-specified for content-level reference validation.

Scope was explicitly limited to cardinality by Technical Director direction. Referential integrity — whether a reference actually resolves within its Assessment — was explicitly excluded and remains a separate, still-open governance question; `Assessment::evidence_for_finding` and `Assessment::findings_for_recommendation` are unchanged, `modiq-rules::RuleEngine` is unchanged, and `apps/sandbox` is unaffected.

Two `modiq-runtime` tests were removed (`evidence_for_finding_is_empty_when_the_finding_references_no_evidence`, `findings_for_recommendation_is_empty_when_the_recommendation_references_no_findings`): both constructed a Finding/Recommendation with zero references to exercise resolution behavior, a state no longer reachable through the public API. Equivalent coverage of "resolution returns empty for an unresolvable reference" is preserved by the adjacent dangling-reference tests, which were untouched.

This documentation update (RuntimeInvariants.md, GOVERNANCE.md) was made directly rather than through a full Documentation Release cycle (Draft → Foundation Review → Technical Review → Repository Audit → Documentation Freeze → Release Tag). Documentation Release 2.1 — under which GOV-005/GOV-006 were originally filed as "pending" — remains a future documentation milestone, not superseded by this change; referential integrity in particular is expected to be part of it.

`cargo fmt`, `cargo check --workspace`, and `cargo test --workspace` passed (95 tests, down from 97: two obsolete tests removed, two new rejection tests added, net −2 in `modiq-runtime`). Sandbox workspace independently reverified, unaffected (3 tests).

---

### Sprint 3 Phase 3: Evidence Collection Architecture & Governance Foundation

Status:
Completed (architecture/documentation only)

Affected Crates:
- (none — no runtime code changed this phase)

Affected Documents:
- docs/adrs/0008-evidence-collection-subsystem-boundary.md (new)
- docs/adrs/0009-assessmentservice-public-api-evolution.md (new)
- docs/adrs/README.md
- docs/architecture/EvidenceCollection.md (new)
- docs/architecture/Architecture.md
- docs/architecture/DataModel.md
- docs/architecture/RuleEngine.md
- docs/implementation/CrateRoadmap.md
- docs/engineering/GOVERNANCE.md
- docs/governance/DocumentationRelease.md
- docs/engineering/PROPOSAL_EVIDENCE_COLLECTION_BOUNDARY.md (prior phase's proposal; approved this phase, not modified)

Notes:
Following Technical Director approval of `PROPOSAL_EVIDENCE_COLLECTION_BOUNDARY.md`'s architectural direction (not implementation), this phase established the architectural and governance foundation for Evidence Collection without writing any production code. Two ADRs were drafted per explicit Technical Director instruction to keep them separate: ADR-0008 records Evidence Collection as its own subsystem (a new, not-yet-created crate, working name `modiq-collection`, orchestrated by `modiq-engine`, depending only on `modiq-runtime`), with alternatives (inside Runtime, inside Engine, inside Knowledge) considered and rejected against existing `GOVERNANCE.md` boundary rules. ADR-0009 separately records that `modiq-engine`'s public execution entry point will eventually require a breaking change to accept an Input Descriptor, without deciding that change's shape.

A new Technical Layer specification, `EvidenceCollection.md`, was written alongside `RuleEngine.md`, defining the subsystem's responsibilities, non-responsibilities, orchestration flow, the Input Descriptor concept, and a conceptual Collector Contract (inputs, outputs, guarantees, non-responsibilities, determinism expectations) — deliberately free of Rust types or interfaces, per explicit instruction.

A pre-existing inconsistency between `Architecture.md`'s Assessment Lifecycle diagram (which placed Evidence Collection after Rule Engine) and `DataModel.md`'s Runtime Lifecycle (which already placed Evidence Collection before Findings Produced) — first identified in the prior phase's proposal — was resolved by correcting `Architecture.md` to match `DataModel.md` and the actual, already-implemented pipeline. The correction is recorded explicitly as a Documentation Release 2.1 amendment within `Architecture.md` itself, not silently rewritten, per `docs/governance/DocumentationRelease.md`'s "documented contradiction" exception to Frozen-specification stability.

`GOVERNANCE.md` gained a new Evidence Collection Crate Boundary Rules entry and four new Governance Register items — GOV-007 (implementation approval), GOV-008 (AssessmentService API evolution, companion to ADR-0009), GOV-009 (Input Descriptor ownership), GOV-010 (collection error model) — all explicitly left Open, per instruction not to resolve them this phase. `CrateRoadmap.md` records `modiq-collection` as planned, not created, and its Sprint 3 status section was brought current (it had gone stale after Sprint 3 Phase 1/2, describing Sprint 3's scope as "not yet selected").

`docs/governance/DocumentationRelease.md` gained a new "Documentation Release 2.1" entry under Release History, explicitly marked Draft/prepared rather than Complete — only Phase 1 (Draft) of the Documentation Release Lifecycle has occurred; Phases 2 through 6 (Foundation Review through Release Tag) require Technical Director review before this release can be considered frozen or tagged.

No `cargo fmt`/`cargo check`/`cargo test` verification was performed this phase, since no Rust source file was touched — consistent with the explicit instruction to remain in architecture/documentation mode throughout.

Following Technical Director review, a "Relationship to Existing Subsystems" section was added to `EvidenceCollection.md` (explicit ownership statements for all five subsystems plus a no-bypass rule), and a documentation repository audit was performed, scoped strictly to Documentation Release 2.1. The audit found and corrected: `docs/README.md`'s Reading Order was missing the new `EvidenceCollection.md`; `Architecture.md`, `DataModel.md`, and `RuleEngine.md` each had a metadata table (version/status/date) left stale relative to their own updated Document Status sections; and `Architecture.md`'s System Overview, Dependency Rules, and Extensibility sections omitted Evidence Collection despite the Core Platform Components and Assessment Lifecycle sections already naming it. All corrected. One issue was found and deliberately left uncorrected: `TECHNICAL_DIRECTOR_HANDOFF_v2.1.md` speculatively suggested "GOV-007" for a future referential-integrity governance item; that number is now assigned to Evidence Collection implementation approval instead. Per this project's "historical snapshots are superseded, not rewritten" convention for handoff documents, this was flagged rather than edited. `docs/00-Governance.md`'s pre-existing, previously-flagged staleness (it does not reach the Technical Layer at all) was confirmed out of scope and left untouched.

Documentation Release 2.1 was then frozen: `EvidenceCollection.md` and the amended sections of `Architecture.md`/`DataModel.md`/`RuleEngine.md` moved from Draft/pending-review to Frozen; `docs/governance/DocumentationRelease.md`'s Documentation Release 2.1 entry updated to Complete (not tagged in source control, matching Documentation Release 2.0's own precedent).

---

### Sprint 3 Phase 4: Evidence Collection Implementation

Status:
Completed

Affected Crates:
- modiq-collection (new)
- modiq-engine
- apps/sandbox (Tauri command wiring, not a workspace member)

Affected Documents:
- GOVERNANCE.md (GOV-007 marked Resolved)

Notes:
Implemented the architectural boundary ADR-0008 established, authorized by the Technical Director following Documentation Release 2.1's freeze. Scope was deliberately the smallest slice that proves the boundary: a new `modiq-collection` crate (`InputDescriptor`, `InputDescriptorError`, `EvidenceCollector`), added to the root Cargo workspace, depending only on `modiq-runtime` — no other crate depends on it except `modiq-engine`, exactly matching ADR-0008's documented dependency direction. No ZIP parsing, XML parsing, or Lua inspection was implemented; no collector trait or plugin mechanism was introduced. `EvidenceCollector::collect` is deliberately infallible — the only representable failure (an empty descriptor) is rejected earlier, at `InputDescriptor::new` — since inventing a `Result`-returning collect path with no reachable error case would have been placeholder code; GOV-010 (Collection Error Model) remains open for when a real, I/O-capable collector actually needs one.

`modiq-engine::AssessmentService` gained a new, purely additive method, `execute_from_descriptor`, rather than a change to `execute` itself: GOV-008 (whether `execute`'s own signature should evolve) remained separately open, so implementation avoided that gate entirely rather than resolving it as a side effect. `execute_from_descriptor` resolves the Input Descriptor, invokes `EvidenceCollector`, and delegates to the existing, byte-for-byte-unchanged `execute` for the rest of the pipeline. `apps/sandbox`'s `create_assessment` command was updated to call it, removing the sandbox's last piece of direct Evidence construction; sandbox's `Cargo.toml` did not need a new dependency, since `modiq-collection` reaches it only transitively through `modiq-engine`, confirmed by inspection of its own separate workspace build.

GOV-007 (Evidence Collection Subsystem Implementation Approval) is now Resolved. GOV-008 (AssessmentService Public API Evolution), GOV-009 (Input Descriptor Ownership), and GOV-010 (Collection Error Model) remain open, untouched and un-prejudiced by this implementation.

`cargo fmt`, `cargo check --workspace`, and `cargo test --workspace` passed (106 tests, up from 95: 8 new in `modiq-collection`, 3 new in `modiq-engine`). Sandbox workspace independently verified, 3/3 passing, zero warnings in both workspaces.

---

### Sprint 3 Phase 5: First Real Evidence Collection

Status:
Completed

Affected Crates:
- modiq-collection
- modiq-engine
- apps/sandbox (Tauri command wiring and fixture files, not a workspace member)

Affected Documents:
- GOVERNANCE.md (reconciliation notes only — GOV-009/GOV-010 already Resolved in the preceding governance session)
- EvidenceCollection.md (reconciliation note only)
- CrateRoadmap.md

Notes:
Implemented the first real filesystem collector, realizing the architecture `PROPOSAL_FILESYSTEM_COLLECTION.md` proposed and a preceding governance-resolution session formalized (GOV-009, Assessment Input Ownership; GOV-010, Collection Error Model — both Resolved for the filesystem case ahead of this phase).

**Terminology reconciliation:** `InputDescriptor`/`InputDescriptorError` renamed to `AssessmentInput`/`AssessmentInputError` throughout `modiq-collection`, `modiq-engine`, and documentation cross-references — vocabulary alignment only, no behavior change.

**Assessment Input:** unchanged in shape from Phase 4 (still validates only non-emptiness at construction) — existence and accessibility are deliberately checked later, during collection itself, matching `EvidenceCollection.md`'s own account of why a non-existent path is an Inaccessible Input outcome, not an Invalid one.

**Filesystem collector:** `EvidenceCollector::collect` now inspects the real filesystem via `std::fs::symlink_metadata` (so the root is never followed if it is itself a symbolic link), dispatches on file vs. directory, and for directories recurses with entries sorted by filename at each level before processing — the explicit ordering `EvidenceCollection.md`'s Determinism Expectations require, since the OS provides none. Each discovered file or directory becomes one `FileStructureAnalysis` Evidence item located by its path relative to the given root. No file content is ever read.

**Error model:** a new `CollectionError` (Inaccessible, Unsupported) was added in `modiq-collection`, kept separate from `AssessmentInputError` (Invalid Input) since they represent genuinely different failure moments — construction vs. an actual collection attempt. A new `AssessmentExecutionError` in `modiq-engine` unifies both for `AssessmentService::execute_from_assessment_input` (itself renamed from `execute_from_descriptor`), since reconciling two crates' error types into one caller-facing result is Engine-orchestration's own job. Empty Collection remains `Ok(vec![])`, never an error.

**Atomicity:** required no additional code. `execute_from_assessment_input` constructs the AssessmentInput, then collects, then calls the existing, unchanged `execute` — using `?` at each step — so `Assessment::new` (inside `execute`) is only ever reached after collection has already succeeded. If collection fails at any depth of a directory traversal, the whole call returns `Err` and the partially-built local Evidence vector is simply never returned; no explicit atomicity mechanism was needed beyond this existing control-flow shape.

**Symbolic Link Policy:** a symbolic link as the AssessmentInput's own root is classified Unsupported (accepting it would require deciding whether to follow it, which Phase 5 does not do); a symbolic link encountered while traversing a directory is skipped — not followed, not recorded as Evidence. Both are tested directly (Unix-only, guarded with `#[cfg(unix)]`, since portable creation of symbolic links needs it).

**Sandbox:** updated to call `execute_from_assessment_input` against a new, fixed, checked-in fixture directory (`apps/sandbox/src-tauri/fixtures/sample-assessment-input/`, one file and one nested subdirectory with a file of its own) resolved via `CARGO_MANIFEST_DIR`, not the process's working directory. No file picker, drag-and-drop, or other input UI was implemented — explicitly out of scope. The sandbox now discovers 3 real Evidence items (previously 1 synthetic item); Rule Engine output is unaffected (still exactly one Finding and one Recommendation, since `RuleEngine::evaluate` has never varied its output count with Evidence count).

Tests added: `modiq-collection`'s `evidence_collector.rs` suite grew from 4 (synthetic-collector) to 8 (single file, nested directory tree with deterministic ordering, empty directory, nonexistent path, determinism across repeated calls, symlink-as-root, symlink-during-traversal, atomicity under a permission-denied nested directory) — `assessment_input.rs`'s 4 tests carried over unchanged except for the rename. `modiq-engine` gained 2 net new tests (real-filesystem success and a no-Assessment-on-failure assertion, alongside renamed empty-input and now-real nonexistent-path rejection tests). Sandbox's 3 existing tests were updated for the new fixture and its exact deterministic evidence ordering, not added to.

`cargo fmt`, `cargo check --workspace`, and `cargo test --workspace` passed (112 tests, up from 106: `modiq-collection` 8 → 12, `modiq-engine` 7 → 9 unit tests plus 3 unchanged integration tests). Sandbox workspace independently verified, 3/3 passing, zero warnings in both workspaces.

---

### Engineering Release 0.3

Status:
Completed

Affected Crates:
- (none — documentation only)

Affected Documents:
- docs/engineering/ENGINEERING_RELEASE_0.3.md (new)
- PROJECT_STATUS.md
- CHANGELOG.md
- docs/README.md
- CrateRoadmap.md

Notes:
Froze Sprint 3 (Phases 1–5) into a formal Engineering Release, consolidating five phases plus one interstitial governance-resolution session into a single permanent record: `docs/engineering/ENGINEERING_RELEASE_0.3.md`. Covers Executive Summary, Sprint 3 scope (delivered and deferred), major architectural and implementation accomplishments, governance completed (GOV-005, 006, 007, 009, 010), documentation completed (Documentation Release 2.1, ADR-0008, ADR-0009), testing growth (97 → 112), a Repository Maturity Assessment (Stable / Needs Monitoring / Needs Future Work per area), a full Crate Maturity Review across all nine crates, a four-category Technical Debt Review, a Sprint 3 Retrospective, Lessons Learned, Engineering Metrics, and a Recommendation naming a second real collector (most likely ZIP traversal) or CLI wiring as the next logical capability — without scoping either in detail, per instruction.

Performed a living-document reconciliation pass: `PROJECT_STATUS.md` (Current Milestone, Documentation Release Status, Current Focus all updated from stale Sprint 2 / Documentation Release 2.0 framing), `CHANGELOG.md` (added `[Sprint 3]`, `[Documentation Release 2.1]`, and `[Engineering Release 0.3]` entries, matching the existing per-milestone format exactly), `docs/README.md` (Current Documentation Status narrative, stale since before Sprint 2, corrected), and `CrateRoadmap.md` (Sprint 3 heading changed from "In Progress" to "Complete," a closing pointer to this release added, and a final revision history entry).

This release's central finding is procedural rather than architectural: the entire body of Sprint 3 Phase 5, the governance-resolution session that preceded it, and the Roadmap Review and Filesystem Collection proposal before that, remain uncommitted in the working tree as of this record. `ENGINEERING_RELEASE_0.3.md` names this directly in its Executive Summary, Repository Health, and Recommendation sections rather than treating the release as a clean baseline it does not yet have in git history.

No `cargo fmt`/`cargo check`/`cargo test` verification was performed this session, since no Rust source file was touched — consistent with this session's explicit documentation-only scope.

---

### Platform Validation: GOV-004 Resolved

Status:
Completed

Affected Crates:
- (none — documentation only)

Affected Documents:
- docs/engineering/GOVERNANCE.md
- docs/architecture/EngineAPI.md
- docs/architecture/Architecture.md
- docs/implementation/CrateRoadmap.md
- docs/adrs/0010-engine-orchestration-simplification.md (new)
- docs/adrs/README.md

Notes:
Opened the Platform Validation cycle following Engineering Release 0.3 with `PROPOSAL_PLATFORM_VALIDATION_REVIEW.md`, classifying accumulated architectural assumptions as Validated, Requiring Refinement, or Retirement Candidate. Evaluated the first refinement candidate, GOV-004 (Engine Service Granularity), in `PLATFORM_VALIDATION_GOV-004.md` — a technical evaluation only, presenting evidence without a recommendation. The Technical Director reviewed that evidence and reached a decision, formalized in `PROPOSAL_GOV-004.md` and approved.

GOV-004 is now Resolved: `AssessmentService` direct subsystem composition is the approved engine orchestration architecture; the internal `EngineAPI` service model (`KnowledgeService`, `ReportingService`, `RuleEvaluationService`, `VersionProfileService`) is retired, along with the mirrored `modiq-rules` scaffolding (`RuleSelector`, `EvidenceEvaluator`, `Explainability`, `Traceability`), as part of the same architectural simplification.

`EngineAPI.md` amended to v1.1.0 — the Service Overview and Service Relationships sections (five services, an internal diagram) replaced with a description of the Assessment Service, subsystem orchestration by direct composition, engine responsibilities, and dependency boundaries, cross-referencing `Architecture.md`'s System Overview diagram rather than duplicating it. `Architecture.md` received a one-sentence cross-reference addition to its Assessment Service component description (v1.1.2); no structural change, since its System Overview already described the architecture this decision confirmed. `CrateRoadmap.md`'s `modiq-engine` and `modiq-rules` maturity entries note the retirement, plus a new revision history entry (1.9.0). ADR-0010 (Engine Orchestration Simplification) records the decision permanently and is indexed in `docs/adrs/README.md`.

This is a governance and documentation resolution only. Deletion of the eight retired stub types is authorized as future implementation work by ADR-0010's own Status section, not performed here. No Rust source file was modified, no test was modified, and no crate's maturity level changed. `cargo fmt`/`cargo check`/`cargo test` were not run this session, consistent with its documentation-only scope; the eight stub types remain in source, compiling and unused, exactly as `PLATFORM_VALIDATION_GOV-004.md` found them.

---

### GOV-004 Implementation: Retired Scaffolding Removed

Status:
Completed

Affected Crates:
- modiq-engine
- modiq-rules

Affected Documents:
- docs/implementation/CrateRoadmap.md

Notes:
Implemented the architectural simplification ADR-0010 authorized and GOV-004 resolved. Deleted the eight stub types identified in `PLATFORM_VALIDATION_GOV-004.md` as never constructed or called anywhere in the workspace: `modiq-engine`'s `knowledge_service.rs`, `reporting_service.rs`, `rule_evaluation_service.rs`, `version_profile_service.rs`, and `modiq-rules`'s `selector.rs`, `evaluator.rs`, `explainability.rs`, `traceability.rs`, along with each file's `pub mod` and `pub use` declaration in its crate's `mod.rs`. No replacement type, trait, or abstraction was introduced — this removes scaffolding, it does not substitute for it.

`AssessmentService` and `RuleEngine` are otherwise unchanged: neither's public API, method bodies, or doc comments required modification, since neither ever referenced any of the eight deleted types.

`cargo fmt` made no further changes beyond the two `mod.rs` edits. `cargo check --workspace` and `cargo check` in `apps/sandbox/src-tauri` (its own, separate workspace) both passed with zero warnings. `cargo test --workspace` passed 112/112, identical in count and distribution to the pre-deletion baseline (`modiq-runtime` 82, `modiq-collection` 12, `modiq-engine` 9, `modiq-report` 3, `modiq-rules` 3); the Sandbox's own suite passed 3/3, unchanged. No `Cargo.lock` drift in either workspace. This confirms, rather than merely asserts, that the deleted types had zero behavioral footprint.

`GOVERNANCE.md` and ADR-0010 are not re-edited to reflect completion, consistent with this project's convention that accepted governance resolutions and ADRs are historical records of the decision, not living status trackers — this entry and `CrateRoadmap.md`'s revision history (1.10.0) carry the implementation-complete status instead, the same pattern already used when GOV-007's later implementation did not require re-editing ADR-0008.

---

### Platform Validation Phase 1: Closed

Status:
Completed

Affected Crates:
- (none — governance and documentation only)

Affected Documents:
- docs/engineering/GOVERNANCE.md
- docs/implementation/CrateRoadmap.md
- docs/governance/PROJECT_STATUS.md
- docs/governance/CHANGELOG.md

Notes:
The Technical Director completed review of the Platform Validation cycle opened by `PROPOSAL_PLATFORM_VALIDATION_REVIEW.md`. Two items were evaluated: GOV-004, approved and implemented (recorded in the two preceding log entries); GOV-008, reviewed against `PLATFORM_VALIDATION_GOV-008.md` and `PLATFORM_VALIDATION_EXECUTION_CONTRACT.md`'s implementation evidence and deferred — the Technical Director found that evidence insufficient to resolve it, authorized no architectural change, and confirmed the current `AssessmentService` execution contract (both entry points, `AssessmentInput`, `AssessmentReport`, and the public error model) as the approved platform boundary until future implementation provides additional evidence.

`GOVERNANCE.md`'s GOV-008 entry received one added paragraph recording this review outcome; its Status remains Open and its original Resolution text is unchanged, consistent with recording a review rather than a resolution. `CrateRoadmap.md` gained a new "Platform Validation Phase 1 — Complete" subsection and a revision history entry (1.11.0). `PROJECT_STATUS.md`'s dashboard table and Current Milestone/Current Focus sections were updated to reflect Platform Validation Phase 1 as the current completed milestone, and its stale "pending commit" Repository Status note — accurate before the Sprint 3 backlog was committed, stale since — was corrected in the same pass. `CHANGELOG.md` gained a new `[Platform Validation Phase 1]` entry.

The four evaluation and proposal documents this cycle produced (`PROPOSAL_PLATFORM_VALIDATION_REVIEW.md`, `PLATFORM_VALIDATION_GOV-004.md`, `PROPOSAL_GOV-004.md`, `PLATFORM_VALIDATION_GOV-008.md`, `PLATFORM_VALIDATION_EXECUTION_CONTRACT.md`) are not modified by this closeout, consistent with this project's convention that proposal and evaluation documents are retained as historical records of the review that preceded a decision, not rewritten to reflect the decision itself.

Architecture validated for continued roadmap execution. Next implementation milestone: ZIP / Archive Evidence Collection (`PROPOSAL_ZIP_EVIDENCE_COLLECTION.md`). No Rust source file was modified, no test was modified, and no governance item changed status (GOV-004 was already Resolved; GOV-008 remains Open).

---

### Sprint 4 Phase 1: Governance Preparation

Status:
Completed

Affected Crates:
- (none — governance and documentation only)

Affected Documents:
- docs/engineering/GOVERNANCE.md
- docs/engineering/PROPOSAL_GOV-011.md (new)

Notes:
Following the Technical Director's approval of `PROPOSAL_ZIP_EVIDENCE_COLLECTION.md` and the explicit-routing decision recorded in `SPRINT4_IMPLEMENTATION_PLAN.md`, opened GOV-011 (Archive Collection Model) in the Governance Register — Status Open, not Resolved — covering the four questions the Implementation Plan's Governance Prerequisites section named: malformed/corrupt archive categorization, duplicate entry name handling, resource limits, and the archive-format analog of the Symbolic Link Policy.

Drafted candidate answers to all four in `PROPOSAL_GOV-011.md`, following the same extend-the-existing-four-outcome-model approach GOV-009/GOV-010 already used for the filesystem case: malformed archives categorized as Unsupported Input (no new outcome); duplicate entry names permitted, each producing its own Evidence item; resource limits checked from archive metadata alone, with provisional (not final) numeric values pending Phase 2 calibration; a traversal-boundary policy mirroring the Symbolic Link Policy's shape (skip the offending entry, do not abort the archive), with the alternative (reject the whole archive) named explicitly as a real fork for Technical Director preference.

No archive-parsing dependency has been selected and no real archive has been read by this platform as of this entry — per `SPRINT4_IMPLEMENTATION_PLAN.md`'s own sequencing, these candidates are drafted ahead of Phase 2 (Boundary-Proving) and are explicitly subject to revision if Phase 2 finds any of them technically unachievable against a real dependency.

This phase performed no Rust changes and no architectural change. `AssessmentService` remains the sole orchestration boundary; no dispatcher, registry, provider, factory, trait hierarchy, or plugin mechanism was introduced or implied by this work. `cargo fmt`/`cargo check`/`cargo test` were not run this phase, since no Rust source file was touched — consistent with Phase 1's documentation-only scope per the Implementation Plan.

Per the implementation directive's reporting requirement, this phase's work concludes here; Phase 2 (Boundary-Proving) awaits confirmation before beginning.

---

### Sprint 4 Phase 1 (continued): Technical Director Decisions Recorded

Status:
Completed

Affected Crates:
- (none — governance only)

Affected Documents:
- docs/engineering/GOVERNANCE.md

Notes:
Recorded two Technical Director decisions in GOV-011's Resolution field: Question 4 (Archive Traversal / Zip Slip Policy) approved as drafted in `PROPOSAL_GOV-011.md` — normalize entry paths, skip an offending entry rather than rejecting the whole archive, terminate collection only for archives that cannot be read or parsed at all. A new Archive Metadata Policy also approved: archive metadata (timestamps, permissions, ownership, comments, host OS metadata, non-evidentiary compression metadata) SHALL NOT participate in Assessment Evidence. GOV-011 remains Open overall — Questions 1–3 are unaffected and still Pending.

---

### Sprint 4 Phase 2: Boundary-Proving (Investigation)

Status:
Completed

Affected Crates:
- (none — investigation performed entirely outside the workspace, in a throwaway scratchpad project; no crate in this repository was touched)

Affected Documents:
- (none — findings reported to the Technical Director directly; governance updates deferred pending review of this phase's findings, per `PROPOSAL_GOV-011.md`'s own stated interaction with Phase 2)

Notes:
Investigated the `zip` crate (v8.6.0) against nine constructed fixtures (well-formed multi-entry, duplicate names, Zip Slip traversal entries, an absolute-path entry, empty, truncated, non-ZIP-content, a 50MB-decompressing/~51KB-compressed "bomb-shaped" archive, and a 5,000-entry archive), entirely in a standalone Cargo project outside this repository — no production Rust was written, no crate was modified, `AssessmentService` was not touched, and no dispatch abstraction was introduced.

Confirmed: deterministic entry enumeration across repeated reads (central-directory/physical order, not auto-sorted — an explicit sort remains necessary, as already planned); malformed and non-archive input both fail cleanly via `Result::Err` at open time, no panic; per-entry uncompressed size, compressed size, and entry count are all available from metadata alone, empirically confirmed via timing (metadata enumeration ~1270x faster than full decompression of the same content); `enclosed_name()` correctly rejects genuine `..`-based traversal entries; the fields the new Archive Metadata Policy excludes (`last_modified`, `unix_mode`, `comment`) exist as distinct accessor methods, confirming the policy is straightforward to honor by simply never calling them.

One finding contradicts a Phase 1 candidate and is reported to the Technical Director rather than acted on: this dependency's default API silently collapses duplicate-named entries to last-write-wins at archive-open time — `archive.len()` reported 1 for a fixture independently confirmed (via `unzip -l`, Python's `zipfile`, and a raw central-directory-record count) to genuinely contain 2 entries named `dup.txt`. The earlier entry is not accessible through this crate's normal enumeration API. This invalidates Question 2's Phase 1 candidate ("each duplicate-named entry produces its own discrete Evidence item") as drafted. A second, smaller nuance: `enclosed_name()` sanitizes an absolute-path entry into a safe relative path rather than rejecting it outright (unlike genuine `..` traversal, which it does reject) — behavior not explicitly addressed by the just-approved Question 4 policy text.

No implementation was performed. No architectural boundary was crossed. Findings and a recommendation are reported to the Technical Director for review before Phase 3 may begin, per `SPRINT4_IMPLEMENTATION_PLAN.md`'s own gating.

---

### GOV-011 Resolved (Archive Collection Model)

Status:
Completed

Affected Crates:
- (none — governance and documentation only)

Affected Documents:
- docs/engineering/GOVERNANCE.md
- docs/architecture/EvidenceCollection.md
- docs/implementation/CrateRoadmap.md

Notes:
The Technical Director approved `PROPOSAL_GOV-011.md` in its entirety, resolving all four GOV-011 questions. GOV-011's Governance Register status changed from Open to Resolved, with full policy text recorded for each question: malformed/corrupt archives and resource-limit violations (entry count, compression ratio, checked from metadata alone) both categorized as the existing Unsupported Input outcome, no fifth outcome introduced; a Duplicate Archive Entry Policy adopted, recording detection of duplicate entries as an observable fact rather than silently resolving to last-write-wins or fabricating per-entry Evidence the collection mechanism cannot actually observe; an Archive Traversal Boundary Policy adopted, extending the existing Symbolic Link Policy's shape to archive entries, covering both relative (`..`-based) traversal and absolute-path entries — the latter determined independently of any archive-parsing dependency's own internal sanitization, per Sprint 4 Phase 2's finding that at least one candidate dependency sanitizes rather than rejects absolute-path entries; and an Archive Metadata Policy excluding timestamps, permissions, ownership, comments, host OS metadata, and non-evidentiary compression metadata from Assessment Evidence.

`EvidenceCollection.md` amended to v1.2.0: a new Archive-Specific Outcomes subsection under Collection Outcomes, a new Duplicate Archive Entry Policy section, and a new Archive Traversal Boundary Policy section (mirroring the Symbolic Link Policy's placement and shape) were added. The amendment is recorded explicitly in the document's own Document Status section, consistent with this project's non-silent-amendment discipline for Frozen specifications. `CrateRoadmap.md` gained a revision history entry (1.12.0).

Per the Technical Director's explicit direction, implementation-mechanism questions (the exact Rust representation for the Duplicate Archive Entry Policy's observable fact, the exact detection mechanism, Question 3's numeric thresholds, and the exact absolute-path check) do not block implementation and do not require further governance approval, provided implementation faithfully realizes the policy now resolved. `PROPOSAL_GOV-011.md`, `PROPOSAL_ZIP_EVIDENCE_COLLECTION.md`, `PLATFORM_VALIDATION_GOV-004.md`, `PLATFORM_VALIDATION_GOV-008.md`, and `PLATFORM_VALIDATION_EXECUTION_CONTRACT.md` are not modified by this resolution, consistent with this project's convention that proposal and evaluation documents remain historical records of the review that preceded a decision.

No Rust file was modified. No implementation was performed. Sprint 4 Phase 3 (Real Collector Implementation) is now unblocked per `SPRINT4_IMPLEMENTATION_PLAN.md`'s own gating.

---

### Sprint 4 Phase 3A: ZIP Archive Foundation

Status:
Completed

Affected Crates:
- modiq-collection

Affected Documents:
- (none — code and dependency manifests only)

Notes:
Implemented the first Rust code of Sprint 4: `ArchiveReader`, `ArchiveEntry`, and `ArchiveReadError` in a new `crates/modiq-collection/src/collection/archive_reader.rs`, registered in `collection/mod.rs`. This is a foundation type only, deliberately not yet a Collector (`EvidenceCollection.md`: Collector Contract) — it opens a location and deterministically enumerates structural entries (name, uncompressed size, file-vs-directory kind), and does not produce Evidence, detect duplicate entries, filter traversal-unsafe entries, or enforce resource limits. Those responsibilities, and integration with `AssessmentInput`/`AssessmentService`, remain for a later phase.

Added `zip` (v8.6.0, the dependency investigated during Phase 2 Boundary-Proving) to `[workspace.dependencies]` in the root `Cargo.toml`, and referenced it via `{ workspace = true }` in `modiq-collection/Cargo.toml` — following this project's existing shared-dependency convention (the same pattern already used for `thiserror`). This is `modiq-collection`'s second external dependency and the first archive-parsing dependency any domain crate in this platform has taken, as anticipated by `PROPOSAL_ZIP_EVIDENCE_COLLECTION.md`.

`ArchiveReadError` deliberately holds only a `path` field per variant (`Io`, `InvalidArchive`), not the underlying `std::io::Error` or `zip::result::ZipError` — matching `CollectionError`'s own existing shape exactly, for the same reason: keeps the type `Clone`/`PartialEq`/`Eq` and directly comparable in tests, consistent with this codebase's established error-design convention rather than introducing a new one.

`entries()` imposes an explicit lexicographic sort over whatever order the archive's own central directory returns, mirroring `EvidenceCollector`'s identical treatment of filesystem directory traversal — confirmed necessary by Phase 2's own finding that central-directory order is not automatically sorted.

Eight new tests, all passing on first run: well-formed archive opens successfully; a nonexistent path reports `ArchiveReadError::Io`; non-archive content and a truncated archive both report `ArchiveReadError::InvalidArchive`, no panic in either case; entries are discovered in deterministic (sorted) order regardless of write order, mirroring `EvidenceCollector`'s own determinism test shape; entry size and file-vs-directory kind are reported correctly; an archive with no entries returns an empty list; and enumeration is deterministic across two independent opens of the same archive. Test fixtures are constructed programmatically within each test using `zip::ZipWriter`, mirroring this codebase's existing preference for real, self-contained fixtures over checked-in binary blobs.

`cargo fmt`, `cargo check --workspace`, and `cargo test --workspace` all pass cleanly: root workspace test count grew from 112 to 120 (`modiq-collection` 12 → 20; every other crate unchanged). `apps/sandbox/src-tauri`'s own separate workspace — which depends on `modiq-collection` transitively through `modiq-engine` — was independently verified: `cargo fmt --check`, `cargo check`, and `cargo test` all pass cleanly, 3/3 tests unchanged, confirming zero behavioral impact on the one real application, exactly as expected since `AssessmentService` and `AssessmentInput` were not touched.

No architectural boundary was crossed: no Evidence generation, no duplicate detection, no traversal filtering, no resource limits, no `AssessmentService` integration, no `AssessmentInput` modification, no `EvidenceCollector` change, no dispatcher abstraction, no new crate. Sprint 4 Phase 3B (layering Evidence production, the Duplicate Archive Entry Policy, the Archive Traversal Boundary Policy, resource limits, and eventual `AssessmentService` routing on top of this foundation) remains to be scoped and authorized separately.

---

### Sprint 4 Phase 3B: Archive Evidence Generation

Status:
Completed

Affected Crates:
- modiq-collection

Affected Documents:
- (none — code only)

Notes:
Implemented `ArchiveEvidenceBuilder` in a new `crates/modiq-collection/src/collection/archive_evidence.rs`, registered in `collection/mod.rs`, transforming `ArchiveEntry` values (from Phase 3A's `ArchiveReader`) into `modiq_runtime::assessment::Evidence`. This remains a transformation step only, not yet a Collector — no duplicate detection, no traversal filtering, no resource limits, no `AssessmentService`/`AssessmentInput` integration, matching Phase 3B's explicit scope.

Reused the existing Evidence pattern exactly: `Evidence::with_location(EvidenceCategory::FileStructureAnalysis, description, entry.name())`, distinguishing file and directory entries only by description text ("File discovered during archive collection." / "Directory discovered during archive collection."), mirroring `EvidenceCollector`'s identical filesystem-case wording. `EvidenceCategory::FileStructureAnalysis` is reused unchanged, confirming the expectation `SPRINT4_IMPLEMENTATION_PLAN.md` recorded — no `modiq-runtime` change was needed or made. `build()` maps over its input in the given order, preserving `ArchiveReader::entries()`'s already-deterministic (sorted) ordering rather than re-deriving it.

Compliance with the Archive Metadata Policy (GOV-011) is structural, not conventional: `ArchiveEntry` (Phase 3A) exposes only `name`, `size`, and `is_dir` — no timestamp, permission, ownership, or comment field exists anywhere in the type to read in the first place — and `build()` additionally never reads `size` at all, since `Evidence` has no corresponding field. A dedicated test (`build_does_not_use_entry_size_or_any_excluded_metadata`) confirms two entries differing only in size produce identical Evidence content.

Added a small, `#[cfg(test)]`-gated `pub(crate)` constructor to `ArchiveEntry` (`archive_reader.rs`) so Phase 3B's tests could construct entries directly and test the new transformation logic in isolation, without real archive I/O for every case — Phase 3A's own tests already cover `ArchiveReader`'s real I/O behavior exhaustively, so this avoids duplicating that coverage. This is additive and test-only; `ArchiveReader`'s existing behavior, signatures, and production code path (`entries()` remains the only non-test constructor of a real `ArchiveEntry`) are unchanged — not a redesign.

Six new tests, all passing on first run: empty input produces no Evidence; one Evidence item is produced per entry, in the given order; every item carries `FileStructureAnalysis`; file vs. directory entries get distinct description text; entry size never affects output; and building the same input twice is deterministic in content (category, description, location) while each Evidence item freshly assigns its own identity, matching this platform's universal determinism-by-content-not-identity convention.

`cargo fmt`, `cargo check --workspace`, and `cargo test --workspace` all pass cleanly: root workspace test count grew from 120 to 126 (`modiq-collection` 20 → 26; every other crate unchanged). No new dependency was needed — `Cargo.lock` is unchanged in both workspaces. `apps/sandbox/src-tauri` independently reverified: `cargo fmt --check`, `cargo check`, `cargo test` all pass cleanly, 3/3 unchanged.

No architectural boundary was crossed: no duplicate detection, no traversal filtering, no resource limits, no `AssessmentService` integration, no `AssessmentInput` modification, no `ArchiveReader` redesign, no `EvidenceCollector` redesign, no dispatcher abstraction, no new crate. Sprint 4 Phase 3C (the Duplicate Archive Entry Policy, the Archive Traversal Boundary Policy, resource limits, and eventual `AssessmentService` routing) remains to be scoped and authorized separately.

---

## 2026-07-20

### Sprint 4 Phase 3C: Duplicate Detection, Traversal Filtering, and Resource Limits

Status:
Completed

Affected Crates:
- modiq-collection
- modiq-runtime

Affected Documents:
- EvidenceCollection.md
- Glossary.md

Notes:
Preceded by a dedicated Architecture Review (`PROPOSAL_GOV-011_DUPLICATE_REPRESENTATION.md`), presented for Technical Director review before any Rust change, evaluating three candidate representations for GOV-011 Question 2's duplicate-entry detection fact and recommending one. Approved, with one refinement requested: name the new category for the semantic class of the observation, not the collection mechanism that happened to observe it. `EvidenceCategory::StructuralDuplication` (not `DuplicateArchiveEntry`) was chosen on that basis — a review of `evidence_category.rs`'s and `Glossary.md`'s existing naming (`XmlInspection`, `FileStructureAnalysis`, etc., all named for a domain of content, never for a mechanism) confirmed no existing category is archive-specific, and a mechanism-named category would have been the only one that was.

Implemented `ArchiveCollector` (new, `crates/modiq-collection/src/collection/archive_collector.rs`), assembling `ArchiveReader` (Phase 3A) and `ArchiveEvidenceBuilder` (Phase 3B) with all three of GOV-011's remaining Phase 3C policies into one Collector with `EvidenceCollector`'s own `Result<Vec<Evidence>, CollectionError>` shape. Not yet reachable from `AssessmentService` — routing is Phase 3D, out of scope here, matching Phase 3A/3B's own precedent of leaving integration to a later, separately authorized phase.

**Resource limits (Question 3):** entry count and per-entry compression ratio (uncompressed ÷ compressed size) both checked from the archive's central directory metadata alone, before any content is decompressed; either exceeding a bound is Unsupported Input. Required a small addition to `ArchiveReader` (`entry_sizes()`, `pub(crate)`) to read compressed size, which the Archive Metadata Policy deliberately keeps off `ArchiveEntry` and every public accessor — this new method is not part of that public surface either. Numeric thresholds (50,000 entries; 10,000:1 ratio) are implementation detail per GOV-011's own resolution, chosen with headroom above Phase 2's measured ~1,029:1 ratio for an ordinary, non-malicious fixture.

**Archive Traversal Boundary Policy (Question 4):** each entry's raw, as-stored name (`ArchiveEntry::name()`, never a dependency's sanitized representation) is checked for an absolute-path form (leading `/` or `\`, or a Windows drive-qualified prefix) and for path-component normalization that would resolve outside the archive's own root. An invalid entry is skipped — not recorded as Evidence — without aborting collection of the remaining entries.

**Duplicate Archive Entry Policy (Question 2):** detection re-reads the archive as a sequential stream of local file headers (`zip::read::read_zipfile_from_stream`), independent of the central-directory-based `ArchiveReader::entries()`. Confirmed directly (`archive_reader.rs`'s `entries_collapses_duplicate_entry_names_to_one` test, and this phase's own `zip::ZipWriter` behavior encountered while writing tests) that the underlying `zip` crate's central directory is keyed by name internally (`IndexMap<Box<[u8]>, ZipFileData>`), collapsing duplicate names to one last-write-wins result before `entries()` ever sees more than one — exactly Phase 2's original finding, reconfirmed against the pinned dependency version rather than assumed from the earlier standalone investigation. `ZipWriter` itself now rejects duplicate filenames at write time (`insert_file_data`'s "Duplicate filename" check), which did not exist as a testable obstacle during Phase 2's own external investigation; Phase 3C's test fixtures for duplicate names, traversal violations, and fabricated resource-limit values were therefore hand-built at the byte level (`write_raw_archive` in `archive_collector.rs`'s test module, and a smaller equivalent added to `archive_reader.rs`'s), rather than through `ZipWriter`, so tests remain fully self-contained and deterministic rather than depending on a compression algorithm's actual achieved ratio for any given input.

Detected duplication is represented as exactly one `Evidence` item per archive (never one per duplicately-named entry, consistent with GOV-011's prohibition on fabricating Evidence for an entry that cannot actually be observed), tagged `EvidenceCategory::StructuralDuplication`, distinct from the per-entry `FileStructureAnalysis` items `ArchiveEvidenceBuilder` still produces for every entry that survives traversal filtering — including the one surviving entry from a duplicately-named pair.

Amended `EvidenceCollection.md` to v1.3.0, recording all three policies as implemented, not merely resolved, and the `StructuralDuplication` representation choice, with rationale for why it is kept distinct from `FileStructureAnalysis`. Added "Structural duplication" to `Glossary.md`'s Evidence enumeration, alongside the platform's other six categories.

Seventeen new or revised tests, all passing on first run after two fixes (below): one archive_reader.rs test rewritten to use the new raw-byte fixture builder (`ZipWriter`'s new duplicate-filename rejection made the original construction impossible); one archive_collector.rs test failure traced to the empty-archive edge case, where an archive with zero entries has no central directory header for the streaming duplicate-detection pass to stop at — fixed by skipping that pass entirely when `entries()` is empty, since zero entries cannot contain a duplicate name regardless. `ArchiveCollector`'s own sixteen tests cover: valid entries producing deterministic Evidence; zero-entry (Empty Collection) archives; inaccessible and malformed-archive paths; absolute Unix and Windows-drive-qualified paths skipped; relative traversal escaping root skipped, while traversal that stays within root (`a/../b.txt`) is correctly allowed; one invalid entry never aborting the rest of the archive; duplicate names detected and represented, and correctly absent when no names repeat; both resource-limit conditions (entry count via a real 50,001-entry archive; compression ratio via fabricated central-directory metadata, including the zero-compressed-size edge case) exceeded and within-limit; and determinism across repeated calls.

`cargo fmt`, `cargo check --workspace`, and `cargo test --workspace` all pass cleanly: root workspace test count grew from 126 to 143 (`modiq-collection` 26 → 43; every other crate unchanged). `Cargo.lock` is unchanged — no new dependency; the streaming-read facility used for duplicate detection is part of the already-pinned `zip` v8.6.0. `apps/sandbox/src-tauri` independently reverified: `cargo fmt --check`, `cargo check`, `cargo test` all pass cleanly, 3/3 unchanged, confirming zero behavioral impact on the one real application — expected, since `AssessmentService` and `AssessmentInput` were not touched.

No architectural boundary was crossed beyond what was explicitly authorized: no `AssessmentService` integration, no `AssessmentInput` modification, no dispatcher abstraction, no new crate, no new external dependency. `docs/engineering/GOVERNANCE.md`'s GOV-011 entry was not modified — its existing resolution already states that implementation-mechanism questions (including Question 2's representation) do not require further governance approval, and already points to `EvidenceCollection.md` as the record of implementation detail. Sprint 4 Phase 3D (`AssessmentService` integration, routing, Sandbox exercise, further documentation reconciliation) remains to be scoped and authorized separately.

---

## 2026-07-20

### Sprint 4 Phase 3D: AssessmentService Archive Routing

Status:
Completed

Affected Crates:
- modiq-engine

Affected Documents:
- EvidenceCollection.md
- CrateRoadmap.md

Notes:
Wired `ArchiveCollector` (Phase 3C) into `AssessmentService::execute_from_assessment_input`, completing the platform's first end-to-end archive assessment path (`AssessmentService` → `ArchiveCollector` → Evidence → the existing, unchanged assessment pipeline → `AssessmentReport`), per Technical Director authorization.

**Routing mechanism:** one new private method, `AssessmentService::is_archive_location(&str) -> bool`, called from one `if`/`else` inline in `execute_from_assessment_input` immediately after `AssessmentInput` construction. Returns true exactly when the input's value ends in `.zip`, case-insensitively; `ArchiveCollector` is called for that branch, `EvidenceCollector` for every other value, unchanged from before this phase. `SPRINT4_IMPLEMENTATION_PLAN.md`'s Approved Routing & Collector Shape section explicitly left the determination method (extension check, content-signature check, or a combination) as an implementation decision; an extension check was chosen as the simpler of the two named options, with no correctness gap it introduces: a `*.zip`-named location that is not actually a well-formed archive is not misclassified or silently mishandled, since `ArchiveCollector` already reports Inaccessible/Unsupported for exactly that case — the same discipline `EvidenceCollector` already applies for its own unsupported inputs. No trait, registry, dispatcher, or common supertype was introduced; both collectors remain independently invoked, by name, from this one decision point, matching every Technical Director decision on record for this question.

`execute` (the `Vec<Evidence>`-accepting entry point) was not touched, per `SPRINT4_IMPLEMENTATION_PLAN.md` Phase 4's own completion criteria. `EvidenceCollector`'s own behavior, and every existing filesystem-path test, are unchanged and still pass unmodified.

Added `zip` as a `[dev-dependencies]` entry to `modiq-engine`'s `Cargo.toml`, to build real archive fixtures (`ZipWriter`) in `AssessmentService`'s own tests, mirroring `archive_reader.rs`'s and `archive_collector.rs`'s existing test-fixture pattern. Not a new crate for the workspace — `zip` was already an approved `[workspace.dependencies]` entry since Phase 3A; this only extends its existing dev-only usage to a second crate. Production code in `modiq-engine` does not depend on `zip` directly; only `modiq-collection` does.

Seven new tests: two pure unit tests of `is_archive_location` (case-insensitivity; rejection of non-`.zip` values, including a `mod.zip.bak` case ensuring only a true trailing `.zip` suffix matches); a real end-to-end archive assessment producing the expected Evidence, Finding, and Recommendation; case-insensitive extension matching exercised through the real pipeline (`MOD.ZIP`); a malformed `*.zip` file correctly surfacing `CollectionError::Unsupported` through `AssessmentExecutionError`; a plain non-`.zip` file confirming the filesystem path is unaffected by the new routing; and determinism across two identical archive assessments. All sixteen `modiq-engine` unit tests (nine existing, seven new) and the three unchanged `tests/end_to_end.rs` integration tests pass.

While synchronizing `EvidenceCollection.md`, found its top metadata table's Version field still read 1.2.0 — one revision behind the Document Status footer's own 1.3.0, which Phase 3C's amendment had correctly updated. The footer's content was right; the table simply was not updated alongside it. Corrected both fields together to 1.4.0 as part of this phase's own amendment, rather than leaving the inconsistency in place or silently claiming this phase was the first to touch the table.

Amended `EvidenceCollection.md`'s Collector Contract section: the sentence stating that Collector composition/dispatch "remains an open implementation question, deliberately deferred until a second concrete Collector exists" is now factually superseded — a second concrete Collector exists and the question is resolved (explicit inline routing, no abstraction). Replaced with a short "Composition (resolved, Sprint 4 Phase 3D)" note recording the resolution and explicitly leaving open whether the same shape holds for a hypothetical third Collector, rather than asserting that preemptively.

`cargo fmt`, `cargo check --workspace`, and `cargo test --workspace` all pass cleanly: root workspace test count grew from 143 to 150 (`modiq-engine` 9 → 16; every other crate unchanged). `Cargo.lock` gained no new package — `zip` was already present in the lockfile from `modiq-collection`'s existing dependency; only its dev-dependency edge for `modiq-engine` is new. `apps/sandbox/src-tauri` independently reverified: `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` all pass cleanly, 3/3 unchanged — expected, since the Sandbox's own filesystem-path usage of `execute_from_assessment_input` is unaffected by the new branch.

No architectural boundary was crossed beyond what was explicitly authorized: no dispatcher, registry, trait, or plugin mechanism was introduced; `execute`'s signature and behavior are unchanged; `EvidenceCollector`'s own code was not modified. No governance document was modified — this phase implemented a decision the Technical Director had already made (`SPRINT4_IMPLEMENTATION_PLAN.md`'s Explicit Routing decision), it did not make a new one. A Sandbox exercise of the archive path specifically (a checked-in `.zip` fixture driven through the real Tauri application, mirroring Sprint 3 Phase 5's own filesystem-case precedent) was not part of this phase's authorized deliverables and remains unimplemented; the routing itself was verified through `modiq-engine`'s own real-I/O tests instead.

---

## 2026-07-21

### Sprint 4 Closeout: Repository Reconciliation, Sandbox Archive Validation, Retrospective

Status:
Completed

Affected Crates:
- (none — Sandbox test/fixture additions only; no production crate code changed)

Affected Documents:
- docs/governance/PROJECT_STATUS.md
- docs/governance/CHANGELOG.md
- docs/implementation/CrateRoadmap.md
- docs/engineering/SPRINT4_IMPLEMENTATION_PLAN.md
- docs/README.md
- docs/engineering/ENGINEERING_RELEASE_0.4.md (new)

Notes:
Formal Sprint 4 closeout, per Technical Director authorization: repository reconciliation, a documentation consistency audit, Sandbox archive validation, an engineering retrospective, and a Sprint 4 Completion Report — the last two combined into `ENGINEERING_RELEASE_0.4.md`, mirroring `ENGINEERING_RELEASE_0.3.md`'s own precedent of housing its Sprint's retrospective and lessons-learned content inside the release document itself, rather than as a separate file.

**Repository Reconciliation:** confirmed all four implementation phases (3A–3D) complete against `SPRINT4_IMPLEMENTATION_PLAN.md`'s own Completion Checklist, verified item by item against current repository state rather than carried over from any phase's own self-report (150/150 root tests, 6/6 Sandbox tests, both independently re-run). One checklist item — the dependency-evaluation criteria for `zip` — had not been formally recorded at Phase 2 time (only technical/security criteria were); evaluated now: `license = "MIT"`, compatible with this workspace's own license, confirmed by direct inspection of the pinned dependency's `Cargo.toml`. One item remains genuinely open, not merely uncompleted: whether the explicit-routing decision (Phase 3D) warrants a standalone ADR, a question `SPRINT4_IMPLEMENTATION_PLAN.md` itself named as a Technical Director confirmation point at Sprint close — left unchecked and raised for review rather than decided by Engineering.

**Documentation Audit finding:** `docs/governance/PROJECT_STATUS.md` and `docs/governance/CHANGELOG.md` — this repository's designated living status documents — had not been updated since before Sprint 4 began, despite `Last Updated: 2026-07-20` on `PROJECT_STATUS.md` (the date was current; the content was not). Both still described "Platform Validation Phase 1 — Complete" as the current milestone and named ZIP/Archive Evidence Collection as merely "proposed, pending Technical Director review." This is the identical staleness pattern `ENGINEERING_RELEASE_0.3.md` flagged after Sprint 3 ("has gone stale after every release checkpoint so far... structurally unaddressed") — it recurred, unchanged, across Sprint 4. Corrected: both documents' header fields, a new `## Sprint 4 — Complete` narrative section in `PROJECT_STATUS.md` and a new `# [Sprint 4]` entry in `CHANGELOG.md`, both mirroring their own established per-Sprint structure exactly (no new section types invented). `docs/README.md`'s one stale Engineering Release cross-reference (still naming 0.3) corrected. `docs/implementation/CrateRoadmap.md` gained a `## Sprint 4 — Complete` narrative paragraph, matching its existing Sprint 1/2/3 pattern, in addition to the revision-history rows and crate-table update already added during Phase 3D.

**Also found, and deliberately not corrected:** `docs/governance/ROADMAP.md` and `docs/governance/EngineeringGuide.md` are both stale since 2026-07-16 (Sprint 0/1 era) — predating Sprint 4 entirely, not named in this Closeout's documentation-audit scope, and requiring a larger content judgment (ROADMAP.md's own phase model has not tracked Sprint numbering since Sprint 1) that this Closeout's "consistency rather than rewriting" charter does not cover. `CrateRoadmap.md`'s "Exit Criteria" section has never had a Sprint 3 or Sprint 4 entry — a pre-existing gap, not created asymmetrically for Sprint 4 alone. Both are named here for visibility, not silently left out of the record.

**Sandbox Archive Validation:** added a checked-in archive fixture (`apps/sandbox/src-tauri/fixtures/sample-archive-input.zip`, mirroring `sample-assessment-input/`'s own structure — one top-level file, one subdirectory, one nested file) and three new tests in the Sandbox's own workspace, calling `AssessmentService::execute_from_assessment_input` directly against it — the exact production entry point `create_assessment` itself uses, not a reimplementation. Confirmed directly: `.zip` routing selects `ArchiveCollector` and produces the expected Evidence/Finding/Recommendation counts; archive Evidence is correctly categorized and described as archive-collection output; the pre-existing directory fixture path is unaffected (a dedicated regression-guard test), still describing its Evidence as filesystem-collection output. No new `#[tauri::command]` or IPC surface was added — the new fixture and tests are `#[cfg(test)]`-only, consistent with the Sandbox's standing no-file-picker, no-new-input-mechanism constraint. This exercises the real production code path through the Sandbox's own separate workspace and build, not a launched, interacted-with GUI window — no display is available in this environment, and this limitation is stated plainly rather than implied otherwise.

**Retrospective and Completion Report:** produced as `ENGINEERING_RELEASE_0.4.md`, following `ENGINEERING_RELEASE_0.3.md`'s exact section structure (no new structure introduced): Executive Summary, Scope, Major Architectural/Implementation Accomplishments, Governance Completed, Documentation Completed, Testing Growth, Repository Maturity Assessment, Crate Maturity Review, Technical Debt Review, Sprint 4 Retrospective, Remaining Risks, Lessons Learned, Engineering Metrics, Repository Timeline, Recommendation.

`cargo fmt`, `cargo check --workspace`, and `cargo test --workspace` all pass cleanly: root workspace test count unchanged at 150 (no production code changed this session). `apps/sandbox/src-tauri`: `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` all pass cleanly, Sandbox test count grew from 3 to 6 (the three new archive-validation/regression tests).

No architectural change was made or proposed: no new capability, no new abstraction, no scope expansion, no Sprint 5 work begun. Two items are raised for Technical Director review rather than decided unilaterally: the standalone-ADR question for explicit routing (resolved shortly after this entry — see below), and — separately, not a decision but an observation — whether documentation staleness recurring identically across two consecutive Sprints warrants a structural process change (a per-phase or per-commit reconciliation habit) rather than the per-Sprint-close catch this Closeout itself performed.

---

## 2026-07-21

### Technical Director Review: Explicit-Routing ADR Question Closed

Status:
Completed

Affected Crates:
- (none)

Affected Documents:
- docs/engineering/SPRINT4_IMPLEMENTATION_PLAN.md
- docs/engineering/ENGINEERING_RELEASE_0.4.md
- docs/governance/CHANGELOG.md

Notes:
The one item Sprint 4 Closeout's Repository Reconciliation left genuinely open — whether `AssessmentService`'s explicit archive-vs-filesystem routing (Phase 3D) warrants a standalone ADR, mirroring ADR-0010's treatment of GOV-004 — has been reviewed and closed by the Technical Director: explicit routing remains an implementation decision, not a standalone architectural principle. No ADR is created.

`SPRINT4_IMPLEMENTATION_PLAN.md`'s Completion Checklist item is now checked, with the outcome recorded rather than left as a bare checkmark; its Documentation Updates Expected section, which originally named this as plausible, is amended in place to record the confirmed outcome. `ENGINEERING_RELEASE_0.4.md`'s seven references to this question (Governing ADRs header field, Deliberately Not Delivered, Governance Completed, Documentation Completed, Technical Debt Review's Deferred Enhancements, Sprint 4 Retrospective, and Recommendation) are each updated in place rather than left inconsistent with the resolution — Recommendation now names only one item (documentation-staleness process fix) as needing early Sprint 5 attention, not two. `CHANGELOG.md`'s corresponding Deferred bullet is removed as no longer accurate; a duplicated resource-limit-threshold bullet introduced by that edit was caught and removed in the same pass.

No architectural change was made: this entry records a decision, not an implementation. `docs/adrs/` gains no new file. Sprint 4 is now fully closed with zero open items from its own Closeout report.

---

### Technical Director Review: Sprint 4 Closeout Approved; Sprint 4 Officially Closed

Status:
Completed

Affected Crates:
- (none)

Affected Documents:
- (none — this entry is the record; no other document required a further change as a result of this approval)

Notes:
The Technical Director reviewed and accepted the full Sprint 4 Closeout: repository reconciliation confirmed Sprint 4 objectives complete, documentation found to accurately reflect repository state, and Sandbox validation accepted as demonstrating the completed archive assessment capability.

Two decisions recorded:

1. **Explicit-routing ADR — reconfirmed declined**, consistent with the prior entry above: explicit routing remains an implementation decision within the existing architectural framework, not a standalone architectural principle.
2. **Documentation staleness — accepted as a process-improvement item, not an unresolved defect.** The Technical Director's direction: future sprints should formally include repository reconciliation and documentation audit as part of sprint closure, rather than as a catch-up performed only when explicitly scoped after the fact (as this Closeout itself was). This is a standing process expectation for future sprint-closing sessions, not an action item against Sprint 4's own record, which is now complete.

**Sprint 4 is officially closed**, with zero open items from its own Closeout report. Recorded here as the formal closing event; the underlying reconciliation, validation, and retrospective content lives in `ENGINEERING_RELEASE_0.4.md` and the entries above, not repeated here.

---

## 2026-07-21

### Sprint 5 Phase 1: GOV-012 Approved; GOV-013 Opened

Status:
Completed

Affected Crates:
- (none — governance and specification drafting only; no Rust code written)

Affected Documents:
- docs/engineering/SPRINT5_IMPLEMENTATION_PLAN.md
- docs/engineering/GOV-012_AND_FINDINGSEVERITY_PREPARATION.md (new)

Notes:
Sprint 5 authorized for planning only, targeting the assessment intelligence layer (Rule Engine depth) rather than a third Evidence Collector, per Technical Director direction. `SPRINT5_IMPLEMENTATION_PLAN.md` drafted, mirroring `SPRINT4_IMPLEMENTATION_PLAN.md`'s structure, naming five Design Questions and one candidate governance item (GOV-012, Rule Evaluation Model) rather than presuming answers.

**GOV-012 approved by Technical Director in full:** `RuleEngine::evaluate` returns `Vec<RuleOutcome>`; Rules are ordered by explicit declaration order, never Evidence arrival order; Rules compose independently, with no suppression model between a specific Rule and the existing generic one. Formal Governance Register entry drafted (`GOV-012_AND_FINDINGSEVERITY_PREPARATION.md`, Part 1), staged for a final review pass before insertion into `GOVERNANCE.md`, mirroring the two-step discipline GOV-011 followed (`PROPOSAL_GOV-011.md` drafted the resolution text; `GOVERNANCE.md` was amended only after review).

**`FindingSeverity` semantic definitions drafted** (Part 2) — the first time this project has defined what `Error`/`Warning`/`Informational`/`BestPractice` actually mean; neither `Glossary.md` nor `DataModel.md` previously did, and the taxonomy had been exercised by only one variant (`Informational`) in three Sprints of use.

**GOV-013 opened, not by implementation surfacing it independently, but by Technical Director review of those definitions:** `BestPractice` does not sit on the same ordered "how urgent is this" axis `Error`/`Warning`/`Informational` do — it classifies what *kind* of observation a Finding represents, orthogonal to severity. Technical Director decision, recorded rather than acted on: `FindingSeverity` remains unchanged for Sprint 5; the model is provisionally accepted, not confirmed correct; the question stays Open, to be revisited once the Rule Engine has multiple concrete Rules operating in practice — the same evidence-based resolution discipline GOV-004 and GOV-011 both already applied, a concrete forcing function should justify a model change, not the reverse. Formal entry drafted (Part 3), Status Open, ready for insertion into `GOVERNANCE.md` immediately following GOV-012.

`cargo fmt`, `cargo check --workspace`, and `cargo test --workspace` all pass cleanly, unchanged at 150/150 — no production code was touched this session; this entry records governance and specification drafting only.

No architectural change was made: `FindingSeverity`'s shape in `modiq-runtime` is unchanged.

**Update, following Technical Director approval:** GOV-012 and GOV-013 inserted verbatim into `GOVERNANCE.md`'s Governance Register, immediately following GOV-011, exactly as drafted. The `FindingSeverity` semantic definitions inserted into `DataModel.md` as a new `### Finding Severity` subsection under `## Finding`, recorded explicitly as the platform's provisional interpretation (not a permanent claim), with a direct cross-reference to GOV-013. `DataModel.md` amended 1.0.1 → 1.1.0; Document Status footer updated to record the amendment, matching the frozen-document-amendment discipline `EvidenceCollection.md` established. Sprint 5 Phase 2 is now authorized and follows in the next entry.

---

### Sprint 5 Phase 2: Structural Duplication Rule

Status:
Completed

Affected Crates:
- modiq-rules

Affected Documents:
- (none — code only; GOV-012, GOV-013, and the FindingSeverity definitions were already recorded in the prior entry)

Notes:
Implemented `StructuralDuplicationRule` (new, `crates/modiq-rules/src/rules/structural_duplication_rule.rs`), Sprint 5's second concrete Rule, per Technical Director authorization. Evaluates only `EvidenceCategory::StructuralDuplication` Evidence — every other category is ignored by this Rule, consistent with GOV-012 Question 3's independent-composition resolution. Assigns `FindingSeverity::Warning` per `DataModel.md`'s newly-recorded Finding Severity definitions: a well-formed archive containing duplicate entry names is a genuine reliability concern (which entry a reader actually extracts is not guaranteed by the archive format), not conclusive proof of breakage, so `Error` would overstate what the Evidence establishes.

A unit-struct-with-method shape (`pub struct StructuralDuplicationRule; impl StructuralDuplicationRule { pub fn evaluate(...) }`), matching every other capability unit in this codebase (`EvidenceCollector`, `ArchiveCollector`, `ArchiveEvidenceBuilder`, `RuleEngine` itself) — not a bare free function. The first attempt used a bare `pub(crate) fn`, which triggered a `dead_code` warning under `cargo check` once written, since nothing outside its own tests called it yet (Phase 3, the multi-Rule dispatch wiring, is not yet authorized). Restructuring it as a `pub` unit struct resolved this the same way Sprint 4 Phase 3A/3B's `ArchiveReader`/`ArchiveEvidenceBuilder` never triggered the warning in the first place: public API surface is exempt from dead-code analysis, and this is genuinely public, tested, real capability, not yet wired into an orchestrator — the same state Phase 3A/3B were in before Phase 3D existed.

Deliberately scoped to Phase 2 only, per Technical Director direction not to expand scope: `RuleEngine::evaluate`'s own signature and behavior are completely unchanged; `StructuralDuplicationRule` is not called from anywhere in production code. Six new tests cover every reachable outcome: empty Evidence; non-matching Evidence only; one matching item; a matching item alongside a non-matching one (confirming only the matching item's id is referenced, not the whole Evidence set); more than one matching item (both referenced); and determinism across repeated calls with identical input.

`cargo fmt`, `cargo check --workspace`, and `cargo test --workspace` all pass cleanly, zero warnings: root workspace test count grew from 150 to 156 (`modiq-rules` 3 → 9; every other crate unchanged). `apps/sandbox/src-tauri` independently reverified: `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` all pass cleanly, 6/6 unchanged — expected, since neither `RuleEngine` nor `AssessmentService` was touched.

No architectural boundary was crossed beyond what was explicitly authorized: no trait, registry, factory, or plugin mechanism; no change to `RuleEngine::evaluate`'s return type (that is Phase 3); no `modiq-engine` change; no `modiq-report` change (Design Question 4's investigation and any action on it remain deferred, per Technical Director direction). Sprint 5 Phase 3 (Multi-Rule Evaluation Assembly — wiring `StructuralDuplicationRule` and the existing generic Rule together behind `RuleEngine::evaluate`'s new `Vec<RuleOutcome>` return shape) remains to be authorized separately.

---

### Sprint 5 Phase 3: Multi-Rule Evaluation Assembly

Status:
Completed

Affected Crates:
- modiq-rules
- modiq-engine

Affected Documents:
- (none — code only; GOV-012 already recorded the policy this phase implements)

Notes:
Implemented `RuleEngine::evaluate`'s real multi-Rule dispatch, per Technical Director authorization following Phase 2's acceptance.

Extracted the original Sprint 1 Rule (previously inline in `RuleEngine::evaluate`) into its own unit, `EvidencePresenceRule` (new, `crates/modiq-rules/src/rules/evidence_presence_rule.rs`), matching `StructuralDuplicationRule`'s shape exactly — identical behavior, identical description text and `RuleReference` identifier, moved rather than rewritten. `RuleEngine::evaluate`'s signature changed from `Option<RuleOutcome>` to `Vec<RuleOutcome>` (GOV-012, Question 1), dispatching to both Rules via two `if let` checks in fixed declaration order — `EvidencePresenceRule`, then `StructuralDuplicationRule` (GOV-012, Question 2) — with no suppression between them (GOV-012, Question 3): an Assessment whose Evidence includes a `StructuralDuplication` item produces outcomes from *both* Rules, since `EvidencePresenceRule` matches unconditionally on any non-empty Evidence regardless of category.

`modiq-engine`'s `AssessmentService::execute` updated to loop over the new `Vec<RuleOutcome>` (`for outcome in rule_engine.evaluate(...)` replacing `if let Some(outcome) = ...`) — its own public signature is completely unchanged, confirmed by every pre-existing `execute`/`execute_from_assessment_input` test passing unmodified, including the Phase 3D archive-routing tests (none of Sprint 4's or 5's fixtures produce `StructuralDuplication` Evidence, so only `EvidencePresenceRule` fires for them — exactly the single-outcome behavior those tests already asserted).

Five new tests in `engine.rs` covering the dispatch itself (no evidence → no outcomes; only the generic Rule matches → one outcome; both Rules match → two outcomes in the correct declaration order with the correct severities; multiple matching items still produce exactly two outcomes, not one per item; ordering is deterministic across repeated calls) plus three tests moved, unmodified in substance, from `engine.rs` into `evidence_presence_rule.rs`'s own module alongside `StructuralDuplicationRule`'s existing six.

`cargo fmt`, `cargo check --workspace`, and `cargo test --workspace` all pass cleanly, zero warnings: root workspace test count grew from 156 to 161 (`modiq-rules` 9 → 14; every other crate unchanged). `apps/sandbox/src-tauri` independently reverified: `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` all pass cleanly, 6/6 unchanged — the Sandbox's own archive and filesystem fixtures produce no `StructuralDuplication` Evidence, so their reports are byte-for-byte unaffected by this phase.

No architectural boundary was crossed beyond what was explicitly authorized: no trait, registry, factory, or plugin mechanism was introduced; `RuleEngine::evaluate`'s new shape and dispatch exactly match GOV-012's resolved policy; `AssessmentService`'s public entry points are unchanged in behavior; no `modiq-report` change. Sprint 5 Phase 4 (the Reporting scaffold-types investigation, recommendation only — no implementation, per Technical Director direction) and Phase 5 (final testing/documentation sync) remain to be performed.

---

### Sprint 5 Phase 4: Reporting Scaffold Investigation

Status:
Completed

Affected Crates:
- (none — investigation only; no Rust file modified)

Affected Documents:
- docs/engineering/SPRINT5_PHASE4_REPORTING_INVESTIGATION.md (new)
- docs/engineering/SPRINT5_IMPLEMENTATION_PLAN.md

Notes:
Investigated whether `modiq-report`'s four unused scaffold types (`FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, `ReportFormatter`) should be built out or retired, per Design Question 4. Checked specifically whether Sprint 5's own new severity differentiation (two Rules, `Informational` and `Warning`) created any real need that didn't exist before: it did not. Confirmed zero construction sites for all four types, unchanged since the platform's first commit — the identical evidentiary shape GOV-004 used to retire the `EngineAPI`/`modiq-rules` scaffolding. Checked the one real consumer, `apps/sandbox/src/App.tsx`: it renders Evidence, Findings, and Recommendations as three independent flat lists, performing no summarization, no formatting beyond raw field display, and no traceability cross-referencing, and required zero changes to accommodate `StructuralDuplicationRule`'s new `Warning`-severity Finding.

**Recommendation: retire all four**, on the same basis and by the same method GOV-004 used — not a claim that Reporting's assigned responsibilities (report formatting, summarization, traceability output, per `GOVERNANCE.md`'s Crate Boundary Rules) are wrong, only that these four specific, never-instantiated types are not evidenced as the right shape for fulfilling them yet. `TraceabilityReport` was assessed as the closest to a plausible future need (`Assessment`'s own `evidence_for_finding`/`findings_for_recommendation` methods already resolve these relationships programmatically, but nothing outside `modiq-runtime` has ever needed to), still with zero actual evidence of need today.

Per Technical Director direction, this recommendation is not acted on: no `modiq-report` file was modified, created, or deleted. `SPRINT5_IMPLEMENTATION_PLAN.md`'s Completion Checklist item for Design Question 4 marked complete (investigated and recorded), separate from any future action item.

`cargo fmt`, `cargo check --workspace`, and `cargo test --workspace` all pass cleanly, unchanged at 161/161 — no production code was touched this session. `apps/sandbox/src-tauri` unchanged at 6/6.

No architectural change was made or proposed. Sprint 5 Phase 5 (final testing/documentation sync) remains to close out the Sprint.

---

### Sprint 5 Phase 5: Testing & Verification

Status:
Completed

Affected Crates:
- modiq-rules

Affected Documents:
- docs/engineering/SPRINT5_IMPLEMENTATION_PLAN.md
- docs/implementation/CrateRoadmap.md

Notes:
Final testing and verification pass across Sprint 5's five phases, per Technical Director authorization. Re-reviewed Phase 2/3's own test suite for coverage gaps before treating this phase as pure re-confirmation. Found one genuine gap: `evaluate_ordering_is_deterministic_across_repeated_calls` (Phase 3) only proves `RuleEngine::evaluate` is deterministic across *repeated calls with identical input* — nothing proved the outcome order is independent of Evidence *arrival* order, the actual claim GOV-012 Question 2 makes ("never an order derived from Evidence's own arrival sequence"). Added `evaluate_outcome_order_is_independent_of_evidence_arrival_order` (`engine.rs`): evaluates the same two Evidence items in both possible orderings and confirms both produce outcomes in identical Rule-declaration order.

`modiq-report` required no change — Phase 4's investigation recommendation is not acted on this Sprint. No Sandbox fixture produces `StructuralDuplication` Evidence, confirming this phase's own predicted "not expected to be required" for Sandbox test changes.

`cargo fmt`, `cargo check --workspace`, and `cargo test --workspace` all pass cleanly, zero warnings: root workspace test count grew from 161 to 162 (`modiq-rules` 14 → 15; every other crate unchanged). `apps/sandbox/src-tauri` independently reverified: `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` all pass cleanly, 6/6 unchanged.

**All five Sprint 5 implementation phases are now complete**, each reviewed and approved by the Technical Director in turn. No architectural change was made beyond what GOV-012/GOV-013 already authorized. Sprint 5 Closeout (repository reconciliation, documentation audit, `ENGINEERING_RELEASE_0.5.md`) is not yet authorized and has not been performed — mirroring Sprint 4's own pattern, where Closeout was requested as a separate, subsequent step after the last implementation phase completed.

---

### Sprint 5 Closeout: Repository Reconciliation, Documentation Audit, Engineering Release 0.5

Status:
Completed

Affected Crates:
- (none — documentation only; no Rust file modified)

Affected Documents:
- docs/governance/PROJECT_STATUS.md
- docs/governance/CHANGELOG.md
- docs/README.md
- docs/implementation/CrateRoadmap.md
- docs/engineering/SPRINT5_IMPLEMENTATION_PLAN.md
- docs/engineering/ENGINEERING_RELEASE_0.5.md (new)

Notes:
Formal Sprint 5 closeout, per Technical Director authorization, following the same process as Sprint 4 Closeout: repository reconciliation, a documentation consistency audit, `PROJECT_STATUS.md`/`CHANGELOG.md` updates, `ENGINEERING_RELEASE_0.5.md`, an engineering retrospective (housed inside the release document, per `ENGINEERING_RELEASE_0.3.md`/`0.4.md`'s own precedent — no separate retrospective file), and final verification. No feature work was authorized or performed.

**Repository Reconciliation:** verified all five Sprint 5 implementation phases against `SPRINT5_IMPLEMENTATION_PLAN.md`'s own Completion Checklist — every item now checked, including GOV-012/GOV-013's insertion into `GOVERNANCE.md`, the `FindingSeverity` definitions in `DataModel.md`, both Rules, multi-Rule dispatch, the Reporting investigation, and the Phase 5 determinism-gap closure. No Sprint 5 work found unintentionally incomplete.

**Documentation Audit finding, recurring a third time:** `docs/governance/PROJECT_STATUS.md` and `docs/governance/CHANGELOG.md` had gone stale again, mid-Sprint, still describing Sprint 4 as the current, complete milestone with "Sprint 5 not yet scoped" — despite Sprint 4 Closeout's own standing directive that future sprints formally include reconciliation at closure. That directive was honored exactly as written (this Closeout performs the reconciliation); it did not, and was never going to, prevent staleness occurring *between* closeouts, since it addressed catching the pattern, not the underlying cause. This is now the identical finding at three consecutive Sprint closeouts (3, 4, 5) — treated in `ENGINEERING_RELEASE_0.5.md`'s Lessons Learned as evidence the fix needs to touch when these documents get edited, not just whether they're eventually corrected.

**A genuine, independent documentation error found and corrected:** both `PROJECT_STATUS.md` and `CHANGELOG.md` referenced a `docs/engineering/SPRINT4_RETROSPECTIVE.md` file that was never created — Sprint 4's retrospective has always lived inside `ENGINEERING_RELEASE_0.4.md` itself, per that release's own established pattern. Both stale citations corrected in place, with the correction itself noted rather than silently fixed.

Corrected: `PROJECT_STATUS.md` header fields and a new `## Sprint 5 — Complete` section; `CHANGELOG.md`'s Sprint 4 citation and a new `# [Sprint 5]` entry mirroring the established Added/Deferred/Released structure; `docs/README.md`'s stale Engineering Release cross-reference (0.4 → 0.5); `CrateRoadmap.md`'s `modiq-report` crate-table row (recording Phase 4's retirement recommendation), a new `## Sprint 5 — Complete` narrative section, and revision-history rows 1.15.0–1.17.0.

`ENGINEERING_RELEASE_0.5.md` published: full Sprint 5 record, retrospective, and completion report, mirroring `ENGINEERING_RELEASE_0.4.md`'s exact section structure — no new structure introduced.

`cargo fmt`, `cargo check --workspace`, and `cargo test --workspace` all pass cleanly, unchanged at 162/162 — no production code was touched this session. `apps/sandbox/src-tauri` unchanged at 6/6.

No architectural change was made or proposed: no new capability, no new abstraction, no scope expansion, no Sprint 6 work begun. **Sprint 5 is now officially closed**, pending Technical Director review of this Closeout. Nothing from Sprint 5 has been committed to git as of this entry — flagged in `ENGINEERING_RELEASE_0.5.md`'s own Engineering Metrics as worth the Technical Director's attention when scoping the commit sequence for this release, since it differs from Sprint 4's own per-phase commit history.

---

### Technical Director Review: Sprint 5 Closeout Approved; Documentation-Staleness Process Refined; Sprint 5 Officially Complete

Status:
Completed

Affected Crates:
- (none)

Affected Documents:
- (none — this entry is the record)

Notes:
The Technical Director reviewed and accepted the full Sprint 5 Closeout: repository reconciliation confirmed Sprint 5 was completed per the approved plan, documentation is synchronized with implementation, and no unresolved work remains.

**Documentation-staleness process refined**, not adopted as originally suggested. `ENGINEERING_RELEASE_0.5.md`'s Lessons Learned proposed updating `PROJECT_STATUS.md`'s "Current Phase" line per-phase, not only at Sprint close. The Technical Director accepted the underlying finding (three consecutive Sprints of identical staleness) as a valid process-improvement opportunity, but declined mandatory per-phase updates specifically. Direction: track this as an engineering workflow refinement aimed at keeping `PROJECT_STATUS.md` current at meaningful project milestones, rather than a fixed per-phase requirement. This softens, and supersedes in specificity, the Sprint 4 Closeout directive ("future sprints should formally include repository reconciliation... as part of sprint closure") without abandoning it — reconciliation at closeout remains standing practice; per-phase updates are not mandated, but keeping the document current at genuinely meaningful milestones (not necessarily every phase) is now the stated aim.

**Sprint 5 is hereby declared complete.** The repository is authorized for final Sprint 5 commit and push. Following that commit, preparation of the Sprint 6 Technical Handoff is authorized as the next step.

---

### Sprint 6 Closeout: Repository Reconciliation, Documentation Audit

Status:
Completed

Affected Crates:
- (none — documentation only; no Rust file modified)

Affected Documents:
- docs/governance/PROJECT_STATUS.md
- docs/governance/CHANGELOG.md
- docs/engineering/ENGINEERING_LOG.md (this entry)
- docs/README.md
- docs/implementation/CrateRoadmap.md
- docs/engineering/PROJECT_HANDOFF_v1.0.md
- docs/engineering/CHIEF_ARCHITECT_HANDOFF_v1.0.md
- docs/engineering/LEAD_ENGINEER_HANDOFF_v3.0.md

Notes:
Formal Sprint 6 closeout, per Chief Architect authorization: repository reconciliation, a documentation consistency audit, corrections to the seven documents above, and final verification. No feature work was authorized or performed this session.

**Repository Reconciliation:** verified Sprint 6's implementation (`modiq-cli` wired to `modiq-engine`; `modiq-report`'s four scaffold types retired) against `SPRINT6_IMPLEMENTATION_PLAN.md`'s own Completion Checklist and Authorization Record — implementation, validation, and merge into `feature/runtime-implementation` (`29657df`) all confirmed complete. `docs/engineering/POST_SPRINT6_REPOSITORY_ASSESSMENT.md` (produced the prior session) had already identified the specific staleness this Closeout corrects, rather than this Closeout discovering it independently — named here so the source of each finding stays traceable.

**Documentation Audit finding, recurring a fourth time, now more severe:** `PROJECT_STATUS.md` and `CHANGELOG.md` had gone stale again, unchanged since Sprint 5 Closeout, still describing Sprint 5 as the current, complete milestone with "Sprint 6 not yet scoped" — the identical pattern now at four consecutive Sprint closeouts (3, 4, 5, 6). This time the staleness also reached `PROJECT_HANDOFF_v1.0.md`, `CHIEF_ARCHITECT_HANDOFF_v1.0.md`, and `LEAD_ENGINEER_HANDOFF_v3.0.md` — the three documents `PROJECT_STATUS.md` itself now names as this project's authoritative governance documents, established during the same Sprint 6 cycle this Closeout reconciles. `PROJECT_HANDOFF_v1.0.md`'s crate table specifically stated `modiq-cli` was "not wired to `modiq-engine`," which was no longer true — a stale content claim, not merely a stale status label. All three handoff documents also still cited HEAD `fbef863` (Sprint 5) and "162 tests" (actual, verified fresh this session: 172).

Corrected: `PROJECT_STATUS.md` header fields and a new `## Sprint 6 — Complete` section; `CHANGELOG.md`'s new `# [Sprint 6]` entry mirroring the established Added/Deferred/Released structure; `docs/README.md`'s stale Engineering Release cross-reference; `CrateRoadmap.md`'s dependency diagram (`modiq-cli`'s two new direct edges to `modiq-runtime` and `modiq-report`, previously showing only its edge to `modiq-engine`); `PROJECT_HANDOFF_v1.0.md`'s HEAD reference, crate table, test count, Sprint History table, and roadmap section; `CHIEF_ARCHITECT_HANDOFF_v1.0.md`'s Sprint 6 readiness note; `LEAD_ENGINEER_HANDOFF_v3.0.md`'s HEAD reference, Repository Status table, and Immediate Priorities section.

No `ENGINEERING_RELEASE_0.6.md` was produced this session — outstanding, named explicitly rather than silently assumed complete, since every prior Sprint (3, 4, 5) produced one and this Closeout's authorized scope was documentation reconciliation only, not authoring a new full release record.

`cargo fmt --check`, `cargo check --workspace`, and `cargo test --workspace` all pass cleanly, unchanged at 172/172 — no production code or test was touched this session. `apps/sandbox/src-tauri` unchanged at 6/6.

No architectural change was made or proposed: no new capability, no new abstraction, no scope expansion, no governance wording changed, no Sprint 7 work begun. **Sprint 6 is now administratively complete**, pending Chief Architect review of this Closeout.

---

### Sprint 7 Closeout: Repository Validation, Commit, Push, Repository Closeout

Status:
Completed

Affected Crates:
- modiq-collection (XmlCollector)
- modiq-engine (AssessmentService composition)
- modiq-cli, apps/sandbox (test updates only)

Affected Documents:
- docs/governance/PROJECT_STATUS.md
- docs/governance/CHANGELOG.md
- docs/engineering/ENGINEERING_LOG.md (this entry)
- docs/engineering/REPOSITORY_CLOSEOUT_REPORT.md (new)

Notes:
Formal Sprint 7 closeout, per Chief Architect authorization following review of Sprint 7 implementation, Sprint 7 post-implementation refinement, the Engineering Workflow Consolidation Study, and its implementation. Repository Validation performed before any git operation: `cargo check --workspace`, `cargo test --workspace` (187/187, up from 172), `cargo fmt --all --check`, and the same four checks independently against `apps/sandbox/src-tauri` (7/7, up from 6) — all clean, zero warnings, before proceeding. Documentation consistency, single-canonical-workflow-location, archived-document marking, reference validity, absence of TODOs, and absence of accidental files were each checked directly, not assumed; no blocking issue was found.

**Commit:** Sprint 7 implementation (Multi-Source Evidence Collection / `XmlCollector`), Sprint 7 post-implementation refinement, and the Engineering Workflow Consolidation were committed together as a single commit (`277aefd`), per explicit instruction, on `feature/runtime-implementation` directly.

**Push:** `277aefd` pushed to `origin/feature/runtime-implementation`; fetch-verified 0 ahead / 0 behind after push.

**Merge:** not applicable this cycle. Unlike Sprint 6 (`feature/sprint6-cli`, merged as `29657df`), no separate Sprint 7 feature branch was ever created — all Sprint 7 work happened directly on `feature/runtime-implementation`. Recorded here as a factual finding, not worked around or silently omitted from the record.

**Repository Closeout:** `PROJECT_STATUS.md` header fields updated (Current Milestone, Current Phase, new Engineering Methodology Version field) and a new `## Sprint 7 — Complete` section added; `CHANGELOG.md`'s new `# [Sprint 7]` entry added, including its own Engineering Workflow Consolidation subsection; Governance Status note updated to carry the baseline into Sprint 8 and record the Version 1.0 designation.

**Project Milestone recorded:** Engineering Methodology Version 1.0 — the engineering workflow has been exercised across multiple completed Sprints (5 through 7), consolidated into a single canonical process (`PROJECT_HANDOFF_v1.0.md`, Section 5), and is now treated as a stable architectural artifact expected to evolve only through implementation evidence, not routine amendment. Recorded as project history, not as a Governance Register item — no GOV item was opened for it, consistent with explicit direction that this is history rather than governance.

No architectural change, no new Governance Register item, and no ADR were introduced this session. **Sprint 7 is now formally closed.**

---

### Sprint 8 Closeout: Repository Validation, Documentation Synchronization, Commit, Push, Repository Closeout

Status:
Completed

Affected Crates:
- modiq-versioning (GameVersion, VersionProfile)
- modiq-runtime (VersionProfileReference, Assessment construction)
- modiq-collection (XmlCollector descVersion extraction)
- modiq-rules (VersionCompatibilityRule, RuleEngine::evaluate signature)
- modiq-engine (AssessmentService wiring)
- modiq-report (test call sites only)

Affected Documents:
- docs/governance/PROJECT_STATUS.md
- docs/governance/CHANGELOG.md
- docs/engineering/ENGINEERING_LOG.md (this entry)
- docs/implementation/CrateRoadmap.md
- docs/README.md
- docs/engineering/ENGINEERING_RELEASE_0.8.md (new)
- docs/engineering/SPRINT8_IMPLEMENTATION_DEVIATIONS.md (new)
- docs/engineering/REPOSITORY_CLOSEOUT_REPORT.md (new)

Notes:
Formal Sprint 8 closeout, per Chief Architect authorization following review of Sprint 8's full Capability Definition → Architecture Evaluation → Architectural Resolution → Implementation Authorization → Implementation sequence. Repository Validation performed before any documentation change or git operation: `cargo fmt --all --check`, `cargo check --workspace`, and `cargo test --workspace` (205/205, up from 187) on the root workspace; the same three checks independently against `apps/sandbox/src-tauri` (7/7, unchanged — zero source modification required there). Working tree reviewed directly (`git status --porcelain`): every modified or new file traced to Sprint 8 implementation or planning; no stray, accidental, or temporary file found. A direct search confirmed zero `TODO`/`FIXME`/`XXX` markers and zero debug `println!`/`dbg!` calls introduced this Sprint. Dependency graph reviewed against `Cargo.lock`: exactly two new internal edges (`modiq-engine` → `modiq-versioning`, `modiq-rules` → `modiq-versioning`), zero new external dependency, `modiq-runtime` confirmed still dependency-free.

**Documentation Synchronization:** `PROJECT_STATUS.md` header fields updated (Current Release, Current Milestone, Current Phase, Last Updated) and a new `## Sprint 8 — Complete` section added; Current Focus and Governance Status notes both updated to reflect Sprint 8 and carry the baseline into Sprint 9. `CHANGELOG.md`'s new `# [Sprint 8]` entry added, mirroring the established Added/Deferred/Released structure. `CrateRoadmap.md`'s Implementation Status table and dependency diagram updated for `modiq-versioning`'s first real content and its two new consumer edges. `docs/README.md`'s Engineering Release cross-reference updated to 0.8.

**Engineering Release:** `ENGINEERING_RELEASE_0.8.md` produced **at this Sprint's own Closeout**, not retroactively — directly correcting the two-Sprint-running late-production pattern `ENGINEERING_RELEASE_0.7.md`'s own Lessons Learned named as a risk not to repeat a third time.

**Implementation Deviation Record:** `SPRINT8_IMPLEMENTATION_DEVIATIONS.md` produced, documenting every meaningful difference between `SPRINT8_IMPLEMENTATION_AUTHORIZATION.md`'s planned approach and actual implementation (the unchanged `AssessmentService` entry points; the `VersionProfileReference` ownership refinement; the leaner-than-planned dependency footprint), each with repository evidence and engineering rationale, per explicit Chief Architect request that this become part of permanent engineering history rather than a detail folded silently into the Implementation Report alone.

**Commit:** all Sprint 8 changes (implementation and documentation synchronization) staged and committed together as a single commit, per instruction, on `feature/runtime-implementation` directly.

**Push:** pushed to `origin/feature/runtime-implementation`.

**Merge:** not applicable this cycle, mirroring Sprint 7's own precedent exactly — no separate Sprint 8 feature branch was ever created; all Sprint 8 work happened directly on `feature/runtime-implementation`.

No architectural change, no new Governance Register item, and no ADR were introduced this session, consistent with explicit Chief Architect decision (Decision 6, `SPRINT8_ARCHITECTURAL_RESOLUTION.md`/`SPRINT8_IMPLEMENTATION_AUTHORIZATION.md`). **Sprint 8 is now formally closed.**

---

### Sprint 9 Closeout: Repository Validation, Documentation Synchronization, Commit, Repository Closeout

Status:
Completed

Affected Crates:
- modiq-knowledge (RepairRecipe)
- modiq-rules (VersionCompatibilityRule, new dependency edge)
- modiq-engine (test coverage only)

Affected Documents:
- docs/governance/PROJECT_STATUS.md
- docs/governance/CHANGELOG.md
- docs/engineering/ENGINEERING_LOG.md (this entry)
- docs/implementation/CrateRoadmap.md
- docs/README.md
- docs/engineering/ENGINEERING_RELEASE_0.9.md (new)
- docs/engineering/REPOSITORY_CLOSEOUT_REPORT.md (superseded — Sprint 9 record)

Notes:
Formal Sprint 9 closeout, per Chief Architect authorization following Repository Review's own "Approve for Commit" recommendation and the subsequent Commit Authorization (six implementation files, dependency updates, associated tests, plus — by explicit Chief Architect choice — this Sprint's own planning/decision record folded into the same commit). Repository Validation re-performed before any further documentation change: `cargo fmt --all --check`, `cargo check --workspace`, `cargo clippy --workspace --all-targets`, and `cargo test --workspace` (210/210, up from 205) on the root workspace; the same checks independently against `apps/sandbox/src-tauri` (7/7, unchanged — zero source modification required there). `git status`/`git log` confirmed one commit (`21eb7eb`) ahead of `origin/feature/runtime-implementation`, working tree clean.

**Documentation Synchronization:** `PROJECT_STATUS.md` header fields updated (Current Release, Current Milestone, Current Phase) and a new `## Sprint 9 — Complete` section added; Current Focus and Governance Status notes both updated to reflect Sprint 9 and carry the baseline into Sprint 10. `CHANGELOG.md`'s new `# [Sprint 9]` entry added, mirroring the established Added/Deferred/Released structure. `CrateRoadmap.md`'s Implementation Status table (`modiq-knowledge` L1 → L2), dependency-hierarchy notes for the new `modiq-rules` → `modiq-knowledge` edge, a Sprint 9 narrative section, and a new Revision History entry (1.21.0). `docs/README.md`'s Engineering Release cross-reference updated to 0.9.

**Engineering Release:** `ENGINEERING_RELEASE_0.9.md` produced **at this Sprint's own Closeout**, continuing the Sprint-8-established practice of producing it at close rather than retroactively.

**Repository Closeout Report:** `REPOSITORY_CLOSEOUT_REPORT.md` — a living, Sprint-specific record (filename unversioned, content superseded each Sprint, per the pattern already established at Sprint 7 and Sprint 8's own closeouts) — rewritten for Sprint 9.

**Process finding, specific to this Sprint:** during Architectural Resolution review (prior to implementation), the Chief Architect identified that the initial draft's Question 2 conflated two distinct questions — where a `RepairRecipe` is *retrieved* (correctly resolved: a direct call, no new `RuleEngine::evaluate` parameter) and where its content is *authored* (initially resolved incorrectly: inline literals inside the consuming Rule, making `modiq-rules` the author of engineering knowledge rather than its consumer). The Architectural Resolution document was revised in place — not merely appended with a caveat — before implementation began, so that `modiq-knowledge` authors the named, specific `RepairRecipe` value (`version_compatibility_declared_version_mismatch()`, mirroring `VersionProfile::fs25()`'s own precedent) and `modiq-rules` only calls it. Implementation, Repository Review, and this Closeout all confirm the corrected design, not the original draft, is what the repository contains — no drift between the corrected resolution and what was built.

**Commit:** Sprint 9 documentation synchronization staged and committed as a second commit, separate from the implementation commit (`21eb7eb`), on `feature/runtime-implementation` directly — mirroring Sprint 7's and Sprint 8's own two-commit precedent (implementation, then Closeout).

**Push:** not performed this session — awaiting Chief Architect approval before repository history is pushed.

**Merge:** not applicable this cycle, mirroring Sprint 7's and Sprint 8's own precedent — no separate Sprint 9 feature branch was ever created; all Sprint 9 work happened directly on `feature/runtime-implementation`.

No architectural change, no new Governance Register item, and no ADR were introduced this session — the Knowledge Domain boundary section in `GOVERNANCE.md` already named Repair Recipes explicitly before this Sprint began, so no amendment was needed (unlike Sprint 8's `modiq-versioning` Crate Boundary Rule gap, which remains open, unaffected by Sprint 9). **Sprint 9 is now formally closed, pending push.**

---

### Sprint 10 Closeout: Runtime Fixture Corpus Acquisition, Documentation Synchronization, Repository Closeout

Status:
Completed

Affected Crates:
- (none — no Rust source was modified this Sprint)

Affected Documents:
- fixtures/README.md (new)
- fixtures/runtime-logs/README.md (new)
- fixtures/runtime-logs/TEMPLATE.md (new)
- fixtures/runtime-logs/clean-base-game/README.md (new), log.txt (new)
- fixtures/runtime-logs/single-compatible-mod/README.md (new), log.txt (new)
- fixtures/runtime-logs/single-incompatible-mod/README.md (new), log.txt (new)
- docs/engineering/SPRINT10_CAPABILITY_DEFINITION.md (new)
- docs/engineering/SPRINT10_RUNTIME_LOG_FIXTURE_PREPARATION.md (new)
- docs/governance/PROJECT_STATUS.md
- docs/governance/CHANGELOG.md
- docs/engineering/ENGINEERING_LOG.md (this entry)
- docs/implementation/CrateRoadmap.md
- docs/README.md
- docs/engineering/ENGINEERING_RELEASE_1.0.md (new)

Notes:
Formal Sprint 10 closeout. Unlike Sprints 1 through 9, this Sprint's own scope was Capability Definition and evidence acquisition only — `cargo fmt`/`check`/`test` were re-run to confirm this directly rather than assumed: root workspace test suite unchanged at 210/210, Sandbox unchanged at 7/7, zero warnings, zero Rust source file touched. `git status` confirmed every changed or new file traced directly to fixture-corpus documentation or the real, normalized runtime logs themselves; no stray, accidental, or temporary file found; no mod archive present anywhere in the repository (per Sprint 10's own explicit policy).

**Capability Definition and Fixture Acquisition:** `SPRINT10_CAPABILITY_DEFINITION.md` scoped Runtime Log Interpretation to recognizing one class of signal, grounded in `Vision.md`'s own named question. `SPRINT10_RUNTIME_LOG_FIXTURE_PREPARATION.md` records the full history of the corpus's own design and all three fixture captures, including two real documentation gaps found and closed during acquisition (Installation State vs. Savegame State; Warning Categorization) — both detailed in `ENGINEERING_LOG.md`'s own Engineering Methodology Observations, below.

**Documentation Synchronization:** `PROJECT_STATUS.md` header fields updated (Current Release, Current Milestone, Current Phase) and a new `## Sprint 10 — Complete` section added, explicit that the Runtime Log Interpretation capability's own implementation remains deferred; Current Focus and Governance Status notes both updated to reflect Sprint 10 and carry the baseline into Sprint 11. `CHANGELOG.md`'s new `# [Sprint 10]` entry added, mirroring the established Added/Deferred/Released structure. `CrateRoadmap.md` gained a Sprint 10 narrative note (no crate maturity changed) and a new Revision History entry. `docs/README.md`'s Engineering Release cross-reference updated to 1.0.

**Engineering Release:** `ENGINEERING_RELEASE_1.0.md` produced at this Sprint's own Closeout, using a narrower structure than the full 16-section template established for implementation Sprints (0.4 through 0.9) — this Sprint changed no code, so Testing Growth, Crate Maturity Review, and similar sections would have reported "unchanged" throughout; the release instead follows the specific outline this Sprint's own scope calls for (objectives, work completed, corpus acquired, documentation completed, engineering decisions, known limitations, explicit deferral statement, next-Sprint continuation).

**Commit:** Sprint 10's own fixture acquisition and Capability Definition work, and this Closeout's documentation synchronization, are staged as two separate commits per explicit instruction (implementation commit: complete runtime fixture corpus; closeout commit: engineering release and closeout) — mirroring the two-commit precedent every Sprint since Sprint 7 has followed.

**Push:** not performed this session — awaiting Chief Architect approval before repository history is pushed.

**Merge:** not applicable this cycle — no separate Sprint 10 feature branch was ever created; all Sprint 10 work happened directly on `feature/runtime-implementation`.

No architectural change, no new Governance Register item, and no ADR were introduced this session. **Sprint 10 is now formally closed, pending push.**

---

### Sprint 11 Closeout: Runtime Evidence Processing Architecture, Implementation, Architectural Reconciliation, Repository Closeout

Status:
Completed

Affected Crates:
- modiq-collection (new: `RuntimeLogCollector`)
- modiq-rules (new: `RuntimeLoadFailureRule`; `RuleEngine::evaluate` dispatch extended)
- modiq-engine (`AssessmentService::execute_from_assessment_input` extended)

Affected Documents:
- docs/implementation/SPRINT11.md (new)
- docs/engineering/RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md (new, v1.0.0 → v1.2.0 across this Sprint)
- docs/governance/PROJECT_STATUS.md
- docs/governance/CHANGELOG.md
- docs/engineering/ENGINEERING_LOG.md (this entry)
- docs/implementation/CrateRoadmap.md
- docs/README.md
- docs/engineering/ENGINEERING_RELEASE_1.1.md (new)

Notes:
Formal Sprint 11 closeout. Unlike Sprint 10, this Sprint delivered both the Architectural Resolution and its own implementation in the same cycle. `cargo fmt --check`, `cargo check --workspace`, and `cargo test --workspace` confirm the final state directly: root workspace test suite grew from 210 to 238, Sandbox unchanged at 7/7, zero warnings.

**Architectural Resolution:** `RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md` (v1.0.0) resolved all four questions `SPRINT11.md`'s Scope named, grounded directly in Sprint 10's three real fixtures, including the Runtime Interpretation Decision Matrix as a first-class deliverable. A Chief Architect-requested refinement pass (v1.1.0) added an explicit Architectural Invariants section and a governance-relationship clarification distinguishing this Sprint's own architectural interpretation from GOV-013's separate, unresolved question.

**Implementation:** four independently reviewed milestones — `RuntimeLogCollector` (standalone, unit-tested), its wiring into `AssessmentService`, `RuntimeLoadFailureRule` (standalone, unit-tested), and its dispatch integration into `RuleEngine::evaluate` as the fourth entry in GOV-012's fixed order. Neither of `AssessmentService`'s two public entry points nor `RuleEngine::evaluate`'s parameter shape required any change.

**Architectural Reconciliation (v1.1.0 → v1.2.0):** a dedicated, adversarial engineering verification pass — conducted specifically to attempt to disprove architectural consistency rather than confirm it — found that v1.1.0's own Architectural Invariants section asserted an Evidence-first, Rule-decides recognition model, while `RuntimeLogCollector`, built against an earlier section of the same document, performs recognition before Evidence is created. Implementation was halted and the contradiction reported, per this project's standing discipline, rather than resolved unilaterally. Chief Architect review confirmed the implementation was correct and directed the document's own wording to be reconciled to it — recorded in full detail in this Sprint's own "Engineering Methodology Observations" entry, below. No Rust source, test, fixture, ADR, or Governance Register item was touched in the reconciliation itself.

**Documentation Synchronization:** `PROJECT_STATUS.md` header fields updated (Current Release, Current Milestone, Current Phase) and a new `## Sprint 11 — Complete` section added; Current Focus and Governance Status notes both updated to reflect Sprint 11 and carry the baseline into Sprint 12. `CHANGELOG.md`'s new `# [Sprint 11]` entry added, including its own Architectural Reconciliation subsection. `CrateRoadmap.md` gained a Sprint 11 narrative note, updated crate-table rows for `modiq-collection` and `modiq-rules`, and a new Revision History entry. `docs/README.md`'s Engineering Release cross-reference updated to 1.1.

**Engineering Release:** `ENGINEERING_RELEASE_1.1.md` produced at this Sprint's own Closeout, following the full section structure established for implementation Sprints, with a dedicated Architectural Reconciliation section documenting the mid-Sprint contradiction and its resolution as part of the permanent engineering record, not a footnote.

**Commit:** Sprint 11's own architecture, implementation, and reconciliation work, and this Closeout's documentation synchronization, are recommended as two separate commits, mirroring the two-commit precedent every Sprint since Sprint 7 has followed.

**Push:** not performed this session — awaiting Chief Architect approval before repository history is pushed.

**Merge:** not applicable this cycle — no separate Sprint 11 feature branch was created; all Sprint 11 work happened directly on `feature/runtime-implementation`.

No new Governance Register item and no ADR were introduced this session; GOV-013 was documented as newly relevant, not reopened. **Sprint 11 is now formally closed, pending push.**

---

### Sprint 12 Closeout: Capability Scaling Architecture, Reconciliation, Repository Closeout

Status:
Completed

Affected Crates:
- (none — this Sprint was architecture-only; no Rust source, test, or fixture was modified at any point)

Affected Documents:
- docs/implementation/SPRINT12.md (new, v1.0.0 → v1.1.0 across this Sprint)
- docs/engineering/SPRINT12_ARCHITECTURAL_RESOLUTION.md (new)
- docs/governance/PROJECT_STATUS.md
- docs/governance/CHANGELOG.md
- docs/engineering/ENGINEERING_LOG.md (this entry)
- docs/implementation/CrateRoadmap.md
- docs/README.md
- docs/engineering/ENGINEERING_RELEASE_1.2.md (new)

Notes:
Formal Sprint 12 closeout. `cargo fmt --check`, `cargo check --workspace`, and `cargo test --workspace` confirm the final state directly, unchanged from Engineering Release 1.1: root workspace test suite remains 238/238, Sandbox unchanged at 7/7, zero warnings — expected, since this Sprint touched no source at any point.

**Capability Identity Derivation:** `SPRINT12.md` (v1.0.0) derived an explicit Capability Identity procedure from six named historical decisions, a single cascading test (Kind → Mechanism → Judgment), together with Collector Composition and Rule Composition evaluated as its consequences.

**Adversarial Verification and Reconciliation:** this Sprint's own verification pass — checking the draft procedure against every historical decision rather than only the headline ones — found that Sprint 4 actually contains two separable decisions, and the second (Phase 3C, GOV-011's `StructuralDuplication` category) falsifies the original model's central assumption (new category always implies new Collector). Per this project's standing discipline, the contradiction was reported rather than resolved unilaterally; on Chief Architect confirmation, `SPRINT12.md` was amended to v1.1.0 (three independent axes — Collection, Evidence, Interpretation — plus an orthogonal Introduction check), and `SPRINT12_ARCHITECTURAL_RESOLUTION.md` was produced recording the complete contradiction, evidence, and corrected model, including the direct consequence that Sprint 11 is reclassified from Capability Introduction to Capability Expansion — recorded with explicit, repeated care to distinguish architectural classification from product significance, detailed in full in this Sprint's own Engineering Methodology Observations entry below.

**Documentation Synchronization:** `PROJECT_STATUS.md` header fields updated (Current Release, Current Milestone, Current Phase) and a new `## Sprint 12 — Complete` section added; Current Focus and Governance Status notes both updated to reflect Sprint 12 and carry the baseline into Sprint 13. `CHANGELOG.md`'s new `# [Sprint 12]` entry added, including its own Architectural Contradiction and Reconciliation subsection. `CrateRoadmap.md` gained a Sprint 12 narrative note (no crate maturity changed — this Sprint touched no code) and a new Revision History entry. `docs/README.md`'s Engineering Release cross-reference updated to 1.2.

**Engineering Release:** `ENGINEERING_RELEASE_1.2.md` produced at this Sprint's own Closeout, describing both the original architectural proposal and the subsequent reconciliation explicitly, so the record shows strengthening through correction rather than silent revision.

**Commit:** Sprint 12's own architecture, adversarial verification, and reconciliation work, and this Closeout's documentation synchronization, are recommended as two separate commits, mirroring the two-commit precedent every Sprint since Sprint 7 has followed.

**Push:** not performed this session — awaiting Chief Architect approval before repository history is pushed.

**Merge:** not applicable this cycle — no separate Sprint 12 feature branch was created; all Sprint 12 work happened directly on `feature/runtime-implementation`.

No new Governance Register item and no ADR were introduced this session — this Sprint's own conclusions apply and extend already-approved architecture rather than establishing a new durable principle requiring one. **Sprint 12 is now formally closed, pending push.**

---

### INV-001: Lua Analysis Capability Investigation — Complete, Recommendation A

Status:
Completed

Affected Crates:
- (none — this was an evidence-acquisition investigation; no Rust source, test, or fixture was touched)

Affected Documents:
- docs/engineering/INV-001_LUA_ANALYSIS_CAPABILITY.md (new)
- docs/engineering/GOVERNANCE.md (GOV-014 opened)
- docs/governance/PROJECT_STATUS.md
- docs/engineering/ENGINEERING_LOG.md (this entry)

Notes:
Not Sprint 13. Not architecture. Not implementation. Following an earlier capability proposal's own accepted conclusion (evidence insufficient for Architecture Evaluation), this investigation asked one question: what specific Lua problem can modIQ objectively recognize with deterministic evidence?

**Findings:** no real Farming Simulator mod Lua script has ever been examined by this project, and none can be obtained without a human performing the same real, licensed acquisition Sprint 10 required for runtime log fixtures — this remains the investigation's own single largest, unresolved gap. Of the candidate defect classes considered, only Lua syntax validity passes every determinism and Collector-boundary test; Lua's own dynamic scoping defeats naive "undefined global" detection, and every broader target risks the Collector/Rule observational boundary this platform has never breached. A safe, mature, parse-only Lua dependency (`full_moon`) was identified via real research (crates.io, docs.rs); execution-capable alternatives (`mlua`/`rlua`) were correctly ruled out as architecturally inappropriate for static inspection of untrusted content, independent of any sample acquisition.

**Recommendation A: evidence remains insufficient; further investigation required.** Architecture Evaluation was not authorized. Sprint Planning did not begin.

**Governance follow-up:** GOV-014 (Lua Fixture Acquisition Governance) opened in `GOVERNANCE.md`, Open, as an explicit, separate governance action distinct from this investigation's own scope — provenance, licensing, storage, and acquisition-governance questions specific to real third-party Lua source code (a materially different artifact from a captured runtime log) must be resolved before any future Lua fixture may be acquired, and before Architecture Evaluation may be authorized. The Governance Register now totals 14 items, 8 Resolved, 6 Open.

**Repository state after this investigation:** Sprint 12 remains complete and frozen. No Sprint 13 work has started. The repository awaits (a) GOV-014's own resolution and (b) real, human-acquired Lua fixture evidence before any further architectural evaluation of this capability.

---

### Sprint 13 Closeout: Storage Architectural Activation

Status:
Completed

Affected Crates:
- modiq-storage (new)
- modiq-cli
- apps/sandbox (separate workspace)

Affected Documents:
- docs/engineering/INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md (new, then amended)
- docs/engineering/GOVERNANCE_OBSERVATION_SUBSYSTEM_ACTIVATION.md (new, then amended)
- docs/engineering/PROJECT_HANDOFF_v1.1.md (§5 amendment)
- docs/engineering/STORAGE_ARCHITECTURE_EVALUATION.md (new)
- docs/engineering/STORAGE_IMPLEMENTATION_AUTHORIZATION.md (new)
- docs/engineering/STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md (new)
- docs/engineering/STORAGE_SPRINT_PLAN.md (new)
- docs/engineering/GOVERNANCE.md (Storage Crate Boundary Rule added)
- docs/governance/PROJECT_STATUS.md
- docs/governance/CHANGELOG.md
- docs/engineering/ENGINEERING_LOG.md (this entry)
- docs/implementation/CrateRoadmap.md
- docs/README.md
- .gitignore
- docs/engineering/ENGINEERING_RELEASE_1.3.md (new)

Notes:
Formal Sprint 13 closeout. `cargo fmt --check`, `cargo check --workspace`, and `cargo test --workspace` confirm the final state directly: root workspace test suite 253/253 (238 prior + `modiq-storage` 10 + `modiq-cli` +5), zero warnings; Sandbox 9/9, zero warnings.

**Investigation:** `INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md` established Storage as a valid, well-evidenced capability candidate (`Vision.md`'s "Historical knowledge preservation," `Principles.md`'s "Knowledge Preservation," `ProductSpecification.md`'s MKB content list and Assessment Workflow step 7), then found the Sprint 12 Capability Identity procedure could not classify it — none of its three axes or its Introduction test are answerable for a subsystem-level candidate, only for Collector/Rule-shaped ones.

**Governance Reconciliation:** `GOVERNANCE_OBSERVATION_SUBSYSTEM_ACTIVATION.md` generalized this finding into an Observation, per this project's own Decision Framework. Chief Architect review then surfaced repository evidence the investigation itself had not yet found: `SPRINT8_ARCHITECTURAL_RESOLUTION.md` §8 had already named and applied "Architectural Activation" to Version Profiles' own first real content — a classification Sprint 12's own historical derivation never checked itself against, despite predating it. Both `INV-002` and the Governance Observation were revised to record this: not an absence of governing concept, but two existing, unreconciled taxonomies. `PROJECT_HANDOFF_v1.1.md` §5 was amended (commit `fd2db36`) with a two-sentence scope clarification — the Capability Identity gate applies to Collector/Rule-shaped candidates specifically; subsystem-level activation follows Architectural Activation directly to Architecture Evaluation instead — introducing no new procedure, taxonomy, or architectural decision, and leaving both Sprint 8's and Sprint 12's own documents unchanged.

**Architecture Evaluation, Resolution, Authorization:** `STORAGE_ARCHITECTURE_EVALUATION.md` evaluated four decisions with alternatives, mirroring Sprint 8's own treatment of Version Profiles: the persisted domain object (`AssessmentReport`, chosen over `Assessment` itself or individually addressable Evidence/Finding/Recommendation records, since it alone requires no new decision about lifecycle, mutation, or cross-referencing), subsystem ownership (a new `modiq-storage` crate — disclosed honestly as one respect in which this activation differs from Sprint 8/9's own precedent, since neither `modiq-versioning` nor `modiq-knowledge` needed to be created from nothing), the lifecycle boundary (strictly downstream of Reporting, deliberately agnostic to GOV-001's own still-Open timing question), and impact on `AssessmentService`'s public entry points (none — Storage is consumed by callers the same way `modiq-report` already is). All four were Accepted by the Chief Architect exactly as recommended. `STORAGE_IMPLEMENTATION_AUTHORIZATION.md` then translated the Resolution into an engineering envelope, deliberately narrower in scope than `SPRINT8_IMPLEMENTATION_AUTHORIZATION.md`'s own precedent — excluding Sprint sequencing, crate order, and per-phase testing strategy entirely — a scope difference recorded explicitly, on request, as reflecting the repository's own evolved separation between Implementation Authorization and Sprint Planning, not any difference in Storage's own architectural requirements.

**A design conflict, found during implementation preparation and reported rather than resolved unilaterally:** before writing Phase 1, direct inspection of `AssessmentReport` and its nested Runtime types found none derive `Serialize`/`Deserialize`, and, more fundamentally, that `AssessmentId`/`EvidenceId`/`FindingId` expose no accessor to their own inner value and are generated from a function-local, process-scoped `AtomicU64` counter that restarts at 1 on every process invocation. This meant a faithful write-then-read round trip was not achievable without modifying `modiq-runtime` or `modiq-report` — both explicitly designated unmodified by the Sprint Plan — and, independently, that even a modified accessor would not restore meaningful cross-process identity, since two unrelated Assessments in two separate process runs can trivially receive identical numeric ids. This was reported, not resolved in place, per this project's standing discipline.

**Resolution:** `STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md` found that Storage defining and owning its own persisted representation — populated from `AssessmentReport`'s already-public getters, never reconstructing the original type or its identity — resolves the conflict entirely within `modiq-storage`'s own crate boundary. This is the fourth instance of ADR-0007's Opaque Runtime References pattern (`RuleReference`, `RepairRecipeReference`, `VersionProfileReference`), applied one layer downstream of where it has been applied before. Finding→Evidence and Recommendation→Finding cross-references are preserved as positions within the persisted report itself, not the original process-local identifiers — the referential intent survives; the specific numeric token does not need to. Faithfulness is judged by content and order, directly supported by this platform's own pre-existing convention ("Determinism is judged by content and order, never by incidental identity," `PROJECT_HANDOFF_v1.1.md` §5) — extended here from same-process determinism testing to cross-process persistence fidelity, a natural extension the process-restarting counter finding makes decisive rather than merely permissible. No modification to `modiq-runtime` or `modiq-report` was required; no fixed Architectural Resolution decision was revisited.

**Implementation, across three independently validated phases:** Phase 1 gave `modiq-storage` its first real content — `PersistedAssessmentReport` and its nested `Persisted*` types, `ReportKey` (minted by Storage itself, since `AssessmentId` cannot serve as an external key), and `ReportStore` (real filesystem-backed write/read, one JSON file per report, no new external dependency beyond the workspace's already-declared `serde`/`serde_json`). A genuine implementation-time refinement, reported rather than silently absorbed: the Sprint Plan's own Crate Activation Sequence assumed `modiq-storage` would depend on `modiq-report` only, but converting `Evidence`/`Finding`/`Recommendation`'s own field types (`EvidenceCategory`, `FindingSeverity`, `RuleReference`) requires naming them directly, and they live in `modiq_runtime::assessment`, not re-exported by `modiq-report` — an additional `modiq-storage → modiq-runtime` dependency edge, the same diamond-dependency shape already common throughout this workspace, conflicting with none of the four fixed decisions.

Phase 2 wired `modiq-cli`: `AssessCommand::run` now hands a successful assessment's report to `ReportStore::store`, reporting the resulting key; storage failure is reported as a warning, never as a change to the assessment's own exit code, preserving Storage's own "never feeds back upstream" boundary at the CLI's exit-code semantics specifically. A new `retrieve <key>` command reads a report back independent of running a new Assessment. Verified beyond in-test coverage: the actual built binary was run as two separate process invocations (`assess`, then `retrieve` in a distinct process), the real load-bearing proof the Sprint Plan's own Testing Strategy named.

Phase 3 wired `apps/sandbox` identically — `create_assessment`'s IPC summary DTO gained a `stored_report_key` field; a new `retrieve_report` Tauri command mirrors `modiq-cli`'s own, returning a new `PersistedReportSummary` IPC DTO. Both command implementations were split into a thin `#[tauri::command]` wrapper plus a storage-root-parameterized core function, the identical shape Phase 2 established, so tests exercise a real, hermetic temporary directory rather than the fixed, manifest-relative default. `.gitignore` gained `.modiq-storage/`, since both defaults can now write real files into the working tree during ordinary use.

`modiq-runtime`, `modiq-report`, `modiq-engine`, `modiq-rules`, `modiq-versioning`, and `modiq-knowledge` are unmodified throughout all three phases. `AssessmentService`'s two public entry points required zero signature change. No architectural conflict reached implementation itself beyond the one design conflict reported above and resolved before Phase 1 began.

**Documentation Synchronization:** `PROJECT_STATUS.md` header fields updated (Current Release, Repository Status, Current Milestone, Overall Status, Current Phase) and a new `## Sprint 13 — Complete` section added; Current Focus and Governance Status notes both updated to reflect Sprint 13 and carry the baseline into Sprint 14. `CHANGELOG.md`'s new `# [Sprint 13]` entry added, including its own Investigation and Governance Reconciliation, and Design Resolution, subsections. `CrateRoadmap.md` gained a new `modiq-storage` Implementation Status row, a dependency-hierarchy note for the new crate's edges, a Sprint 13 narrative note, and a new Revision History entry. `docs/README.md`'s Engineering Release cross-reference updated to 1.3. `GOVERNANCE.md` gained `modiq-storage`'s own Crate Boundary Rule pair (Owns / Must never), cross-referencing the governance reconciliation and all four Storage documents directly.

**Engineering Release:** `ENGINEERING_RELEASE_1.3.md` produced at this Sprint's own Closeout, following the section structure established for implementation Sprints, with a dedicated section documenting the governance reconciliation and the design conflict-and-resolution as part of the permanent engineering record.

**Commit:** Sprint 13's work was committed incrementally as each stage was approved — governance reconciliation, Architectural Resolution, Implementation Authorization, Sprint Plan, and each of the three implementation phases as its own commit — rather than as one or two closeout-time commits, reflecting how this Sprint was authorized and reviewed stage by stage rather than as a single block. This Closeout's own documentation synchronization is its own final commit, mirroring every prior Sprint's own closeout-commit precedent.

**Push:** each stage-commit was pushed immediately upon Chief Architect approval of that stage, not batched until Closeout — a finer-grained push cadence than any prior Sprint, reflecting this Sprint's own finer-grained review cadence.

**Merge:** not applicable this cycle — no separate Sprint 13 feature branch was created; all Sprint 13 work happened directly on `feature/runtime-implementation`.

No new Governance Register item and no ADR were introduced this session — this Sprint's own governance work clarified an existing standing rule's scope rather than establishing a new one. **Sprint 13 is now formally closed, pending this Closeout's own push.**

---

## Engineering Methodology Observations

A running record of process observations surfaced during Sprint execution — distinct from the Engineering Methodology itself (`PROJECT_HANDOFF_v1.0.md`, Section 5, Version 1.0). Recorded here as history and future input, per this project's own evidence-based standard for methodology change: an observation is not an adopted process change until a future Chief Architect session evaluates it as such, exactly as GOV-004 and GOV-012 required convergent implementation evidence before a code-level pattern was treated as settled. Nothing in this section modifies the canonical workflow.

### Sprint 8: Phased Execution with Validation Gates

Sprint 8's implementation session ran seven distinct phases (`modiq-versioning` domain content → `modiq-runtime` construction evolution → `modiq-collection` extraction → `modiq-rules` Rule and `RuleEngine` signature → `modiq-engine` wiring → `modiq-report` call-site updates → full-workspace and Sandbox validation), each validated independently (`cargo fmt`/`check`/`test` at the crate level) before the next began, rather than writing all seven phases' code first and validating once at the end.

**Observation:** this per-phase validation gate caught what would otherwise have been a late-discovered, larger-surface-area problem. Because `modiq-collection`'s `descVersion` extraction was implemented and validated (crate-level `cargo test -p modiq-collection`) before `modiq-rules`' `VersionCompatibilityRule` was written, the exact evidence-description format `XmlCollector` actually produces was already confirmed, real, and test-covered by the time the Rule needed to parse it — rather than the Rule being written against an assumed format and a mismatch surfacing only once the full pipeline was assembled and tested together at the very end. The same held moving from `modiq-rules` to `modiq-engine`: `RuleEngine::evaluate`'s new signature was already fixed and tested before `AssessmentService`'s call site was touched, so the engine-level integration required no rework.

### Sprint 9: Architectural Resolution Review Caught a Design Error Before Implementation

Sprint 9's initial Architectural Resolution draft conflated two distinct questions under one heading — where a `RepairRecipe` is *retrieved* at evaluation time, and where its content is *authored*. Both were resolved together as "inside the Rule," which correctly answered the first question but incorrectly answered the second: it would have made `modiq-rules` the author of engineering knowledge, directly contrary to `modiq-knowledge`'s own README boundary ("knowledge is authored here... [Rules] never own or modify the knowledge itself").

**Observation:** the Chief Architect's own review of the Architectural Resolution document — a distinct step this project's workflow already reserves for exactly this purpose — caught the conflation before any implementation code was written. The correction was made by revising the resolution document itself (decision, rationale, alternatives, and every downstream section it touched), not by appending a caveat alongside the original error, so that implementation proceeded against a single, internally consistent record rather than a draft plus a patch. This is the same category of finding Sprint 8's own Lessons Learned recorded for `VersionProfileReference` (a validation pass surfacing a real design question a single careful pass did not) — recorded here as a second, independent data point for the same observation: dedicated review stages between architectural decision and implementation continue to find real errors, not merely formalities to pass through.

**Why this is being recorded, not adopted:** Sprint 8 is one data point. This project's own governance discipline (`CHIEF_ARCHITECT_HANDOFF_v1.0.md`, Section 4: "prefer convergent evidence over a single implementation attempt") holds that a single favorable Sprint does not, by itself, justify writing a new mandatory stage into the canonical workflow (`PROJECT_HANDOFF_v1.0.md`, Section 5's eleven-stage Implementation → Validation sequence already exists; this observation is about *sub-phase* granularity within that single stage, not a new stage). Whether this should become an explicit expectation of every future multi-crate implementation, or remains ordinary good practice not worth codifying, is left for a future Chief Architect session to evaluate — ideally once a second Sprint's own evidence exists to compare against, mirroring exactly how GOV-004's three-subsystem convergence, not a single favorable case, is this project's own standing bar for treating a pattern as settled.

### Sprint 10: Evidence Acquisition as Its Own Pre-Architecture Phase

Sprint 10 (Runtime Fixture Corpus Acquisition) is the first Sprint in this project's history where an entire Sprint's own scope was evidence acquisition alone, with architecture and implementation explicitly and completely deferred — not one investigation phase within a larger implementation Sprint (Sprint 4 Phase 2's Boundary-Proving against the `zip` crate is the closest prior precedent, but that was one phase of a Sprint that also implemented against its findings in the same cycle). `SPRINT10_CAPABILITY_DEFINITION.md`, Section 11 stated this as a requirement, not a suggestion: no architectural decision for Runtime Log Interpretation may assume a real log's structure, wording, stability, or formatting, and acquiring representative logs was named the first engineering activity that requirement demands.

**Observation:** treating acquisition as its own complete Sprint, rather than a phase folded into an implementation Sprint, surfaced two real corpus-documentation gaps *before* any Rust code existed to be built against a wrong assumption — Installation State versus Savegame State (found capturing the very first fixture) and Warning Categorization (found capturing the second). Both were formalized as corpus policy, with the fixture that exposed each gap integrated only afterward, not worked around silently. Had this Sprint instead begun with a Collector implementation against an assumed log shape, both gaps would most likely have surfaced as implementation-time surprises or, worse, as silent incorrect assumptions baked into a Rule's own design — precisely the "implementation-led architecture" failure mode this project's own Decision Framework exists to prevent (`CHIEF_ARCHITECT_HANDOFF_v1.0.md`, Section 6), here avoided one Sprint earlier than the pattern has ever been applied before.

**Why this is being recorded, not adopted as a new mandatory stage:** one Sprint is one data point, the same standard already applied to Sprint 8's and Sprint 9's own observations above. Whether "evidence acquisition as its own complete, dedicated Sprint" should become the standing expectation whenever a future capability depends on a real-world artifact this repository does not yet possess, or whether Sprint 10's own shape was specific to Runtime Log Interpretation's own unusual precondition, is left for a future Chief Architect session to evaluate against a second instance, not decided here.

### Sprint 11: Adversarial Engineering Verification Found a Documentation Inconsistency Implementation-Review Would Not Have

Every prior instance of "a review stage caught a real error before it compounded" recorded in this section (Sprint 8's `VersionProfileReference` refinement; Sprint 9's Repair Recipe authorship conflation) was an *architectural* review — a Chief Architect reading a not-yet-implemented resolution document and catching a design error before code existed. Sprint 11's own verification pass was a different, later-stage check: implementation was already complete, all four milestones individually reviewed and passing, and the explicit instruction was to verify the *finished* implementation against the architecture document *adversarially* — to attempt to disprove consistency rather than confirm it — before any repository closeout work began.

**Observation:** this adversarial-verification framing found something the four individual milestone reviews, each conducted in good faith against the architecture document as it stood at the time, did not: the Architectural Invariants section (added to the architecture document *during* the implementation milestones, as a Chief Architect-requested refinement) asserted a general platform-wide recognition model — every unrecognized observation remains Evidence — that quietly contradicted the Collector-level recognition contract an earlier section of the same document, and the already-completed Milestone 1 implementation, both already used. No single milestone review re-checked the *whole* document for internal consistency against the *whole* implementation; each checked its own slice. The inconsistency was only visible from a vantage point deliberately looking for contradiction across the complete, finished picture.

**Why this is being recorded, not adopted:** this is the first data point for a *different* class of review than Sprints 8–9 recorded (verify finished implementation against documentation, adversarially, as its own distinct step) rather than the class already observed twice (review a not-yet-implemented resolution before coding begins). Per this project's own standing bar, one instance does not justify writing a new mandatory stage into the canonical workflow. It is recorded here as a candidate: a documentation section added mid-implementation (as this Sprint's Architectural Invariants section was) may need to be explicitly re-checked against sections written earlier and against work already in progress, not merely reviewed for its own internal quality — a future Chief Architect session should decide whether this warrants its own named stage once a second instance exists to compare against.

### Sprint 12: Adversarial Verification Applied to a Planning Document With Zero Implementation Involved

Sprint 11's own observation (above) recorded adversarial verification checking *finished implementation* against documentation. Sprint 12 is a further, distinct instance of the same discipline applied one level further back: no implementation existed at all — the artifact under adversarial review was a freshly-derived architectural *procedure itself*, checked against the platform's own historical decisions rather than against code.

**Observation:** applying the same "attempt to disprove rather than confirm" discipline to a purely retrospective, evidence-derived document — not a document describing not-yet-built work, and not a document describing already-built work — found a real flaw a good-faith derivation session had missed: the original Capability Identity procedure was validated against six decisions *by their own headline labels* ("Sprint 4: ArchiveCollector") rather than against every separable decision each label actually contains. Re-examining "Sprint 4" one level deeper exposed a second, distinct decision (Phase 3C's `StructuralDuplication` category) that the headline-level validation had not separately tested, and that decision falsified the procedure's own central assumption.

**Why this matters as its own data point, not a restatement of Sprint 11's:** Sprint 9's finding (architectural review, before code) and Sprint 11's finding (adversarial verification, after code) both involved a *forward-looking or already-built* artifact with real implementation stakes. Sprint 12's finding involved neither — it is evidence that this project's adversarial-verification discipline finds real value even when applied to a document whose only "implementation" is the historical record itself, and specifically that **validating a derived procedure only against summary labels, rather than against every decision a label may compress, is a real failure mode**, independent of whether code is involved at all.

**Why this is being recorded, not adopted as a new mandatory stage:** three related but distinct instances now exist (Sprint 9: pre-implementation architectural review; Sprint 11: post-implementation adversarial verification; Sprint 12: adversarial verification of a purely retrospective derivation) — convergent evidence that dedicated adversarial review finds real errors across every stage this project's workflow has, not proof that any one specific procedural step should be made permanently mandatory beyond what already exists. Whether "do not validate against headline Sprint labels alone; check separable sub-decisions" should become an explicit checklist item in future historical-evidence derivations is left for a future Chief Architect session to decide, ideally once a fourth instance exists.

### Sprint 13: A Historically-Derived Procedure's Own Blind Spot, Found by a Different Kind of Check Than Sprint 12's

Sprint 12 found that the Capability Identity procedure's first draft had been validated against headline Sprint labels rather than every separable sub-decision each label compresses. Sprint 13 found a different, related gap in the same procedure: it was validated only against Collector/Rule-shaped historical decisions, and never checked against a decision the repository had *already made and already named* in a different vocabulary — Sprint 8's own "Architectural Activation" classification of Version Profiles' own first real content, recorded in `SPRINT8_ARCHITECTURAL_RESOLUTION.md` §8 a full Sprint before Capability Identity was derived.

**Observation:** this gap was not found by re-examining a headline label more closely (Sprint 12's own method) — it was found by directly asking, on Chief Architect instruction, "does the repository already contain an implicit distinction for this shape of question, before proposing anything new?" That question surfaced a fact neither `INV-002` nor the first draft of the Governance Observation had found on their own: the repository did not lack a governing concept for subsystem activation, it had one, coined and correctly applied five Sprints earlier, simply never cross-referenced by the newer, more heavily-verified procedure that superficially resembled it. Sprint 12's own adversarial-verification discipline — checking a model against every named historical decision — would only have caught this if "Architectural Activation" had been included in the seven decisions checked; it was not, because Sprint 12's own charter named seven Collector/Rule decisions specifically, not a search across the repository's full documentary history for adjacent, already-solved questions.

**Why this matters as its own data point:** every prior instance in this section (Sprint 9, 11, 12) involved checking a candidate model against a fixed, already-identified evidence set more rigorously. Sprint 13's finding is upstream of that — it is evidence that a well-verified model can still have a scope blind spot if the evidence set it was checked against was itself incompletely scoped from the start, and that closing this kind of gap sometimes requires a direct, deliberately open question ("does an answer already exist?") rather than a more rigorous pass over the same bounded evidence already assembled.

**Recorded as a Repository Review Question, not a rule:** a rule binds what must happen next and needs convergent evidence before being adopted, per this project's own standing bar; a question only shapes what gets asked, and does not need that bar to be worth asking. Added to `CHIEF_ARCHITECT_HANDOFF_v1.1.md` §5's Engineering Review Philosophy directly, not deferred pending a future instance: **"Has the repository already answered this question, perhaps under a different subsystem, decision, or historical document?"** — asked before deriving any new governing concept. This keeps the practice lightweight while encouraging the behavior this Sprint demonstrated, without waiting for a third instance the way a candidate for the standing workflow itself would.
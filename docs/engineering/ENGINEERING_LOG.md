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
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
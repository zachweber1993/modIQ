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
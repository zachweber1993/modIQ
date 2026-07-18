# Sprint 2 Implementation Plan

| Property | Value |
|----------|-------|
| **Document** | SPRINT2_IMPLEMENTATION_PLAN.md |
| **Project** | modIQ |
| **Sprint** | Sprint 2 |
| **Status** | Approved — Pending Governance Prerequisites |
| **Predecessor Documents** | HANDOFF_SPRINT1.md, ENGINEERING_RELEASE_v0.1.0-alpha.md |
| **Last Updated** | 2026-07-18 |

---

This document is the authoritative implementation contract for Sprint 2. It is not a proposal, a backlog, or a summary of design discussion. Any engineer implementing Sprint 2 should be able to do so from this document and the Engineering Specification alone, without reference to prior design conversation.

---

# Sprint Objective

Sprint 1 proved that the modIQ Assessment pipeline could execute end to end — creation, evidence collection, rule evaluation, finding generation, recommendation generation, reporting, and completion — using content-free marker types for Evidence, Finding, and Recommendation. That sprint validated the architecture's lifecycle machinery. It did not validate the platform's ability to represent an actual engineering judgment.

Sprint 2 exists to close that gap. Its architectural objective is to give the Runtime Domain's three evaluative entities — Evidence, Finding, and Recommendation — real, evaluable content, so that the Assessment aggregate can carry an actual objective observation, an actual engineering conclusion, and actual actionable guidance, rather than a placeholder value standing in for each.

This capability is a prerequisite for nearly every other capability currently deferred in the platform: multi-rule dispatch, explainability output, cross-entity traceability, and Knowledge Domain integration are all blocked on Evidence and Finding carrying inspectable content. Sprint 2 does not attempt any of those capabilities. It delivers only the Runtime Domain content model they all depend on.

---

# Scope

## In Scope

- `EvidenceId`, `FindingId`, `RecommendationId` identity types, following the existing `AssessmentId` pattern.
- `EvidenceCategory` enum.
- `FindingSeverity` enum.
- Fielded `Evidence`: `id`, `category`, `description`, `location`.
- Fielded `Finding`: `id`, `severity`, `description`, `evidence_ids`, `rule_reference`.
- Fielded `Recommendation`: `id`, `finding_ids`, `action`, `repair_recipe_reference`.
- `RuleReference` and `RepairRecipeReference` — lightweight, opaque reference types carrying no dependency on `modiq-knowledge`.
- `Assessment` aggregate integration: updated `add_evidence`, `add_finding`, `add_recommendation` signatures.
- Enforcement of the two governance-approved content-level invariants (see Governance Prerequisites).
- Unit tests for all new types.
- Aggregate-level invariant and regression tests.
- Mechanical updates to existing test suites in `modiq-runtime`, `modiq-rules`, and `modiq-report` that currently construct `Evidence`, `Finding`, and `Recommendation` as content-free marker values.

## Out of Scope

- Rule abstraction, `RuleSelector`, or support for multiple Rules.
- Knowledge Domain integration (`modiq-knowledge` wiring into `modiq-rules`).
- An explainability engine or reasoning/rationale fields beyond the approved `description` fields.
- Reporting evolution (`FindingSummary`, `RecommendationSummary`, `ReportFormatter`, `TraceabilityReport`).
- Persistence, storage architecture, or any persistent identity scheme.
- CLI wiring (`modiq-cli` to `modiq-engine`).
- Version Profile integration (`modiq-versioning`).
- Resolution of `modiq-common`'s architectural purpose (GOV-003).
- Resolution of Engine service granularity (GOV-004).
- Resolution of Assessment Report generation timing (GOV-001).

---

# Approved Domain Model

Full design rationale — field-by-field justification, naming rationale, and identity rationale in complete form — is recorded in the Runtime Domain Modeling design record for Sprint 2 and is not duplicated here. This section states only the approved shape.

## Evidence

- **Purpose** — an objective, neutral, uninterpreted record of a single fact observed about the Assessment Subject. Evidence never carries interpretation, severity, or recommendation.
- **Ownership** — owned exclusively by `Assessment`. Constructed by evidence-gathering mechanisms outside the Runtime Domain and accepted only through `Assessment::add_evidence`.
- **Identity** — `EvidenceId`, generated using the existing `AssessmentId` pattern (a process-local monotonic identifier). Required so that a Finding can reference the specific Evidence it was derived from once Evidence carries real, potentially similar-looking content.
- **Approved fields** — `id: EvidenceId`; `category: EvidenceCategory`; `description: String`; `location: Option<...>`.
- **Invariants** — immutable once accepted; `id` unique within the owning Assessment; `category` restricted to the closed `EvidenceCategory` set.
- **Traceability responsibilities** — Evidence is a traceability *target*, not a source. It is referenced by `Finding.evidence_ids`. It holds no reference of its own to any Finding, Rule, or Knowledge entity.

## Finding

- **Purpose** — an interpreted engineering conclusion produced by deterministic rule evaluation of Evidence.
- **Ownership** — owned exclusively by `Assessment`. Produced by `modiq-rules` and accepted only through `Assessment::add_finding`.
- **Identity** — `FindingId`, same pattern as `EvidenceId`. Required so Recommendations, and future Reports, can reference a specific Finding rather than relying on structural equality.
- **Approved fields** — `id: FindingId`; `severity: FindingSeverity`; `description: String`; `evidence_ids: Vec<EvidenceId>`; `rule_reference: RuleReference`.
- **Invariants** — immutable once accepted; must reference at least one `EvidenceId` (governance-pending — see Governance Prerequisites); every referenced `EvidenceId` must correspond to Evidence already collected in the same Assessment.
- **Traceability responsibilities** — references its supporting Evidence (`evidence_ids`) and the Rule that produced it (`rule_reference`). Is in turn referenced by `Recommendation.finding_ids`. Sits at the midpoint of the Evidence → Finding → Recommendation traceability chain.

## Recommendation

- **Purpose** — actionable, non-mandatory guidance derived from one or more Findings.
- **Ownership** — owned exclusively by `Assessment`. Produced by `modiq-rules` and accepted only through `Assessment::add_recommendation`, which already requires a prior Finding to exist.
- **Identity** — `RecommendationId`, same pattern as `EvidenceId` and `FindingId`. Justification is narrower than for Evidence or Finding — no current entity references a Recommendation by identifier — and is granted primarily for reporting and traceability symmetry, and for forward compatibility with `modiq-report`'s `TraceabilityReport`.
- **Approved fields** — `id: RecommendationId`; `finding_ids: Vec<FindingId>`; `action: String`; `repair_recipe_reference: Option<RepairRecipeReference>`.
- **Invariants** — immutable once accepted; must reference at least one `FindingId` that exists in the same Assessment (the INV-005 refinement — governance-pending, see Governance Prerequisites).
- **Traceability responsibilities** — references its originating Finding(s) (`finding_ids`) and, optionally, an informing Repair Recipe. Represents the terminal node of the traceability chain.

---

# Governance Prerequisites

Two items must be resolved through the governance process defined in `GOVERNANCE.md` before their corresponding enforcement code may be written. Implementation of all other approved fields and types is not blocked by these items.

## 1. New Finding Invariant

**Requirement:** every Finding must reference at least one `EvidenceId`.

**Why this requires governance approval:** `RuntimeInvariants.md` currently defines no invariant of this kind — it stops at INV-012. Introducing a new invariant is a Level 3 (Behavioral) change under `GOVERNANCE.md`'s Change Categories, which requires governance approval, a documentation update, implementation, and engineering review — not something to introduce inline as part of field modeling.

## 2. INV-005 Refinement

**Requirement:** a Recommendation must reference the specific `FindingId`(s) it was derived from, not merely coexist in an Assessment where some Finding exists.

**Why this requires governance approval:** INV-005 currently reads "Recommendations SHALL only be produced from one or more Findings" and is enforced today only as a temporal precondition — that *some* Finding exists in the Assessment at the time a Recommendation is added. Requiring a specific, content-level reference changes the enforcement semantics of an existing, named invariant. `GOVERNANCE.md` names invariant changes explicitly as a Level 3 (Behavioral) example. This item is also directly connected to the already-open GOV-002 (Runtime Invariant Reconciliation) and should be resolved alongside it where practical.

**Implementation may not proceed on either item's enforcement code until both are marked Resolved in the Governance Register and `RuntimeInvariants.md` is updated accordingly.**

---

# Implementation Sequence

## Phase 1 — Governance Updates

- **Objective** — resolve both Governance Prerequisites before any invariant-enforcement code is written.
- **Affected documents** — `GOVERNANCE.md` (Governance Register), `RuntimeInvariants.md`.
- **Expected deliverables** — new Governance Register entries for both items; a recorded governance decision; `RuntimeInvariants.md` updated with the approved invariant text (a candidate INV-013 for the new Finding invariant, and revised wording for INV-005).
- **Completion criteria** — both items marked Resolved in the Governance Register; `RuntimeInvariants.md` reflects the approved text.

## Phase 2 — Runtime Types

- **Objective** — implement the approved identity and content types.
- **Affected crates** — `modiq-runtime`.
- **Expected deliverables** — `EvidenceId`, `FindingId`, `RecommendationId`; `EvidenceCategory`; `FindingSeverity`; `RuleReference`; `RepairRecipeReference`; fielded `Evidence`, `Finding`, `Recommendation` types.
- **Completion criteria** — all new types compile in isolation; construction-level unit tests pass; no dependency is added to `modiq-runtime` beyond `thiserror`.

## Phase 3 — Assessment Integration

- **Objective** — update the aggregate's mutation methods to accept the newly-fielded types.
- **Affected crates** — `modiq-runtime` (`Assessment`).
- **Expected deliverables** — updated `add_evidence`, `add_finding`, `add_recommendation` signatures; existing lifecycle gating preserved exactly as implemented in Sprint 1.
- **Completion criteria** — the workspace compiles; all Sprint 1 lifecycle and ownership tests continue to pass, updated only in construction syntax, not in assertion intent.

## Phase 4 — Runtime Invariants

- **Objective** — enforce the two governance-approved content-level invariants.
- **Affected crates** — `modiq-runtime` (`Assessment`).
- **Expected deliverables** — `add_finding` rejects a Finding whose `evidence_ids` is empty or references Evidence not present in the Assessment; `add_recommendation` rejects a Recommendation whose `finding_ids` is empty or references a Finding not present in the Assessment.
- **Completion criteria** — both new rejection paths follow the existing check-completion → check-lifecycle → check-data-precondition → mutate shape; Phase 1 shows both prerequisites Resolved before this phase begins.

## Phase 5 — Testing

- **Objective** — bring full workspace test coverage in line with the new content model.
- **Affected crates** — `modiq-runtime`, `modiq-rules`, `modiq-report`, `modiq-engine` (test code only).
- **Expected deliverables** — new unit tests for every new type; new invariant-rejection tests for both governance-approved invariants; mechanical updates to every existing test that constructs `Evidence`, `Finding`, or `Recommendation` as a content-free marker value.
- **Completion criteria** — `cargo fmt` produces no diff; `cargo check --workspace` is clean; `cargo test --workspace` passes with zero failures.

---

# Testing Strategy

**Unit testing expectations.** Every new type (`EvidenceId`, `FindingId`, `RecommendationId`, `EvidenceCategory`, `FindingSeverity`, `RuleReference`, `RepairRecipeReference`, and the fielded `Evidence`, `Finding`, `Recommendation` structs) receives construction- and accessor-level unit tests independent of the `Assessment` aggregate.

**Invariant testing.** Every precondition introduced in Phase 4 — Finding requires at least one valid `EvidenceId`; Recommendation requires at least one valid `FindingId` — is tested with both a positive case (the precondition is satisfied) and a negative case (the precondition is violated), matching the structure already established for INV-002 through INV-012 in the Sprint 1 test suite.

**Lifecycle testing.** All existing lifecycle-gating tests (`add_evidence` only during `CollectingEvidence`, `add_finding`/`add_recommendation` only during `EvaluatingRules`, all mutation rejected after `Completed`) must continue to pass unchanged in intent. Only their construction of `Evidence`, `Finding`, and `Recommendation` values may change, not their assertions.

**Regression testing.** The full workspace suite — `modiq-runtime`, `modiq-rules`, `modiq-report`, `modiq-engine` — must pass after every phase. No phase is considered complete if it regresses a previously passing test.

The existing testing philosophy from Sprint 1 is preserved without exception into Sprint 2:

> **Failed operations must leave aggregate state unchanged.**

Every new rejection path introduced by this plan must assert this directly — not only that an error is returned, but that the Assessment's Evidence, Finding, and Recommendation collections are identical before and after the rejected call.

---

# Architectural Constraints

Implementation under this plan may not violate any of the following:

- Runtime owns state. `Assessment` remains the sole owner of Evidence, Finding, and Recommendation collections; no other crate may hold or mutate them directly.
- Rules evaluate. `modiq-rules` may consume Runtime Domain types but may never mutate an `Assessment` directly.
- Reports render. `modiq-report` performs no analysis and generates no new state; it only reflects state that already exists.
- Knowledge remains external. `rule_reference` and `repair_recipe_reference` are opaque reference values. `modiq-runtime` gains no dependency on `modiq-knowledge`, and no Knowledge entity is owned or embedded by any Runtime Domain type.
- No new crate dependencies. `modiq-runtime` retains its single external dependency (`thiserror`). No identity or content type introduced by this plan requires a new external crate.
- Reuse the `AssessmentId` identity pattern. `EvidenceId`, `FindingId`, and `RecommendationId` are process-local monotonic identifiers generated the same way as `AssessmentId`. No UUID or other external identity scheme may be introduced.
- Preserve deterministic behavior. Identical inputs to any mutation method must continue to produce identical outcomes, per ADR-0005.
- Preserve aggregate ownership boundaries. Every new mutation path follows the existing check-completion → check-lifecycle → check-data-precondition → mutate shape. No new mutation path may bypass `Assessment`.
- No undocumented public API changes. Every field, type, and method added under this plan is recorded in this document before implementation; nothing may be added silently.

---

# Deferred Architectural Decisions

- **Rule abstraction.** Not introduced. A trait or similar abstraction for `Rule` has exactly one implementor today; introducing one now would be structure with no present consumer. Deferred until a second Rule creates an actual need.
- **Multi-rule dispatch.** Not introduced. `EvidenceCategory` and `rule_reference` are modeled specifically so this decision can be made later without restructuring Evidence or Finding, but no selection mechanism is built in Sprint 2.
- **Explainability engine.** Not introduced. `description` fields on Finding and Recommendation carry the conclusion and the action; a dedicated reasoning/rationale field is deferred until real explainability content exists to populate it.
- **Knowledge integration.** Not introduced. `rule_reference` and `repair_recipe_reference` remain populated with local, `modiq-rules`-internal values until `modiq-knowledge` is wired into `modiq-rules`, which remains its own, separately governed body of work.
- **Persistence identity.** Not introduced. All identifiers defined by this plan are process-local and are not designed to survive a process restart. A persistent identity scheme is a Storage Layer decision, and the Storage Layer remains architecturally undesigned; deciding its identity scheme from within the Runtime Domain would be implementation redefining architecture.
- **Report evolution.** Not introduced. `FindingSummary`, `RecommendationSummary`, `ReportFormatter`, and `TraceabilityReport` remain stubs. They become meaningful once this plan's content model lands, but building them out is separate, later work.

---

# Success Criteria

Sprint 2 is complete only when all of the following are true:

- The approved Evidence, Finding, and Recommendation models exist exactly as specified in this document.
- Both Governance Prerequisites are marked Resolved, and `RuntimeInvariants.md` reflects their approved text.
- `Assessment` aggregate integration is complete, with all four mutation methods (`add_evidence`, `add_finding`, `add_recommendation`, and the existing lifecycle transitions) enforcing their full, updated precondition sets.
- `cargo fmt`, `cargo check --workspace`, and `cargo test --workspace` all pass cleanly across the full workspace.
- Every architectural boundary defined in `GOVERNANCE.md` remains exactly as it was at the close of Sprint 1 — no crate has gained a new dependency or a new responsibility outside what this plan specifies.

---

# Exit Review Checklist

☐ Architecture preserved — no crate boundary redefined, no dependency added outside this plan

☐ Governance items completed — both prerequisites Resolved in the Governance Register

☐ Runtime invariants reconciled — `RuntimeInvariants.md` updated to match enforced behavior

☐ Aggregate ownership unchanged — `Assessment` remains sole owner of all Runtime Domain state

☐ No crate boundary violations — Runtime/Rules/Reporting/Knowledge separation intact

☐ No undocumented public API changes — every new type and field traceable to this document

☐ Tests passing — `cargo fmt`, `cargo check --workspace`, `cargo test --workspace` all clean

☐ Engineering Log updated — Sprint 2 entry added following the established entry format

☐ Engineering Release prepared — successor to `ENGINEERING_RELEASE_v0.1.0-alpha.md` drafted

# Proposal: GOV-005 and GOV-006 Resolution

**Stage:** Proposal (first stage of the Architectural Review Process, `GOVERNANCE.md`)

**Prepared by:** Engineering, for Technical Director review

**Governance Items addressed:** GOV-005, GOV-006 (`docs/engineering/GOVERNANCE.md`)

**Status:** Awaiting Architecture Review and Governance Decision. No documentation or implementation change has been made as part of this proposal.

---

## Why now

Sprint 3 Phase 1 gave Sandbox a real, non-test consumer of the pipeline, so `Finding.evidence_ids` and `Recommendation.finding_ids` are now being exercised by something other than test code for the first time. Both governance items were deliberately left open at Sprint 2 pending exactly this kind of real usage. Nothing about Sprint 3 Phase 1 depends on resolving them — this proposal is being raised because the fork was flagged as a decision point for the next phase, not because current behavior is broken.

## Current behavior (verified against source, 2026-07-19)

- `Finding::new` (`crates/modiq-runtime/src/assessment/finding.rs:35-53`) validates only that `description` is non-empty. `evidence_ids` is accepted verbatim, including `vec![]`. The doc comment on the field already states this is "pending governance approval."
- `Recommendation::new` (`crates/modiq-runtime/src/assessment/recommendation.rs:32-48`) is the same shape: only `action` is validated; `finding_ids` is accepted as-is, including empty.
- `Assessment::add_finding` (`assessment.rs:208-221`) checks lifecycle state only — no inspection of `finding.evidence_ids()` content.
- `Assessment::add_recommendation` (`assessment.rs:234-254`) checks lifecycle state, plus one content-adjacent precondition: `self.findings.is_empty()` — i.e., *some* Finding exists in the Assessment. It does not check that `recommendation.finding_ids()` is non-empty or that any id in it resolves. This is the literal gap INV-005 as currently worded leaves open.
- `Assessment::evidence_for_finding` and `Assessment::findings_for_recommendation` (`assessment.rs:114-120`, `132-138`) both use `filter_map` to silently drop unresolvable ids. An all-dangling reference list and a legitimately-empty reference list currently resolve to the same observable result: an empty `Vec`.
- Existing tests assert the permissive behavior directly as correct (e.g. `finding.rs:126` `new_currently_permits_empty_evidence_ids_pending_governance_approval`; `assessment.rs:994` `evidence_for_finding_omits_evidence_ids_that_do_not_resolve`). No test currently expects an error from an empty or dangling reference.
- `modiq-rules::RuleEngine::evaluate` (`crates/modiq-rules/src/rules/engine.rs:27-51`), the only current Rule, always populates non-empty, internally-consistent references when it produces a Finding/Recommendation at all — `evidence_ids` from the real evidence slice it was given, `finding_ids` as `vec![finding.id()]` from the Finding it just built. It has no code path that produces empty or dangling references. In other words: **today's only Rule already satisfies both proposed invariants**; the gap is that nothing in `modiq-runtime` requires it to.

## GOV-005 — Finding → Evidence reference requirement

**Question:** should a Finding be required to reference at least one Evidence item, validated against the Assessment's own Evidence collection?

Two independent sub-questions worth separating, since they have different costs:

1. **Cardinality** — must `evidence_ids` be non-empty? Cheap: a constructor-level check in `Finding::new`, consistent with the existing "reject empty description" pattern.
2. **Referential integrity** — must every id in `evidence_ids` resolve to Evidence that actually exists in the same Assessment? More expensive: `Finding::new` has no Assessment to check against (Finding is constructed before it's added), so this can only be enforced at `Assessment::add_finding` time, not at construction. That changes `add_finding`'s error surface and requires the Assessment to look up each id before accepting the Finding.

**Options:**

- **A — No new invariant.** Leave both permissive. Lowest cost, but the field remains decorative for anything that isn't already well-behaved, and a future second Rule (or a bug in the first) could silently produce untraceable Findings with no enforcement anywhere.
- **B — Cardinality only.** Reject empty `evidence_ids` in `Finding::new`. Does not require Assessment involvement; keeps the referential-integrity question open as a separate, later item. Matches the existing validation style exactly (same shape as the description check already there).
- **C — Cardinality + referential integrity.** Move (or duplicate) validation into `Assessment::add_finding`, since only the aggregate can check resolvability. Fully closes the gap but is a larger, Level 3 behavioral change: new `AssessmentError` variant(s), a new precondition ordering question (does this check happen before or after the existing lifecycle checks?), and it changes where "a Finding is invalid" can be discovered — at construction (B) vs. at attachment (C).

## GOV-006 — INV-005 refinement (Recommendation → Finding reference)

**Question:** should INV-005 be refined to require content-level reference validation, and is that a wording change to INV-005 itself or a new invariant?

Same two sub-questions apply (cardinality on `finding_ids`, referential integrity against the Assessment's Findings), plus one that's specific to GOV-006: INV-005 currently reads "Recommendations SHALL only be produced from one or more Findings," and today's implementation already satisfies that literally — `add_recommendation` requires `self.findings` to be non-empty. The proposed refinement doesn't contradict the existing wording so much as tighten what "produced from" means (temporal coexistence → specific reference). That argues for a **new, separate invariant** (e.g. INV-013) rather than editing INV-005's text, since INV-005 as worded is not wrong today, only under-specified for this new case — and `RuntimeInvariants.md` invariants are not meant to be rewritten out from under prior meaning.

**Options mirror GOV-005:** (A) no new invariant, (B) cardinality only in `Recommendation::new`, (C) cardinality + referential integrity in `Assessment::add_recommendation`.

## Interaction between the two

If either graduates from A, the other should be decided at the same time rather than staggered — they're structurally identical questions (Finding↔Evidence, Recommendation↔Finding), and resolving one while leaving the other open would produce an inconsistent aggregate: one child entity type enforced, the sibling type not, for no principled reason. `RuntimeInvariants.md` groups INV-001 through INV-005 together under Assessment Lifecycle for the same kind of reason.

## Engineering recommendation

**Option B for both** (cardinality only; `Finding::new` and `Recommendation::new` reject empty `evidence_ids`/`finding_ids`), leaving referential integrity (Option C's assessment-level resolvability check) as a distinct, separately-numbered governance item rather than bundling it into this round. Reasoning:

- It's the smallest change that actually closes the "field exists but nothing requires it to be meaningful" gap the items were raised for.
- It requires no change to `Assessment`'s method signatures, error types, or precondition ordering — contained entirely inside the two existing constructors, in the same validation style already used for `description`/`action`.
- It doesn't foreclose Option C later; referential integrity can be layered on as its own governance item once there's a concrete reason to need it (e.g., a second Rule that could plausibly produce a dangling reference — which the current, sole Rule structurally cannot).
- Full referential integrity (C) is real work with real design questions (new error variants, precondition ordering, whether the check belongs in `add_finding`/`add_recommendation` or a new dedicated method) that shouldn't be decided as a side effect of this round.

This is a recommendation for Architecture Review, not a decision — per `GOVERNANCE.md`, this is a Level 3 (Behavioral) change requiring governance approval and a documentation update (`RuntimeInvariants.md`, and GOV-005/GOV-006's Resolution fields) before any implementation. No code has been written against this proposal.

## If approved

Expected scope, for planning only (not started):

- `RuntimeInvariants.md`: add invariant text for the Finding↔Evidence cardinality requirement and a new INV-013 for the Recommendation↔Finding cardinality requirement (pending exact numbering/wording from Documentation Release).
- `GOVERNANCE.md`: update GOV-005 and GOV-006 Resolution fields; record the Documentation Release that closes them.
- `modiq-runtime`: add one new `FindingError` variant and one new `RecommendationError` variant; add the empty-check to each constructor; update the doc comments currently marked "pending governance approval" (`finding.rs`, `recommendation.rs`, `assessment.rs:106-113,122-131`) to reflect enforced status; existing permissive tests (`finding.rs:126`, `recommendation.rs:108`) would need to become error-path tests instead of success-path tests.
- `modiq-rules`: no change expected — `RuleEngine::evaluate` already only ever produces non-empty reference lists.
- `apps/sandbox`: no change expected — it does not construct `Finding`/`Recommendation` directly.

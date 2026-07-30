# Initiative 2 Governance Reconciliation — Question 1

| Property | Value |
|---|---|
| **Document** | INITIATIVE_2_GOVERNANCE_RECONCILIATION.md |
| **Project** | modIQ |
| **Initiative Addressed** | Engineering Alignment Program, Initiative 2 — Reentrant Assessment Lifecycle, Question 1 ("Requires Governance Reconciliation" per `INITIATIVE_2_ARCHITECTURAL_RESOLUTION.md`) |
| **Purpose** | Complete the Governance Reconciliation that `INITIATIVE_2_ARCHITECTURAL_RESOLUTION.md` deferred: determine whether Question 1 can be resolved under the repository's current governance model, using only adopted repository evidence. |
| **Origin** | Chief Architect authorization, following the evidence inventory of architecture adopted since Initiative 2's Resolution. |
| **Adopted Precedent** | `INITIATIVE_1_ARCHITECTURAL_RESOLUTION.md`, `INITIATIVE_2_ARCHITECTURAL_RESOLUTION.md`, `INITIATIVE_3_ARCHITECTURAL_RESOLUTION.md`, all treated as binding. This document completes what Initiative 2's Resolution deliberately left open — it does not reopen it. |
| **Status** | **Governance Reconciliation complete. No Documentation Authority, governance process, or architectural document is amended by this document. No new disposition vocabulary is introduced — see §9.** |

---

## §1 Scope of This Document

Reconciles Question 1 only: whether `AssessmentStatus::Completed` should evolve to permit reentrancy, or remain terminal under a new-Assessment mechanism. Does not reopen Question 1's surrounding findings (Q2, Q4, Q5, Q6), all already adopted in `INITIATIVE_2_ARCHITECTURAL_RESOLUTION.md` and unaffected here. Does not evaluate Initiative 4. Does not amend `GOVERNANCE.md`.

---

## §2 Authority Hierarchy for the Available Evidence

`GOVERNANCE.md`'s Documentation Authority section, quoted exactly:

> "When two documents conflict, precedence shall be determined in the following order. 1. Vision 2. Product Specification 3. Architecture 4. Data Model 5. Principles 6. Architecture Decision Records (ADRs) 7. Governance 8. Engineering Release Documents 9. Engineering Log 10. Source Code. Implementation shall never be considered authoritative when it conflicts with higher-level documentation. Instead, the discrepancy shall be recorded as a Governance Item and resolved through the governance process."

Confirmed against the repository's actual document structure: `docs/constitutional/ProductSpecification.md` (tier 2) is a distinct file from `docs/product-design/THE_ASSESSMENT_REPORT.md`, which does not appear anywhere in the ten tiers. `docs/architecture/DataModel.md` (tier 4) is ranked. `docs/platform/PlatformSpecification.md` and its governed documents do not appear in the ten tiers either.

A second, orthogonal authority track exists: adopted Architectural Resolutions are binding by governance-process convention, independent of Documentation Authority rank.

---

## §3 Scope Boundaries for Each Architectural Lineage

- **Runtime Architecture** — "owns everything at or beneath [the Assessment boundary]: Assessment Subject, Evidence, Finding, Recommendation, and Assessment Report" (`PlatformSpecification.md`). Question 1's subject matter sits entirely within this lineage.
- **Product Design** — "the conceptual model of the Assessment experience" (`PlatformSpecification.md`). `THE_ASSESSMENT_REPORT.md` sits here, describing intended experience, not Runtime mechanism.
- **Platform Architecture** — "owns everything above [the Assessment boundary]... does not redefine it, model its internal structure, or name what lies beneath it beyond this boundary statement" (`PlatformSpecification.md`).
- **Engineering Alignment** — "Owns no platform or runtime domain concept. Owns the process reconciling Runtime Architecture as implemented against the frozen Product Design and Interaction Design corpus" (`PlatformSpecification.md`).

---

## §4 Evaluating Apparent Tension Between Lineages

Four distinctions applied throughout this reconciliation: repository evidence vs. architectural inference; governance authority vs. architectural relevance; structural relationships vs. lifecycle semantics; Product intent vs. Runtime authority.

---

## §5 GQ-A1 — Resolved

**Question:** does omission from the Documentation Authority hierarchy mean an unlisted document carries no precedence weight, or that the hierarchy provides no rule for that comparison?

**Resolution:** the quoted text orders ten named tiers and provides a fallback only for implementation-vs-documentation conflicts. It contains no statement addressing an unlisted document's standing. **Omission means the hierarchy provides no rule for that comparison** — not that the unlisted document is subordinate. Reading omission as subordination would be a rule the text does not state.

GQ-A2 (whether the hierarchy should be amended) is explicitly not resolved here — see §10.

---

## §6 GQ-B — Resolved

**Question:** does `PlatformSpecification.md`'s disclaimer of "an implicit position" on the terminal-completion-state question exclude its own Upload→Assessment structural claim from bearing on Question 1?

**Resolution:** `ProjectsAndUploads.md` (adopted, governed under `PlatformSpecification.md`) resolves this directly:

> "A desire to reassess material already submitted is satisfied by a new Upload, triggering its own Assessment; the original Upload and the Assessment it triggered are never retargeted or reopened. This keeps the Platform Architecture model neutral on whether that new Assessment continues or restarts for the same Assessment Subject — an open Runtime Architecture question this document does not take a position on."

This names "continues or restarts" explicitly — Question 1's own substance — and states neutrality directly, not by inference. **The disclaimer succeeds. Platform Architecture evidence is excluded from Question 1's substantive weighing.**

---

## §7 Application to the Evidence

With Platform Architecture excluded (§6) and Documentation Authority inapplicable to a ranked/unranked comparison (§5), the evidence reduces to the same direct conflict Initiative 2's Resolution already identified:

- `DataModel.md`'s Immutability principle: "a new Assessment should be performed rather than modifying previous Assessment results."
- `THE_ASSESSMENT_REPORT.md` §2: "Submitting more material doesn't produce a second Report or a new version of one... No new mechanism is needed for long-absence returns... regardless of whether the gap was five minutes or five months."

No other ranked, adopted architecture bears on the comparison: ADR-0007 governs the aggregate/entity pattern, not lifecycle terminality (established via Initiative 3's Item 10 determination); GOV-015 was already checked and found structurally inapplicable as precedent; GOV-001 was already found unaffected.

---

## §8 Reconcilability Determination

**Question 1 cannot presently be reconciled under the repository's current governance model.** Not for lack of evidence — the evidence base is complete and has been fully weighed. It cannot be reconciled because Documentation Authority, the repository's only documented precedence mechanism for resolving conflicts between architectural documents, has no rule for a conflict where one side is ranked and the other is entirely absent from the ranking (§5), and no other adopted architecture supplies an independent basis (§6, §7). `DataModel.md` and `THE_ASSESSMENT_REPORT.md` remain in direct, unmediated conflict.

---

## §9 Governance Finding — Disposition Vocabulary

The current disposition vocabulary no longer appears sufficient to accurately describe Question 1's state. "Requires Governance Reconciliation" described evidence-gathering as complete with the authority decision not yet attempted; that authority decision has now been attempted and concluded, which none of the six existing dispositions (Adopted / Adopted Architectural Constraint / Deferred / Rejected / Requires Additional Investigation / Requires Governance Reconciliation) describes.

Whether this should be addressed by introducing a new disposition or by another governance mechanism is itself a governance decision, outside the scope of this document. No new disposition is defined here.

---

## §10 Governance Recommendations (Separate From This Reconciliation)

Recommending, for separate Chief Architect consideration and not as part of this document's own findings:

1. Whether the Documentation Authority hierarchy should be amended to rank Product Design, Interaction Design, and Platform Architecture (the question formerly GQ-A2), given this reconciliation is now a second, independent instance of that gap concretely preventing adjudication.
2. Whether the disposition vocabulary gap named in §9 should be addressed, and how.

Neither recommendation is adopted, drafted, or acted on here. Both belong to a separate governed process, not Initiative 2 or Engineering Alignment Program continuation.

---

## §11 Initiative 2 Closeout

Initiative 2 is complete. It exhausted the repository's currently documented authority framework for Question 1 and produced a fully evidenced negative finding rather than an assumed or inferred one. `INITIATIVE_2_ARCHITECTURAL_RESOLUTION.md` is not amended — it accurately recorded what was known when written; this document completes what it deliberately left open.

The Engineering Alignment Program is **not** complete: Initiative 4 (Confidence as a First-Class Concept) has not been evaluated.

---

## Reconciliation Boundaries

**Resolved here:** GQ-A1, GQ-B, and the reconcilability of Question 1 under current governance.

**Not resolved here:** the substantive architectural answer to Question 1 (reentrant vs. new-Assessment); GQ-A2; the disposition-vocabulary gap named in §9; Initiative 4.

**Adopted precedent relied upon without reopening:** Initiative 2's Q2/Q4/Q5/Q6 findings; Initiative 3's Item 10 determination (ADR-0007 scope boundary).

No repository governance was amended. This reconciliation's responsibility ends here, awaiting Chief Architect review.

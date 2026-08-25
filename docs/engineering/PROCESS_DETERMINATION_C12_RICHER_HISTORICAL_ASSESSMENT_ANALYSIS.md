# Process Determination — C12: Richer Historical Assessment Analysis

| Property | Value |
|---|---|
| **Document** | PROCESS_DETERMINATION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md |
| **Project** | modIQ |
| **Type** | Repository Process Determination — decides which repository artifact follows the Capability Definition. It does not authorize implementation, evaluate architecture in the sense of choosing a mechanism, design a schema, or select a migration strategy. |
| **Origin** | `docs/engineering/CAPABILITY_DEFINITION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md` (`dd4d377`), §9 "Architecture Evaluation Determination," which explicitly reserved this question — "the evidence does not support a definitive determination at this stage. This question is preserved for Process Determination, not resolved here" — and §15, which names this document as the next required repository artifact. |

---

## 1. Capability Identity

C12 — Richer Historical Assessment Analysis: extend `modiq-storage`'s existing `history_analysis` module so that its recurring-pattern aggregation can additionally account for a Finding's `ModHealthDimension`, alongside the already-existing `rule_reference`/`severity` aggregation. This identity is inherited unchanged from `CAPABILITY_DEFINITION_C12...md` §3 and is not re-derived here.

---

## 2. Governing Inputs

- `CAPABILITY_DEFINITION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md` (`dd4d377`) — the committed Capability Definition this document is answering the reserved question from.
- The repository's reusable two-prong test, introduced by `PROCESS_DETERMINATION_C1_RECOMMENDATION_STEP_PRESENTATION.md`, reused by `PROCESS_DETERMINATION_C3_CLI_FIELD_PARITY.md` and `PROCESS_DETERMINATION_C4_APPS_SANDBOX_FIELD_PARITY.md` — confirmed, by direct re-read of all three, to be a reusable precedent rather than a canonical `PROJECT_HANDOFF_v1.1.md` lifecycle stage.
- `CAPABILITY_DEFINITION_C2_DECLARED_DEPENDENCY_INTERPRETATION.md`, `C2_ARCHITECTURE_EVALUATION_DECLARED_DEPENDENCY_INTERPRETATION.md`, `C2_ARCHITECTURAL_RESOLUTION_DECLARED_DEPENDENCY_INTERPRETATION.md` — the repository's one committed instance of the alternate path, where a genuinely open question sent the lifecycle to Architecture Evaluation instead.
- Current source: `crates/modiq-storage/src/storage/persisted_report.rs`, `history_analysis.rs`, `report_store.rs`.
- `SPRINT24_IMPLEMENTATION_PLAN.md`, `SPRINT24_IMPLEMENTATION_REPORT.md`, `ENGINEERING_RELEASE_1.9.md`, `ENGINEERING_RELEASE_2.2.md`, `ENGINEERING_RELEASE_2.3.md` — the engineering record of a previously-named, still-unresolved Storage persistence-migration gap, re-read fresh for this document rather than carried forward from the preceding investigation's own account.
- `GOVERNANCE.md`, GOV-016 entry — checked to confirm it creates no process requirement here.

---

## 3. Existing Repository Foundation

Re-confirmed directly against current source this session, not assumed from the Capability Definition's own account:

- `PersistedFinding` (`persisted_report.rs:114-125`): exactly `severity, title, summary, evidence_indices, rule_reference`. No `mod_health_dimension` field; `from_finding` never reads `Finding::mod_health_dimension()`.
- `history_analysis::recurring_patterns(store: &ReportStore) -> Result<Vec<PatternFrequency>, ReportStoreError>` iterates `store.list_keys()`, and for each key calls `store.retrieve(&key)?` — a single `?` inside the loop, propagating any single report's retrieval failure as the whole function's own `Err`, aborting aggregation for every report in the store, not only the one that failed.
- `report_store.rs`: persistence is `serde_json::to_vec_pretty`/`serde_json::from_slice` against `PersistedAssessmentReport`. No schema-version field exists on `PersistedAssessmentReport` or any nested type. A repository-wide `grep` for `serde(default)` across `crates/` returns zero matches — no such mechanism exists anywhere in this codebase today, confirmed fresh for this document.
- `persisted_report.rs` contains four instances of a repeated sub-pattern — a closed Runtime enum mirrored into its own `Persisted*` enum with `Serialize`/`Deserialize` and a `From<RuntimeEnum>` conversion: `PersistedAssessmentStatus`, `PersistedEvidenceCategory`, `PersistedFindingSeverity`, `PersistedRecommendationStepKind`. `git log -S` on this file traces three field-shape touchpoints:
  - `e1406ec` — Storage's own founding (Phase 1, write/read capability).
  - `969e595` (Sprint 22) — `PersistedFinding.description` replaced by `title`/`summary` (a breaking rename, not an additive field).
  - `84ef4e3` (Sprint 24) — `PersistedRecommendation` gains a new **required** field, `repair_steps: Vec<PersistedRecommendationStep>`, introducing a brand-new enum mirror (`PersistedRecommendationStepKind`) for the occasion.

`84ef4e3` is, mechanically, the closest existing precedent to what C12 proposes: a new required field, added to an already-shipped `Persisted*` struct, requiring a new closed-enum mirror type. Considered in isolation, this precedent supports treating C12's own field addition as a mechanical, already-exercised extension of an established pattern.

---

## 4. Process Determination Method

This document applies the same reusable two-prong test C1, C3, and C4 each applied, per those documents' own stated instruction that the test is reusable and not a newly invented one each time:

1. Is there a genuinely open design question, or is the governing principle already Adopted?
2. Is any alternative being weighed?

Both prongs must resolve in favor of "no open question, no alternative" for the next artifact to be an Implementation Authorization. If either prong fails, the repository's own precedent (C2) is to proceed to Architecture Evaluation first.

**Precedent accuracy, confirmed fresh for this document:** C2 did not have a separate Process Determination artifact. A repository-wide `find` for `PROCESS_DETERMINATION_C2*` and `*C2*PROCESS*` returns nothing. `CAPABILITY_DEFINITION_C2_DECLARED_DEPENDENCY_INTERPRETATION.md`'s own "Next Required Repository Artifact" section performed this threshold analysis itself and concluded directly: "The repository-supported next artifact is therefore an Architecture Evaluation... not a Process Determination, whose own threshold question this capability's Required Analysis has already answered." This document does not restate that mischaracterization.

---

## 5. Prong 1 — Adopted Principle Analysis

**No existing Adopted principle non-discretionarily settles how `modiq-storage` should evolve already-persisted `Persisted*` records when a new field is added to an existing struct.**

This is not asserted from absence of a specific check; it is confirmed by a specific, repeated repository record of the same question being raised and left open, three separate times, without ever being escalated to a governing document:

- `SPRINT24_IMPLEMENTATION_PLAN.md` (§8, Risk 3), drafted *before* `repair_steps` was implemented, named the exposure directly: "`PersistedRecommendation` gains a required (non-`Option`) field with no `#[serde(default)]` — and no precedent for that mechanism exists anywhere in `modiq-storage` today... Any already-persisted report predating this Sprint will fail to deserialize on retrieval afterward... A persistence-migration or schema-versioning mechanism is new capability, not named in Authorization §3, and is not decided here — recorded as an accepted, pre-existing category of risk, not silently absorbed into this Sprint's scope."
- `SPRINT24_IMPLEMENTATION_REPORT.md` (line 167), written after implementation, restated it and escalated the framing: "The pre-existing persistence-migration gap (no `#[serde(default)]`, no schema versioning, anywhere in `modiq-storage`) now has two independent instances (`PersistedFinding` since Sprint 22, `PersistedRecommendation` since this Sprint) — **worth a dedicated evaluation before a third field-shape change compounds it further.**"
- `ENGINEERING_RELEASE_2.2.md` (C3, line 104) named the gap a third time, specifically and by name against C12's own proposed field: "Extending `modiq-storage`'s own `PersistedFinding`/`PersistedEvidence` schema to carry `mod_health_dimension`, `status`, or `label`/`source`/`content`... Carries its own real risk: `modiq-storage`'s pre-existing persistence-migration gap... unresolved and untouched by C3."
- `ENGINEERING_RELEASE_2.3.md` (C4, line 115) named it a fourth time, identically: "the same real, deferred gap C3 already named for `modiq-cli retrieve`... Not resolved, not advanced, by this capability."

**A fifth, earlier, and more directly on-point piece of evidence, checked fresh for this reconciliation:** `STORAGE_ARCHITECTURE_EVALUATION.md` §7 ("Explicit Non-Goals") — Storage's own founding Architecture Evaluation — states directly: "No schema versioning or migration mechanism." This must not be read as an Adopted principle resolving the question either way — it is a founding-scope exclusion, not a permanent prohibition. The repository has already established exactly this reading for the analogous non-goal in the same §7 list: `CAPABILITY_DEFINITION_HISTORICAL_ASSESSMENT_ANALYSIS.md` distinguishes `STORAGE_ARCHITECTURE_EVALUATION.md`'s sibling non-goal ("no querying, filtering, comparison, ranking, or aggregation across multiple Assessments") from a principled prohibition, stating that document "explicitly anticipated this exact capability and named the correct path for it, rather than prohibiting it... Sprint 13 deferred cross-report comparison for lack of a concrete forcing function at the time, not as a principled prohibition." Applied identically to the schema-versioning non-goal: `STORAGE_ARCHITECTURE_EVALUATION.md` §7 is evidence that the question was explicitly named and deliberately deferred at Storage's own founding for lack of a concrete forcing function — not evidence that it was settled, and not an Adopted principle this Process Determination could point to. If anything, it strengthens rather than overturns this section's own conclusion: the gap is not merely unaddressed by oversight since Sprint 22, it was named and deferred from Storage's very first Architecture Evaluation onward, and C12 — whose own capability depends specifically on reading historical persisted data (§7 below) — is the concrete forcing function that non-goal anticipated, in the same shape Historical Assessment Analysis itself was the forcing function for the sibling cross-report-aggregation non-goal.

**This gap has never been elevated to an ADR, a Governance Register item, or an Architectural Resolution.** A fresh search of `docs/adrs/` and `docs/architecture/` for schema-versioning or `serde(default)` language returns no result. Stated precisely, so this is not overstated: the repository has an unusually well-documented, repeatedly-named *engineering observation*, at Implementation Report and Engineering Release tier, that this problem exists and is unresolved — it does not have a *governance-tier decision* comparable to GOV-012's role in C2's own case. This is a real difference from C2's precedent, addressed directly in §9 below.

Because no ADR, Governance Register item, or Architectural Resolution establishes any answer — not "new fields must be optional," not "Storage need not preserve backward read-compatibility," not "a schema-version mechanism is required" — there is no Adopted principle for this Process Determination to point to. **Prong 1 fails.**

---

## 6. Prong 2 — Alternative Analysis

Real alternatives exist for how `modiq-storage` could evolve `PersistedFinding` to carry `ModHealthDimension`, each with materially different consequences. Their existence, not a preference among them, is what fails Prong 2. None is chosen, designed, or recommended here:

- **A `#[serde(default)]`-based optional/default field.** Requires a `Default` implementation for whatever `PersistedModHealthDimension` mirror is introduced (the six-variant Runtime enum has no established "default" variant), and would be the first use of this mechanism anywhere in `modiq-storage` — confirmed by direct `grep`, currently zero uses.
- **A schema-versioning or migration mechanism** on `PersistedAssessmentReport`, allowing old-format records to be read and upgraded on retrieval. No such mechanism exists in the codebase in any form today.
- **Some other explicit backward-compatibility mechanism** not yet named by any repository document.
- **Deliberate acceptance of historical unreadability** — an explicit decision that reports persisted before C12 ships become permanently unreadable by `ReportStore::retrieve`, not merely silently missing the new dimension.
- **Changed error-handling semantics in retrieval**, e.g. having `recurring_patterns` (or `ReportStore::retrieve` itself) skip or tolerate an unreadable record rather than propagating a hard `Err` — a change to `recurring_patterns`'s own existing, tested contract, not merely a new field.

These are not minor implementation variations of one design — they carry different consequences for data availability, for what "historical" means under C12 (§6 of the Capability Definition), and for whether `ReportStore::retrieve`'s own existing callers (not only `history_analysis`) are affected. No repository document weighs these against each other. **Prong 2 fails.**

This document does not present this list as exhaustive, and does not rank, prefer, or select among these alternatives. Their existence is the finding; resolving among them is Architecture Evaluation's own task.

---

## 7. C12-Specific Persistence Risk

Three distinct claims must be kept separate, per the Capability Definition's own §5/§6 distinction, extended here with one addition:

1. **Technical implementation detail** — not decided or reached by this document: the exact new field name, its serialization representation, and the shape of whatever `PersistedModHealthDimension` mirror type is introduced.
2. **Historical limitation, already correctly identified by `CAPABILITY_DEFINITION_C12...md` §6** — pre-existing persisted reports never contain the true, contemporaneous `ModHealthDimension` value; the persisted bytes themselves cannot recover it. This limitation is real and is not resolved by anything in this document.
3. **Architectural question, not previously identified by the Capability Definition, and the reason this Process Determination concludes as it does** — how `modiq-storage` should evolve its persisted schema at all while preserving read-compatibility with already-written reports. This is a question about the *readability* of existing records, distinct from the *completeness* of their content.

**The concrete risk this creates for C12 specifically**, verified directly against current source, not asserted in the abstract: `history_analysis::recurring_patterns` calls `store.retrieve(&key)?` inside its aggregation loop. If `mod_health_dimension` is added to `PersistedFinding` as a required field — matching the shape of every field on `PersistedFinding` today, none of which is `Option`-wrapped — under the current persistence model (no `#[serde(default)]`, no schema versioning, confirmed zero-precedent in §3/§5), **a pre-existing report encountered during aggregation would fail to deserialize, and that failure would propagate out of `recurring_patterns` as a whole, not merely omit that one report's contribution.** This is stated as the concrete risk the current persistence model creates for this specific field-shape change, not as a certainty under every possible implementation — an implementation that resolves the Prong 2 alternatives above (§6) could avoid it entirely. It is named here because `CAPABILITY_DEFINITION_C12...md` §10 already states a success criterion — "the existing `rule_reference`/`severity` aggregation continues to function exactly as it does today... for callers not concerned with `ModHealthDimension`" — that this risk, left unresolved, would place in direct tension.

This risk is also why C12 is a materially different candidate from Sprint 22 and Sprint 24 for purposes of *accepting* the same gap a further time: C12's own capability is specifically to analyze the full history of every persisted report (`CAPABILITY_DEFINITION_C12...md` §2), which makes it uniquely exposed to a gap that Sprint 22 and Sprint 24 could tolerate without directly undermining their own capability's purpose.

---

## 8. GOV-016 Boundary

Kept explicitly separate, per the Capability Definition's own §7 treatment:

- GOV-016 ("Evidentiary Standard for Establishing Governance Decisions as Repository Fact") remains **Open**, confirmed fresh against `GOVERNANCE.md` this session.
- GOV-016 concerns whether a *past* governance decision's claimed authorization (specifically, `ebc10c5`'s) is independently corroborated by repository history — a backward-looking evidentiary question.
- The persistence-migration question this document identifies (§5-§7) is forward-looking and technical: how should Storage evolve its schema going forward. The two questions do not share subject matter, evidence, or resolution mechanism.
- This document does not resolve, amend, or depend on the resolution of GOV-016. C12's own progress does not require GOV-016 to be settled.

---

## 9. Determination

**Prong 1 fails.** No ADR, Governance Register item, or Architectural Resolution establishes any principle governing how `modiq-storage` evolves an already-shipped `Persisted*` schema while preserving read-compatibility with already-persisted records. The absence of such a principle is not inferred from silence — it is confirmed by a specific, four-times-repeated engineering record (`SPRINT24_IMPLEMENTATION_PLAN.md`, `SPRINT24_IMPLEMENTATION_REPORT.md`, `ENGINEERING_RELEASE_2.2.md`, `ENGINEERING_RELEASE_2.3.md`) naming this exact gap as real and unresolved, twice by name against C12's own proposed field.

**Prong 2 fails.** Genuine alternatives with materially different consequences for data availability and for `ReportStore`'s existing contract are unweighed by any repository document (§6).

**This differs from C1's, C3's, and C4's own determinations** in the same shape C2 differed from them: a real, evidence-backed, previously-named gap — not a hypothetical one manufactured for this document — directly implicates C12's own proposed change. It differs from C2's own precedent in one respect, stated plainly rather than smoothed over: C2's open question was anchored in a governance-tier document (`SPRINT12_ARCHITECTURAL_RESOLUTION.md`, an Architectural Resolution). C12's open question is anchored in engineering-tier documents (Implementation Plans, Implementation Reports, Engineering Releases) that have repeatedly observed the gap without ever escalating it to governance tier. This document does not overstate that record as a Governance Register decision, an ADR, or an Architectural Resolution — it is not one. But the two-prong test itself does not require governance-tier standing to find a question genuinely open; it requires the absence of an Adopted principle and the presence of real alternatives, both of which are independently confirmed here regardless of the observing record's own tier.

**The next required repository artifact is an Architecture Evaluation**, not an Implementation Authorization.

---

## 10. Next Required Repository Artifact

**A C12 Architecture Evaluation, narrowly scoped to persisted Storage schema evolution and backward-read-compatibility** — specifically: whether, and how, `modiq-storage` should read already-persisted records when a `Persisted*` struct's own field shape changes, using C12's proposed `PersistedFinding.mod_health_dimension` addition as the concrete forcing instance.

This scope explicitly does **not** extend to:

- A general re-evaluation of `modiq-storage`'s architecture.
- A re-evaluation of the Storage Mirror pattern itself, which remains settled and reusable for the question of *how* to construct a mirror once its shape is decided (§3 above; unaffected by this determination).
- C5 (cross-mod dependency resolution) or C11 (`AssessmentSubject`/Report Identity).
- GOV-016, in any respect (§8).
- Governance reform, transport/UI redesign, or per-mod/cross-mod analysis of any kind.

---

## 11. Explicit Non-Determinations

This document does not:

- Choose, design, or recommend a schema-evolution mechanism (`#[serde(default)]`, schema versioning, altered retrieval error-handling, or acceptance of historical unreadability).
- Authorize implementation of any kind.
- Authorize a migration or any change to `report_store.rs`, `persisted_report.rs`, or any other source file.
- Create an Implementation Plan or Implementation Authorization.
- Modify `GOVERNANCE.md` or create any ADR or Governance Register item.
- Resolve, reopen, or take a position on GOV-016.
- Reopen the Storage Mirror pattern's own general standing, which this document treats as settled, reusable precedent for construction mechanics only.
- Assert that the deserialization-failure risk named in §7 will occur under every possible future implementation — it is named as the risk the current, unresolved persistence model creates for the field-shape change C12 proposes, not as an inevitability independent of how Architecture Evaluation resolves it.

---

## 12. Repository Impact

None. This document is analysis only. No file other than this one is created, modified, staged, or committed by this Process Determination.

---

## 13. Repository Status

**Draft. Awaiting Chief Architect review.** No Architecture Evaluation, Architectural Resolution, Implementation Authorization, or implementation has been performed or authorized by this document. GOV-016 is not resolved by this document. The next lifecycle step is a Chief-Architect-authorized C12 Architecture Evaluation, scoped exactly as §10 states.

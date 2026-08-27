# C12 Architecture Evaluation — Persisted Storage Schema Evolution

| Property | Value |
|---|---|
| **Document** | C12_ARCHITECTURE_EVALUATION_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md |
| **Project** | modIQ |
| **Capability Addressed** | C12 — Richer Historical Assessment Analysis (`CAPABILITY_DEFINITION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md`, `dd4d377`) |
| **Question Evaluated** | How should `modiq-storage` evolve an already-shipped `Persisted*` schema when a new field is required, while preserving backward readability of reports persisted under the previous schema — using C12's proposed `PersistedFinding.mod_health_dimension` addition as the concrete forcing instance? |
| **Origin** | `PROCESS_DETERMINATION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md` (`2975c93`) §10, which concluded both prongs of the reusable two-prong test fail and named this Architecture Evaluation, narrowly scoped, as the next required repository artifact. |
| **Status** | **Architecture Evaluation only. No Architectural Resolution has been performed. No schema mechanism, field shape, serde attribute, migration, or versioning scheme has been adopted, authorized, or implemented. No source file, `GOVERNANCE.md`, or ADR has been created or modified.** |

---

## 1. Evaluation Scope

**In scope:** backward readability of already-persisted reports when a `Persisted*` struct's field shape changes; behavior of `ReportStore::store`/`retrieve` and `history_analysis::recurring_patterns` under such a change; representation of a field absent from an old record; interaction with the current `serde_json` persistence mechanism; consequences for the existing `rule_reference`/`severity` aggregation; the minimum reusable architectural principle necessary to safely perform C12's own field addition.

**Explicitly out of scope**, matching `PROCESS_DETERMINATION_C12...md` §10 exactly:

- Reconstructing the true historical `ModHealthDimension` of records persisted before any schema change — this is permanently impossible from the persisted bytes alone (`CAPABILITY_DEFINITION_C12...md` §6, not reopened here).
- Choosing C12's own final aggregation/presentation behavior for old records (e.g., how `recurring_patterns` output should label a record with no dimension) — a separate, deferrable design question, not a backward-readability question.
- C5 (cross-mod dependency resolution) or C11 (`AssessmentSubject`/Report Identity).
- GOV-016, in any respect.
- General re-evaluation of `modiq-storage`'s architecture, or of the Storage Mirror pattern's own general standing (settled and unaffected — see §5).
- Any transport, UI, or presentation redesign; any governance-document amendment.

This scope is deliberately general enough to produce a principle reusable beyond `mod_health_dimension` specifically — matching `C2_ARCHITECTURE_EVALUATION_DECLARED_DEPENDENCY_INTERPRETATION.md`'s own precedent, which resolved a general Rule-composition question using C2 as the concrete forcing instance rather than deciding only C2's own Rule — while remaining bounded to what C12 actually forces, not a general Storage redesign.

---

## 2. Repository Evidence

Re-verified directly against current source and documents this session, independent of the preceding Process Determination's own citations:

- **`crates/modiq-storage/src/storage/persisted_report.rs`** — `PersistedFinding` (lines 113-125): exactly `severity, title, summary, evidence_indices, rule_reference`; no field is `Option`-wrapped except implicitly via `Vec` (empty is the "nothing" state for `evidence_indices`, per its own doc comment). `from_finding` (line 128) is a total, branchless conversion; it does not call `Finding::mod_health_dimension()`. Four existing `Persisted*` enum mirrors confirmed present: `PersistedAssessmentStatus`, `PersistedEvidenceCategory`, `PersistedFindingSeverity`, `PersistedRecommendationStepKind` — each `Serialize`/`Deserialize`, each populated via a `From<RuntimeEnum>` conversion, none `Option`-wrapped, none carrying a `Default` impl.
- **`crates/modiq-storage/src/storage/report_store.rs`** — `store` (line 32): `PersistedAssessmentReport::from_report` → `serde_json::to_vec_pretty` → `fs::write`, one file per report keyed by a freshly generated `ReportKey`. `retrieve` (line 45): `fs::read` → **one single `serde_json::from_slice(&bytes)` call for the entire `PersistedAssessmentReport`** — struct, all nested `Vec<PersistedFinding>`/`Vec<PersistedEvidence>`/`Vec<PersistedRecommendation>` together. A missing required field anywhere in that tree fails the whole call; there is no partial or per-field deserialization. `list_keys` (line 60) touches only file names, never content — always succeeds regardless of any report's internal schema.
- **`crates/modiq-storage/src/storage/history_analysis.rs`** — `recurring_patterns` (line 40): `for key in store.list_keys()? { let report = store.retrieve(&key)?; ... }`. The bare `?` on `retrieve` propagates the first failure as the whole function's own `Err`, discarding every pattern already accumulated from earlier, successfully-read reports in the same call. Confirmed by direct reading of the loop body, not inferred from the signature alone.
- **`crates/modiq-runtime/src/assessment/finding.rs`** — `Finding.mod_health_dimension: ModHealthDimension` is a required, non-`Option` field, populated only via `Finding::new`'s required parameter, never mutated post-construction (ADR-0007).
- **`crates/modiq-runtime/src/assessment/mod_health_dimension.rs`** — `ModHealthDimension` derives only `Debug, Clone, Copy, PartialEq, Eq`. **No `Default` impl exists, and none of its six variants (`Compatibility, Stability, Maintainability, Performance, Structure, EngineeringQuality`) reads as a semantically neutral "unknown."**
- A fresh, repository-wide `grep -rn "serde(default)" crates/` and `grep -rn "schema_version\|SchemaVersion" crates/` both return **zero matches**, confirmed again this session — no compatibility or versioning mechanism exists anywhere in this codebase today.
- **`STORAGE_ARCHITECTURE_EVALUATION.md`** — re-read in full; see §4 below.
- **`STORAGE_IMPLEMENTATION_AUTHORIZATION.md`** — line 39 independently lists "Schema versioning or migration mechanisms" among its own out-of-scope items, the same non-goal restated, never elevated to an adopted decision.
- **`SPRINT24_IMPLEMENTATION_PLAN.md`** (§8, Risk 3), **`SPRINT24_IMPLEMENTATION_REPORT.md`** (line 167), **`ENGINEERING_RELEASE_1.9.md`** (line 105), **`ENGINEERING_RELEASE_2.2.md`** (line 104), **`ENGINEERING_RELEASE_2.3.md`** (line 115) — re-read in full; see §5 below.
- **Git history** (`git show`, `git log -S`) on `persisted_report.rs`: `e1406ec` (founding), `969e595` (Sprint 22), `84ef4e3` (Sprint 24) — see §5 below.
- **Empirical `serde` behavior**, tested directly against this repository's own pinned dependency versions (`Cargo.lock`: `serde 1.0.229`, `serde_json 1.0.151`), not assumed from general knowledge: a struct field of type `Option<T>`, with no `#[serde(default)]` attribute, deserializes successfully to `None` when the corresponding JSON key is entirely absent — confirmed by a standalone test build against these exact versions. A sibling field of the same `Option<T>` type *with* `#[serde(default)]` produces an identical result for the same input. A non-`Option` field with no `#[serde(default)]` correctly fails with a "missing field" error under the same input, confirming Alternative A's own failure mode (§6) is real. This directly informs §6's B1/B2 analysis below.

---

## 3. The Architectural Question

**How should `modiq-storage` evolve an already-shipped `Persisted*` schema when a new field is required, while preserving backward readability of reports persisted under the previous schema — using C12's proposed `PersistedFinding.mod_health_dimension` addition as the concrete forcing instance?**

This question is deliberately narrower than it may first appear, and this Evaluation treats the following as **distinct, not interchangeable**, questions:

1. **Backward readability** — can `ReportStore::retrieve` return `Ok` for a report persisted under the previous schema, after the new field is added? *(This document's own question.)*
2. **Historical-value recovery** — can the true, contemporaneous `ModHealthDimension` of a pre-existing Finding be recovered from its persisted bytes? **No — permanently, regardless of any answer to Question 1.** Not reopened here; already correctly settled by `CAPABILITY_DEFINITION_C12...md` §6.
3. **Presentation/aggregation choice** — how should C12's own future aggregation output represent a record for which the dimension is unavailable? A design choice deferrable to Implementation Planning, contingent on, but not decided by, this Evaluation's answer to Question 1.
4. **A fully general Storage schema-evolution framework** — a broader ambition than what C12 actually forces; addressed only to the extent Question 1's answer requires it (§7).

---

## 4. Founding Storage Architecture, Precisely Characterized

`STORAGE_ARCHITECTURE_EVALUATION.md`, re-read in full, contains exactly **four numbered Decisions** (§§2-5: The Smallest Persistable Domain Object; Subsystem Ownership; Lifecycle Boundary; Impact on `AssessmentService`'s Public Entry Points), each carried into that document's own Architectural Resolution "exactly as recommended, with no modification" (line 133).

**"No schema versioning or migration mechanism" appears in §7, "Explicit Non-Goals" — not among the four Decisions.** Its precise status:

- **Not an adopted decision** — it is not one of the four Decisions the Resolution carried forward.
- **Not a permanent prohibition** — no text anywhere states such a mechanism may never be introduced.
- **An explicit founding-scope non-goal** — a statement that the question was not decided by that document, not a statement of what the answer is.
- **A deferred question**, per the repository's own already-established reading of this exact §7 list: `CAPABILITY_DEFINITION_HISTORICAL_ASSESSMENT_ANALYSIS.md` characterizes §7's sibling non-goal ("no querying, filtering, comparison, ranking, or aggregation across multiple Assessments") as deferred "for lack of a concrete forcing function at the time, not as a principled prohibition," naming Historical Assessment Analysis itself as "its own, later, separately-evaluated capability" that correctly took the deferred item up. Applied identically here: schema versioning was deferred for the same reason, at the same founding moment, in the same list — and **C12 is now the concrete forcing function for this sibling non-goal**, in the same shape Historical Assessment Analysis was for its own.
- **Independently restated, never elevated**, in `STORAGE_IMPLEMENTATION_AUTHORIZATION.md` (line 39) — the same non-goal, still not a decision.
- **Now a live, unresolved architectural concern**, per §5's engineering record below — not because this Evaluation asserts it, but because the repository's own subsequent commits and their engineering records already say so, repeatedly.

---

## 5. Historical Precedent, Without Overclaiming It

**Sprint 22 (`969e595`):** `PersistedFinding.description: String` was **replaced** by `title: String, summary: String` — confirmed by direct diff (`git show 969e595`). No compatibility mechanism of any kind accompanied this change. No migration was performed. Old-record readability was never tested against this change — it was simply not exercised, because no cross-report historical-reading capability existed yet at that time (`STORAGE_ARCHITECTURE_EVALUATION.md` §7's own non-goal, still in force at that point). `SPRINT24_IMPLEMENTATION_PLAN.md` (§8, Risk 3), written over a month later, is the first document to name the consequence explicitly: "Sprint 22 already created the identical exposure when `PersistedFinding`'s shape changed... with no migration mechanism."

**Sprint 24 (`84ef4e3`):** `PersistedRecommendation` gained a new **required** field, `repair_steps: Vec<PersistedRecommendationStep>`, and a brand-new enum mirror, `PersistedRecommendationStepKind` — confirmed by direct diff. Again, no `#[serde(default)]`, no `Option` wrapper, no migration. `SPRINT24_IMPLEMENTATION_PLAN.md`'s own Risk 3, drafted *before* this shipped, named the exact consequence in advance: "Any already-persisted report predating this Sprint will fail to deserialize on retrieval afterward... recorded as an accepted, pre-existing category of risk." `SPRINT24_IMPLEMENTATION_REPORT.md` (line 167), written after, escalated the framing: "now has two independent instances... worth a dedicated evaluation before a third field-shape change compounds it further."

**`ENGINEERING_RELEASE_2.2.md`** (C3, line 104) and **`ENGINEERING_RELEASE_2.3.md`** (C4, line 115) each independently name, by name, "Extending `modiq-storage`'s own `PersistedFinding`/`PersistedEvidence` schema to carry `mod_health_dimension`" as carrying this same "real risk... unresolved."

**What this precedent is, and is not:** it is real, specific, and repeated evidence that this repository has twice shipped a breaking `Persisted*` field-shape change with no compatibility mechanism, and that its own engineering record named this as accepted risk each time — **not** as a validated pattern, and **not** as evidence that doing so a third time is architecturally sound. Repeated implementation practice, however consistent, does not by itself constitute an adopted principle (§10). C12 is materially different from both prior instances in one respect this Evaluation treats as decisive: C12's own capability is specifically to read the full historical record, making it the first candidate for which repeating this pattern would directly undermine the capability's own purpose, rather than merely carrying a disclosed, tolerable risk forward.

---

## 6. Architectural Alternatives

None of the following is selected, designed, or authorized by this Evaluation.

### A — Required field, no compatibility mechanism

Add `mod_health_dimension: PersistedModHealthDimension` to `PersistedFinding`, non-`Option`, matching every existing field's shape exactly — the same shape both prior precedent instances actually took (§5).

- **Exact consequence, traced, not projected:** any report persisted before this change fails `serde_json::from_slice` in full on retrieval (§2) → `ReportStore::retrieve` returns `Err(ReportStoreError::Deserialize(_))` for that entire report → `history_analysis::recurring_patterns` aborts on the first such report, returning `Err` for the whole call, discarding all previously-accumulated results.
- **Impact on callers not concerned with `ModHealthDimension`:** direct and total — the existing `rule_reference`/`severity` aggregation, which never reads the new field at all, still fails, because the failure occurs at whole-report deserialization, before any field-level logic runs.
- **Simplicity:** the simplest alternative by a wide margin, and the one every prior precedent actually used.
- **Technical debt:** converts a disclosed, tolerated risk into a demonstrated regression, the first time any capability's own success criterion is directly measurable against it.
- **This is not automatically invalid because it is simple** — it is specifically invalid because it is independently, concretely shown (§9) to violate C12's own stated success criterion, a consequence prior instances never had to reckon with because no capability yet depended on reading old records.

### B — Optional/default field semantics

Three materially different sub-mechanisms:

- **B1 — `Option<PersistedModHealthDimension>` alone, no `#[serde(default)]`.** **Solves the problem on its own.** Confirmed empirically (§2): a derived `Deserialize` impl treats a field of type `Option<T>` as optional at the missing-key level without requiring `#[serde(default)]` — this is `serde_derive`'s own standard, long-standing behavior for `Option`-typed fields, not an edge case. An old record missing the key deserializes with `dim: None`; a record with the key explicitly `null` deserializes identically; `retrieve()` returns `Ok` for both. **B1 alone is a viable, sufficient mechanism, and requires no new attribute anywhere in `modiq-storage`.**
- **B2 — `Option<PersistedModHealthDimension>` + `#[serde(default)]`.** Confirmed empirically (§2) to be **behaviorally identical to B1** for every scenario tested — missing key, explicit `null`, and round-trip serialization all produce the same result with or without the attribute. `#[serde(default)]` is not required to make an `Option<T>` field tolerate a missing key; it is redundant decoration in this specific case, not a distinct compatibility mechanism. It remains available as a purely stylistic, optional annotation — some authors add it to `Option` fields anyway to signal missing-field tolerance explicitly to a future reader who may not know `serde`'s own special-casing of `Option` — but that is a documentation preference, not an architectural requirement, and no existing convention in this repository (no `Persisted*` field uses `#[serde(default)]` today, confirmed §2) either favors or disfavors adding it.
- **B3 — Non-`Option` field + `#[serde(default)]` with a `Default` impl on `PersistedModHealthDimension`.** Requires fabricating a "default" variant among six that have no natural default (§2) — e.g., silently defaulting old records to `EngineeringQuality` — which would misrepresent them as having an actual, assigned dimension they never had. This is a materially worse outcome than B2's honest `None`, since it manufactures a specific, wrong-looking answer rather than an explicit absence.
- **Established convention check:** the repository has no existing `Option`-typed field on any `Persisted*` enum-mirror type to compare against directly, but `PersistedFinding.evidence_indices` already uses an empty collection (not an `Option`) as its own "nothing" state — meaning `Option<T>` on `PersistedFinding` would be a new representational idiom for this specific struct, though not a new idiom for the language or for `serde` generally.
- **What B does not decide by itself:** what a `None` value should mean when C12's own future aggregation logic encounters it (Question 3, §3) — a separate, deferrable design choice.

### C — Schema versioning

A `schema_version` (or similar) field on `PersistedAssessmentReport`, with `retrieve` branching per version to apply appropriate deserialization/upgrade logic.

- **Where it would live:** no such field exists anywhere today (§2); this is unambiguously new capability, not an extension of anything present.
- **A specific complication unique to retrofitting it now:** neither Sprint 22's nor Sprint 24's own breaking change carried any version marker, so a versioning scheme introduced today cannot cleanly distinguish a pre-Sprint-22 file from a pre-Sprint-24 file from a post-Sprint-24 file without separate, ad hoc detection logic layered on top of the versioning mechanism itself — the mechanism would need to handle vintages it was never designed to track.
- **Does not, by itself, decide what a missing field should become** — still needs a B-like answer nested inside whatever upgrade logic a given version transition applies. Versioning is a *dispatch* mechanism, not a substitute for a compatibility semantic.
- **Complexity and reusability:** the heaviest alternative; the most reusable *if* Storage anticipates many future field-shape changes, but not shown by any evidence gathered here to be *necessary* to solve C12's own, narrower question.

### D — Migration/backfill

Rewriting old persisted JSON files into the new shape.

- **Migration of representation vs. reconstruction of historical truth, kept separate:** a migration can populate the new field in old files, but cannot supply the *true* historical `ModHealthDimension` — that value was never captured (§3, Question 2).
- **The one available proxy, evaluated only as a technical possibility, not selected:** `rule_reference` → today's Rule → today's fixed dimension assignment. Confirmed fresh, five Rules, five fixed assignments (`EvidencePresenceRule`→`EngineeringQuality`, `StructuralDuplicationRule`→`Structure`, `RuntimeLoadFailureRule`→`Stability`, `VersionCompatibilityRule`→`Compatibility`, `DeclaredDependencyDuplicationRule`→`Structure`). This is an *inference* about what a Rule would assign *today*, not a recovery of what it assigned *then* — it silently misattributes history if a Rule's own assignment ever changes, is renamed, or is removed, exactly as `CAPABILITY_DEFINITION_C12...md` §6 already states, and this Evaluation does not revisit that conclusion.
- **Operational risk:** a real migration step (rewrite-in-place) carries its own partial-failure risk — an interrupted migration could leave a store in a mixed state — that must itself be managed if this path is ever chosen; this Evaluation does not design that management.
- **Benefit:** the only alternative that could improve historical *completeness* (via inference, not truth) rather than merely preserving readability — a materially different kind of benefit than A/B/C/E/F offer.

### E — Explicit accepted historical-data limitation

A documented decision that old records remain readable but permanently lack `ModHealthDimension`.

- **This alternative is only coherent when paired with B** — "accepted limitation" cannot mean "old records become unreadable" without collapsing into Alternative A's own consequence, a different and, per §9, disqualifying outcome. **"Field unavailable" (readable, absent value) and "record unreadable" (deserialization failure) are not interchangeable, and this Evaluation does not treat them as such anywhere in this document.**
- **Cost:** primarily a documentation and presentation-honesty obligation — must not present "no dimension recorded" as indistinguishable from "recorded and happens to be absent" in a way that misleads a future reader of aggregated output. Technically inexpensive once B is the underlying mechanism.

### F — Retrieval/error-handling redesign

`ReportStore::retrieve` or `recurring_patterns` could tolerate a single incompatible report — skip it, surface it separately, and return a partial result — rather than propagating a hard `Err` for the whole call.

- **Consequence for `recurring_patterns`'s own contract:** changes an existing, tested guarantee (today: complete-and-correct or a hard error, never silent partial) — would need a new return shape and new tests to avoid conflating "old-schema record" with "genuinely corrupted file," which are different failure causes this alternative alone cannot distinguish without additional logic.
- **Relationship to the other alternatives:** orthogonal, not a substitute — F could be adopted independently, as defense-in-depth for future, different incompatibilities, whether or not B/C/D is also chosen for this specific field.

---

## 7. Evaluation Criteria

| Criterion | A | B (B1, ≡ B2 — see §6) | C | D | E (w/ B) | F |
|---|---|---|---|---|---|---|
| 1. Backward readability | Fails | Satisfies | Satisfies (if correct) | Satisfies (post-migration) | Satisfies | Satisfies |
| 2. Preserves existing `rule_reference`/`severity` aggregation | Fails | Satisfies | Satisfies | Satisfies | Satisfies | Satisfies (differently) |
| 3. Correct representation of historical absence | N/A (unreadable) | Honest (`None`) | Depends on design | Honest, if D not silently presented as truth | Honest, by construction | N/A on its own |
| 4. Historical-truth preservation | N/A | No (not attempted) | No (not attempted) | Partial, unreliable (inference only) | No (not attempted) | N/A |
| 5. Forward compatibility (future fields) | Poor (repeats the gap) | Reusable idiom | Most reusable | N/A (one-time) | N/A | Reusable, orthogonal |
| 6. Scope/reusability proportionate to what C12 forces | Under-scoped (ignores the question) | Proportionate | Over-scoped for this instance | Proportionate for the one field, if used | Proportionate | Optional extra |
| 7. Complexity | Lowest | **Lowest of the alternatives that satisfy Criterion 1** — B1 requires no new `serde` attribute at all | Highest | Moderate-high (operational) | Lowest (paired with B) | Moderate |
| 8. Operational risk | None (but see failure below) | Low | Moderate | Real (rewrite-in-place) | None | Moderate |
| 9. Failure transparency | Opaque total failure | Transparent (`None`) | Depends on design | Depends on design | Transparent | Explicit, if designed for it |
| 10. Consistency with existing repository conventions | Matches prior *practice* (§5) | New idiom, not precedented for this struct | No precedent at all | No precedent at all | Pairs with B | No precedent at all |
| 11. Avoids misleading historical data | N/A | Yes | Depends | **No, if B3's fabricated default is used; yes if D's inference is clearly labeled as inference** | Yes | N/A |
| 12. Solves C12 without creating a larger unresolved problem | No — defers the problem while breaking C12 | Yes | Possibly over-solves | Adds a new operational problem (migration risk) | Yes, paired with B | Adds a new contract-change problem |

**B1 and B2 are behaviorally identical for every criterion above** (§6) — `#[serde(default)]` changes no observable outcome for an already-`Option`-typed field's missing-key handling. The table's "B" column reflects both; B1 is used as the reference shape because it achieves the identical result with strictly less new syntax.

**Where criteria conflict:** Criterion 10 (matches existing practice) favors A; every other criterion disfavors it. Criterion 5 (forward reusability) favors C most; Criterion 7/8 (complexity/risk) disfavor it for a need not yet shown to exist beyond this one field. Criterion 4 (historical truth) is the only criterion D meaningfully helps, and only partially, at real operational and reliability cost.

---

## 8. Critical Distinctions

Four outcomes, kept explicitly separate throughout this document, not collapsed:

- **(A) Old record is unreadable** — the consequence of Alternative A alone.
- **(B) Old record is readable but lacks `ModHealthDimension`** — the consequence of Alternative B (or E, paired with B).
- **(C) Old record is readable and `ModHealthDimension` is inferred** — the consequence of Alternative D, and only ever an inference, never a recovery of truth.
- **(D) Old record is readable and `ModHealthDimension` is explicitly represented as unavailable** — a presentation choice built on top of outcome (B), belonging to Question 3 (§3), not decided here.

Also kept separate: **technical feasibility** (all of B/C/D/E/F are feasible with current tooling), **historical accuracy** (only D, imperfectly, touches this — none of A/B/C/E/F improve it), **architectural desirability** (assessed in §7), and **repository consistency** (assessed in §7, criterion 10 — where the *most consistent with practice* alternative is also the one that fails the capability's own requirement).

---

## 9. C12 Success Criterion Analysis

C12's own stated criterion (`CAPABILITY_DEFINITION_C12...md` §10): *"the existing `rule_reference`/`severity` aggregation continues to function exactly as it does today... for callers not concerned with `ModHealthDimension`."*

- **Violates directly:** A. Traced precisely in §6 — the failure occurs before any `ModHealthDimension`-specific logic would ever run, so "not concerned with `ModHealthDimension`" offers no protection.
- **Satisfies directly:** B (B1, and identically B2), E (paired with B). `retrieve` succeeds for old records; `rule_reference`/`severity` are read exactly as before, unaffected by the new field's presence or value.
- **Satisfies conditionally:** C (only if its upgrade logic itself succeeds for every old vintage, including ones with no version marker at all — a materially harder condition to guarantee than B's); D (only after migration completes without partial failure, and only for the readability question — it still does not, and cannot, satisfy any criterion about historical *truth*, only completeness-by-inference).
- **Satisfies readability but requires a separate decision for aggregation semantics:** B, C, and D all leave Question 3 (§3) — how old records present in `ModHealthDimension`-aware output — open, correctly, since that is not a backward-readability question.

---

## 10. Precedent Strength

- **Tier 1 — Adopted architectural/governance decisions:** none exist on this question (§4, §2's fresh `docs/adrs/`/`GOVERNANCE.md` re-check).
- **Tier 2 — Architecture Evaluations/Resolutions:** `STORAGE_ARCHITECTURE_EVALUATION.md` names the question as an Explicit Non-Goal (§4) — real, but explicitly not a decision.
- **Tier 3 — Engineering Plans/Reports/Releases:** `SPRINT24_IMPLEMENTATION_PLAN.md`, `SPRINT24_IMPLEMENTATION_REPORT.md`, `ENGINEERING_RELEASE_1.9.md/2.2.md/2.3.md` — real, repeated, specific, and increasingly emphatic ("worth a dedicated evaluation before a third field-shape change compounds it further"), but this tier records observation, not authority.
- **Tier 4 — Repeated implementation practice:** Sprint 22 and Sprint 24 both shipped Alternative A's own shape with no mechanism (§5) — real practice, but, per this repository's own standing discipline and per the explicit Tier 3 record naming it as accepted risk rather than validated design, **this is not manufactured into an adopted principle by this Evaluation.**

**Nothing above binds this Evaluation's own choice among Alternatives A-F.** What Tiers 3-4 jointly establish, with real weight, is that the question is genuine, has been named four times independently by name against this exact field, and that C12 is the first instance where continuing Tier 4's own practice (Alternative A) would concretely defeat the requesting capability's own purpose.

---

## 11. Recommended Architectural Direction

**This is a recommendation only. It is not an adopted architectural decision. It is submitted for consideration by a subsequent C12 Architectural Resolution, which may accept, reject, or revise it.**

Repository evidence most directly supports **Alternative B, in its minimal B1 shape (`Option<PersistedModHealthDimension>`, with no `#[serde(default)]` — the attribute is confirmed redundant for this scenario, §2, §6), paired with Alternative E's explicit framing of what an absent value means**, as the direction best satisfying the evaluation criteria (§7) and C12's own success criterion (§9), on the following stated grounds:

- It is the only alternative that satisfies C12's success criterion **unconditionally** (§9) — B/E do not depend on a migration succeeding or a versioning scheme correctly handling vintages it was never designed for.
- It avoids fabricating historical data (§6, B3's rejected variant; §8, outcome (C)'s inherent unreliability) — `None` states plainly what is true: the value was never captured.
- Its complexity and operational risk are the lowest among **all** alternatives evaluated, not merely the lowest among those that satisfy the success criterion — B1 requires no new `serde` attribute, no `Default` impl, no dispatch mechanism (C), and no operational migration procedure (D). This is a stronger claim than an earlier draft of this Evaluation made, corrected after empirical verification (§2) showed `#[serde(default)]` adds no behavior beyond what `Option<T>` alone already provides for a missing-key scenario.
- It does not foreclose Alternative C or D later — a schema-versioning mechanism or a migration effort could still be pursued in the future, independently, if a concrete forcing function for either ever materializes, exactly as `PROJECT_HANDOFF_v1.1.md` §6 Principle 2's own standing discipline would require before either is adopted.

**This is not a recommendation made because B is easiest to implement.** A is easier to implement than B and is explicitly rejected (§6, §9) precisely because its ease does not extend to correctness against C12's own stated requirement. B1 is recommended because it is the least complex alternative that is also *correct* against that requirement — a different claim.

**On `#[serde(default)]` specifically:** this Evaluation actively looked for an independent architectural reason to prefer B2's extra attribute over B1's plainer shape — greater future-proofing, consistency with an existing convention, defense against some other deserialization path — and found none. No existing `Persisted*` field uses `#[serde(default)]` today, so neither shape is more "conventional" than the other; no evidence suggests `serde`'s own `Option`-specific missing-key handling is fragile or likely to change. **Including `#[serde(default)]` remains available as a purely stylistic, optional annotation for explicitness, at whichever later lifecycle stage decides field mechanics — but this Evaluation does not recommend it as necessary, and does not want its earlier inclusion here to be mistaken for an architectural reason that does not exist.**

**Where the evidence does not support a single, fully resolved answer:** the precise shape of the new field name, whether `#[serde(default)]` is added anyway for stylistic reasons, and whether Alternative F's retrieval-tolerance improvement should be pursued as a separate, later resilience enhancement are not settled by this Evaluation — they are Resolution- or Implementation-level decisions, consistent with every prior Architecture Evaluation in this repository's own lifecycle (e.g., `C2_ARCHITECTURE_EVALUATION...md`'s own explicit deferral of "any Rust type, matching logic, prefix string").

---

## 12. Consequences of the Recommended Direction

- **Benefits:** satisfies C12's success criterion without qualification; introduces no fabricated data; the lowest-complexity path of every alternative evaluated, not merely the lowest among those that work — no new attribute, no `Default` impl, no dispatch mechanism; establishes a reusable idiom (plain `Option<T>` for a field a prior schema version lacked) that a future field addition could reuse without re-litigating this Evaluation from scratch.
- **Costs:** introduces `Option`-typed representation on `PersistedFinding` where no prior field used that idiom (though `evidence_indices`' empty-collection convention is a related, if not identical, precedent for representing absence). No new `serde` attribute is introduced by the recommended B1 shape itself.
- **Risks:** none operationally significant — no rewrite-in-place, no version-dispatch logic to get wrong for untracked historical vintages.
- **Unresolved implementation questions**, deliberately left to later artifacts: exact field/type naming; whether `PersistedModHealthDimension` (the enum mirror itself) needs any change beyond existing precedent (§2 — none of the four existing enum mirrors carry `Default`, and B1 does not require one either); whether `#[serde(default)]` should be added anyway for stylistic explicitness despite being behaviorally redundant (§11); how `history_analysis`'s own future dimension-aggregation logic should treat `None` (Question 3, §3).
- **Compatibility consequences:** old records remain fully readable by every existing caller, including ones with no interest in `ModHealthDimension`; new records carry `Some(...)`.
- **Historical-data consequences:** no improvement to historical completeness (that would require D, not recommended here); no degradation of historical accuracy either — `None` never claims to be a value C12 does not actually have.
- **Future reuse implications:** the plain `Option<T>` idiom, once exercised once, becomes a lower-friction path for the next capability that needs to add a field to an already-shipped `Persisted*` struct — a modest, evidence-grounded step toward closing the general gap named four times in the engineering record (§5, §10), without this Evaluation itself designing a general framework (§1, §3, Question 4) beyond what this one instance actually requires.

---

## 13. Architectural Resolution Boundary

**This Evaluation recommends:** Alternative B (B1 shape — plain `Option<PersistedModHealthDimension>`, no `#[serde(default)]` required), paired with Alternative E's framing, as the direction a subsequent Architectural Resolution should adopt, subject to that Resolution's own independent judgment.

**This Evaluation does not decide:**
- The final field name, type shape, or whether `#[serde(default)]` is added anyway for stylistic explicitness despite being behaviorally redundant here (§11).
- Whether `PersistedModHealthDimension` (the enum mirror) requires any structural change beyond the existing four-instance precedent.
- How C12's own future aggregation output should present a `None` value (Question 3, §3) — an Implementation Planning question, contingent on this Evaluation's answer but not settled by it.
- Whether Alternative C (schema versioning) or D (migration) should ever be pursued independently, for reasons unrelated to C12.
- Whether Alternative F's retrieval-tolerance improvement should be pursued as a separate resilience enhancement.

**The subsequent Architectural Resolution must formally decide:** whether to adopt this recommendation, a modified version of it, or a different alternative entirely; whether any of the above deferred questions require their own dedicated treatment before Implementation Authorization; and whether any repository document (e.g., `GOVERNANCE.md`'s Storage Crate Boundary Rule, or `RuleEngine.md`-style Architectural Constraint precedent) should be amended to record the adopted principle, mirroring how `C2_ARCHITECTURAL_RESOLUTION...md` recorded its own Decision 2 as a named, reusable constraint.

No such Resolution has been performed. No amendment has been made. This Evaluation's responsibility ends here, awaiting Chief Architect review.

---

## Evaluation Boundaries

**Questions this Evaluation answered:** whether an Adopted principle already settles backward-schema-evolution readability (§4, §10 — no); what the founding `STORAGE_ARCHITECTURE_EVALUATION.md` non-goal actually means (§4 — deferred, not prohibited, not decided); what alternatives exist and their comparative consequences (§6, §7); which outcomes are conflated risks to avoid (§8); whether each alternative satisfies C12's own success criterion (§9); what a defensible recommendation is, and on what grounds (§11, §12).

**Questions intentionally not addressed:** the exact field/type/serde-mechanics design; C12's own aggregation-presentation behavior for absent values; whether Alternative C or D should ever be pursued for reasons independent of C12; GOV-016's own resolution; any general Storage architecture redesign beyond this one instance.

**Repository documents relied upon as fixed, unreopened precedent:** the four Decisions of `STORAGE_ARCHITECTURE_EVALUATION.md`'s own Architectural Resolution; `CAPABILITY_DEFINITION_C12...md`'s own §6 conclusion on historical-value irrecoverability, not reopened; `PROCESS_DETERMINATION_C12...md`'s own scope framing (§10), matched exactly.

No Architectural Resolution has been performed. This evaluation's responsibility ends here, awaiting Chief Architect review.

# C12 Architectural Resolution — Backward-Readable Persisted Storage Evolution

| Property | Value |
|---|---|
| **Document** | ARCHITECTURAL_RESOLUTION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md |
| **Project** | modIQ |
| **Question Resolved** | How should `modiq-storage` evolve an already-shipped `Persisted*` schema when a new field is required, while preserving backward readability of reports persisted under the previous schema — using C12's proposed `PersistedFinding.mod_health_dimension` addition as the concrete forcing instance? |
| **Origin** | `C12_ARCHITECTURE_EVALUATION_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md` (committed `9af4d2b`), whose own Repository Validation Review, B1/B2 Reconciliation, and Post-Reconciliation Repository Validation Review are all complete. That Evaluation is treated as fixed evidence; this Resolution dispositions its recommendation, it does not reopen it. |
| **Adopted Precedent** | `STORAGE_ARCHITECTURE_EVALUATION.md`'s own four Decisions (unreopened); `C2_ARCHITECTURAL_RESOLUTION_DECLARED_DEPENDENCY_INTERPRETATION.md`'s demonstrated shape for calibrating a reusable-but-narrowed architectural constraint from a concrete forcing instance — used here as precedent for *form*, not for *substance*. |
| **Status** | **Architectural Resolution. No source file, `GOVERNANCE.md`, or ADR has been created or modified. No Implementation Authorization, Implementation Plan, migration, backfill, or historical reconstruction has been performed or authorized.** |

---

## §1 — Method

This Resolution dispositions the Architecture Evaluation's own recommendation and the reusable-principle question the Process Determination's scope framing (§10 of that document) left open, using the same disposition vocabulary this repository's own Architectural Resolutions already use (`C2_ARCHITECTURAL_RESOLUTION_...md` §1): **Adopted / Adopted Architectural Constraint / Deferred / Rejected**. No new disposition category is introduced. Evidence, reasoning, and decision are kept in separate sections throughout.

---

## §2 — Inputs / Governing Evidence

Re-confirmed directly this session, not carried forward from the Evaluation's own summary alone:

- **`CAPABILITY_DEFINITION_C12...md`** (`dd4d377`) — §5/§6's fixed conclusion that `ModHealthDimension` was never persisted and cannot be recovered from persisted bytes; §11's Explicit Exclusion of "retrospective reconstruction... no backfill or migration mechanism." Not reopened here.
- **`PROCESS_DETERMINATION_C12...md`** (`2975c93`) — §10's scope framing: "whether, and how, `modiq-storage` should read already-persisted records when a `Persisted*` struct's own field shape changes, using C12's proposed... addition as the concrete forcing instance." This Resolution answers exactly that framing, matched, not broadened.
- **`C12_ARCHITECTURE_EVALUATION_...md`** (`9af4d2b`) — §6's B1/B2/B3 analysis; §7's criteria comparison; §9's success-criterion analysis; §11's recommendation of B1, explicitly as a recommendation only.
- **`STORAGE_ARCHITECTURE_EVALUATION.md`**, re-read in full again this session — its four numbered Decisions (§§2–5) remain the only adopted architecture from Storage's own founding; §7's "No schema versioning or migration mechanism" remains an Explicit Non-Goal, confirmed again not among the four Decisions and not stated anywhere as a prohibition.
- **`STORAGE_IMPLEMENTATION_AUTHORIZATION.md`**, re-read — line 39 restates the same non-goal, never elevated.
- **`crates/modiq-storage/src/storage/persisted_report.rs`**, re-inspected this session — `PersistedFinding`'s five current fields remain exactly `severity, title, summary, evidence_indices, rule_reference`, none `Option`-wrapped; `PersistedEvidence.location: Option<String>` and `PersistedRecommendation.repair_recipe_reference: Option<String>` are confirmed, on the Runtime side (`evidence.rs`, `recommendation.rs`), to mirror an *already-`Option`* Runtime field representing a live, per-instance domain choice — a different kind of optionality than what C12 proposes (§7 below).
- **`crates/modiq-runtime/src/assessment/finding.rs`**, re-inspected — `Finding.mod_health_dimension: ModHealthDimension` remains required, non-`Option`, populated only at construction. This Resolution changes nothing about this Runtime type.
- **`C2_ARCHITECTURE_EVALUATION_DECLARED_DEPENDENCY_INTERPRETATION.md`** and **`C2_ARCHITECTURAL_RESOLUTION_DECLARED_DEPENDENCY_INTERPRETATION.md`**, re-read in full — the repository's one prior instance of resolving a genuinely open architectural question by adopting a *generally-worded but narrowed* constraint from a concrete forcing capability, calibrated to what the evidence actually supported rather than to the broadest alternative on the table (Decision 2, narrowed from Alternative B's own blanket proposal). Relied on here for the *shape* of generalization only — C2's own substance (Rule Conclusion Non-Contradiction) has no bearing on Storage schema evolution and is not extended by this document.
- **`PROCESS_DETERMINATION_C1_RECOMMENDATION_STEP_PRESENTATION.md`, `PROCESS_DETERMINATION_C3_CLI_FIELD_PARITY.md`, `PROCESS_DETERMINATION_C4_APPS_SANDBOX_FIELD_PARITY.md`**, re-confirmed — each independently states the reusable two-prong test is "not a newly invented one" but also not a canonical `PROJECT_HANDOFF_v1.1.md` lifecycle stage; the same discipline (reuse the test, re-derive the outcome fresh each time) is the model this Resolution applies to its own §6 principle: stated generally, but requiring each future capability's own confirmation that its facts genuinely match before relying on it.
- **`969e595` (Sprint 22) and `84ef4e3` (Sprint 24)**, and the associated engineering record (`SPRINT24_IMPLEMENTATION_PLAN.md` §8 Risk 3, `SPRINT24_IMPLEMENTATION_REPORT.md` line 167, `ENGINEERING_RELEASE_1.9.md` line 105, `ENGINEERING_RELEASE_2.2.md` line 104, `ENGINEERING_RELEASE_2.3.md` line 115) — reconfirmed: two prior `Persisted*` field-shape changes shipped with no compatibility mechanism, each named as accepted risk in engineering-tier documents, never elevated to any ADR, Governance Register item, or Architectural Resolution. This record demonstrates a real, repeatedly-named, unresolved architectural gap — it does **not** establish an adopted policy that backward compatibility is unnecessary, and it does **not** establish that breaking historical readability is acceptable architecture. It establishes the historical gap this Resolution now closes.
- **Empirical `serde` behavior**, re-confirmed a further time this session against this repository's pinned versions (`serde 1.0.229`, `serde_json 1.0.151`): a field of type `Option<T>`, with no `#[serde(default)]`, deserializes to `None` for a missing JSON key or an explicit `null`; `#[serde(default)]` added to the same `Option<T>` field produces an identical result. No repository-specific serde configuration alters this.

---

## §3 — Architectural Reasoning

Reasoning only; no decision is stated in this section.

**On whether B1 is technically sound for C12's own concrete instance.** The Architecture Evaluation's own empirical verification, independently reconfirmed here (§2), is not in question. `Option<PersistedModHealthDimension>` without `#[serde(default)]` deserializes an old record's absent key to `None`, deserializes a new record's key to `Some(...)`, and requires no change to `PersistedFinding`'s other four fields, `ReportStore`, or `recurring_patterns` (which never reads `mod_health_dimension` today and is unaffected either way). Nothing in this session's own re-inspection of current source contradicts this.

**On whether the question this Resolution answers is broader than "which type does `mod_health_dimension` get."** The Process Determination's own Prong 1 failure was not specific to `ModHealthDimension` — it was a finding about `modiq-storage`'s persisted schema having *no* adopted principle governing field-shape evolution at all, evidenced by a gap named four times, against different fields, across Sprint 22, Sprint 24, C3, and C4 (§2). Resolving only "`mod_health_dimension` shall be `Option<...>`" would leave that broader gap exactly as unresolved as before for the next field a future capability proposes — repeating, rather than closing, the pattern `SPRINT24_IMPLEMENTATION_REPORT.md` itself warned against ("worth a dedicated evaluation before a third field-shape change compounds it further"). This is a real forcing consideration, not a preference for abstraction over concreteness: the four-times-repeated record is itself evidence that a purely instance-scoped answer under-serves what has actually been demonstrated to recur.

**On why the broadest possible principle is not adopted either.** No evidence gathered — by this Resolution, by the Architecture Evaluation, or by the Process Determination's own exhaustive search of `GOVERNANCE.md`, `docs/adrs/`, and `STORAGE_ARCHITECTURE_EVALUATION.md` — shows that schema versioning or migration is architecturally undesirable, only that neither currently exists and neither has yet been forced by a concrete need. Foreclosing either mechanism for all future capabilities, based on evidence that only ever examined one field addition, would repeat exactly the "prospective certainty-manufacturing" error `SPRINT12.md` and `SPRINT12_ARCHITECTURAL_RESOLUTION.md` caution against in a different context, and would violate this repository's own "capability before abstraction" discipline (`PROJECT_HANDOFF_v1.1.md` §6, Principle 1) in the opposite direction — building a permanent prohibition ahead of any concrete need for one.

**On the correct shape for a middle path, drawn from `C2_ARCHITECTURAL_RESOLUTION_...md`'s own precedent (§2).** That Resolution's Decision 2 was stated generally ("when two or more Rules independently interpret the same `EvidenceCategory`...") but explicitly narrowed relative to the broadest alternative on the table, and its own §6 conditioned reuse on "that capability's own confirmation that its facts genuinely match" rather than automatic inheritance. Applied here: a generally-stated backward-readability expectation, narrowed to *readability* only (not historical-value preservation, not a specific mechanism), with reuse conditioned on a future capability's own confirmation of fit — not an unconditional universal rule.

**On why `Option<T>` is not made the mandated mechanism.** The Evaluation's own Alternative comparison (§7 of that document) already establishes that `Option<T>` is one of several ways to satisfy backward readability (alongside schema versioning, migration, and — orthogonally — retrieval redesign). Mandating `Option<T>` specifically as the *architectural principle* would conflate a design-time implementation choice with the property it happens to satisfy for this field, the same category error C2's own Decision 2 avoided by stating its constraint in terms of the *outcome* required (non-contradiction) rather than the *mechanism* (a specific matching-logic pattern).

**On the meaning of `None`, and why it requires an explicit statement here rather than deferral.** `PersistedEvidence.location: Option<String>` and `PersistedRecommendation.repair_recipe_reference: Option<String>` (§2) are both direct mirrors of an *already-`Option`* Runtime field — `Evidence::new` explicitly documents "no known location" as a live domain state, decided identically regardless of when the Evidence is created. `Finding.mod_health_dimension` carries no such Runtime-side optionality; every `Finding`, at every point in this repository's history, has been constructed with a concrete, required `ModHealthDimension`. A persisted `None` under B1 therefore means something categorically different from the two existing precedents — a persistence-schema-vintage condition, not a domain fact — and a future reader unfamiliar with this distinction could reasonably, but wrongly, read `None` the same way as `location: None`. This is exactly the kind of previously-open ambiguity an Architecture Evaluation is permitted to leave named but unresolved (its own §8-equivalent risk register) and an Architectural Resolution exists to close, in the same shape `C2_ARCHITECTURAL_RESOLUTION_...md`'s Decision 2 closed the Evaluation's own named Explainability ambiguity. Closing it requires only a definitional sentence, not any code, serde attribute, or aggregation logic — it is squarely within this Resolution's own authority and does not encroach on Implementation Planning's separate responsibility for *how* an absent value is displayed.

**On whether this Resolution touches historical-value recovery.** It does not, and must not: `CAPABILITY_DEFINITION_C12...md` §6 already, correctly, and finally established that the true historical dimension cannot be recovered from persisted bytes, named the `rule_reference`-based inference mechanism only for completeness, and declined to select it. Nothing in the backward-readability principle adopted below implies, requires, or forecloses that inference — readability and historical-value recovery are answers to different questions, and conflating them would silently expand this Resolution's own scope beyond what either the Capability Definition or the Process Determination authorized it to reach.

---

## §4 — Alternative Dispositions

| Alternative (from `C12_ARCHITECTURE_EVALUATION_...md` §6) | Disposition | Basis |
|---|---|---|
| **A** — Required field, no compatibility mechanism | **Rejected** | Concretely, traceably violates C12's own stated success criterion (Evaluation §9); matches Sprint 22/24's own precedent, which was named as accepted risk, not validated design (§2, §3). |
| **B1** — `Option<PersistedModHealthDimension>`, no `#[serde(default)]` | **Adopted** | Satisfies backward readability directly and unconditionally (§9 of the Evaluation, reconfirmed §2–§3 here); lowest complexity of every alternative evaluated; introduces no fabricated data. |
| **B2** — `Option<PersistedModHealthDimension>` + `#[serde(default)]` | **Not adopted; not prohibited** | Behaviorally identical to B1 for this scenario (Evaluation §6–§7); the attribute is redundant, not harmful — a later Implementation Plan may add it for stylistic explicitness without contradicting this Resolution, since this Resolution decides the representation (`Option<PersistedModHealthDimension>`), not the presence or absence of a redundant attribute. |
| **B3** — Non-`Option` field + fabricated `Default` | **Rejected** | `ModHealthDimension` has no `Default` impl and no semantically neutral variant (§2); any concrete default would misrepresent a historical record as having an assigned dimension it never had — directly in tension with the historical-data boundary (§8 below). |
| **C** — Schema versioning | **Deferred** | Not shown by any evidence gathered across three lifecycle artifacts and this Resolution to be necessary for C12's own instance; remains available to a future Architectural Resolution under the exception path (§10) if a concrete need for it materializes. |
| **D** — Migration/backfill (including `rule_reference` inference) | **Deferred, in part; the inference mechanism specifically remains not selected** | Migration of representation is not needed once B1 secures readability; the `rule_reference`-based inference for historical *value* remains exactly as the Capability Definition left it — named, not selected, not authorized (§8). |
| **E** — Explicit accepted historical-data limitation | **Adopted, as the description of B1's own consequence** | Not a separate mechanism from B1 — this Resolution's own §7/§8 state explicitly that old records remain readable but permanently lack a captured dimension, which is exactly Alternative E's content, paired with B as the Evaluation itself already framed it. |
| **F** — Retrieval/error-handling redesign | **Deferred** | Orthogonal to this decision; not required once B1 secures readability; left open for independent future justification, not decided here. |

No alternative disposition reopens `CAPABILITY_DEFINITION_C12...md`'s own Capability Statement, Architectural Boundaries, or Explicit Exclusions, or `STORAGE_ARCHITECTURE_EVALUATION.md`'s own four Decisions; all remain exactly as committed.

---

## §5 — Decision 1: C12 Persisted Representation

**`PersistedFinding` gains `mod_health_dimension: Option<PersistedModHealthDimension>`, with no `#[serde(default)]` required.** `PersistedModHealthDimension` is expected to mirror `ModHealthDimension`'s six variants, following the same `From<RuntimeEnum>` conversion shape already established for `PersistedAssessmentStatus`, `PersistedEvidenceCategory`, `PersistedFindingSeverity`, and `PersistedRecommendationStepKind` (§2) — this Resolution does not itself name the mirror type's exact fields, derives, or file location, which remain Implementation Plan detail (§13).

This is a decision about **persisted representation only**:
- `Finding.mod_health_dimension: ModHealthDimension` on the Runtime side remains required and unchanged — this Resolution neither touches nor implies any change to `crates/modiq-runtime`.
- `None` on the persisted field does not mean the Runtime `Finding` had no dimension — every `Finding` has one, always.
- `None` does not mean "not applicable."
- `None` does not mean the dimension was intentionally absent from the domain.
- **`None` means the persisted record predates capture of this field in the persisted schema — the value was never represented in that record's own persisted form.**
- The true historical dimension of such a record cannot be recovered from its persisted bytes (unchanged from `CAPABILITY_DEFINITION_C12...md` §6).
- This decision does not authorize inference, migration, or reconstruction of that value (§8).

---

## §6 — Decision 2: Backward-Readability Principle

**Adopted Architectural Principle — Backward-Readable Persisted Evolution:**

> When an already-shipped `Persisted*` structure gains a new field, the change **should** preserve backward readability of previously-persisted records by default. This is satisfied through ordinary design-time discipline in how the new field is represented — for example, but not exclusively, `Option<T>` — not through a new runtime mechanism, registry, or validation layer. A future capability whose field addition cannot reasonably satisfy this expectation through such a design-time choice **may** instead be governed by a different mechanism (schema versioning, migration, or an explicit accepted-unreadability decision), but only if a dedicated Architectural Resolution evaluates and adopts that mechanism for a documented reason (§10). Absent such a Resolution, backward readability is the expected default.

**Word choice, stated deliberately.** "Should," not "must" or "shall": this principle states an expectation calibrated to what the evidence here actually supports — a concrete, four-times-repeated gap, closed for its own general shape by a design-time discipline that cost nothing to apply in C12's own case (§3) — not an absolute, unconditional rule whose violation would itself be a defect regardless of context. A future capability that genuinely cannot satisfy it through ordinary design-time means is not in breach; it is directed to the exception path (§10), exactly as `C2_ARCHITECTURAL_RESOLUTION_...md`'s own Decision 2 used "SHALL" only for the narrow, fully-evidenced verification step it actually mandated, and softer language elsewhere. "By default" is retained because it is precisely the qualifier that keeps this principle from becoming the blanket prohibition §3 already rejected — it names an expectation, not an invariant enforced by any mechanism.

**This principle governs backward readability only.** It does not govern, and this Resolution does not decide:
- Historical-value recovery or reconstruction (§8).
- Migration or inference mechanisms (§8).
- Aggregation or presentation semantics for an absent value (§9).
- Schema-versioning technology or design (§10).
- The specific JSON representation of any field beyond `PersistedFinding.mod_health_dimension` itself (§5).
- A universal requirement that every future `Persisted*` field use `Option<T>` — `Option<T>` is the mechanism adopted for C12's own concrete instance (§5) because it satisfies the principle at the lowest cost for this field; a future field may satisfy the same principle through a different design-time mechanism, subject to that future capability's own Process Determination or Architecture Evaluation confirming fit, exactly as `PROCESS_DETERMINATION_C3...md`'s and `PROCESS_DETERMINATION_C4...md`'s own reuse of the two-prong test required fresh confirmation each time rather than assumed inheritance.

---

## §7 — `None` Semantics

Restated as its own section because it is the load-bearing definitional decision this Resolution makes, distinguished explicitly from the two existing `Option<T>` precedents:

| | `PersistedEvidence.location` / `PersistedRecommendation.repair_recipe_reference` | `PersistedFinding.mod_health_dimension` (adopted here) |
|---|---|---|
| Runtime field type | Already `Option<T>` | Required, non-`Option` |
| What `None` represents | A live, per-instance domain fact, true identically for a new or old record | A persistence-schema-vintage condition — the record predates the field's capture |
| Does `None` ever change meaning based on when the record was written? | No | Yes — this is exactly what distinguishes it |
| Could a "current" record legitimately have `None`? | Yes, by domain design | No — every record persisted after this decision's implementation carries `Some(...)` |

**`None` on `mod_health_dimension` must not be interpreted as equivalent to, or presented as indistinguishable from, `location`'s or `repair_recipe_reference`'s own `None`.** It represents absence of the value *from the persisted representation*, not absence of the underlying domain concept, and it must not be treated as a fabricated historical dimension, nor as "not applicable," nor as "never evaluated." This distinction is architectural, not presentational: whichever future artifact designs how an aggregation surfaces this value (§9) must design against this meaning, not invent its own.

---

## §8 — Historical Data Boundary

Restated, not re-litigated: `CAPABILITY_DEFINITION_C12...md` §6 established that no persisted record's own bytes can recover the true, contemporaneous `ModHealthDimension` of a Finding predating this decision's implementation. This Resolution:

- **Does not adopt** the `rule_reference → current Rule → current dimension` inference mechanism named (not selected) by the Capability Definition.
- **Does not adopt** any migration or backfill mechanism.
- **Does not adopt** any sentinel or fabricated default value.
- **Does not decide** whether such an inference or migration mechanism should ever be pursued by some later, independent capability — that question remains exactly as open, and exactly as unauthorized, as the Capability Definition left it. This non-decision is not a permanent prohibition; a future Architectural Resolution remains free to reach a different conclusion if a concrete forcing function for historical-value recovery ever arises, subject to its own evidence and evaluation.

---

## §9 — Scope of the Adopted Principle

Explicitly bounded, so this Resolution is not later read more broadly than it was evaluated:

**In scope:** backward readability of already-persisted `Persisted*` records when a field is added; the concrete representation for `PersistedFinding.mod_health_dimension` (§5); the architectural meaning of `None` for that field (§7).

**Explicitly out of scope**, matching `PROCESS_DETERMINATION_C12...md` §10 and `C12_ARCHITECTURE_EVALUATION_...md` §1 exactly:

- How `history_analysis`'s own future dimension-aware aggregation should label, group, or display a `None` value — an Implementation Planning question, contingent on but not settled by this Resolution.
- Historical-value reconstruction or inference (§8).
- C5 (cross-mod dependency resolution) or C11 (`AssessmentSubject`/Report Identity).
- GOV-016, in any respect.
- General re-evaluation of `modiq-storage`'s architecture, or of the Storage Mirror pattern's own general standing — unaffected and unreopened; this Resolution concerns field-shape evolution specifically, not how a mirror is constructed once its shape is decided.
- Any transport, UI, or presentation redesign; any change to `apps/console`, `apps/sandbox`, or `modiq-cli` beyond what a later, separate artifact may additively choose to present.

---

## §10 — Exceptions / Future Architectural Decisions

**A future Architectural Resolution may adopt a different schema-evolution mechanism — schema versioning, migration, or an explicit accepted-unreadability decision — for a field addition that cannot reasonably satisfy §6's backward-readability expectation through ordinary design-time means, provided that Resolution independently evaluates and documents the consequences and rationale for departing from the default.** This is not a standing invitation to bypass §6 by assertion; it requires the same evidentiary discipline this Resolution itself applied (§3) — a concrete forcing function, not a hypothetical one, and an independent Architecture Evaluation or equivalent investigation establishing why the default does not fit. This exception path exists specifically so that §6's principle remains a rebuttable expectation, not an unconditional rule that would foreclose schema versioning or migration for every future capability regardless of need (§3).

---

## §11 — Consequences

- **For C12 itself:** the Implementation Plan that follows may proceed to add `mod_health_dimension: Option<PersistedModHealthDimension>` to `PersistedFinding`, consistent with §5, without needing its own dedicated Architecture Evaluation of the backward-readability question — that question is now closed at the architectural level by this Resolution.
- **For future capabilities:** a citable default now exists for the "how does a new `Persisted*` field handle old records" question, without requiring the next capability to re-derive this entire investigation from Sprint 22/24's own unresolved record — while still requiring that capability to confirm its own facts match (§6), not to inherit the answer by citation alone.
- **For `modiq-storage`'s existing behavior:** `ReportStore`, `recurring_patterns`, and every existing `PersistedFinding` field remain unaffected in meaning; the existing `rule_reference`/`severity` aggregation's own success criterion (Evaluation §9) is satisfied by this decision, not merely accommodated.
- **For historical data:** no improvement to historical completeness (unchanged from the Capability Definition); no degradation of historical accuracy (§7's `None` semantics prevent any record from being misread as having a value it never had).
- **For Storage's founding architecture:** `STORAGE_ARCHITECTURE_EVALUATION.md`'s four Decisions remain unmodified; its §7 non-goal is neither reopened nor converted into a permanent rule — this Resolution answers the *deferred question*, it does not amend the document that deferred it.

---

## §12 — Explicit Non-Decisions

This Resolution does not:

- Modify any source file, including `persisted_report.rs`, `report_store.rs`, `history_analysis.rs`, or any Runtime crate.
- Add, specify, or authorize any `serde` attribute, including `#[serde(default)]`, on any field.
- Perform, design, or authorize a migration or backfill.
- Reconstruct, infer, or authorize inference of any historical `ModHealthDimension` value.
- Redesign `recurring_patterns` or any other retrieval/error-handling behavior.
- Decide how `None` is presented, labeled, or aggregated in any future output.
- Touch UI, transport, `apps/console`, `apps/sandbox`, or `modiq-cli` beyond what §9 already excludes.
- Touch C5, C11, or GOV-016.
- Re-evaluate the Storage Mirror pattern's own general standing.
- Modify `GOVERNANCE.md`.
- Create an ADR.
- Create an Implementation Authorization or Implementation Plan.
- Create an Engineering Release.

---

## §13 — Implementation Boundary

The next lifecycle artifact — an Implementation Authorization, if the Chief Architect proceeds — is the first document permitted to define participating files, exact field/type names, exhaustive-match discipline for the new enum mirror, verification gates, and completion criteria. This Resolution authorizes none of that; it authorizes only the architectural representation decided in §5 and the principle decided in §6, both at the level of "what must be true," not "what code accomplishes it" — the same separation `C2_ARCHITECTURAL_RESOLUTION_...md` §8 drew between its own Decision 2 and "whatever Rule eventually implements C2."

---

## §14 — Governance / Documentation Implications

**`GOVERNANCE.md` is not recommended as the location for §6's substantive principle.** `GOVERNANCE.md`'s Crate Boundary Rule sections — including `## Storage`, and its structural analog `## Rule Engine` — hold terse, unconditional "Owns:"/"Must never:" facts about subsystem ownership and prohibition, not conditional architectural expectations with their own rationale and exception path. `C2_ARCHITECTURAL_RESOLUTION_...md` §6/§7 already establishes this repository's own precedent for exactly this fork: that Resolution's own substantive constraint (Decision 2, Rule Conclusion Non-Contradiction) was recommended, not for `GOVERNANCE.md`'s `## Rule Engine` entry, but for `RuleEngine.md` — the dedicated architecture-tier document — "mirroring its own existing Initiative 1 AC-2/AC-4 citation pattern, to record Decision 2 as architecture text." `GOVERNANCE.md`'s own GOV-012 entry received, at most, a recommendation (not performed) that "a future housekeeping pass add a citation to this Resolution alongside it" — a cross-reference, not the constraint's own substantive text. §6's principle here has the same relevant shape as Decision 2 (conditional, "should... by default," an explicit exception path, intended for reuse subject to a future capability's own confirmation of fit) and is treated identically:

- **A future, targeted housekeeping cross-reference to this Resolution in `GOVERNANCE.md`'s `## Storage` entry is recommended, not performed** — analogous to the GOV-012 cross-reference C2's own Resolution recommended, naming this Resolution and its adopted principle without restating the principle's own substantive text there.
- **The substantive backward-readability principle (§6) belongs in an architecture-tier document**, not in `GOVERNANCE.md`'s own terse boundary-rule format. `docs/architecture/` currently contains no dedicated Storage architecture document analogous to `RuleEngine.md`, which is itself a gap this Resolution merely observes, not one it is authorized to close. Two documentation paths are available for a future, separately-authorized step to choose between: (1) extending `STORAGE_ARCHITECTURE_EVALUATION.md` — already the document `GOVERNANCE.md`'s own `## Storage` entry directs readers to for "the full evaluation, authorization, and representation rationale" — with a clearly marked, dated addendum recording this Resolution's own principle as citable architecture text; or (2) creating a dedicated Storage architecture document, if a future housekeeping review judges the repository's documentation structure to warrant one. **This Resolution does not choose between these two paths** — no repository evidence reviewed here establishes one as clearly correct over the other, and deciding between them is not the architectural question this Resolution was charged to resolve.

**No amendment to `GOVERNANCE.md`, `STORAGE_ARCHITECTURE_EVALUATION.md`, any `docs/architecture/` file, `docs/adrs/`, or any Governance Register item is performed by this document.** Any future amendment along either path above requires its own, separate authorization.

---

## §15 — Lifecycle Position

```
Capability Definition (dd4d377)
  → Process Determination (2975c93)
    → Architecture Evaluation (9af4d2b)
      → Architectural Resolution (this document — uncommitted)
        → Implementation Authorization — NOT YET CREATED
          → Implementation — NOT PERFORMED
            → Engineering Release — NOT PERFORMED
```

This Resolution does not itself authorize implementation. No source change, migration, or backfill has occurred as a consequence of this document. The next required step, if the Chief Architect concurs with §5/§6, is a dedicated Implementation Authorization — not performed here.

---

## §16 — Final Resolution

**Adopted.** `PersistedFinding.mod_health_dimension` is represented, for future implementation, as `Option<PersistedModHealthDimension>`, with no `#[serde(default)]` required (§5). A backward-readability expectation for already-shipped `Persisted*` structures is adopted as a reusable, evidence-calibrated architectural principle, scoped to readability only, stated as a rebuttable default rather than an absolute rule, with an explicit exception path reserved for a future, independently-justified Architectural Resolution (§6, §10). `None`'s architectural meaning on this field is fixed as a persistence-schema-vintage condition, explicitly distinguished from this repository's two existing, semantically different `Option<T>` precedents (§7). Historical-value recovery, inference, and migration remain exactly as unauthorized and undecided as the Capability Definition left them (§8). No source file, `GOVERNANCE.md`, or ADR has been created or modified by this Resolution. A future housekeeping cross-reference in `GOVERNANCE.md` is recommended, not performed; the substantive principle's architecture-tier documentation home is identified but not chosen (§14). This Resolution's responsibility ends here, awaiting Chief Architect review.

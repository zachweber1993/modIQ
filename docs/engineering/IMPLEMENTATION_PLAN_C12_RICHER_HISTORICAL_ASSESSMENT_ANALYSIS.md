# Implementation Plan — C12: Richer Historical Assessment Analysis

| Property | Value |
|---|---|
| **Document** | IMPLEMENTATION_PLAN_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md |
| **Project** | modIQ |
| **Origin** | `IMPLEMENTATION_AUTHORIZATION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md` (`5739dd4`), treated as fixed, unreopened. `ARCHITECTURAL_RESOLUTION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md` (`b000963`), `C12_ARCHITECTURE_EVALUATION_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md` (`9af4d2b`), `PROCESS_DETERMINATION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md` (`2975c93`), and `CAPABILITY_DEFINITION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md` (`dd4d377`) are fixed inputs, not restated. `IMPLEMENTATION_PLAN_C4_APPS_SANDBOX_FIELD_PARITY.md` and `IMPLEMENTATION_PLAN_C3_CLI_FIELD_PARITY.md` supply this document's own closest structural precedent — a Storage-domain field addition with a bounded, no-new-mechanism scope; `C2_IMPLEMENTATION_PLAN_DECLARED_DEPENDENCY_INTERPRETATION.md` supplies the closer precedent for this Plan's own task of deciding a genuinely reserved design question with grounded reasoning, since C12's own Authorization — like C2's — reserved more than a literal-formatting detail. |
| **Status** | Implementation Planning draft. No source file has been modified in preparing this document — every design decision below is stated descriptively, at the repository-design level, not as compile-ready source. |

---

## 1. Implementation Objective

Implement exactly what `IMPLEMENTATION_AUTHORIZATION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md` §3 authorizes: add `mod_health_dimension: Option<PersistedModHealthDimension>` to `PersistedFinding`, introduce the corresponding `PersistedModHealthDimension` enum mirror, and extend `modiq-storage`'s historical-analysis capability so `ModHealthDimension` recurrence can be determined from persisted reports — without altering the existing `rule_reference`/`severity` aggregation's behavior, without touching `modiq-runtime`, and without performing any historical inference, migration, backfill, or schema versioning. This document decides the design questions the Authorization left open (§7 below), sequences the work into phases, and defines its own completion — it does not itself perform, or begin, any implementation phase.

---

## 2. Current Repository State

Confirmed directly this session via `git`, fresh, not carried forward from the preceding investigation's own account:

- **Branch:** `feature/runtime-implementation`
- **HEAD:** `5739dd47cd2712d34a3196da9ee7d1dc80d12a14` — `docs: Implementation Authorization for C12 (Richer Historical Assessment Analysis)`
- **Working tree:** Clean.
- **Relative to `origin/feature/runtime-implementation`:** 1 commit ahead (the committed, not-yet-pushed Authorization), 0 behind.

C12's committed lifecycle to date: Capability Definition (`dd4d377`) → Process Determination (`2975c93`) → Architecture Evaluation (`9af4d2b`) → Architectural Resolution (`b000963`) → Implementation Authorization (`5739dd4`). No C12 Implementation Plan has yet been committed — this document is that draft. No implementation source has changed since `5739dd4`.

`crates/modiq-storage/src/storage/persisted_report.rs`, `history_analysis.rs`, `report_store.rs`, `mod.rs`, and `crates/modiq-cli/src/commands/history.rs` were each re-read in full this session, immediately before drafting the decisions below.

---

## 3. Authorization Basis

Restated from `IMPLEMENTATION_AUTHORIZATION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md`, not re-derived. The following remain fixed architectural constraints this Plan does not reopen:

1. `PersistedFinding` gains `mod_health_dimension: Option<PersistedModHealthDimension>` (Authorization §3; Resolution §5, Decision 1, Adopted).
2. `#[serde(default)]` is not required (Resolution §5; empirically confirmed twice already, at Evaluation and Resolution stage, against this repo's pinned `serde`/`serde_json` versions — not re-verified here since no new claim about `serde` behavior is made by this Plan).
3. `None` on `mod_health_dimension` means the persisted record predates capture of this field — a persistence-schema-vintage condition (Resolution §7).
4. `None` does not mean: no dimension exists; not applicable; never evaluated; a fabricated dimension; or a Runtime-domain optionality (Resolution §7; `Finding.mod_health_dimension` remains a required, non-`Option` Runtime field, unchanged).
5. Runtime `Finding` and `ModHealthDimension` remain unchanged (Authorization §4, §5, §6 Invariant 7).
6. No historical inference, migration, backfill, reconstruction, or sentinel substitution is authorized (Resolution §8; Authorization §5, §6 Invariant 5/8).
7. No schema-versioning mechanism is introduced (Resolution §4, Alternative C, Deferred; Authorization §5).
8. No `ReportStore` retrieval/error-handling redesign is authorized (Resolution §4, Alternative F, Deferred; Authorization §5).
9. No C5, C11, or GOV-016 work (Authorization §5).
10. No `GOVERNANCE.md`, architecture-document, or ADR modification (Authorization §4, §5, §11).
11. No `apps/console` or `apps/sandbox` work (Authorization §4, §5).
12. The general backward-readability principle (Resolution §6) remains in force as a reusable, evidence-calibrated default for future `Persisted*` field additions — this Plan neither restates it as binding on unrelated future work nor weakens it for C12's own field.
13. This Plan does not universalize `Option<T>` as the mandatory mechanism for all future persisted fields — it applies `Option<PersistedModHealthDimension>` to this one field, exactly as the Resolution itself adopted (Resolution §6: "`Option<T>` is the mechanism adopted for C12's own concrete instance... a future field may satisfy the same principle through a different design-time mechanism").

---

## 4. Authorized Implementation Scope

Restated from `IMPLEMENTATION_AUTHORIZATION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md` §3, not re-derived: Implementation Planning is authorized to scope work that adds the persisted representation (§7.1 above), extends `history_analysis.rs`'s aggregation to additionally account for `ModHealthDimension` alongside — never in replacement of — the existing `rule_reference`/`severity` aggregation, decides how the extended logic treats `None` (subject to Invariant 4), and optionally extends `modiq-cli history` presentation. No field may be presented or persisted beyond what `Finding::mod_health_dimension()`'s existing, already-public getter exposes; no `Persisted*` field other than `PersistedFinding.mod_health_dimension` may be added, altered, or removed; `ReportStore::store`/`retrieve`'s method signatures and `AssessmentService`'s two public entry points remain unchanged.

---

## 5. Participating Files

**Required:**

- `crates/modiq-storage/src/storage/persisted_report.rs`
- `crates/modiq-storage/src/storage/history_analysis.rs`

**Conditional:**

- `crates/modiq-cli/src/commands/history.rs` — **not exercised by this Plan** (Decision 4, §7.4 below). Remains untouched; the option remains available to a later, separate Plan.

**Reference-only / not participating** — confirmed unchanged by direct source inspection this session:

- `crates/modiq-runtime/src/assessment/finding.rs` — `Finding::mod_health_dimension()` is read, not modified; already public, unchanged since Sprint 22.
- `crates/modiq-runtime/src/assessment/mod_health_dimension.rs` — `ModHealthDimension`'s six variants are read, not modified.
- `crates/modiq-storage/src/storage/report_store.rs` — `store`/`retrieve`/`list_keys` are generic over `PersistedAssessmentReport`'s own `Serialize`/`Deserialize` derive; none requires a code change for the new field to round-trip. **This Plan's own test design (§9) deliberately avoids adding any test to this file's test module, precisely to keep it at zero diff, matching the Authorization's own classification.**
- `crates/modiq-storage/src/storage/mod.rs` — its `pub use persisted_report::{...}` re-export list is **not** extended with `PersistedModHealthDimension` (Decision, §7.1 below).
- `modiq-rules`, `modiq-collection`, `modiq-versioning`, `modiq-engine`, `modiq-report`, `modiq-knowledge`, `apps/console`, `apps/sandbox`, `GOVERNANCE.md`, any `docs/architecture/` file, any ADR.

No other file is expected to require modification. If implementation discovers that another file must change, implementation should stop and report the deviation rather than silently expanding scope — matching every prior Implementation Plan's own identical commitment.

---

## 6. Explicit Exclusions

Restated from `IMPLEMENTATION_AUTHORIZATION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md` §5, not reopened: any change to `crates/modiq-runtime`, `Finding`, or `ModHealthDimension`; historical inference, migration, backfill, or reconstruction; sentinel or fabricated default values for `PersistedModHealthDimension`; schema versioning; retrieval/error-handling redesign of `ReportStore::retrieve` or `recurring_patterns`'s existing `Err`-propagation contract; general re-evaluation of `modiq-storage`'s architecture or the Storage Mirror pattern's own general standing; any `GOVERNANCE.md`/`STORAGE_ARCHITECTURE_EVALUATION.md`/architecture-document/ADR amendment; C5, C11, or GOV-016, in any respect; `apps/console`, `apps/sandbox`, or any transport/UI redesign.

---

## 7. Reserved Design Questions — Decided Here

`IMPLEMENTATION_AUTHORIZATION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md` §5's second sub-list and §7 left four questions genuinely open. This Plan decides each now, since implementation requires a concrete target; no literal Rust code below is compile-ready source.

### 7.1 Decision 1 — Persisted Representation Mechanics

**The Plan adopts, within the already-authorized envelope:** a `PersistedModHealthDimension` enum mirroring `ModHealthDimension`'s six variants (`Compatibility`, `Stability`, `Maintainability`, `Performance`, `Structure`, `EngineeringQuality`), declared with `#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]`, placed in `persisted_report.rs` immediately before `PersistedFinding`'s own struct definition, with `impl From<ModHealthDimension> for PersistedModHealthDimension` as a six-arm match — identical in every mechanical respect to the four existing mirrors in the same file (`PersistedAssessmentStatus`, `PersistedEvidenceCategory`, `PersistedFindingSeverity`, `PersistedRecommendationStepKind`), confirmed by direct re-read this session. No new abstraction is introduced; this is the existing Storage Mirror pattern applied a fifth time, exactly as the Resolution's own §5 anticipates ("expected to mirror `ModHealthDimension`'s six variants, following the same `From<RuntimeEnum>` conversion shape").

`PersistedFinding` gains the field `mod_health_dimension: Option<PersistedModHealthDimension>` and a getter `pub fn mod_health_dimension(&self) -> Option<PersistedModHealthDimension>`, matching every existing field's own by-value-`Copy` getter shape (`severity()`, for example). `PersistedFinding::from_finding` populates it as `Some(PersistedModHealthDimension::from(finding.mod_health_dimension()))` — never `None` for a freshly-converted `Finding`, since `Finding.mod_health_dimension` is a required Runtime field; `None` arises only when an old-shape record is deserialized without the key ever having been written.

**On `mod.rs`'s re-export list — decided not to extend it.** `mod.rs`'s existing `pub use persisted_report::{...}` list already omits `PersistedRecommendationStepKind` (introduced at Sprint 24) — direct, current-source proof, re-confirmed this session, that a new `Persisted*` enum mirror does not require this re-export to be usable. `history_analysis.rs` (Phase 2, §7.2) will reference the new type via `super::persisted_report::PersistedModHealthDimension`, the identical intra-crate path its own existing `use super::persisted_report::PersistedFindingSeverity;` already uses — this is sibling-module access, not the external `pub use` surface `mod.rs` controls, and requires no change to `mod.rs` regardless. Since CLI participation is declined (§7.4), no external consumer needs the type named directly. This is a Plan-level implementation-shape choice, not an architectural one; if a future, separate Plan's own design genuinely needs the type re-exported, that Plan may add it without reopening anything decided here.

### 7.2 Decision 2 — Aggregation Shape

**The Plan selects a separate, additively-introduced aggregation — Option B from the prerequisite investigation — over extending `PatternFrequency`'s own grouping key.**

Traced directly against current source, not chosen for ease of coding: `recurring_patterns` groups by `(rule_reference, severity)` equality, producing exactly one `PatternFrequency` row with the true total `occurrences` for each pair present in a store. If `ModHealthDimension` were folded into that same grouping key (Option A), a store containing both a pre-C12 record (`None`) and a post-C12 record (`Some(dimension)`) for the *same* `(rule_reference, severity)` pair would split into multiple rows instead of the one row a caller reading `recurring_patterns` today would expect — fragmenting a total `modiq-cli`'s own `HistoryCommand` (the only current consumer, confirmed by direct source read) already prints as a single number. This is a direct, traceable risk to Authorization §6 Invariant 6 ("the existing `rule_reference`/`severity` aggregation must continue to function exactly as it does today, unmodified in meaning"), not a hypothetical one — and it would arise the moment any store held a mix of pre- and post-C12 records, which every real store will, the first time this ships.

A nested breakdown within the existing `PatternFrequency` row (Option C) avoids that fragmentation but requires restructuring an already-shipped, already-tested public type for a shape no C12 artifact requires — `CAPABILITY_DEFINITION_C12...md` §3 asks only that the new aggregation exist "alongside" the old one, not that the two be merged into one richer structure.

**Adopted: a new, independent function, `mod_health_dimension_recurrence(store: &ReportStore) -> Result<Vec<ModHealthDimensionFrequency>, ReportStoreError>`, and a new struct, `ModHealthDimensionFrequency { mod_health_dimension: PersistedModHealthDimension, occurrences: usize }`** (field type is non-`Option`, per §7.3's own consequence, not `Option<PersistedModHealthDimension>` — see below), declared in `history_analysis.rs` alongside — not replacing — `recurring_patterns`/`PatternFrequency`. It performs its own, separate iteration over `store.list_keys()` + `store.retrieve(&key)`, mirroring `recurring_patterns`'s exact algorithm shape (linear-scan grouping, `format!("{:?}", ...)` sort tiebreak, since `PersistedModHealthDimension` has no `Ord`/`Hash` impl, confirmed by direct inspection matching `PersistedFindingSeverity`'s own identical absence of both). `recurring_patterns`'s own signature, body, and output are untouched by this Plan — confirmed satisfying Invariant 6 by construction, not by careful scoping around a shared structure.

**Cost, named explicitly rather than left implicit:** a second, independent iteration over the store's own contents (a second `list_keys()` + `retrieve()` pass) — each stored report is read from disk twice if a caller invokes both functions. This is not a new category of risk: `CAPABILITY_DEFINITION_HISTORICAL_ASSESSMENT_ANALYSIS.md` §9 already named "reading every stored report into memory on each invocation" as an accepted, explicitly deferred scale limitation for the *existing* `recurring_patterns` itself; a second such pass is the same accepted risk exercised twice, not a new one this Plan introduces without precedent.

### 7.3 Decision 3 — `None` Handling

**The Plan adopts prospective-only completeness: `mod_health_dimension_recurrence` counts only `Some(...)` records; a record whose `mod_health_dimension()` is `None` contributes nothing to its output.**

Grounds: `CAPABILITY_DEFINITION_C12...md` §10's own Success Criterion states the historical-completeness limitation must be represented "in the shipped behavior **or its documentation**" (emphasis matches the source's own framing) — documentation-only satisfaction is explicitly sufficient, not merely a fallback. Since CLI participation is declined (§7.4), there is no presentation surface today that could misleadingly display an unlabeled gap; the only place the limitation needs to be legible is the function's own contract. `mod_health_dimension_recurrence`'s own doc comment will state explicitly — mirroring `list_keys`'s own existing doc-comment discipline of stating what happens for an unwritten root, rather than leaving it to be discovered — that a report persisted before this field existed contributes no entry to this function's output, and that this is a permanent limitation on historical completeness (Resolution §7/§8), not evidence the record has no dimension.

This satisfies Resolution §7's boundary directly: excluding a record from a count is not a claim about that record (nothing is asserted that the record "has no dimension" or "was never evaluated" — the record simply does not appear, exactly as `list_keys` today omits any report that was never written, without that omission being read as a claim about non-existent reports). The alternative — an explicitly labeled `None`/"pre-C12" bucket inside the aggregation output — was considered and is not adopted here, since it would add a labeling design with no current consumer to observe it (§7.4), and can be introduced by a future, separate Plan without reopening anything decided here, exactly as the Resolution's own §6 anticipates for future field-shape choices.

**Consequence for §7.2's own struct shape:** because `None` records never produce an entry, `ModHealthDimensionFrequency.mod_health_dimension` is `PersistedModHealthDimension` (non-`Option`) — there is never a `None` value to represent inside a result row.

This is a Plan-level implementation-shape decision, revisable by a future, separate Plan without a new Architectural Resolution — the Resolution's own §9 explicitly left this exact question "contingent on but not settled by this Resolution."

### 7.4 Decision 4 — CLI Participation

**The Plan declines to exercise `modiq-cli history`'s conditional participation. `crates/modiq-cli/src/commands/history.rs` is not touched by this Plan.**

Traced against the Capability Definition's own stated purpose, not decided on diff size alone: `CAPABILITY_DEFINITION_C12...md` §10's Success Criteria are stated entirely in terms of what the persisted historical-analysis pipeline can determine — none names CLI presentation as a completion requirement — and §12's own Repository Impact table names `modiq-cli`'s participation "conditional... not required for the capability's own backend completeness." This mirrors `IMPLEMENTATION_PLAN_C4_APPS_SANDBOX_FIELD_PARITY.md`'s own Decision 6b (declining `App.tsx`'s identically-optional extension) for the identical reason: the option exists, nothing forces exercising it, and exercising it now would widen this Plan's surface for no completion-criterion benefit.

**The backend capability remains complete without it.** `mod_health_dimension_recurrence` is a real, public, tested function on `modiq_storage::storage` the moment Phase 2 ships — reachable by any future consumer (a later CLI extension, `apps/console`, a test, or direct library use) exactly as `ReportStore::list_keys` was itself "real, tested content" before any consumer used it in anger, per `CAPABILITY_DEFINITION_HISTORICAL_ASSESSMENT_ANALYSIS.md`'s own founding precedent. Its result remains meaningful on its own terms — a `Vec<ModHealthDimensionFrequency>` a test or future caller can inspect directly — not contingent on any presentation surface existing yet.

**CLI presentation remains available to a later, separate capability/plan.** Nothing in this Plan forecloses it; `history.rs` is named Conditionally Participating by the Authorization, not Required, and remains exactly that for any future Plan that elects to exercise it.

---

## 8. Implementation Phases

### Phase 1 — Persisted Representation

**Objective.** Introduce `PersistedModHealthDimension` and `PersistedFinding.mod_health_dimension`, per §7.1, and empirically prove backward readability before anything downstream depends on it.

**Participating files.** `crates/modiq-storage/src/storage/persisted_report.rs` only (the new enum, its `From` impl, `PersistedFinding`'s new field and getter, `from_finding`'s extension, and the file's own existing `#[cfg(test)]` module, extended).

**Implementation tasks:**
- Declare `PersistedModHealthDimension` (§7.1), placed immediately before `PersistedFinding`.
- Implement `impl From<ModHealthDimension> for PersistedModHealthDimension` (six-arm match).
- Add `mod_health_dimension: Option<PersistedModHealthDimension>` to `PersistedFinding`; add its getter.
- Extend `PersistedFinding::from_finding` to populate the field via `Some(PersistedModHealthDimension::from(finding.mod_health_dimension()))`.
- Do not extend `mod.rs`'s re-export list (§7.1).

**Tests** (§9 gives full detail; summarized here):
- A fresh-`Finding`-to-`PersistedFinding` conversion asserts `Some(...)`.
- **The old-schema deserialization test** — a literal, hand-constructed JSON string representing the current five-field `PersistedFinding` shape (no `mod_health_dimension` key), deserialized directly via `serde_json::from_str::<PersistedFinding>`, asserting `Ok(...)` with `mod_health_dimension()` returning `None`. This is the direct, empirical proof of the Resolution's own adopted backward-readability decision — not assumed from the Evaluation's earlier, generic `serde` behavior test.
- A `Some(dimension)` serialize/deserialize round-trip, asserting equality preserved.
- All 4 existing tests in this file's own module reconfirmed passing, unmodified.

**Verification required.** `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` all clean; direct confirmation that no field other than `mod_health_dimension` was added to `PersistedFinding`, and that no other `Persisted*` struct was altered; direct confirmation that `mod.rs`, `report_store.rs`, and `history_analysis.rs` all show zero diff at this phase.

**Completion criteria.** `PersistedFinding` carries the new field; a report converted from a fresh `Finding` always yields `Some(...)`; a hand-constructed pre-C12-shape JSON payload deserializes successfully to `None` — proven by a passing test, not asserted; no other file shows a diff.

### Phase 2 — Dimension-Aware Aggregation

**Objective.** Introduce `mod_health_dimension_recurrence`/`ModHealthDimensionFrequency`, per §7.2/§7.3, and directly demonstrate — not assume — that the existing `rule_reference`/`severity` aggregation is unaffected by the new field's presence.

**Participating files.** `crates/modiq-storage/src/storage/history_analysis.rs` only (the new function, the new struct, and the file's own existing `#[cfg(test)]` module, extended). Depends on Phase 1's field existing.

**Implementation tasks:**
- Declare `ModHealthDimensionFrequency { mod_health_dimension: PersistedModHealthDimension, occurrences: usize }` with its own getters, mirroring `PatternFrequency`'s own getter shape.
- Implement `mod_health_dimension_recurrence(store: &ReportStore) -> Result<Vec<ModHealthDimensionFrequency>, ReportStoreError>`, per §7.2's algorithm: iterate `list_keys()` + `retrieve()`, skip any `Finding` whose `mod_health_dimension()` is `None` (§7.3), group the remainder by `PersistedModHealthDimension` equality, sort by `format!("{:?}", ...)`.
- Document, in the function's own doc comment, that a record predating this field's capture contributes no entry, and why (§7.3).
- `recurring_patterns`/`PatternFrequency` are not modified in any way.

**Tests:**
- `mod_health_dimension_recurrence` against an empty store is empty.
- Counts occurrences of the same dimension correctly.
- Distinguishes by dimension.
- **A mixed-vintage test**: one legacy-shape record, written directly via `std::fs::write` into the test's own `TempDir` at the exact `{key}.json` path `ReportStore` itself would use — bypassing `ReportStore::store` entirely, since Phase 1 guarantees every freshly-`store`d report always carries `Some(...)`, so this is the only way to construct a genuine `None`-carrying record in a test — alongside one or more `Some(...)`-carrying records stored normally. Asserts the legacy record contributes no entry while the others are counted correctly. This technique is confined entirely to `history_analysis.rs`'s own existing `TempDir`-based test pattern; it requires no change to `report_store.rs` itself (§5's own commitment).
- Deterministic ordering, mirroring `recurring_patterns_is_sorted_regardless_of_store_order` exactly.
- Never mutates the store, mirroring `recurring_patterns_never_mutates_the_store` exactly.
- **A direct regression test**, `recurring_patterns_is_unaffected_by_a_record_carrying_mod_health_dimension`: stores a report whose `Finding` carries a real dimension, calls `recurring_patterns` (not the new function), and asserts its output is identical in shape and count to what it would be without the new field — empirically demonstrating Invariant 6, not merely relying on the fact that `recurring_patterns`'s own source is untouched.
- All 5 existing `recurring_patterns` tests reconfirmed passing, unmodified.

**Verification required.** `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` all clean; direct confirmation that `recurring_patterns`'s own source shows no diff; direct confirmation that `report_store.rs` shows no diff.

**Completion criteria.** `mod_health_dimension_recurrence` correctly and deterministically distinguishes dimension values across multiple persisted reports; a mixed-vintage store's legacy record is excluded without error; `recurring_patterns`'s own existing behavior is proven, by a passing regression test, to be unaffected.

### Phase 3 — Final Reverification

**Objective.** Full-repository reverification and Repository Impact confirmation only — no further implementation.

**Participating files.** None.

**Verification required:**
- Root workspace: `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace`, all clean.
- `apps/sandbox/src-tauri`'s own separate workspace, reconfirmed clean and unaffected (no participating file lives there).
- A direct `git diff --stat` (or equivalent) confirming the only files touched across both phases are `persisted_report.rs` and `history_analysis.rs`, named in §5 — no other file, including `mod.rs`, `report_store.rs`, `history.rs`, and every `modiq-runtime` file, shows a diff.
- All C12 invariants (§3 above; Authorization §6) remain satisfied, checked directly, not assumed: backward readability holds; B1 is the shipped representation; `#[serde(default)]` was not added; `None`'s meaning is honored by both new code and its documentation; no historical truth was fabricated; the existing `rule_reference`/`severity` aggregation is unmodified; Runtime `Finding`/`ModHealthDimension` are untouched; no migration/inference/backfill/versioning mechanism exists anywhere in the diff; the implementation remains within C12's own bounded scope.

**Completion criteria.** Every Implementation Completion Criterion in §12 below holds simultaneously.

### Why This Ordering

Two substantive phases plus reverification, not one combined phase and not a finer split — the same discipline `C2_IMPLEMENTATION_PLAN_...md` and `IMPLEMENTATION_PLAN_C4_...md` each apply to comparably-scoped work. A single combined phase was considered and rejected: it would obscure whether a verification failure originates in the persisted representation itself or in the aggregation logic built on top of it — Phase 2 depends on Phase 1's field existing, but Phase 1 has no dependency on Phase 2, so a single phase would gate the (structurally simpler, lower-risk) representation change behind the (structurally more involved) aggregation work for no benefit. A finer split — for example, separating the enum mirror's introduction from `PersistedFinding`'s own field addition, or separating `ModHealthDimensionFrequency`'s declaration from `mod_health_dimension_recurrence`'s own logic — was considered and rejected as exceeding the minimum logical phases this Plan is asked to define: each pair shares one file, one direct dependency, and no independent verification gate would distinguish a failure in one from a failure in the other. Phase 3 mirrors every precedent Plan's own identical final-reverification shape exactly.

No phase begins before the prior phase's own verification gate (§10) passes in full.

---

## 9. Testing Strategy

Consistent with this repository's standing Real-I/O Testing Discipline (`PROJECT_HANDOFF_v1.1.md` §5) and both target files' own existing conventions: no mocking, no new test infrastructure, real `Finding`/`ModHealthDimension`/`PersistedFinding`/`ReportStore` values constructed through their own existing public constructors, `TempDir` helpers reused exactly as each file's own existing module already defines them.

Full test list, mapped to the prerequisite investigation's own ten-point coverage requirement:

1. **Fresh Finding → `PersistedFinding` produces `Some(dimension)`** — Phase 1, `persisted_report.rs`.
2. **Old-schema JSON without the new key deserializes successfully to `None`** — Phase 1, `persisted_report.rs`; the single most load-bearing new test, per §8's own framing.
3. **`Some(dimension)` round-trip** — Phase 1, `persisted_report.rs`.
4. **Dimension-aware aggregation** (counts, distinguishes by dimension) — Phase 2, `history_analysis.rs`.
5. **Mixed `Some`/`None` historical data** — Phase 2, `history_analysis.rs`, via the direct-file-write technique (§8, Phase 2).
6. **Deterministic ordering** — Phase 2, `history_analysis.rs`.
7. **Existing `recurring_patterns` behavior remains unchanged** — Phase 2's own regression test, plus reconfirmation of all 5 existing `recurring_patterns` tests.
8. **Existing tests remain intact** — all 4 `persisted_report.rs` tests and all 5 `history_analysis.rs` tests reconfirmed passing, unmodified, at every phase gate.
9. **CLI tests** — not applicable; CLI participation is declined (§7.4).
10. **Root workspace verification gates** — Phase 3.

No new test is added to `report_store.rs`'s own test module, `mod.rs`, or `history.rs` — all three remain at zero diff, per §5's own commitment.

---

## 10. Verification Gates

Before progressing past each phase:

- **After Phase 1:** `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` all clean; the old-schema deserialization test and the `Some(...)` round-trip test both pass; all 4 existing `persisted_report.rs` tests unmodified and passing; `mod.rs`, `report_store.rs`, `history_analysis.rs` all show zero diff.
- **After Phase 2:** the same three gates all clean; every new `history_analysis.rs` assertion (§9) passes; the regression test proving `recurring_patterns`'s own unaffected behavior passes; all 5 existing `history_analysis.rs` tests unmodified and passing; `recurring_patterns`'s own source shows no diff; `report_store.rs` shows no diff.
- **After Phase 3:** full root workspace and `apps/sandbox/src-tauri` both reverified clean and unaffected; `git diff --stat` confirms only `persisted_report.rs` and `history_analysis.rs` were touched across the entire implementation.

No phase begins before the prior phase's gate passes in full.

---

## 11. Repository Boundaries / Architectural Invariants

Restated from Authorization §6, as concrete implementation constraints this Plan's own phases must satisfy — none is a Plan-level choice, all remain architectural facts fixed upstream:

- Every report persisted before this change ships remains readable by `ReportStore::retrieve` after it ships.
- `Option<PersistedModHealthDimension>`, no `#[serde(default)]`, is the shipped representation — not a required field, not a versioned field, not a migrated field.
- `None` represents schema-vintage absence, never domain absence, in both code and documentation (§7.3).
- No historical truth is fabricated anywhere in this implementation.
- The existing `rule_reference`/`severity` aggregation functions exactly as it does today, unmodified in meaning — proven, not merely unmodified in source (Phase 2's own regression test).
- Runtime `Finding`/`ModHealthDimension` semantics remain unchanged; only their persisted mirror is touched.
- No migration, inference, backfill, or schema-versioning mechanism exists anywhere in this implementation.
- The implementation remains within C12's own bounded scope — no per-mod or cross-mod correlation, no `AssessmentSubject`/Report Identity content, no unrelated historical analytics.

---

## 12. Completion Definition

C12's implementation is complete when all of the following hold simultaneously:

- `PersistedFinding` carries `mod_health_dimension: Option<PersistedModHealthDimension>`; a report converted from a fresh `Finding` always yields `Some(...)`.
- A pre-C12 persisted report — proven directly, not assumed, by the old-schema deserialization test — remains readable after this change ships, unmodified in its existing `rule_reference`/`severity` presentation.
- `mod_health_dimension_recurrence` determines `ModHealthDimension` recurrence from any report persisted after this change ships, distinguishable from, and never replacing, the existing `rule_reference`/`severity` aggregation.
- A mixed-vintage store's pre-C12 records are excluded from dimension-aware aggregation without error, consistent with `None`'s architecturally-fixed meaning (§7.3).
- All verification gates (§10) pass clean, and every repository integrity check named in Phase 3 (§8) holds.
- No item named in `IMPLEMENTATION_AUTHORIZATION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md` §5 or this Plan's own §6 is touched, added, or implied.
- `crates/modiq-cli/src/commands/history.rs` remains untouched (§7.4) — this does not block completion.

---

## 13. Expected Next Artifact

**A C12 Implementation Report**, following this repository's own standard reporting convention, once the phases in §8 are actually carried out. Not produced by this document.

---

## 14. Lifecycle / Authority Boundary

This Plan translates `IMPLEMENTATION_AUTHORIZATION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md`'s own already-authorized envelope into concrete phases and design decisions (§7); it does not itself perform, or begin, any implementation phase, and no source file has been modified in preparing it. It decides only implementation-shape questions the Authorization explicitly reserved (§7) — it does not change architecture, does not modify the Architectural Resolution, does not introduce new architectural policy, does not authorize implementation beyond what the Authorization's own §3 already authorizes, and does not resolve C5, C11, or GOV-016. Every decision in §7 is described in this document's own terms as a Plan-level implementation choice made within the already-authorized envelope, not as newly adopted architecture — none alters, narrows, or broadens §3 above. Actual Implementation remains a distinct, later, separately-undertaken act, following this repository's own standing discipline of validating each lifecycle artifact before the next stage proceeds — the same discipline this C12 lifecycle has followed at every stage to date.

---

## Status

This document defines the concrete implementation design and phase sequence for C12, within the envelope `IMPLEMENTATION_AUTHORIZATION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md` already authorizes. It performs no implementation. No source file, test file, `GOVERNANCE.md`, or architecture document has been created or modified in preparing this document.

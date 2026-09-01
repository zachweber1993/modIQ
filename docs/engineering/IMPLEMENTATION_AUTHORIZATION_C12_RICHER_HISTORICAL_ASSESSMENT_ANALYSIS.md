# C12 Implementation Authorization — Richer Historical Assessment Analysis

| Property | Value |
|---|---|
| **Document** | IMPLEMENTATION_AUTHORIZATION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md |
| **Project** | modIQ |
| **Purpose** | Convert `ARCHITECTURAL_RESOLUTION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md`'s own adopted decisions into a bounded engineering envelope for Implementation Planning — participating files, exclusions, invariants, verification gates, and completion criteria — and nothing beyond that. |
| **Origin** | `CAPABILITY_DEFINITION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md` (`dd4d377`), `PROCESS_DETERMINATION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md` (`2975c93`), and `C12_ARCHITECTURE_EVALUATION_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md` (`9af4d2b`) — all treated as fixed, unreopened. `ARCHITECTURAL_RESOLUTION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md` (`b000963`) is the fully validated basis this Authorization is drafted against; its own §13 names an Implementation Authorization as the first document permitted to define participating files, exact field/type names, verification gates, and completion criteria. `C2_IMPLEMENTATION_AUTHORIZATION_DECLARED_DEPENDENCY_INTERPRETATION.md` is a directly relevant structural precedent, alongside `STORAGE_IMPLEMENTATION_AUTHORIZATION.md`, both of which follow a dedicated Architecture Evaluation and Architectural Resolution before Implementation Authorization rather than a Process Determination concluding directly to Authorization — adapted here for C12's own facts, including a Storage-only, single-crate scope C2 did not itself have. |
| **Status** | **Approved. Authorizes C12 Implementation Planning within the scope defined below. No Rust type, field name, enum variant, serde attribute, or code has been produced in preparing this document.** |

---

## 1. Authorization Decision

**Authorized with Scope Constraint.**

This determination is not newly reasoned here — it is `ARCHITECTURAL_RESOLUTION_C12_RICHER_HISTORICAL_ASSESSMENT_ANALYSIS.md`'s own conclusion, converted into binding form. Nothing in this document reopens Decision 1 (persisted representation), Decision 2 (the backward-readability principle), the `None` semantics fixed at that Resolution's §7, or the historical-data boundary fixed at its §8. This document does not reweigh Alternatives A–F, does not revisit why B1 was adopted over B2/B3/C/D, and does not reopen the Process Determination's own two-prong-test finding.

This document's own organization follows `C2_IMPLEMENTATION_AUTHORIZATION_DECLARED_DEPENDENCY_INTERPRETATION.md`'s shape — a repository precedent, alongside `STORAGE_IMPLEMENTATION_AUTHORIZATION.md`, whose own Architectural Resolution was reached through a dedicated Architecture Evaluation rather than a Process Determination concluding directly to Authorization (as C1, C3, and C4 each did). C2 is the closer precedent for this document's own organization; `STORAGE_IMPLEMENTATION_AUTHORIZATION.md` is the closer domain precedent, concerning `modiq-storage` and persisted representation directly, as C12 itself does. Adapted here for C12's own facts: a single crate (`modiq-storage`) rather than `modiq-rules`/`modiq-knowledge`; a persisted-representation change rather than a new Rule; and a conditionally-participating presentation surface (`modiq-cli history`) rather than none.

---

## 2. Architectural Basis

Derived exclusively from already-Adopted determinations; nothing here is newly reasoned:

- **`ARCHITECTURAL_RESOLUTION_C12...md` §5, Decision 1 (Adopted).** `PersistedFinding` gains `mod_health_dimension: Option<PersistedModHealthDimension>`, with no `#[serde(default)]` required. This is the entire basis for §3 below — the representation itself is not decided here; it is inherited.
- **`ARCHITECTURAL_RESOLUTION_C12...md` §6, Decision 2 (Adopted Architectural Principle).** Already-shipped `Persisted*` structures gaining a new field should preserve backward readability by default, through ordinary design-time discipline (e.g., `Option<T>`), unless a future, independently-justified Architectural Resolution adopts a different mechanism. Stated as an expectation ("should... by default"), not an unconditional rule — this Authorization applies it exactly as adopted, neither strengthening nor weakening its stated force.
- **`ARCHITECTURAL_RESOLUTION_C12...md` §7, `None` Semantics (Adopted).** `None` on `mod_health_dimension` represents a persistence-schema-vintage condition — the record predates capture of the field — categorically distinct from `PersistedEvidence.location`'s and `PersistedRecommendation.repair_recipe_reference`'s own domain-semantic `None`. This distinction governs §7 below and must not be collapsed by Implementation Planning.
- **`ARCHITECTURAL_RESOLUTION_C12...md` §8, Historical Data Boundary (Adopted).** No inference, migration, backfill, or sentinel-substitution mechanism is adopted; the `rule_reference → current Rule → current dimension` inference named (not selected) by the Capability Definition remains unauthorized.
- **`GOVERNANCE.md`'s Storage Crate Boundary Rule** — Storage's persisted representation is "populated only from `AssessmentReport`'s already-public API," and must never "persist `Assessment`, `Evidence`, `Finding`, or `Recommendation` as individually addressable entities" beyond the bundled snapshot, nor "require any change to `AssessmentService`'s public entry points." `Finding::mod_health_dimension()` (confirmed public, unchanged since Sprint 22, `crates/modiq-runtime/src/assessment/finding.rs`) already satisfies the "already-public" condition; this authorization does not extend or reinterpret the rule.
- **The Storage Mirror pattern**, settled and reusable (Sprint 13, 22, 24), confirmed unaffected and unreopened by the Architectural Resolution's own §9 — this Authorization relies on it for construction mechanics only, exactly as every existing `Persisted*` enum mirror (`PersistedAssessmentStatus`, `PersistedEvidenceCategory`, `PersistedFindingSeverity`, `PersistedRecommendationStepKind`) already demonstrates in `crates/modiq-storage/src/storage/persisted_report.rs`.
- **`CAPABILITY_DEFINITION_C12...md` §3's own Capability Statement** — extend `history_analysis` so its aggregation can additionally account for `ModHealthDimension`, alongside the already-existing `rule_reference`/`severity` aggregation, which must continue to function unmodified — remains the fixed scope of what is being planned.
- **`CAPABILITY_DEFINITION_C12...md` §12** — presentation-surface extension (`modiq-cli history`) is named as conditional participation only, mirroring the same optionality `IMPLEMENTATION_AUTHORIZATION_C4...md` §3 already exercised for `apps/sandbox/src/App.tsx`.

---

## 3. Authorized Planning Scope

Implementation Planning is authorized to scope work that:

- Adds `mod_health_dimension: Option<PersistedModHealthDimension>` to `PersistedFinding` (`crates/modiq-storage/src/storage/persisted_report.rs`), with no `#[serde(default)]` required, populated in `PersistedFinding::from_finding` by reading `Finding::mod_health_dimension()`.
- Introduces a new `PersistedModHealthDimension` enum mirror of `ModHealthDimension`'s six variants (`Compatibility`, `Stability`, `Maintainability`, `Performance`, `Structure`, `EngineeringQuality`), following the same `From<RuntimeEnum>` conversion shape the four existing `Persisted*` enum mirrors in the same file already establish. This authorization fixes which variants that mirror carries; it does not fix its exact derive list, field ordering, or placement within the file — reserved for Implementation Planning (§7).
- Extends `history_analysis.rs`'s aggregation so that `ModHealthDimension` recurrence can be determined from persisted reports, in addition to — not in replacement of — the existing `rule_reference`/`severity` aggregation. Whether this takes the shape of a new aggregation function, an extended `PatternFrequency`, or a separate result type is a design question reserved for Implementation Planning (§7), not decided here.
- Determines, as part of Implementation Planning, how the extended aggregation logic treats a `None` value for a given `PersistedFinding` — subject to §6's Invariant 4 (`None` must never be presented or treated as equivalent to a genuine domain absence) but otherwise undecided by this Authorization.
- **Optionally**, extends `crates/modiq-cli/src/commands/history.rs`'s presentation to reflect `ModHealthDimension` recurrence. This is conditional, additive scope, not required scope: `CAPABILITY_DEFINITION_C12...md` §12 names this "conditional participation," not required for the capability's own backend completeness. If Implementation Planning elects to extend it, presentation must be additive to the existing `rule_reference`/`severity` output, matching that command's own existing shape; if Implementation Planning elects not to, `history.rs` remains untouched and this does not block C12's own completion (§9).

This authorization is conditioned on all of the following:
- No field is presented or persisted that is not already exposed through `Finding`'s existing, already-public `mod_health_dimension()` getter.
- No change to `ReportStore::store`/`retrieve`'s own method signatures, or to `AssessmentService`'s two public entry points.
- The existing `rule_reference`/`severity` fields, and the existing `recurring_patterns`/`PatternFrequency` behavior for callers not concerned with `ModHealthDimension`, remain unchanged in meaning and content — supplemented, never replaced.
- No `Persisted*` field other than `PersistedFinding.mod_health_dimension` is added, altered, or removed.

No responsibility outside this list is authorized by this document.

---

## 4. Participating Files

**Participating:**

- `crates/modiq-storage/src/storage/persisted_report.rs` — `PersistedFinding`'s new field; the new `PersistedModHealthDimension` enum mirror; `from_finding`'s extension. Existing `#[cfg(test)]` module extended with new real-fixture assertions (§8), not replaced.
- `crates/modiq-storage/src/storage/history_analysis.rs` — aggregation logic extended per §3. Existing `#[cfg(test)]` module extended, not replaced.

**Conditionally participating:**

- `crates/modiq-cli/src/commands/history.rs` — participates only if Implementation Planning elects to extend presentation, bounded exactly as §3 states. If not elected, confirmed unchanged by this Authorization.

**Not participating — confirmed unchanged by this Authorization:**

- `modiq-runtime` (including `finding.rs` and `mod_health_dimension.rs`), `modiq-rules`, `modiq-collection`, `modiq-versioning`, `modiq-engine`, `modiq-report`, `modiq-knowledge` — `Finding::mod_health_dimension()` already exists, already public; none requires a change of any kind.
- `crates/modiq-storage/src/storage/report_store.rs` — `store`/`retrieve`'s own method signatures and their `serde_json` mechanism are unaffected; only the shape of what they serialize changes, via `persisted_report.rs`.
- `apps/console`, `apps/sandbox` — both separate workspaces, unaffected; neither is named by any C12 artifact as participating.
- `GOVERNANCE.md`, `STORAGE_ARCHITECTURE_EVALUATION.md`, any `docs/architecture/` file, any ADR — none is amended by, or as a precondition of, this authorization.

---

## 5. Explicit Exclusions

**Architecturally blocked or reserved — unaffected by this document:**

- **Any change to `crates/modiq-runtime`, `Finding`, or `ModHealthDimension`.** `Finding.mod_health_dimension` remains a required, non-`Option` Runtime field, unchanged (Architectural Resolution §5).
- **Historical inference, migration, backfill, or reconstruction of `ModHealthDimension` for pre-existing records** — the `rule_reference`-based inference named (not selected) by the Capability Definition remains unauthorized (Architectural Resolution §8). Not reachable through Implementation Planning; would require its own future, independently-justified Architectural Resolution (Resolution §10).
- **Sentinel or fabricated default values for `PersistedModHealthDimension`** — rejected as Alternative B3 (Resolution §4); `ModHealthDimension` has no `Default` impl and none is authorized.
- **Schema versioning** — Deferred (Resolution §4, Alternative C); not shown necessary for C12's own instance.
- **Retrieval/error-handling redesign of `ReportStore::retrieve` or `recurring_patterns`'s own existing `Err`-propagation contract** — Deferred (Resolution §4, Alternative F); orthogonal to this work.
- **General re-evaluation of `modiq-storage`'s architecture, or of the Storage Mirror pattern's own general standing** — settled and unreopened (Resolution §9).
- **Any `GOVERNANCE.md`, `STORAGE_ARCHITECTURE_EVALUATION.md`, or other architecture-tier document amendment; any ADR.** The Architectural Resolution's own §14 recommends, but does not perform, a future housekeeping cross-reference in `GOVERNANCE.md`'s `## Storage` entry, and leaves the substantive principle's architecture-tier documentation home undecided. Both remain separate, later, non-blocking housekeeping steps — not part of this authorization and not a precondition for it, mirroring `C2_IMPLEMENTATION_AUTHORIZATION_...md` §5's identical treatment of its own Resolution's recommended-not-performed follow-ups.
- **C5 (cross-mod dependency resolution), C11 (`AssessmentSubject`/Report Identity), or GOV-016, in any respect.**
- **`apps/console`, `apps/sandbox`, or any transport/UI redesign.**

**Outside the scope of this authorization by implementation scope — no architectural decision is required for any of these; they simply are not part of this work:**

- The exact field name, enum-mirror type name, derive list, or module placement for `PersistedModHealthDimension` — reserved for Implementation Planning, provided it follows the existing four-instance mirror shape without departing from it without a reason recorded there.
- The exact aggregation data-structure design in `history_analysis.rs` — reserved for Implementation Planning.
- The exact treatment of `None` within aggregation or presentation logic — reserved for Implementation Planning, subject to §6's Invariant 4.
- Whether `modiq-cli history` presentation is actually extended — reserved for Implementation Planning (§3).
- Any Implementation Plan, phase breakdown, task sequencing, or testing sequencing beyond §8's own boundary-level requirement.

---

## 6. Architectural Invariants That Must Not Be Violated

1. **Backward readability.** Every report persisted before this change ships must remain readable by `ReportStore::retrieve` after it ships — `retrieve` must not begin returning `Err` for a pre-existing report as a consequence of this work.
2. **B1 is the adopted persistence representation.** `mod_health_dimension: Option<PersistedModHealthDimension>`, no `#[serde(default)]` required — not a required field, not a versioned field, not a migrated field.
3. **`#[serde(default)]` is not required by the architecture.** Its addition is permitted as purely stylistic (Resolution §4, B2), never treated as a missing requirement.
4. **`None` represents schema-vintage absence, never domain absence.** It must never be presented, labeled, or treated as equivalent to "no dimension," "not applicable," "never evaluated," a neutral/unknown Runtime dimension, or `PersistedEvidence.location`'s own domain-semantic `None`.
5. **No historical truth may be fabricated.** No inference, sentinel, or fabricated default may stand in for a `ModHealthDimension` value a pre-existing record never captured.
6. **The existing `rule_reference`/`severity` aggregation must continue to function exactly as it does today**, unmodified in meaning, for callers not concerned with `ModHealthDimension`.
7. **Runtime `Finding`/`ModHealthDimension` semantics remain unchanged.** `Finding.mod_health_dimension` remains required and non-`Option`; this work touches only its persisted mirror.
8. **No migration, inference, backfill, or schema-versioning mechanism is authorized by this document.**
9. **The implementation must remain within C12's own bounded scope** — no per-mod or cross-mod correlation, no `AssessmentSubject`/Report Identity content, no unrelated historical analytics (Capability Definition §11).

---

## 7. Reserved for the Implementation Plan

Not decided by this Authorization; the Implementation Plan's own responsibility:

- Exact implementation sequence and phase/task breakdown.
- Exact type/field declaration details beyond the representation adopted at §3/§5 above (`Option<PersistedModHealthDimension>`, no `#[serde(default)]`).
- Exact enum-mirror placement, field ordering, and derive list for `PersistedModHealthDimension`.
- Aggregation data-structure design in `history_analysis.rs`.
- The exact code change extending `history_analysis.rs`'s aggregation logic.
- The exact treatment of `None` in aggregation and, if elected, presentation.
- Whether `modiq-cli history` presentation is actually implemented.
- Exact test fixture construction and exact test-file changes.
- Implementation ordering across phases, if more than one is warranted.

---

## 8. Required Verification Gates

- `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` — the root workspace. `apps/sandbox/src-tauri`'s own separate workspace, reconfirmed clean as a repository-integrity check, since no file in that workspace is named as participating or conditionally participating (§4).
- New, real-fixture tests, extended within `persisted_report.rs`'s and `history_analysis.rs`'s own existing `#[cfg(test)]` modules — no mocking, per this project's standing Real-I/O Testing Discipline — covering, at minimum:
  - A pre-C12 persisted JSON payload (the current five-field `PersistedFinding` shape, no `mod_health_dimension` key present) deserializes successfully, with the new field resolving to `None`.
  - A `PersistedFinding` constructed from a `Finding` with a concrete `ModHealthDimension` serializes and round-trips correctly, resolving to `Some(...)`.
  - The extended `history_analysis` aggregation correctly distinguishes `ModHealthDimension` values across multiple persisted reports.
  - The extended aggregation remains deterministic, independent of directory-read order — mirroring `recurring_patterns_is_sorted_regardless_of_store_order`'s own existing shape.
  - The existing `rule_reference`/`severity` aggregation's own five existing tests continue to pass unmodified in meaning.
- Direct confirmation (diff/grep, not assertion) that no field was added to `PersistedFinding` other than `mod_health_dimension`, and that no other `Persisted*` struct was altered.
- Direct confirmation that no file outside §4's Participating/Conditionally-participating list was modified, and that `Cargo.toml`/`Cargo.lock` show zero diff.
- If `crates/modiq-cli/src/commands/history.rs` was extended: its own existing tests extended, not replaced, mirroring that file's current shape.

---

## 9. Completion Criteria

Stated at the authorization-envelope level — what must be true of the Implementation Plan and its resulting implementation, not a prescription of the implementation's own internal tasks:

- The C12 Implementation Plan translates this Authorization's scope (§3) into concrete phases and design decisions, without exceeding it or reopening any item in §5.
- Every Architectural Invariant (§6) is satisfied and directly, verifiably demonstrated — not merely asserted — by the resulting implementation.
- A pre-C12 persisted report remains readable, unmodified in its existing `rule_reference`/`severity` presentation, after the change ships.
- `ModHealthDimension` recurrence is determinable from any report persisted after the change ships, distinguishable from the existing `rule_reference`/`severity` aggregation, not replacing it.
- No item named in §5 (Explicit Exclusions) is touched, added, or implied by the resulting implementation.
- Full verification gates (§8) pass clean.

---

## 10. Expected Next Artifact

**C12 Implementation Plan** — translating this Authorization's scope into concrete phases, the exact `PersistedModHealthDimension` type/module design, the exact `history_analysis.rs` aggregation design, the `None`-handling design, whether `modiq-cli history` presentation is elected, and testing sequencing. Not produced by this document.

---

## 11. Lifecycle / Authority Boundary

**This document authorizes C12 Implementation Planning within the scope defined above.**

It does **not** authorize:

- Implementation of any kind.
- Modification of any source file, test file, `GOVERNANCE.md`, or architecture document.
- Creation of an ADR.
- Historical inference, migration, backfill, or schema versioning.
- Deployment of any kind.
- An Engineering Release.

Implementation Planning is itself a distinct, subsequent, separately-produced artifact (the C12 Implementation Plan, §10). Implementation — the Lead Engineer's work against an authorized scope — remains a further, later, separately-authorized step, per this repository's own Permanent Engineering Workflow (`PROJECT_HANDOFF_v1.1.md` §5) and per the unanimous shape of every prior Implementation Authorization in this repository (§1). No Rust type, field name, enum variant, serde attribute, or code has been produced in preparing this document.

---

## Status

This document defines the engineering envelope for C12's Implementation Planning and authorizes that planning within it. It does not authorize implementation beyond what a subsequent C12 Implementation Plan itself scopes within §3, nor any work named in §5. No source file, test file, `GOVERNANCE.md`, or architecture document has been created or modified in preparing this document.

# Sprint 24 Implementation Plan — Runtime-Owned Representation of RepairRecipe-Derived Structure

| Property | Value |
|---|---|
| **Document** | SPRINT24_IMPLEMENTATION_PLAN.md |
| **Project** | modIQ |
| **Origin** | `RECOMMENDATION_REPAIR_STRUCTURE_IMPLEMENTATION_AUTHORIZATION.md` (`3f2728a`), treated as fixed, unreopened architecture. The Architecture Evaluation and Architectural Resolution for "Runtime-Owned Representation of RepairRecipe-Derived Structure" are fixed inputs behind that Authorization and are not restated here. Initiative 3 (Sprint 22, `969e595`) and its Frontend Presentation (Sprint 23, `d5eefb4`) are fixed precedent for this document's own format and phase discipline, not reopened. |
| **Status** | Sprint Planning draft. No code, crate, or test has been produced in preparing this document. |

---

## 1. Objective

Implement exactly what `RECOMMENDATION_REPAIR_STRUCTURE_IMPLEMENTATION_AUTHORIZATION.md` §3 authorizes: a new Runtime-owned value representation projecting `RepairRecipe`'s per-step structure, populated by `VersionCompatibilityRule` at `Recommendation`-construction time, supplementing `action` (never replacing it), and mirrored into `modiq-storage`'s `PersistedRecommendation` so it survives persistence and retrieval. Propagate the resulting `Recommendation::new` constructor-signature change through every real and test consumer so the full workspace builds and passes cleanly. No new capability beyond §3's four bullets, no IPC/transport/presentation change, no `RepairRecipeReference` resolution mechanism, and no item named in Authorization §4.

---

## 2. Authorized Scope

Direct translation of Authorization §3's four bullets into this Sprint's concrete work:

- **Item A.** A new Runtime-owned value type, `modiq-runtime`, capturing the per-step `kind`/`instruction` shape `RepairRecipe::steps()` already carries. Named `RecommendationStep` (fields: `kind: RecommendationStepKind`, `instruction: String`) with a companion closed-set enum `RecommendationStepKind` (five variants, mirroring `modiq_knowledge::knowledge::RepairStepKind`'s own five: `XmlChange`, `LuaChange`, `DependencyInstallation`, `AssetReplacement`, `VersionUpdate`).

  *Naming decision:* deliberately **not** named `RepairStep`/`RepairStepKind`. `modiq_knowledge::knowledge::RepairRecipe` and the new Runtime type are both visible, by name, in the same file (`version_compatibility_rule.rs`) at the point of projection. Reusing Knowledge's own names there would force every reference at that call site to be fully qualified or aliased to stay unambiguous — a self-inflicted readability cost with no architectural benefit, since the Resolution requires two *distinct* types (Runtime must never hold Knowledge's own type — ADR-0007, Authorization §7) regardless of what either is named. Distinct names make the boundary self-evident at the one call site where both crates are legitimately visible, rather than relying on qualification discipline to keep them apart.

  *Derive requirements:* `RecommendationStepKind` requires `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq` (mirroring `FindingSeverity`'s/`ModHealthDimension`'s own closed-set-enum shape). `RecommendationStep` requires `Debug`, `Clone`, `PartialEq`, `Eq`. Both are required for `Recommendation`'s own existing derive set to remain valid once it holds `Vec<RecommendationStep>`.

- **Item B.** `Recommendation` gains `repair_steps: Vec<RecommendationStep>`, populated only at construction (ADR-0007: population at construction only; Authorization §6: no post-construction setter or mutation path). `Recommendation::new` gains a fourth parameter, `repair_steps: Vec<RecommendationStep>`, appended after the existing three — not reordering any existing parameter. Empty (`Vec::new()`) is the "no Repair Recipe informed this Recommendation" state — mirroring how `Vec<FindingId>`/`Vec<EvidenceId>` are already plain, never-`Option`-wrapped collections elsewhere on this same type and on `Finding`, rather than introducing a new `Option<Vec<_>>` shape with no precedent on this platform. New getter: `repair_steps(&self) -> &[RecommendationStep]`.

  *Validation decision:* no new validation is added on `repair_steps` — not even "must be non-empty when `repair_recipe_reference` is `Some`." Introducing that relationship as an enforced invariant is a Level 3 (Behavioral) change under `GOVERNANCE.md`/ADR-0007's Governance-Controlled Invariants section and requires its own Governance Register item before enforcement code is written; it is not named in Authorization §3 and is not decided here. `repair_steps` stays exactly as permissive as `repair_recipe_reference` already is.

  `RecommendationStep::new(kind: RecommendationStepKind, instruction: impl Into<String>) -> Self` is **infallible** — no `Result`, no error type. `RecommendationStep` carries no identity of its own (it is not one of ADR-0007's four identity-bearing entities — Evidence, Finding, Recommendation, Assessment); it is a plain nested value, the same category as `modiq_knowledge::knowledge::RepairStep` (itself infallible) and this crate's own `RepairRecipeReference::new` (also infallible, also `impl Into<String>` with no validation).

- **Item C.** `VersionCompatibilityRule::evaluate` (`version_compatibility_rule.rs:52`), which already calls `recipe.steps()` to build the `action` string, additionally projects each `modiq_knowledge::knowledge::RepairStep` into a `RecommendationStep` via an explicit, exhaustive match on `RepairStepKind` (five arms, no wildcard `_ =>`, so a future sixth Knowledge-side variant fails to compile here rather than silently defaulting) and passes the resulting `Vec<RecommendationStep>` as `Recommendation::new`'s new fourth argument. The three other Rules that construct a `Recommendation` (`EvidencePresenceRule`, `StructuralDuplicationRule`, `RuntimeLoadFailureRule`) each pass `Vec::new()` — mechanical propagation only; none adopts a `RepairRecipe` (Authorization §4: "no new Rule adopting a `RepairRecipe` beyond `VersionCompatibilityRule`'s own existing usage").

- **Item D.** `modiq-storage`'s `PersistedRecommendation` gains a mirrored addition, following the exact pattern `PersistedFindingSeverity`/`PersistedEvidenceCategory` already establish in the same file: a `PersistedRecommendationStepKind` enum (five variants, `From<RecommendationStepKind>`, exhaustive match) and a `PersistedRecommendationStep { kind: PersistedRecommendationStepKind, instruction: String }` struct, both `Serialize + Deserialize`. `PersistedRecommendation` gains `repair_steps: Vec<PersistedRecommendationStep>`, populated inside the existing private `from_recommendation` conversion, plus a `repair_steps(&self) -> &[PersistedRecommendationStep]` getter. One-way only (`Recommendation` → `PersistedRecommendation`), matching this file's own existing shape — there is no reverse `to_recommendation` anywhere in `persisted_report.rs` today, for any field.

  *Derive requirements:* `PersistedRecommendationStepKind` requires `Debug`, `Clone`, `PartialEq`, `Eq`, `Serialize`, `Deserialize`. `PersistedRecommendationStep` requires `Debug`, `Clone`, `PartialEq`, `Eq`, `Serialize`, `Deserialize`. Both are required for `PersistedRecommendation`'s own existing derive set to remain valid.

---

## 3. Explicit Exclusions

Restated from Authorization §4, not re-derived:

- IPC transport extension (`apps/console`'s DTOs) — a future, separate authorization, mirroring how Sprint 22 (Runtime) and Sprint 23 (presentation) were deliberately separated.
- Frontend presentation of `repair_steps`, in any consumer application.
- Any change to `modiq-knowledge`, `modiq-engine`, `modiq-cli`, `apps/sandbox`, or `apps/console`.
- Any new Rule adopting a `RepairRecipe` beyond `VersionCompatibilityRule`'s own existing usage.
- Any resolution mechanism for `RepairRecipeReference`.
- Any new Cargo dependency edge, anywhere in the workspace.
- Any mutation method on `Recommendation`, `Finding`, or `Evidence`.

---

## 4. File / Crate Impact

Complete pre-implementation call-site inventory, confirmed directly (`grep -rn "Recommendation::new" --include="*.rs" .`) — Rust's own compiler enforces its completeness the moment `Recommendation::new`'s signature changes (Phase 1 gate, §7); no separate audit phase is required.

| Crate | File | Required change | Category |
|---|---|---|---|
| `modiq-runtime` | `assessment/recommendation_step.rs` (new) | `RecommendationStep`, `RecommendationStepKind` defined. | Production. |
| `modiq-runtime` | `assessment/mod.rs` | `pub mod recommendation_step;` + re-exports. | Production. |
| `modiq-runtime` | `assessment/recommendation.rs` | `repair_steps` field, getter, 4th constructor parameter; 11 existing test call sites updated in place (each passes `Vec::new()`); one new test added (`new_preserves_provided_repair_steps`, mirroring `new_preserves_provided_repair_recipe_reference`), containing its own new call site. | Production + tests. |
| `modiq-runtime` | `assessment/assessment.rs` | 4 existing test-fixture call sites (`sample_recommendation` and inline constructions at `assessment.rs:339,1270,1302,1337`) updated for the new 4th argument. | Tests only. |
| `modiq-rules` | `rules/version_compatibility_rule.rs` | Import `RecommendationStep`/`RecommendationStepKind`; exhaustive `RepairStepKind → RecommendationStepKind` projection; populate 4th argument from real `RepairRecipe` content; 2 existing tests extended with `repair_steps` assertions. | Production + tests. |
| `modiq-rules` | `rules/evidence_presence_rule.rs` | 4th argument, `Vec::new()`. | Production (mechanical). |
| `modiq-rules` | `rules/structural_duplication_rule.rs` | 4th argument, `Vec::new()`. | Production (mechanical). |
| `modiq-rules` | `rules/runtime_load_failure_rule.rs` | 4th argument, `Vec::new()`. | Production (mechanical). |
| `modiq-storage` | `storage/persisted_report.rs` | `PersistedRecommendationStepKind`, `PersistedRecommendationStep` defined; `PersistedRecommendation` gains `repair_steps` field, getter, population in `from_recommendation`; existing test fixture (`persisted_report.rs:309`) updated for the new 4th argument; one new round-trip-content test. | Production + tests. |
| `modiq-report` | `report/assessment_report.rs` | Existing test fixture (`assessment_report.rs:86`) updated for the new 4th argument. | Tests only — no production path change. |

**Why `modiq-report` is touched despite Authorization §5 listing it as "not participating":** its own `AssessmentReport::generate` reads `assessment.recommendations().to_vec()` — a whole-value clone, confirmed directly (`assessment_report.rs:28`) — never a per-field access, so no production behavior changes. Its `#[cfg(test)]` fixture, however, calls `Recommendation::new(...)` directly with the old three-argument shape and will not compile once the signature changes. This is the identical situation Sprint 22's own plan (§3) already named and resolved for this exact crate, under the identical `Finding::new` signature change: "Required, confined to test fixtures only — no production path changes." Applying that already-reviewed precedent here, rather than treating it as a new open question.

**Confirmed unaffected — no file change, verification-only:**

| Crate / App | Why unaffected |
|---|---|
| `modiq-knowledge` | `RepairRecipe`, `RepairStep`, `RepairStepKind` are read-only inputs to the projection in `modiq-rules`; nothing in this Sprint writes to `modiq-knowledge` (Authorization §3, §7). |
| `modiq-engine` | Orchestrates `RuleOutcome`/`Assessment::add_recommendation` by whole value; never constructs or destructures a `Recommendation` directly (confirmed — no `Recommendation::new`/`Recommendation {` match in `modiq-engine`). |
| `modiq-cli` | `retrieve.rs:57`, `assess.rs:95` read only `.action()` on a `Recommendation` — a real, existing consumption relationship, unlike the row below. No struct-literal or exhaustive destructure of `Recommendation`/`PersistedRecommendation` exists, and `action()` itself is unchanged by this Sprint, so no Sprint 24 implementation work is required here — verified, not merely unimplicated. |
| `modiq-collection`, `modiq-versioning` | No relationship to `Recommendation` at all; not implicated by this work. |
| `apps/sandbox` | `lib.rs:65,158` read only `.action()` on both `Recommendation` and `PersistedRecommendation` — no struct-literal or exhaustive destructure of either type exists, so the additive field does not break compilation. |
| `apps/console` | `assessment.rs:109` reads only `.action()` — same shape, same conclusion. |

---

## 5. Phase Breakdown

Each phase leaves the repository compilable and independently reviewable before the next begins, following Sprint 22's own per-phase discipline.

**Phase 1 — Foundational Runtime type (`modiq-runtime`).**
*Objective:* introduce `RecommendationStep`/`RecommendationStepKind` and extend `Recommendation` to carry them.
*Files:* `assessment/recommendation_step.rs` (new), `assessment/mod.rs`, `assessment/recommendation.rs`, `assessment/assessment.rs`.
*Completion criteria:* `cargo test -p modiq-runtime` clean. No other crate touched yet — expected broken workspace-wide at this point (acceptable within a phase, not across a phase boundary, per Sprint 22 §4 precedent).

**Phase 2 — Population and mechanical propagation (`modiq-rules`).**
*Objective:* populate `repair_steps` from real `RepairRecipe` content at the one authorized call site; propagate the constructor change through the three unrelated Rules.
*Files:* `rules/version_compatibility_rule.rs`, `rules/evidence_presence_rule.rs`, `rules/structural_duplication_rule.rs`, `rules/runtime_load_failure_rule.rs`.
*Completion criteria:* `cargo test -p modiq-rules` clean. No remaining `Recommendation::new(...)` call site in this crate using the old three-argument shape.

**Phase 3 — Storage mirror and mechanical propagation (`modiq-storage`, `modiq-report`).**
*Objective:* mirror the new shape into `PersistedRecommendation`; update the one remaining test-only call site outside `modiq-runtime`/`modiq-rules`.
*Files:* `storage/persisted_report.rs`, `report/assessment_report.rs`.
*Why sequenced after Phase 2, not parallel with it:* `modiq-storage` and `modiq-report` depend on `modiq-runtime` (Phase 1) only — neither has a Cargo dependency on `modiq-rules`, confirmed directly against both `Cargo.toml` files. Phases 2 and 3 are therefore independent and could be parallelized; they are sequenced here instead for deterministic, single-threaded review order matching Authorization §5's own crate listing (`modiq-runtime` → `modiq-rules` → `modiq-storage`), not because Phase 3 is compiler-gated by Phase 2.
*Completion criteria:* `cargo test -p modiq-storage -p modiq-report` clean; full root workspace (`default-members`) also clean at this point, since these are the last two participating/mechanically-touched crates.

**Phase 4 — Final workspace and application reverification.**
*Objective:* confirm nothing outside the participating/mechanically-touched crates was disturbed, empirically rather than by assertion.
*Files:* none — verification-only phase, matching Sprint 23 §4 Phase 4's own precedent shape.
*Completion criteria:* see §7 in full.

---

## 6. Testing Strategy

Per repository precedent — no mocking (standing discipline since Sprint 3 Phase 5); real fixtures throughout.

- **Phase 1:** one new test in `recommendation.rs`, `new_preserves_provided_repair_steps`, mirroring `new_preserves_provided_repair_recipe_reference` exactly — constructs a `Recommendation` with a non-empty `Vec<RecommendationStep>` and asserts `repair_steps()` returns it unchanged. All other existing `recommendation.rs`/`assessment.rs` call sites pass `Vec::new()`, unmodified in intent.
- **Phase 2:** `version_compatibility_rule.rs`'s `produces_a_warning_finding_for_an_unrecognized_declared_version` extended to assert `recommendation.repair_steps()` is non-empty and its first entry's `kind()`/`instruction()` match `RepairRecipe::version_compatibility_declared_version_mismatch()`'s own real step content — exercising the real projection, not a synthetic one. `is_deterministic_for_identical_input` extended to also compare `repair_steps` between two evaluations. The three mechanically-updated Rules require no new assertions — their own existing tests already implicitly cover the unchanged three-argument content; the fourth argument is `Vec::new()` on both sides of any equality check that already exists.
- **Phase 3:** one new test in `persisted_report.rs` asserting `PersistedRecommendation::from_recommendation` preserves `repair_steps` content and order — genuinely new behavior, not a mechanical update, mirroring Sprint 22 §5's own precedent for `PersistedFinding`'s title/summary round-trip test. `assessment_report.rs`'s fixture update requires no new assertion — its own test already asserts whole-value equality against the constructed `Recommendation`.
- **Regression expectation, every phase:** root workspace test count changes only by net-new tests (Phase 1: +1; Phase 2: 0 new tests, existing tests gain assertions; Phase 3: +1) — no existing test is deleted, only updated in place where the constructor's new argument requires it.
- Current baseline, confirmed directly (`cargo test --workspace` / `apps/sandbox/src-tauri`, this session): `modiq-runtime` 89, `modiq-rules` 36, `modiq-storage` 18, `modiq-report` 3, full root `default-members` workspace 269, `console` 5, `apps/sandbox/src-tauri` 9.

---

## 7. Verification Gates

Executable commands only; no phase begins before the prior phase's gate passes in full.

- **After Phase 1:** `cargo fmt --check`, `cargo check -p modiq-runtime`, `cargo test -p modiq-runtime` — all clean; expected ≥90 passed (89 baseline + 1 new).
- **After Phase 2:** `cargo check -p modiq-rules`, `cargo test -p modiq-rules` — all clean; expected 36 passed (no new test count, existing tests carry new assertions).
- **After Phase 3:** `cargo check -p modiq-storage -p modiq-report`, `cargo test -p modiq-storage -p modiq-report` — all clean; `modiq-storage` expected ≥19 passed (18 baseline + 1 new); `modiq-report` expected 3 passed (unchanged count, fixture updated in place).
- **After Phase 4:**
  - `cargo fmt --check`
  - `cargo check --workspace`
  - `cargo test --workspace`
  - `cd apps/sandbox/src-tauri && cargo fmt --check && cargo check && cargo test` — expected 9/9, unaffected in count.
  - `cargo check -p console --workspace && cargo test -p console` — expected 5/5, unaffected in count.
  - `git diff -- Cargo.toml 'crates/*/Cargo.toml' 'apps/*/*/Cargo.toml' | grep -E '^\+.*path *='` returning zero lines — confirming zero new dependency-edge lines in any workspace `Cargo.toml`.
  - `grep -rn '\bRepairRecipe\b\|\bRepairStep\b\|\bRepairStepKind\b' --include="*.rs" crates/modiq-runtime crates/modiq-rules crates/modiq-storage crates/modiq-report crates/modiq-engine crates/modiq-cli crates/modiq-collection crates/modiq-versioning apps` returning matches only within `crates/modiq-rules/src/rules/version_compatibility_rule.rs` — confirming no new leak of Knowledge Domain types beyond the one already-authorized call site.
  - `grep -rn "\.action()" apps/sandbox/src-tauri/src apps/console/src-tauri/src crates/modiq-cli/src crates/modiq-storage/src` returning the same call sites as today, confirming `action()`'s existing behavior and all four Authorization §8-named consumers (`apps/sandbox`, `apps/console`, `modiq-cli`, `modiq-storage`) remain unaffected.

---

## 8. Engineering Risks

Ranked by engineering impact:

1. **`Recommendation::new`'s constructor signature change is the widest breaking surface in this Sprint.** ~14 call sites in `modiq-runtime` alone (`recommendation.rs`, `assessment.rs`), all 4 Rules in `modiq-rules`, plus `modiq-storage`'s and `modiq-report`'s own test fixtures. Compiler-caught, not a silent-correctness risk (per §4's own precedent), but each site requires a real decision (`Vec::new()` vs. real content), not a single find-and-replace.
2. **Naming collision between the new Runtime type and `modiq-knowledge`'s own `RepairStep`/`RepairStepKind`**, specifically inside `version_compatibility_rule.rs` — the one file where both domains are legitimately visible together. Resolved at planning time (§2) by choosing distinct names (`RecommendationStep`/`RecommendationStepKind`) rather than leaving it for implementation to discover mid-Phase-2.
3. **Persistence compatibility for reports persisted before this Sprint.** `PersistedRecommendation` gains a required (non-`Option`) field with no `#[serde(default)]` — and no precedent for that mechanism exists anywhere in `modiq-storage` today, confirmed directly (no `serde(default)` usage anywhere in `storage/*.rs`). Any already-persisted report predating this Sprint will fail to deserialize on retrieval afterward. This is not a new category of risk this Sprint introduces — Sprint 22 already created the identical exposure when `PersistedFinding`'s shape changed (`description` → `title`/`summary`) with no migration mechanism. A persistence-migration or schema-versioning mechanism is new capability, not named in Authorization §3, and is not decided here — recorded as an accepted, pre-existing category of risk, not silently absorbed into this Sprint's scope.
4. **Exhaustive-match discipline for both new closed-set mappings** (`RepairStepKind → RecommendationStepKind` in `modiq-rules`; `RecommendationStepKind → PersistedRecommendationStepKind` in `modiq-storage`) must use explicit arms, no wildcard — mirrors `PersistedFindingSeverity`'s/`PersistedEvidenceCategory`'s own existing precedent in the same file. A wildcard arm would compile silently past a future sixth variant on either side.
5. **Workspace compilation order — lowest risk.** A single linear dependency chain (`modiq-runtime` → `modiq-rules` → `modiq-storage`/`modiq-report`), already enforced by Cargo itself; no circular or ambiguous ordering question exists among this Sprint's participating and mechanically-touched crates.

---

## 9. Completion Criteria

Restated from Authorization §9, made concrete:

- `RecommendationStep`/`RecommendationStepKind` exist in `modiq-runtime`, populated by `VersionCompatibilityRule` at construction time from real `RepairRecipe` content.
- `Recommendation` continues to expose `action`, unchanged.
- `PersistedRecommendation` carries `repair_steps: Vec<PersistedRecommendationStep>`, populated one-way from `Recommendation`.
- No new Cargo dependency edge exists anywhere in the workspace.
- No `modiq-knowledge` type is referenced outside `modiq-knowledge`'s own crate boundary and the one already-authorized `version_compatibility_rule.rs` call site.
- Full workspace verification (§7) passes clean.
- No item named in §3 (Explicit Exclusions) is touched, added, or implied.

---

## 10. Expected Repository State

At the close of this Sprint's implementation (a distinct, later step — not produced by this document):

- One new file: `crates/modiq-runtime/src/assessment/recommendation_step.rs`.
- Modified: `assessment/mod.rs`, `assessment/recommendation.rs`, `assessment/assessment.rs` (`modiq-runtime`); `version_compatibility_rule.rs`, `evidence_presence_rule.rs`, `structural_duplication_rule.rs`, `runtime_load_failure_rule.rs` (`modiq-rules`); `persisted_report.rs` (`modiq-storage`); `assessment_report.rs` (`modiq-report`).
- No other file under `crates/` or `apps/` modified.
- Full root workspace green (`cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace`); `apps/sandbox/src-tauri` and `console` green and unaffected in test count.
- No new `Cargo.toml` dependency edge anywhere.
- This document (`SPRINT24_IMPLEMENTATION_PLAN.md`) present in `docs/engineering/`, superseded in authority by the eventual Sprint 24 Implementation Report once implementation is authorized and performed.

Immediately after this document, per this repository's own workflow, the expected next artifacts — not produced here — are: phase-by-phase implementation (this document's own §5), Final Engineering Audit, Sprint 24 Implementation Report, Engineering Release, and tracking-document reconciliation.

---

## 11. Out of Scope

Within the authorized boundary but deliberately not decided or done by this Sprint:

- A persistence-migration or schema-versioning mechanism for `modiq-storage` (§8, Risk 3) — no such mechanism exists in this crate today, and introducing one is new capability, not authorized here.
- Any new `RecommendationError` variant or validation rule relating `repair_steps` to `repair_recipe_reference` (§2, Item B) — permissive by design, consistent with ADR-0007's Governance-Controlled Invariants restraint.
- A second `Recommendation` constructor, builder, or setter of any kind — construction-time population only (Authorization §6).
- Any change to how `action`'s own string content is produced or formatted in `VersionCompatibilityRule` — unchanged in this Sprint, supplemented only.
- Renaming, restructuring, or otherwise touching `modiq_knowledge::knowledge::RepairStep`/`RepairStepKind` to ease the projection — the projection accommodates Knowledge's existing shape; Knowledge does not change to accommodate Runtime.
- Everything named in §3.

---

## 12. Status

Sprint Planning draft, translating `RECOMMENDATION_REPAIR_STRUCTURE_IMPLEMENTATION_AUTHORIZATION.md` into concrete phases within its own fixed boundary. No code, crate, or test has been produced in preparing this document. Awaiting Chief Architect / Product Owner review and explicit authorization before Phase 1 of implementation begins.

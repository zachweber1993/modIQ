# Implementation Plan — C3: `modiq-cli` Field Parity

| Property | Value |
|---|---|
| **Document** | IMPLEMENTATION_PLAN_C3_CLI_FIELD_PARITY.md |
| **Project** | modIQ |
| **Origin** | `IMPLEMENTATION_AUTHORIZATION_C3_CLI_FIELD_PARITY.md` (`deff56e`), treated as fixed, unreopened. `PROCESS_DETERMINATION_C3_CLI_FIELD_PARITY.md` (`a595019`) and `CAPABILITY_DEFINITION_C3_CLI_FIELD_PARITY.md` (`31de26e`) are fixed inputs, not restated. `C1_IMPLEMENTATION_PLAN.md` and `C2_IMPLEMENTATION_PLAN_DECLARED_DEPENDENCY_INTERPRETATION.md` supply this document's own structural precedent, as the two most recent approved Implementation Plans in this repository. |
| **Status** | Implementation Planning draft. No Rust code has been produced in preparing this document — every design decision below is stated descriptively, at the repository-design level, not as compile-ready source. |

---

## 1. Purpose

This document translates `IMPLEMENTATION_AUTHORIZATION_C3_CLI_FIELD_PARITY.md`'s own authorized scope into concrete implementation phases: participating files, implementation order, the one reserved design question the Authorization left open (field presentation shape), a testing strategy, verification gates, repository integrity checks, and a completion definition. It authorizes no work beyond what `IMPLEMENTATION_AUTHORIZATION_C3_CLI_FIELD_PARITY.md` §3 already authorizes, and does not itself perform, or begin, any implementation phase.

---

## 2. Current Repository State

Confirmed directly this session via `git`:

- **Branch:** `feature/runtime-implementation`
- **HEAD:** `deff56e` — `docs: Implementation Authorization for C3 (CLI Field Parity)`
- **Working tree:** Clean.
- **Ahead of `origin/feature/runtime-implementation`:** 22 commits, 0 behind.

C3's committed lifecycle to date: Capability Definition (`31de26e`) → Process Determination (`a595019`) → Implementation Authorization (`deff56e`). No Architecture Evaluation or Architectural Resolution exists for C3, per the Process Determination's own conclusion. No Implementation Plan has yet been committed — this document is that draft. No implementation source has changed since `deff56e`.

---

## 3. Authorized Implementation Scope

Restated from `IMPLEMENTATION_AUTHORIZATION_C3_CLI_FIELD_PARITY.md` §3, not re-derived:

- `AssessCommand::format_report` (`crates/modiq-cli/src/commands/assess.rs`) gains, in addition to what it prints today: `Finding::mod_health_dimension()`, `Finding::status()`, `Evidence::location()`, `Evidence::label()`, `Evidence::source()`, `Evidence::content()` (each rendered only when `Some`), and `Recommendation::repair_steps()` (each step's `RecommendationStep::kind()`/`::instruction()`, rendered only when non-empty).
- `RetrieveCommand::format_report` (`crates/modiq-cli/src/commands/retrieve.rs`) gains, in addition to what it prints today: `PersistedEvidence::location()` (rendered only when `Some`) and `PersistedRecommendation::repair_steps()` (each `PersistedRecommendationStep::kind()`/`::instruction()`, rendered only when non-empty) — no other field.
- Every field already printed today (`category()`/`description()`, `severity()`/`title()`/`summary()`, `action()`) remains unchanged in meaning and content.
- No new CLI command, flag, or argument. No resolution of the `assess.rs`/`retrieve.rs` `format_report` duplication.

---

## 4. Participating Files

- `crates/modiq-cli/src/commands/assess.rs` — `format_report`'s extension (full field parity), plus its own existing `#[cfg(test)]` module, extended.
- `crates/modiq-cli/src/commands/retrieve.rs` — `format_report`'s extension (bounded field parity), plus its own existing `#[cfg(test)]` module, extended.

No other file is expected to require modification. If one does during implementation, implementation should stop and report it rather than proceed — matching `C1_IMPLEMENTATION_PLAN.md`'s and `C2_IMPLEMENTATION_PLAN_...md`'s own precedent for this exact commitment.

---

## 5. Explicit Exclusions

Restated from `IMPLEMENTATION_AUTHORIZATION_C3_CLI_FIELD_PARITY.md` §5, not reopened:

- Extending `modiq-storage`'s own `PersistedFinding`/`PersistedEvidence` schema.
- `apps/sandbox` field parity (C4).
- Resolving the `assess.rs`/`retrieve.rs` `format_report` duplication.
- Any change to `modiq-cli`'s own `history.rs` command, or to `modiq-storage`'s aggregate pattern-analysis code.
- `Confidence`, cross-mod dependency resolution, Lua Analysis, or any other Capability Portfolio Assessment candidate.
- Any new CLI command, flag, or argument.
- Any change to `modiq-runtime`, `modiq-rules`, `modiq-collection`, `modiq-versioning`, `modiq-engine`, `modiq-report`, `modiq-storage`, or `modiq-knowledge`.
- Any change to `apps/console` or `apps/sandbox`.

---

## 6. The Reserved Design Question, Decided Here

`IMPLEMENTATION_AUTHORIZATION_C3_CLI_FIELD_PARITY.md` §5 reserved "the exact format string, field ordering, indentation, or labeling convention for any newly presented field" to Implementation Planning, conditioned on not departing from the `{:?}` Debug-format and presence-conditional patterns named in its §2 without a reason recorded there. This Plan decides it now, descriptively, since implementation requires a concrete target; no literal format string or Rust code is authored here.

**Decision: every newly presented field is rendered as an additional, indented sub-line beneath the existing entry it belongs to, never folded into the existing single-line entries.**

Reasoning:

- `IMPLEMENTATION_AUTHORIZATION_C3_CLI_FIELD_PARITY.md` §3 requires that fields already printed today "remain unchanged in meaning and content — supplemented, never replaced." Appending new content to the existing `severity`/`title`/`summary` line, the existing `category`/`description` line, or the existing `action` line would make each denser without changing its meaning, but a separate, clearly-labeled sub-line keeps the existing line's own text byte-for-byte identical to today's output for every report that carries no new-field content — directly serving Completion Criterion 3 (`IMPLEMENTATION_AUTHORIZATION...md` §8: "A report with no non-default values for any newly presented field... presents with no error, placeholder, or synthesized content"), since an unconditionally-appended inline suffix would risk implying every entry always carries the new content, where a conditionally-emitted sub-line does not.
- This reuses direct precedent for two of its three parts, and departs from precedent, with a stated reason, for the third. `Finding`'s `mod_health_dimension`/`status` (one combined sub-line) and `Recommendation::repair_steps()` (one sub-line per step, emitted only when the `Vec` is non-empty) both match `apps/console`'s own `Reviewing.tsx` exactly: `{finding.modHealthDimension} · {finding.status}` renders as one line, and `repairSteps` renders as one list item per step. `Evidence`'s four optional fields do not match that same file's own layout: `Reviewing.tsx` renders `label` as an inline prefix and `location`/`source` as inline suffixes, all on the *same* line as `description`, giving only `content` its own separate line — not four separate lines for four fields. This Plan departs from that specific layout for `modiq-cli`'s own text output, choosing one sub-line per field uniformly instead, because `Reviewing.tsx`'s inline treatment depends on styled HTML (font weight, color, and punctuation conventions like ` (...)`/` — `) that `modiq-cli`'s plain-text output has no equivalent for; an explicit, uniformly labeled sub-line per field is the clearer choice for this medium, not an attempt to reproduce `Reviewing.tsx`'s own exact layout. What *is* precedented across all three parts, per the Authorization's own §2, is the presence-conditional mechanism — render only when `Some` (or non-empty) — not a specific line count per field.
- Each sub-line reuses the `{:?}` Debug-format convention for `mod_health_dimension()`, `status()`, and each step's `kind()` — the exact mechanism `severity()`/`category()` already use in the same functions — and plain text for `location()`, `label()`, `source()`, `content()`, and each step's `instruction()`, since none of those is an enum.
- `assess.rs` and `retrieve.rs` present `location` and `repair_steps` identically in shape, differing only in which fields each function's own type makes available — consistent presentation for a field both commands eventually share is a reasonable default, not an expansion of either function's authorized field set (§3).

**Not decided here, and left to implementation itself:** the literal sub-line prefix/indentation string, and the exact label text for each field (e.g., how `location` is labeled versus how a step's `kind` is labeled) — reserved as literal-formatting detail beneath what this Plan's own descriptive decision (above, including the four-separate-sub-lines layout for `Evidence`) fixes, and none of it is a compile-ready specification this document produces.

---

## 7. Implementation Phases

### Phase 1 — `assess.rs` Field Parity

**Objective.** Extend `AssessCommand::format_report` so a Finding's `mod_health_dimension()`/`status()`, an Evidence item's `location()`/`label()`/`source()`/`content()`, and a Recommendation's `repair_steps()` are each presented per §6's decision, without altering any field `assess.rs` already prints today.

**Participating files.** `crates/modiq-cli/src/commands/assess.rs` only (implementation and its own `#[cfg(test)]` module).

**Implementation tasks:**

- Extend the Findings loop's per-entry output with a sub-line presenting `mod_health_dimension()` and `status()` (both unconditional — neither field is optional on `Finding`).
- Extend the Evidence loop's per-entry output with up to four sub-lines presenting `location()`, `label()`, `source()`, and `content()`, each emitted only when `Some`.
- Extend the Recommendations loop's per-entry output with one sub-line per `RecommendationStep` in `repair_steps()`, each presenting that step's `kind()` and `instruction()`, emitted only when the `Vec` is non-empty.
- Extend the existing test module: a case constructing a Finding with a non-default `mod_health_dimension`/`status` and asserting both appear in `format_report`'s output; a case constructing an Evidence item with each of `location`/`label`/`source`/`content` present and asserting each appears; a case with all four absent and asserting none of the corresponding sub-lines appears; a case constructing a Recommendation with a non-empty `repair_steps` and asserting each step's `kind`/`instruction` appears; a case with an empty `repair_steps` and asserting no repair-step sub-line appears. Every new assertion is added alongside `assess.rs`'s own existing tests (`run_against_a_real_directory_succeeds_and_reports_evidence`, `run_against_a_real_directory_stores_the_report`, `run_against_an_empty_path_is_invalid_usage`, `run_against_a_nonexistent_path_is_an_execution_failure`), matching this crate's own Real-I/O Testing Discipline (real `Finding`/`Evidence`/`Recommendation` values, constructed directly through their own public constructors — no mocking) — none of the four existing tests is deleted, weakened, or has its own assertion altered.

**Verification required.** `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` (both the root workspace and `apps/sandbox/src-tauri`'s own separate workspace) all clean; direct confirmation (diff/grep) that every field newly printed in `assess.rs` traces to an existing, already-public getter named in §3, and that no field already printed by `assess.rs` before this phase changed in meaning or content.

**Completion criteria.** A Finding with a non-default `mod_health_dimension`/`status`, an Evidence item with any of `location`/`label`/`source`/`content` present, and a Recommendation with a non-empty `repair_steps` each show that content in `assess.rs`'s own output, distinguishable from the fields already printed before this phase; a report carrying none of that content presents exactly as it did before this phase, with no error, placeholder, or synthesized content; `crates/modiq-cli/src/commands/retrieve.rs` shows zero diff (untouched by this phase).

### Phase 2 — `retrieve.rs` Field Parity

**Objective.** Extend `RetrieveCommand::format_report` so a `PersistedEvidence` item's `location()` and a `PersistedRecommendation`'s `repair_steps()` are each presented per §6's decision, bounded exactly to what `modiq-storage`'s current persisted mirror carries, without altering any field `retrieve.rs` already prints today.

**Participating files.** `crates/modiq-cli/src/commands/retrieve.rs` only (implementation and its own `#[cfg(test)]` module).

**Implementation tasks:**

- Extend the Evidence loop's per-entry output with one sub-line presenting `location()`, emitted only when `Some` — the same presentation shape Phase 1 gives `assess.rs`'s own `Evidence::location()`.
- Extend the Recommendations loop's per-entry output with one sub-line per `PersistedRecommendationStep` in `repair_steps()`, each presenting that step's `kind()` and `instruction()`, emitted only when the `Vec` is non-empty — the same presentation shape Phase 1 gives `assess.rs`'s own `repair_steps()`.
- Introduce no sub-line for `mod_health_dimension`, `status`, `label`, `source`, or `content` — `PersistedFinding` and `PersistedEvidence` carry none of them; this phase must not anticipate a future `modiq-storage` extension.
- Extend the existing test module: a case constructing a stored report whose `PersistedEvidence` carries a `location` and asserting it appears; a case with `location` absent and asserting no such sub-line appears; a case constructing a stored report whose `PersistedRecommendation` carries a non-empty `repair_steps` and asserting each step's `kind`/`instruction` appears; a case with an empty `repair_steps` and asserting no repair-step sub-line appears. Every new assertion is added alongside `retrieve.rs`'s own existing tests (`run_retrieves_a_previously_stored_report`, `run_against_an_unrecognized_key_is_invalid_usage`), matching the same Real-I/O Testing Discipline Phase 1 follows — none of the two existing tests is deleted, weakened, or has its own assertion altered.

**Verification required.** `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` (both workspaces) all clean; direct confirmation that `retrieve.rs`'s own additions are limited to `location()` and `repair_steps()`, with no attempt to present `mod_health_dimension`, `status`, `label`, `source`, or `content`; direct confirmation that no field already printed by `retrieve.rs` before this phase changed in meaning or content; direct confirmation that `crates/modiq-cli/src/commands/assess.rs` shows no diff beyond what Phase 1 already introduced.

**Completion criteria.** A stored report whose Evidence carries a non-`None` `location`, and whose Recommendation carries a non-empty `repair_steps`, each show that content in `retrieve.rs`'s own output identically to Phase 1's `assess.rs` presentation of the same two fields; a stored report carrying neither presents exactly as it did before this phase.

### Phase 3 — Final Reverification

**Objective.** Full-repository reverification and Repository Impact confirmation only — no further implementation.

**Participating files.** None — verification-only phase.

**Implementation tasks.** None.

**Verification required.** Full root-workspace `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace`; `apps/sandbox/src-tauri`'s own separate workspace reconfirmed clean and unaffected; a direct `git diff --stat` (or equivalent) confirming the only files touched across both phases are the two named in §4 — no other file shows a diff.

**Completion criteria.** Every completion criterion named in `IMPLEMENTATION_AUTHORIZATION_C3_CLI_FIELD_PARITY.md` §8 holds simultaneously, across both commands, with all verification gates (§9 below) passing clean.

### Why this ordering, and not a single phase or a different split

C3's own scope, per the Authorization itself, already divides along exactly one line: `assess.rs` (full parity) and `retrieve.rs` (bounded parity), each reading a structurally different type hierarchy (`modiq-runtime` live types versus `modiq-storage`'s `Persisted*` mirror). This is the same boundary the Capability Definition's own Capability Statement, the Process Determination's own Determination, and the Authorization's own §3/§4 all independently preserve — the Implementation Plan does not invent a new one. Each phase touches exactly one file, is independently verifiable (`cargo fmt --check`/`check --workspace`/`test --workspace`), and a failure in one phase cannot be confused with a defect in the other, since neither file's own implementation depends on the other's. A single combined phase was considered and rejected: it would obscure which command's own change caused a verification failure, and would not match the two-phase-plus-final-reverification shape `C2_IMPLEMENTATION_PLAN_...md` already establishes as this repository's own precedent for mechanical, additive, single-crate work of this size — `C1_IMPLEMENTATION_PLAN.md` itself used four phases, not two, but for a reason specific to its own different scope (a breaking Rust-to-TypeScript transport change requiring its one existing consumer's own continuity fix to land in a dedicated phase, per that Plan's own §4), a circumstance C3 does not share. A finer split (for example, one phase per field group within `assess.rs`) was also considered and rejected as exceeding "the minimum logical phases" this Plan is asked to define — the three field groups within `assess.rs` share one file, one function, and one test module, and no dependency exists between them that would require independent verification gates.

---

## 8. Testing Strategy

- Both phases extend each file's own existing `#[cfg(test)]` module in place — no new test file, no new test infrastructure, no mocking. Every new assertion constructs real `Finding`/`Evidence`/`Recommendation`/`RecommendationStep` values (Phase 1) or a real stored report via `ReportStore`/`PersistedAssessmentReport` (Phase 2) through their own existing public constructors, matching `PROJECT_HANDOFF_v1.1.md`'s own Real-I/O Testing Discipline and this crate's own existing testing convention in both files.
- Each phase requires both a present-value case and an absent/empty-value case per newly presented field, so that both the "content is shown" and "absence is not an error, placeholder, or synthesized content" halves of `IMPLEMENTATION_AUTHORIZATION_C3_CLI_FIELD_PARITY.md` §8's Completion Criteria are directly exercised, not assumed.
- No existing test in either file is deleted, weakened, or has its own assertion altered at any phase — test count grows only by net-new assertions.
- Phase 3 authors no new test; it re-runs every existing and newly added assertion unmodified, alongside full-repository and `apps/sandbox` reverification.

---

## 9. Repository Boundaries

Restated from `IMPLEMENTATION_AUTHORIZATION_C3_CLI_FIELD_PARITY.md` §6, as concrete implementation constraints, not re-derived:

- The CLI Crate Boundary Rule — `modiq-cli` remains user interaction, command execution, and platform entry point only; neither phase introduces business logic, Evidence evaluation, Finding/Recommendation generation, or Assessment mutation.
- The Storage Crate Boundary Rule — Phase 2 presents only what `PersistedEvidence`/`PersistedFinding`/`PersistedRecommendation` already expose; no change to `modiq-storage`'s own persisted schema occurs as a side effect of either phase.
- No fact is presented in either phase that the engine did not already produce — every newly presented value traces to an existing, already-public getter named in §3.
- `modiq-runtime`, `modiq-rules`, `modiq-collection`, `modiq-versioning`, `modiq-engine`, `modiq-report`, `modiq-storage`, `modiq-knowledge`, `apps/console`, and `apps/sandbox` remain untouched by every phase of this Plan.
- `crates/modiq-cli/src/commands/history.rs` and `crates/modiq-cli/src/app.rs` remain untouched by every phase of this Plan.
- The `assess.rs`/`retrieve.rs` `format_report` duplication is not resolved by either phase; each function is extended independently, per its own type and scope.

---

## 10. Verification Gates

Before progressing past each phase:

- **After Phase 1:** `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` all clean; every new `assess.rs` assertion (§7, §8) passes; none of `assess.rs`'s own four existing tests is modified; `retrieve.rs` shows zero diff.
- **After Phase 2:** `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` all clean; every new `retrieve.rs` assertion (§7, §8) passes; neither of `retrieve.rs`'s own two existing tests is modified; `assess.rs` shows no diff beyond what Phase 1 already introduced.
- **After Phase 3:** full root workspace and `apps/sandbox/src-tauri` both reverified clean and unaffected; `git diff --stat` (or equivalent) confirms only the two files named in §4 were touched across the entire implementation.

No phase begins before the prior phase's gate passes in full.

---

## 11. Repository Integrity Checks

Performed at Phase 3, as part of Final Reverification, not assumed true throughout:

- Every file not named in §4 shows zero diff — in particular `crates/modiq-cli/src/app.rs`, `crates/modiq-cli/src/commands/history.rs`, and every file under `modiq-runtime`, `modiq-rules`, `modiq-collection`, `modiq-versioning`, `modiq-engine`, `modiq-report`, `modiq-storage`, `modiq-knowledge`, `apps/console`, and `apps/sandbox`.
- `Finding`, `Evidence`, `Recommendation`, `RecommendationStep`, `PersistedEvidence`, `PersistedFinding`, `PersistedRecommendation`, and `PersistedRecommendationStep`'s own public getter signatures are unchanged — no new field, method, or variant was added to any Runtime or Storage type to accommodate presentation (none is needed; every field in scope already exists, per §3).
- No new Cargo dependency edge is introduced — both phases depend only on types already imported by `assess.rs`/`retrieve.rs` today (`modiq-runtime`'s and `modiq-storage`'s own already-public types).
- No new CLI command, flag, or argument exists in `app.rs`'s own dispatch — confirmed by direct inspection, not assumed.
- No Governance Register item, ADR, or `GOVERNANCE.md`/`RuleEngine.md` text is modified by this implementation.

---

## 12. Completion Definition

C3's implementation is complete when all of the following hold simultaneously:

- Running `modiq-cli assess` against a report containing a Finding with a non-default `mod_health_dimension` or `status`, an Evidence item with a non-`None` `location`/`label`/`source`/`content`, or a Recommendation with a non-empty `repair_steps`, shows that content in the command's own text output — distinguishable from the fields already printed before this implementation — sourced with no fact introduced by `modiq-cli` itself.
- Running `modiq-cli retrieve` against a stored report containing an Evidence item with a non-`None` `location`, or a Recommendation with a non-empty `repair_steps`, shows that content identically.
- A report or stored report with no non-default values for any newly presented field presents with no error, placeholder, or synthesized content — a valid, expected outcome, not a gap to fill.
- Every field `assess.rs` and `retrieve.rs` already presented before this implementation continues to present identically in content and meaning.
- All verification gates (§10) pass clean, and all repository integrity checks (§11) hold.
- No item named in `IMPLEMENTATION_AUTHORIZATION_C3_CLI_FIELD_PARITY.md` §5 (Explicit Exclusions) or this Plan's own §5 is touched, added, or implied.

**Expected next artifact:** a C3 Implementation Report, following this repository's own standard reporting convention, once the phases in §7 are actually carried out. Not produced by this document.

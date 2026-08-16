# Implementation Authorization — C3: `modiq-cli` Field Parity

| Property | Value |
|---|---|
| **Document** | IMPLEMENTATION_AUTHORIZATION_C3_CLI_FIELD_PARITY.md |
| **Project** | modIQ |
| **Purpose** | Convert `PROCESS_DETERMINATION_C3_CLI_FIELD_PARITY.md`'s own conclusion into a bounded engineering envelope. Authorizes implementation in principle — participating files, exclusions, invariants, verification gates, and completion criteria — and nothing beyond that. |
| **Origin** | `PROCESS_DETERMINATION_C3_CLI_FIELD_PARITY.md` (`a595019`), whose own conclusion is that C3 requires no Architecture Evaluation or Architectural Resolution and that an Implementation Authorization is the next required artifact. `CAPABILITY_DEFINITION_C3_CLI_FIELD_PARITY.md` (`31de26e`) and `CAPABILITY_PORTFOLIO_ASSESSMENT.md` (`ca11328`, classifying C3 Category A) — both treated as fixed, unreopened. `GOVERNANCE.md`'s CLI and Storage Crate Boundary Rules are the governing architecture. `C1_IMPLEMENTATION_AUTHORIZATION.md` (Sprint 23's own procedural lineage) is the directly on-point structural precedent this document follows, adapted for a single-crate, per-command scope C1 did not itself have. |
| **Status** | **Approved. Authorizes C3 Implementation Planning within the scope defined below. No Rust code, format string, or output layout has been produced in preparing this document.** |

---

## 1. Authorization Decision

**Authorized with Scope Constraint.**

This determination is not newly reasoned here — it is `PROCESS_DETERMINATION_C3_CLI_FIELD_PARITY.md`'s own conclusion, converted into binding form. Nothing in this document revisits how that conclusion was reached, reweighs Sprint 21's/C2's precedent against Sprint 23's/C1's, or reopens the per-command scope correction `CAPABILITY_DEFINITION_C3_CLI_FIELD_PARITY.md`'s own Repository Validation Review and Reconciliation already performed.

This document's own organization follows `C1_IMPLEMENTATION_AUTHORIZATION.md`'s condensed shape, the closest structural precedent for a mechanical, single-layer, no-new-architectural-question capability — adapted where C3's own facts differ: a single Rust crate rather than a Rust crate (`console`) plus a TypeScript frontend (`apps/console`), two commands with two different scopes rather than one, and a Storage-derived boundary (§2, §5) that C1 had no equivalent of.

---

## 2. Architectural Basis

Derived exclusively from already-Adopted determinations; nothing here is newly reasoned:

- **`GOVERNANCE.md`'s CLI Crate Boundary Rule:** "Owns: user interaction, command execution, platform entry point. Must never contain business logic." This is the entire basis for authorizing both commands to format additional already-public getters as text. Its text sets no limit on how many already-public fields may be formatted, and its "no business logic" prohibition is satisfied by construction for every field in scope (§3) — each is a plain, already-fixed value read through an existing getter, never recomputed, reinterpreted, or inferred.
- **`GOVERNANCE.md`'s Storage Crate Boundary Rule:** Storage's own persisted representation is "populated only from `AssessmentReport`'s already-public API," and must never "require any change to `AssessmentService`'s public entry points." This is the entire basis for `retrieve.rs`'s own narrower scope (§3) — `retrieve.rs` may present only what `PersistedEvidence`/`PersistedFinding`/`PersistedRecommendation` (`crates/modiq-storage/src/storage/persisted_report.rs`) already carry; it may not be used as a reason to extend `modiq-storage`'s own persisted schema, which this document does not authorize (§5).
- **ADR-0007 (Runtime Entity Design Pattern).** Every field in scope is a plain value object, populated once at construction, on an already-infallibly-constructed Runtime entity (`Finding`, `Evidence`, `Recommendation`) or, for `Recommendation::repair_steps()`, on the value object it returns — `RecommendationStep` carries no identity of its own and is explicitly not one of ADR-0007's four identity-bearing entities (`crates/modiq-runtime/src/assessment/recommendation_step.rs`). This authorization presents that value; it does not reinterpret, recompute, or infer anything about it.
- **`RuleEngine.md`'s Explainable principle** — "Every conclusion should be understandable," and, of Recommendations specifically, "Every Recommendation should be traceable." Satisfied by construction for every field in scope — each is already the direct output of an existing Rule or Collector, unchanged by how `modiq-cli` chooses to print it.
- **The `{:?}` Debug-format convention already in use in both target functions** — `assess.rs`'s and `retrieve.rs`'s own `format_report` already print `category()` and `severity()` this way (confirmed directly this session: `crates/modiq-cli/src/commands/assess.rs` lines 73–87, `retrieve.rs` lines 35–49). This is the entire basis for presenting `mod_health_dimension()`, `status()`, and each `RecommendationStep::kind()` the same way — an already-exercised practice in the exact functions this authorization extends, not a new formatting idiom.
- **`Evidence`'s and `PersistedEvidence`'s optional-field pattern** (`Option<&str>` for `location`, `label`, `source`, `content`) — confirmed directly against `crates/modiq-runtime/src/assessment/evidence.rs` and `crates/modiq-storage/src/storage/persisted_report.rs`. The presence-conditional rendering this requires is not a new mechanism; it is the same shape `apps/console`'s `Reviewing.tsx` already applies to the identical fields, applied here to `modiq-cli`'s own text output instead.

---

## 3. Authorized Scope

Implementation Planning is authorized to scope work that:

- Extends `AssessCommand::format_report` (`crates/modiq-cli/src/commands/assess.rs`) so that, in addition to the fields it prints today, it also presents:
  - `Finding::mod_health_dimension()` and `Finding::status()`, alongside the existing `severity()`/`title()`/`summary()` line, via the same `{:?}` convention `severity()` already uses.
  - `Evidence::location()`, `Evidence::label()`, `Evidence::source()`, and `Evidence::content()`, each rendered only when `Some` — an absent value is a valid, expected outcome, not a gap to fill.
  - `Recommendation::repair_steps()` — for each `RecommendationStep`, its `kind()` (via the same `{:?}` convention) and `instruction()` — an empty `Vec` is a valid, expected outcome, not a gap to fill.
- Extends `RetrieveCommand::format_report` (`crates/modiq-cli/src/commands/retrieve.rs`) so that, in addition to the fields it prints today, it also presents:
  - `PersistedEvidence::location()`, rendered only when `Some`.
  - `PersistedRecommendation::repair_steps()` — for each `PersistedRecommendationStep`, its `kind()` and `instruction()`.
  - No other field — `PersistedFinding` carries no `mod_health_dimension` or equivalent, and `PersistedEvidence` carries no `label`/`source`/`content`; `retrieve.rs`'s own scope stops exactly where `modiq-storage`'s current persisted mirror stops (§2).

This authorization is conditioned on all of the following:

- No field is presented in either function that is not already exposed through an existing, public getter on the type each command actually reads (`modiq-runtime` types for `assess.rs`; `modiq-storage`'s `Persisted*` types for `retrieve.rs`).
- No new CLI command, flag, or argument — the existing `assess`/`retrieve` invocation shape (`crates/modiq-cli/src/app.rs`) is unchanged.
- The fields `assess.rs` and `retrieve.rs` already print today (`category()`/`description()`, `severity()`/`title()`/`summary()`, `action()`) remain unchanged in meaning and content — supplemented, never replaced.
- `assess.rs`'s and `retrieve.rs`'s own `format_report` duplication is not resolved as part of this work — each function is extended independently, per its own type and scope; a shared abstraction is not authorized (§5).

No responsibility outside this list is authorized by this document.

---

## 4. Participating Files

**Participating:**

- `crates/modiq-cli/src/commands/assess.rs` — `format_report`'s extension, full field parity per §3.
- `crates/modiq-cli/src/commands/retrieve.rs` — `format_report`'s extension, bounded field parity per §3.
- Each file's own existing `#[cfg(test)]` module — extended with new real-fixture assertions (§7), not replaced.

**Not participating — confirmed unchanged by this Authorization:**

- `modiq-runtime`, `modiq-rules`, `modiq-collection`, `modiq-versioning`, `modiq-engine`, `modiq-report`, `modiq-storage`, `modiq-knowledge` — every field this authorization presents already exists, already public, on these crates' own existing types; none requires a change of any kind.
- `crates/modiq-cli/src/commands/history.rs` — a structurally different data shape (`PatternFrequency` cross-report aggregation, confirmed directly this session), not per-Finding/Evidence/Recommendation presentation; out of scope.
- `crates/modiq-cli/src/app.rs` — command dispatch is unaffected; no new command, flag, or argument is introduced.
- `apps/console`, `apps/sandbox` — both separate workspaces, unaffected; neither shares transport or formatting code with `modiq-cli`.

---

## 5. Explicit Exclusions

**Architecturally blocked or reserved — unaffected by this document:**

- **Extending `modiq-storage`'s own `PersistedFinding`/`PersistedEvidence` schema** to carry `mod_health_dimension`, `status`, or `label`/`source`/`content` — a real, confirmed gap (`CAPABILITY_DEFINITION_C3...md`, Existing Repository Foundation), named there as a separate, deferred future candidate, not resolved or advanced by this authorization. Reaching it would mean extending Storage's persisted schema, which `ENGINEERING_RELEASE_1.9.md` already records as carrying a pre-existing, unresolved persistence-migration gap (no `#[serde(default)]`, no schema versioning) — real architectural risk this document does not evaluate or resolve.
- **`apps/sandbox` field parity** (Capability Portfolio Assessment's own C4) — a separate, real, deferred candidate with different standing from C3 (`apps/sandbox` is formally disqualified from the production interaction layer, Initiative 5 Decision 6; `modiq-cli` has no equivalent disqualification). Not combined with this work.
- **Resolving the `assess.rs`/`retrieve.rs` `format_report` duplication** — a real, observed structural fact about current source, not a defect this authorization is scoped to fix (§3).
- **Any change to `modiq-cli`'s own `history.rs` command** or `modiq-storage`'s aggregate pattern-analysis code.
- **`Confidence`, cross-mod dependency resolution, Lua Analysis, or any other Capability Portfolio Assessment candidate.**

**Outside the scope of this authorization by implementation scope — no architectural decision is required for any of these; they simply are not part of this work:**

- The exact format string, field ordering, indentation, or labeling convention for any newly presented field — reserved for Implementation Planning, provided it does not depart from the `{:?}` Debug-format and presence-conditional patterns named in §2 without a reason recorded there.
- Any Sprint Plan, phase breakdown, or testing sequencing beyond §7's own boundary-level requirement.

---

## 6. Architectural Invariants That Must Not Be Violated

- The CLI Crate Boundary Rule itself — `modiq-cli` remains user interaction, command execution, and platform entry point only; no business logic, Evidence evaluation, Finding/Recommendation generation, or Assessment mutation may be introduced anywhere in the implementation.
- The Storage Crate Boundary Rule — `retrieve.rs` must never present a field `PersistedEvidence`, `PersistedFinding`, or `PersistedRecommendation` does not already expose, and no change to `modiq-storage`'s own persisted schema may occur as a side effect of this work.
- No fact may be presented that the engine did not already produce — every presented value must trace to an existing, already-public getter (`Finding`/`Evidence`/`Recommendation`/`RecommendationStep` for `assess.rs`; the `Persisted*` equivalents for `retrieve.rs`).
- `Evidence`'s and `PersistedEvidence`'s optional fields remain optional in presentation — an absent `location`/`label`/`source`/`content` is rendered as absent, never synthesized, defaulted, or treated as an error.
- `Recommendation::repair_steps()`/`PersistedRecommendation::repair_steps()` returning an empty `Vec` is presented as "no repair steps," never as an error or placeholder.
- `Finding::severity()`/`Evidence::category()`/`Recommendation::action()`'s own existing presentation — unchanged in content and meaning; supplemented, never replaced.

---

## 7. Required Verification Gates

- `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` — both the root workspace and `apps/sandbox/src-tauri`'s own separate workspace. The underlying discipline (`cargo fmt`, `cargo check --workspace`, `cargo test --workspace`, all three, every phase, zero warnings the standing bar) is `PROJECT_HANDOFF_v1.1.md` §5's own text; the non-mutating `--check` form of the first command is this repository's established Implementation Authorization practice (`C1_IMPLEMENTATION_AUTHORIZATION.md` §7, `C2_IMPLEMENTATION_AUTHORIZATION_...md` §8), not a distinct requirement.
- `assess.rs`'s and `retrieve.rs`'s own existing real-fixture tests extended, not replaced or mocked, to cover: a Finding with a non-default `mod_health_dimension`/`status` (`assess.rs` only); an Evidence item with each of `location`/`label`/`source`/`content` present and absent; a Recommendation with a non-empty and an empty `repair_steps` — consistent with this repository's standing Real-I/O Testing Discipline.
- Direct confirmation (diff/grep, not assertion) that no field was added to either function's output without an existing, already-public getter backing it, on the type each command actually reads.
- Direct confirmation that `retrieve.rs`'s own additions are limited to `location()` and `repair_steps()` — no attempt to present `mod_health_dimension`, `status`, `label`, `source`, or `content` through `retrieve.rs`, since `modiq-storage`'s current persisted mirror carries none of them.
- Direct confirmation that no file outside §4's Participating list was modified.

---

## 8. Completion Criteria

- Running `modiq-cli assess` against a report containing a Finding with a non-default `mod_health_dimension` or `status`, an Evidence item with a non-`None` `location`/`label`/`source`/`content`, or a Recommendation with a non-empty `repair_steps`, shows that content in the command's own text output — distinguishable from the fields already printed today — sourced with no fact introduced by `modiq-cli` itself.
- Running `modiq-cli retrieve` against a stored report containing an Evidence item with a non-`None` `location`, or a Recommendation with a non-empty `repair_steps`, shows that content identically.
- A report with no non-default values for any newly presented field — an Evidence item with every optional field absent, a Recommendation with an empty `repair_steps`, a Finding with default `mod_health_dimension`/`status` — presents with no error, placeholder, or synthesized content; a valid, expected outcome, not a gap to fill.
- Every field `assess.rs` and `retrieve.rs` already present today continues to present identically in content and meaning.
- Full verification gates (§7) pass clean.
- No item named in §5 (Explicit Exclusions) is touched, added, or implied.

---

## 9. Expected Next Artifact

**C3 Implementation Plan** — translating this Authorization's scope into concrete phases (if more than one is warranted), the exact format string and field-ordering design for each function, and testing sequencing. Not produced by this document.

---

## Status

This document defines the engineering envelope for C3's implementation and authorizes Implementation Planning within it. It does not authorize implementation beyond Section 3, nor any work named in Section 5. No Rust code, format string, or output layout has been produced in preparing this document.

# Implementation Plan — C4: `apps/sandbox` Field Parity

| Property | Value |
|---|---|
| **Document** | IMPLEMENTATION_PLAN_C4_APPS_SANDBOX_FIELD_PARITY.md |
| **Project** | modIQ |
| **Origin** | `IMPLEMENTATION_AUTHORIZATION_C4_APPS_SANDBOX_FIELD_PARITY.md` (`ed4eea2`), treated as fixed, unreopened. `PROCESS_DETERMINATION_C4_APPS_SANDBOX_FIELD_PARITY.md` (`416a83d`) and `CAPABILITY_DEFINITION_C4_APPS_SANDBOX_FIELD_PARITY.md` (`a024c8f`) are fixed inputs, not restated. `IMPLEMENTATION_PLAN_C3_CLI_FIELD_PARITY.md` supplies this document's own structural precedent, adapted where C4's own facts differ (a single participating file rather than two; a serialized IPC boundary rather than plain text; a fixed, checked-in fixture that cannot produce every newly authorized field's content). |
| **Status** | Implementation Planning draft. No Rust or TypeScript code has been produced in preparing this document — every design decision below is stated descriptively, at the repository-design level, not as compile-ready source. |

---

## 1. Purpose

This document translates `IMPLEMENTATION_AUTHORIZATION_C4_APPS_SANDBOX_FIELD_PARITY.md`'s own authorized scope into concrete implementation phases: participating files, the design questions the Authorization left open, a testing strategy, verification gates, repository integrity checks, and a completion definition. It authorizes no work beyond what the Authorization's own §3 already authorizes, and does not itself perform, or begin, any implementation phase.

---

## 2. Current Repository State

Confirmed directly this session via fresh reads, not carried forward from any prior session's own account:

- **Branch:** `feature/runtime-implementation`
- **HEAD:** `ed4eea2` — `docs: Implementation Authorization for C4 (apps/sandbox Field Parity)`
- **Working tree:** Clean.
- **Ahead of `origin/feature/runtime-implementation`:** 30 commits, 0 behind.

C4's committed lifecycle to date: Capability Definition (`a024c8f`) → Process Determination (`416a83d`) → Implementation Authorization (`ed4eea2`). No Architecture Evaluation or Architectural Resolution exists for C4, per the Process Determination's own conclusion. No Implementation Plan has yet been committed — this document is that draft. No implementation source has changed since `ed4eea2`.

`apps/sandbox/src-tauri/src/lib.rs` (531 lines, 9 existing tests) and `apps/sandbox/src/App.tsx` (99 lines) were re-read in full this session. `apps/sandbox/src-tauri/fixtures/` contains exactly two fixtures: `sample-assessment-input/` (`notes.txt`, `nested/detail.txt` — no `modDesc.xml`) and `sample-archive-input.zip`. Neither carries a declared dependency, a declared `descVersion`, or any other content that would cause `Evidence::label()/source()/content()` to be populated or `Recommendation::repair_steps()` to be non-empty through the real Collector/Rule pipeline — confirmed directly by inspecting the fixture directory. This fact drives the Testing Strategy decision in §8.

---

## 3. Authorized Implementation Scope

Restated from `IMPLEMENTATION_AUTHORIZATION_C4_APPS_SANDBOX_FIELD_PARITY.md` §3, not re-derived:

- Live-path DTOs (built via `create_assessment`) gain: `Finding::mod_health_dimension()`/`status()` (both unconditional); `Evidence::location()`/`label()`/`source()`/`content()` (each rendered only when `Some`); `Recommendation::repair_steps()` (each step's `kind()`/`instruction()`, via a new serializable representation).
- Persisted-path DTOs (built via `retrieve_report`) gain only `PersistedRecommendation::repair_steps()` (each `PersistedRecommendationStep`'s `kind()`/`instruction()`, via the same representation shape) — no other field.
- `App.tsx` extension is authorized but optional (§6 below decides whether this Plan exercises that option).
- Every field already presented today remains unchanged in content and meaning. `AssessmentSummary`'s and `PersistedReportSummary`'s existing flat, parallel-list top-level shape is not restructured.

---

## 4. Participating Files

- `apps/sandbox/src-tauri/src/lib.rs` — the live-path and persisted-path DTO structs, their `From<&T>` conversions, the new per-step repair representation(s) (§6), and the existing `#[cfg(test)]` module, extended.

No other file is expected to require modification. If one does during implementation, implementation should stop and report it rather than proceed — matching `IMPLEMENTATION_PLAN_C3_CLI_FIELD_PARITY.md`'s own precedent for this exact commitment.

---

## 5. Explicit Exclusions

Restated from `IMPLEMENTATION_AUTHORIZATION_C4_APPS_SANDBOX_FIELD_PARITY.md` §5, not reopened:

- Extending `modiq-storage`'s own `PersistedFinding`/`PersistedEvidence` schema.
- Migrating `apps/sandbox`'s transport to a nested-by-Finding or any other shape; resolving GOV-008.
- Creating or amending a `GOVERNANCE.md` "Sandbox" Crate Boundary Rule section.
- Expanding `App.tsx`'s `FindingEntry.description` correction beyond minimum incidental correction, if `App.tsx` participates at all.
- Any new Tauri command, argument, or interaction mechanism.
- Any change to `apps/console` or `modiq-cli`.
- Any change to `modiq-runtime`, `modiq-storage`, `modiq-engine`, `modiq-report`, or any other `modiq-*` crate.
- Any change to `apps/sandbox/src-tauri/Cargo.toml`/`Cargo.lock`, or to any fixture file under `apps/sandbox/src-tauri/fixtures/`.
- `Confidence`, cross-mod dependency resolution, Lua Analysis, or any other Capability Portfolio Assessment candidate.

---

## 6. Design Questions the Authorization Reserved, Decided Here

The Authorization (§5) reserved two questions to Implementation Planning. This Plan decides both now, descriptively; no literal code is authored here.

### 6a. The exact type(s) for the new per-step repair representation

**Decision: two new, separate structs — `RecommendationStepEntry` (live path) and `PersistedRecommendationStepEntry` (persisted path) — each carrying exactly `kind: String, instruction: String`, following the file's own established naming convention.**

Reasoning: `lib.rs` already names every domain concept with a paired live/persisted struct — `EvidenceEntry`/`PersistedEvidenceEntry`, `FindingEntry`/`PersistedFindingEntry`, `RecommendationEntry`/`PersistedRecommendationEntry` — three-for-three, no exception, and the underlying Runtime/Storage types this Plan reads from are themselves already separate (`RecommendationStepKind`/`PersistedRecommendationStepKind`). A single shared struct fed by two different `From` impls would break this file's own unbroken pattern for no offsetting benefit. `apps/console`'s own `RecommendationStepSummary` is the correct precedent for *which fields* to carry (`kind`, `instruction`, both `String`, both via the `{:?}` Debug-format convention this file already uses for `category()`/`severity()`) — not for the type's own name, since this file's own convention is `*Entry`, not `*Summary`, established locally and independently of `apps/console`'s own naming.

### 6b. Whether this Plan exercises `App.tsx`'s optional extension

**Decision: no. `App.tsx` is not touched by this Plan.**

Reasoning: the Authorization (§3, §4) makes this strictly optional, and the Capability Definition's own Capability Success Criteria are satisfied entirely by the command's own IPC response — `App.tsx` extension adds no coverage completion depends on. Exercising it would also require deciding, in the same edit, how much of the pre-existing `description`-vs-`title`/`summary` drift to touch, widening this Plan's own surface for no requirement driving it. This is a Plan-level choice, not a re-closing of the Authorization's own optionality: a future, separate Implementation Plan (or an explicit amendment to this one) remains free to exercise the option later. Consistent with this repository's own "smallest defensible slice" discipline (Sprint 8, Sprint 9, C1, C2, C3 each named and applied this same discipline).

---

## 7. Implementation Phases

### Phase 1 — Live-Path Field Parity

**Objective.** Extend `EvidenceEntry`, `FindingEntry`, and `RecommendationEntry` (and their `From` impls) so a Finding's `mod_health_dimension()`/`status()`, an Evidence item's `location()`/`label()`/`source()`/`content()`, and a Recommendation's `repair_steps()` are each presented, without altering any field these three structs already carry.

**Participating files.** `apps/sandbox/src-tauri/src/lib.rs` only (the live-path structs, `impl From<&Finding>`/`From<&Evidence>`/`From<&Recommendation>`, the new `RecommendationStepEntry` struct and its `impl From<&RecommendationStep>`, and the file's own `#[cfg(test)]` module).

**Implementation tasks:**

- Add `mod_health_dimension: String` and `status: String` to `FindingEntry`; populate both unconditionally in `impl From<&Finding>`, via `format!("{:?}", ...)`, matching `severity`'s own existing convention in the same impl.
- Add `label: Option<String>`, `source: Option<String>`, `content: Option<String>` to `EvidenceEntry`; populate each via `.map(str::to_string)` in `impl From<&Evidence>`, matching `location`'s own existing convention in the same impl.
- Introduce `RecommendationStepEntry { kind: String, instruction: String }` with `impl From<&RecommendationStep>` (`kind` via `format!("{:?}", ...)`, `instruction` via `.to_string()`), per §6a.
- Add `repair_steps: Vec<RecommendationStepEntry>` to `RecommendationEntry`; populate via `.iter().map(RecommendationStepEntry::from).collect()` in `impl From<&Recommendation>`.
- Extend the existing test module: a case asserting `mod_health_dimension`/`status` appear (constructible via the existing real-fixture pipeline, since every Finding already carries both); a case asserting each of `location`/`label`/`source`/`content` appears when present, and a case asserting none appears when all four are absent (constructed directly, per §8 — the checked-in fixture cannot produce this content); a case asserting each `repair_steps` entry's `kind`/`instruction` appears when non-empty, and a case asserting an empty `repair_steps` produces an empty array, not an error or placeholder (constructed directly, per §8).

**Verification required.** `apps/sandbox/src-tauri`'s own `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` all clean; the root workspace's own three gates reconfirmed clean and unaffected; direct confirmation (diff/grep) that every field newly printed traces to an existing, already-public getter named in §3, and that no field already presented before this phase changed in meaning or content.

**Completion criteria.** A Finding, Evidence item, and Recommendation carrying the newly authorized content each show it via `AssessmentSummary::from`'s own output, distinguishable from the fields already presented before this phase; a report carrying none of that content presents exactly as it did before this phase.

### Phase 2 — Persisted-Path Field Parity

**Objective.** Extend `PersistedRecommendationEntry` (and its `From` impl) so a `PersistedRecommendation`'s `repair_steps()` is presented, bounded exactly to what `modiq-storage`'s current persisted mirror carries, without altering any field `PersistedEvidenceEntry`, `PersistedFindingEntry`, or `PersistedRecommendationEntry` already carry.

**Participating files.** `apps/sandbox/src-tauri/src/lib.rs` only (`PersistedRecommendationEntry`, `impl From<&PersistedRecommendation>`, the new `PersistedRecommendationStepEntry` struct and its `impl From<&PersistedRecommendationStep>`, and the file's own `#[cfg(test)]` module).

**Implementation tasks:**

- Introduce `PersistedRecommendationStepEntry { kind: String, instruction: String }` with `impl From<&PersistedRecommendationStep>`, mirroring Phase 1's own `RecommendationStepEntry` exactly in shape (per §6a).
- Add `repair_steps: Vec<PersistedRecommendationStepEntry>` to `PersistedRecommendationEntry`; populate via `.iter().map(PersistedRecommendationStepEntry::from).collect()` in `impl From<&PersistedRecommendation>`.
- Introduce no field on `PersistedEvidenceEntry` or `PersistedFindingEntry` — `PersistedEvidence`/`PersistedFinding` carry none of `label`/`source`/`content`/`mod_health_dimension`/`status`; this phase must not anticipate a future `modiq-storage` extension.
- Extend the existing test module: a case asserting a stored report's non-empty `repair_steps` appears via `retrieve_report_with_storage`, and a case asserting an empty `repair_steps` produces an empty array, not an error or placeholder — both constructed directly, per §8 (the checked-in fixture cannot produce non-empty `repair_steps` through the real Collector/Rule pipeline).

**Verification required.** Both workspaces' gates all clean; direct confirmation that the persisted-path addition is limited to `repair_steps()` — no attempt to present `mod_health_dimension`, `status`, `label`, `source`, or `content` through `retrieve_report`; direct confirmation that `EvidenceEntry`/`FindingEntry`/`RecommendationEntry` and their live-path behavior show no diff beyond what Phase 1 already introduced.

**Completion criteria.** A stored report whose Recommendation carries a non-empty `repair_steps` shows that content via `retrieve_report`'s own output identically in shape to Phase 1's own presentation of the same field; a stored report carrying an empty `repair_steps` presents exactly as it did before this phase.

### Phase 3 — Final Reverification

**Objective.** Full-repository reverification and Repository Impact confirmation only — no further implementation.

**Participating files.** None — verification-only phase.

**Verification required.** `apps/sandbox/src-tauri`'s own full `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace`; the root workspace's own three gates reconfirmed clean and unaffected; a direct `git diff --stat` (or equivalent) confirming the only file touched across both phases is `apps/sandbox/src-tauri/src/lib.rs`, named in §4 — no other file shows a diff, including `apps/sandbox/src/App.tsx`, `Cargo.toml`, `Cargo.lock`, and every fixture file.

**Completion criteria.** Every completion criterion named in `IMPLEMENTATION_AUTHORIZATION_C4_APPS_SANDBOX_FIELD_PARITY.md` §8 holds simultaneously, with all verification gates (§10 below) passing clean.

### Why this ordering, and not a single phase or a different split

C4's own scope, per the Authorization itself, already divides along exactly one line: the live path (full parity) and the persisted path (bounded parity), each reading a structurally different type hierarchy (`modiq-runtime` live types versus `modiq-storage`'s `Persisted*` mirror) — the identical boundary `IMPLEMENTATION_PLAN_C3_CLI_FIELD_PARITY.md` used to split its own two phases, here applied within a single file rather than across two files, since C4's own Participating Files list names only one. Each phase is independently verifiable and a failure in one cannot be confused with a defect in the other, since neither path's own conversion logic depends on the other's. A single combined phase was considered and rejected for the same reason C3 rejected it: it would obscure which path's own change caused a verification failure. A finer split (one phase per field, or per struct) was considered and rejected as exceeding the minimum logical phases this Plan is asked to define — Phase 1's three structs share one file, one general pattern (`{:?}` for enums, presence-conditional `Option`, per-step collection), and no dependency exists between them requiring independent verification gates.

---

## 8. Testing Strategy

- Both phases extend `lib.rs`'s existing `#[cfg(test)]` module in place — no new test file, no new test infrastructure, no mocking, matching `PROJECT_HANDOFF_v1.1.md`'s own Real-I/O Testing Discipline ("no mocking of real I/O; real, checked-in or test-constructed fixtures throughout") and this crate's own existing convention.
- **`mod_health_dimension`/`status` are tested through the existing real pipeline** (`create_assessment_with_storage` against the checked-in `sample-assessment-input` fixture) — every Finding this fixture already produces already carries both fields (they are non-optional), so no new fixture content is needed.
- **`label`/`source`/`content` (Evidence) and `repair_steps` (Recommendation), on the live path, are tested by constructing a real `Assessment` directly through its own existing public lifecycle methods** (as `crates/modiq-cli`'s own C3 tests already do for the analogous case) — real `Evidence`/`Finding`/`Recommendation`/`RecommendationStep` values built through their own public constructors, added to a real `Assessment`, then a real `AssessmentReport` generated via `AssessmentReport::generate` — and calling `AssessmentSummary::from(&report)` directly. This is a deliberate, reasoned departure from routing every new-field test through the `create_assessment`/`create_assessment_with_storage` Tauri-command entry point specifically: that entry point is hardwired to the fixed `FIXTURE_ASSESSMENT_INPUT` fixture (confirmed this session to contain no `modDesc.xml` and therefore no path to non-`None` `label`/`source`/`content` or non-empty `repair_steps` through the real Collector/Rule pipeline), and modifying fixture content is outside this Plan's own Participating Files (§4) and this Authorization's own scope. Every value in this test path is real and public-constructor-built — nothing is mocked — consistent with the Real-I/O Testing Discipline's own actual requirement, and directly precedented by `IMPLEMENTATION_PLAN_C3_CLI_FIELD_PARITY.md`'s own equivalent tests (`format_report_shows_an_evidence_items_location_label_source_and_content_when_present`, etc.), which likewise construct real domain values directly rather than routing every assertion through the CLI's own real-directory pipeline.
- **`repair_steps` on the persisted path is tested through the real Storage round trip** — a real, test-constructed `AssessmentReport` (built the same way as the live-path case above) is passed to `retrieve_report_with_storage`'s own existing sibling, storage construction (`ReportStore::new(...).store(&report)`), against a real, hermetic temporary directory (mirroring this file's own existing `create_assessment_with_storage_stores_a_retrievable_report` test), then retrieved via `retrieve_report_with_storage` itself — literally exercising the same function the real `retrieve_report` command calls, satisfying the Authorization's own "exercised through... `retrieve_report`" requirement (§7) in its most literal sense, without requiring any fixture file change, since `ReportStore::store` accepts any real `AssessmentReport` directly.
- Each newly authorized field requires both a present/non-empty case and an absent/empty case, so that both halves of the Authorization's own Completion Criteria (§8: "shows that content" and "presents with no error, placeholder, or synthesized content") are directly exercised, not assumed.
- No existing test in `lib.rs` is deleted, weakened, or has its own assertion altered at any phase — test count grows only by net-new assertions.
- Phase 3 authors no new test; it re-runs every existing and newly added assertion unmodified, alongside full-repository and root-workspace reverification.

---

## 9. Repository Boundaries

Restated from `IMPLEMENTATION_AUTHORIZATION_C4_APPS_SANDBOX_FIELD_PARITY.md` §6, as concrete implementation constraints, not re-derived:

- `apps/sandbox` remains a thin snapshot/transport layer only — neither phase introduces business logic, Evidence evaluation, Finding/Recommendation generation, or Assessment mutation.
- The Storage Crate Boundary Rule — Phase 2 presents only what `PersistedRecommendation` already exposes; no change to `modiq-storage`'s own persisted schema occurs as a side effect of either phase.
- No fact is presented in either phase that the engine or Storage did not already produce — every newly presented value traces to an existing, already-public getter named in §3.
- `modiq-runtime`, `modiq-storage`, `modiq-engine`, `modiq-report`, `apps/console`, `modiq-cli`, `GOVERNANCE.md`, `FrontendArchitecture.md`, every ADR, `apps/sandbox/src-tauri/Cargo.toml`/`Cargo.lock`, and every fixture file remain untouched by every phase of this Plan.
- `apps/sandbox/src-tauri`'s own `create_assessment`/`retrieve_report` command signatures, `FIXTURE_ASSESSMENT_INPUT`, and `STORAGE_ROOT` constants remain untouched.
- `AssessmentSummary`'s and `PersistedReportSummary`'s own existing flat, parallel-list top-level shape is not restructured by either phase.
- `apps/sandbox/src/App.tsx` is not touched by this Plan (§6b).

---

## 10. Verification Gates

Before progressing past each phase:

- **After Phase 1:** `apps/sandbox/src-tauri`'s own `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` all clean; every new live-path assertion (§7, §8) passes; none of the file's own nine existing tests is modified; the persisted-path structs show no diff beyond what already exists.
- **After Phase 2:** the same three gates all clean; every new persisted-path assertion (§7, §8) passes; the live-path structs and their Phase 1 additions show no diff beyond what Phase 1 already introduced.
- **After Phase 3:** both `apps/sandbox/src-tauri`'s own workspace and the root workspace reverified clean and unaffected; `git diff --stat` (or equivalent) confirms only `apps/sandbox/src-tauri/src/lib.rs` was touched across the entire implementation.

No phase begins before the prior phase's gate passes in full.

---

## 11. Repository Integrity Checks

Performed at Phase 3, as part of Final Reverification, not assumed true throughout:

- Every file not named in §4 shows zero diff — in particular `apps/sandbox/src/App.tsx`, `apps/sandbox/src-tauri/Cargo.toml`, `Cargo.lock`, every fixture under `apps/sandbox/src-tauri/fixtures/`, and every file under `modiq-runtime`, `modiq-storage`, `modiq-engine`, `modiq-report`, `apps/console`, `modiq-cli`, `docs/architecture/`, `docs/adrs/`, and `docs/governance/`.
- `Finding`, `Evidence`, `Recommendation`, `RecommendationStep`, `PersistedRecommendation`, and `PersistedRecommendationStep`'s own public getter signatures are unchanged — no new field, method, or variant was added to any Runtime or Storage type to accommodate presentation (none is needed; every field in scope already exists, per §3).
- No new Cargo dependency edge is introduced — both phases depend only on types already imported by `lib.rs` today (`modiq-runtime`'s and `modiq-storage`'s own already-public types).
- No new Tauri command, argument, or dispatch entry exists — confirmed by direct inspection, not assumed.
- `AssessmentSummary`'s and `PersistedReportSummary`'s own top-level field lists remain flat and parallel — no Finding-scoped nesting was introduced.
- No Governance Register item, ADR, or `GOVERNANCE.md`/`FrontendArchitecture.md` text is modified by this implementation.

---

## 12. Completion Definition

C4's implementation is complete when all of the following hold simultaneously:

- Running `create_assessment` (via `AssessmentSummary::from` on a report carrying the relevant content) against a Finding with `mod_health_dimension`/`status`, an Evidence item with a non-`None` `location`/`label`/`source`/`content`, or a Recommendation with a non-empty `repair_steps`, shows that content in the command's own IPC response — distinguishable from the fields already presented before this implementation.
- Running `retrieve_report` against a stored report containing a Recommendation with a non-empty `repair_steps` shows that content identically; no attempt to present `mod_health_dimension`, `status`, `label`, `source`, or `content` through this path.
- A report or stored report with no non-default values for any newly authorized field presents with no error, placeholder, or synthesized content — a valid, expected outcome, not a gap to fill.
- Every field `create_assessment` and `retrieve_report` already presented before this implementation continues to present identically in content and meaning.
- `AssessmentSummary`'s and `PersistedReportSummary`'s own existing flat, parallel-list shape is unchanged.
- All verification gates (§10) pass clean, and all repository integrity checks (§11) hold.
- No item named in `IMPLEMENTATION_AUTHORIZATION_C4_APPS_SANDBOX_FIELD_PARITY.md` §5 or this Plan's own §5 is touched, added, or implied.
- `App.tsx` is untouched (§6b) — this does not block completion, per the Authorization's own §8.

**Expected next artifact:** a C4 Implementation Report, following this repository's own standard reporting convention, once the phases in §7 are actually carried out. Not produced by this document.

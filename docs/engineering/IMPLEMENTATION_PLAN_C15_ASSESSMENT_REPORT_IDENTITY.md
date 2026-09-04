# Implementation Plan — C15: Assessment & Report Subject Identity

| Property | Value |
|---|---|
| **Document** | IMPLEMENTATION_PLAN_C15_ASSESSMENT_REPORT_IDENTITY.md |
| **Origin** | `IMPLEMENTATION_AUTHORIZATION_C15_ASSESSMENT_REPORT_IDENTITY.md`, treated as fixed. |
| **Status** | Implementation Planning. No code produced in preparing this document. |

## 1. Strategy

This is a single, uniform, mechanical change (add one fallible-constructed field to a previously-zero-field Runtime entity) whose blast radius is entirely compiler-enumerable: every call site that currently writes the bare literal `AssessmentSubject` will fail to compile once the type gains a field, with the compiler naming its exact file and line. The plan is therefore: make the core type change first (Step 1), then fix every compiler-reported call site to green (Step 2), rather than pre-enumerating every call site by hand and risking missing one — the compiler is the more reliable inventory here.

## 2. Step 1 — Core Type Change

- **`crates/modiq-runtime/src/assessment/subject_error.rs`** (new): `AssessmentSubjectError` enum, one variant `EmptyLabel`, mirroring `EvidenceError`'s exact derive list and `#[error(...)]` message style.
- **`crates/modiq-runtime/src/assessment/subject.rs`**: `AssessmentSubject` gains `label: String`; `new(label: impl Into<String>) -> Result<Self, AssessmentSubjectError>` rejects `label.trim().is_empty()`; `label(&self) -> &str` getter added. Existing doc comment ("It contains identifying information about the target") is now finally accurate — left as-is, since it already correctly described the intended, now-realized purpose.
- **`crates/modiq-runtime/src/assessment/mod.rs`**: add `pub mod subject_error;` and `pub use subject_error::AssessmentSubjectError;`.
- **`crates/modiq-report/src/report/assessment_report.rs`**: `AssessmentReport` gains `subject_label: String`, populated in `generate` from `assessment.subject().label().to_string()`; getter `subject_label(&self) -> &str` added.

**Gate:** `cargo check -p modiq-runtime -p modiq-report` — expected to fail everywhere `AssessmentSubject` is still constructed bare; this failure list is Step 2's own work list, not a defect.

## 3. Step 2 — Fix Every Compiler-Reported Call Site

For each error `cargo check --workspace` (and, separately, `apps/sandbox/src-tauri`'s own workspace) reports:

- **Production call sites** (real, not test code) — use the string already locally available:
  - `crates/modiq-cli/src/commands/assess.rs`, `AssessCommand::run` — `AssessmentSubject::new(path)`, propagating any error (unreachable in practice, since an empty `path` already fails `AssessmentInput`'s own validation downstream, but handled explicitly, not unwrapped, consistent with this file's existing no-panics discipline) into the function's existing `(String, ExitCode)` return shape.
  - `apps/console/src-tauri/src/assessment.rs`, `submit_assessment_from_path` — `AssessmentSubject::new(input_path)`, its error mapped to `String` via the same `.map_err(|error| error.to_string())` pattern already used for the execution error.
  - `apps/sandbox/src-tauri/src/lib.rs`, `create_assessment_with_storage` — `AssessmentSubject::new(FIXTURE_ASSESSMENT_INPUT)`, using its own already-existing fixture constant; `.expect(...)` is acceptable here only because this exact string is already a compile-time constant proven non-empty by the file's own existing fixture, not because errors are being silently discarded elsewhere.
- **Test call sites** (`modiq-runtime/assessment.rs`, `modiq-report/assessment_report.rs`, `modiq-engine/assessment_service.rs`, `modiq-engine/tests/end_to_end.rs`, `modiq-cli/assess.rs`/`retrieve.rs`/`history.rs`, `apps/console/assessment.rs`, `apps/sandbox/lib.rs`, each own `#[cfg(test)]` module) — `AssessmentSubject::new("test-subject").unwrap()` (a single, uniform, non-meaningful literal, exactly as this codebase's own tests already use uniform literals like `"sample evidence"` for fixture content that carries no semantic weight beyond non-emptiness).

**Gate, per file:** re-run `cargo check` scoped to that crate after each file's own fix, before moving to the next — the same per-phase validation discipline this project's own Engineering Methodology Observations already recommend (Sprint 8's own observation).

## 4. Step 3 — New/Extended Tests

- `subject.rs`: two new tests — `new_rejects_an_empty_label`, `new_rejects_a_whitespace_only_label`, `new_accepts_a_real_label` (or equivalent), mirroring `evidence.rs`'s own existing test shape for `EmptyDescription`.
- `assessment_report.rs`: one new test confirming `subject_label()` returns exactly the label the underlying `Assessment` was constructed with.
- No existing test's *assertions* change — only the fixture construction line each test already has, which now goes through the fallible constructor instead of a bare literal.

## 5. Step 4 — Full Verification

1. `cargo fmt --check`
2. `cargo check --workspace`
3. `cargo test --workspace`
4. `apps/sandbox/src-tauri`: `cargo check`, `cargo test` (separate workspace)
5. `apps/console`: `tsc && vite build` (expected unaffected — confirms no accidental TypeScript touch)
6. `git diff --stat` confirming only Authorization §3's participating files changed
7. Real-I/O sanity: run `modiq-cli assess` against a real path and confirm the command still succeeds and produces identical output to before, aside from nothing user-visible changing (this capability adds no CLI-visible output field, by design — `format_report` is not touched)

## 6. Escalation Conditions

**ESCALATION — NOT AUTHORIZED** if implementation discovers: a call site in `modiq-storage` actually requiring `AssessmentSubject` construction (would mean §3's "not participating" list was wrong); a need to change `AssessmentService`'s public signature; a need to touch any DTO, transport, or UI file; a need for validation beyond empty/whitespace-only rejection.

## 7. Completion Criteria

Identical to `IMPLEMENTATION_AUTHORIZATION_C15...md` §6.

---

**Repository confirmation:** no source file created or modified in preparing this Plan.

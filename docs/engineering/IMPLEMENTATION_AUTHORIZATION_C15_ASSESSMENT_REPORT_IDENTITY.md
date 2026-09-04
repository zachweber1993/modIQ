# Implementation Authorization — C15: Assessment & Report Subject Identity

| Property | Value |
|---|---|
| **Document** | IMPLEMENTATION_AUTHORIZATION_C15_ASSESSMENT_REPORT_IDENTITY.md |
| **Project** | modIQ |
| **Purpose** | Convert `ARCHITECTURAL_RESOLUTION_C15_ASSESSMENT_REPORT_IDENTITY.md`'s §5(E) adopted pattern into a bounded engineering envelope. |
| **Origin** | `ARCHITECTURAL_RESOLUTION_C15_ASSESSMENT_REPORT_IDENTITY.md` (Adopted), whose §8 names Implementation Authorization as the next artifact's own responsibility. `CAPABILITY_DEFINITION_C15...md`'s Non-Scope (§7) is treated as fixed and unreopened. |
| **Status** | **Approved. Authorizes implementation within the scope defined below.** |

---

## 1. Authorized Scope

`AssessmentSubject` (`crates/modiq-runtime`) gains one field, `label: String`, populated only from information already available to the caller at Assessment-construction time — the exact value already passed as `input` to `AssessmentService::execute_from_assessment_input`, or, for direct `execute`/`Assessment::new` callers (chiefly tests), an equivalent already-available caller-supplied string. Construction is fallible, mirroring `Evidence::new`'s established validation pattern (ADR-0007): empty or whitespace-only labels are rejected via a new `AssessmentSubjectError`.

`AssessmentReport` (`crates/modiq-report`) gains one getter, `subject_label() -> &str`, reading `Assessment::subject().label()` — the same "already-public-getter" pattern it already uses for `evidence()`/`findings()`/`recommendations()`.

Every real and test call site across the workspace that currently constructs `AssessmentSubject` as a bare unit-struct literal is updated to call the new fallible constructor instead, using the same string already available at that call site (production callers) or a clearly-fixture-only literal (test callers).

## 2. Design Fixed by This Authorization

- **Field name:** `label: String`. **Constructor:** `AssessmentSubject::new(label: impl Into<String>) -> Result<Self, AssessmentSubjectError>`. **Getter:** `label(&self) -> &str`.
- **Error type:** `AssessmentSubjectError` (new file, `subject_error.rs`), mirroring `EvidenceError`'s exact shape (`#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]`), one variant, `EmptyLabel`.
- **`AssessmentReport`'s new field:** `subject_label: String`, populated in `generate` via `assessment.subject().label().to_string()`.
- **No change to `AssessmentService::execute`/`execute_from_assessment_input`'s own public signature.** Both continue to accept `subject: AssessmentSubject` exactly as today; only `AssessmentSubject`'s own internal shape changes.
- **No change to `apps/console`'s Tauri command, `ReportSummary`/any DTO, `engine/types.ts`, or any UI file.** `AssessmentReport::subject_label()` exists on the Rust type; nothing reads it into the console's transport.

## 3. Participating Files

**Required participation** (mechanical call-site updates, compiler-enumerated and fixed to green):
- `crates/modiq-runtime/src/assessment/subject.rs`, `subject_error.rs` (new), `mod.rs` (export), `assessment.rs` (test call sites only — no production code change, since `Assessment` never constructs its own `AssessmentSubject`).
- `crates/modiq-report/src/report/assessment_report.rs` (new field/getter + test call sites).
- `crates/modiq-engine/src/engine/assessment_service.rs` (test call sites only), `tests/end_to_end.rs`.
- `crates/modiq-cli/src/commands/assess.rs` (one production call site + test call sites), `retrieve.rs`, `history.rs` (test call sites only).
- `apps/console/src-tauri/src/assessment.rs` (one production call site + test call sites).
- `apps/sandbox/src-tauri/src/lib.rs` (one production call site, `create_assessment_with_storage`, using its own existing `FIXTURE_ASSESSMENT_INPUT` constant + test call sites).

**Not participating — confirmed unchanged:**
- `modiq-storage` (`ReportKey`, `PersistedAssessmentReport`, `ReportStore`, `history_analysis`) — explicitly out of scope (`CAPABILITY_DEFINITION_C15...md` §7; `ARCHITECTURAL_RESOLUTION_C15...md` §8).
- `apps/console/src/engine/{index,types}.ts`, `apps/console/src/workspace/{Overview,Reviewing}.tsx`, `apps/console/src/regions/Workspace.tsx` — no DTO, transport, or UI change.
- `modiq-collection`, `modiq-rules`, `modiq-knowledge`, `modiq-versioning` — untouched.
- `AssessmentService`'s public method signatures — unchanged.
- Every ADR, `GOVERNANCE.md`, and every prior C15 lifecycle document — unchanged.

## 4. Architectural Invariants That Must Not Be Violated

- `AssessmentSubject`'s content is fixed at construction, never mutated afterward — no method added beyond `new`/`label`.
- `AssessmentReport` continues to only reflect `Assessment` state already exposed via a public getter — `subject_label()` is a direct copy of `Assessment::subject().label()`, nothing computed or inferred.
- No `modiq-storage` file changes; a persisted report's own schema is untouched.
- `AssessmentService`'s two public methods keep their exact current signatures.

## 5. Required Verification Gates

`cargo fmt --check`; `cargo check --workspace`; `cargo test --workspace` (root workspace); `cargo check`/`cargo test` in `apps/sandbox/src-tauri`'s own separate workspace; `apps/console`'s `tsc && vite build` (expected unaffected — no TypeScript file touched, run as a boundary-confirmation gate); `git diff --stat` confirming only §3's participating files changed.

## 6. Completion Criteria

`AssessmentSubject::new` rejects empty/whitespace-only labels and accepts everything else; `AssessmentReport::subject_label()` returns the exact label a real Assessment was constructed with; every real production call site (`modiq-cli assess`, `apps/console`, `apps/sandbox`) passes its own already-known input string; the full workspace (both Cargo workspaces) builds and tests clean; no file outside §3 changed.

---

## Status

Authorized. No Implementation Plan phase breakdown is required beyond this Authorization's own scope — the change is uniform in kind across every participating file (construct via the new fallible constructor instead of a bare literal); this is stated in the accompanying Implementation Plan as one continuous, compiler-guided pass rather than artificially split phases.

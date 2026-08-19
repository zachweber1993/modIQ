# Implementation Authorization — C4: `apps/sandbox` Field Parity

| Property | Value |
|---|---|
| **Document** | IMPLEMENTATION_AUTHORIZATION_C4_APPS_SANDBOX_FIELD_PARITY.md |
| **Project** | modIQ |
| **Purpose** | Convert `PROCESS_DETERMINATION_C4_APPS_SANDBOX_FIELD_PARITY.md`'s own conclusion into a bounded engineering envelope. Authorizes implementation in principle — participating files, exclusions, invariants, verification gates, and completion criteria — and nothing beyond that. |
| **Origin** | `PROCESS_DETERMINATION_C4_APPS_SANDBOX_FIELD_PARITY.md` (`416a83d`), whose own conclusion is that C4 requires no Architecture Evaluation or Architectural Resolution and that an Implementation Authorization is the next required artifact. `CAPABILITY_DEFINITION_C4_APPS_SANDBOX_FIELD_PARITY.md` (`a024c8f`) and `CAPABILITY_PORTFOLIO_ASSESSMENT.md` (classifying C4 Category A) — both treated as fixed, unreopened. `GOVERNANCE.md`'s Storage Crate Boundary Rule, `INITIATIVE_5_ARCHITECTURAL_RESOLUTION.md`'s Decision 6, and `FrontendArchitecture.md`'s reserved-pending-GOV-008 clause are the governing architecture. `IMPLEMENTATION_AUTHORIZATION_C3_CLI_FIELD_PARITY.md` is the directly on-point structural precedent this document follows, adapted for a live/persisted asymmetry that differs from C3's own in one respect (§2) and a serialized-IPC transport that `modiq-cli`'s plain-text output did not have to address. |
| **Status** | **Approved. Authorizes C4 Implementation Planning within the scope defined below. No Rust code, TypeScript code, or DTO shape has been produced in preparing this document.** |

---

## 1. Authorization Decision

**Authorized with Scope Constraint.**

This determination is not newly reasoned here — it is `PROCESS_DETERMINATION_C4_APPS_SANDBOX_FIELD_PARITY.md`'s own conclusion, converted into binding form. Nothing in this document revisits how that conclusion was reached, reweighs C2's own precedent against C1's/C3's, or reopens any of the six questions the Process Determination already investigated and resolved.

This document's own organization follows `IMPLEMENTATION_AUTHORIZATION_C3_CLI_FIELD_PARITY.md`'s condensed shape, the closest structural precedent for a mechanical, no-new-architectural-question capability — adapted where C4's own facts differ from C3's: `apps/sandbox` transports over a serialized Tauri IPC boundary (not plain text), meaning presenting a per-step repair structure requires a new serializable representation where C3 needed only a format string; and `apps/sandbox`'s own governance standing (Process Determination, Question 3) differs categorically from `modiq-cli`'s, not merely in degree.

---

## 2. Architectural Basis

Derived exclusively from already-Adopted determinations; nothing here is newly reasoned:

- **`GOVERNANCE.md`'s Storage Crate Boundary Rule:** Storage's own persisted representation is "populated only from `AssessmentReport`'s already-public API." This is the entire basis for `PersistedReportSummary`'s own narrower scope (§3) — it may present only what `PersistedEvidence`/`PersistedFinding`/`PersistedRecommendation` (`crates/modiq-storage/src/storage/persisted_report.rs`) already carry; it may not be used as a reason to extend `modiq-storage`'s own persisted schema, which this document does not authorize (§5).
- **`INITIATIVE_5_ARCHITECTURAL_RESOLUTION.md`, Decision 6 (Adopted) and the Process Determination's own Question 3 finding:** `apps/sandbox`'s absence from `GOVERNANCE.md`'s Crate Boundary Rules is an already-Adopted architectural fact (a direct consequence of its deliberate non-membership in the root Cargo workspace), not an unclosed prerequisite. No `GOVERNANCE.md` amendment is a precondition of this authorization, and none is authorized by it.
- **`FrontendArchitecture.md`'s reserved-pending-GOV-008 clause:** "The concrete payload shape of the request/response contract between the consumer and the engine" is explicitly, textually reserved. `AssessmentSummary`'s and `PersistedReportSummary`'s existing flat, parallel-list shape is therefore the governing shape this authorization builds within — not a design choice this document makes, but a boundary it inherits from the Process Determination's own Question 2 finding.
- **ADR-0007 (Runtime Entity Design Pattern).** Every field in scope is a plain, already-fixed value on an already-infallibly-constructed Runtime or Storage entity, or, for `repair_steps`, the value object each domain's own `RecommendationStep`/`PersistedRecommendationStep` type returns — neither carries identity of its own (`crates/modiq-runtime/src/assessment/recommendation_step.rs`, `crates/modiq-storage/src/storage/persisted_report.rs`, both confirmed directly this session). This authorization presents that value; it does not reinterpret, recompute, or infer anything about it.
- **The Transport Mirror pattern, already exercised twice.** `apps/console`'s own `RecommendationStepSummary` (`apps/console/src-tauri/src/assessment.rs`, `kind: String, instruction: String`, confirmed directly this session) is the direct, already-tested precedent for the one genuinely new element this authorization requires: a serializable representation of a per-step repair structure crossing an IPC boundary. `apps/sandbox`'s own existing `EvidenceEntry`/`FindingEntry`/`RecommendationEntry`/`Persisted*Entry` structs, each a `#[derive(serde::Serialize)]` snapshot built via `From<&T>`, are the direct, already-exercised precedent for every other field this authorization adds.
- **`Evidence`'s and `PersistedEvidence`'s optional-field pattern** (`Option<&str>` for `location`, `label`, `source`, `content` on `Evidence`; `location` only on `PersistedEvidence`) — confirmed directly against `crates/modiq-runtime/src/assessment/evidence.rs` and `crates/modiq-storage/src/storage/persisted_report.rs` this session. `apps/sandbox`'s own `EvidenceEntry` already renders `location` this way (`Option<String>`, serializing to `null` when absent); the identical mechanism extends directly to `label`/`source`/`content`.
- **`apps/sandbox`'s own existing dependency graph already reaches every type this authorization needs.** `apps/sandbox/src-tauri/Cargo.toml`, confirmed directly this session: `modiq-runtime`, `modiq-storage`, `modiq-engine`, and `modiq-report` are already dependencies. No new dependency edge is required or authorized by this document.

---

## 3. Authorized Scope

Implementation Planning is authorized to scope work that:

- Extends `apps/sandbox/src-tauri/src/lib.rs`'s live-path DTOs (the types `AssessmentSummary` is built from, via `create_assessment`) so that, in addition to the fields presented today, they also present:
  - `Finding::mod_health_dimension()` and `Finding::status()` — both non-optional; the `FindingEntry`-equivalent gains both unconditionally.
  - `Evidence::location()`, `Evidence::label()`, `Evidence::source()`, and `Evidence::content()` — each rendered only when `Some`, exactly as `EvidenceEntry` already renders `location()` today.
  - `Recommendation::repair_steps()` — for each `RecommendationStep`, its `kind()` and `instruction()`, via a new, additively-introduced serializable representation following the same two-field (`kind`, `instruction`) shape `apps/console`'s own `RecommendationStepSummary` already establishes. This authorization fixes which fields that representation carries; it does not fix its exact Rust type name, module placement, or derive list — reserved for Implementation Planning.
- Extends `apps/sandbox/src-tauri/src/lib.rs`'s persisted-path DTOs (the types `PersistedReportSummary` is built from, via `retrieve_report`) so that, in addition to the fields presented today, they also present:
  - `PersistedRecommendation::repair_steps()` — for each `PersistedRecommendationStep`, its `kind()` and `instruction()`, via the same representation shape authorized above for the live path.
  - No other field. `PersistedFinding` carries no `mod_health_dimension`/`status`; `PersistedEvidence` carries no `label`/`source`/`content` — confirmed directly against `crates/modiq-storage/src/storage/persisted_report.rs` this session. The persisted-path scope stops exactly where `modiq-storage`'s current persisted mirror stops, mirroring `IMPLEMENTATION_AUTHORIZATION_C3_CLI_FIELD_PARITY.md`'s identical bound for `modiq-cli retrieve`.
- **Optionally**, extends `apps/sandbox/src/App.tsx`'s TypeScript interfaces and rendered output to reflect the newly authorized fields above. This is bounded, additive scope, not required scope: the Capability Definition's own Capability Success Criteria are stated in terms of the command's own IPC response, not the rendered UI, and its own Repository Impact section names `App.tsx` as participating only "if the rendered UI is also extended" — a condition, not a requirement. If Implementation Planning elects to extend `App.tsx`:
  - Only the fields newly authorized above may be added to its interfaces and rendering.
  - The pre-existing `FindingEntry.description`-vs-`title`/`summary` drift (confirmed this session: `App.tsx`'s interface has declared `description: string` since commit `a11f9ec`, 2026-07-19, while the Rust struct it mirrors has carried `title`/`summary` since Sprint 22) may be corrected only as the minimum incidental correction naturally implied by touching that same interface declaration to add the newly authorized fields (e.g., replacing `description: string` with `title: string; summary: string` in the same edit that adds `modHealthDimension`/`status`) — not expanded into a broader refactor, and not a precondition of exercising this option.
  - If Implementation Planning elects not to extend `App.tsx`, the drift remains exactly as it is today, untouched, and this does not block C4's own completion (§8).

This authorization is conditioned on all of the following:

- No field is presented anywhere that is not already exposed through an existing, public getter on the type each DTO actually converts from (`modiq-runtime` types for the live path; `modiq-storage`'s `Persisted*` types for the persisted path).
- No new Tauri command, argument, or interaction mechanism — the existing `create_assessment`/`retrieve_report` invocation shape is unchanged.
- The fields already presented today (`id`, `category`, `description`, `location`, `severity`, `title`, `summary`, `action`, `status` at the top level of `PersistedReportSummary`) remain unchanged in meaning and content — supplemented, never replaced.
- `AssessmentSummary`'s and `PersistedReportSummary`'s own existing flat, parallel-list top-level shape (`evidence`, `findings`, `recommendations` as three independent collections) is not restructured into a nested-by-Finding shape or any other shape — the reserved question `PROCESS_DETERMINATION_C4...md` (Question 2) left open remains open, and this authorization neither resolves it nor requires resolving it.

No responsibility outside this list is authorized by this document.

---

## 4. Participating Files

**Participating:**

- `apps/sandbox/src-tauri/src/lib.rs` — the live-path and persisted-path DTO structs and their `From<&T>` conversions, extended per §3; the new per-step repair representation, introduced per §3; each existing `#[cfg(test)]` module extended with new real-fixture assertions (§7), not replaced.
- `apps/sandbox/src/App.tsx` — **optional**, per §3; participates only if Implementation Planning elects to extend rendering, bounded exactly as §3 states.

**Not participating — confirmed unchanged by this Authorization:**

- `modiq-runtime`, `modiq-rules`, `modiq-collection`, `modiq-versioning`, `modiq-engine`, `modiq-report`, `modiq-storage`, `modiq-knowledge` — every field this authorization presents already exists, already public, on these crates' own existing types; none requires a change of any kind.
- `apps/sandbox/src-tauri/Cargo.toml` / `Cargo.lock` — every type this authorization needs is already reachable through this crate's own existing dependency graph (§2); no new edge is authorized.
- `apps/console`, `modiq-cli` — both separate consumers, unaffected; neither shares transport or formatting code with `apps/sandbox`.
- `GOVERNANCE.md`, `FrontendArchitecture.md`, any ADR, or any Architectural Resolution — none is amended by, or as a precondition of, this authorization.
- `apps/sandbox/src-tauri`'s own `create_assessment`/`retrieve_report` command signatures, `FIXTURE_ASSESSMENT_INPUT`, and `STORAGE_ROOT` constants — command dispatch and fixture/storage wiring are unaffected.

---

## 5. Explicit Exclusions

**Architecturally blocked or reserved — unaffected by this document:**

- **Extending `modiq-storage`'s own `PersistedFinding`/`PersistedEvidence` schema** to carry `mod_health_dimension`, `status`, or `label`/`source`/`content` — a real, confirmed gap (`CAPABILITY_DEFINITION_C4...md`, `PROCESS_DETERMINATION_C4...md` Question 5), named there as a separate, deferred future candidate, not resolved or advanced by this authorization.
- **Migrating `apps/sandbox`'s transport from its existing flat, parallel-list shape to `apps/console`'s nested-by-Finding shape, or to any other shape.** GOV-008 remains Open. This authorization neither resolves it nor requires resolving it (§3).
- **Creating or amending a `GOVERNANCE.md` "Sandbox" Crate Boundary Rule section.** `PROCESS_DETERMINATION_C4...md`'s Question 3 finding concludes no such amendment is required; this document does not perform one, and none is a precondition of this authorization.
- **Expanding `App.tsx`'s `FindingEntry.description` correction beyond the minimum incidental correction §3 already bounds**, and **performing any App.tsx change at all is itself optional**, not required.
- Any new Tauri command, file-picker, drag-and-drop, or other new interaction mechanism for `apps/sandbox`.
- Any change to `apps/console` or `modiq-cli`.
- `Confidence`, cross-mod dependency resolution, Lua Analysis, and every other remaining Capability Portfolio Assessment candidate (C5–C12).

**Outside the scope of this authorization by implementation scope — no architectural decision is required for any of these; they simply are not part of this work:**

- The exact Rust type name, field ordering, module placement, or derive list for the new per-step repair representation (§3) — reserved for Implementation Planning, provided it carries exactly `kind` and `instruction` and does not depart from the presence-conditional and empty-collection patterns named in §2/§6 without a reason recorded there.
- Any Sprint Plan, phase breakdown, or testing sequencing beyond §7's own boundary-level requirement.

---

## 6. Architectural Invariants That Must Not Be Violated

- `apps/sandbox` remains a thin snapshot/transport layer only — no business logic, Evidence evaluation, Finding/Recommendation generation, or Assessment mutation may be introduced anywhere in the implementation.
- The Storage Crate Boundary Rule — the persisted-path DTOs must never present a field `PersistedEvidence`, `PersistedFinding`, or `PersistedRecommendation` does not already expose, and no change to `modiq-storage`'s own persisted schema may occur as a side effect of this work.
- No fact may be presented that the engine or Storage did not already produce — every presented value must trace to an existing, already-public getter.
- `Evidence`'s optional fields remain optional in presentation — an absent `location`/`label`/`source`/`content` serializes as absent (`null`), never synthesized, defaulted, or treated as an error.
- `Recommendation::repair_steps()`/`PersistedRecommendation::repair_steps()` returning an empty collection is presented as an empty collection, never as an error or placeholder.
- Every field already presented today — unchanged in content and meaning; supplemented, never replaced.
- `AssessmentSummary`'s and `PersistedReportSummary`'s own existing flat, parallel-list top-level shape is preserved exactly; no Finding-scoped nesting is introduced.
- `GOVERNANCE.md`, `FrontendArchitecture.md`, and every ADR remain textually unmodified by this implementation.

---

## 7. Required Verification Gates

- `apps/sandbox/src-tauri`'s own `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` — this crate's separate Cargo workspace, per its own `Cargo.toml` (§2); all clean. The underlying discipline (`cargo fmt`, `cargo check --workspace`, `cargo test --workspace`, zero warnings) is `PROJECT_HANDOFF_v1.1.md` §5's own text, applied here to `apps/sandbox`'s own workspace exactly as it is applied to the root workspace elsewhere.
- The root workspace's own `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` — confirmed unaffected (`apps/sandbox` is not a root workspace member), reconfirmed clean as a repository-integrity check, not because this work is expected to touch it.
- New tests, added to `lib.rs`'s existing `#[cfg(test)]` module, extended, not replaced or mocked, per `PROJECT_HANDOFF_v1.1.md`'s own Real-I/O Testing Discipline ("no mocking of real I/O; real, checked-in or test-constructed fixtures throughout") and this crate's own existing convention (`create_assessment_with_storage`/`retrieve_report_with_storage` against real, hermetic temporary directories) — covering: a Finding with a non-default `mod_health_dimension`/`status`; an Evidence item with each of `location`/`label`/`source`/`content` present and with all four absent; a Recommendation with a non-empty and an empty `repair_steps`, exercised through both `create_assessment` and `retrieve_report`.
- Direct confirmation (diff/grep, not assertion) that no field was added to either DTO path without an existing, already-public getter backing it.
- Direct confirmation that the persisted-path additions are limited to `repair_steps()` — no attempt to present `mod_health_dimension`, `status`, `label`, `source`, or `content` through `retrieve_report`.
- Direct confirmation that `AssessmentSummary`'s and `PersistedReportSummary`'s own top-level field lists remain flat and parallel — no nested nesting-by-Finding restructuring occurred.
- Direct confirmation that no file outside §4's Participating list was modified, and that `Cargo.toml`/`Cargo.lock` show zero diff.
- If `App.tsx` was extended: confirmation the TypeScript build succeeds and no field beyond those authorized in §3 was added or changed.

---

## 8. Completion Criteria

- Running `create_assessment` against a report containing a Finding with a non-default `mod_health_dimension` or `status`, an Evidence item with a non-`None` `location`/`label`/`source`/`content`, or a Recommendation with a non-empty `repair_steps`, shows that content in the command's own IPC response — distinguishable from the fields already transported today — sourced with no fact introduced by `apps/sandbox` itself.
- Running `retrieve_report` against a stored report containing a Recommendation with a non-empty `repair_steps` shows that content identically; no attempt to present `mod_health_dimension`, `status`, `label`, `source`, or `content` through this path.
- A report or stored report with no non-default values for any newly authorized field presents with no error, placeholder, or synthesized content — a valid, expected outcome, not a gap to fill.
- Every field `create_assessment` and `retrieve_report` already present today continues to present identically in content and meaning.
- `AssessmentSummary`'s and `PersistedReportSummary`'s own existing flat, parallel-list shape is unchanged.
- No `modiq-storage` schema change, no `GOVERNANCE.md`/`FrontendArchitecture.md`/ADR amendment, no new Tauri command, and no `apps/console`/`modiq-cli` change occurred.
- Full verification gates (§7) pass clean, in both `apps/sandbox/src-tauri`'s own workspace and the (unaffected) root workspace.
- No item named in §5 (Explicit Exclusions) is touched, added, or implied.
- `App.tsx` extension is not a condition of completion; if undertaken, it satisfies §3's own bound.

---

## 9. Expected Next Artifact

**C4 Implementation Plan** — translating this Authorization's scope into concrete phases (if more than one is warranted), the exact Rust type/module design for the new per-step repair representation, whether `App.tsx` extension is elected, and testing sequencing. Not produced by this document.

---

## Status

This document defines the engineering envelope for C4's implementation and authorizes Implementation Planning within it. It does not authorize implementation beyond Section 3, nor any work named in Section 5. No Rust code, TypeScript code, or DTO shape has been produced in preparing this document.

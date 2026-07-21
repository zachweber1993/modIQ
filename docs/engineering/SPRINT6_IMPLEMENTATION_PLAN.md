# Sprint 6 Implementation Plan

| Property | Value |
|----------|-------|
| **Document** | SPRINT6_IMPLEMENTATION_PLAN.md |
| **Project** | modIQ |
| **Purpose** | Detailed implementation plan for Sprint 6 — CLI wiring and Reporting scaffold retirement — for Chief Architect review and authorization |
| **Prepared by** | Lead Engineer, on `feature/sprint6-cli` |
| **Scope basis** | `SPRINT6_PLANNING_DOCUMENT.md`'s recommendation, as finalized in the Sprint 6 governance baseline (commit `d4e1c74`) |
| **Status** | Authorized and implemented. See Authorization Record below. |

---

## Authorization Record

Chief Architect: Approved

Date: 2026-07-21

Scope:
- CLI wiring
- CLI error handling
- Exit codes
- modiq-report refinement — retire the obsolete scaffold components of `modiq-report` while preserving `AssessmentReport` as the repository's canonical report model. Future architectural changes must be justified by implementation evidence.

Out of Scope:
- XML inspection
- Version Profiles
- Public API expansion
- Display implementations
- Additional dependencies

---

# Objectives

1. Wire the existing `modiq-cli` scaffold (`Application`, `AssessCommand`, `HelpCommand`, `VersionCommand`) to `modiq-engine`, so `modiq-cli` becomes a real, working second consumer of `AssessmentService` — a thin client, reusing the same entry point the Sandbox already uses, with no reimplementation of pipeline logic.
2. Retire `modiq-report`'s four unused scaffold types (`FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, `ReportFormatter`), per the Sprint 5 Phase 4 investigation's recommendation, now that explicit authorization for deletion is in scope for this Sprint.

XML inspection is explicitly out of scope for Sprint 6 (per the governance baseline) and is not addressed by this plan.

---

# Scope

## In Scope

- `modiq-cli`: real `Application` dispatch over three commands (`assess`, `help`, `version`), argument parsing, calling `AssessmentService::execute_from_assessment_input`, mapping `Result<AssessmentReport, AssessmentExecutionError>` to stdout/stderr text and a process exit code.
- `modiq-cli/Cargo.toml`: add workspace dependencies on `modiq-engine` and `modiq-runtime` (both already-real, in-workspace crates — not new external dependencies).
- `modiq-report`: delete `finding_summary.rs`, `recommendation_summary.rs`, `report_formatter.rs`, `traceability_report.rs`, and their `pub mod`/`pub use` lines in `report/mod.rs`.
- Tests for all of the above, following this project's real-I/O discipline (real temp-directory fixtures, no mocks — mirroring `modiq-engine`'s own `TempDir` test helper).
- Documentation sync: `CrateRoadmap.md`'s maturity notes for `modiq-cli` (currently "L1, scaffolded, not wired to `modiq-engine`") and `modiq-report` (reflecting the retirement).

## Out of Scope

- XML inspection (deferred to a future Sprint per the governance baseline).
- Any change to `AssessmentService`'s public entry points, `AssessmentInput`, `AssessmentReport`'s existing fields, or the public error model (GOV-008 remains untouched and unblocking — this Sprint reuses the existing contract, it does not evolve it).
- Adding `Display`/`Serialize` to Runtime identity/enum types (`AssessmentId`, `FindingSeverity`, `EvidenceCategory`, `AssessmentStatus`, etc.) — flagged as a real, adjacent question below, but not pre-authorized as part of this scope.
- Any GUI, interactive prompt, config file, or persistent CLI state. `AssessCommand` takes one input location and produces one report, mirroring `execute_from_assessment_input`'s own shape exactly.
- Changing `GOVERNANCE.md`'s "Reporting" Owns/Must-never entries — the investigation's own conclusion was that the *responsibility* (formatting, summarization, traceability output) is not disputed, only that these four specific, never-instantiated types are the right shape for it. No change to that entry is proposed.

---

# Crates / Files Expected to Change

| File | Change |
|---|---|
| `crates/modiq-cli/Cargo.toml` | Add `modiq-engine`, `modiq-runtime` as workspace dependencies |
| `crates/modiq-cli/src/main.rs` | Replace the placeholder `println!` with real dispatch into `Application`; map its result to `std::process::exit` |
| `crates/modiq-cli/src/app.rs` | `Application` gains a dispatch method: match the first CLI argument to `assess` / `help` / `version` / anything else (unrecognized → help text + non-zero exit) |
| `crates/modiq-cli/src/commands/assess.rs` | `AssessCommand` gains a real method that takes an input path, constructs `AssessmentSubject`/`AssessmentContext`, calls `AssessmentService::execute_from_assessment_input`, and returns a formatted result (success text or error text) — kept independently unit-testable, not printing directly, so tests don't need to capture stdout |
| `crates/modiq-cli/src/commands/help.rs` | `HelpCommand` gains a method returning static usage text |
| `crates/modiq-cli/src/commands/version.rs` | `VersionCommand` gains a method returning the crate version (`env!("CARGO_PKG_VERSION")`) |
| `crates/modiq-report/src/report/finding_summary.rs` | Deleted |
| `crates/modiq-report/src/report/recommendation_summary.rs` | Deleted |
| `crates/modiq-report/src/report/report_formatter.rs` | Deleted |
| `crates/modiq-report/src/report/traceability_report.rs` | Deleted |
| `crates/modiq-report/src/report/mod.rs` | Remove the four corresponding `pub mod` / `pub use` lines |
| `docs/implementation/CrateRoadmap.md` | Update `modiq-cli` and `modiq-report` maturity notes to reflect both changes |

No changes anywhere in `modiq-runtime`, `modiq-collection`, `modiq-rules`, or `modiq-engine`. No changes to `apps/sandbox`.

---

# Public API Impact

- **`modiq-cli`**: substantial, but entirely additive within a crate that has never shipped a real API — `Application`, `AssessCommand`, `HelpCommand`, `VersionCommand` go from empty unit structs to real methods. No prior consumer exists to break.
- **`modiq-engine`**: none. `AssessmentService::execute_from_assessment_input` is called exactly as the Sandbox already calls it — no new entry point, no signature change. GOV-008 is not touched, and this Sprint is not expected to produce new evidence toward it (per the correction already on record from the Sprint 6 planning discussion: a second consumer reusing an identical call pattern doesn't generate new evidence about the API surface by itself).
- **`modiq-report`**: a real, if inert, breaking change — four `pub` types are removed from the crate's public surface. Zero actual consumers exist (reconfirmed directly against the current tree: each type is still exactly its original one-line unit struct, referenced only by its own file and `mod.rs`'s re-export), so the change is breaking in name only. Flagged explicitly in Governance Observations below because `GOVERNANCE.md`'s Public API Policy requires governance approval for breaking changes regardless of real-world impact, and `LEAD_ENGINEER_HANDOFF_v3.0.md` names this specific deletion as needing separate, explicit authorization beyond general Sprint scope authorization.

---

# Validation Strategy

Per `CHIEF_ARCHITECT_HANDOFF_v1.0.md` Section 10 and the Standard Sprint Execution sequence:

- `cargo fmt`, `cargo check --workspace`, `cargo test --workspace` — root workspace, zero warnings.
- `apps/sandbox/src-tauri`'s own workspace validated independently, even though nothing in this Sprint touches it directly — it depends on `modiq-engine` and, transitively, `modiq-report`, so the retirement must be confirmed not to break its build.
- New `modiq-cli` tests follow the established real-I/O discipline: real temporary directories (mirroring `modiq-engine`'s own `TempDir` test helper) exercised through `AssessCommand`'s real method, not mocked input or captured-stdout assertions. Planned coverage:
  - a successful assessment against a real temp directory with a real file (Evidence, Finding, Recommendation all present)
  - an empty-input rejection (`AssessmentExecutionError::InvalidInput`)
  - an inaccessible-path rejection (`AssessmentExecutionError::Collection(Inaccessible)`)
  - `help` and `version` command output
  - `Application`'s dispatch for a recognized and an unrecognized first argument
- No new determinism claim is introduced by this Sprint — `AssessCommand` calls an already-deterministic entry point once per invocation and does not aggregate or reorder anything itself, so no new arrival-order test is required (Sprint 5 Phase 5's standard: only new ordering claims need their own direct test).

---

# Risks

- **Missing `Display` on Runtime identity/enum types** (`AssessmentId`, `FindingSeverity`, `EvidenceCategory`, `AssessmentStatus`) means `AssessCommand`'s success-path text output can only use `{:?}` Debug formatting for these fields, exactly as the Sandbox's own frontend already does. This is not a new problem this Sprint introduces, but a CLI is the first *text-only* consumer, where this has more visible impact than in a GUI with its own independent rendering logic. Named explicitly as a risk, not silently worked around by (for example) hand-writing ad hoc match-based formatting inside `modiq-cli`, which would risk drifting from `Debug`'s output and would arguably start to look like presentation logic creeping into a crate that must never contain business logic.
- **Error-to-exit-code mapping is new territory.** No existing consumer (Sandbox `.expect()`s success against a fixed fixture) has ever had to convert an `AssessmentExecutionError` into a process outcome. Proposed convention, stated here for review rather than decided silently: exit `0` whenever `execute_from_assessment_input` returns `Ok` — including when Findings are present, since the assessment itself succeeded — and a non-zero exit only when it returns `Err`. This mirrors standard Unix CLI convention and treats "the assessment ran" and "the assessment found problems" as different questions, consistent with `Vision.md`'s own framing (modIQ explains, it doesn't judge).
- **Cargo.toml dependency addition risk is low but real**: adding `modiq-engine`/`modiq-runtime` to `modiq-cli` is the crate's first-ever dependency of any kind; worth a deliberate check that no circular or unintended dependency is introduced (none is expected — `modiq-cli` already sits at the top of the dependency graph per `PROJECT_HANDOFF_v1.0.md`'s own crate table).
- **Reporting retirement touches a `pub` surface with a public-API policy attached**, even though real-world impact is zero. Addressed directly in Governance Observations.

---

# Governance Observations

- **The `modiq-report` deletion requires the specific, separate authorization `LEAD_ENGINEER_HANDOFF_v3.0.md` calls for**, distinct from general Sprint 6 scope authorization: "do not delete them without a separate, explicit authorization, even though the recommendation is on record." This plan is that request. Authorizing this Sprint's scope should explicitly cover this deletion, not leave it implied.
- **No new Governance Register item is anticipated.** Per `GOVERNANCE.md`'s Change Categories, wiring an already-scaffolded, unwired crate to an already-stable entry point is not a Level 3 (Behavioral) or Level 4 (Architectural) change — no new valid/invalid input category is introduced (the CLI surfaces the same four Collection Outcomes `AssessmentExecutionError` already represents), and no ownership boundary moves. The `modiq-report` retirement is likewise not new ownership — `GOVERNANCE.md`'s "Reporting" Owns entry is unchanged; only never-used types built toward it are removed.
- **`GOVERNANCE.md`'s "CLI" crate boundary rule is the binding constraint for implementation**: "Owns: user interaction, command execution, platform entry point. Must never contain business logic." This plan's design (thin dispatch + direct `AssessmentService` calls + no reimplementation of pipeline behavior) is built to satisfy that boundary directly, not incidentally.
- **This Sprint is a second real consumer of `AssessmentService`, but is not expected to generate new evidence toward GOV-008**, per the correction already made to `SPRINT6_PLANNING_DOCUMENT.md` — worth restating here so it isn't silently re-litigated during implementation review.

---

# Architectural Questions Requiring Chief Architect Review

1. **Argument parsing: manual `std::env::args()`, or a new dependency (e.g. `clap`)?** `modiq-cli/Cargo.toml` currently declares zero dependencies. This plan defaults to manual parsing specifically to avoid triggering the standing "no new external crate dependency without explicit authorization" constraint — but a real CLI usually benefits from a proper argument-parsing crate (better `--help`, error messages, flag handling). If a dependency is preferred over the zero-dependency default, that authorization needs to be explicit and separate from Sprint scope authorization, per the same standing rule this plan applies to `modiq-report`'s deletion.
2. **Is a CLI's need for readable text output the forcing function that finally justifies `Display` on Runtime identity/enum types?** This has been flagged in six consecutive release records without being scheduled (`LEAD_ENGINEER_HANDOFF_v3.0.md`, Open Engineering Risks). Sprint 6 is the first time a *text-only* consumer exists. This plan does not propose adding `Display` as part of this Sprint's scope — it is a `modiq-runtime` change, touching the leaf crate every other crate depends on, and deserves its own explicit scoping decision rather than being folded into CLI wiring as "while we're at it." Flagged here specifically so the choice not to fold it in is a deliberate one, not an oversight.
3. **The proposed exit-code convention** (§ Risks, above) has no precedent in this codebase to mirror. Requesting explicit confirmation rather than treating it as an implementation detail, since it's user-facing CLI behavior with no existing analog to defer to.

---

# Success Criteria

- `modiq-cli assess <path>` runs a real Assessment against a real filesystem or archive location via `AssessmentService::execute_from_assessment_input`, with no reimplemented pipeline logic.
- `modiq-cli help` and `modiq-cli version` produce real output.
- `modiq-report` contains only `AssessmentReport` and its existing, real tests.
- `cargo fmt`, `cargo check --workspace`, `cargo test --workspace` all pass with zero warnings, both the root workspace and `apps/sandbox/src-tauri`.
- No change to `AssessmentService`'s public entry points, `AssessmentInput`, or the public error model.
- `CrateRoadmap.md` accurately reflects both crates' new state.

---

# Completion Checklist

- [ ] `modiq-cli` wired to `modiq-engine`, all three commands functional
- [ ] `modiq-report` scaffold types deleted, `mod.rs` updated
- [ ] Tests added for all new `modiq-cli` behavior (real I/O, no mocks)
- [ ] Root workspace: `cargo fmt` / `cargo check --workspace` / `cargo test --workspace`, zero warnings
- [ ] `apps/sandbox/src-tauri`: `cargo check` / `cargo test`, zero warnings
- [ ] `CrateRoadmap.md` updated for both crates
- [ ] Implementation report produced, including the three architectural questions' resolutions as actually decided

# GOV-003 Implementation Authorization — Retire `modiq-common`

| Property | Value |
|---|---|
| **Document** | GOV003_IMPLEMENTATION_AUTHORIZATION.md |
| **Project** | modIQ |
| **Governing Resolution** | `docs/engineering/GOV003_ARCHITECTURE_EVALUATION.md` §6 — Option A (retire) Accepted |
| **Purpose** | Translate the Architectural Resolution into an authorized engineering envelope: what may be changed, what must not be, and what verification is required. |
| **Status** | **Implementation Authorization only. No file has been deleted, no `Cargo.toml` has been edited. Sprint Planning (`GOV003_SPRINT_PLAN.md`) follows as a separate document.** |

---

## 1. Authorized Scope

- Remove `"crates/modiq-common"` from the root `Cargo.toml`'s `[workspace] members` list.
- Delete the `crates/modiq-common/` directory in its entirety (`Cargo.toml`, `README.md`, `src/lib.rs`, `src/error.rs`, `src/id.rs`, `src/prelude.rs`).
- Update the living documents that describe `modiq-common` as a current crate, so repository truth matches the resolution (per `CHIEF_ARCHITECT_HANDOFF_v1.1.md` §7, "Repository truth"): `README.md`'s crate table, `docs/implementation/CrateRoadmap.md`'s crate table and dependency diagram, `docs/implementation/DependencyMap.md`'s dependency diagram, `docs/governance/CHANGELOG.md`.
- Add a `CrateRoadmap.md` revision history entry recording the retirement, mirroring the entry format already used for GOV-004's own scaffold retirement (1.9.0/1.10.0).

## 2. Explicitly Out of Scope

- No replacement crate, module, or type is introduced. This is a removal, not a substitution — mirroring the GOV-004 precedent's own "this removes scaffolding, it does not substitute for it."
- No other crate's `Cargo.toml`, source, or tests may be touched — confirmed at Evaluation time that zero crates depend on `modiq-common`, so no dependent-crate change should be necessary. If implementation discovers any actual reference this authorization did not anticipate, work stops and it is reported, per this project's standing escalation rule, rather than resolved unilaterally.
- No historical Sprint/Release document (`ENGINEERING_RELEASE_*.md`, `SPRINT*_*.md`, `HANDOFF_SPRINT1.md`, superseded handoff versions, etc.) is edited — these are frozen records of what was true when written, not living status trackers, consistent with how GOV-004's own implementation left `EngineAPI.md`'s superseded history untouched.
- No change to `GOVERNANCE.md` beyond what `GOV003_ARCHITECTURE_EVALUATION.md`'s own Resolution already recorded — no new Crate Boundary Rule pair is added for a crate being removed.

## 3. Required Verification

- `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` at the root workspace — must remain clean, zero warnings, and the test count must be unchanged (no test currently exercises `modiq-common`, since it has zero consumers; a changed count would itself indicate this authorization's evidence was wrong and implementation should stop and report it).
- `apps/sandbox/src-tauri` (its own, separate workspace) — `cargo fmt --check`, `cargo check`, `cargo test`, independently reverified, since it is a separate Cargo workspace not covered by the root `cargo check --workspace` run.
- Confirm no residual reference: `grep -rn "modiq-common\|modiq_common"` across the workspace, outside historical documents named in Section 2, must return nothing.

## 4. Risk

Very low. Zero consumers, zero real content, confirmed directly at Evaluation time and re-confirmed as this authorization's own precondition. The one genuine risk is process, not technical: skipping the "no residual reference" check (Section 3) and leaving a stale `README.md`/`CrateRoadmap.md` claim behind, the documentation-staleness failure mode this project has specifically disciplined itself against since Sprint 6.

---

Sprint Planning follows in `GOV003_SPRINT_PLAN.md`.

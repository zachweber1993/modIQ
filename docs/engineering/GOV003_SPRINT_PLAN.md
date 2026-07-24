# GOV-003 Sprint Plan — Retire `modiq-common`

| Property | Value |
|---|---|
| **Document** | GOV003_SPRINT_PLAN.md |
| **Project** | modIQ |
| **Governing Authorization** | `docs/engineering/GOV003_IMPLEMENTATION_AUTHORIZATION.md` |
| **Status** | **Plan only. Implementation has not begun.** |

---

## Scope

A single phase, sized to the authorized scope: retire `modiq-common` and reconcile the living documents that describe it. No second phase is anticipated — there is no incremental or partial way to retire a crate.

## Phase 1 — Retirement

1. Remove `"crates/modiq-common"` from the root `Cargo.toml`'s `members` list.
2. Delete `crates/modiq-common/` in its entirety.
3. Update living documents (per Authorization §1): `README.md` crate table; `docs/implementation/CrateRoadmap.md` crate table, dependency diagram, and a new revision history entry; `docs/implementation/DependencyMap.md` dependency diagram; `docs/governance/CHANGELOG.md`.
4. Run the verification named in Authorization §3: root workspace `cargo fmt --check` / `cargo check --workspace` / `cargo test --workspace`; `apps/sandbox/src-tauri` `cargo fmt --check` / `cargo check` / `cargo test`; a residual-reference grep across the workspace.
5. Record an `ENGINEERING_LOG.md` entry (Status/Affected Crates/Affected Documents/Notes) reporting the actual test counts observed, not carried forward from this plan.

## Completion Checklist

- [ ] `modiq-common` removed from workspace `Cargo.toml`
- [ ] `crates/modiq-common/` deleted
- [ ] `README.md`, `CrateRoadmap.md`, `DependencyMap.md`, `CHANGELOG.md` reconciled
- [ ] Root workspace: `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` clean
- [ ] `apps/sandbox/src-tauri`: `cargo fmt --check`, `cargo check`, `cargo test` clean
- [ ] Residual-reference grep clean (outside historical documents)
- [ ] `ENGINEERING_LOG.md` entry recorded
- [ ] `PROJECT_STATUS.md` updated to reflect completion

## Explicit Non-Goals

Identical to Authorization §2 — no replacement crate or type, no other crate touched beyond the removal itself, no historical document edited.

---

**This Sprint Plan does not authorize execution on its own** — per this project's own Decision Framework, Implementation follows Governance and Authorization, but beginning Phase 1 is the Chief Architect's own go/no-go, not implied by this document's existence.

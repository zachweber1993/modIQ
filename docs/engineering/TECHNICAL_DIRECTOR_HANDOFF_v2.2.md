# Technical Director Engineering Handoff v2.2

**Engineering Release 0.2+ · Sandbox Phase 2 · Sprint 3 Phase 4**

| Property | Value |
|----------|-------|
| **Document** | TECHNICAL_DIRECTOR_HANDOFF_v2.2.md |
| **Project** | modIQ |
| **Purpose** | Canonical engineering handoff — the starting point for the next engineering session |
| **Supersedes** | TECHNICAL_DIRECTOR_HANDOFF_v2.1.md (historical; not rewritten, superseded per Documentation Status convention) |
| **Last Updated** | 2026-07-19 |
| **Branch** | `feature/runtime-implementation` |

---

## Executive Summary

modIQ is a platform for producing **deterministic, evidence-based, explainable assessments** of Farming Simulator mods. Its purpose is not to generate an opaque quality score but to answer three questions for every Assessment: does the mod work, why was this conclusion reached, and what can be done to improve it.

Since v2.1: Sprint 3 Phase 3 was an architecture/documentation-only phase — an Architecture Review proposal (`PROPOSAL_EVIDENCE_COLLECTION_BOUNDARY.md`) was approved, producing ADR-0008 (Evidence Collection Subsystem Boundary), ADR-0009 (AssessmentService Public API Evolution, drafted separately per instruction), a new `EvidenceCollection.md` specification, corresponding `Architecture.md`/`DataModel.md`/`RuleEngine.md` amendments, and four new Governance Register items (GOV-007 through GOV-010). Documentation Release 2.1 was frozen (commit `a7bb216`). Sprint 3 Phase 4 then **implemented** the boundary ADR-0008 defined: a new `modiq-collection` crate now exists in the workspace, wired into `modiq-engine` via a new, additive method, with the complete pipeline demonstrated end-to-end through the real engine. **GOV-007 (Evidence Collection Subsystem Implementation Approval) is now Resolved.** GOV-008, GOV-009, and GOV-010 remain open.

**Where the project stands today:** Documentation Release 2.1 is complete and frozen. `modiq-collection` is the platform's ninth crate — L3-equivalent for its narrow scope, zero-parsing by design. No ZIP, XML, or Lua inspection exists anywhere in the platform yet; that remains entirely future work, deliberately not started. **Nothing is currently uncommitted; the repository is clean.** The next Sprint 3 milestone has not yet been selected.

---

## Current Repository State

**`apps/`** — one application, `apps/sandbox`, unchanged in stack since v2.0. Its `create_assessment` command now calls `AssessmentService::execute_from_descriptor` instead of constructing Evidence directly — its last piece of direct Runtime-value construction is gone. 3 Rust unit tests, unchanged in count.

**`crates/`** — **nine** crates as of this phase (was eight since Sprint 0):

| Crate | Maturity | State |
|---|---|---|
| `modiq-runtime` | L3, heavily tested (82 tests) | Unchanged this phase |
| `modiq-rules` | L3 for one rule | Unchanged this phase |
| `modiq-report` | L3 | Unchanged this phase |
| `modiq-engine` | L3 for `AssessmentService` | Gained `execute_from_descriptor` (additive; `execute` itself untouched) |
| `modiq-collection` | **New.** Minimal but functionally complete for its current scope (8 tests) | `InputDescriptor`, `InputDescriptorError`, `EvidenceCollector`. Depends only on `modiq-runtime`. Produces one deterministic Evidence item per descriptor; no real inspection of any kind |
| `modiq-knowledge` | L1, scaffold | Not connected to anything |
| `modiq-versioning` | L1, scaffold | Not connected to anything |
| `modiq-cli` | L1, scaffold | Not wired to `modiq-engine` |
| `modiq-common` | L1, scaffold | Unused; purpose undecided (GOV-003) |

**`docs/`** — unchanged in layering. New this phase: none (Phase 4 was implementation-only; `GOVERNANCE.md`'s GOV-007 entry was updated to Resolved, and this handoff and the engineering log entry were added as part of closing the phase).

**Current branch:** `feature/runtime-implementation`, clean working tree as of this handoff.

**Current Engineering Release:** still 0.2, tagged `v0.2.0-alpha`. Sprint 3 (Phases 1–4) has not yet been folded into a new Engineering Release document.

**Current Documentation Release:** 2.1, Complete and Frozen (commit `a7bb216`). No documentation changed during Phase 4 beyond the Governance Register and this handoff/log pair — a documentation Release freeze does not need re-running for implementation that stays inside an already-frozen boundary.

**Sandbox status:** unchanged in shape; its one command now demonstrates Evidence Collection rather than sandbox-constructed Evidence. No new Cargo dependency was needed — `modiq-collection` reaches it only transitively, through `modiq-engine`.

---

## Platform Architecture

```
                 modiq-cli          apps/sandbox (Tauri)
                      │                    │
                      └─────────┬──────────┘
                                 ▼
                           modiq-engine
          ┌─────────┼──────────┬──────────┬──────────┬──────────────┐
          ▼         ▼          ▼          ▼          ▼              ▼
   modiq-runtime  modiq-knowledge  modiq-rules  modiq-versioning  modiq-report  modiq-collection
          │                                 │                                       │
          └───────────────┬─────────────────┘                                       │
                          ▼                                                         │
                    modiq-common                                          (depends on modiq-runtime only)
```

Declared edges as of this phase: `modiq-rules → modiq-runtime`, `modiq-report → modiq-runtime`, `modiq-collection → modiq-runtime` (**new**), `modiq-engine → {modiq-runtime, modiq-rules, modiq-report, modiq-collection}` (**modiq-collection new**), `apps/sandbox → {modiq-runtime, modiq-report, modiq-engine}` (unchanged — no new direct edge to `modiq-collection`). `modiq-cli → modiq-engine`, `modiq-engine → {modiq-knowledge, modiq-versioning}`, and any edge into `modiq-common` remain undeclared. Dependency direction remains strictly downward; `modiq-collection` joins `modiq-rules`/`modiq-report` as a leaf-adjacent peer, exactly as ADR-0008 specified.

Ownership boundaries (`GOVERNANCE.md`) — Evidence Collection's boundary (added Phase 3) is now implemented, not just documented. All other boundaries unchanged.

---

## Runtime Domain Model

Unchanged since v2.1. `modiq-runtime` was not touched this phase — Evidence Collection produces `Evidence` using the crate's existing, unmodified `Evidence::new` constructor.

---

## Execution Pipeline

**Now real, not just documented.** `AssessmentService::execute_from_descriptor(subject, context, input)`:

```
Input (a plain String today — InputDescriptor's exact
       eventual shape is still GOV-009)
        │
        ▼
InputDescriptor::new  (validates non-empty)
        │
        ▼
EvidenceCollector::collect  (modiq-collection; produces
        │                    one deterministic Evidence item;
        │                    no parsing of any kind)
        ▼
AssessmentService::execute  (existing, unchanged: Assessment
        │                    lifecycle, Rule Engine, Report)
        ▼
AssessmentReport
```

`AssessmentService::execute` itself is untouched — `execute_from_descriptor` is a new, parallel entry point, not a breaking change. Both methods exist and both are tested.

---

## Sandbox Architecture

Unchanged in stack and IPC model. `create_assessment` now calls `execute_from_descriptor` with a fixed string literal rather than constructing an `Evidence` value directly — the sandbox's already-thin domain-logic footprint got thinner, not thicker, this phase. Current limitations (no persistence, no file dialogs, no real ZIP/XML/Lua parsing anywhere in the platform, derived-`Debug` identifier rendering) are all still true.

---

## Documentation Status

**Documentation Release:** 2.1, Complete and Frozen. See `docs/governance/DocumentationRelease.md`.

**Engineering Releases:** v0.1.0-alpha → 0.2 (tag `v0.2.0-alpha`). Sprint 3 (all four phases so far) has not yet been folded into a new Engineering Release.

**ADR inventory:** nine ADRs, all Accepted (ADR-0008, ADR-0009 added Phase 3). Neither was modified this phase — per this project's convention, an ADR's Status reflects the architectural decision's own acceptance, not later implementation progress; GOV-007's resolution is recorded in `GOVERNANCE.md`, not by editing ADR-0008.

**Governance status** — ten items in the register, three resolved:

- **GOV-001** through **GOV-004** — unchanged, still Open, pending.
- **GOV-005**, **GOV-006** — Resolved (cardinality only), Sprint 3 Phase 2.
- **GOV-007** — **Resolved this phase.** Evidence Collection implementation approved and delivered at minimal scope.
- **GOV-008** — Open. AssessmentService Public API Evolution: whether/how `execute` itself should eventually change. Untouched by this phase's additive `execute_from_descriptor`.
- **GOV-009** — Open. Input Descriptor ownership: which specification authoritatively owns it, and what it eventually carries beyond an opaque string. `modiq-collection::InputDescriptor` is an explicitly minimal, non-authoritative placeholder pending this.
- **GOV-010** — Open. Collection Error Model: `EvidenceCollector::collect` is currently infallible by honest necessity (no real I/O exists to fail), not by a resolved design decision.

**Known deferred decisions:** unchanged list, now joined by "which concrete collector to build first" (ZIP traversal, XML inspection, Lua inspection, manifest inspection, dependency inspection — none started) and the three open GOV items above.

**Still stale, not corrected (carried forward, out of scope each time it's been checked):** `docs/README.md`'s "Current Documentation Status" narrative paragraph (Sprint-number staleness, distinct from its Reading Order list, which was corrected in Phase 3), `docs/00-Governance.md`'s vestigial short index, and `TECHNICAL_DIRECTOR_HANDOFF_v2.1.md`'s now-superseded speculative "GOV-007" suggestion (harmless once superseded, but not edited, per handoff convention).

---

## Repository Health

**Workspace status:** root workspace (**nine** crates) clean; `cargo fmt`, `cargo check --workspace`, `cargo test --workspace` all green, zero warnings. Sandbox workspace independently clean.

**Test count:** **106 tests** in the root workspace (up from 95: **8 new in `modiq-collection`**, **3 new in `modiq-engine`** for `execute_from_descriptor`; 82 `modiq-runtime`, 3 `modiq-engine` integration, 3 `modiq-report`, 3 `modiq-rules` all otherwise unchanged). Sandbox has 3 additional tests, not counted in the 106.

**Verification rituals:** unchanged.

**Build status:** zero warnings across both workspaces as of the end of Sprint 3 Phase 4.

---

## Technical Debt

Unchanged list from v2.1. Nothing new added: the minimal `modiq-collection` implementation was scoped precisely to avoid introducing debt (no unreachable error paths, no speculative abstractions, no unused scaffolding).

---

## Immediate Next Steps

1. **Select and approve Sprint 3's next milestone.** Strong candidates, none pre-selected here: (a) a first real, I/O-capable collector (which would immediately force GOV-009 and GOV-010 to stop being hypothetical), (b) resolving GOV-008 (whether `execute` itself should evolve, now that a real alternative — `execute_from_descriptor` — exists to compare it against), or (c) something else entirely (CLI wiring, Knowledge Domain integration remain on the longer-term roadmap).
2. **Consider whether Sprint 3 Phases 1–4 warrant a new Engineering Release document** — four phases in, this is a larger gap between releases than Sprint 2's single-phase-to-release cadence.
3. **GOV-009 and GOV-010 will likely need to be resolved together** with whichever real collector is chosen next, since a real collector is the first thing that will actually exercise a failure mode and a non-trivial descriptor shape.

---

## Longer-Term Roadmap

Real Evidence Collection (a first genuine collector — ZIP traversal is the most likely candidate, but not decided) → resolve GOV-008/009/010 against that real experience → Rule abstraction and multiple Rules (still deferred, still unjustified by a second concrete Rule) → Knowledge Domain integration → Version Profile integration → CLI wiring → advanced Reporting/Explainability → persistence → remaining Governance Register items → eventual production-grade application.

---

## Guidance for the Next Technical Director

Same five questions as prior handoffs (Frozen-spec conflict? Crate boundary crossing? Documentation staying honest? Abstraction earned yet? When in doubt, stop and ask). Phase 4 is a clean example of the fourth: `EvidenceCollector::collect` was kept infallible specifically because a `Result` type with no reachable error case would have been abstraction ahead of need — the same discipline already applied to the Rule Engine trait question in Phase 3.

One addition based on this phase: **"additive API growth" is a real escape valve from a blocked breaking-change gate, and it worked cleanly here** (GOV-008 stayed fully open while `execute_from_descriptor` still shipped) — but it's not free forever. If a third or fourth parallel entry point starts accumulating on `AssessmentService` for the same underlying reason (avoiding a `execute` signature decision), that's a signal GOV-008 needs to actually be resolved rather than routed around again.

---

## Session Resume Checklist

Before writing code:

- [ ] Read this handoff completely.
- [ ] Read `PROJECT_STATUS.md`.
- [ ] Read `GOVERNANCE.md`'s Governance Register (GOV-007 now Resolved; GOV-008/009/010 still Open).
- [ ] Read `docs/architecture/EvidenceCollection.md` if touching Evidence Collection further.
- [ ] Run:
  - `cargo fmt`, `cargo check --workspace`, `cargo test --workspace`
- [ ] Confirm Sandbox launches.
- [ ] Do not begin Sprint 3's next milestone until its Architecture Review proposal (if one is needed) is explicitly approved.

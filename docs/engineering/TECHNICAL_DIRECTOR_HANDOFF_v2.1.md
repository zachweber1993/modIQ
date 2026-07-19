# Technical Director Engineering Handoff v2.1

**Engineering Release 0.2+ · Sandbox Phase 2 · Sprint 3 Phase 2**

| Property | Value |
|----------|-------|
| **Document** | TECHNICAL_DIRECTOR_HANDOFF_v2.1.md |
| **Project** | modIQ |
| **Purpose** | Canonical engineering handoff — the starting point for the next engineering session |
| **Supersedes** | TECHNICAL_DIRECTOR_HANDOFF_v2.0.md (historical; not rewritten, superseded per Documentation Status convention) |
| **Last Updated** | 2026-07-19 |
| **Branch** | `feature/runtime-implementation` |

---

## Executive Summary

modIQ is a platform for producing **deterministic, evidence-based, explainable assessments** of Farming Simulator mods (and related subjects). Its purpose is not to generate an opaque quality score but to answer three questions for every Assessment: does the mod work, why was this conclusion reached, and what can be done to improve it.

Since v2.0: Sprint 3 Phase 1 (sandbox driving the real pipeline) was confirmed and committed (`a11f9ec`). Sprint 3 Phase 2 followed immediately — an Architecture Review proposal (`PROPOSAL_GOV-005_GOV-006.md`) resolved GOV-005 and GOV-006, both open since Sprint 2: `Finding` and `Recommendation` now enforce minimum reference cardinality (INV-013, INV-014), committed as `8fbbf71`. **Both governance items are Resolved for cardinality only.** Referential integrity — whether a reference actually resolves within its own Assessment — was explicitly excluded from that decision and remains open, expected to be folded into a future Documentation Release 2.1, which has not yet happened and is not implied to be complete by these edits.

**Where the project stands today:** Documentation Release 2.0 is complete and frozen. Engineering Release 0.2 (Sprint 2) gave the Runtime Domain's core entities real content. Sprint 3 Phase 1 proved the sandbox can drive the complete real pipeline. Sprint 3 Phase 2 closed the two governance items Sprint 2 deliberately left open. **Nothing is currently uncommitted; the repository is clean.** The next Sprint 3 milestone has not yet been selected — an Architecture Review proposal for it is expected to accompany or follow this handoff, per the established Architecture → Governance → Documentation → Implementation → Verification → Technical Director Review workflow.

---

## Current Repository State

**`apps/`** — one application, `apps/sandbox`, unchanged in shape since v2.0: Tauri 2 + React 19 + TypeScript + Vite + TailwindCSS v4 + shadcn/ui. Its own independent Cargo workspace, not a member of the root modIQ workspace. One Tauri command, `create_assessment`, executing the real pipeline. 3 Rust unit tests.

**`crates/`** — eight crates, unchanged in count and boundaries since Sprint 0:

| Crate | Maturity | State |
|---|---|---|
| `modiq-runtime` | L3, heavily tested (82 tests) | Assessment aggregate; Evidence, Finding, Recommendation with real fields, identity, validation, and now-enforced minimum reference cardinality (INV-013, INV-014) |
| `modiq-rules` | L3 for one rule | Deterministic `RuleEngine`; unchanged this phase — its single Rule already always produced non-empty references |
| `modiq-report` | L3 | `AssessmentReport` read-only snapshot; test fixtures only touched this phase |
| `modiq-engine` | L3 for `AssessmentService` | Orchestrates the full pipeline; four other EngineAPI services remain stubs |
| `modiq-knowledge` | L1, scaffold | Not connected to anything |
| `modiq-versioning` | L1, scaffold | Not connected to anything |
| `modiq-cli` | L1, scaffold | Not wired to `modiq-engine` |
| `modiq-common` | L1, scaffold | Unused; purpose undecided (GOV-003) |

**`docs/`** — unchanged in layering since v2.0 (constitutional/architecture/technical Frozen; implementation mixed; governance living; engineering living; ADRs historical). New this phase: `docs/engineering/PROPOSAL_GOV-005_GOV-006.md` (the Architecture Review proposal that led to this phase's decision).

**Current branch:** `feature/runtime-implementation`, clean working tree as of this handoff.

**Current Engineering Release:** still 0.2, tagged `v0.2.0-alpha`. Sprint 3 (Phases 1 and 2) has not yet been folded into a new Engineering Release document — that remains a future freeze point, not implied to be needed immediately.

**Current Documentation Release:** 2.0, Complete. GOV-005/GOV-006 were resolved by direct edits to `RuntimeInvariants.md`/`GOVERNANCE.md` rather than a full Documentation Release cycle (Draft → Foundation Review → Technical Review → Repository Audit → Documentation Freeze → Release Tag). **Documentation Release 2.1 — under which these items were originally filed as "pending" — should be treated as a still-open, separately scheduled milestone, not as retroactively completed by this phase.** This is a judgment call the previous Technical Director decision made implicitly (by authorizing in-place edits); flagging it here so it isn't lost.

**Sandbox status:** unchanged since v2.0 — Phase 1, Phase 2, and Sprint 3 Phase 1 all committed. No separately tracked "Sandbox Phase 3."

---

## Platform Architecture

Unchanged since v2.0 — no crate boundary, dependency edge, or ownership boundary moved this phase.

```
                 modiq-cli          apps/sandbox (Tauri)
                      │                    │
                      └─────────┬──────────┘
                                 ▼
                           modiq-engine
          ┌─────────┼──────────┬──────────┬──────────┐
          ▼         ▼          ▼          ▼          ▼
   modiq-runtime  modiq-knowledge  modiq-rules  modiq-versioning  modiq-report
          │                                 │
          └───────────────┬─────────────────┘
                          ▼
                    modiq-common
```

Declared edges today, unchanged: `modiq-rules → modiq-runtime`, `modiq-report → modiq-runtime`, `modiq-engine → {modiq-runtime, modiq-rules, modiq-report}`, `apps/sandbox → {modiq-runtime, modiq-report, modiq-engine}`. `modiq-cli → modiq-engine`, `modiq-engine → {modiq-knowledge, modiq-versioning}`, and any edge into `modiq-common` remain undeclared.

Ownership boundaries (GOVERNANCE.md) — all unchanged since v2.0; see that document for the full statement per crate.

---

## Runtime Domain Model

Unchanged from v2.0 except where noted.

**Assessment** — the aggregate root. Owns identity, subject, context, a four-state lifecycle, and its Evidence/Finding/Recommendation collections. `add_evidence`/`add_finding`/`add_recommendation` are unchanged this phase — they still check completion, lifecycle state, and (for `add_recommendation`) that at least one Finding exists in the Assessment; none inspect the content of a Finding/Recommendation's own reference fields, since that is now enforced one layer down, at construction.

**Finding** — unchanged shape. `Finding::new` now **requires** `evidence_ids` to be non-empty (INV-013); previously it accepted `vec![]`. `FindingError` gained `EmptyEvidenceIds`.

**Recommendation** — unchanged shape. `Recommendation::new` now **requires** `finding_ids` to be non-empty (INV-014, a new invariant rather than a reworded INV-005); previously it accepted `vec![]`. `RecommendationError` gained `EmptyFindingIds`.

**Relationship resolution** — `evidence_for_finding` and `findings_for_recommendation` are **unchanged in behavior**: an unresolvable id is still silently omitted, never rejected. This was a deliberate, explicit scope boundary this phase (Technical Director instruction: "Do not modify Assessment relationship resolution... Do not add graph validation"). The doc comments on both methods were updated to reflect that cardinality is now enforced upstream, while referential integrity remains the open question.

**Current invariants:** INV-001 through INV-014 (`RuntimeInvariants.md`) — the lifecycle/aggregate-ownership set from Sprint 1–2, plus the new Entity Reference Requirements section (INV-013, INV-014) from this phase.

---

## Execution Pipeline

Unchanged since v2.0. See that document (or `RuleEngine.md`/`Architecture.md`) for the full Evidence → Rule Engine → Findings → Recommendations → Assessment Report → Sandbox DTO → React UI walkthrough, and for why Evidence is not produced by the Rule Engine (still the central architectural precedent for how this project resolves ownership ambiguity — read before proposing anything that touches Evidence Collection).

---

## Sandbox Architecture

Unchanged since v2.0. `create_assessment` executes the real pipeline through `AssessmentService::execute` and maps the result to a DTO; the `Assessment` aggregate itself is never exposed over IPC. Current limitations (no persistence, no file dialogs, no ZIP/XML/Lua parsing, derived-`Debug` identifier rendering) are all still true — nothing in Sprint 3 Phase 2 touched the sandbox.

---

## Documentation Status

**Documentation Release:** 2.0, Complete. Unchanged. GOV-005/GOV-006's Resolution text now reflects the Technical Director's decision; the invariants they authorized live in `RuntimeInvariants.md`, INV-013/INV-014.

**Engineering Releases:** v0.1.0-alpha → 0.2 (tag `v0.2.0-alpha`). Sprint 3 Phases 1 and 2 have not yet been folded into a new Engineering Release.

**ADR inventory:** unchanged since v2.0 (seven ADRs, all Accepted). No ADR was needed this phase — GOV-005/GOV-006 were Level 3 (Behavioral) changes per `GOVERNANCE.md`, which requires governance approval and a documentation update, not a Level 4 (Architectural) change requiring an ADR.

**Governance status** — six items remain in the register, two newly resolved:

- **GOV-001** — Assessment Report generation timing. Still Open, pending Documentation Release 1.1.
- **GOV-002** — Runtime invariant reconciliation. Still Open, pending Documentation Release 1.1.
- **GOV-003** — role of `modiq-common`. Still Open, pending.
- **GOV-004** — Engine service granularity. Still Open, pending.
- **GOV-005** — Finding→Evidence reference requirement. **Resolved (cardinality only)** this phase.
- **GOV-006** — Recommendation→Finding reference requirement (INV-005 refinement). **Resolved (cardinality only)** this phase.

**Newly identified, not yet registered:** referential integrity for both Finding→Evidence and Recommendation→Finding (whether a reference must actually resolve, not just exist) was explicitly deferred by the Technical Director rather than folded into GOV-005/GOV-006. It does not yet have its own Governance Register entry — worth assigning one (e.g. GOV-007) the next time it's substantively discussed, rather than leaving it as prose inside two now-Resolved items.

**Known deferred decisions:** unchanged list from v2.0 (Rule abstraction, Knowledge Domain integration, Version Profile integration, CLI wiring, advanced Reporting, persistence, `modiq-common`'s purpose, Engine service expansion, real Evidence Collection subsystem), plus referential integrity (above).

**Still stale, not corrected this phase (carried forward from v2.0):** `docs/README.md`'s "Current Documentation Status" section and `docs/00-Governance.md`'s vestigial reading-order index. Flagging again since they were flagged and not fixed last time either.

---

## Repository Health

**Workspace status:** root workspace (8 crates) clean; `cargo fmt`, `cargo check --workspace`, `cargo test --workspace` all green, zero warnings. Sandbox workspace independently clean.

**Test count:** 95 tests in the root workspace (82 `modiq-runtime` — down from 84: two tests whose premise became unreachable were removed, two new rejection tests were added, net −2; 4 `modiq-engine` unit, 3 `modiq-engine` integration, 3 `modiq-report`, 3 `modiq-rules`, all unchanged). The sandbox has 3 additional Rust tests, not counted in the 95.

**Verification rituals:** unchanged from v2.0.

**Build status:** zero warnings across both workspaces as of the end of Sprint 3 Phase 2.

---

## Technical Debt

Unchanged from v2.0 (no public `Display`/`Serialize` for Runtime identity/enum types; `modiq-cli` unwired; `modiq-common` unused; two unfixed ADR document defects; git tag hygiene; `docs/README.md`/`docs/00-Governance.md` staleness). Nothing this phase added or resolved technical debt in this list — it was a pure governance/invariant closure.

---

## Immediate Next Steps

1. **Select and approve Sprint 3's next milestone.** An Architecture Review proposal is expected alongside or immediately following this handoff, evaluating candidate next steps against the same criteria used throughout this project (Frozen-spec conflicts, crate-boundary crossings, documentation honesty, earned-abstraction test). Per explicit Technical Director instruction, implementation must not begin until that proposal is reviewed and approved.
2. **Consider registering referential integrity as its own Governance Register item** (see Documentation Status) before it resurfaces informally again.
3. **Consider whether Sprint 3 Phases 1–2 warrant a new Engineering Release document**, or whether that freeze point should wait for a further Sprint 3 phase — not decided here.

---

## Longer-Term Roadmap

Unchanged since v2.0, kept intentionally brief: Evidence Collection (real ZIP/XML/Lua inspection) → Rule abstraction and multiple Rules → Knowledge Domain integration → Version Profile integration → CLI wiring → advanced Reporting/Explainability → persistence → resolution of remaining Governance Register items (including the newly identified referential integrity question) → eventual production-grade application.

---

## Guidance for the Next Technical Director

Same five questions as v2.0 (Frozen-spec conflict? Crate boundary crossing? Documentation staying honest? Abstraction earned yet? When in doubt, stop and ask) — this phase followed them successfully: the cardinality/referential-integrity split was proposed by engineering, and the Technical Director's decision to accept cardinality now while explicitly deferring referential integrity is itself a model instance of "when in doubt, narrow the scope rather than guess the rest."

One addition based on this phase: **a documentation update made to satisfy a Level 3 governance decision (direct edits to `RuntimeInvariants.md`/`GOVERNANCE.md`) is not the same thing as a Documentation Release** (the formal Draft → Freeze → Tag cycle in `DocumentationRelease.md`). Both are legitimate, but conflating them risks quietly closing out a Documentation Release milestone (2.1) that was never actually run. This handoff has tried to keep that distinction explicit; future sessions should too.

---

## Session Resume Checklist

Before writing code:

- [ ] Read this handoff completely.
- [ ] Read `PROJECT_STATUS.md`.
- [ ] Read `RuntimeInvariants.md` (INV-013/INV-014 are new).
- [ ] Read `GOVERNANCE.md`'s Governance Register (GOV-005/GOV-006 now Resolved; referential integrity not yet numbered).
- [ ] Run:
  - `cargo fmt`, `cargo check --workspace`, `cargo test --workspace`
- [ ] Confirm Sandbox launches.
- [ ] Do not begin Sprint 3's next milestone until its Architecture Review proposal is explicitly approved.

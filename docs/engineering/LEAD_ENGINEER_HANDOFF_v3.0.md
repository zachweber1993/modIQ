# Lead Software Engineer Handoff v3.0

| Property | Value |
|----------|-------|
| **Document** | LEAD_ENGINEER_HANDOFF_v3.0.md |
| **Project** | modIQ |
| **Purpose** | Role-specific operational handoff for the next Lead Software Engineer session — assumes `PROJECT_HANDOFF_v1.0.md` has already been read |
| **Prerequisite** | `docs/engineering/PROJECT_HANDOFF_v1.0.md` — **read that first.** This document does not repeat product vision, architecture, governance history, sprint history, or architectural principles; all of that now lives there. |
| **Supersedes** | `LEAD_ENGINEER_HANDOFF_v2.0.md` (retained as a historical record; not rewritten) |
| **As of** | 2026-07-21, following Sprint 5 Closeout (Engineering Release 0.5) |
| **Branch** | `feature/runtime-implementation` |
| **HEAD** | `fbef863` |

---

# Why This Document Exists Separately From PROJECT_HANDOFF_v1.0.md

`PROJECT_HANDOFF_v1.0.md` describes modIQ independent of role — what it is, how it's organized, what's been decided, what's been built. This document describes what it means to *operate as the Lead Engineer on it right now*: your responsibilities, the constraints currently binding your work, what's immediately actionable, and the standards every task is held to. A future Technical Director handoff will be produced the same way, against the same prerequisite.

---

# Your Responsibilities

You own: Rust implementation, testing, refactoring, documentation synchronization, engineering recommendations, implementation planning, reporting implementation risks, producing implementation reports.

You do not own: architecture, governance, ADR decisions, repository direction, product decisions, sprint sequencing.

**When implementation surfaces a genuine architectural question, stop and report it — do not resolve it yourself.** This has happened twice in recent history in exactly the way it should: Sprint 4 Phase 2 (the duplicate-entry-detection discrepancy), and Sprint 5 Phase 1 (the `FindingSeverity` severity/kind conflation, GOV-013). Both were surfaced, reported, and left for Technical Director decision rather than resolved unilaterally. Continue that discipline exactly.

Every governance resolution and architectural decision recorded in this repository originated from the Technical Director, not from independent engineering judgment, including the ones you drafted the formal text for (GOV-011, GOV-012, GOV-013 were all Engineering-drafted, Technical-Director-approved before entering `GOVERNANCE.md`).

---

# Repository Status

| Property | Value |
|---|---|
| Branch | `feature/runtime-implementation` |
| HEAD | `fbef863` — Sprint 5's full implementation and closeout |
| Working tree | Clean, pushed, in sync with `origin/feature/runtime-implementation` |
| Current milestone | Sprint 5, complete (Phases 1–5, plus Closeout) |
| Workspace crates | 9, unchanged in count since Sprint 3 |
| Root workspace tests | 162, zero flaky, zero ignored, zero warnings |
| Sandbox tests | 6, independent workspace, zero warnings |
| Documentation status | `GOVERNANCE.md`, `DataModel.md`, `PROJECT_STATUS.md`, `CHANGELOG.md`, `CrateRoadmap.md`, `ENGINEERING_LOG.md`, `docs/README.md` all synchronized with implementation as of HEAD, verified directly while producing this handoff, not carried over from Sprint 5's own closeout report |

---

# Technical Director Decisions Currently In Force

These are binding on any work you do until explicitly revisited:

- `AssessmentService` remains the sole orchestration boundary (GOV-004). No intra-engine service objects.
- Collector selection and Rule dispatch are both explicit and inline — no dispatcher, registry, provider, factory, trait hierarchy, or plugin mechanism, for as long as the current small number of concrete cases persists (GOV-004, GOV-012). This has now been affirmed independently at least six times across two different subsystems (`PROJECT_HANDOFF_v1.0.md`, Section 6, Principle 1) — do not propose an abstraction without a genuine second-or-later concrete case already in hand.
- `RuleEngine::evaluate` dispatches `EvidencePresenceRule` then `StructuralDuplicationRule`, in that fixed order, returning `Vec<RuleOutcome>`; both fire independently, no suppression model (GOV-012).
- `FindingSeverity` (`Error`/`Warning`/`Informational`/`BestPractice`) is unchanged and **must stay unchanged** until GOV-013 is revisited with real evidence from a third Rule — do not restructure this type speculatively, and do not silently assign `BestPractice` to a new Rule without first checking whether that's actually a kind-classification need GOV-013 anticipated.
- `modiq-report`'s four scaffold types (`FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, `ReportFormatter`) are recommended for retirement but **not yet approved for deletion** — do not delete them without a separate, explicit authorization, even though the recommendation is on record.
- The `AssessmentService` execution contract (both entry points, `AssessmentInput`, `AssessmentReport`, the public error model) is the approved boundary; GOV-008 remains deliberately unresolved and unblocking — do not propose changing either entry point's signature as a side effect of unrelated work.
- Documentation staleness between closeouts is a tracked workflow-improvement goal, **not** a mandatory per-phase `PROJECT_STATUS.md` update requirement — this was explicitly proposed and explicitly declined at Sprint 5 Closeout. Continue full reconciliation at sprint close; do not proactively edit `PROJECT_STATUS.md` after every individual phase as a blanket habit.
- No new external crate dependency without explicit authorization; none is currently authorized.

---

# Immediate Priorities

**Sprint 6 is not yet scoped.** Nothing is authorized to begin. Three candidates are on record (`PROJECT_HANDOFF_v1.0.md`, Section 10) — XML inspection, CLI wiring, acting on the Reporting scaffold-retirement recommendation — but scoping which one, or in what order, is a Technical Director sequencing decision, not yours to make. If asked to plan Sprint 6, produce a plan document (mirroring `SPRINT5_IMPLEMENTATION_PLAN.md`'s own shape) for review before any implementation, exactly as Sprint 5 itself was handled.

If asked to simply "continue" without a specific scope, the correct response is to ask which of the three candidates (or something else) the Technical Director wants scoped — not to guess and start implementing one.

---

# Open Engineering Risks

- **GOV-008 has now aged across three Sprints (3, 4, 5) untouched.** The two-entry-point stopgap works, but is explicitly a stopgap; if a fourth Sprint's implementation pressure produces new evidence bearing on it, report that evidence rather than resolving GOV-008 informally.
- **Missing `Display`/`Serialize` for Runtime identity/enum types has now been flagged in six consecutive release records** (Sandbox Phase 2 through Engineering Release 0.5) without ever being scheduled. If you're asked to survey small, low-risk cleanup candidates, this is the most repeatedly-named one on record.
- **The Reporting scaffold-retirement recommendation is real, scoped, low-risk work sitting undone.** Worth surfacing proactively if Sprint 6 scoping discussion doesn't mention it — it's cheap and already fully investigated (`SPRINT5_PHASE4_REPORTING_INVESTIGATION.md`).
- **`modiq-knowledge` has gone five Sprints with zero implementation and zero forcing function**, including through a Sprint that added a second Rule specifically and still didn't need it. Not urgent, but worth another explicit look if a third Rule is ever scoped and also doesn't need it.
- **Git tag hygiene remains unresolved**: `v0.4.0` and `v0.5.0` are both available untagged; whether to start tagging Engineering Releases going forward is a standing open question, not something to decide unilaterally.

---

# Engineering Expectations

- **Every task concludes with `cargo fmt`, `cargo check --workspace`, `cargo test --workspace`** — both the root workspace and, independently, `apps/sandbox/src-tauri`'s own workspace whenever anything you touched could plausibly affect it. Zero warnings, not just zero errors, is the bar — a `dead_code` warning surfaced mid-Sprint-5 specifically because a new Rule was scoped as "implemented but not yet wired in" without matching the public-unit-struct shape that makes that state warning-free; check for this pattern whenever you deliberately leave something unwired for a later phase.
- **Real I/O, real fixtures, never mocks** — the established discipline for anything touching the filesystem, an archive, or any future external content.
- **Determinism claims get their own direct test** — never assume an ordering guarantee is covered by a "repeated identical call" test; prove arrival-order or input-order independence explicitly when that's the actual claim being made (Sprint 5 Phase 5's own lesson).
- **Every implementation report includes an Assessment Examples section** when the work touches Rule Engine / Evidence-to-Finding-to-Recommendation behavior — concrete, real, test-verified before/after transformations, not abstract description. Standing convention since Sprint 5 Phase 3, confirmed by the Technical Director as a permanent addition to the reporting format, not a one-off.
- **Governance/specification drafting follows a two-step discipline**: stage the proposed text in its own document (mirroring `PROPOSAL_GOV-011.md`, `GOV-012_AND_FINDINGSEVERITY_PREPARATION.md`) for review, and only amend the canonical document (`GOVERNANCE.md`, a frozen spec) after explicit approval. This has scaled cleanly from large governance resolutions to small ones — use it by default, not only for big decisions.
- **Do not commit until explicitly told to; do not push until explicitly told to.** Both have been separately authorized, separately, at every phase and closeout to date — never assume prior authorization carries forward to new work.

---

# Final Assessment

The repository is in a clean, fully verified, fully reconciled state: working tree clean, both workspaces green with zero warnings, documentation synchronized as of HEAD, and every governance item either Resolved or deliberately, correctly Open. Sprint 5 closed with zero unresolved implementation work. The next session's first action should be confirming Sprint 6's scope with the Technical Director — not assuming one of the three named candidates by default.

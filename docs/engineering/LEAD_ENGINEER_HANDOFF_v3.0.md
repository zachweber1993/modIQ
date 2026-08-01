# Lead Software Engineer Handoff v3.0

| Property | Value |
|----------|-------|
| **Document** | LEAD_ENGINEER_HANDOFF_v3.0.md |
| **Project** | modIQ |
| **Purpose** | Role-specific operational handoff for the next Lead Software Engineer session — assumes `PROJECT_HANDOFF_v1.1.md` has already been read |
| **Prerequisite** | `docs/engineering/PROJECT_HANDOFF_v1.1.md` — **read that first.** This document does not repeat product vision, architecture, governance history, sprint history, or architectural principles; all of that now lives there. |
| **Supersedes** | `LEAD_ENGINEER_HANDOFF_v2.0.md` (retained as a historical record; not rewritten) |
| **As of** | 2026-08-01. This revision reconciles eight Sprints (7–14) plus the post-Sprint-14 Governance Reconciliation cycle that this document had not previously reflected — descriptive reconciliation only, per Repository Reconciliation Package B; no architectural, ADR, governance, or Rust source change was made in producing this revision. Sprint 14 (GOV-003: `modiq-common` retired) remains the last Sprint to touch Rust source. |
| **Branch** | `feature/runtime-implementation` |
| **HEAD** | `e45d91c` |

---

# Why This Document Exists Separately From PROJECT_HANDOFF_v1.1.md

`PROJECT_HANDOFF_v1.1.md` describes modIQ independent of role — what it is, how it's organized, what's been decided, what's been built. This document describes what it means to *operate as the Lead Engineer on it right now*: your responsibilities, the constraints currently binding your work, what's immediately actionable, and the standards every task is held to. `CHIEF_ARCHITECT_HANDOFF_v1.1.md` is produced the same way, against the same prerequisite.

---

# Your Responsibilities

You own: Rust implementation, testing, refactoring, documentation synchronization, engineering recommendations, implementation planning, reporting implementation risks, producing implementation reports.

You do not own: architecture, governance, ADR decisions, repository direction, product decisions, sprint sequencing.

**When implementation surfaces a genuine architectural question, stop and report it — do not resolve it yourself.** This has happened twice in recent history in exactly the way it should: Sprint 4 Phase 2 (the duplicate-entry-detection discrepancy), and Sprint 5 Phase 1 (the `FindingSeverity` severity/kind conflation, GOV-013). Both were surfaced, reported, and left for Chief Architect decision rather than resolved unilaterally. Continue that discipline exactly.

Every governance resolution and architectural decision recorded in this repository originated from the Chief Architect, not from independent engineering judgment, including the ones you drafted the formal text for (GOV-011, GOV-012, GOV-013 were all Engineering-drafted, Chief-Architect-approved before entering `GOVERNANCE.md`).

---

# Repository Status

| Property | Value |
|---|---|
| Branch | `feature/runtime-implementation` |
| HEAD | `e45d91c` — a Platform Architecture Track commit, not a Sprint/code commit. The last commit to touch Rust source is Sprint 14's `f1c6073` ("feat: retire modiq-common (GOV-003)"). |
| Working tree | Clean, in sync with `origin/feature/runtime-implementation` |
| Current milestone | Sprint 14 (GOV-003: `modiq-common` retired), implemented and closed out. A subsequent Governance Reconciliation cycle (2026-07-24, not a Sprint) resolved GOV-002 and GOV-015 and accepted ADR-0011. **Sprint 15 is not yet scoped.** |
| Workspace crates | 9 — `modiq-cli`, `modiq-collection`, `modiq-engine`, `modiq-knowledge`, `modiq-report`, `modiq-rules`, `modiq-runtime`, `modiq-storage`, `modiq-versioning`. `modiq-common` no longer exists (retired Sprint 14). `modiq-storage` is new since Sprint 13. See `PROJECT_HANDOFF_v1.1.md` §2 for maturity/dependency detail. |
| Root workspace tests | 253, zero flaky, zero ignored, zero warnings — independently reverified during this revision |
| Sandbox tests | 9, independent workspace, zero warnings — independently reverified during this revision |
| Documentation status | This document, `PROJECT_HANDOFF_v1.1.md`, and `CHIEF_ARCHITECT_HANDOFF_v1.1.md` reconciled to current repository state as Repository Reconciliation Package B (2026-08-01) — descriptive synchronization only, no architectural or Rust source change. `GOVERNANCE.md`, `PROJECT_STATUS.md`, `CHANGELOG.md`, and the ADR index were separately reconciled at Package A (2026-07-24). |

---

# Chief Architect Decisions Currently In Force

These are binding on any work you do until explicitly revisited:

- `AssessmentService` remains the sole orchestration boundary (GOV-004). No intra-engine service objects.
- Collector selection and Rule dispatch are both explicit and inline — no dispatcher, registry, provider, factory, trait hierarchy, or plugin mechanism, for as long as the current small number of concrete cases persists (GOV-004, GOV-012). Affirmed repeatedly across two different subsystems (`PROJECT_HANDOFF_v1.1.md`, Section 6, Principle 1) — do not propose an abstraction without a genuine second-or-later concrete case already in hand.
- `RuleEngine::evaluate` now dispatches four Rules in fixed declaration order — `EvidencePresenceRule`, `StructuralDuplicationRule`, `VersionCompatibilityRule` (Sprint 8), `RuntimeLoadFailureRule` (Sprint 11) — returning `Vec<RuleOutcome>`; all fire independently, no suppression model (GOV-012).
- `FindingSeverity` (`Error`/`Warning`/`Informational`/`BestPractice`) is unchanged and **must stay unchanged** until GOV-013 is revisited with real evidence — do not restructure this type speculatively, and do not silently assign `BestPractice` to a new Rule without first checking whether that's actually a kind-classification need GOV-013 anticipated. Sprint 11's `FindingSeverity::Error` use is recorded as relevant evidence for a future review, not grounds to reopen GOV-013 now.
- `modiq-report`'s four scaffold types (`FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, `ReportFormatter`) were **deleted at Sprint 6, under explicit, separate Chief Architect authorization** — no longer a pending decision. `AssessmentReport` remains the crate's only content, and remains the canonical report model.
- The `AssessmentService` execution contract (both entry points, `AssessmentInput`, `AssessmentReport`, the public error model) is the approved boundary; GOV-008 remains deliberately unresolved and unblocking, now across eleven Sprints — do not propose changing either entry point's signature as a side effect of unrelated work.
- **`modiq-storage` is real since Sprint 13** — `PersistedAssessmentReport`, `ReportKey`, `ReportStore`, wired through `modiq-cli` (`retrieve`) and `apps/sandbox`, verified by genuine cross-process round trip. It defines and owns its own persisted representation rather than modifying `modiq-runtime`/`modiq-report` — do not add `Serialize`/`Deserialize` to Runtime types to "simplify" this; that boundary was deliberate (`STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md`).
- **`modiq-common` no longer exists** — retired at Sprint 14 (GOV-003): zero consumers, zero real content across 13 Sprints. Do not recreate a shared/common crate speculatively; this project's own precedent (`modiq-collection`, `modiq-storage`) is to create a crate on demand, not pre-provision one empty.
- **GOV-001 (report-generation timing) was narrowed, not resolved, post-Sprint-13**: both public entry points generate the report exactly once, always before completion — confirmed spec-conformant, no inconsistency found. The open question is only whether `AssessmentStatus::Completed` being permanently unreachable on any real report is intended; no forcing function currently requires deciding it. Do not treat this as license to change generation timing.
- **GOV-002 (Runtime Invariant Reconciliation) is now Resolved** (2026-07-24) — all fourteen Runtime Invariants confirmed to conform to the implementation, no implementation change made or required.
- **ADR-0011 (`AssessmentReport` Ownership Correction)**, accepted 2026-07-24, supersedes ADR-0003 solely with respect to one illustrative example — it does not change Assessment's status as sole aggregate root or any binding rule ADR-0003 otherwise states.
- Documentation staleness between closeouts is a tracked workflow-improvement goal, **not** a mandatory per-phase `PROJECT_STATUS.md` update requirement — this was explicitly proposed and explicitly declined at Sprint 5 Closeout. Continue full reconciliation at sprint close; do not proactively edit `PROJECT_STATUS.md` after every individual phase as a blanket habit.
- No new external crate dependency without explicit authorization. `zip` (Sprint 4) and `roxmltree` (Sprint 7) are the two authorized since Sprint 0; none is currently pending.

---

# Immediate Priorities

**Sprint 14 is complete** — implemented, reviewed, and closed out. The post-Sprint-14 Governance Reconciliation cycle (GOV-002, GOV-015, ADR-0011, Repository Reconciliation Packages A and B) is also complete, entirely at the documentation level. **Sprint 15 is not yet scoped.** Nothing is authorized to begin. No capability proposal has yet been classified through the Sprint 12 Capability Identity procedure and committed to the repository for a next Sprint — named candidates (Lua Analysis, still hard-blocked on GOV-014; Storage follow-ons such as cross-mod validation or MKB accumulation; the Extension Layer, still dormant) are all unscoped, not pre-selected. Scoping one is a Chief Architect sequencing decision, not yours to make. If asked to plan the next Sprint, prepare a Capability Definition and implementation plan for Chief Architect review, per `PROJECT_HANDOFF_v1.1.md` Section 5's canonical workflow, before any implementation.

If asked to simply "continue" without a specific scope, the correct response is to ask what the Chief Architect wants scoped — not to guess and start implementing one. Also worth noting: the Platform Architecture Track and Product & Interaction Design Track (`docs/platform/`, `docs/product-design/`, `docs/interaction-design/`) completed independently between 2026-07-28 and 2026-08-01 — real, substantial repository work, but not Sprint work, and not yet reconciled with this Sprint lineage. Do not assume either one implicitly authorizes or scopes an implementation Sprint.

---

# Open Engineering Risks

- **GOV-008 (`AssessmentService` public API evolution) has now aged across eleven Sprints untouched.** Neither Sprint 13 (Storage) nor Sprint 14 (`modiq-common` retirement) touched either entry point's signature, generating no new evidence toward it. The two-entry-point stopgap works, but is explicitly a stopgap; if a future Sprint's implementation pressure produces new evidence bearing on it, report that evidence rather than resolving GOV-008 informally.
- **GOV-013 (`FindingSeverity` kind/severity conflation) remains open, deliberately**, even with four Rules now dispatched — no Rule has ever assigned `BestPractice`, so the specific tension it raises remains unexercised. Sprint 11's `FindingSeverity::Error` use is recorded as relevant evidence for a future review only.
- **GOV-014 (Lua Fixture Acquisition Governance) blocks Lua Analysis's own Architecture Evaluation** until provenance/licensing/storage questions for real, human-acquired Lua scripts are resolved (INV-001).
- **GOV-001 was narrowed, not resolved, post-Sprint-13**: whether a persisted report's `AssessmentStatus::Completed` being permanently unreachable was an intended consequence remains open; no forcing function currently requires an answer.
- **Resolved since this document's last revision, no longer open:** GOV-002 (Runtime Invariant Reconciliation) and GOV-015 (ADR-0003's `AssessmentReport` ownership description), both 2026-07-24, plus GOV-003 (`modiq-common`, retired Sprint 14). None required a Rust source change.
- **Missing `Display`/`Serialize` for Runtime identity/enum types** remains flagged and unscheduled as of Sprint 6; not reconfirmed against the repository in the Sprints since — worth an explicit re-check before treating it as still-current, per this project's own "verify before advancing" discipline, rather than carrying it forward from memory.
- **`modiq-engine` does not re-export `AssessmentReport`** — as of Sprint 13, three independent consumers (`apps/sandbox`, `modiq-cli`, and `modiq-storage`'s own equivalent relationship) each depend on `modiq-report` directly to name the type. This reaches this project's own usual three-point convergent-evidence bar (GOV-004's own precedent) for the first time — worth a fresh look, not a decision made here.
- **Git tag hygiene remains unresolved**: `v0.1.0`, `v0.1.0-alpha`, `v0.2.0`, `v0.2.0-alpha`, `v0.3.0` predate and collide oddly with Engineering Release numbering. Sprints 10–12 introduced a new `sprintN-complete` tag convention; whether to extend it to later Sprints is a standing open question, not something to decide unilaterally.

---

# Engineering Expectations

- **Every task concludes with `cargo fmt`, `cargo check --workspace`, `cargo test --workspace`** — both the root workspace and, independently, `apps/sandbox/src-tauri`'s own workspace whenever anything you touched could plausibly affect it. Zero warnings, not just zero errors, is the bar — a `dead_code` warning surfaced mid-Sprint-5 specifically because a new Rule was scoped as "implemented but not yet wired in" without matching the public-unit-struct shape that makes that state warning-free; check for this pattern whenever you deliberately leave something unwired for a later phase.
- **Real I/O, real fixtures, never mocks** — the established discipline for anything touching the filesystem, an archive, or any future external content.
- **Determinism claims get their own direct test** — never assume an ordering guarantee is covered by a "repeated identical call" test; prove arrival-order or input-order independence explicitly when that's the actual claim being made (Sprint 5 Phase 5's own lesson).
- **Every implementation report includes an Assessment Examples section** when the work touches Rule Engine / Evidence-to-Finding-to-Recommendation behavior — concrete, real, test-verified before/after transformations, not abstract description. Standing convention since Sprint 5 Phase 3, confirmed by the Chief Architect as a permanent addition to the reporting format, not a one-off.
- **Governance/specification drafting follows a two-step discipline**: stage the proposed text in its own document (mirroring `PROPOSAL_GOV-011.md`, `GOV-012_AND_FINDINGSEVERITY_PREPARATION.md`) for review, and only amend the canonical document (`GOVERNANCE.md`, a frozen spec) after explicit approval. This has scaled cleanly from large governance resolutions to small ones — use it by default, not only for big decisions.
- **Do not commit until explicitly told to; do not push until explicitly told to.** Both have been separately authorized, separately, at every phase and closeout to date — never assume prior authorization carries forward to new work.

---

# Standard Sprint Execution

**Canonical location:** `PROJECT_HANDOFF_v1.1.md`, Section 5, "The Permanent Engineering Workflow" — the repository's single authoritative Sprint lifecycle (Capability Definition → Architecture Evaluation → Architectural Resolution → Implementation Authorization → Implementation → Validation → Implementation Report → Architectural Conformance Review → Commit → Merge → Repository Closeout), now preceded by the Sprint 12 Capability Identity procedure for any brand-new capability. This section no longer restates it; it previously did, under older terminology ("Sprint Planning," "Authorization," "Architecture Review"), and that copy had already drifted from the canonical version before this consolidation (`ENGINEERING_WORKFLOW_CONSOLIDATION_STUDY.md`).

What follows is the Lead Engineer's own execution checklist against those stages, not a second definition of them:

1. Prepare a Capability Definition and implementation plan.
2. Submit for Architecture Evaluation and Architectural Resolution.
3. Confirm every open architectural question has been explicitly marked Accepted, Rejected, or Deferred — none may carry forward silently.
4. Do not begin Implementation until Implementation Authorization is explicitly granted.
5. Implement only the authorized scope.
6. Complete Validation continuously as work proceeds, not only at the end:
   - `cargo fmt`
   - `cargo check --workspace`
   - `cargo test --workspace`
   - Sandbox validation (`apps/sandbox/src-tauri`, where applicable)
7. Produce an Implementation Report summarizing:
   - completed work
   - validation results
   - architectural observations
   - governance observations
   - recommendations
8. Await Architectural Conformance Review.
9. After approval, proceed to Commit, Merge, and Repository Closeout — each separately authorized, never assumed to carry forward from a prior authorization.

---

# Final Assessment

The repository is in a clean, fully verified, fully reconciled state: working tree clean, both workspaces green with zero warnings (253/253 root, 9/9 Sandbox, independently reverified during this revision), documentation synchronized as of HEAD, and every Governance Register item (15 total: 11 Resolved, 4 Open) either Resolved or deliberately, correctly Open. Sprint 14 closed with zero unresolved implementation work; the subsequent Governance Reconciliation cycle (GOV-002, GOV-015, ADR-0011) and Repository Reconciliation Packages A and B are also both complete. The Platform Architecture Track and Product & Interaction Design Track are separately complete but unreconciled with this Sprint lineage, and are not this document's concern. The next session's first action should be confirming the next Sprint's scope with the Chief Architect — no capability proposal has yet been classified through the Capability Identity procedure, and nothing should be assumed by default.

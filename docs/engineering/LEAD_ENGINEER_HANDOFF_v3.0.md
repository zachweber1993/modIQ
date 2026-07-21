# Lead Software Engineer Handoff v3.0

| Property | Value |
|----------|-------|
| **Document** | LEAD_ENGINEER_HANDOFF_v3.0.md |
| **Project** | modIQ |
| **Purpose** | Role-specific operational handoff for the next Lead Software Engineer session — assumes `PROJECT_HANDOFF_v1.0.md` has already been read |
| **Prerequisite** | `docs/engineering/PROJECT_HANDOFF_v1.0.md` — **read that first.** This document does not repeat product vision, architecture, governance history, sprint history, or architectural principles; all of that now lives there. |
| **Supersedes** | `LEAD_ENGINEER_HANDOFF_v2.0.md` (retained as a historical record; not rewritten) |
| **As of** | 2026-07-21, following Sprint 6 (CLI wiring, `modiq-report` scaffold retirement), implemented, reviewed, and merged; Repository Closeout in progress |
| **Branch** | `feature/runtime-implementation` |
| **HEAD** | `29657df` |

---

# Why This Document Exists Separately From PROJECT_HANDOFF_v1.0.md

`PROJECT_HANDOFF_v1.0.md` describes modIQ independent of role — what it is, how it's organized, what's been decided, what's been built. This document describes what it means to *operate as the Lead Engineer on it right now*: your responsibilities, the constraints currently binding your work, what's immediately actionable, and the standards every task is held to. `CHIEF_ARCHITECT_HANDOFF_v1.0.md` is produced the same way, against the same prerequisite.

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
| HEAD | `29657df` — Sprint 6's merge commit (implementation: `397707f` on `feature/sprint6-cli`) |
| Working tree | Clean, pushed, in sync with `origin/feature/runtime-implementation` |
| Current milestone | Sprint 6, implemented and merged; Repository Closeout in progress |
| Workspace crates | 9, unchanged in count since Sprint 3 |
| Root workspace tests | 172, zero flaky, zero ignored, zero warnings |
| Sandbox tests | 6, independent workspace, zero warnings |
| Documentation status | `GOVERNANCE.md`, `DataModel.md`, `PROJECT_STATUS.md`, `CHANGELOG.md`, `CrateRoadmap.md`, `ENGINEERING_LOG.md`, `docs/README.md`, and the three role handoff documents all reconciled as of HEAD during this Sprint 6 Closeout session — verified directly, not carried over from any prior report. A formal `ENGINEERING_RELEASE_0.6.md` has not yet been produced. |

---

# Chief Architect Decisions Currently In Force

These are binding on any work you do until explicitly revisited:

- `AssessmentService` remains the sole orchestration boundary (GOV-004). No intra-engine service objects.
- Collector selection and Rule dispatch are both explicit and inline — no dispatcher, registry, provider, factory, trait hierarchy, or plugin mechanism, for as long as the current small number of concrete cases persists (GOV-004, GOV-012). This has now been affirmed independently at least six times across two different subsystems (`PROJECT_HANDOFF_v1.0.md`, Section 6, Principle 1) — do not propose an abstraction without a genuine second-or-later concrete case already in hand.
- `RuleEngine::evaluate` dispatches `EvidencePresenceRule` then `StructuralDuplicationRule`, in that fixed order, returning `Vec<RuleOutcome>`; both fire independently, no suppression model (GOV-012).
- `FindingSeverity` (`Error`/`Warning`/`Informational`/`BestPractice`) is unchanged and **must stay unchanged** until GOV-013 is revisited with real evidence from a third Rule — do not restructure this type speculatively, and do not silently assign `BestPractice` to a new Rule without first checking whether that's actually a kind-classification need GOV-013 anticipated.
- `modiq-report`'s four scaffold types (`FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, `ReportFormatter`) were **deleted at Sprint 6, under explicit, separate Chief Architect authorization** — no longer a pending decision. `AssessmentReport` remains the crate's only content, and remains the canonical report model unless future architecture, justified by implementation evidence, replaces it.
- The `AssessmentService` execution contract (both entry points, `AssessmentInput`, `AssessmentReport`, the public error model) is the approved boundary; GOV-008 remains deliberately unresolved and unblocking — do not propose changing either entry point's signature as a side effect of unrelated work.
- Documentation staleness between closeouts is a tracked workflow-improvement goal, **not** a mandatory per-phase `PROJECT_STATUS.md` update requirement — this was explicitly proposed and explicitly declined at Sprint 5 Closeout. Continue full reconciliation at sprint close; do not proactively edit `PROJECT_STATUS.md` after every individual phase as a blanket habit.
- No new external crate dependency without explicit authorization; none is currently authorized.

---

# Immediate Priorities

**Sprint 6 is complete** — implemented, reviewed, merged into `feature/runtime-implementation`, and now administratively closed out (this document reconciled as part of that Closeout). **Sprint 7 is not yet scoped.** Nothing is authorized to begin. Of the three original Sprint 6 candidates (`PROJECT_HANDOFF_v1.0.md`, Section 10), only XML inspection remains undone — but scoping it, or something else, is a Chief Architect sequencing decision, not yours to make. If asked to plan Sprint 7, prepare implementation plans for Chief Architect review (mirroring `SPRINT6_IMPLEMENTATION_PLAN.md`'s own shape, including its Authorization Record) before any implementation, exactly as Sprint 6 itself was handled. Implementation requires Chief Architect authorization.

If asked to simply "continue" without a specific scope, the correct response is to ask what the Chief Architect wants scoped — not to guess and start implementing one. Also worth surfacing proactively: a formal `ENGINEERING_RELEASE_0.6.md` record, matching every prior Sprint's own convention, has not yet been produced.

---

# Open Engineering Risks

- **GOV-008 has now aged across four Sprints (3, 4, 5, 6) untouched.** Sprint 6 specifically reused `execute_from_assessment_input` exactly as designed and was confirmed, at scoping time, not to generate new evidence toward it. The two-entry-point stopgap works, but is explicitly a stopgap; if a future Sprint's implementation pressure produces new evidence bearing on it, report that evidence rather than resolving GOV-008 informally.
- **Missing `Display`/`Serialize` for Runtime identity/enum types has now been flagged in seven consecutive release records** (Sandbox Phase 2 through Sprint 6) without ever being scheduled. Sprint 6 explicitly declined to fold this in despite `modiq-cli` being the first text-only consumer, per direct Chief Architect authorization. If you're asked to survey small, low-risk cleanup candidates, this is still the most repeatedly-named one on record.
- **Resolved at Sprint 6, no longer open:** the Reporting scaffold-retirement recommendation — the four types are deleted.
- **A new, minor architectural item from Sprint 6, not yet a Governance Register item:** `modiq-engine` does not re-export `AssessmentReport`, so both real consumers of `AssessmentService` (the Sandbox and, since Sprint 6, `modiq-cli`) independently depend on `modiq-report` directly just to name the type. Two data points so far — this project's own convergent-evidence bar has favored three (GOV-004). Worth watching for a third occurrence rather than proposing a fix from two.
- **`modiq-knowledge` has gone six Sprints with zero implementation and zero forcing function**, including through two Sprints (5 and 6) that each added real capability elsewhere (a second Rule; a real CLI) without needing it. Not urgent, but worth another explicit look if a third Rule is ever scoped and also doesn't need it.
- **Git tag hygiene remains unresolved**: `v0.4.0` and `v0.5.0` are both available untagged, and no Engineering Release 0.6 (or corresponding tag) exists yet at all; whether to start tagging Engineering Releases going forward is a standing open question, not something to decide unilaterally.

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

Every implementation sprint follows this sequence:

1. Prepare an implementation plan.
2. Submit the plan for Chief Architect review.
3. Do not begin implementation until authorization is explicitly granted.
4. Implement only the approved scope.
5. Complete validation:
   - `cargo fmt`
   - `cargo check --workspace`
   - `cargo test --workspace`
   - Sandbox validation (`apps/sandbox/src-tauri`, where applicable)
6. Produce an implementation report summarizing:
   - completed work
   - validation results
   - architectural observations
   - governance observations
   - recommendations
7. Await Chief Architect implementation review.
8. After approval, finalize documentation, commit, push, and close the sprint.

This is the same permanent lifecycle `CHIEF_ARCHITECT_HANDOFF_v1.0.md` Section 10 describes from the review side (Sprint Planning → Chief Architect Review → Authorization → Implementation → Validation → Implementation Report → Architecture Review → Sprint Closeout); this section states it from the execution side.

---

# Final Assessment

The repository is in a clean, fully verified, fully reconciled state: working tree clean, both workspaces green with zero warnings (172/172 root, 6/6 Sandbox), documentation synchronized as of HEAD, and every governance item either Resolved or deliberately, correctly Open. Sprint 6 closed with zero unresolved implementation work; a formal `ENGINEERING_RELEASE_0.6.md` record remains the one outstanding administrative item, named explicitly rather than silently assumed complete. The next session's first action should be confirming Sprint 7's scope with the Chief Architect — not assuming XML inspection by default, even though it is the only named candidate remaining.

# Sprint 6 Planning Document

| Property | Value |
|----------|-------|
| **Document** | SPRINT6_PLANNING_DOCUMENT.md |
| **Project** | modIQ |
| **Purpose** | Planning document — recommend candidate Sprint 6 scope(s) for Technical Director authorization |
| **Prepared by** | Engineering (Lead Software Engineer session), for Technical Director review |
| **As of** | 2026-07-21, following Sprint 5 Closeout (Engineering Release 0.5), verified directly against HEAD `13781af` |
| **Status** | **Planning only. No code changed. No Runtime, Engine, Collection, Rules, Reporting, or Sandbox modified. No ADRs created. No Governance changed. No documentation outside this file touched.** |
| **Scope authorization** | None. Sprint 6 is not yet scoped. Nothing in this document is authorized for implementation. |

---

# Why This Document Exists

`PROJECT_HANDOFF_v1.0.md` and `LEAD_ENGINEER_HANDOFF_v3.0.md` both state that Sprint 6 has intentionally not been scoped and that scoping which roadmap candidate (or candidates) to pursue, and in what order, is a Technical Director sequencing decision. This document verifies the repository is in the state those handoffs describe, reviews the three roadmap candidates on record, and recommends a scope for review — mirroring `SPRINT5_IMPLEMENTATION_PLAN.md`'s own shape, per `LEAD_ENGINEER_HANDOFF_v3.0.md`'s instruction for how to handle an unscoped Sprint 6 request.

---

# Verification Performed

Rather than carry forward the handoff documents' own stated state, the following was checked directly against the repository:

| Claim | Verification | Result |
|---|---|---|
| Working tree clean, in sync with origin | `git status` | Confirmed — clean, up to date with `origin/feature/runtime-implementation` |
| HEAD is Sprint 5's close | `git log -1` | HEAD is `13781af`, one commit past the handoffs' stated `fbef863` |
| Nothing but documentation changed since Sprint 5 close | `git log fbef863..HEAD --stat` | Confirmed — two commits (`f9429e9`, `13781af`), both producing the handoff documents themselves; zero crate files touched |
| 162 root workspace tests, zero failures | `cargo test --workspace` | Confirmed — 43 (`modiq-collection`) + 16 + 3 (`modiq-engine` unit + integration) + 3 (`modiq-report`) + 15 (`modiq-rules`) + 82 (`modiq-runtime`) = 162, all passing |
| Zero warnings | `cargo check --workspace` | Confirmed — clean build, no warnings |
| Sandbox: 6 tests, zero warnings | `cargo test` in `apps/sandbox/src-tauri` | Confirmed — 6/6 passing, zero warnings |
| `modiq-cli` remains unwired scaffolding | Read `main.rs`, `app.rs`, `commands/*.rs`, `Cargo.toml` | Confirmed — `main.rs` only prints a string; `AssessCommand`/`HelpCommand`/`VersionCommand`/`Application` are one-line unit structs; **zero dependencies declared**, not even an argument-parsing crate |
| Reporting scaffold-retirement evidence is complete | Read `SPRINT5_PHASE4_REPORTING_INVESTIGATION.md` in full | Confirmed — zero construction sites for all four types; Sandbox's actual rendering logic does not need any of them, checked directly against Sprint 5's own new severity differentiation |

No discrepancy was found between the handoff documents and the live repository.

---

# Open Governance Items Relevant to Sprint 6 Scoping

| Item | Status | Relevance |
|---|---|---|
| GOV-008 (`AssessmentService` API Evolution) | Open, 3 sprints | CLI wiring is not expected to generate new architectural evidence regarding `AssessmentService`'s public API. Because it intentionally reuses the Sandbox's existing integration pattern, it exercises the same public interface rather than introducing a new usage model. Any future evidence relevant to GOV-008 would need to arise from concrete implementation requirements discovered during subsequent platform evolution, not from the existence of a second consumer alone. |
| GOV-013 (`FindingSeverity` conflation) | Open, deliberately | Requires a **third Rule** to move. None of the three candidates below provides that on its own. Expected to remain Open after Sprint 6 regardless of scope chosen. |
| GOV-001 / GOV-002 / GOV-003 | Open since v0.1.0-alpha | Dormant, non-blocking, unaffected by any candidate below. |
| Reporting scaffold retirement (not a GOV item) | Investigated, recommended, **not authorized** | Fully resolved evidentially; only a governance/authorization decision is outstanding. |

---

# Candidate 1 — XML Inspection (Third Evidence Collector)

**What it is:** The next Collector in the platform's original sequencing (filesystem → archive → XML), per `PROPOSAL_FILESYSTEM_COLLECTION.md`'s own stated ordering. Deliberately deferred through Sprint 5 so it would build on a mature Rule Engine rather than drive its design.

**Why it's ready now:** The condition that deferred it — Rule Engine maturity — is satisfied (two real Rules, `GOV-012`-resolved multi-Rule dispatch).

**Risks:**
- Almost certainly requires a **new external crate dependency** (an XML parser). No dependency is currently authorized (`LEAD_ENGINEER_HANDOFF_v3.0.md`: "No new external crate dependency without explicit authorization"). This is a real, distinct authorization decision, separate from Sprint scoping itself.
- Largest of the three candidates. Following this project's own proposal-then-implement discipline (never once bypassed for a governance-relevant capability), it needs its own boundary-scoping proposal before implementation — likely its own GOV items, analogous to GOV-009/010 (filesystem) and GOV-011 (archive).
- Scope discipline risk: the filesystem Collector's own proposal explicitly drew the line at "discovering `modDesc.xml` exists" vs. "parsing what's inside it" — an XML collector must decide, deliberately, how far past mere discovery it goes, or it risks quietly absorbing Knowledge Domain territory it isn't ready for.

**Dependencies:** None blocking — Rule Engine maturity is the only stated prerequisite, and it's met.

**Expected outcome:** Third real Collector; likely a new `EvidenceCategory`; does **not** by itself provide GOV-013's forcing function (that needs a third Rule, not a third Collector).

---

# Candidate 2 — CLI Wiring

**What it is:** Wire the existing `modiq-cli` scaffold (`AssessCommand`, `HelpCommand`, `VersionCommand`) to `modiq-engine`, reusing the Sandbox's own proven thin-client pattern — calling `AssessmentService::execute_from_assessment_input` directly, no reimplementation of engine logic.

**Why it's attractive:**
- Smallest, most independent of the three candidates.
- `modiq-cli/Cargo.toml` currently declares **zero dependencies** — confirmed by direct inspection, not assumed. Wiring can plausibly be done with manual `std::env::args()` parsing, meaning it need not trip the standing no-new-dependency constraint at all (though if an argument-parsing crate is judged genuinely necessary during design, that would itself need separate authorization).
- De-risked further every sprint the Sandbox's own pattern goes unreinvented (`PROJECT_HANDOFF_v1.0.md`, Section 10).

CLI wiring is not expected to generate new architectural evidence regarding `AssessmentService`'s public API. Because it intentionally reuses the Sandbox's existing integration pattern, it exercises the same public interface rather than introducing a new usage model. Any future evidence relevant to GOV-008 would need to arise from concrete implementation requirements discovered during subsequent platform evolution, not from the existence of a second consumer alone.

**Risks:** Low. Main risk is scope creep — e.g., building CLI-specific output formatting that overlaps with the still-undecided Reporting scaffold question (Candidate 3). Should explicitly reuse `AssessmentReport`'s existing shape, not invent CLI-specific summarization.

**Dependencies:** None.

**Expected outcome:** A working, real CLI entry point; no new external dependency if scoped carefully; no expected bearing on GOV-008 (see rationale above).

---

# Candidate 3 — Reporting Scaffold Retirement

**What it is:** Delete `FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, `ReportFormatter` from `modiq-report`, per the Sprint 5 Phase 4 investigation's recommendation.

**Why it's ready now:** Investigation is complete, not partial. Zero construction sites across every Sprint to date; the one real consumer (`apps/sandbox/src/App.tsx`) confirmed not to need any of the three responsibilities, re-checked directly against Sprint 5's own new severity differentiation. Nothing about this candidate requires new design work — only an authorization decision that Sprint 5 Closeout explicitly deferred rather than declined.

**Risks:** Minimal. The only risk is procedural: per this project's own two-step governance discipline, the retirement itself should still be staged as an explicit decision (updating `GOVERNANCE.md`'s Crate Boundary Rules language for Reporting, if the Technical Director wants that recorded) before deletion, not treated as a pure code cleanup with no documentation trail.

**Dependencies:** None. Fully decoupled from Candidates 1 and 2.

**Expected outcome:** `modiq-report` reduced to only what's real (`AssessmentReport::generate`); one long-standing open item closed; test count unaffected (all four types were untested, by construction, since unused).

---

# Recommendation

**Primary scope: CLI Wiring (Candidate 2).**

Rationale: smallest, fully independent, no new external dependency required under a careful scope, and reuses an already-validated pattern rather than inventing one. This last point cuts both ways: it is precisely because CLI wiring reuses the Sandbox's existing integration pattern that it is low-risk — and, for the same reason, it is not expected to generate new architectural evidence regarding `AssessmentService`'s public API. Any future evidence relevant to GOV-008 would need to arise from concrete implementation requirements discovered during subsequent platform evolution, not from the existence of a second consumer alone.

**Bundle if Sprint 6 capacity allows: Reporting Scaffold Retirement (Candidate 3).**

Rationale: zero remaining investigation risk, minimal implementation cost, fully decoupled from Candidate 2, and closes an item that has sat authorized-to-consider-but-not-acted-on since Sprint 5 Closeout. Bundling avoids a fully-resolved, low-risk item drifting to a further sprint for no substantive reason.

**Defer: XML Inspection (Candidate 1) to a future sprint of its own.**

Rationale: not because it isn't ready — its stated precondition (Rule Engine maturity) is satisfied — but because it is larger than the other two combined, requires its own proposal-then-implement cycle and likely its own new governance items, and surfaces a distinct external-dependency authorization question that deserves to be decided on its own, not folded into a sprint carrying two unrelated items already. Recommend it be the presumptive lead candidate for Sprint 7, pending its own dedicated scoping proposal.

**Expected outcome of the recommended Sprint 6, if authorized as scoped above:**
- A real, working `modiq-cli` calling `modiq-engine` directly, with test coverage.
- `modiq-report` reduced to only its real, used content.
- No new external crate dependency.
- No resolution attempted on GOV-008 or GOV-013, and no new evidence toward either is expected from this scope as designed — both remain open, by design. If Sprint 6's own implementation nonetheless surfaces genuine new evidence from concrete requirements encountered along the way, that evidence will be reported, not acted on unilaterally.

**This document authorizes nothing.** Per `LEAD_ENGINEER_HANDOFF_v3.0.md`, implementation does not begin until the Technical Director reviews this document and explicitly authorizes a scope — whether the recommendation above, a different combination of the three candidates, or something not listed here.

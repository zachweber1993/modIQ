# Lead Software Engineer Handoff v1.0

| Property | Value |
|----------|-------|
| **Document** | LEAD_ENGINEER_HANDOFF_v1.0.md |
| **Project** | modIQ |
| **Purpose** | Operational handoff from the outgoing Lead Software Engineer to the incoming one — not a project summary |
| **Audience** | The next Claude session assuming the Lead Software Engineer role on modIQ |
| **As of** | 2026-07-19, following Engineering Release 0.3 |
| **Branch** | `feature/runtime-implementation` |

---

## Read This First

You are inheriting a mature, disciplined codebase from an engineer who held the line on architecture for nine implementation phases without a single boundary violation. The Technical Director is a separate party — real, existing, and continuing in that role. You are not inheriting their job. You are inheriting mine.

**Before you write a line of code:** the repository is not git-clean. 24 files are currently uncommitted — the entirety of Sprint 3 Phase 5, the governance-resolution session that preceded it, the Roadmap Review, the Filesystem Collection proposal, and the Engineering Release 0.3 preparation itself, all sitting on top of commit `f1b6051` (`docs(engineering): record Sprint 3 Phase 4, resolve GOV-007`). Read it, verify it matches what this document and `ENGINEERING_RELEASE_0.3.md` claim, and get explicit direction on committing it before you touch anything else. This is the single most important operational fact in this handoff.

---

## 1. Project Overview

modIQ produces **deterministic, evidence-based, explainable assessments** of Farming Simulator mods (and, eventually, mod collections, savegames, and maps). It does not generate an opaque quality score. Every Assessment exists to answer three questions: does the mod work, why was this conclusion reached, and what can be done to improve it. Education and transparency are product goals, not secondary features — this shapes real engineering decisions, not just marketing copy (it's why Reporting's eventual "explanation generation" responsibility matters, and why Evidence Collection was designed to be traceable back to real filesystem facts rather than opaque analysis).

**Engineering philosophy**, consistently applied for nine phases, not just stated:

- Determinism over automation. Every entity's identity is a monotonic counter, never randomness. Every "determinism" test in the codebase compares content, never incidental identity.
- Explainability over convenience. Traceability fields (`evidence_ids`, `rule_reference`, `finding_ids`) exist before any consumer uses them.
- Readability over cleverness, maintainability over abstraction. No trait, no plugin system, no generic dispatch mechanism exists anywhere in this codebase that wasn't justified by a second concrete case actually needing it.
- Crate boundaries are load-bearing, not organizational. They have been pressure-tested — including through the introduction of an entirely new subsystem — and have never once been violated.

**Current maturity:** Documentation Release 2.1 (Frozen). Engineering Release 0.3 (approved). Nine workspace crates. The core pipeline (Runtime → Rule Engine → Reporting, orchestrated by Engine) is mature and stable. Evidence Collection — the platform's newest subsystem — has a proven architectural boundary and one working real capability (filesystem discovery). Four crates remain deliberately unbuilt scaffolding.

Do not attempt to redesign any of this. It is not yours to redesign — see Section 2.

---

## 2. Team Roles

### Technical Director

The Technical Director owns:

- Architecture and technical vision
- Engineering strategy and roadmap sequencing
- Governance — the Governance Register, its resolutions, and the Level 1–4 change-categorization process that decides what needs their approval
- System boundaries — crate responsibilities, dependency direction, subsystem ownership
- Architectural approvals — every ADR, every Documentation Release, every "yes, build this" decision
- Tradeoff decisions between competing engineering approaches
- Overall repository health and sequencing across engineering cycles

### Lead Software Engineer (you)

You own:

- Production implementation — writing the Rust code that realizes approved architecture
- Engineering execution — turning an approved design into working, tested software
- Test development — and, per this project's own track record, real test infrastructure when implementation genuinely needs it (temp directories, symbolic links, permission fixtures were all added this Sprint, for the first time, when Evidence Collection first needed to touch a real filesystem)
- Refactoring — within already-approved boundaries, never across them without raising the question first
- Documentation synchronization — keeping implementation-adjacent documentation (crate maturity tables, test counts, engineering logs) accurate as code changes, without rewriting Frozen specifications yourself
- Implementation planning and engineering recommendations — including proposing what should happen next, as `ENGINEERING_RELEASE_0.3.md`'s own Recommendation section does
- Surfacing architectural questions the moment implementation uncovers them — not resolving them yourself, not guessing, not picking the "obviously right" answer and moving on

**Implementation must never redefine architecture.** This is not a slogan on this project — it is the literal, working discipline that produced nine phases without drift. Every single time this project crossed a real architectural question (Evidence Collection's ownership, the Assessment Input model, the Collection Error Model, whether `execute`'s signature should change), the pattern was identical: an Architecture Review proposal, written in plain conceptual language with no Rust types, submitted for Technical Director review, *then* implemented — never the reverse. When you hit a question like that, follow the same pattern. Examples of what that looks like are sitting in this repository right now: `docs/engineering/PROPOSAL_EVIDENCE_COLLECTION_BOUNDARY.md` and `docs/engineering/PROPOSAL_FILESYSTEM_COLLECTION.md`.

---

## 3. Current Repository State

| Property | Value |
|---|---|
| Workspace crates | 9 — `modiq-cli`, `modiq-collection`, `modiq-common`, `modiq-engine`, `modiq-knowledge`, `modiq-report`, `modiq-rules`, `modiq-runtime`, `modiq-versioning` |
| Subsystems | Runtime Domain, Rule Engine, Evidence Collection, Engine (orchestration), Reporting, Sandbox (dev application). Knowledge Domain, Version Profiles, and CLI are scaffolded, unimplemented. |
| Documentation Release | 2.1, Frozen |
| Engineering Release | 0.3, approved |
| Repository maturity | Mixed by design: core pipeline L3 and stable; Evidence Collection L2, young but proven; four crates L1, pure scaffold since Sprint 0 |
| Test count | 112 root workspace tests + 3 Sandbox tests = 115 total, zero flaky, zero ignored |
| Implementation status | Architecturally and functionally ready for the next capability. **Not git-clean** — see "Read This First," above. |
| Branch | `feature/runtime-implementation` |
| Last commit | `f1b6051` — everything since is uncommitted |

---

## 4. Architecture Summary

Treat every boundary below as **settled** — implementable against freely, not renegotiable without the Technical Director explicitly approving a revisit.

**`modiq-runtime`** — owns the Runtime Domain: `Assessment` (the aggregate root), `Evidence`, `Finding`, `Recommendation`, their identity, and all lifecycle invariants. Sole mutator of everything it owns — no other crate ever mutates `Assessment` state directly. The leaf of the dependency graph: depends on nothing else internal to the platform. Never evaluates Rules, never generates Reports, never owns Knowledge, never orchestrates.

**`modiq-rules`** — the Rule Engine. Owns deterministic evaluation only. Consumes Evidence, produces Findings and Recommendations. Depends on `modiq-runtime` only. Never mutates `Assessment` directly, never constructs Evidence — that boundary was explicitly fought for in Sprint 3 Phase 1 and has held, unthreatened, through four subsequent phases of adjacent work, including the introduction of the entire Evidence Collection subsystem.

**`modiq-collection`** — Evidence Collection. Owns producing Evidence from an Assessment Input. Depends on `modiq-runtime` only; nothing depends on it except `modiq-engine`. Never evaluates what it produces, never mutates `Assessment`, never owns orchestration, never acquires its own input (that's an application-layer concern). Currently has exactly one real capability: deterministic filesystem discovery of files and directories, with no content parsing of any kind.

**`modiq-report`** — Reporting. Owns the read-only `AssessmentReport` snapshot. Performs no analysis; reflects state that already exists. Depends on `modiq-runtime` only.

**`modiq-engine`** — orchestration only. Composes Runtime, Rules, Reporting, and Collection into callable operations (`AssessmentService::execute` and `execute_from_assessment_input`). Owns zero runtime state, implements zero business logic, generates zero reports itself. Notably: three separate subsystems (Rule Engine, Reporting, Evidence Collection) are all composed by *direct instantiation* — the four `EngineAPI` stub services (`KnowledgeService`, `ReportingService`, `RuleEvaluationService`, `VersionProfileService`) have never been used, not once, across the platform's entire history. Don't assume they're load-bearing scaffolding waiting for you to wire something into — that assumption is exactly what GOV-004 (open) is asking the Technical Director to resolve.

**`apps/sandbox`** — a thin, non-production developer visualization tool. Owns zero domain logic. Depends directly on `modiq-runtime`, `modiq-report`, and `modiq-engine` (its own separate Cargo workspace, deliberately not a member of the root one) — but *not* directly on `modiq-collection`, which it reaches only transitively through `modiq-engine`, exactly as the architecture specifies. Currently exercises the real filesystem collector against a fixed, checked-in fixture directory; has no file-picker, drag-and-drop, or other input UI, by deliberate, repeatedly-reaffirmed design.

**`modiq-knowledge`, `modiq-versioning`, `modiq-cli`, `modiq-common`** — all four remain exactly as scaffolded at Sprint 0: empty marker structs, zero logic, zero real usage. This is not neglect. Each is correctly waiting for a concrete second case to justify building it out (a second Rule for Knowledge, a version-varying behavior for Versioning, nothing blocking CLI except sequencing, and — for `modiq-common` specifically — no evidence in nine phases that it's needed at all).

**Settled decisions, explicitly:** Evidence flows one direction only (Collection produces, Rules consume, never reversed). Runtime is the sole mutator of Assessment. Dependency direction is strictly downward with `modiq-runtime` as the leaf. No Rule trait, no Collector trait, no plugin/dispatch mechanism exists anywhere — both were explicitly evaluated and declined, twice, under real pressure to build them, on the principle that a capability should justify an abstraction, not the reverse.

---

## 5. Governance Summary

Ten Governance Register items exist in `GOVERNANCE.md`; this section is not a duplicate of it — it's what the resolved and open items actually mean for how you write code.

**Resolved, and what they mean for implementation:**

- **GOV-005 / GOV-006** — `Finding` and `Recommendation` must reference at least one Evidence/Finding item; construction rejects empty reference lists. This is cardinality only. **Referential integrity is not enforced** — a reference to something that doesn't exist in the Assessment is silently dropped during resolution, never an error. Don't assume a resolved reference is guaranteed to be real.
- **GOV-007** — Evidence Collection's first implementation was deliberately scoped to the smallest slice that proved the architecture. This is the standing default for any new subsystem work you do: prove the boundary minimally before building the real capability out.
- **GOV-009** — Assessment Input: files and directories are both valid; a non-existent path is an *Inaccessible* outcome, not an *Invalid* one (existence is checked at collection time, not construction); symbolic links are not traversed, as a Phase 5-specific boundary, not a permanent one; Collection consumes an Assessment Input but never creates or reinterprets one.
- **GOV-010** — Collection has exactly four architectural outcomes: Invalid Input (never begins), Inaccessible Input (aborts), Unsupported Input (aborts), Empty Collection (succeeds, zero Evidence, not an error). Do not invent a fifth without going back through the Technical Director.

**Open, and what they mean for implementation:**

- **GOV-001** (Report generation timing) — Reports are currently generated *before* Assessment completion. Don't change this ordering without approval.
- **GOV-002** (Runtime invariant doc reconciliation) — a documentation-completeness gap, not a functional one; low implementation relevance.
- **GOV-003** (role of `modiq-common`) — do not invent a use for this crate to "resolve" the open item. Let a real, shared need arise first.
- **GOV-004** (Engine service granularity) — see the `modiq-engine` note in Section 4. Three real data points now argue for reconciling or removing the unused stub services; this is ripe for a Technical Director decision, not something to act on unilaterally.
- **GOV-008** (AssessmentService API evolution) — two parallel entry points (`execute`, `execute_from_assessment_input`) exist specifically to avoid deciding this. A third parallel entry point would be the signal that routing around it has stopped working — at that point, raise it rather than adding a fourth.

---

## 6. Sprint History

**Sprint 0** — Foundation. Workspace scaffolding, crate hierarchy, module structure, governance documentation. Objective: implementation readiness, not implementation itself. Lesson: the discipline invested here (crate boundaries, governance process) is exactly what made every later phase possible without drift.

**Sprint 1** (Engineering Release v0.1.0-alpha) — The first complete, deterministic, end-to-end pipeline, using content-free marker types for Evidence, Finding, and Recommendation. Objective: prove the pipeline's *shape* before investing in real content. Architectural validation: `AssessmentService::execute` as the single orchestration entry point, still the pattern in use today.

**Sprint 2** (Engineering Release 0.2) — Gave Evidence, Finding, and Recommendation real field content, process-local identity, and constructor validation; extended `Assessment` with relationship-resolution methods. Lesson: the resulting Runtime entity design pattern (aggregate ownership, entity identity, value objects, opaque references, constructor validation, identity-based equality, aggregate-owned resolution, governance-controlled invariants, determinism) was formalized in ADR-0007 and is still the template every new Runtime-adjacent type follows. Two invariant questions were identified and deliberately left open (GOV-005, GOV-006) rather than resolved informally — the first real instance of a pattern this project would repeat successfully several more times.

**Sprint 3** (Engineering Release 0.3, five phases) — Objective: give the platform a real, non-synthetic source of Evidence. Phase 1 wired the Sandbox to the real pipeline. Phase 2 resolved GOV-005/006. Phase 3 designed and approved the entire Evidence Collection subsystem boundary (ADR-0008, ADR-0009, `EvidenceCollection.md`, Documentation Release 2.1). Phase 4 built the minimal `modiq-collection` crate proving that boundary. An interstitial governance session resolved GOV-009/010 for the filesystem case. Phase 5 implemented the first real collector: deterministic filesystem discovery, the four-outcome error model, Collection Atomicity, and the Symbolic Link Policy. Major lesson, validated three separate times this Sprint: the proposal-first workflow produces implementations that need zero architectural rework afterward.

Full detail for all of the above lives in `ENGINEERING_RELEASE_0.3.md` and its predecessor `docs/releases/ENGINEERING_RELEASE_0.2.md` — this section is an index, not a replacement.

---

## 7. Current Engineering Position

**Implemented:** the complete Assessment pipeline, from real filesystem Evidence Collection through deterministic Rule evaluation to Report generation, exercised by both the root workspace's own tests and the Sandbox as a real (if thin) application.

**Validated:** every crate boundary, under real pressure, including a brand-new subsystem's introduction. The proposal-first workflow. Deterministic filesystem traversal on the first implementation attempt. Collection Atomicity falling out of ordinary `Result`/`?` control flow with no purpose-built mechanism. The additive-API-growth pattern as a genuine way to extend the platform without prematurely resolving an open governance question.

**Future work:** ZIP, XML, Lua, manifest, and dependency-analysis collectors. Rule abstraction (deliberately un-justified so far). Knowledge Domain integration. Version Profile integration. CLI wiring. Persistence. Resolution of the five still-open Governance Register items.

**Highest confidence:** `modiq-runtime` (82 tests, the most heavily exercised code in the platform) and the Evidence Collection boundary itself (proven twice now — once minimally in Phase 4, once for real in Phase 5).

**Where engineering attention should focus next:** see Section 10. In the immediate term, before any of that: the uncommitted backlog described in "Read This First."

---

## 8. Technical Debt

**Intentional technical debt** *(deliberate, correct to leave as-is)*: `EvidenceCollector::collect` has no unreachable error path — it was kept infallible until a real failure mode existed to design against, and Phase 5 closed that gap honestly rather than speculatively. GOV-008 remains open by design, with additive API growth as a working stopgap. No Rule trait, no Collector trait — both explicitly declined pending a real second case.

**Deferred enhancements** *(no urgency, no forcing function yet)*: reconciling the unused `EngineAPI` stub services and `modiq-rules`'s unused stub submodules (GOV-004-adjacent). Moving from ad hoc, explicitly-recorded documentation amendments to a full formal Documentation Release cycle for future architectural changes, if and when the pace of change justifies the heavier process.

**Future capabilities** *(scoped, sequenced, not started)*: ZIP traversal (next, most likely), XML inspection, Lua inspection, manifest analysis, dependency analysis, Rule abstraction, Knowledge Domain integration, Version Profile integration, CLI wiring, persistent assessment storage.

**Known limitations** *(real, will not resolve without deliberate work)*: no `Display`/`Serialize` implementation for Runtime identity/enum types — flagged across four consecutive release records now without being scheduled; no session has ever visually verified the Sandbox's actual UI; `v0.2.0` and `v0.3.0` already exist as git tags from unrelated earlier history, so Engineering Release 0.3 could not be tagged `v0.3.0` and remains untagged; adversarial-input test coverage exists only for what Phase 5 specifically needed (symlinks, permission denial) and will need real expansion once ZIP/XML parsing introduces untrusted binary content.

Each of these is explained in far more depth, with full reasoning, in `ENGINEERING_RELEASE_0.3.md`.

---

## 9. Guidance for Future Claude Sessions

This is the section to reread whenever you're unsure what to do next.

- **Respect established architecture.** Section 4's boundaries are settled. Build against them; don't renegotiate them in code.
- **Do not introduce speculative abstractions.** This project has said no to a Rule trait and a Collector trait, twice, under real pressure to build them "just in case." When you're tempted to generalize before you have two concrete cases, don't. Build the smaller thing.
- **Preserve crate boundaries.** Before adding a new dependency edge between crates, check it against `CrateRoadmap.md`'s dependency hierarchy and confirm the direction stays strictly downward. Verify by inspection (check the actual `Cargo.toml`, check what actually depends on what), not by assumption.
- **Keep documentation synchronized with implementation — narrowly.** Update what your own change makes stale (test counts, a crate's maturity level, a doc comment describing behavior you just changed). Do not rewrite Frozen specifications yourself; those go through the Technical Director and the Documentation Release process.
- **Surface architectural questions instead of resolving them independently.** If implementation reveals a conflict with documented architecture, a gap no existing governance item covers, or a genuine two-way fork with no obviously smaller option — stop, write it up as an Architecture Review proposal (conceptual language, no Rust types, following `PROPOSAL_EVIDENCE_COLLECTION_BOUNDARY.md` or `PROPOSAL_FILESYSTEM_COLLECTION.md`'s shape), and get it reviewed before writing implementation code against it.
- **Prefer small, well-tested implementations.** Every successful phase in this project's history did the smallest thing that proved the architecture, then stopped. Phase 4's synthetic collector and Phase 5's filesystem-only, no-parsing collector are the clearest examples.
- **Let implementation validate architecture — don't let it redesign architecture.** When something doesn't fit cleanly, that's information for the Technical Director, not license to quietly change the design while writing code.
- **Follow the cycle: Architecture → Governance → Documentation → Implementation → Verification → Technical Director Review.** This project's entire successful history is this cycle, executed correctly, repeatedly. Don't skip steps because a change "seems small."
- **Verification discipline.** Before considering any implementation work done: `cargo fmt`, `cargo check --workspace`, `cargo test --workspace` at the repository root, and the equivalent three commands inside `apps/sandbox/src-tauri` (its own, separate workspace). Zero warnings, zero failures, every time.
- **Commit discipline.** Only commit when explicitly asked to. But when you notice uncommitted work accumulating across sessions — as it currently has — say so plainly rather than letting it compound further.

**When to stop and return to the Technical Director**, concretely: when implementation would require crossing a crate boundary not already documented in Section 4; when a public API change would be genuinely breaking rather than additive (see GOV-008); when a Governance Register item's Resolution text doesn't actually cover the specific case implementation just hit; when the choice of what capability to build next hasn't been decided yet (that's a roadmap decision, not an implementation one); when two implementation approaches are both reasonable and neither is obviously the smaller, less speculative one.

---

## 10. Immediate Engineering Objective

`ENGINEERING_RELEASE_0.3.md`'s own Recommendation names the next logical capability: **a second real collector, most likely ZIP traversal**, with **CLI wiring** available as an independent, low-risk parallel track. This section explains why, without instructing how — that's for the Technical Director to scope.

**Why this comes next:** Filesystem Collection (Phase 5) just proved the exact pattern a second collector needs to follow — the same crate, the same `EvidenceCollector` shape, the same four-outcome error model. Both `PROPOSAL_EVIDENCE_COLLECTION_BOUNDARY.md` and `PROPOSAL_FILESYSTEM_COLLECTION.md` already name ZIP traversal as the immediate next step in Evidence Collection's own sequence. Nothing else on the roadmap — Rule abstraction, Knowledge integration, Version Profiles — has a forcing function yet; each is still correctly waiting for a second concrete case that doesn't exist.

**What architecture already supports it:** the `modiq-collection` crate exists. The Collector Contract in `EvidenceCollection.md` already describes what any Collector must and must not do, in terms general enough to cover an archive-aware one without modification. The dependency direction (`modiq-collection → modiq-runtime` only) doesn't need to change.

**What implementation should verify:** whether a ZIP collector genuinely fits inside the same `EvidenceCollector`/`collect()` shape Phase 5 established, or whether having a *second* real collector in hand (not a hypothetical one) finally provides the concrete case the Collector-dispatch question has been waiting for. Whether GOV-009's Assessment Input definition, resolved for the filesystem case, extends cleanly to "a location inside an archive" or needs a real extension. That a new external dependency (an archive-parsing crate — the first one any domain crate in this platform would ever take) is introduced deliberately, not incidentally.

**What to bring back to the Technical Director if uncertainty arises:** whether ZIP traversal needs its own Architecture Review proposal before implementation — it introduces the platform's first genuinely adversarial input surface (malicious or malformed archives), which nothing built so far has had to defend against. Any resource-limit or security policy question (zip-bomb handling, decompression limits). Whether a second real collector is the concrete case that finally justifies a Collector trait — don't decide that one yourself, even if it seems obvious in the moment.

**Before any of this begins:** commit the outstanding backlog. This is not part of the next capability — it's what has to happen before "the next capability" means anything in terms of a real, shared baseline.

---

## 11. Repository Health

**Strengths:** zero crate-boundary violations across nine implementation phases, including the introduction of an entirely new subsystem. A proposal-first workflow that has produced zero post-implementation architectural rework, three times this Sprint alone. Strong, disciplined test coverage with an explicit determinism-testing convention followed without exception. A Governance Register that has never let a decision slip through informally — every Level 3+ change went through the proposal → governance → documentation → implementation cycle.

**Risks:** the uncommitted backlog (immediate, operational, the most urgent item in this document). GOV-004 and GOV-008 aging with real implementation evidence now available that didn't exist when they were first raised. Documentation staleness as a demonstrated, recurring pattern across every release checkpoint so far — addressed for this cycle, not structurally solved. Four crates with zero validation since Sprint 0 (correctly deferred, but genuinely untested assumptions). Limited adversarial-input test coverage, about to matter more once ZIP/XML collectors introduce untrusted binary content.

**Engineering confidence:** high, for everything actually built — it has been tested, re-tested, and pressure-tested by real subsequent work, not just written once and left alone.

**Architectural confidence:** high, for the same reason — every major boundary in this codebase has survived contact with real implementation at least once, several of them (Runtime's aggregate pattern, the Rule Engine/Evidence Collection producer-consumer line) multiple times.

**Current project maturity:** mixed by design. The core pipeline is production-grade in rigor if not yet in product scope. Evidence Collection is young but already proven twice. Four crates are intentionally, correctly still at Sprint 0's starting line.

---

## 12. Final Engineering Assessment

I'm handing off a codebase I'd be comfortable building on myself, which is the actual bar, not a formality. My confidence in the repository is high: 115 tests, zero flaky, a governance process that has never once been quietly bypassed, and a five-phase Sprint that ended exactly where its own predecessor's exit criteria said the next one should begin. My confidence in the architecture is high for a specific, falsifiable reason — it has been tested against real implementation repeatedly, including one full new-subsystem introduction, and it held every time without needing to bend. My confidence in the engineering workflow itself is high, but conditional: it only works if the incoming engineer actually uses it — writes the proposal before writing the code, stops at a real fork instead of picking the answer that feels obvious, and treats "the Technical Director hasn't decided this yet" as a real blocker rather than an invitation to guess well.

Three things for you, specifically, before you do anything else:

1. **Commit the backlog first.** Read it, verify it against this document's and `ENGINEERING_RELEASE_0.3.md`'s claims, then either commit it or get explicit direction not to. Don't build on top of an uncommitted foundation without knowing that's what you're doing.
2. **Don't let "Sprint 4" become a big upfront plan.** This project's entire successful track record is small, individually-scoped, individually-approved phases — five of them last Sprint alone. That discipline is not incidental to the project's health; it is a large part of why the architecture is still intact after nine phases.
3. **The five open governance items are live questions, not archived ones.** GOV-004 and GOV-008 in particular now have real implementation evidence behind them that didn't exist when they were raised — that's worth surfacing to the Technical Director proactively, not waiting to be asked about.

Welcome to modIQ. The architecture is sound. Keep it that way.

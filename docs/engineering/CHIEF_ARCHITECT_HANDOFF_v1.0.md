# Chief Architect Handoff v1.0

> **An operating manual for the Chief Architect role on modIQ — how to lead the project, not a summary of what it contains.**

---

| Property | Value |
|----------|-------|
| **Document** | CHIEF_ARCHITECT_HANDOFF_v1.0.md |
| **Project** | modIQ |
| **Purpose** | Role-specific operating manual for the Chief Architect — assumes `PROJECT_HANDOFF_v1.0.md` has already been read |
| **Prerequisite** | `docs/engineering/PROJECT_HANDOFF_v1.0.md` — **read that first.** This document does not repeat product vision, architecture description, governance register contents, sprint history, or repository organization; all of that lives there. |
| **Supersedes** | `TECHNICAL_DIRECTOR_HANDOFF_v3.0.md` — retained in git history under the role's prior title. That document itself superseded a Sprint 4-era draft (dated 2026-07-20, frozen at "Engineering Release 0.3 · Sprint 4 · Archive Collection Foundation," predating Sprint 4 Phases 3C/3D, GOV-011's full resolution, and all of Sprint 5) and `TECHNICAL_DIRECTOR_HANDOFF_v2.2.md` before it. All retained in git history; not rewritten in place. This document is a role-title transition — Technical Director and Chief Architect are the same role and the same scope of authority, renamed — not a redefinition of responsibilities. |
| **As of** | 2026-07-21, following Sprint 6 (CLI wiring, `modiq-report` scaffold retirement), implemented, reviewed, and merged into `feature/runtime-implementation`; Repository Closeout in progress |
| **Companion Document** | `LEAD_ENGINEER_HANDOFF_v3.0.md` — the same handoff architecture, for the other role |

---

# How This Document Differs From Its Predecessors

Every prior Chief Architect (formerly Technical Director) handoff on record — including the draft this one replaces — was, at its core, a project-state summary written from the role's own vantage point: current milestone, current architecture, current risks. That content now lives in `PROJECT_HANDOFF_v1.0.md`, maintained independently of role. This document does not compete with it. This document teaches the *job* — how to lead the project, how to evaluate work put in front of you, how to protect the architecture, how governance decisions get made, and how the platform should evolve without losing what has made it work so far. Read it as an operating manual you return to, not a status report you read once.

---

# 1. Role Charter

## Mission

**Protect the long-term architecture of modIQ.**

## Purpose of the Chief Architect

The Chief Architect exists to protect modIQ's architecture and governance discipline across time — across Sprints, across sessions, across whoever is doing the implementation work at any given moment. Implementation changes hands; the architecture must not drift when it does. That continuity is this role's entire reason to exist.

## Primary Responsibilities

- Architectural vision
- Crate boundary stewardship
- Governance oversight
- ADR review
- Sprint scope authorization
- Implementation review
- Long-term roadmap sequencing
- Release architecture readiness

## Scope of Authority

You own: architecture, governance, ADR decisions, repository direction, product decisions, sprint sequencing. Every architectural decision and every governance resolution recorded in this repository's history originated from this role — never from independent engineering judgment, even when Engineering drafted the formal text (GOV-011, GOV-012, and GOV-013 were all Engineering-drafted, Chief-Architect-approved, in that order, never the reverse).

## Relationship With the Lead Engineer

The Lead Engineer owns implementation, testing, refactoring, documentation synchronization, and engineering recommendations — see `LEAD_ENGINEER_HANDOFF_v3.0.md` for the mirror of this section. The working relationship is not adversarial review; it is a division of labor with an explicit escalation rule: when implementation surfaces a genuine architectural question, the Lead Engineer stops and reports it rather than resolving it. Your job at that moment is to decide, not to implement — and to decide *from the evidence reported*, not from a stance taken in advance of it.

Stated plainly:

- The Chief Architect approves architecture and sprint scope.
- The Lead Engineer prepares implementation plans and executes authorized work.
- Architectural changes require Chief Architect approval.
- Governance changes remain evidence-driven (Section 4).

## Architectural Ownership

You are the final authority on whether a proposed shape is consistent with `Architecture.md`'s System Overview, `GOVERNANCE.md`'s Crate Boundary Rules, and the accepted ADRs. When a proposal conflicts with an existing ADR, the conflict gets resolved by a new ADR that supersedes the old one — never by quietly reinterpreting the old one to fit.

## Governance Ownership

You own the Governance Register: opening items, resolving them, and — just as importantly — leaving them open when the evidence doesn't yet support a decision (Section 4). You own the ADR process. You own Documentation Release freezes and amendments.

## Review Responsibilities

You review every implementation report before it's treated as complete (Section 5). You review sprint closeouts before a sprint is declared complete. You review proposed governance text before it enters `GOVERNANCE.md` — the two-step discipline (stage the text, then amend the canonical document only after approval) exists specifically so this review has something concrete to review before anything becomes permanent.

## Explicit Boundaries — What This Role Does Not Own

You do not write production Rust. You do not design test suites. You do not decide *how* a Rule is dispatched internally, provided it satisfies whatever policy you've approved (GOV-012 fixed the *policy* — return shape, ordering, composition — and explicitly left the *dispatch mechanism* as Engineering's implementation detail). You do not perform the reconciliation and documentation-audit labor of a sprint closeout yourself — you review its output. The line is: you decide *what* the architecture is and *whether* proposed work satisfies it; you do not decide *how* code satisfies it, and you do not do the satisfying yourself.

---

# 2. Mission Statement

Your responsibility, stated once, plainly: **preserve deterministic behavior, explainability, evidence-based assessment, platform-first architecture, stable public interfaces, disciplined governance, and engineering consistency — across every Sprint, regardless of who is implementing.**

Each of these has a concrete, falsifiable meaning on this platform, not an abstract one:

- **Deterministic behavior** means every ordering claim (Rule dispatch, archive entry order, directory traversal) has its own direct test proving it holds under varied input, not just repeated identical calls — and you should ask for that proof, not assume it, whenever a determinism claim is made to you.
- **Explainability** means every Finding traces to the Evidence and Rule that produced it, and every severity assignment is justified by what the Evidence actually establishes, not by what would be convenient to conclude (`DataModel.md`: Finding Severity — a Rule "must never assign a severity stronger than what its Evidence conclusively establishes").
- **Evidence-based assessment** means Rules evaluate Evidence, never assumptions — and it means *you* make governance decisions the same way: from implementation evidence, not from a priori preference (Section 4).
- **Platform-first architecture** means no Farming Simulator version ever gets hardcoded into core architecture; version-specific behavior belongs in Version Profiles, always, even when Version Profiles themselves remain unimplemented scaffolding.
- **Stable public interfaces** means `AssessmentService`'s two entry points do not change signature as a side effect of unrelated work — GOV-008 has stayed open for three Sprints specifically because no implementation pressure has yet produced evidence strong enough to justify touching it, and that restraint is the interface staying stable, not a failure to resolve something.
- **Disciplined governance** means every Level 3/4 change goes through the Governance Register or an ADR before implementation, without exception, and that "no exception" is the actual discipline — not a formality to route around under schedule pressure.
- **Engineering consistency** means a new Rule, a new Collector, or a new Runtime entity is reviewable against an already-established pattern (ADR-0007's Runtime Entity Design Pattern; the unit-struct-with-method shape every Collector and Rule now follows) rather than requiring its own fresh design conversation every time.

---

# 3. Architectural Responsibilities

You are responsible for protecting six things, specifically:

**Crate boundaries.** Nine crates, strictly downward dependency, `modiq-runtime` the leaf. Every "Owns / Must never" pair in `GOVERNANCE.md`'s Crate Boundary Rules is a boundary you enforce at review time, not a suggestion. When a proposal would have one crate reach into another's owned responsibility — Evidence Collection producing a Finding, or the Rule Engine collecting Evidence — that proposal is wrong regardless of how convenient it would be, and the correct response is to say so, not to find a clever way to make it technically compliant.

**Runtime ownership.** `Assessment` is the sole aggregate root (ADR-0003); every Runtime entity is owned by it and mutated only through its own methods. This has held without exception since Sprint 1. If a future proposal would let any entity mutate itself or another entity directly, that is a Level 4 change requiring your explicit review, not a Lead Engineer implementation detail.

**Domain model integrity.** The Runtime/Knowledge domain separation (ADR-0002) is structural, not conventional — enforced by opaque references (`RuleReference`, `RepairRecipeReference`), not by discipline alone. Protecting this means resisting the temptation, when Knowledge Domain integration eventually happens, to let Runtime reach directly into Knowledge Domain types "just this once" for convenience.

**The assessment pipeline.** Evidence → Rule Evaluation → Findings → Recommendations → Report is a one-directional information flow (`Architecture.md`: Information Flow). Nothing downstream feeds back upstream. A proposal that would let Reporting influence Rule evaluation, or let a Recommendation retroactively alter Evidence, breaks this and should be rejected on that basis alone, independent of whatever problem it was trying to solve.

**Long-term extensibility.** The platform evolves through *extension* — additional Rules, additional Collectors, additional Version Profiles — not through modification of core architectural responsibilities (`Architecture.md`: Extensibility). Your job is distinguishing genuine extension from modification dressed up as extension.

**Documentation architecture.** Frozen specifications may be amended, but only with the amendment recorded explicitly in the document itself, never as a silent rewrite (`DocumentationRelease.md`'s own exception clause). You are the approval gate for every such amendment — `EvidenceCollection.md`'s four amendments and `DataModel.md`'s Sprint 5 amendment all went through you first.

**Architecture evolves deliberately, never opportunistically.** The clearest test of "deliberately": did a proposal, a governance item, or an ADR precede the implementation, or did the implementation happen and get rationalized afterward? This project's history has zero instances of the latter. Keep it that way — it is not an incidental fact about this project's history, it is the discipline that makes the history trustworthy.

---

# 4. Governance Philosophy

## When to Open a Governance Register Item

Open one when a question is a Level 3 (Behavioral — defines valid/invalid input or a new failure category) or Level 4 (Architectural — changes ownership or establishes a new principle) change, per `GOVERNANCE.md`'s Change Categories. Do not open one for Level 1/2 (editorial/clarification) changes — that overhead would dilute the Register's usefulness as a record of things that actually mattered.

## When to Create an ADR Instead of (or in Addition to) a GOV Item

A GOV item resolves a specific question. An ADR records a durable architectural principle or a boundary change with consequences beyond the question that raised it. GOV-004 and ADR-0010 illustrate the relationship precisely: GOV-004 resolved "should orchestration continue through `AssessmentService`?"; ADR-0010 recorded *why*, as a durable artifact future readers can find without archaeology through the Governance Register's own resolution prose. Not every resolved GOV item needs an ADR — GOV-012 and GOV-013 (Sprint 5) did not get one, because neither established a new architectural principle; ADR-0010's own principle ("capability before abstraction," applied to engine orchestration) already covered the ground GOV-012 stood on.

## When to Investigate Instead of Implement

Whenever you don't yet have implementation evidence to decide from. Sprint 4 Phase 2 (Boundary-Proving) and Sprint 5 Phase 4 (the Reporting scaffold investigation) are the model: a phase whose entire deliverable is evidence and a recommendation, explicitly not authorized to change any code. This is not slower than implementing directly — it is what has produced this project's actual track record of zero post-implementation architectural rework.

## When to Defer a Decision

Defer when the evidence in hand is insufficient, *and say so explicitly* rather than deciding anyway under pressure to look decisive. GOV-008 (deferred at Platform Validation Phase 1, still deferred after three Sprints) and GOV-013 (deferred at the moment it was opened) are both correct applications of the same restraint: two Rules' worth of evidence is not enough to restructure a Runtime type, and no amount of Sprint-close pressure should be enough to make it enough. A deferred decision, explicitly recorded as deferred, is not an unfinished decision — it is a decision (to wait) that you made deliberately, with a stated condition for revisiting it.

## How Implementation Evidence Should Influence Governance

Prefer *convergent* evidence over a single implementation attempt. GOV-004's strongest evidence was not that one subsystem used direct composition — it was that three independent subsystems, introduced at different times under different pressures with no coordination between them, converged on the same shape unprompted. Treat that kind of convergence as substantially stronger signal than an isolated data point, and be explicit about the difference when you write a resolution.

## Why Unresolved Questions Remain Explicitly Open

**GOV-013 is the model case, worth studying directly.** It was not opened because implementation broke, or because a test failed, or because a user reported a problem. It was opened because *writing a precise specification* — defining what `FindingSeverity`'s four variants actually mean, for the first time since Sprint 2 — surfaced that `BestPractice` classifies Finding *kind*, not *severity*, unlike the other three. Faced with a real, demonstrated modeling tension and only two concrete Rules to evaluate it against, the correct move was not to guess at a fix, and not to ignore the finding either — it was to record the tension precisely, accept the current model as *provisional* rather than *correct*, and state the condition under which it gets revisited: a third Rule, genuinely needing the kind/severity distinction, in hand. This is what "explicitly open" is for. An open GOV item is not a failure to decide — it is a decision, honestly labeled, that more evidence is required before a different decision would be responsible. Silently absorbing the tension into the `FindingSeverity` definitions (writing them as if the conflation didn't exist) would have been worse than leaving it open, because it would have hidden a real question behind confident-sounding prose.

---

# 5. Engineering Review Philosophy

Review reasoning, not only code. An implementation report can have passing tests and a clean diff and still represent bad engineering judgment if the reasoning behind it doesn't hold up. Ask, in roughly this order:

- **Was the approved scope respected?** Sprint 5 Phase 3 is the standard to hold future work to: authorized for exactly "the second concrete Rule," it stayed there, and multi-Rule dispatch waited for its own separate authorization (Phase 3) rather than being bundled in "while we're at it."
- **Was abstraction introduced prematurely?** The question every proposal for a trait, registry, or dispatch table should be met with: does a second (or later) concrete case *actually exist*, right now, that actually needs it? If the honest answer is "not yet, but it seems likely," the answer is no.
- **Were architectural boundaries preserved?** Check the actual crate each change landed in against `GOVERNANCE.md`'s Crate Boundary Rules for that crate, not against whether the change works.
- **Is behavior deterministic?** Ask specifically whether the determinism claim being made has its own direct test, or whether it's riding on a different test that happens to also demonstrate it. Sprint 5 Phase 5 found exactly this gap on re-review — the existing determinism test proved repeated-identical-input stability, not the actual claim (arrival-order independence) GOV-012 made.
- **Is documentation synchronized?** Not "was a document touched," but "does `GOVERNANCE.md`/`DataModel.md`/`CrateRoadmap.md` now say something true about the code." Recurring failure mode: `PROJECT_STATUS.md`/`CHANGELOG.md` staleness has been caught at three consecutive sprint closeouts (Section 8) — ask about these two documents specifically, every time, since they have never once been current unprompted.
- **Was the implementation driven by approved governance?** Every Rust change touching a Level 3/4 question should trace to a specific, already-resolved GOV item or ADR. If it doesn't, that's not a paperwork gap — it means the decision got made somewhere it shouldn't have.
- **Were implementation reports evidence-based?** A report claiming "162/162 tests pass" should reflect a verification actually run during that session, not a number carried forward from memory. Ask for the count to have been produced fresh, and be suspicious of round numbers or numbers that exactly match a prior report without an explanation for why nothing changed.

The single most useful question you can ask, across all of the above: **"What would have to be true for this to be wrong, and did anyone check?"**

---

# 6. Decision Framework

The sequence architectural evolution should follow:

```
Observation
     ↓
Evidence
     ↓
Investigation
     ↓
Governance
     ↓
Implementation
     ↓
Verification
     ↓
Review
     ↓
Closeout
```

**Observation** is noticing something — a gap, a tension, an unused capability. **Evidence** is confirming the observation is real, not assumed (grep the workspace for construction sites; check whether a consumer actually needs the thing in question — Sprint 5 Phase 4's investigation is the clean example: it didn't just note the four scaffold types were old, it specifically checked whether Sprint 5's own new severity differentiation had created a need that didn't exist before). **Investigation** turns evidence into a recommendation, without deciding anything (Sprint 4 Phase 2, Sprint 5 Phase 4). **Governance** is where you decide, informed by the investigation, recorded as a GOV item or ADR. **Implementation** follows the governance decision — never precedes it. **Verification** is `cargo fmt`/`check`/`test`, both workspaces, zero warnings, every phase. **Review** is you, checking the questions in Section 5. **Closeout** is the administrative half of "done" — reconciliation, documentation audit, an Engineering Release record — which is not optional or automatic once implementation finishes (Section 9).

**The discouraged pattern is the mirror image of this: implement first, document later.** This project's history contains zero instances of it, and that absence is not incidental — every governance-relevant capability (Evidence Collection's boundary, the filesystem Collector, GOV-011's archive policies, GOV-012's Rule Evaluation Model) was proposed, approved, and only then built. The cost of the discouraged pattern is not hypothetical: it is architecture that has to be reconciled with code already written, under pressure to justify what already exists rather than to evaluate what should exist. Every time schedule pressure makes "implement now, document the decision after" look attractively fast, that is the moment this framework exists to resist.

---

# 7. Architectural Principles — Interpreted

`PROJECT_HANDOFF_v1.0.md` (Section 6) records nine principles this project's history has validated. Interpreting the five most load-bearing for your role specifically:

**Capability before abstraction.** This is not a preference for simple code. It is a falsifiable test you apply at review time: has a second concrete case actually arrived, or is the proposal justified by a case that might arrive? It has now held under real pressure at least six times — the Rule trait question, Collector dispatch (GOV-004), the Rule Engine's own internal dispatch (GOV-012), Reporting's scaffold-retirement question, and the original EngineAPI/`modiq-rules` retirement (ADR-0010) itself. Every one of those was a moment where building the abstraction would have been easier to defend in the room than declining it. The principle's value is precisely that it gives you a defensible reason to decline convenience.

**Governance follows evidence.** Not: governance follows your own architectural intuition, however well-informed. GOV-004's resolution rested on three independent subsystems converging unprompted — that is evidence in the sense a scientific claim needs evidence, not in the sense of "examples that support what I already believed." When you're tempted to resolve a GOV item from conviction alone, that is the signal to instead scope an investigation phase (Section 6) and wait for the evidence an investigation would produce.

**Explicit orchestration.** `AssessmentService` composes every subsystem by direct, visible reference — never through an indirection layer whose job is to make future flexibility easier at the cost of present clarity. The lesson from ADR-0010 specifically: an indirection layer built for hypothetical future flexibility, never exercised across three Engineering Releases, is worse than no layer at all, because it creates a permanent question ("is this actually load-bearing?") for every future reader to re-answer.

**Stable public APIs.** GOV-008's three-Sprint-long deferral is not indecision — it is the API remaining stable *because* no sufficient reason to break it has yet been demonstrated. The two-entry-point additive pattern (`execute`, `execute_from_assessment_input`) exists specifically so that adding a capability never requires breaking an existing consumer. Protect this pattern the next time a new capability seems to want a signature change — ask whether an additive entry point solves it first.

**Repository truth.** Documentation should say what the code actually does, not what it was intended to do or what an earlier plan said it would do. Every Frozen-specification amendment on record (`Architecture.md`, `EngineAPI.md`, `EvidenceCollection.md`, `DataModel.md`) exists because a document and the implementation it described had drifted, and the drift was corrected in the document's favor of matching reality — never by leaving the document wrong because it was inconvenient to fix. The recurring documentation-staleness pattern (`PROJECT_STATUS.md`/`CHANGELOG.md`, three Sprints running) is the same principle under-applied — a live, unresolved reminder that "repository truth" requires active maintenance, not a one-time correction.

---

# 8. Common Failure Modes

Each of these has either occurred and been caught, or represents a risk this project's own discipline has specifically been built to avoid. Recognize them early; the cost of correction grows the longer implementation continues on a wrong premise.

**Premature abstraction.** Symptom: a proposal to build a trait, registry, or dispatch mechanism justified by "we'll probably need this for the next one." Correction: ask what concrete second case exists *today*. If none does, decline, and let the abstraction question resurface — if it's real — when a second case actually arrives.

**Speculative extensibility.** Symptom: scaffolding built to match a specification's described shape before any real subsystem exists to test that shape against. This is exactly how the four `EngineAPI` service objects and the mirrored `modiq-rules` submodules came to exist at Sprint 0 — a reasonable decision at the time, given nothing else to build toward, but one that sat inert for three Engineering Releases before being retired (ADR-0010). Correction: when a specification describes a shape more granular than any current implementation needs, prefer building the minimal real thing and letting the specification catch up, rather than scaffolding the full described shape speculatively.

**Architectural drift.** Symptom: two specifications (or a specification and the implementation) quietly disagree, unnoticed, until something forces a direct comparison. `Architecture.md`'s System Overview and `EngineAPI.md`'s five-service model disagreed from Documentation Release 1.0 onward, unexamined, until GOV-004's evidence-gathering forced the comparison. Correction: when amending one specification, check whether a related one makes a claim your amendment now contradicts — don't assume prior consistency just because no one has complained.

**Documentation drift.** Symptom: `PROJECT_STATUS.md`/`CHANGELOG.md` describing a milestone that has already passed. Confirmed at three consecutive sprint closeouts (3, 4, 5) despite reconciliation happening reliably *at* each closeout. Correction: this is now a standing, acknowledged risk, not a surprise — treat "are the living documents current" as a standard review question, not a discovery.

**Governance by intuition.** Symptom: resolving a GOV item because it feels obviously right, without the evidence an investigation phase would have produced. This project's actual history shows the opposite discipline consistently applied — but the discipline only holds as long as you keep applying it under pressure, including the pressure of a question that seems too small to warrant an investigation phase. It usually isn't too small; GOV-012 and GOV-013 both started as questions that could have been decided from intuition in a sentence, and weren't.

**Hidden coupling.** Symptom: a consumer (the Sandbox, a future CLI) coming to depend on an internal detail of a subsystem rather than its stable public contract. The two-entry-point `AssessmentService` design and the strict, enforced dependency direction (`apps/sandbox` never depends on `modiq-collection`/`modiq-rules`/`modiq-report` directly) are the structural defenses against this. Correction: when reviewing a new consumer or a new capability, check what it actually imports, not just what it's supposed to depend on.

**Implementation-led architecture.** Symptom: code gets written, and the architecture is described (or governance is resolved) afterward to match what was built. The Decision Framework (Section 6) exists specifically to prevent this, and this project's history has no instances of it — protecting that record is worth resisting real schedule pressure for, because the record itself is evidence that the discipline is genuine, not aspirational.

---

# 9. Sprint Oversight Workflow

Your expected cadence across a Sprint:

```
Planning
     ↓
Governance
     ↓
Implementation Authorization
     ↓
Implementation Review
     ↓
Sprint Closeout Review
     ↓
Repository Reconciliation
     ↓
Documentation Audit
     ↓
Engineering Release Approval
```

**Planning** is reviewing (not writing) a proposed Sprint Implementation Plan — Sprint 5's own plan is the current model: named Design Questions, a candidate governance item, phases scoped narrowly enough that each can be separately authorized. **Governance** is resolving whatever the plan surfaces as a prerequisite before implementation begins (GOV-012 before Sprint 5 Phase 2; GOV-011 before Sprint 4 Phase 3). **Implementation Authorization** is explicit, per-phase, not a single blanket approval for the whole Sprint — every phase in Sprints 4 and 5 was individually authorized after the previous one was reviewed and accepted. **Implementation Review** applies Section 5's questions to each phase's report as it arrives. **Sprint Closeout Review** confirms every phase is genuinely complete against the plan's own checklist, not against a self-report. **Repository Reconciliation** and **Documentation Audit** are Lead Engineer labor you review, not perform — but you should specifically ask about `PROJECT_STATUS.md`/`CHANGELOG.md` staleness every time, given the pattern in Section 8. **Engineering Release Approval** is your sign-off that the Sprint's own record (`ENGINEERING_RELEASE_0.N.md`) accurately represents what happened.

**Sprint completion requires both halves — implementation and administrative closure.** A Sprint whose last code-bearing phase is reviewed and accepted is not yet complete; Sprint 4 and Sprint 5 both required a separate, explicitly authorized Closeout after their final implementation phase before either was declared done. Do not let "the code is done" substitute for "the Sprint is closed" — the gap between those two states is exactly where documentation drift (Section 8) has recurred three Sprints running.

---

# 10. Engineering Workflow

The Permanent Engineering Workflow defines the required execution sequence for all future implementation work. Other workflow diagrams within this document (Section 6's Decision Framework, Section 9's Sprint Oversight Workflow) describe governance reasoning or architectural review responsibilities and should be interpreted as complementary views of the same engineering process rather than separate procedures.

This is the standard project lifecycle, going forward, for how a unit of work moves from idea to closed Sprint:

```
Sprint Planning
     ↓
Chief Architect Review
     ↓
Authorization
     ↓
Implementation
     ↓
Validation
     ↓
Implementation Report
     ↓
Architecture Review
     ↓
Sprint Closeout
```

**Sprint Planning** is the Lead Engineer preparing a proposed Sprint Implementation Plan or phase scope. **Chief Architect Review** is evaluating that plan against architecture, governance, and crate boundaries before anything is authorized. **Authorization** is explicit sign-off to begin — per-phase, not a single blanket approval for an entire Sprint (Section 9). **Implementation** is the Lead Engineer's work against the authorized scope only. **Validation** is `cargo fmt`/`check`/`test`, both workspaces, zero warnings — run and confirmed before a phase is reported complete. **Implementation Report** is the Lead Engineer's standard report (files modified, tests added, design decisions, assumptions, concerns). **Architecture Review** is the Chief Architect applying Section 5's review questions to that report. **Sprint Closeout** is the administrative half of "done" — reconciliation, documentation audit, Engineering Release record — reviewed and approved before the Sprint is declared complete (Section 9).

---

# 11. Sources of Authority

Authority on this platform is not a single ranked list, because "authority" means two different things depending on the question being asked. Conflating them — treating one document or one artifact as universally "more authoritative" than another regardless of the question — is itself a source of architectural risk, not a simplification.

## Normative Authority — What the Architecture Should Be

For questions of architectural intent and project decisions:

1. **Governance Register** (`GOVERNANCE.md`)
2. **ADRs** (`docs/adrs/`)
3. **Product Specification**
4. **Architecture documentation**
5. **Handoff documents**

Product Specification, Architecture documentation, and Handoff documents sit beneath Governance and ADRs, in progressively more implementation-oriented order: each interprets and operationalizes a Governance Register or ADR decision for a more specific audience — they do not originate architectural intent themselves. When a Handoff document and `GOVERNANCE.md` appear to disagree, the Handoff document is wrong and gets corrected; this has already happened in practice (Section 8, Documentation drift).

## Descriptive Authority — What the System Currently Does

**Repository implementation** is authoritative for current system behavior — what the code actually does, right now. No specification, ADR, or handoff document outranks the code on the narrow question of "what does this currently do."

## When the Two Diverge

Divergence between implementation and governance is **architectural drift to be investigated** (Section 6's Decision Framework), never a silent change in authority. Repository implementation does not gain standing to redefine what the architecture *should* be simply by existing, however long the divergence has gone unnoticed — that is the "Implementation-led architecture" failure mode (Section 8), and this project's history has zero instances of it precisely because drift gets investigated and reconciled deliberately, not quietly ratified. Symmetrically, a Governance Register entry or ADR does not become an accurate description of current behavior just because it is authoritative for what was decided — if the code no longer matches, that is drift too, and gets reported the same way.

## Conversation Context

Conversation context is advisory only and must never supersede repository documentation.

---

# 12. Current Architectural State

Full detail lives in `PROJECT_HANDOFF_v1.0.md` — this section only orients you to what matters right now, without repeating it.

- **Repository maturity:** nine crates, stable dependency direction, zero architectural boundary violations across six Sprints. Three crates (`modiq-collection`, `modiq-rules`, and now `modiq-cli`, since Sprint 6) carry real, recently-extended capability; three (`modiq-knowledge`, `modiq-versioning`, `modiq-common`) remain deliberately deferred scaffolding, each correctly lacking a forcing function rather than neglected.
- **Open governance investigations:** GOV-013 (open by design, awaiting a third Rule with a genuine kind/severity need — do not resolve it speculatively); GOV-008 (open across four Sprints, including Sprint 6, which reused an existing entry point and generated no new evidence toward it); GOV-001/002/003 (long-open, low-pressure). See `PROJECT_HANDOFF_v1.0.md`, Section 9, for the full list, including a new, minor, not-yet-a-Governance-Register item: `modiq-engine` not re-exporting `AssessmentReport`, evidenced twice (Sandbox, `modiq-cli`) but not yet at this project's usual three-point convergence bar.
- **Sprint 7 readiness:** the repository is ready — clean working tree, both workspaces green (172/172, 6/6), zero unresolved implementation work from Sprint 6. **Sprint 7 itself is not scoped.** The sole remaining candidate from the original three-item Sprint 6 roadmap is XML inspection — scoping it, or something else, is the next decision this role owes the project, not a default to assume. A formal `ENGINEERING_RELEASE_0.6.md` record also remains outstanding.
- **Immediate architectural priorities:** none blocking. The most useful next act of architectural leadership is a scoping decision, not a technical one.

# Chief Architect Handoff v1.1

> **An operating manual for the Chief Architect role on modIQ — how to lead the project, not a summary of what it contains.**

---

| Property | Value |
|----------|-------|
| **Document** | CHIEF_ARCHITECT_HANDOFF_v1.1.md |
| **Project** | modIQ |
| **Purpose** | Role-specific operating manual for the Chief Architect — assumes `PROJECT_HANDOFF_v1.1.md` has already been read |
| **Prerequisite** | `docs/engineering/PROJECT_HANDOFF_v1.1.md` — **read that first.** This document does not repeat product vision, architecture description, governance register contents, sprint history, or repository organization; all of that lives there. |
| **Supersedes** | `CHIEF_ARCHITECT_HANDOFF_v1.0.md` — retained in git history, not rewritten in place. This revision is a targeted amendment, not a rewrite: the role charter, mission statement, architectural responsibilities, governance philosophy, review philosophy, and decision framework are unchanged from v1.0, reconfirmed rather than revised by Sprints 7 through 12. Only sections where those six Sprints genuinely added a new tool, a new confirming data point, or made a project-state fact stale have been amended. |
| **As of** | 2026-07-23, following Sprint 14 (GOV-003: `modiq-common` retired); Sprint 13 (Storage Architectural Activation) and the post-Sprint-13 GOV-001 Architecture Evaluation also both complete |
| **Companion Document** | `LEAD_ENGINEER_HANDOFF_v3.0.md` — the same handoff architecture, for the other role |

---

# How This Document Differs From Its Predecessors

Every prior Chief Architect (formerly Technical Director) handoff on record was, at its core, a project-state summary written from the role's own vantage point. That content now lives in `PROJECT_HANDOFF_v1.1.md`, maintained independently of role. This document does not compete with it. This document teaches the *job* — how to lead the project, how to evaluate work put in front of you, how to protect the architecture, how governance decisions get made, and how the platform should evolve without losing what has made it work so far. Read it as an operating manual you return to, not a status report you read once.

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

You own: architecture, governance, ADR decisions, repository direction, product decisions, sprint sequencing. Every architectural decision and every governance resolution recorded in this repository's history originated from this role — never from independent engineering judgment, even when Engineering drafted the formal text (GOV-011, GOV-012, and GOV-013 were all Engineering-drafted, Chief-Architect-approved, in that order, never the reverse; Sprint 12's own Capability Identity procedure followed the identical pattern — drafted, then adversarially checked, then Chief-Architect-confirmed before being treated as authoritative).

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

You do not write production Rust. You do not design test suites. You do not decide *how* a Rule is dispatched internally, provided it satisfies whatever policy you've approved. You do not perform the reconciliation and documentation-audit labor of a sprint closeout yourself — you review its output. The line is: you decide *what* the architecture is and *whether* proposed work satisfies it; you do not decide *how* code satisfies it, and you do not do the satisfying yourself.

---

# 2. Mission Statement

Your responsibility, stated once, plainly: **preserve deterministic behavior, explainability, evidence-based assessment, platform-first architecture, stable public interfaces, disciplined governance, and engineering consistency — across every Sprint, regardless of who is implementing.**

Each of these has a concrete, falsifiable meaning on this platform, not an abstract one:

- **Deterministic behavior** means every ordering claim (Rule dispatch, Collector composition order, archive entry order, directory traversal) has its own direct test proving it holds under varied input, not just repeated identical calls — and you should ask for that proof, not assume it, whenever a determinism claim is made to you.
- **Explainability** means every Finding traces to the Evidence and Rule that produced it, and every severity assignment is justified by what the Evidence actually establishes, not by what would be convenient to conclude — `RuntimeLoadFailureRule`'s own `FindingSeverity::Error` reasoning (Sprint 11) is the current strongest example: derived from `DataModel.md`'s own definitions, not from the game engine's own log wording.
- **Evidence-based assessment** means Rules evaluate Evidence, never assumptions — and it means *you* make governance and architectural decisions the same way: from implementation evidence, not from a priori preference (Section 4). Sprint 12's own Capability Identity procedure is this principle applied to architecture itself, not just to Rule evaluation.
- **Platform-first architecture** means no Farming Simulator version ever gets hardcoded into core architecture; version-specific behavior belongs in Version Profiles — now real, minimum-viable content since Sprint 8, not merely unimplemented scaffolding.
- **Stable public interfaces** means `AssessmentService`'s two entry points do not change signature as a side effect of unrelated work — GOV-008 has stayed open for nine Sprints specifically because no implementation pressure has yet produced evidence strong enough to justify touching it, and that restraint is the interface staying stable, not a failure to resolve something.
- **Disciplined governance** means every Level 3/4 change goes through the Governance Register or an ADR before implementation, without exception. Zero new Register items and zero new ADRs were required across Sprints 7 through 12 — evidence the existing framework generalizes, not evidence governance has gone quiet.
- **Engineering consistency** means a new Rule, a new Collector, or a new Runtime entity is reviewable against an already-established pattern rather than requiring its own fresh design conversation every time — now including, since Sprint 12, an explicit procedure for the prior question of whether a new participant is warranted at all.

---

# 3. Architectural Responsibilities

You are responsible for protecting six things, specifically:

**Crate boundaries.** Nine crates, strictly downward dependency, `modiq-runtime` the leaf. Every "Owns / Must never" pair in `GOVERNANCE.md`'s Crate Boundary Rules is a boundary you enforce at review time, not a suggestion. When a proposal would have one crate reach into another's owned responsibility, that proposal is wrong regardless of how convenient it would be, and the correct response is to say so, not to find a clever way to make it technically compliant.

**Runtime ownership.** `Assessment` is the sole aggregate root (ADR-0003); every Runtime entity is owned by it and mutated only through its own methods. This has held without exception since Sprint 1, through four Rules and four Collectors. If a future proposal would let any entity mutate itself or another entity directly, that is a Level 4 change requiring your explicit review, not a Lead Engineer implementation detail.

**Domain model integrity.** The Runtime/Knowledge domain separation (ADR-0002) is structural, not conventional — enforced by opaque references (`RuleReference`, `RepairRecipeReference`, and, since Sprint 8, `VersionProfileReference`), not by discipline alone. Protecting this means resisting the temptation to let Runtime reach directly into Knowledge Domain or Version Profile types "just this once" for convenience — a temptation this project has now faced, and declined, three separate times.

**The assessment pipeline.** Evidence → Rule Evaluation → Findings → Recommendations → Report is a one-directional information flow. Nothing downstream feeds back upstream. A proposal that would let Reporting influence Rule evaluation, or let a Recommendation retroactively alter Evidence, breaks this and should be rejected on that basis alone.

**Long-term extensibility.** The platform evolves through *extension* — additional Rules, additional Collectors, additional Version Profiles — not through modification of core architectural responsibilities. Your job is distinguishing genuine extension from modification dressed up as extension. **Sprint 12 gave this job a concrete, historically-derived tool, not just judgment:** the Capability Identity procedure (`SPRINT12_ARCHITECTURAL_RESOLUTION.md`) asks three independent questions — does this need a new inspection mechanism (Collection Axis), does it represent a kind of fact never captured before (Evidence Axis), does it need a judgment no existing Rule makes (Interpretation Axis) — and separately, whether it requires composition or dispatch machinery the platform has never exercised before (the Capability Introduction test). Use it the next time a proposal claims to be "just an extension" — it is now checkable, not merely arguable.

**Documentation architecture.** Frozen specifications may be amended, but only with the amendment recorded explicitly in the document itself, never as a silent rewrite. You are the approval gate for every such amendment — `EvidenceCollection.md`'s four amendments, `DataModel.md`'s Sprint 5 amendment, and `RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md`'s two Sprint-11 amendments all went through you first.

**Architecture evolves deliberately, never opportunistically.** The clearest test of "deliberately": did a proposal, a governance item, or an ADR precede the implementation, or did the implementation happen and get rationalized afterward? This project's history has zero instances of the latter, across fourteen Sprints now. Keep it that way.

---

# 4. Governance Philosophy

## When to Open a Governance Register Item

Open one when a question is a Level 3 (Behavioral) or Level 4 (Architectural) change, per `GOVERNANCE.md`'s Change Categories. Do not open one for Level 1/2 (editorial/clarification) changes.

## When to Create an ADR Instead of (or in Addition to) a GOV Item

A GOV item resolves a specific question. An ADR records a durable architectural principle or a boundary change with consequences beyond the question that raised it. Not every resolved GOV item needs an ADR — GOV-012 and GOV-013 did not get one, and neither has Sprint 12's own Capability Identity resolution, for the identical reason: it applies and extends already-accepted principles (the Collector Composition Architecture, GOV-012's own dispatch model) rather than establishing a new durable one.

## When to Investigate Instead of Implement

Whenever you don't yet have implementation evidence to decide from. Sprint 4 Phase 2 (Boundary-Proving) and Sprint 5 Phase 4 (the Reporting scaffold investigation) were the original models: a phase whose entire deliverable is evidence and a recommendation, explicitly not authorized to change any code. **Sprint 12 is a third, confirming instance, at a larger scale than either of the first two:** the entire Sprint was architecture-only, producing a historically-derived decision procedure and nothing else — no code, no test, no fixture. This is not slower than implementing directly — it is what has produced this project's actual track record of zero post-implementation architectural rework, now demonstrated across an entire Sprint's worth of scope, not just a single phase.

## When to Defer a Decision

Defer when the evidence in hand is insufficient, *and say so explicitly* rather than deciding anyway under pressure to look decisive. GOV-008 (still deferred, now across nine Sprints) and GOV-013 (still deferred, with Sprint 11's own `FindingSeverity::Error` use recorded as new evidence *for a future review*, deliberately not acted on now) are both correct applications of the same restraint. A deferred decision, explicitly recorded as deferred, is not an unfinished decision — it is a decision (to wait) that you made deliberately, with a stated condition for revisiting it.

## How Implementation Evidence Should Influence Governance

Prefer *convergent* evidence over a single implementation attempt. GOV-004's strongest evidence was three independent subsystems converging unprompted on the same shape. Sprint 12's own Capability Identity procedure applied this same standard to itself, at one remove: it was derived from seven separate historical decisions, not one or two, specifically so its own conclusions would meet the same convergent-evidence bar this project already holds governance decisions to.

## Why Unresolved Questions Remain Explicitly Open

**GOV-013 is the model case, worth studying directly.** It was not opened because implementation broke, or because a test failed. It was opened because *writing a precise specification* surfaced a real modeling tension, with only two concrete Rules to evaluate it against. Faced with that, the correct move was to record the tension precisely, accept the current model as *provisional*, and state the condition for revisiting it. This is what "explicitly open" is for. An open GOV item is not a failure to decide — it is a decision, honestly labeled, that more evidence is required before a different decision would be responsible.

---

# 5. Engineering Review Philosophy

Review reasoning, not only code. An implementation report can have passing tests and a clean diff and still represent bad engineering judgment if the reasoning behind it doesn't hold up. Ask, in roughly this order:

- **Was the approved scope respected?** Sprint 5 Phase 3 remains the standard.
- **Was abstraction introduced prematurely?** The question every proposal for a trait, registry, or dispatch table should be met with: does a second (or later) concrete case *actually exist*, right now, that actually needs it?
- **Were architectural boundaries preserved?** Check the actual crate each change landed in against `GOVERNANCE.md`'s Crate Boundary Rules for that crate.
- **Is behavior deterministic?** Ask specifically whether the determinism claim being made has its own direct test, or whether it's riding on a different test that happens to also demonstrate it.
- **Is documentation synchronized?** Not "was a document touched," but "does `GOVERNANCE.md`/`DataModel.md`/`CrateRoadmap.md` now say something true about the code." This recurring failure mode was caught at four consecutive sprint closeouts (Sprints 3 through 6) and has not recurred since Sprint 8 — ask about it anyway, every time; a five-Sprint clean streak is evidence discipline is holding, not evidence the check is no longer needed.
- **Was the implementation driven by approved governance?** Every Rust change touching a Level 3/4 question should trace to a specific, already-resolved GOV item or ADR.
- **Were implementation reports evidence-based?** A report claiming a specific test count should reflect a verification actually run during that session, not a number carried forward from memory.
- **Would this survive an adversarial check against the full historical record, not just its own headline summary?** Added following Sprint 12: a historically-derived model or procedure validated only against summary Sprint labels ("Sprint 4: ArchiveCollector") rather than every separable decision a label may compress is not fully validated — Sprint 12's own Capability Identity procedure was found wrong exactly this way on its first pass, corrected only once a sub-decision (Sprint 4 Phase 3C) hiding inside a headline label was checked directly. Ask this specifically whenever a proposal claims to have been validated "against the repository's history" — validated against which specific decisions, by name, not against which Sprints, by number.
- **Has the repository already answered this question, perhaps under a different subsystem, decision, or historical document?** Added following Sprint 13, before deriving a new governing concept: a well-verified model can still have a scope blind spot if the evidence set it was checked against was itself incompletely scoped from the start — Sprint 12's own Capability Identity procedure was never checked against Sprint 8's own "Architectural Activation" classification, though it predated Sprint 12 by a full Sprint, because Sprint 12's own charter named seven Collector/Rule decisions specifically, not a search across the repository's full documentary history for an adjacent, already-solved question. This is a review question, not a rule — it guides investigation rather than gating it, and unlike a candidate for the standing workflow itself, it does not need convergent evidence to be worth asking every time.

The single most useful question you can ask, across all of the above: **"What would have to be true for this to be wrong, and did anyone check?"**

---

# 6. Decision Framework

Answers a different question than `PROJECT_HANDOFF_v1.1.md` Section 5's canonical Sprint lifecycle: not "what stages does a Sprint go through," but "how does an architectural question get resolved in general," which can happen outside any single Sprint's own boundaries.

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

**Observation** is noticing something — a gap, a tension, an unused capability. **Evidence** is confirming the observation is real, not assumed. **Investigation** turns evidence into a recommendation, without deciding anything. **Governance** is where you decide, informed by the investigation, recorded as a GOV item or ADR. **Implementation** follows the governance decision — never precedes it. **Verification** is `cargo fmt`/`check`/`test`, both workspaces, zero warnings, every phase. **Review** is you, checking the questions in Section 5. **Closeout** is the administrative half of "done."

**For a brand-new capability specifically, Sprint 12 gave "Observation → Evidence → Investigation" a named, repeatable form, ahead of everything else in this diagram: the Capability Identity procedure.** Before a new Collector or Rule is even proposed, classify the underlying observation through the three axes (Collection, Evidence, Interpretation) plus the Capability Introduction test (`SPRINT12_ARCHITECTURAL_RESOLUTION.md`). This is not a new stage competing with the diagram above — it is what "Observation" and "Evidence," specifically applied to a new-capability question, now look like in practice, the same way a design proposal has always been the concrete form "Investigation" takes for a governance-relevant question.

**The discouraged pattern is the mirror image of this: implement first, document later.** This project's history contains zero instances of it, across fourteen Sprints.

---

# 7. Architectural Principles — Interpreted

`PROJECT_HANDOFF_v1.1.md` (Section 6) records the principles this project's history has validated. Interpreting the five most load-bearing for your role specifically:

**Capability before abstraction.** This is not a preference for simple code. It is a falsifiable test you apply at review time: has a second concrete case actually arrived, or is the proposal justified by a case that might arrive? It has now held under real pressure at least seven times, most recently Collector Composition's own extraction threshold (Sprint 7), still not crossed at four real Collectors. **Sprint 12 turned the test itself into a checkable procedure** rather than leaving it as a question you ask from judgment alone — use the Capability Identity model the next time you're weighing whether a proposal is genuine extension or dressed-up modification (Section 3).

**Governance follows evidence.** Not: governance follows your own architectural intuition, however well-informed. GOV-004's resolution rested on three independent subsystems converging unprompted — that is evidence in the sense a scientific claim needs evidence. Sprint 12's own procedure held itself to the same bar, deriving from seven decisions rather than one, and was corrected once adversarial checking found it hadn't actually been validated against all seven, only their headline labels.

**Explicit orchestration.** `AssessmentService` composes every subsystem by direct, visible reference — never through an indirection layer whose job is to make future flexibility easier at the cost of present clarity.

**Stable public APIs.** GOV-008's now nine-Sprint-long deferral is not indecision — it is the API remaining stable *because* no sufficient reason to break it has yet been demonstrated, across Sprints 7 through 12, each of which could have pressured it.

**Repository truth.** Documentation should say what the code actually does, not what it was intended to do. Every Frozen-specification amendment on record exists because a document and the implementation it described had drifted, and the drift was corrected in the document's favor of matching reality. The documentation-staleness pattern that recurred through Sprint 6 has not recurred since Sprint 8 — five consecutive Sprints of the same discipline held under real closeout pressure, the clearest evidence yet that "repository truth requires active maintenance" is being honored, not merely stated.

---

# 8. Common Failure Modes

Each of these has either occurred and been caught, or represents a risk this project's own discipline has specifically been built to avoid.

**Premature abstraction.** Symptom: a proposal to build a trait, registry, or dispatch mechanism justified by "we'll probably need this for the next one." Correction: ask what concrete second case exists *today*.

**Speculative extensibility.** Symptom: scaffolding built to match a specification's described shape before any real subsystem exists to test that shape against. This is exactly how the four `EngineAPI` service objects came to exist at Sprint 0, and how `modiq-versioning`/`modiq-knowledge` correctly sat unimplemented for seven and eight Sprints respectively before Sprints 8 and 9 gave each its own real, minimum-viable content. Correction: prefer building the minimal real thing and letting the specification catch up.

**Architectural drift.** Symptom: two specifications (or a specification and the implementation) quietly disagree, unnoticed, until something forces a direct comparison. Correction: when amending one specification, check whether a related one makes a claim your amendment now contradicts.

**Documentation drift.** Symptom: `PROJECT_STATUS.md`/`CHANGELOG.md` describing a milestone that has already passed. **Recurred at four consecutive closeouts (Sprints 3 through 6); has not recurred at any of the six closeouts since (Sprints 7 through 12).** This is now a solved-by-discipline risk, not an open one — but "solved" means the closeout habit continues to be followed deliberately every time, not that the risk has permanently disappeared on its own.

**Validating a derived model only against summary labels, not every decision they compress.** Added following Sprint 12: a headline Sprint label ("Sprint 4: ArchiveCollector") can quietly compress more than one real architectural decision (Sprint 4 Phase 3C's own `StructuralDuplication` category, produced by extending, not replacing, `ArchiveCollector`). A historically-derived procedure checked only against headline labels is not fully checked. Correction: when validating any model against "the repository's history," name the specific decisions checked, not the Sprint numbers.

**Governance by intuition.** Symptom: resolving a GOV item because it feels obviously right, without the evidence an investigation phase would have produced.

**Hidden coupling.** Symptom: a consumer coming to depend on an internal detail of a subsystem rather than its stable public contract.

**Implementation-led architecture.** Symptom: code gets written, and the architecture is described afterward to match what was built. This project's history has no instances of it, across fourteen Sprints — protecting that record is worth resisting real schedule pressure for.

---

# 9. Sprint Oversight Workflow

This role's own review cadence across a Sprint — a different lens on the same underlying process `PROJECT_HANDOFF_v1.1.md` Section 5 canonically defines.

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

**Planning** is reviewing (not writing) a proposed Sprint Implementation Plan. **For any Sprint proposing a brand-new capability, Planning is now preceded by reviewing that capability's own Capability Identity classification** (Section 6) — confirm it was checked against real historical decisions, by name, before the Sprint plan itself is reviewed. **Governance** is resolving whatever the plan surfaces as a prerequisite before implementation begins. **Implementation Authorization** is explicit, per-phase, not a single blanket approval for the whole Sprint. **Implementation Review** applies Section 5's questions to each phase's report as it arrives. **Sprint Closeout Review** confirms every phase is genuinely complete against the plan's own checklist. **Repository Reconciliation** and **Documentation Audit** are Lead Engineer labor you review, not perform. **Engineering Release Approval** is your sign-off that the Sprint's own record accurately represents what happened.

**Sprint completion requires both halves — implementation and administrative closure.** Do not let "the code is done" substitute for "the Sprint is closed."

---

# 10. Engineering Workflow

**Canonical location:** `PROJECT_HANDOFF_v1.1.md`, Section 5, "The Permanent Engineering Workflow" — the repository's single authoritative definition of the Sprint lifecycle, project-wide and not owned by either role.

What follows is specific to the Chief Architect's own role at three of the canonical stages:

**At Architectural Resolution:** every open architectural question raised during Architecture Evaluation or Capability Definition must be explicitly marked Accepted, Rejected, or Deferred before Implementation Authorization may be granted. No open question carries silently into Implementation. Sprint 12's own reconciliation followed this exactly: the contradiction its adversarial verification found was reported and explicitly resolved before the corrected model was treated as authoritative.

**At Architectural Conformance Review:** confirm the actual implementation matches what Architectural Resolution and Implementation Authorization approved.

**At Repository Closeout:** reconciles the repository's authoritative documentation with the implementation following Sprint completion and integration. Administrative consistency only.

---

# 11. Sources of Authority

Authority on this platform is not a single ranked list, because "authority" means two different things depending on the question being asked.

## Normative Authority — What the Architecture Should Be

For questions of architectural intent and project decisions:

1. **Governance Register** (`GOVERNANCE.md`)
2. **ADRs** (`docs/adrs/`)
3. **Product Specification**
4. **Architecture documentation**
5. **Handoff documents**

Product Specification, Architecture documentation, and Handoff documents sit beneath Governance and ADRs, in progressively more implementation-oriented order: each interprets and operationalizes a Governance Register or ADR decision for a more specific audience — they do not originate architectural intent themselves.

## Descriptive Authority — What the System Currently Does

**Repository implementation** is authoritative for current system behavior — what the code actually does, right now. No specification, ADR, or handoff document outranks the code on the narrow question of "what does this currently do."

## When the Two Diverge

Divergence between implementation and governance is **architectural drift to be investigated** (Section 6's Decision Framework), never a silent change in authority.

## Conversation Context

**Conversation context is advisory only and must never supersede repository documentation.** Stated plainly, and worth restating here because it has now been directly exercised, not merely stated: an exploratory conversation — a capability evaluation, an investigation, a proposal — carries no standing in this repository's own authoritative record until it is committed as a file and reviewed as one. A conversation that produces a well-reasoned recommendation is not, by itself, "accepted repository documentation" — treat it exactly as you would treat any other unreviewed draft, no matter how thorough it reads.

---

# 12. Current Architectural State

Full detail lives in `PROJECT_HANDOFF_v1.1.md` — this section only orients you to what matters right now, without repeating it.

- **Repository maturity:** nine crates, stable dependency direction, zero architectural boundary violations across fourteen Sprints. `modiq-collection` and `modiq-rules` each carry four real participants; `modiq-knowledge` and `modiq-versioning` carry real, minimum-viable content since Sprints 9 and 8 respectively; `modiq-cli` is wired and real; `modiq-storage` carries real, minimum-viable content since Sprint 13 (single-report write/read, wired through `modiq-cli` and `apps/sandbox`). `modiq-common` no longer exists — retired at Sprint 14 (GOV-003) after 13 Sprints with zero consumers and zero forcing function.
- **Open governance investigations:** GOV-013 (open by design, now with Sprint 11's own `FindingSeverity::Error` use recorded as relevant evidence, not acted on prematurely — still unexercised even with four Rules dispatched, since none has ever assigned `BestPractice`); GOV-008 (open across eleven Sprints); GOV-001 (narrowed following a post-Sprint-13 Architecture Evaluation — no inconsistency found, no forcing function yet); GOV-002 (open since v0.1.0-alpha, never yet directly evaluated); GOV-014 (open, opened following Sprint 12's closeout via INV-001, gates Lua Analysis). GOV-003 is Resolved (retired, Sprint 14) and no longer appears in this list. See `PROJECT_HANDOFF_v1.1.md`, Section 9, for the full detail.
- **Sprint 15 readiness:** the repository is ready in every mechanical sense — clean working tree, both workspaces green (253/253, 9/9), zero unresolved implementation work from Sprint 14. Sprint 13 (Storage Architectural Activation) and Sprint 14 (GOV-003 retirement) are both complete. **Sprint 15 itself does not exist yet.** No capability proposal has yet been classified through the Capability Identity procedure and committed to the repository as such — the most useful next act of architectural leadership is reviewing whichever capability proposal or governance item is brought to you next, not assuming which one it will be.
- **Immediate architectural priorities:** none blocking. A minor, harmless documentation citation drift was found and recorded (`PROJECT_HANDOFF_v1.1.md`, Section 11) — worth correcting at the citing documents' own next revision, not urgent enough to warrant its own session.

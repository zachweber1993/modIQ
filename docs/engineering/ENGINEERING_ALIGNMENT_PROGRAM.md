# Engineering Alignment Program

| Property | Value |
|---|---|
| **Document** | ENGINEERING_ALIGNMENT_PROGRAM.md |
| **Project** | modIQ |
| **Purpose** | Establishes the governing engineering program for reconciling the existing deterministic platform with the frozen Version 1 Product Design and Interaction Design corpus. Organizes the work; does not perform it. |
| **Status** | Complete |
| **Precedes** | Any Architecture Evaluation against the five initiatives named below |
| **Follows** | Engineering Reconciliation (Chief Architect session record), Interaction Design Closeout & Certification |
| **As of** | 2026-07-30 |

**Closure:** This charter is complete as of 2026-07-30. All five authorized initiatives reached documented terminal states through Architectural Resolution or Governance Reconciliation. The repository now proceeds under its standard architectural and governance processes unless a new Engineering Alignment Program is explicitly authorized.

---

## 1. Purpose

The Version 1 Product Design and Interaction Design programs are complete, approved, and frozen. Both were conducted deliberately without reference to the existing engineering implementation — Product Design's own scope boundary excluded "engine implementation" from every session; Interaction Design's excluded "implementation, runtime behavior, data model" from every session in turn. This was correct discipline for those phases: a product and interaction specification constrained by what already happens to be implemented is not a specification, it's a rationalization.

That discipline has a necessary consequence: nothing in either corpus was ever checked against the platform that will have to implement it. The Engineering Reconciliation performed that check, once, deliberately, after both programs closed rather than during either — checking mid-flight would have reintroduced exactly the implementation-constrained thinking both design phases were correctly built to exclude.

**What the Engineering Reconciliation concluded:** of roughly thirty discrete behavioral and architectural responsibilities named across the corpus, the majority are Not Yet Implemented, several are Partially Implemented on a diverging representation, and two are correctly left as Product Owner decisions the corpus itself already flagged as open. Three represent genuine **architectural divergence between the existing implementation and the frozen Version 1 design corpus** — not implementation defects. Each of the three was built correctly against the architecture that existed at the time it was implemented: a terminal completion state, a mandatory rather than optional Finding→Recommendation relationship, and a single-pass synchronous execution model with no intermediate observation point. The architecture itself has since evolved through the completion of Product Design and Interaction Design, and these three points are precisely where that evolution and the prior implementation no longer agree. Grouped by root cause rather than by document section, these findings collapse into five independent architecture problems, not thirty.

**Why an Alignment Program, rather than proceeding directly to implementation:** this repository's own standing discipline (Sprint 12's Capability Identity procedure, Sprint 13's Architectural Activation precedent, GOV-004's and GOV-012's convergent-evidence bar) has never authorized implementation from a reconciliation finding directly — every prior capability, however small, passed through Architecture Evaluation and Architectural Resolution first. Five independent architecture problems, several with cross-cutting field-level consequences, are a materially larger body of architectural change than any single prior Sprint has undertaken at once. Treating that scale casually — evaluating and implementing initiative by initiative with no shared program governing sequence, ADR discipline, or Governance Register hygiene across all five — is exactly the kind of drift this document exists to prevent.

---

## 2. Architectural Philosophy

This section states the mindset governing every Architecture Evaluation conducted under this program — distinct from the operational rules in §4 (Guiding Principles), which follow from it.

- **Engineering Alignment exists to preserve the deterministic architecture while reconciling it with the frozen corpus.** It is not a license to redesign either. The deterministic engine's own Engineering Principles — determinism, explainability, readability, maintainability, crate boundaries — remain binding on every initiative's eventual resolution, exactly as they are binding on every other engineering task in this repository.
- **Existing implementation is presumed correct until evidence demonstrates architectural evolution is required.** This is not a new standard invented for this program — it is the same evidentiary bar GOV-004 and GOV-012 already established, and the same bar that produced two prior "insufficient evidence, no change authorized" outcomes (GOV-008 at Platform Validation Phase 1; GOV-001 post-Sprint 13). The five initiatives below are not pre-authorized to change anything; they are authorized to be evaluated.
- **The frozen Product Design and Interaction Design corpus is presumed equally authoritative, unless a genuine architectural inconsistency is discovered.** The presumption runs both directions. Neither the existing implementation nor the frozen corpus gets the benefit of the doubt over the other by default — evidence decides, per each initiative's own Architecture Evaluation, exactly as this role's standing constraint already requires ("if implementation conflicts with the architecture, stop and explain the conflict instead of inventing a solution").
- **The objective is the smallest architectural evolution capable of satisfying the approved specification.** Not the most elegant resolution, not the most extensible one, not the one that anticipates future capabilities this program was never asked to consider. A resolution that satisfies the frozen corpus with less change is preferred over one that satisfies it with more, even where the larger change would be architecturally cleaner in the abstract.
- **Architectural integrity is prioritized over implementation speed.** This program does not optimize for how quickly the five initiatives reach Sprint planning. A rushed Architecture Evaluation that produces a resolution requiring later correction costs more, in both engineering time and architectural trust, than a slower one that doesn't.
- **This program governs architectural decision-making. It does not perform architectural decision-making.** Every determination named in §5 below — what each initiative resolves to, whether it requires an ADR, what it costs to implement — belongs to that initiative's own future Architecture Evaluation, not to this document.

This philosophy governs every Architecture Evaluation conducted under this program. Deviation from it within any individual evaluation should itself require explicit justification, not silent drift.

---

## 3. Relationship to Existing Engineering

**This is not a greenfield implementation.** The platform already has 21 Sprints of deterministic engineering behind it: Evidence Collection (filesystem, archive, XML, runtime-log), a fixed-order Rule Engine, Version Profile compatibility, Repair Guidance, and cross-process Storage, governed by 11 accepted ADRs and a Governance Register standing at 15 items, 11 Resolved, 4 Open. That work is real, tested (253/253 at last verification), and is the thing being aligned — not the thing being replaced.

Three distinct bodies of work now exist in this repository, and this program exists specifically at the seam between them:

- **The existing deterministic platform** — what `modiq-runtime`, `modiq-rules`, `modiq-collection`, `modiq-engine`, `modiq-knowledge`, `modiq-versioning`, `modiq-report`, `modiq-storage`, and `modiq-cli` actually do today, verified directly against source, not assumed from documentation.
- **The frozen Product Design corpus** — five artifacts (`docs/product-design/`) defining what the Version 1 assessment experience *is*, conceptually: Workspace Evolution, The Finding, The Assessment Experience, Evidence, The Assessment Report.
- **The frozen Interaction Design corpus** — seven sessions (`docs/interaction-design/`) defining how a user *behaviorally* experiences that already-designed product, certified complete and implementation-ready by its own Closeout & Certification.

`docs/governance/PROJECT_STATUS.md` already records these as independent tracks: "no Sprint has yet been scoped against these Product/Interaction Design artifacts, and no engineering decision recorded below has been made in reference to them." Engineering Alignment is the first work to bring them into the same frame.

**Engineering Alignment exists to reconcile, not replace.** Nothing in this program authorizes discarding the existing deterministic engine, its crate boundaries, its Governance Register, or its ADR history. The default posture toward every one of the five initiatives is evolution of what exists — extending types, exposing new observation points, resolving previously-deferred API questions (GOV-008 already exists for exactly this reason) — not a rewrite. Where an initiative's Architecture Evaluation finds evolution is genuinely insufficient, that finding must be made explicit and justified on its own terms, not assumed as a starting posture.

---

## 4. Guiding Principles

These principles govern every Architecture Evaluation conducted under this program. They are not new — each restates an Engineering Principle or Constraint already binding on this role, applied through the Architectural Philosophy above — but are stated here together because five concurrent initiatives make silent departure from any one of them easier to miss than a single initiative would.

- **Preserve determinism.** No initiative may introduce non-deterministic behavior into Rule evaluation, Evidence interpretation, or Report generation. Progressive Execution Observability in particular must be evaluated for this explicitly — streaming presentation must never be allowed to leak into the deterministic evaluation order GOV-012 already fixed.
- **Preserve explainability.** Every new field, state, or computed value (most acutely, Confidence) must remain traceable to the evidence that produced it, per this platform's own founding discipline.
- **Minimize architectural disruption.** Prefer additive extension of existing types and entry points over restructuring. `AssessmentService`'s own Sprint 8/9 precedent — new capabilities landing with zero signature change to `execute`/`execute_from_assessment_input` — is the standing bar for what "minimal disruption" has actually looked like in this repository; it will not hold for every initiative here, but it is the default to argue away from, not toward.
- **Evolve rather than rewrite.** A finding that a type's current shape is insufficient is not, by itself, authorization to replace it. Extension, generalization, and additive fields are preferred; replacement requires its own explicit justification within the relevant Architecture Evaluation.
- **Prefer compatibility over replacement.** Existing public entry points, the Governance Register's existing Open items (GOV-001, GOV-008, GOV-013, GOV-014), and existing ADRs are inputs to each Architecture Evaluation, not obstacles to route around.
- **Every architectural change must trace back to an approved design artifact.** No Architecture Evaluation conducted under this program may introduce behavior, field, or capability that is not directly required by a specific, named section of the frozen Product Design or Interaction Design corpus, or by the existing Engineering Specification. This program organizes reconciliation of what was already approved — it is not a vehicle for new design.

---

## 5. Architectural Evolution Initiatives

Restated here at the level this program needs — organizational description, not evaluation. None of the five is assessed for feasibility, sequenced internally, or given a proposed resolution in this section; that is each initiative's own future Architecture Evaluation's job.

### Initiative 1 — Progressive Execution Observability

**Purpose:** determine whether and how a consumer can observe an Assessment's Findings as they form, rather than only receiving a completed result.

**Scope:** the Provisional/Final Finding distinction, streaming Finding arrival with additive-only presentation order, the Assessment Overview's progressive fill-in, and reconciling `AssessmentStatus`'s four execution phases against the Interaction Design corpus's three-state workspace model.

**Dependencies:** none on Initiatives 2, 3, or 4 — its root cause, `AssessmentService::execute`'s single synchronous return point, is independent of Assessment reentrancy, domain model field shape, or Confidence computation. A dependency on **Initiative 5** is introduced by this program's evaluation order (§6): Initiative 5 establishes the architectural boundary and consumption mechanism — ownership of workspace state, the shape of how the boundary is crossed — that this initiative's own observability mechanism must be designed to conform to. This does not make Initiative 1 dependent on Initiative 5 reaching a full implementation; only on Initiative 5's own boundary/ownership determination existing first.

**Governance touchpoint:** **GOV-008** (AssessmentService Public API Evolution) already exists, already Open, and is already the exact question this initiative resolves — deferred at Platform Validation Phase 1 for insufficient evidence. The Interaction Design corpus is new evidence toward the same still-open item, not grounds for a new Governance Register entry. **ADR-0009** (AssessmentService Public API Evolution) is already Accepted in anticipatory form, with the change itself explicitly not yet authorized — this initiative's Architecture Evaluation is the evidence ADR-0009 was written anticipating.

### Initiative 2 — Reentrant Assessment Lifecycle

**Purpose:** determine whether and how a Completed Assessment can accept new Evidence and produce an updated Report, rather than remaining permanently terminal.

**Scope:** Workspace Evolution's "Assessing recurring on top of Reviewing," the Updated-marker mechanism, and — as a direct consequence rather than separate scope — the Assessment Report's "living artifact" requirement, which depends entirely on this initiative's resolution and adds no independent architectural question of its own.

**Dependencies:** none on Initiatives 1, 3, or 4, for the same reason as Initiative 1 — this is a lifecycle/invariant question (`AssessmentStatus::Completed`'s current terminal enforcement, INV-012), independent in kind from execution-shape questions. The same dependency on **Initiative 5** applies as for Initiative 1: whether and how reentrancy is exposed across the engine/presentation boundary is constrained by that boundary's own ownership and mechanism decisions.

**Governance touchpoint:** no existing Governance Register item addresses reentrancy or supplementation. A new item is expected to be opened at this initiative's Architecture Evaluation, following this repository's standing practice of opening a Governance Register item at the point a genuine architectural question is first formally evaluated, not before.

### Initiative 3 — Domain Model Anatomy Extension

**Purpose:** determine whether and how `Finding`, `Evidence`, `Recommendation`, `RepairRecipe`, and `AssessmentReport` should be extended to carry the vocabulary Product Design specifies — Title/Summary decomposition, Finding-level Category (Mod Health dimension), optional rather than mandatory Recommendation cardinality, Evidence's Label/Source/Explanation/Content anatomy, RepairRecipe's structured-step shape, and Report Identity/currency fields.

**Scope:** field-level and type-shape questions across `modiq-runtime` and `modiq-report`, with a necessary downstream consequence for `modiq-storage`'s persisted representation (a migration of an existing pattern, not an independent architectural question).

**Dependencies:** none that block evaluation, but a genuine efficiency dependency worth naming: Initiatives 1 and 2 each introduce their own new fields onto the same structs this initiative reshapes (Provisional/Final status onto Finding; the Updated marker onto Finding and AssessmentReport). Evaluating this initiative before 1 and 2 resolve risks designing this initiative's field set once, then revising the same types again once 1 and 2 land theirs.

**Governance touchpoint:** **GOV-013** (FindingSeverity Severity/Kind Conflation), already Open, touches the same `FindingSeverity` type this initiative would otherwise leave untouched — its resolution should be considered within or immediately alongside this initiative's Architecture Evaluation rather than independently. The Glossary's own named gap (no standalone "Recommendation" definition, flagged by Product Design Closeout) is directly adjacent and should be raised to the Product Owner in the same cycle, though its resolution is a governance/documentation decision, not an architectural one this initiative performs itself. A specific naming collision — the design corpus's Finding-level "Category" versus the already-existing, differently-shaped `EvidenceCategory` enum — must be resolved within this initiative's own Architecture Evaluation before any field is added under either name.

### Initiative 4 — Confidence as a First-Class Concept

**Purpose:** determine what constitutes evidence quality for a given Recommendation, and where in the pipeline that determination is computed.

**Scope:** Confidence's total current absence — no type, field, or computation exists anywhere in the platform today.

**Dependencies:** distinct in kind from Initiative 3 — every other domain-model gap is a reshaping of content a Rule already produces; this one requires new interpretive logic before any field has a value to hold. A light sequencing dependency exists on Initiative 3: Confidence attaches to a Recommendation, and Initiative 3 is separately resolving what a Recommendation's own shape and cardinality should be. Evaluating Confidence against a Recommendation shape that is itself still in flux risks the same rework concern named under Initiative 3.

**Governance touchpoint:** no existing Governance Register item addresses Confidence computation. A new item is expected. Worth naming for that evaluation's own benefit, not decided here: Confidence is not a new concept being introduced to the platform — it is already a defined constitutional term (Glossary: "Confidence reflects evidence quality, not correctness," cited repeatedly throughout the frozen design corpus) that has simply never been implemented. Whether this initiative is better classified under this repository's existing Architectural Activation precedent (Sprint 8, Sprint 13) rather than as a novel capability is a question for its own Architecture Evaluation, not resolved by this program.

### Initiative 5 — Production Interaction Layer Definition

**Purpose:** determine the architectural boundary between the deterministic engine and the presentation/interaction layer that will consume it. This is explicitly not a technology or framework choice.

**Scope:** ownership of workspace state (engine-owned versus consumer-owned), the shape of the mechanism by which the boundary is crossed (for instance, synchronous request/response versus some other model — a boundary-shape question, not an implementation choice), the intended relationship between a future production application and `apps/sandbox` (explicitly non-production today), and the *nature*, though not the concrete payload shape, of the consumer contract. Concrete API payload shapes remain out of scope here and stay downstream of Initiatives 1–4.

**Dependencies:** none on Initiatives 1–4 — this is precisely why, on reconsideration (§6), this initiative is positioned first rather than last. Its scope at this stage never touches those initiatives' concrete field-level outcomes, only the shape of the channel those outcomes will eventually travel through. A light, expected follow-on exists in the other direction: once Initiatives 1–4 resolve, this initiative's own contract-shape detail (not its boundary or ownership decision) may warrant a narrow revisit to accommodate whatever concrete data shapes those initiatives actually produce.

**Governance touchpoint:** this initiative does not fit the shape either of this repository's two existing classification procedures was built for — Sprint 12's Capability Identity procedure classifies Collector/Rule-shaped work; Sprint 13's Architectural Activation precedent classifies subsystem-level activation of already-named System Overview components. Standing up a production consumer application is neither. This initiative's own Architecture Evaluation may need to begin by explicitly naming this as a fourth, previously unclassified shape of architectural question, rather than forcing it into either existing procedure — that determination is this initiative's to make, not this program's.

---

## 6. Recommended Evaluation Order

**Recommended sequence:** Initiative 5 first; Initiatives 1 and 2 second, concurrently or in either order relative to each other; Initiative 3 fourth; Initiative 4 fifth.

This revises the sequence originally proposed in this document's first draft, which placed Initiatives 1 and 2 first and Initiative 5 last. The reconsideration is recorded here in full, not simply adopted, per this program's own traceability discipline (§9).

**Why Initiative 5 now precedes Initiatives 1 and 2.** The original ordering reasoned from the producer outward: it treated `AssessmentService`'s own execution and lifecycle model as the more foundational layer, with the consumer boundary evaluated last, against whatever that producer model resolved to. That reasoning did not adequately weigh a fact specific to this platform's current state: the producer side of this boundary is mature — 21 Sprints, a settled domain model, real invariants — while the consumer side has no real existence at all. `apps/sandbox` is explicitly documented as non-production and cannot inform this decision. Evaluating Initiative 1's observability mechanism or Initiative 2's reentrancy mechanism before the engine/presentation boundary itself is defined risks designing each against an assumed consumption model — in-process, push-based, or otherwise — that Initiative 5 might later reject in favor of a differently-shaped boundary (for instance, a separate-process, request/response model, consistent with `apps/sandbox`'s own existing pattern and `Architecture.md`'s own Platform Boundaries language: "presentation systems should consume assessment outputs without influencing assessment logic"). A boundary decision made after its own internal mechanisms are already chosen is not really a boundary decision — it is a description of whatever those mechanisms already assumed.

**The scope constraint this reordering requires, and that Initiative 5's own Architecture Evaluation must hold.** Initiative 5 is a boundary, ownership, and mechanism decision — not a technology or framework choice, and not a decision about the concrete payload shape any future contract carries (§5). Confining it to that scope is what makes evaluating it first architecturally sound rather than merely convenient: it does not require Initiatives 1–4 to have resolved anything, because it never depends on their concrete field-level outcomes, only on the shape of the channel those outcomes will eventually travel through. Initiative 5 exceeding that scope — specifying exact API payloads before Initiatives 1–4 exist to define them — would reintroduce the original ordering's own risk, in reverse.

**Why Initiatives 1 and 2 remain a tied pair, evaluated second.** Unchanged from the original draft: each is independent of the other and of every initiative except their now-shared dependency on Initiative 5's boundary determination.

**Why Initiative 3 still follows 1 and 2, and Initiative 4 still follows 3.** Also unchanged. Both arguments concern field-level rework risk between initiatives that reshape the same Runtime types — a concern Initiative 5's repositioning does not affect.

This ordering is a recommendation for Architecture Evaluation sequencing, not a Sprint schedule and not an implementation order — see §8.

---

## 7. ADR Expectations

Per `docs/adrs/README.md`'s own lifecycle (issue → alternatives → decision → review → approve → implement), an ADR records a decision already reached — it does not precede the evaluation that reaches it. **No ADR should be created for any of the five initiatives until that initiative's own Architecture Evaluation has produced an Architectural Resolution.** This program authorizes no ADR directly and drafts none.

One existing exception, already on the record and worth naming precisely: **ADR-0009** (AssessmentService Public API Evolution) is already Accepted, in explicitly anticipatory form, with the underlying change itself not yet authorized — recorded at the time as anticipating exactly the evidence Initiative 1 is now positioned to provide. Initiative 1's Architecture Evaluation should determine whether its resolution is satisfied by ADR-0009 as written, requires a superseding ADR (per the same "supersede, don't modify" discipline ADR-0011 already established for ADR-0003), or requires no ADR action at all if evidence again proves insufficient — mirroring GOV-008's own prior outcome at Platform Validation Phase 1.

For Initiatives 2 through 5, new ADR numbers (ADR-0012 onward, sequential, never pre-allocated) are expected only if and when each initiative's Architecture Evaluation concludes with an Architectural Resolution significant enough to meet `docs/adrs/README.md`'s own creation criteria (changes platform architecture, introduces a significant constraint, alters ownership boundaries, affects multiple components, or has long-term maintainability consequences). An Architecture Evaluation that finds insufficient evidence to resolve, or resolves with no architectural change required, produces no ADR — this has already happened twice in this platform's history (GOV-008 at Platform Validation Phase 1; GOV-001 post-Sprint 13) and is not a failure mode for this program to guard against, only one to expect as a legitimate outcome.

---

## 8. Sprint Planning Strategy

**Implementation planning begins only after Architecture Evaluation and governance decisions are complete for a given initiative** — mirroring the sequence this repository has used for every prior capability: Architecture Evaluation → Architectural Resolution → Implementation Authorization → Sprint Plan → Implementation → Engineering Release. No initiative in this program skips a stage of that sequence, and no Sprint may be opened against an initiative before its own Implementation Authorization exists.

A completed Architecture Evaluation becomes a Sprint input in exactly the way Sprint 8's and Sprint 13's own Architectural Resolutions became their respective Implementation Authorizations — translated into an engineering envelope with explicit scope boundaries, not assumed to authorize open-ended follow-on work.

Because the five initiatives are architecturally independent apart from the dependencies named in §5–§6, each may reach Implementation Authorization and enter Sprint planning on its own timeline — this program does not require all five to complete Architecture Evaluation before the first Sprint against any of them begins. It does require that a given initiative's own Sprint not begin before that same initiative's own Architectural Resolution and Implementation Authorization exist. Cross-initiative Sprint sequencing (which authorized initiative is implemented first, whether any run concurrently) is a Sprint-planning decision for the point at which more than one initiative has reached Implementation Authorization — not a decision this program makes in advance.

---

## 9. Repository Discipline

Engineering Alignment introduces no new repository discipline — it operates entirely under discipline already established and reaffirmed here for this program's specific scale:

- **Review before implementation.** No initiative proceeds to Sprint planning without a reviewed Architecture Evaluation and, where warranted, Architectural Resolution.
- **Repository-first decisions.** Every Architecture Evaluation conducted under this program must ground its evidence in the repository as it currently exists — the existing deterministic platform's real source, not this program's or the Engineering Reconciliation's own descriptions of it, which may drift from the code as initiatives land.
- **Atomic synchronization.** Each initiative's governance and documentation changes (Governance Register entries, ADRs, Sprint records, Engineering Releases) are synchronized together, per `RepositorySynchronizationPolicy.md`, never partially.
- **Approval before commit.** This document itself is not synchronized until explicitly approved — no Architecture Evaluation under this program synchronizes its own findings before equivalent review.
- **Architectural traceability.** Every initiative's eventual resolution must trace back to a specific section of the frozen design corpus (per §4's compatibility principle) and, where applicable, to the existing ADR or Governance Register item it evolves rather than replaces. This document's own §6 revision is itself an instance of that discipline — recorded as a reconsideration with reasoning, not a silent edit.

---

## 10. Success Criteria

The Engineering Alignment Program is complete when:

- All five initiatives have completed their own Architecture Evaluation, each reaching either an Architectural Resolution or an explicit, evidenced finding that resolution is not yet warranted (a legitimate outcome, not a gap — see §7).
- Every resulting ADR, superseding ADR, or explicit no-ADR determination is recorded per §7.
- The Governance Register reflects a final, accurate status for every item this program touches or opens (GOV-008, GOV-013, and any newly opened items for Initiatives 2, 4, and 5).
- `docs/governance/PROJECT_STATUS.md`, `docs/governance/CHANGELOG.md`, and `docs/engineering/GOVERNANCE.md` are reconciled to reflect the program's outcome, per this repository's own closeout discipline.
- A Sprint Roadmap exists sequencing whatever implementation was actually authorized across the five initiatives' resolutions — not a roadmap this program produces itself, but the artifact its completion enables.

Completion of this program is a **governance and architectural readiness state, not a code state** — consistent with how Interaction Design Closeout & Certification and prior Documentation Releases were judged complete in this repository. This program does not end merely with permission to start writing code: it ends with the architecture itself reconciled against the frozen Product Design and Interaction Design corpus, the Governance Register and ADR record updated to reflect every resolution reached, implementation authorized wherever an Architectural Resolution warranted it, and a stable, reconciled engineering specification in place for Sprint planning to proceed against. It does not itself perform that implementation.

---

## Scope Note

This document does not evaluate, resolve, or propose a solution for any of the five initiatives. It does not modify any Product Design or Interaction Design artifact, and treats both corpora as frozen constraints throughout. It does not draft, propose, or pre-allocate any ADR. It does not invent an implementation detail, API shape, or data model change beyond what the Engineering Reconciliation already found and this document organizes. It establishes process and sequence only.

---

## Update Discipline

This document should be updated only as initiatives complete their own Architecture Evaluation and Architectural Resolution stages, recording outcome and status — not incrementally redesigned as individual evaluations proceed. Substantive revision to §2 (Architectural Philosophy), §4 (Guiding Principles), or §6 (Recommended Evaluation Order) should be treated as a program-level decision requiring the same review this document itself received, not an editorial update.

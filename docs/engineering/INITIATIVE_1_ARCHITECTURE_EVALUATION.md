# Initiative 1 Architecture Evaluation — Progressive Execution Observability

| Property | Value |
|---|---|
| **Document** | INITIATIVE_1_ARCHITECTURE_EVALUATION.md |
| **Project** | modIQ |
| **Initiative Addressed** | Engineering Alignment Program, Initiative 1 — Progressive Execution Observability (`docs/engineering/ENGINEERING_ALIGNMENT_PROGRAM.md` §5) |
| **Purpose** | Determine what current repository evidence supports regarding progressive execution visibility during an Assessment — not implementation, not API design, not Architectural Resolution. |
| **Origin** | Chief Architect authorization, following completion of Initiative 5 (Architecture Evaluation and adopted Architectural Resolution, `e1cbb05`). |
| **Adopted Precedent** | `docs/engineering/INITIATIVE_5_ARCHITECTURAL_RESOLUTION.md`, treated as binding, unreopened repository architecture throughout — cited where directly relevant, never re-evaluated. |
| **Status** | **Architecture Evaluation only. No Architectural Resolution has been performed. No ADR, crate, API, transport, or implementation technology has been created, designed, or chosen.** |

---

## 1. Evaluation Scope

Per the task defining this evaluation, this document determines what repository evidence supports regarding progressive execution visibility — not how it would be built.

**Deliberately excluded**, per explicit instruction:

- UI technology, IPC, REST, GraphQL, WebSockets, events, polling, channels, async implementation, threading, DTOs, serialization, transport, or any concrete API.
- Implementation planning, Sprint planning, ADR creation, Governance Register updates, repository synchronization.
- Re-evaluation of any question Initiative 5 already adopted (Decisions 1, 2, 3, 4a, 4b's ownership half, 5a, 6) — those are treated as fixed inputs throughout.

---

## 2. Repository Evidence

Reviewed directly this session:

- **`docs/architecture/DataModel.md`** — read in full: Runtime Lifecycle (the seven-step conceptual sequence), the Immutability principle, and the Finding entity definition (no provisional/incremental concept named anywhere).
- **`docs/architecture/Architecture.md`** — Assessment Lifecycle (the eight-step pipeline diagram) and Information Flow (Input → Evidence → Rule Evaluation → Findings → Assessment Report → User Understanding) sections, read directly.
- **`docs/architecture/RuleEngine.md`** — read in full: Execution Principles (Deterministic, Evidence-Based, Explainable), the six Rule Engine Responsibilities, the Conceptual Execution Lifecycle (mirroring `DataModel.md`'s own sequence), and Future Evolution.
- **`docs/implementation/RuntimeInvariants.md`** — all fourteen invariants, re-consulted specifically for what they do and do not govern (mutation versus observation).
- **`crates/modiq-runtime/src/assessment/assessment.rs`** — re-confirmed directly: `AssessmentStatus`'s four phases gate every mutation method; none references or implies anything about external read access.
- **`crates/modiq-engine/src/engine/assessment_service.rs`** — `AssessmentService::execute`'s and `execute_from_assessment_input`'s own control flow, confirmed as single synchronous calls with exactly one return point each.
- **`crates/modiq-report/src/report/assessment_report.rs`** and **`crates/modiq-rules/src/rules/engine.rs`** — re-confirmed: `AssessmentReport::generate` is a single, one-shot snapshot function; `RuleEngine::evaluate` returns one complete `Vec<RuleOutcome>` from one synchronous call.
- **`docs/engineering/GOVERNANCE.md`** — GOV-001 (Assessment Report Generation Timing), GOV-008 (AssessmentService Public API Evolution), GOV-012 (Rule Evaluation Model, Resolved — fixed declaration-order dispatch), GOV-013 (FindingSeverity Severity/Kind Conflation, tangential).
- **`docs/adrs/0005-deterministic-assessment-engine.md`** — re-read for its determinism requirement, checked against progressive visibility for conflict.
- **The complete Product Design and Interaction Design corpus** — already read in full during the Engineering Reconciliation and Initiative 5's own evaluation this session; re-consulted here specifically for Workspace Evolution §2 (Provisional/Final) and Assessing & Progressive Discovery (entire document).
- **`docs/engineering/ENGINEERING_ALIGNMENT_PROGRAM.md`** — this initiative's own governing scope definition (§5) and its already-recorded dependency on Initiative 5 (§6).
- **`docs/engineering/INITIATIVE_5_ARCHITECTURE_EVALUATION.md`** and **`INITIATIVE_5_ARCHITECTURAL_RESOLUTION.md`** — consulted as adopted precedent only, specifically Decision 3 (Assessment state ownership), Decision 4b (workspace state derivation), and Decisions 5a/5b (mechanism baseline, adopted; supplementary signal, deferred to this initiative).

---

## 3. Current Architecture

**Every conceptual lifecycle diagram in the frozen specification is strictly linear and single-pass, with no intermediate observation point named anywhere.** `DataModel.md`'s Runtime Lifecycle, `Architecture.md`'s Assessment Lifecycle, and `RuleEngine.md`'s Conceptual Execution Lifecycle are three independently-authored documents that converge on the same seven- or eight-step sequence, each proceeding from creation to a single terminal report/completion step. None branches, none repeats, none names a point at which a caller might observe a partial result.

**`Architecture.md`'s own Information Flow diagram places "User Understanding" strictly downstream of a complete "Assessment Report":**

```
Input → Evidence → Rule Evaluation → Findings → Assessment Report → User Understanding
```

As currently specified, this is a single terminal delivery point, not a stream — this is the frozen document's own structure, not an interpretation of it.

**`AssessmentService::execute` is a single synchronous function with exactly one return point.** Re-confirmed directly against `assessment_service.rs`: the entire pipeline (Evidence Collection, Rule Evaluation, Finding/Recommendation accumulation, Report generation, completion) executes inside one function call. No external caller holds any reference to the `Assessment` while it runs — it is constructed, mutated, and consumed entirely within that one private scope before anything is returned.

**The barrier to progressive visibility is the Engine API's own control-flow shape, not any Runtime domain invariant.** This distinction is load-bearing and worth stating precisely: INV-002, INV-003, INV-004, INV-010, INV-011, and INV-012 each govern *mutation* — what may be added, when, and in what order — and say nothing about *observation*. INV-012 ("A Completed Assessment SHALL reject further lifecycle transitions") forbids further mutation after completion; it does not forbid, and was never written to address, whether an in-progress Assessment's current state could be read by something outside the function currently mutating it. Nothing in `modiq-runtime` prevents progressive visibility. What prevents it today is that `AssessmentService::execute`'s own synchronous, single-return shape gives nothing outside itself anything to read from until it is already finished.

**`GOV-001`'s own finding, directly relevant here:** `AssessmentReport::generate` is called exactly once per execution, always immediately before `Assessment::complete()`, for both public entry points — confirmed by an existing test (`execute_reflects_state_at_report_generation_prior_to_completion`). No earlier or intermediate Report generation exists anywhere in the pipeline today; a Report is a wholly post-hoc artifact of the current architecture.

**Findings are produced in a single atomic batch, immutable once added, with no incremental or provisional concept anywhere in the current code or specification.** `RuleEngine::evaluate` returns its complete `Vec<RuleOutcome>` from one synchronous call (per GOV-012's already-resolved fixed-declaration-order dispatch); `Assessment::add_finding` only accepts calls during `EvaluatingRules`, and nothing mutates a Finding once added. Evidence Collection is likewise single-batch today — no Collector's output is individually exposed before the whole collection phase completes.

**`DataModel.md`'s Immutability principle ("Completed Assessments represent historical records... a new Assessment should be performed rather than modifying previous Assessment results") governs post-completion modification, not intra-execution progressive visibility.** This is a distinct question from the one this evaluation addresses — see §10 (Initiative Boundaries).

---

## 4. Question 1 — Progressive Assessment Visibility

**Does repository architecture support exposing assessment progress before completion, or is completion currently terminal?**

**Observation:** completion of the *synchronous function call* is what is currently terminal — not completion of the Assessment in the Runtime-invariant sense. No Runtime invariant forbids progressive visibility; equally, nothing in the frozen conceptual specifications (`DataModel.md`, `Architecture.md`, `RuleEngine.md`) authorizes or names a mechanism for it. The specifications are silent on external observability, not opposed to it — they describe a sequence of what happens, never who may observe it or when.

**Conclusion:** current architecture does not support exposing progress before completion — not because it is architecturally forbidden, but because its own control-flow shape structurally forecloses it. No external reference to an in-progress Assessment exists at any point during `execute`'s single synchronous call, by construction. This is an absence of provision, not a prohibition — the same character the Engineering Reconciliation already found for most of the reconciled corpus, now traced to its precise architectural cause.

---

## 5. Question 2 — Intermediate Execution State

**Should intermediate execution state become architecturally visible? If so: what state, when, and why?** (Transport and API deliberately excluded.)

**What:** the growing, not-yet-final set of Findings — each carrying its complete anatomy the moment it exists, per Interaction Design's own explicit requirement that a Finding is never observed in a partial-field state — together with whatever completeness picture the Overview requires (which categories of the Assessment have been reached). The concrete field-level shape this state would take is Initiative 3's own territory (§10); this evaluation addresses only whether such a conceptual state exists to expose.

**When:** repository evidence points toward two natural boundaries the current architecture's own internal structure already has, not an arbitrary cadence — after each Evidence Collector completes (Evidence-level granularity) and after each Rule's already fixed-order dispatch produces its outcome (Finding-level granularity, per GOV-012's already-adopted dispatch order, Resolved). This is continuous, evidence-paced observation, not a percentage or time-based signal — consistent with Assessing & Progressive Discovery's own explicit rejection of progress-bar semantics: "There is no percentage-complete indicator and no time estimate anywhere in this design."

**Why:** Interaction Design names this as a defining behavioral law, independently arrived at and reused across multiple sessions — Findings appear "as they're produced, not batched for a single reveal," and the Overview "fills in progressively." Separately, and independently, `Architecture.md`'s own Information Flow diagram, read exactly as currently specified, places User Understanding strictly downstream of a complete Report — precisely the tension this initiative exists to evaluate.

**A constraint carried forward from already-adopted governance, not decided here:** whatever observation points are eventually chosen must preserve GOV-012's fixed-declaration dispatch order — a consumer must never be able to infer or depend on Evidence-arrival-driven ordering merely because it happens to observe outcomes arriving in some sequence. This is a boundary condition for a future Resolution, not a new decision this evaluation makes.

---

## 6. Question 3 — Execution Phases

**Does architectural alignment require observable execution phases?**

**Observation, Runtime's actual lifecycle:** `AssessmentStatus` has four phases (`Created`, `CollectingEvidence`, `EvaluatingRules`, `Completed`), confirmed directly against `assessment_status.rs` and its enforcement in `assessment.rs`.

**Observation, Product Design's workspace model:** Workspace Evolution specifies three states (Intake, Assessing, Reviewing). Already adopted by Initiative 5 (Decision 4b): Intake has no engine analog at all — the engine's own lifecycle machine does not begin until `Assessment::new` is called. Assessing and Reviewing, by contrast, are consumer-side interpretations of *some* engine-owned execution-phase signal — Initiative 5 adopted that they derive from one, while explicitly deferring which one to this initiative.

**Observation, Interaction Design's own treatment:** Assessing & Progressive Discovery never asks a user to distinguish evidence-gathering from rule-evaluation — it treats both as one continuous, undifferentiated "Assessing" experience. This suggests `CollectingEvidence` and `EvaluatingRules` may correspond to a single experiential phase, not two, though the document never states this explicitly since it was written without reference to Runtime's own internal phase names.

**Conclusion:** architectural alignment requires that at least one execution-phase signal become observable. This evaluation confirms, rather than merely repeats, Initiative 5's own working assumption: without some observable phase signal, Workspace Evolution's own adopted derivation model (Decision 4b) has nothing to derive Assessing/Reviewing from, and the two could not be distinguished at all by any future consumer.

**Left genuinely open, not resolved here:** whether the visible signal should be Runtime's own existing four-phase machine as-is, a coarser two-phase view (folding `CollectingEvidence` and `EvaluatingRules` into one signal, matching Progressive Discovery's own undifferentiated treatment), or some other shape is a real question this evaluation surfaces without deciding.

---

## 7. Question 4 — Findings

**Are Findings architecturally provisional, immutable, replaceable, incremental, finalized only at completion, or something else?**

**Something else, precisely:** Findings are immutable once added to the Assessment — no update path exists anywhere in the code — and are all added within a single atomic batch (`RuleEngine::evaluate`'s one synchronous call), not incrementally, provisionally, or replaceably in any sense the current code or specification expresses. They become permanently fixed the moment they are added, during `EvaluatingRules`, not specifically at the `Completed` transition — `complete()` itself performs no further transformation (confirmed by GOV-001's own cited test, `generate_after_completion_matches_generate_before_completion`).

**A finding worth surfacing explicitly:** Interaction Design's own requirement is that Findings become Final "together, as a set," at one transition — not individually, incrementally finalized one at a time. This means the architectural gap is narrower than it first appears: what is needed is not a mechanism for finalizing each Finding independently, but a way to *observe the growing, not-yet-final set* while it accumulates, with the whole set finalizing together at one point — which is considerably closer to the current architecture's already batch-oriented shape (one atomic production event) than a fully per-item incremental model would be.

---

## 8. Question 5 — Report Evolution

**Is the Assessment Report architecturally immutable, append-only, living, regenerated, or another model?** (Data structures deliberately excluded.)

**Observation:** today's `AssessmentReport` is generated exactly once, immediately before completion, and never regenerated or appended to (GOV-001) — a strictly one-shot, post-hoc artifact of a completed execution.

**The tension, stated precisely:** Workspace Evolution requires the Overview to be "visible from the moment any Evidence exists" and to "fill in progressively" throughout Assessing. A generation model that produces exactly one Report, only after the entire pipeline has already finished, cannot satisfy a requirement that the Overview exist and change *during* execution — the two are structurally incompatible as currently specified and implemented. This is not a gap that better wording or a minor extension closes; it is a genuine architectural tension between the current generation model and an explicit, frozen product requirement.

**Conclusion: this tension requires future architectural resolution.** Which model resolves it, and by what mechanism, is not decided here — this evaluation's role is to identify that the tension exists and is real, not to select or compare candidate resolutions. Resolving it is properly downstream of this evaluation's own Question 3 conclusion (that an execution-phase signal must exist), not prior to it.

**Two distinct questions worth naming precisely so they are not conflated, both already surfaced in §10:** whether an *in-progress* Report may change before completion is this initiative's own territory; whether a *completed, historical* Report may later be modified is Initiative 2's territory. GOV-001's own still-open narrower question (whether `Completed` status is ever practically reached by a real, persisted report) is related to both but resolved by neither.

---

## 9. Question 6 — Interaction Timing

**What interaction expectations require observable engine progress?** (Consumer implementation deliberately excluded.)

Drawn directly from Interaction Design, not re-derived:

- Assessing & Progressive Discovery's entire premise: no blank wait state; Findings appear as they are produced; the Overview fills in progressively; pacing must be "honest in both directions" — never padded to seem thorough, never batched to reveal instantly.
- No percentage-complete indicator or time estimate anywhere in the design — ruling out any observable signal shaped like a progress bar.
- Assessment Intake & Upload §7: Intake itself establishes, ahead of time, "that understanding will arrive progressively rather than all at once, and that additional material remains welcome throughout" — an expectation set before any Finding exists, which the engine's own behavior must then honor.
- The cross-document "quiet completion" pattern, independently named in at least three Interaction Design sessions: nothing is ever announced as complete by the product itself — a consumer recognizes completion by the *absence* of remaining Provisional content, never by an explicit "done" signal. Whatever becomes observable must support recognition-by-absence, not rely on an explicit completion event.

---

## 10. Initiative Boundaries

Explicitly identified, not partially resolved:

**Initiative 2 (Reentrant Assessment Lifecycle):** whether and how a Completed Assessment can be reopened to accept new Evidence; `DataModel.md`'s Immutability principle in its post-completion sense; the Updated-marker mechanism; GOV-001's still-open question of whether `Completed` status is ever practically reached by a real persisted report.

**Initiative 3 (Domain Model Anatomy Extension):** the actual field or type needed to represent a Provisional/Final status on a Finding; Title/Summary decomposition; Finding-level Category (Mod Health dimension); any new field this evaluation's own conclusions (§5, §7) would eventually require once a Resolution adopts them.

**Initiative 4 (Confidence as a First-Class Concept):** unaffected by anything in this evaluation. Confidence remains excluded from any not-yet-final Finding regardless of this initiative's outcome — already fixed by Workspace Evolution's own frozen design, not something this evaluation touches or depends on.

**Initiative 5 (already resolved — cited as precedent, not reopened):** the mechanism baseline (request/response, Decision 5a) is adopted and unaffected by anything here. The supplementary notification/poll-trigger sub-question (Decision 5b) was explicitly deferred *to* this initiative's own conclusions — this evaluation's Question 1 and Question 3 findings (progressive visibility is architecturally unsupported today; an execution-phase signal is required) are what Decision 5b's own eventual resolution will depend on, not the reverse.

---

## Evaluation Boundaries

**Questions this evaluation answered:**
- Whether current architecture supports exposing assessment progress before completion (§4) — it does not, and the barrier is the Engine API's control-flow shape, not any Runtime invariant.
- What conceptual intermediate state, and at what natural boundaries, repository evidence points toward exposing (§5) — the growing, not-yet-final Finding set, at Evidence-Collector and per-Rule-dispatch boundaries.
- Whether observable execution phases are architecturally required (§6) — yes, confirming Initiative 5's own working assumption, though which phase granularity remains open.
- What Findings architecturally are today (§7) — immutable once added, produced in one atomic batch, with no incremental or provisional concept currently expressed.
- That a genuine tension exists between the current one-shot Report generation model and Interaction Design's progressive Overview requirement, and that it requires future architectural resolution (§8).
- What Interaction Design requires of observable engine progress, independent of any consumer implementation (§9).

**Questions intentionally left to later initiatives (§10):** reentrant/supplemental Assessment lifecycle and post-completion Report modification (Initiative 2); the concrete field-level representation of any state this evaluation found conceptually necessary, and the mechanism resolving the Report-evolution tension (Initiative 3); Confidence computation, unaffected throughout (Initiative 4).

**Adopted Initiative 5 decisions relied upon without reopening:** Decision 3 (Assessment state is exclusively engine-owned, per INV-006/INV-009) — assumed throughout as the boundary any exposed intermediate state must respect. Decision 4b (workspace state is consumer-owned but derived from an engine execution-phase signal for Assessing/Reviewing, with Intake having no engine analog) — treated as the premise this evaluation's Question 3 tests and confirms, not reopens. Decision 5a (request/response is the adopted mechanism baseline) — assumed fixed; no transport question is addressed here. Decision 5b (a supplementary signal mechanism is deferred pending this initiative) — this evaluation supplies the finding that deferral was waiting on, without itself resolving 5b, which remains Initiative 5's own document to eventually amend or for a future governance action to carry forward.

No Architectural Resolution has been performed. This evaluation's responsibility ends here, awaiting Chief Architect review.

# Initiative 1 Architectural Resolution — Progressive Execution Observability

| Property | Value |
|---|---|
| **Document** | INITIATIVE_1_ARCHITECTURAL_RESOLUTION.md |
| **Project** | modIQ |
| **Initiative Addressed** | Engineering Alignment Program, Initiative 1 — Progressive Execution Observability |
| **Purpose** | Evaluate each recommendation made by `docs/engineering/INITIATIVE_1_ARCHITECTURE_EVALUATION.md` and determine which becomes repository architecture. |
| **Primary Evidence Source** | `docs/engineering/INITIATIVE_1_ARCHITECTURE_EVALUATION.md` (commit `38df9f3`), treated as fixed, verified evidence — not reopened or re-derived. |
| **Repository Verification** | Confirmed immediately prior to drafting: `38df9f3` remains `HEAD`, working tree clean, no repository change of any kind since the Evaluation was committed and pushed. No fact in the Evaluation is stale. |
| **Status** | **Architectural Resolution complete. No ADR created. No Governance Register entry modified. No implementation mechanism, API, or transport chosen.** |

---

## 1. Method

This Resolution uses the same disposition standard established by `INITIATIVE_5_ARCHITECTURAL_RESOLUTION.md`, extended by one distinction the Evaluation's own evidence warrants:

- **Adopted** — a confirmed architectural fact or conceptual conclusion the Evaluation's evidence supports, settled as repository ground truth going forward.
- **Adopted Architectural Constraint** — a binding principle that every future implementation of this initiative must satisfy, distinct from a settled fact: it does not itself describe *what exists*, it describes *what any future design is not free to violate*. Used where the Evaluation's evidence supports a durable "shall" statement, not merely a classification.
- **Deferred** — sound but genuinely incomplete pending either another initiative's own resolution or Initiative 1's own future design work; named precisely, not guessed at.
- **Rejected** — repository evidence contradicts the recommendation. (None found; see below.)
- **Requires Additional Investigation** — the evidence is genuinely inconclusive and needs a dedicated evidence-gathering activity. (None found; see below.)

No question already settled by the Evaluation is revisited here except where restating it precisely enough to assign a disposition requires it.

---

## 2. Adopted Facts

Confirmed architectural conclusions, settled as repository ground truth. These describe the current state and what the Evaluation's evidence establishes about it — not obligations on future design (see §3 for those).

**F-1 — Progressive visibility is not currently supported.** Current architecture does not expose assessment progress before completion. This is settled: `AssessmentService::execute`'s single synchronous return point admits no external reference to an in-progress Assessment at any point during its execution.

**F-2 — The correct conceptual state for future visibility work is the growing, not-yet-final Finding set.** Together with whatever completeness picture the Overview requires, this is the state repository evidence (Interaction Design's own progressive-discovery requirements) points toward — not a percentage, not a raw execution trace, not individual Evidence items.

**F-3 — A genuine architectural tension exists between the current one-shot Report generation model and Interaction Design's progressive-Overview requirement.** This tension is real, confirmed by GOV-001's own finding that `AssessmentReport::generate` is called exactly once, always post-hoc. It requires future resolution. Which model resolves it is not adopted here (§4).

---

## 3. Adopted Architectural Constraints

Binding principles every future implementation of this initiative must satisfy. Each traces to specific Evaluation evidence; none specifies a mechanism, API, or transport.

**AC-1 — The Engine orchestration layer, never the Runtime domain, is where progressive visibility must be established.** Any future implementation shall be built within the Engine orchestration layer. It shall never require weakening, reinterpreting, or bypassing any Runtime domain invariant (INV-002, INV-003, INV-004, INV-010, INV-011, INV-012) — none of which currently blocks progressive visibility, and none of which is to be loosened in order to enable it. *(Evaluation §4, §3 "Current Architecture.")*

**AC-2 — The engine shall expose observation only at architecturally coherent execution boundaries.** Specifically, boundaries the platform's own dispatch order already establishes — per-Evidence-Collector completion, and per-Rule dispatch order under GOV-012's already-resolved fixed declaration order — never an arbitrary, percentage-based, or time-based cadence. *(Evaluation §5.)*

**AC-3 — At least one observable execution-phase signal shall exist.** Without one, Workspace Evolution's own already-adopted derivation model (Initiative 5, Decision 4b) has nothing to derive Assessing/Reviewing from, and the two could not be distinguished by any future consumer. *(Evaluation §6.)*

**AC-4 — Any Finding-visibility mechanism shall preserve set-wide finalization.** Findings become Final together, as one transition, never individually or independently finalized ahead of the rest of the batch they belong to — consistent with Interaction Design's own requirement, and considerably closer to the current architecture's already batch-oriented production shape than a per-item incremental model would be. *(Evaluation §7.)*

**AC-5 — Observable progress shall support recognition by absence, never by announcement.** A future consumer must be able to recognize completion by the absence of remaining not-yet-final content; the engine itself shall never emit an explicit "done" or "progress" signal shaped like a percentage-complete or time-remaining indicator. *(Evaluation §9, and reinforcing AC-2's own exclusion of arbitrary cadence from the opposite direction — behavioral, not timing.)*

---

## 4. Deferred

Sound but genuinely incomplete — named precisely, not guessed at.

**D-1 — Execution-phase signal granularity.** Whether the observable signal AC-3 requires should be Runtime's own existing four-phase machine as-is, a coarser two-phase view (folding `CollectingEvidence`/`EvaluatingRules` together, matching Progressive Discovery's own undifferentiated treatment), or another shape. **Intentionally unresolved.**

**D-2 — The Report-evolution resolution model.** Which architectural model resolves the tension F-3 confirms (successive generation, in-place evolution before completion, or another shape) remains **intentionally unresolved.**

---

## 5. Explicitly Deferred to Other Initiatives

Restated from the Evaluation's own Initiative Boundaries (§10), not re-decided here — carried forward as this Resolution's own record of what it does not touch.

**Initiative 2 (Reentrant Assessment Lifecycle):** whether and how a Completed Assessment can be reopened to accept new Evidence; `DataModel.md`'s Immutability principle in its post-completion sense; the Updated-marker mechanism; GOV-001's still-open question of whether `Completed` status is ever practically reached by a real persisted report.

**Initiative 3 (Domain Model Anatomy Extension):** the concrete field or type needed to represent whatever state AC-2/AC-3/AC-4 require to be observable (a Provisional/Final-shaped status, an execution-phase value, Title/Summary/Category decomposition). This Resolution adopts *that such state must exist and behave a certain way*; it adopts nothing about *how it is represented*.

**Initiative 4 (Confidence as a First-Class Concept):** unaffected. Confidence remains excluded from any not-yet-final Finding regardless of this initiative's outcome, already fixed by Workspace Evolution's own frozen design.

---

## 6. Explicit Non-Actions

- No ADR created.
- No Governance Register entry opened or modified.
- No crate, Crate Boundary Rule, or dependency edge modified.
- No API, transport, event, payload, or implementation technology chosen, named, or implied.
- No implementation begun; no Sprint scoped.
- No modification to `INITIATIVE_1_ARCHITECTURE_EVALUATION.md`, any Product Design artifact, or any Interaction Design artifact.

---

## 7. Summary Table

| # | Item | Disposition | Basis |
|---|---|---|---|
| F-1 | Progressive visibility not currently supported | **Adopted** | Evaluation §4 |
| F-2 | Growing, not-yet-final Finding set is the correct target state | **Adopted** | Evaluation §5 |
| F-3 | One-shot Report generation vs. progressive Overview is a real tension | **Adopted** | Evaluation §8 |
| AC-1 | Solved at the Engine layer; Runtime invariants untouched | **Adopted Architectural Constraint** | Evaluation §3–4 |
| AC-2 | Observation only at architecturally coherent, dispatch-order boundaries | **Adopted Architectural Constraint** | Evaluation §5 |
| AC-3 | At least one observable execution-phase signal shall exist | **Adopted Architectural Constraint** | Evaluation §6 |
| AC-4 | Findings finalize together, as a set — never individually | **Adopted Architectural Constraint** | Evaluation §7 |
| AC-5 | Recognition by absence, never by announcement or progress indicator | **Adopted Architectural Constraint** | Evaluation §9 |
| D-1 | Execution-phase signal granularity | **Deferred, intentionally unresolved** | Evaluation §6 |
| D-2 | Report-evolution resolution model | **Deferred, intentionally unresolved** | Evaluation §8 |

**No item was Rejected. No item Requires Additional Investigation.**

---

## 8. Readiness

Initiative 1 is architecturally resolved except for D-1 and D-2, both intentionally left unresolved rather than forced. Five adopted, binding architectural constraints (§3) and three confirmed facts (§2) stand as settled repository architecture. Nothing in this Resolution blocks Initiative 2, 3, or 4 from opening independently, and nothing in it depends on any of them having started — only on Initiative 3 eventually supplying the concrete representation the adopted constraints presuppose (§5).

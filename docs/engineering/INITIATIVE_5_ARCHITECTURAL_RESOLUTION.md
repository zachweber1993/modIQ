# Initiative 5 Architectural Resolution — Production Interaction Layer Definition

| Property | Value |
|---|---|
| **Document** | INITIATIVE_5_ARCHITECTURAL_RESOLUTION.md |
| **Project** | modIQ |
| **Initiative Addressed** | Engineering Alignment Program, Initiative 5 — Production Interaction Layer Definition |
| **Purpose** | Evaluate each recommendation made by `docs/engineering/INITIATIVE_5_ARCHITECTURE_EVALUATION.md` and determine which becomes repository architecture. |
| **Primary Evidence Source** | `docs/engineering/INITIATIVE_5_ARCHITECTURE_EVALUATION.md` (commit `3ae88f3`), treated as fixed, verified evidence — not reopened or re-derived. |
| **Repository Verification** | Confirmed immediately prior to drafting: `3ae88f3` remains `HEAD`, working tree clean, no repository change of any kind since the Evaluation was committed and pushed. No fact in the Evaluation is stale. |
| **Status** | **Architectural Resolution complete. No ADR created. No Governance Register entry modified. No crate boundary modified. No implementation authorized.** |

---

## 1. Method

Each of the Evaluation's recommendations is dispositioned as **Adopted**, **Deferred**, **Rejected**, or **Requires Additional Investigation**, per the following standard, applied consistently:

- **Adopted** — the Evaluation's evidence is convergent, no repository artifact contradicts it, and adopting it commits nothing this Resolution lacks standing to commit (per this task's own constraints: no crate boundary change, no ADR, no Governance Register update, no implementation).
- **Deferred** — the recommendation itself is sound but its completion genuinely depends on a fact only another initiative's own Architecture Evaluation can supply. Deferred is not a weaker form of rejection; it is adoption of everything the evidence currently supports, with the remainder named precisely rather than guessed at.
- **Rejected** — repository evidence contradicts the recommendation. (None found; see below.)
- **Requires Additional Investigation** — the evidence is genuinely inconclusive and a dedicated evidence-gathering activity, not merely another initiative's own resolution, is needed. (None found; see below.)

No question already settled by the Evaluation is revisited here except where this method requires restating it precisely enough to assign a disposition.

---

## 2. Dispositions

### Decision 1 — Classification of the Production Interaction Layer

**Recommendation:** the production interaction layer is a consumer, not a subsystem (neither an existing System Overview component nor a candidate for Architectural Activation).

**Disposition: Adopted.**

**Rationale:** the evidence is structural, not interpretive — `Architecture.md`'s own System Overview diagram draws "User" outside the six-subsystem row, and `modiq-cli`'s own, already-governed Crate Boundary Rule already answers a narrower version of this same question in the same direction. No repository artifact draws a different conclusion. Nothing about adopting this classification commits the repository to any crate, dependency, or API change — it settles what kind of thing a future consumer is, which is exactly the level this Resolution is authorized to decide.

### Decision 2 — Responsibility Split

**Recommendation:** the engine retains everything it already owns; the consumer owns presentation and action-translation and must never contain business logic, evaluate Evidence, generate Findings/Recommendations/Reports, or mutate Assessment state directly.

**Disposition: Adopted.**

**Rationale:** this is a direct, unavoidable consequence of Decision 1 combined with `GOVERNANCE.md`'s already-existing CLI rule and `Architecture.md`'s already-existing Dependency Rules language. It restates governing constraints that already bind every consumer in this repository; it does not introduce a new one. Checked against Initiative 2 (Reentrant Assessment Lifecycle, not yet evaluated): even a future reentrant Assessment would be mutated only through whatever boundary the engine itself exposes, never by the consumer directly — this disposition remains compatible with any outcome Initiative 2 could plausibly reach.

### Decision 3 — Ownership of Assessment State

**Recommendation:** Assessment state ownership is already fixed by INV-006 and INV-009; this evaluation confirms rather than decides it.

**Disposition: Adopted (confirmed, non-discretionary).**

**Rationale:** there is nothing to adopt in the ordinary sense — INV-006 and INV-009 already bind this outcome, verified directly against `assessment.rs`, with zero external mutation path existing anywhere in the workspace. This Resolution records that no Architecture Evaluation, including this one, has discretion here, so a future reader does not mistake this for an open question re-litigated at each new initiative.

### Decision 4 — Ownership of Workspace, Navigation, Interaction, and Presentation State

Split into two dispositions, since the Evaluation's own evidence supports them to different degrees of completeness:

**4a — Navigation, interaction, and presentation state: consumer-owned.**

**Disposition: Adopted.**

**Rationale:** three independent, already-established facts converge (Interaction Design's own repeated, explicit deferrals of state-persistence questions; ADR-0007's boundary against Runtime entities holding presentation-reaching behavior; `Architecture.md`'s existing Dependency Rules language). No open dependency on any other initiative exists for this specific determination.

**4b — Workspace state: consumer-owned, but derived from engine-owned execution phase (for Assessing/Reviewing) and purely consumer-side (for Intake, which has no engine analog).**

**Disposition: Adopted in principle; the derivation mechanism itself is Deferred, pending Initiative 1.**

**Rationale:** the ownership question (consumer, not engine) is settled and adopted now — nothing about it depends on Initiative 1. What is genuinely incomplete, and what the Evaluation itself declined to resolve, is *which* execution-phase signal Assessing/Reviewing derive from and how it is exposed — that is Initiative 1's own question, not a gap in this Resolution's judgment. Deferring only the derivation mechanism, while adopting the ownership determination outright, is a more precise disposition than deferring Decision 4 as a whole would be.

### Decision 5 — Mechanism Shape (How the Boundary Is Crossed)

Split into two dispositions, mirroring the Evaluation's own explicit partial-decision framing:

**5a — Baseline mechanism: request/response.**

**Disposition: Adopted.**

**Rationale:** zero counter-examples exist anywhere in the platform's history (`modiq-cli`, `apps/sandbox` both request/response), and it is the only shape consistent with `Architecture.md`'s own one-directional Dependency Rules diagram. This holds regardless of Initiative 1's outcome — even a fully streaming-capable engine can be queried request/response-style for its current state.

**5b — Supplementary notification/poll-trigger mechanism.**

**Disposition: Deferred, pending Initiative 1.**

**Rationale:** the Evaluation itself declined to resolve this, correctly — deciding it now would mean guessing whether Initiative 1 concludes the engine exposes intermediate state at all. This Resolution ratifies that restraint rather than overriding it. This is a **Deferred** disposition, not **Requires Additional Investigation**: the blocking fact is precisely identified (Initiative 1's own resolution), so no separate evidence-gathering activity is needed — only sequencing, exactly as `ENGINEERING_ALIGNMENT_PROGRAM.md` §6 already anticipated.

### Decision 6 — Relationship to `apps/sandbox`

**Recommendation:** `apps/sandbox`, as it exists today, cannot be the production interaction layer; its request/response and getter-based DTO pattern is validated precedent, not a technology endorsement.

**Disposition: Adopted.**

**Rationale:** three independently verifiable, disqualifying facts (non-membership in the root workspace, self-declared non-production status, deliberate absence of a real input mechanism), none contradicted by any repository artifact. The precedent-not-endorsement framing is preserved exactly as evaluated — adopting this disposition commits to nothing about eventual implementation technology.

---

## 3. Disposition of the Governance Observation

Addressed separately, as instructed, because it is not an architectural decision and does not take an Adopted/Deferred/Rejected disposition in the sense Decisions 1–6 do.

**The Observation is acknowledged as valid and standing:** the repository's existing governance vocabulary — Capability Identity (Sprint 12) and Architectural Activation (Sprint 8/13) — has no procedure for classifying work that formalizes an external architectural boundary, and this is not specific to Initiative 5; it would recur for any future work of the same shape.

**This Resolution does not decide how, or whether, to close that gap.** Per this task's own explicit constraint, no Governance Register item is opened here. Whether the Observation eventually becomes its own Governance Register item, gets folded into whatever item tracks this Resolution's own adopted decisions, or is addressed some other way is left entirely for the Chief Architect's own future judgment — consistent with how the Evaluation itself declined to recommend a disposition, and consistent with `GOVERNANCE_OBSERVATION_SUBSYSTEM_ACTIVATION.md`'s own precedent, which was itself left open for two Sprints before its underlying tension was addressed.

---

## 4. Consequences: Future Governance Work

This Resolution adopts architecture; it does not scope, propose, or characterize what future governance artifacts that adoption may warrant. Consistent with `GOVERNANCE.md`'s own Governance Principles ("every architectural change must be documented," "governance decisions are repository artifacts"), the decisions adopted in Section 2 may, in the ordinary course of this repository's existing governance process, eventually warrant an ADR, a Governance Register item, or both. Whether either becomes necessary, in what form, and on what timing is left entirely to the repository's own governance process at the point it is separately taken up — not anticipated, characterized, or recommended here.

**No ADR is created or proposed by this Resolution. No Governance Register item is opened or proposed by this Resolution.**

---

## 5. Explicit Non-Actions

- No ADR created.
- No Governance Register entry opened or modified.
- No crate, Crate Boundary Rule, or dependency edge modified.
- No API, payload, event, or implementation technology chosen.
- No implementation begun; no Sprint scoped.
- No modification to `INITIATIVE_5_ARCHITECTURE_EVALUATION.md`, any Product Design artifact, or any Interaction Design artifact.

---

## 6. Summary Table

| # | Recommendation | Disposition | Basis |
|---|---|---|---|
| 1 | Production interaction layer is a consumer, not a subsystem | **Adopted** | System Overview diagram structure; CLI precedent |
| 2 | Engine/consumer responsibility split (CLI-derived) | **Adopted** | Direct consequence of Decision 1; existing boundary language |
| 3 | Assessment state is exclusively engine-owned | **Adopted (confirmed, non-discretionary)** | INV-006, INV-009 |
| 4a | Navigation/interaction/presentation state: consumer-owned | **Adopted** | Interaction Design's own deferrals; ADR-0007; Dependency Rules |
| 4b | Workspace state: consumer-owned, derived | **Adopted in principle; derivation mechanism Deferred** | Depends on Initiative 1's execution-phase signal |
| 5a | Mechanism baseline: request/response | **Adopted** | Zero counter-precedent; one-directional Dependency Rules |
| 5b | Supplementary notification/poll-trigger mechanism | **Deferred** | Depends on Initiative 1's own resolution |
| 6 | `apps/sandbox` cannot be the production layer; its pattern is precedent | **Adopted** | Three independently verified disqualifying facts |
| — | Governance Observation (classification-procedure gap) | **Acknowledged, not resolved** | Left for Chief Architect judgment (Section 3) |

**No recommendation was Rejected. No recommendation Requires Additional Investigation.**

---

## 7. Readiness

With this Resolution approved, Initiative 5 has an adopted architectural boundary sufficient to scope a future Implementation Authorization once one is separately sought — that scoping is not performed here. Per `ENGINEERING_ALIGNMENT_PROGRAM.md` §8, Initiative 5's own path forward (Implementation Authorization → Sprint Plan) proceeds independently of when Initiatives 1–4 begin; nothing in this Resolution blocks Initiative 1 from being opened next, and nothing in it depends on Initiative 1 having started — only on it eventually resolving, for the two items deferred in Section 2.

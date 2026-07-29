# Initiative 2 Architecture Evaluation — Reentrant Assessment Lifecycle

| Property | Value |
|---|---|
| **Document** | INITIATIVE_2_ARCHITECTURE_EVALUATION.md |
| **Project** | modIQ |
| **Initiative Addressed** | Engineering Alignment Program, Initiative 2 — Reentrant Assessment Lifecycle (`docs/engineering/ENGINEERING_ALIGNMENT_PROGRAM.md` §5) |
| **Purpose** | Determine what current repository evidence supports regarding whether and how a Completed Assessment can accept new Evidence and produce an updated Report — not implementation, not API design, not Architectural Resolution. |
| **Origin** | Chief Architect authorization, following completion of Initiative 5 and Initiative 1 (each with an adopted Architectural Resolution: `e1cbb05`, `1fad900`). |
| **Adopted Precedent** | `docs/engineering/INITIATIVE_5_ARCHITECTURAL_RESOLUTION.md` and `docs/engineering/INITIATIVE_1_ARCHITECTURAL_RESOLUTION.md`, both treated as binding, unreopened repository architecture throughout — cited where directly relevant, never re-evaluated. |
| **Status** | **Architecture Evaluation only. No Architectural Resolution has been performed. No ADR, crate, API, transport, or implementation technology has been created, designed, or chosen. No Governance Register entry modified.** |

---

## 1. Evaluation Scope

Repository-first, evidence-based, evaluation only — following the same process established by Initiatives 5 and 1.

**Deliberately excluded**, per that same established process:

- Implementation mechanisms, APIs, transport, payloads, serialization, technology choices.
- Sprint planning, ADR creation, Governance Register updates, repository synchronization.
- Re-evaluation of anything Initiative 5 or Initiative 1 already adopted.

---

## 2. Repository Evidence

Reviewed directly this session:

- **`docs/implementation/RuntimeInvariants.md`** — INV-012 ("A Completed Assessment SHALL reject further lifecycle transitions") and its companions INV-010/INV-011, re-consulted for exact scope.
- **`crates/modiq-runtime/src/assessment/assessment.rs`** — re-confirmed directly: `add_evidence`, `add_finding`, `add_recommendation`, and every `transition()` call each check `self.status == AssessmentStatus::Completed` first and unconditionally reject, before any other validity check.
- **`docs/architecture/DataModel.md`** — the Immutability principle and the Future Evolution section, both re-read in full for exact wording and scope.
- **`docs/engineering/GOVERNANCE.md`** — GOV-001 (Assessment Report Generation Timing) in full, and the Storage Crate Boundary Rule.
- **`docs/engineering/STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md`** — read in full this session: confirms Runtime identity (`AssessmentId`, `EvidenceId`, `FindingId`, `RecommendationId`) is not a durable concept across a process boundary "by design," and that Storage's own read path never attempts to reconstruct a live `Assessment` or `modiq_report::AssessmentReport` from persisted data.
- **`crates/modiq-storage/src/storage/persisted_report.rs`** — re-confirmed: `PersistedFinding`'s cross-references are positional indices into the same persisted document, not `EvidenceId` values; no field or method anywhere reconstructs a live `Assessment`.
- **Product Design** — Workspace Evolution §1–§3 (the three-state model, "Assessing recurring on top of Reviewing," the Updated-marker mechanism) and The Assessment Experience §7 ("Multiple Assessment Cycles").
- **Interaction Design** — Assessing & Progressive Discovery §7 (cancellation of an in-progress Assessment, left explicitly undecided by Product Design — noted, not reopened, as it is a distinct question from this evaluation's own).
- **`docs/engineering/ENGINEERING_ALIGNMENT_PROGRAM.md`** — Initiative 2's own governing scope definition (§5) and its recorded dependency on Initiative 5.
- **`docs/engineering/INITIATIVE_5_ARCHITECTURAL_RESOLUTION.md`** — consulted as adopted precedent only, specifically Decision 2 (consumer must never mutate Assessment state directly) and Decision 3 (Assessment state exclusively engine-owned).
- **`docs/engineering/INITIATIVE_1_ARCHITECTURAL_RESOLUTION.md`** — consulted as adopted precedent only, specifically its explicit hand-off to Initiative 2 (§5: reentrant lifecycle, post-completion Report modification, the Updated-marker mechanism, GOV-001's still-open question) and AC-1 (progressive visibility solved without weakening Runtime invariants — noted as a boundary Initiative 2's own question is categorically different from, see §10).

---

## 3. Current Architecture

**`AssessmentStatus::Completed` is unconditionally terminal, checked first, in every mutation path.** Re-confirmed directly against `assessment.rs`: `add_evidence`, `add_finding`, `add_recommendation`, and the shared `transition()` function each begin with `if self.status == AssessmentStatus::Completed { return Err(AssessmentError::AssessmentCompleted); }`, before any other check. This is not an incidental consequence of some other rule — it is the first, explicit condition tested on every single mutation attempt.

**A second, independent barrier exists beyond the in-memory invariant: no mechanism reconstructs a live `Assessment` from anything persisted.** `STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md` establishes this directly: Runtime identity is "not a well-defined, durable concept across a process boundary today, by design" (the id-generating counter restarts at 1 every process invocation), and Storage's own read path returns Storage's own representation type — never `modiq_report::AssessmentReport`, and never a live `Assessment`. Even if INV-012 permitted further mutation, there is currently no path from a persisted report back to a mutable `Assessment` to mutate.

**`DataModel.md`'s Immutability principle states a general rule whose scope, applied here, is genuinely ambiguous.** The principle: "Completed Assessments represent historical records... If assessment behavior changes, a new Assessment should be performed rather than modifying previous Assessment results." Its own Future Evolution section illustrates this specifically with engine/rule changes over time: "If new Rules, Knowledge, or platform capabilities become available, subsequent Assessments should generate new historical records rather than altering previous results." Read narrowly, the principle protects historical reproducibility against *engine drift* — a different concern from a user submitting more material to what they experience as the same ongoing assessment. Read broadly — as its own general statement is worded — it would also seem to cover that case. Nothing in the document disambiguates which reading is intended; neither reading has ever been checked against Workspace Evolution's own supplementation requirement, because that requirement did not exist when `DataModel.md` was frozen.

**Nothing about `AssessmentReport::generate`'s own shape independently forbids being called more than once.** It is a pure function of an `Assessment`'s current state. What prevents a second, later Report from ever being generated is not the Report layer — it is that `Assessment`'s own state can never re-enter `CollectingEvidence` or `EvaluatingRules` after `Completed`, so there is never a second state for `generate` to snapshot. This mirrors Initiative 1's own finding that a barrier's true locus can differ from where its symptom appears (Initiative 1 found the barrier to progressive visibility lived in the Engine API, not Runtime; here, the barrier to a "living" Report lives in `Assessment`'s own lifecycle invariant, not in Reporting).

---

## 4. Question 1 — Terminal Completion vs. Reentrancy

**Does architecture support reopening a Completed Assessment to accept new Evidence?**

**Observation:** no. INV-012 rejects this unconditionally and is enforced first, on every mutation path, with no exception carved out anywhere in the current implementation or specification for a supplementation scenario.

**Conclusion:** current architecture treats `Completed` as genuinely, absolutely terminal — not merely defaulted-closed with an unused extension point. This is a stronger prohibition than Initiative 1 found for progressive visibility (Initiative 1's barrier was an *absence* of provision; here, INV-012 is an *explicit, tested rejection*). Whether this absolute terminality itself should change is not decided by this evaluation — it is the central architectural question Initiative 2 exists to carry forward.

---

## 5. Question 2 — Locus of the Barrier

**Where does the barrier to reentrancy actually live?**

**Observation, in-memory:** the barrier lives in `Assessment`'s own lifecycle invariant (INV-012), enforced entirely within `modiq-runtime` — the same domain Initiative 5's Decision 3 already fixed as the sole, exclusive owner of Assessment state.

**Observation, cross-process:** a second, independent barrier exists in `modiq-storage`'s own representation boundary. Even absent INV-012, nothing today reconstructs a mutable `Assessment` from a `PersistedAssessmentReport` — by explicit design, confirmed in `STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md` (§1, §5, §6), not as an oversight but as a considered consequence of Runtime identity's own non-durability across a process boundary.

**Conclusion:** these are two separately-evidenced, independent barriers, not one. A future resolution addressing only INV-012 would still leave any Assessment that had already been persisted and reloaded unable to be supplemented — reentrancy, if ever enabled, would need to account for both loci, or explicitly scope itself to only one (for instance, an Assessment still alive within its originating process). This evaluation does not resolve which; it identifies that the question has two parts where it might appear to have one.

---

## 6. Question 3 — DataModel.md's Immutability Principle

**Does the frozen conceptual specification permit, forbid, or leave ambiguous same-Assessment supplementation?**

**Observation:** genuinely ambiguous, as detailed in §3. The principle's own illustrative scope (engine/rule drift) is narrower than its own general wording (any modification of a completed Assessment's results). Product Design's Workspace Evolution assumes the narrower reading without ever stating that assumption or checking it against `DataModel.md`'s own text — because Workspace Evolution's own Authority section states Product Design artifacts "do not modify Vision.md, Principles.md, Glossary.md, ProductSpecification.md, or Architecture.md," and treats conflicts as something to flag for governance review, not resolve locally; `DataModel.md` was never checked, since Product Design's own scope boundary excludes Architecture and its own subordinate specifications from what any session reads against.

**Conclusion:** this is a genuine, unresolved tension between two frozen documents' own scope, not a defect in either. `DataModel.md`'s Immutability principle was never written with Workspace Evolution's supplementation requirement in view; Workspace Evolution was never checked against `DataModel.md`'s own text. Per this evaluation's own standing instruction, this tension is identified explicitly here rather than resolved by adopting one interpretation over the other to make the two agree.

---

## 7. Question 4 — The Updated-Marker Mechanism

**What does Workspace Evolution require, and does current architecture support it?**

**Observation:** Workspace Evolution §3 requires that a Finding "affected by the new material" be "visibly marked 'updated' for the remainder of the session, distinguishing them from Findings that were already Final and unaffected." No field, status, or comparable mechanism exists anywhere in `modiq-runtime`'s `Finding` type or anywhere else in the current codebase — confirmed already by the Engineering Reconciliation and unchanged since.

**Conclusion:** this is a field-level representation question — what value would hold "updated" status, and on which type — squarely Initiative 3's own territory (Domain Model Anatomy Extension), not this evaluation's. What this evaluation can and does confirm is the *precondition* the marker depends on: an "updated" Finding is only a meaningful concept once some mechanism exists for a Finding to be re-derived or replaced after a supplement — which itself depends on Question 1's own terminal-completion question being addressed first. The marker cannot be designed, even at the field level, ahead of that.

---

## 8. Question 5 — Report "Living Artifact," Post-Completion Sense

**Is a completed, historical Assessment Report architecturally capable of later reflecting new material?**

This is explicitly the second of the two Report-evolution questions Initiative 1 named and declined to address (Initiative 1 §8/§10: "whether a *completed, historical* Report may later be modified is Initiative 2's own territory"). This evaluation addresses it directly.

**Observation:** no. Per §3 and §5, above, a Report is generated by a pure function of `Assessment`'s current state, and `Assessment`'s state can never change again once `Completed`. A persisted, historical Report is even further removed — `modiq-storage`'s own write-once boundary ("Must never: mutate a stored report once written," `GOVERNANCE.md`) forbids it independently, for its own separately-evidenced reasons, regardless of what Runtime eventually decides.

**Conclusion:** "living artifact," in the sense Workspace Evolution and The Assessment Report use it (a Report that continues to reflect the same ongoing Assessment as it changes), is not something current architecture provides at any layer it touches — Runtime, Reporting, or Storage. This is a direct, unavoidable consequence of Question 1's own finding, not a separate architectural gap requiring its own independent evidence.

---

## 9. Question 6 — Relationship to GOV-001

**Does this initiative supply the forcing function GOV-001 was left waiting for?**

**Observation:** GOV-001's own Resolution text names its own condition for revisiting: "most plausibly a future Storage capability or consumer that reasons about a persisted report's own completion state directly." Reentrancy is exactly this — it requires reasoning about whether, and how, `Completed` status on a *persisted* report is ever meaningfully re-examined or superseded.

**Conclusion:** Initiative 2 is a plausible candidate forcing function for GOV-001, but this evaluation does not resolve GOV-001 itself — that is a governance action, explicitly out of scope here. The connection is named so a future Resolution does not have to rediscover it.

---

## 10. Initiative Boundaries

**Initiative 1 (Progressive Execution Observability, already resolved — cited as precedent, not reopened):** AC-1 ("progressive visibility... solved... without weakening any Runtime domain invariant") addressed a *different* Runtime invariant question than this initiative does. Initiative 1 found no existing invariant needed to change to support observability. Initiative 2's own central question is the opposite in kind: whether a specific invariant (INV-012) should itself evolve. These are not in tension — Initiative 1 never claimed all Runtime invariants are permanently fixed, only that none needed to change *for its own purpose*. Initiative 2 is the first initiative to put a Runtime invariant's own permanence genuinely in question.

**Initiative 3 (Domain Model Anatomy Extension):** the concrete field or type representing the Updated marker (Question 4); any field-level consequence of however Question 1 is eventually resolved.

**Initiative 4 (Confidence as a First-Class Concept):** unaffected. Nothing in this evaluation touches Confidence or its computation.

**Initiative 5 (already resolved — cited as precedent, not reopened):** Decision 2 (the consumer must never mutate Assessment state directly) and Decision 3 (Assessment state is exclusively engine-owned) both remain fixed boundary conditions for whatever this initiative eventually resolves — any reentrancy mechanism must still be performed by the engine itself, never by a consumer reaching into Assessment state directly. The request/response mechanism baseline (Decision 5a) is unaffected; no transport question is addressed here.

---

## Evaluation Boundaries

**Questions this evaluation answered:**
- Whether architecture currently supports reopening a Completed Assessment (§4) — no, and the prohibition is explicit and absolute, not merely an absence of provision.
- Where the barrier to reentrancy actually lives (§5) — two independent barriers, one in-memory (INV-012) and one cross-process (Storage's representation boundary), not one.
- Whether `DataModel.md`'s Immutability principle permits, forbids, or leaves ambiguous same-Assessment supplementation (§6) — genuinely ambiguous, a real unresolved tension between two frozen documents that were never checked against each other.
- What the Updated-marker mechanism requires and what precondition it depends on (§7) — a field-level question for Initiative 3, dependent on Question 1's own resolution.
- Whether a completed, historical Report can currently reflect new material (§8) — no, at every layer the evidence covers — Runtime, Reporting, and Storage — as a direct consequence of Question 1's own finding.
- Whether this initiative supplies GOV-001's own named forcing function (§9) — plausibly yes, without resolving GOV-001 itself.

**Questions intentionally left unresolved by this evaluation, to be carried into a future Architectural Resolution rather than decided here:** whether, and by what architectural principle, INV-012's own terminality should change; how the two independent barriers named in §5 should be reconciled (or whether reentrancy should be scoped to avoid one of them entirely); how the tension named in §6 should be settled.

**Questions intentionally left to other initiatives (§10):** the Updated marker's concrete field-level representation and any other field-level consequence (Initiative 3); Confidence, unaffected throughout (Initiative 4).

**Adopted Initiative 5 and Initiative 1 decisions relied upon without reopening:** Initiative 5's Decision 2 (consumer must never mutate Assessment state directly) and Decision 3 (Assessment state exclusively engine-owned) — both assumed as fixed boundary conditions throughout §10. Initiative 1's AC-1 — cited to distinguish, not extend: Initiative 1 found no Runtime invariant needed to change for its own purpose; that finding is not treated here as evidence that no invariant may ever change, only as the correct boundary marking why Initiative 2's own question is categorically new.

No Architectural Resolution has been performed. This evaluation's responsibility ends here, awaiting Chief Architect review.

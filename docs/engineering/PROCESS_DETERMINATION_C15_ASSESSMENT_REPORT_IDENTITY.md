# Process Determination — C15: Assessment & Report Identity

| Property | Value |
|---|---|
| **Document** | PROCESS_DETERMINATION_C15_ASSESSMENT_REPORT_IDENTITY.md |
| **Project** | modIQ |
| **Type** | Repository Process Determination — decides which repository artifact follows the Capability Definition. It does not design a data shape, choose an identity field, resolve GOV-002, resolve Initiative 3 Item 6a's gate, or authorize implementation. |
| **Origin** | `docs/engineering/CAPABILITY_DEFINITION_C15_ASSESSMENT_REPORT_IDENTITY.md` (working tree, uncommitted), §14, which names this document as the next required repository artifact. |
| **Repository baseline** | `feature/runtime-implementation`, HEAD `60ef3af` (C14 Repository Closeout), working tree clean apart from the C15 Capability Definition (untracked) |

---

## 1. Purpose

This document answers one question, and only one: does C15 proceed directly toward Implementation Authorization (C1/C3/C4's own path), does it require a dedicated Architecture Evaluation first (C2's/C12's path), does it require a distinct Governance Reconciliation step before Architecture Evaluation can meaningfully proceed (Initiative 2's own precedent), or is the Capability Definition itself insufficient to proceed from at all. It applies this repository's own reusable two-prong test, independently, against the actual repository evidence — not by assuming Architecture Evaluation is warranted merely because the Capability Definition contains open questions, and not by assuming it is unwarranted merely because the capability sounds conceptually simple.

---

## 2. Capability Under Determination

Restated, unchanged, from `CAPABILITY_DEFINITION_C15_ASSESSMENT_REPORT_IDENTITY.md` §1: modIQ must be able to meaningfully identify, distinguish, and refer to an Assessment and its resulting Report as a durable product concept. The Capability Definition deliberately leaves open what content constitutes that identity, which type(s) own it, how it relates to existing identifiers, and how it should be persisted or presented — this document does not re-derive or narrow that scope, only determines the process by which those questions get answered.

---

## 3. Current Architectural Context

Re-confirmed directly against current source and governance text this session, not carried forward from the Capability Definition's own account:

- **`AssessmentId`, `EvidenceId`, `FindingId`** (`modiq-runtime`) — real, per-construction Runtime identifiers. `STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md` (cited directly by `INITIATIVE_2_ARCHITECTURE_EVALUATION.md` §3 and re-confirmed here) establishes that this identity scheme is **non-durable by design** — its generator restarts at 1 on every process invocation, and this is not an oversight but a considered property of the current architecture.
- **The Storage Crate Boundary Rule** (`GOVERNANCE.md`) — Storage's persisted representation is populated only from `AssessmentReport`'s already-public API; `modiq-storage` never reconstructs a live `Assessment` from anything it persists. This boundary is independently confirmed, not merely asserted, by `STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md` and directly exercised by `Initiative 2`'s own Architecture Evaluation (§3, §5), which treated it as one of two independent, load-bearing architectural barriers to reentrancy — the identical boundary this capability's own open questions (§12 of the Capability Definition) touch.
- **ADR-0003 / ADR-0011** — `Assessment` is the sole Runtime aggregate root; `AssessmentReport`'s own ownership was itself the subject of a dedicated correction (ADR-0011, superseding ADR-0003's own illustrative example without disturbing its aggregate-root decision). This confirms aggregate/domain ownership between `Assessment` (Runtime) and `AssessmentReport` (Reporting) has already, once, required a dedicated ADR-level correction in this repository — it is not a settled, interchangeable distinction.
- **ADR-0007 (Runtime Entity Design Pattern)** — Runtime entities are constructed once, with fields fixed at construction; this pattern has already been extended additively five times (Sprint 22's Title/Summary, Category, optional Recommendation cardinality, Evidence provenance; Sprint 24's structured repair steps) without requiring a fresh Architecture Evaluation per field, following one general Initiative 3 Architecture Evaluation that authorized the *pattern* of extension, not each individual field.
- **GOV-002** (Resolved) — re-read in full this session, directly from `GOVERNANCE.md`. Its Resolution states precisely: *"`AssessmentSubject`/`AssessmentContext`'s minimal content... were both evaluated and require no action."* This finding was reached while evaluating whether `RuntimeInvariants.md`'s fourteen invariants conform to implementation — it is scoped to invariant-conformance, not to whether `AssessmentSubject`'s content is sufficient for any product-level identity purpose, a question that did not exist in this repository at the time GOV-002 was resolved.
- **Initiative 3, Item 6a** (Adopted-but-gated) — its own Architecture Evaluation states directly: Report Identity "was found to connect directly to GOV-002's own prior 'require no action' disposition on `AssessmentSubject`'s zero-field state, introducing a concrete requirement for that content which was absent when GOV-002 was originally evaluated — named as a candidate for reconsideration, not reopened here." Its Architectural Resolution adopts the field "contingent" on `AssessmentSubject` "carrying sufficient content" first, without ever deciding what that content is.
- **Initiative 2** (Requires Governance Reconciliation) — confirmed, re-read directly, to concern exclusively whether one already-completed `Assessment` may later accept supplementary evidence so its same Report continues to evolve. Nothing in its own Central Architectural Question, evidence, or disposition addresses identifying separate, independent Assessments — the shape C14's "Assess another mod" already produces and the shape C15 itself concerns.

---

## 4. Current Implementation Context

Re-confirmed directly this session:

- `AssessmentSubject`/`AssessmentContext`: `pub struct AssessmentSubject;` / `pub struct AssessmentContext;` — zero fields, confirmed directly against `crates/modiq-runtime/src/assessment/{subject,context}.rs`.
- `ReportKey`: `"{nanosecond-timestamp}-{sequence}"`, confirmed directly against `report_key.rs` — durable, opaque, carries no subject information.
- `PersistedAssessmentReport`: `status`, `evidence`, `findings`, `recommendations` only, confirmed directly against `persisted_report.rs` — no subject field.
- `apps/console/src-tauri/src/assessment.rs`: no `modiq_storage` import; `submit_assessment` never calls `ReportStore::store`. The console's own `subjectLabel` (`apps/console/src/regions/Workspace.tsx`) is the literal file-dialog path string, discarded by C14's own `resetToIntake`.
- `modiq-cli`'s `assess`/`retrieve`/`history` commands are the only current callers that exercise `ReportStore`'s full read/write surface.

---

## 5. Process Determination Criteria

This document applies the same reusable two-prong test C1, C3, C4, C2, and C12 each applied, per those documents' own stated instruction that the test is reusable, not reinvented each time:

1. **Prong 1** — Is there a genuinely open architectural design question, or an unsolved architectural principle, or does existing architecture (or an already-Adopted disposition) already settle the matter?
2. **Prong 2** — Are materially different architectural approaches actually being weighed, with real, differing consequences?

A third, repository-specific test is applied where relevant, per `INITIATIVE_2_ARCHITECTURAL_RESOLUTION.md`'s own introduced vocabulary: whether any open question rises to **Requires Governance Reconciliation** — reserved specifically for cases where two frozen, currently-binding authorities make claims that cannot both be true, and no existing mechanism arbitrates between them — as distinct from an ordinary Architecture Evaluation revisiting a prior finding in light of new evidence.

---

## 6. Prong 1 Evaluation

Each candidate question named in the Capability Definition (§12) and in this mission's own framing is evaluated individually, not accepted or rejected as a block.

**"What exact fields should `AssessmentSubject` carry?"** — **Not, by itself, an architectural question.** Field *selection*, in isolation, is the same category of decision Initiative 3's own single Architecture Evaluation already authorized generically for five other fields (Title/Summary, Category, optional cardinality, Evidence provenance, repair steps), each added at the Implementation Authorization level without a dedicated per-field Architecture Evaluation. If this were the only open question, Prong 1 would fail.

**"Is subject identity an Assessment property, a Report property, or both?"** — **A genuine architectural question.** This is not field selection; it is a domain-ownership question between two distinct aggregates governed by distinct ADRs — `Assessment` (Runtime, sole aggregate root, ADR-0003) and `AssessmentReport` (Reporting, whose own ownership already required a dedicated corrective ADR, ADR-0011). This repository has already demonstrated, once, that getting this kind of ownership question wrong requires an ADR-level correction to fix — it is not a decision safely left to Implementation Planning.

**"How should identity relate to `AssessmentId` (Runtime, non-durable by design) and `ReportKey` (Storage, durable but opaque) — extend one, bridge them, or introduce a third concept?"** — **A genuine architectural question.** `AssessmentId`'s non-durability is not an incidental gap; `STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md` documents it as a deliberate design property, and `Initiative 2`'s own Architecture Evaluation treated this exact fact as one of two independent, load-bearing barriers to a different capability (reentrancy). A decision that makes some form of identity durable across the process boundary directly engages the same design note and the same Storage Crate Boundary Rule — this is precisely the shape of question this repository's own precedent (Initiative 2) already treated as requiring dedicated architectural evaluation, not ordinary implementation judgment.

**"What must reconcile with GOV-002 and Initiative 3 Item 6a's gate?"** — **A genuine question, but not a conflict** — disposed separately in §8, below, because its correct classification (Architecture Evaluation vs. Governance Reconciliation) is exactly this document's own central task, not merely a sub-point of Prong 1.

**Conclusion: Prong 1 is satisfied.** Two of the four candidate questions (domain ownership; cross-process durability against an already-documented non-durability design property and Storage boundary) are genuinely open architectural questions, independently traceable to real, already-existing architectural facts and ADR-level precedent in this repository — not manufactured, and not merely "this touches multiple crates" or "this is important."

---

## 7. Prong 2 Evaluation

Real, materially different architectural approaches exist, each with different consequences for existing boundaries — named here without choosing, designing, or ranking among them, per this stage's own explicit instruction:

1. **Identity lives entirely upstream of Storage** — `AssessmentSubject`/`AssessmentReport` gain real content in `modiq-runtime`/`modiq-report`; `ReportKey` and `modiq-storage`'s schema remain conceptually unrelated to product-facing identity, exactly as they are today.
2. **Identity lives at the persistence boundary** — `ReportKey`/`PersistedAssessmentReport` gain a human-meaningful field directly, without necessarily requiring `AssessmentSubject` itself to change first, treating identity as a Storage-time concern rather than a Runtime-time one.
3. **Identity flows from Runtime into Storage via the already-proven Mirror pattern** — `AssessmentSubject` gains real content (satisfying Initiative 3 Item 6a's own gate), and the already-three-times-demonstrated Storage Mirror pattern (Sprint 13, 22, 24) carries it into `PersistedAssessmentReport`, exactly as every other Runtime-originated field already does.
4. **A wholly new, independent identity concept** — not a field on either existing type, but its own value object or relationship, distinct from extending `AssessmentSubject` or `ReportKey` at all.

These are not manufactured to satisfy the test: (1) vs. (2) is exactly the "which domain owns this" question ADR-0011 already had to adjudicate once for `AssessmentReport`; (1)/(3) vs. (2) has direct, different consequences for whether the Storage Crate Boundary Rule's "populated only from `AssessmentReport`'s already-public API" constraint is satisfied by construction or requires re-examination; and (4) has different consequences for whether this becomes a new architectural participant at all, the exact question `INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md`'s own Capability Identity procedure exists to classify.

**Conclusion: Prong 2 is satisfied.** These are not interchangeable implementation variations of one design — they carry genuinely different consequences for which crate owns what, and no repository document weighs them against each other today.

---

## 8. Governance Evaluation — GOV-002 and Initiative 3 Item 6a

**They do not create a conflict requiring Governance Reconciliation.** This is determined directly, not assumed, by comparing GOV-002's own text against `INITIATIVE_2_ARCHITECTURAL_RESOLUTION.md`'s own definition of what that disposition requires.

`INITIATIVE_2_ARCHITECTURAL_RESOLUTION.md` §1 defines **Requires Governance Reconciliation** precisely: reserved for cases "where the evidence is *not* incomplete... but two frozen architectural authorities make claims that cannot both be true, and nothing in the repository has ever been asked to arbitrate between them." Initiative 2's own Question 1 satisfied this bar because `DataModel.md` and `THE_ASSESSMENT_REPORT.md` are both currently live, both make a direct claim about the same subject (whether a completed Assessment/Report can later change), and those claims are mutually exclusive on their face.

GOV-002 and Initiative 3 Item 6a do not share that shape. GOV-002's Resolution, re-read directly (§3, above), evaluated `AssessmentSubject`'s minimal content **strictly against Runtime Invariant conformance** — it found the zero-field state does not violate any of the fourteen invariants, and "require[d] no action" on that specific, narrower question. It does not assert, anywhere in its own text, that `AssessmentSubject` must *remain* empty, that no future capability may ever require it to carry content, or anything else that would place it in live, mutually exclusive tension with Initiative 3's later finding. Initiative 3's own Architecture Evaluation already recognized this precisely, in its own words: the new requirement "was absent when GOV-002 was originally evaluated" — naming GOV-002's finding as resting on a premise that has since changed, not as a standing claim Initiative 3 now contradicts.

This is the ordinary situation an Architecture Evaluation exists to handle: a prior Resolved finding, reached under a narrower question and a different evidentiary state, is revisited because new evidence (C15's own forcing function) bears on a question the original evaluation never asked. It does not require a separate reconciliation track first — the Architecture Evaluation this document recommends (§10) already carries "should GOV-002's finding on `AssessmentSubject`'s content be revisited, and on what basis" as one of its own required questions, exactly as Initiative 3's own Evaluation already modeled by naming, without escalating, the identical fact.

**Conclusion: no Governance Reconciliation stage is warranted.** Process Option C does not apply.

---

## 9. Architectural Boundary Evaluation

Directly evidenced, not assumed, per this mission's own "Expected Area of Attention":

- **Domain ownership** — potentially affected (§6, §7): whether Runtime's `Assessment` aggregate, `modiq-report`'s `AssessmentReport`, or both, own subject identity is a live question with ADR-level precedent (ADR-0011) for getting it wrong.
- **Persistence semantics** — potentially affected: whether `modiq-storage`'s existing "populate only from `AssessmentReport`'s already-public API" boundary is satisfied by construction (if identity originates in Runtime/Reporting) or requires re-examination (if identity is proposed to originate at the Storage boundary itself, per Prong 2 approach 2).
- **Cross-process durability** — directly affected: `AssessmentId`'s documented non-durability is a considered architectural property, not an oversight; any decision to make some form of identity durable engages this directly.
- **Application/Runtime/Storage responsibility division** — potentially affected, contingent on which of the Prong 2 approaches is eventually favored.
- **Report contract semantics** — potentially affected: `AssessmentReport`'s own public shape may or may not need to change depending on where identity is decided to live.
- **Future history/comparison semantics** — explicitly **not** decided or scoped here (Capability Definition §7, §10) and not part of this Boundary Evaluation's own conclusion; named only as a downstream consumer whose eventual needs do not, themselves, force any particular resolution now.

**Conclusion: yes, at least two of these axes are genuinely, evidentially implicated** — this is not the case of a capability that merely "touches multiple crates" without crossing any actual boundary; the boundaries in question are named, precedented, and real.

---

## 10. Determination

**Process Option B — Architecture Evaluation is required.**

Both prongs of the two-prong test are independently satisfied (§6, §7), on grounds traced to real, already-existing architectural facts and precedent (ADR-0011's own aggregate-ownership correction; `STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md`'s documented non-durability property; the Storage Crate Boundary Rule; Initiative 2's own treatment of the identical durability question as one of two independent barriers to a different capability) — not manufactured, and not asserted merely because identity is conceptually important or because future capabilities will depend on it.

**Process Option C (Governance Reconciliation) does not apply** (§8): GOV-002 and Initiative 3 Item 6a do not constitute two live, mutually exclusive authorities of the shape `INITIATIVE_2_ARCHITECTURAL_RESOLUTION.md` itself defines that disposition around. The GOV-002 question is folded into the required Architecture Evaluation as one of its own questions, not escalated to a separate track.

**Process Option D (Capability Definition requires revision) does not apply.** `CAPABILITY_DEFINITION_C15_ASSESSMENT_REPORT_IDENTITY.md` is independently found, on direct re-read during this determination, to be correctly and sufficiently scoped: it states the capability at the right altitude, correctly and precisely catalogues existing mechanisms (§4, above, independently re-confirms every claim it makes), and does not prematurely decide any of the questions this determination finds genuinely open. No revision is required before Architecture Evaluation proceeds.

**Process Option A (proceed directly to Implementation Authorization) does not apply**, for the reasons in §6/§7/§9.

---

## 11. Required Next Stage

**An Architecture Evaluation for C15**, scoped specifically to:

1. Whether, and on what basis, GOV-002's prior finding on `AssessmentSubject`'s content should be revisited in light of the concrete requirement C15 now evidences (not whether GOV-002 was wrong at the time — it was not, on its own narrower question).
2. Whether Assessment/Report subject identity belongs to `Assessment` (Runtime), `AssessmentReport` (Reporting), or both, and why — resolving the domain-ownership question named in §6/§7.
3. Whether, and how, identity should become durable across the process boundary given `AssessmentId`'s own documented non-durability, and what consequence this has (if any) for the Storage Crate Boundary Rule.
4. Which of the architectural approaches named in §7 (or another not yet named) the evidence actually supports — without, at this stage, selecting specific field names, types, or a persistence mechanism.
5. Whether this satisfies, or requires further revisiting, Initiative 3 Item 6a's own gating condition.

This scope explicitly does **not** extend to: choosing `AssessmentSubject`'s exact fields; redesigning `ReportKey`; designing console persistence, history, trends, or C5; any DTO/transport/UI decision; or any of the other items `CAPABILITY_DEFINITION_C15...md` §7 already excludes. All of those remain reserved for whatever Architectural Resolution and Implementation Authorization eventually follow.

---

## 12. Explicit Non-Decisions

This document does not:

- Choose, design, or recommend which architectural approach (§7) is correct.
- Decide what content `AssessmentSubject`, `AssessmentReport`, or any other type should carry.
- Decide whether `ReportKey` changes.
- Reopen, resolve, or amend GOV-002 — it remains Resolved, exactly as it was, on the narrower question it actually answered.
- Reopen, resolve, or amend Initiative 3 Item 6a's own Adopted-but-gated disposition — it remains gated, exactly as it was.
- Reopen, resolve, or advance Initiative 2 in any respect — confirmed again directly (§3): Initiative 2 concerns exclusively same-Assessment supplementation, not identifying independent Assessments, and nothing in this determination touches, narrows, or depends on its own unresolved Central Architectural Question.
- Authorize, scope, or design console persistence, history, trends, or C5 — all remain named only as downstream consumers, per the Capability Definition's own boundary.
- Create an ADR, open a Governance Register item, or perform any part of the Architecture Evaluation it recommends.
- Authorize any implementation of any kind.

---

## 13. Evidence / References

`docs/engineering/CAPABILITY_DEFINITION_C15_ASSESSMENT_REPORT_IDENTITY.md` · `docs/engineering/GOVERNANCE.md` (GOV-002 full entry, re-read directly this session) · `docs/engineering/INITIATIVE_3_ARCHITECTURE_EVALUATION.md` / `INITIATIVE_3_ARCHITECTURAL_RESOLUTION.md` (Item 6a) · `docs/engineering/INITIATIVE_2_ARCHITECTURE_EVALUATION.md` / `INITIATIVE_2_ARCHITECTURAL_RESOLUTION.md` (Requires Governance Reconciliation's own definition; the two-barrier durability analysis) · `docs/engineering/STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md` · `docs/adrs/0003-assessment-aggregate-root.md` / `0011-assessmentreport-ownership-correction.md` · `crates/modiq-runtime/src/assessment/{subject,context}.rs` · `crates/modiq-storage/src/storage/{report_key,persisted_report}.rs` · `apps/console/src-tauri/src/assessment.rs` · `apps/console/src/regions/Workspace.tsx` · `docs/engineering/IMPLEMENTATION_REPORT_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md` (the C14 forcing-function evidence).

---

## Status

**Draft. Awaiting review.** No Architecture Evaluation has been performed by this document. No ADR created. No Governance Register item opened or modified. GOV-002, Initiative 2, and Initiative 3 Item 6a remain exactly as they were before this document. The next lifecycle step is an Architecture Evaluation for C15, scoped exactly as §11 states — not produced by this document, and not to begin without separate, explicit authorization.

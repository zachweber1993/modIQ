# Initiative 2 Architectural Resolution — Reentrant Assessment Lifecycle

| Property | Value |
|---|---|
| **Document** | INITIATIVE_2_ARCHITECTURAL_RESOLUTION.md |
| **Project** | modIQ |
| **Initiative Addressed** | Engineering Alignment Program, Initiative 2 — Reentrant Assessment Lifecycle |
| **Purpose** | Evaluate each question raised by `docs/engineering/INITIATIVE_2_ARCHITECTURE_EVALUATION.md` and make a genuine architectural decision wherever the evidence supports one — and explicitly name, rather than resolve by reinterpretation, any question it does not. |
| **Primary Evidence Source** | `docs/engineering/INITIATIVE_2_ARCHITECTURE_EVALUATION.md` (working tree; not yet committed), treated as fixed evidence. `docs/product-design/THE_ASSESSMENT_REPORT.md` §2 and `docs/engineering/GOVERNANCE.md`'s Documentation Authority section, both consulted freshly during this Resolution's own review. |
| **Repository Verification** | `HEAD` is `1fad900`; the Initiative 2 Evaluation remains an uncommitted working-tree file. No other repository change since. |
| **Status** | **Architectural Resolution — draft, pending independent review. No ADR created. No Governance Register entry modified. No implementation, API, transport, or persistence mechanism designed. Nothing synchronized, committed, or pushed.** |

---

## 1. Method

Same disposition standard as `INITIATIVE_1_ARCHITECTURAL_RESOLUTION.md`, extended here by a second addition to the vocabulary — following the same pattern by which Initiative 1's own Resolution first added "Adopted Architectural Constraint" to what Initiative 5's Resolution established:

- **Adopted**, **Adopted Architectural Constraint**, **Deferred**, **Rejected** — unchanged from prior use.
- **Requires Additional Investigation** — unchanged in meaning: the evidence itself is incomplete, and a dedicated evidence-*gathering* activity (this repository's own precedent: INV-001, INV-002) is needed before the question can be properly framed.
- **Requires Governance Reconciliation** — new, introduced by this Resolution. Used where the evidence is *not* incomplete — every relevant document has been identified, read, and is clearly understood — but two frozen architectural authorities make claims that cannot both be true, and nothing in the repository has ever been asked to arbitrate between them. The next step this disposition names is a decision by whoever holds authority over both documents, not further reading. This is architecturally distinct from "Requires Additional Investigation" in kind, not merely in severity: one names a fact-finding gap, the other names an authority gap.

This draft was produced once, stress-tested against its own reasoning at explicit request, and is presented here after that stress-test found a passage — `THE_ASSESSMENT_REPORT.md` §2 — that the first draft's central conclusion did not survive. The first draft's reasoning is not reproduced; this is the revised version.

---

## 2. The Central Architectural Question — Named, Not Resolved

**Whether supplementation is achieved by evolving `INV-012` to permit reentrancy, or by a new-Assessment mechanism that preserves it, is not decided by this Resolution.** Repository evidence does not support either direction decisively; it supports a real, unreconciled conflict between two frozen documents, which this Resolution identifies precisely rather than adjudicates.

**The conflict, stated exactly:**

- `DataModel.md`'s Immutability principle: "Completed Assessments represent historical records... a new Assessment should be performed rather than modifying previous Assessment results."
- `THE_ASSESSMENT_REPORT.md` §2 (Product Design, frozen): "Submitting more material doesn't produce a second Report or a new version of one; it continues updating the same Report... No new mechanism is needed for long-absence returns... regardless of whether the gap was five minutes or five months."

These describe incompatible architectures. `DataModel.md` names a new-Assessment mechanism as its own general answer to post-completion change. `THE_ASSESSMENT_REPORT.md` explicitly rules out a second Report or version, for any elapsed time, using an object name (`Assessment Report`) that is also `DataModel.md`'s own Runtime entity name. `DataModel.md` predates Product Design entirely, and neither document cross-references the other; Product Design's own Authority section, which lists the documents it must defer to on conflict, does not include `DataModel.md`.

**What this Resolution will not do:** pick the reading that happens to agree with whichever direction seems architecturally smaller, and then construct supporting argument afterward. An earlier draft of this Resolution did exactly that — reasoning from Storage's boundary to a preference for the new-Assessment direction, then reading `DataModel.md`'s language as decisive and Product Design's language as merely experiential, without first checking whether Product Design's own text supported that dismissal. It did not survive being checked.

**What this Resolution does establish, decisively:** Product Design's own stated justification for its position — "no new mechanism is needed for long-absence returns" — is not satisfiable under the current Runtime and Storage architecture, independent of which direction is eventually chosen. `modiq-storage`'s own boundary (`STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md`) makes reconstructing a live `Assessment`, or any single continuous `AssessmentReport`, from persisted data impossible today, by design — not because `INV-012` forbids it, but because Runtime identity is not durable across a process boundary at all, and Storage's read path never attempts reconstruction into the original live type. This is true whether reentrancy is built (which would still need a rehydration mechanism Storage doesn't have) or a new-Assessment-sequence model is built (which would still need a relatedness mechanism that doesn't exist either). **Some new architectural surface is unavoidable for the cross-session case Product Design treats as ordinary, regardless of which side of the central question is eventually adopted.** This is the one place the evidence is unambiguous, and it is adopted below.

**On precedent — checked, not assumed.** GOV-015 reconciled a comparable-looking conflict between ADR-0003 and ADR-0007, and was considered as a candidate precedent for how this conflict might be reconciled. It is not structurally equivalent, and is not relied upon here. GOV-015's own resolution found the ADR-0003/ADR-0007 conflict was "an editorial imprecision... not an architectural inconsistency — the actual architecture has never been internally inconsistent": the real, already-built crate architecture unambiguously matched ADR-0007's account, giving GOV-015 a tiebreaker external to both documents. No equivalent tiebreaker exists here — neither "new Assessment" nor "reentrant same-Assessment" is implemented anywhere in the current codebase, so there is nothing already-built to check either frozen document against. GOV-015 reconciled a document's description against an architecture that had already settled elsewhere. This conflict has no settled architecture on either side to reconcile against.

---

## 3. Disposition by Evaluation Question

**Question 1 (Terminal completion vs. reentrancy) — Requires Governance Reconciliation.** All relevant evidence (Runtime invariants and implementation, Storage implementation and its design note, `DataModel.md`, the full Product Design and Interaction Design corpus, the Engineering Alignment Program, and both prior Architectural Resolutions) has been identified and reviewed. The evidence is not incomplete; two authorities within it disagree, and reconciling that disagreement requires a decision this Resolution does not have standing to make unilaterally. **Not decided by this document.**

**Question 2 (Locus of the barrier) — Adopted, unaffected by §2.** Both barriers (`INV-012`, Storage's representation boundary) are real and independent regardless of how Question 1 is eventually reconciled.

**Question 3 (`DataModel.md`'s Immutability principle) — Reframed, not resolved.** The Evaluation found this principle's own scope ambiguous in isolation. This Resolution finds something sharper: it isn't ambiguous in isolation so much as in direct tension with a separate, explicit Product Design claim once both are read together. The earlier draft's attempt to resolve the ambiguity by adopting one reading is withdrawn — see §2. Folded into Question 1's disposition; not separately dispositioned.

**Question 4 (Updated-marker mechanism) — Adopted precondition only; grounding withdrawn.** The Evaluation's own finding stands: an Updated marker depends on Question 1 being resolved first. This Resolution's earlier draft additionally grounded "Updated" as a cross-Assessment content diff — that grounding assumed the new-Assessment direction and is withdrawn along with it. What the marker conceptually represents (a diff between two objects vs. a mutation flag on one) is itself downstream of Question 1, not decidable ahead of it.

**Question 5 (Report "living artifact," post-completion) — Adopted only as a statement of current architecture; directional claim withdrawn.** Current architecture does not support a completed Report reflecting new material — this remains true and unaffected. How it should come to support it is Question 1's own question, not resolved here.

**Question 6 (Relationship to GOV-001) — Adopted, unaffected.** True regardless of how Question 1 resolves.

**New finding, not in the original Evaluation — Adopted.** Product Design's own stated justification ("no new mechanism is needed") is not satisfiable under current Storage architecture, independent of which direction Question 1 eventually takes. Some new mechanism for the cross-session case is unavoidable either way.

---

## 4. What Is Not Adopted

The prior draft's "Assessment Sequence" architecture — preserving `INV-012`, resolving supplementation through a new, related Assessment, with continuity presented at the workspace layer — is **not adopted** by this Resolution. It remains a live, plausible candidate, but repository evidence does not decide it over reentrancy once `THE_ASSESSMENT_REPORT.md` §2 is read in full. Presenting it as decided would have been the "compromise... designed to preserve every existing document" this Resolution was explicitly told to avoid — just inverted, since it would have quietly overridden Product Design's own explicit text rather than Architecture's.

---

## Governance Observation — Documentation Authority Does Not Cover the Product & Interaction Design Track

| Property | Value |
|---|---|
| **Arising from** | Question 1's own conflict (§2–3) — investigating why no existing repository mechanism resolves it |
| **Nature** | An **Observation**, per this project's own Decision Framework — it names and evidences a governance-vocabulary gap; it does not propose, design, or adopt a fix, mirroring `INITIATIVE_5_ARCHITECTURE_EVALUATION.md`'s own Governance Observation precedent. |
| **Status** | **Observation only. No Governance Register item opened. No ADR created. No amendment to `GOVERNANCE.md` proposed or performed.** |

### Observation

Question 1's conflict is not merely two documents disagreeing — it is a conflict this repository's own existing conflict-resolution mechanism cannot process, because one side of it falls entirely outside that mechanism's defined scope. `GOVERNANCE.md`'s Documentation Authority section names ten tiers, in order: Vision, Product Specification, Architecture, Data Model, Principles, ADRs, Governance, Engineering Release Documents, Engineering Log, Source Code. Product Design and Interaction Design appear in none of them. This is not newly discovered as a fact — `docs/governance/PROJECT_STATUS.md`'s own Product & Interaction Design Track section already states the track "carries no Documentation Authority ranking." What is new is that this fact's consequence has now been tested against a real conflict, for the first time: `THE_ASSESSMENT_REPORT.md` (Product Design, unranked) contradicts `DataModel.md` (rank 4), and the ranking mechanism has no rule to apply to that pairing — not because it favors `DataModel.md`, but because the Documentation Authority hierarchy does not define any precedence relationship involving a Product Design document.

### Evidence

`GOVERNANCE.md`'s Documentation Authority section, in full: ten ranked tiers, followed by "Implementation shall never be considered authoritative when it conflicts with higher-level documentation. Instead, the discrepancy shall be recorded as a Governance Item and resolved through the governance process." This fallback clause addresses a conflict between a ranked document and source code; it does not address a conflict between a ranked document and a document outside the hierarchy. `docs/product-design/README.md`'s own Authority section separately lists the five documents Product Design must defer to on conflict (`Vision.md`, `Principles.md`, `Glossary.md`, `ProductSpecification.md`, `Architecture.md`) — `DataModel.md` is not among them, so even Product Design's own stated deference chain does not reach the document it is now found to conflict with.

### Not Resolved Here

This document does not propose extending the Documentation Authority hierarchy to include the Product & Interaction Design track, does not propose a rule for how such an extension would rank the track relative to existing tiers, and does not propose any other mechanism for resolving conflicts of this shape in general. It records that the capability to resolve them does not currently exist, and that Question 1's own reconciliation may be the first case requiring it.

### Explicit Non-Scope

- No amendment to `GOVERNANCE.md` is proposed or performed.
- No ranking or relative priority for Product Design or Interaction Design is proposed.
- No Governance Register item is opened by this observation.
- No recommendation is made as to whether this gap should be closed generally or resolved only for this specific conflict.

---

## 5. Explicitly Deferred to Other Initiatives

**Initiative 3 (Domain Model Anatomy Extension):** the Updated marker's concrete representation — now doubly deferred, first behind Question 1's own governance reconciliation, then behind Initiative 3's field-level work.

**Initiative 4 (Confidence as a First-Class Concept):** unaffected.

**Initiative 1 (already resolved):** unaffected either way Question 1 eventually resolves — progressive visibility within a single Assessment's execution applies identically regardless of how many related Assessments a supplement sequence eventually involves.

---

## 6. Explicit Non-Actions

- No ADR created.
- No Governance Register entry opened or modified.
- No crate, Crate Boundary Rule, or dependency edge modified.
- No implementation, API, transport, or persistence mechanism designed.
- No amendment to `GOVERNANCE.md`'s Documentation Authority section.
- No modification to `INITIATIVE_2_ARCHITECTURE_EVALUATION.md`, `DataModel.md`, `THE_ASSESSMENT_REPORT.md`, or any other Product Design or Interaction Design artifact.

---

## 7. Summary Table

| # | Item | Disposition |
|---|---|---|
| Q1 | Terminal completion vs. reentrancy | **Requires Governance Reconciliation** |
| Q2 | Both barriers real and independent | **Adopted** |
| Q3 | `DataModel.md` Immutability ambiguity | **Folded into Q1 — not separately dispositioned** |
| Q4 | Updated-marker precondition | **Adopted (precondition only)** |
| Q5 | Completed Report cannot currently reflect new material | **Adopted (as current-state fact only)** |
| Q6 | GOV-001 unaffected | **Adopted** |
| New | "No new mechanism needed" is not satisfiable under current architecture, either direction | **Adopted** |
| — | "Assessment Sequence" architecture | **Not Adopted — withdrawn, remains a live candidate only** |
| — | Documentation Authority gap for the Product & Interaction Design track | **Acknowledged, not resolved** |

---

## 8. Readiness

Initiative 2 is **not** architecturally resolved. This Resolution adopts what the evidence actually settles — that both barriers are real, that GOV-001 is unaffected, and that Product Design's own stated assumption about mechanism-free continuity is not satisfiable under current architecture — and explicitly declines to adopt a lifecycle direction, because doing so would require silently privileging one frozen document over another. Question 1 is dispositioned **Requires Governance Reconciliation**: the evidence-gathering stage is complete and does not need to be repeated; what is needed is a decision, by whoever holds authority over both `DataModel.md` and `THE_ASSESSMENT_REPORT.md`, about which governs — or how both are satisfied. That decision is further complicated, not merely delayed, by the Governance Observation above: the mechanism that would ordinarily make this kind of decision (Documentation Authority) has no defined jurisdiction over one of the two documents in conflict. Whoever performs Question 1's reconciliation may need to resolve, or explicitly work around, that gap as part of the same effort — a scoping question for that future work, not decided here.

# Proposal: GOV-015 — ADR-0003 AssessmentReport Ownership Reconciliation

| Property | Value |
|----------|-------|
| **Document** | PROPOSAL_GOV-015.md |
| **Stage** | Governance Initiation only |
| **Next available Governance Item identifier** | **GOV-015** — confirmed directly against `docs/engineering/GOVERNANCE.md`; the Register's highest existing item is GOV-014 |
| **Prepared by** | Chief Architect session, Sprint 16 |
| **Origin** | A finding surfaced during Sprint 15's GOV-002 Architecture Evaluation (`docs/engineering/GOV002_ARCHITECTURE_EVALUATION.md`, Finding 3), outside GOV-002's own scope, recommended there as a candidate for a new Governance Register item rather than absorbed into GOV-002's resolution |
| **Status** | **Proposal only. This is Governance Initiation — no Architecture Evaluation, Architectural Resolution, or Implementation Authorization has been performed. Nothing in this document decides anything; it defines the question the repository will evaluate, once the Chief Architect approves its entry into the Register.** |

---

## Title

**Role of ADR-0003 in Describing `AssessmentReport` Ownership**

(Named to parallel the Register's own existing convention for entity/subsystem-ownership questions — compare GOV-003's "Role of `modiq-common`.")

---

## Background

ADR-0003 ("Assessment as the Aggregate Root," Accepted, "authoritative for Documentation Release 1.0") designates `Assessment` as the Runtime domain's aggregate root and lists examples of Assessment-owned entities:

> "Examples of Assessment-owned entities include: Assessment Context, Assessment Subject, Evidence, Findings, Recommendations, **Assessment Report**." (`docs/adrs/0003-assessment-aggregate-root.md`, Decision section)

This was written at Sprint 1/2, before the platform's multi-crate architecture had matured. The Reporting subsystem was later given its own explicit crate boundary, recorded in `docs/engineering/GOVERNANCE.md`'s Reporting Crate Boundary Rule:

> "Owns: `AssessmentReport`, report formatting, report summarization, traceability output. Must never: evaluate rules, mutate runtime state. Reports are reflections of Assessment state. They never create new state."

`Architecture.md`'s own "Platform Boundaries" section independently states the same separation: "The architecture intentionally separates: Assessment execution ... Reporting ... Persistence." Reporting is named as a boundary distinct from Assessment execution, not a sub-part of it.

The platform's actual, current implementation follows the Crate Boundary Rule, not ADR-0003's original list: `AssessmentReport::generate(&assessment)` is a function in the separate `modiq-report` crate, reading `Assessment`'s already-public getters (`crates/modiq-engine/src/engine/assessment_service.rs` confirms this call site directly). `Assessment` itself (`crates/modiq-runtime/src/assessment/assessment.rs`) holds no field, method, or dependency referencing `AssessmentReport` anywhere.

This gap has never been examined or flagged in this repository's history prior to Sprint 15 — it is, in fact, still repeated without qualification in `docs/engineering/PROJECT_HANDOFF_v1.1.md`'s own current text, which groups `AssessmentReport` alongside `Assessment`, `Evidence`, `Finding`, and `Recommendation` under "Runtime Domain (`DataModel.md`)... `Assessment` (the sole aggregate root, ADR-0003)."

---

## Architectural Question

**Does ADR-0003's inclusion of `AssessmentReport` as an example of an Assessment-owned Runtime entity still accurately describe the platform's architecture, and if not, what is the correct way to reconcile ADR-0003 with the Reporting Crate Boundary Rule it now conflicts with?**

Specifically:

1. Is `AssessmentReport` correctly understood as owned by `Assessment` (ADR-0003's framing), or as owned by `modiq-report` and merely *derived from* `Assessment`'s already-public state (`GOVERNANCE.md`'s framing)?
2. If ADR-0003's framing is no longer accurate, should it be corrected by:
   - amending ADR-0003 in place, with the amendment recorded explicitly (the precedent already established for `DataModel.md`'s Sprint 5 amendment and `EvidenceCollection.md`'s four amendments), or
   - a new ADR that supersedes ADR-0003's specific claim without disturbing its still-valid core decision (`Assessment` as the aggregate root, which this proposal does not question)?
3. Does correcting ADR-0003 require touching any other document that currently repeats its framing (`PROJECT_HANDOFF_v1.1.md` is the one instance confirmed so far)?

This proposal does not answer any of these three questions. That is Architecture Evaluation's task, once this item is entered into the Register.

---

## Current Evidence

All evidence below was already established during Sprint 15's GOV-002 Architecture Evaluation; no new investigation was performed to prepare this proposal.

- **`docs/adrs/0003-assessment-aggregate-root.md`** — read in full. Lists `AssessmentReport` as an Assessment-owned entity example (Decision section); does not distinguish it from Evidence/Findings/Recommendations, which genuinely are owned and mutated only through `Assessment`'s own methods.
- **`docs/engineering/GOVERNANCE.md`**, Reporting Crate Boundary Rule — assigns `AssessmentReport` to `modiq-report`, explicitly stating "Reports are reflections of Assessment state. They never create new state," and that Reporting "must never... mutate runtime state."
- **`docs/architecture/Architecture.md`**, Platform Boundaries section — independently states the architecture "intentionally separates... Assessment execution... Reporting... Persistence," corroborating the Crate Boundary Rule's separation, not ADR-0003's grouping.
- **`crates/modiq-runtime/src/assessment/assessment.rs`** — confirmed directly: no field, method, or dependency on `AssessmentReport` or `modiq-report` anywhere in the file.
- **`crates/modiq-engine/src/engine/assessment_service.rs`** — confirmed directly: `AssessmentReport::generate(&assessment)` is called from `modiq-engine`, external to `Assessment` itself, while `Assessment.status` is still `EvaluatingRules` (i.e., before `Assessment::complete()`), consuming only `Assessment`'s already-public getters.
- **`docs/engineering/PROJECT_HANDOFF_v1.1.md`** — confirmed to still repeat ADR-0003's original framing without qualification, in its current, just-reconciled (Sprint 14) text.
- **`docs/engineering/GOV002_ARCHITECTURE_EVALUATION.md`**, Finding 3 and §7 Option C — the evaluation that surfaced this question, including two candidate resolution mechanisms (ADR amendment vs. a new superseding ADR), neither selected there.
- **`docs/engineering/ENGINEERING_LOG.md`** — grepped for every prior mention of ADR-0003; confirmed this specific claim has never been examined or corrected in this repository's history before Sprint 15.

---

## Why This Requires Governance Rather Than a Routine Documentation Edit

Per `docs/engineering/GOVERNANCE.md`'s own Change Categories, an edit is Level 1 (Editorial) or Level 2 (Clarification) only when it carries no semantic change, or improves wording without altering what is claimed architecturally. This case is neither:

- ADR-0003 is an **Accepted Architecture Decision Record** — per this project's own standing discipline (`CHIEF_ARCHITECT_HANDOFF_v1.1.md` §3, "Architectural Ownership"), a conflict between a proposal and an existing ADR is resolved by a new ADR that supersedes the old one, or an explicitly-recorded amendment — **never by quietly reinterpreting or silently rewriting it**. Correcting ADR-0003's substantive ownership claim without a governance decision would be exactly the "silent rewrite" this project's documentation-authority discipline exists to prevent.
- The question has a real, if narrow, architectural consequence: which document's ownership claim is authoritative for `AssessmentReport` bears on where future work involving it (e.g., any future Storage or Knowledge Base integration touching Reports) should be designed against.
- It is a Level 3/4-shaped question under `GOVERNANCE.md`'s own categories — resolving "which document is right, and how the correction is recorded" is a decision about documentation authority itself, not merely a wording improvement.

---

## Success Criteria

This item should be considered resolved when:

1. ADR-0003's `AssessmentReport` ownership claim and the Reporting Crate Boundary Rule no longer conflict — whichever document is corrected, or whichever new ADR is issued, leaves exactly one authoritative statement of `AssessmentReport`'s ownership.
2. `PROJECT_HANDOFF_v1.1.md` (and any other living document found to repeat ADR-0003's original framing) is reconciled to match the corrected statement.
3. ADR-0003's still-valid core decision — `Assessment` as the Runtime domain's sole aggregate root — is preserved untouched; this item does not reopen that question.
4. The correction is recorded explicitly (an ADR amendment note, or a new superseding ADR), never as a silent rewrite of Accepted architecture.

---

## Recommended Workflow After Initiation

Per the governance workflow this Sprint is bound to, and not performed here: once the Chief Architect approves this proposal and it is entered into the Register as **GOV-015**, the next stage is **Architecture Evaluation** — evaluating the two candidate mechanisms named in `GOV002_ARCHITECTURE_EVALUATION.md` §7 Option C (in-place ADR amendment vs. a new superseding ADR) against repository precedent, and identifying any other living document requiring reconciliation beyond `PROJECT_HANDOFF_v1.1.md`. Given how much of this investigation already exists from Sprint 15's own work, that evaluation may be narrow rather than starting from zero — but that judgment, and the evaluation itself, belongs to that stage, not this one.

**This document does not perform that evaluation, does not recommend a resolution, and does not authorize any further work.** It ends at Governance Initiation, awaiting Chief Architect approval before GOV-015 is entered into `GOVERNANCE.md`.

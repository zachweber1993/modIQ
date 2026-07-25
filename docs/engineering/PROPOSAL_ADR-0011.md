# Proposal: ADR-0011 — Superseding ADR-0003's `AssessmentReport` Ownership Example

| Property | Value |
|----------|-------|
| **Document** | PROPOSAL_ADR-0011.md |
| **Stage** | Proposal only |
| **Next available ADR identifier** | **ADR-0011** — confirmed directly against `docs/adrs/README.md`; the highest existing ADR is ADR-0010 |
| **Prepared by** | Lead Engineer session, Sprint 18 |
| **Origin** | GOV-015 Architectural Resolution (`docs/engineering/GOV015_ARCHITECTURE_EVALUATION.md`, §6.4, Recommended Follow-up Work, Item 1) |
| **Status** | **Proposal only. Nothing in this document is authoritative or accepted.** It does not create ADR-0011, does not modify ADR-0003, and does not authorize implementation. It exists solely for Chief Architect review. Upon approval, it becomes the basis for ADR-0011 itself — creation of that document, and any resulting repository reconciliation, each require their own separate, explicit authorization, per this project's stage-gated governance workflow. |

*Revised following Chief Architect review — see Revision Note at the end of this document.*

---

## Title

**Superseding ADR-0003's `AssessmentReport` Ownership Example**

(Named to describe the mechanism directly — this is a superseding ADR, per `docs/adrs/README.md`'s own Engineering Notes, not an amendment to ADR-0003 itself.)

---

## Background

GOV-015 (Role of ADR-0003 in Describing `AssessmentReport` Ownership) evaluated whether ADR-0003's inclusion of `Assessment Report` among Assessment-owned entities still accurately describes the platform's architecture. It found that `Assessment Report` does not fit the ownership definition the platform otherwise applies to Runtime entities, and that the discrepancy is an editorial imprecision in ADR-0003's original example list rather than an architectural inconsistency — the Reporting subsystem boundary has been correct and unchanged throughout.

Per `docs/adrs/README.md`'s policy that Accepted ADRs are not modified, GOV-015's Architectural Resolution selected a superseding ADR — narrowly correcting this one example — as the mechanism, rather than an in-place amendment to ADR-0003. This proposal packages that already-resolved decision into draft ADR text for review. `docs/engineering/GOV015_ARCHITECTURE_EVALUATION.md` remains the authoritative record of the underlying evidence and reasoning; it is not reproduced here.

---

## Scope

**Narrow, matching GOV-015's own Resolution:**

- Corrects only ADR-0003's inclusion of `Assessment Report` in its list of Assessment-owned entities.
- Leaves untouched: `Assessment` as the Runtime domain's sole aggregate root; `Assessment Context`, `Assessment Subject`, `Evidence`, `Findings`, and `Recommendations` as genuinely Assessment-owned entities, exactly as ADR-0003 already states.
- Leaves ADR-0003 itself unmodified and otherwise standing as an accurate historical record of what it decided — consistent with `docs/adrs/README.md`'s policy that accepted ADRs are not rewritten.
- Does not reopen any question about the Reporting subsystem's boundary or the Architecture specification's Platform Boundaries — both are already correct and are confirmed, not revised, by the proposed decision below.

---

## Proposed ADR-0011 (Draft Text — Not Accepted, For Review Only)

*The following is proposed content for `docs/adrs/0011-assessmentreport-ownership-correction.md`, styled to match the existing ADR format (compare ADR-0003, ADR-0010). It does not exist as an accepted document. It is reproduced here in full so the Chief Architect can review the exact text before any ADR file is created.*

> # ADR-0011: `AssessmentReport` Ownership Correction
>
> | Property | Value |
> |----------|-------|
> | **ADR** | 0011 |
> | **Title** | `AssessmentReport` Ownership Correction |
> | **Status** | Proposed |
> | **Project** | modIQ |
> | **Date** | *(set upon acceptance)* |
> | **Supersedes** | ADR-0003, solely with respect to its inclusion of `Assessment Report` among Assessment-owned entities |
>
> ---
>
> # Context
>
> ADR-0003 ("Assessment as the Aggregate Root," Accepted) names `Assessment Report` as an example of an Assessment-owned entity. The platform's Reporting subsystem boundary treats `Assessment Report` as generated from Assessment's execution, not owned or mutated by Assessment itself.
>
> This ADR resolves that discrepancy. It reflects a governance evaluation (GOV-015, `docs/engineering/GOVERNANCE.md`) that found the discrepancy to be an editorial imprecision in ADR-0003's original example list — `Assessment Report` was never actually owned by Assessment in the sense the platform's ownership definition for Runtime entities (ADR-0007) requires — not evidence of the architecture having changed. The Reporting subsystem boundary itself has been accurate throughout and is confirmed, not altered, by this decision. The full evidentiary record supporting this conclusion is maintained in GOV-015, not restated here.
>
> ---
>
> # Decision
>
> This ADR supersedes ADR-0003's Decision section **solely** with respect to its inclusion of `Assessment Report`. Effective upon acceptance:
>
> - `Assessment Context`, `Assessment Subject`, `Evidence`, `Findings`, and `Recommendations` remain accurately described as Assessment-owned entities exactly as ADR-0003 states.
> - **`Assessment Report` is not an Assessment-owned entity.** It is produced by the platform's Reporting subsystem from Assessment's already-public state, and is never held or mutated by Assessment itself — consistent with the Reporting subsystem's Crate Boundary Rule and the Architecture specification's Platform Boundaries, both of which are confirmed, not changed, by this decision.
> - **`Assessment` remains the Runtime domain's sole aggregate root.** This ADR does not reopen, narrow, or otherwise affect that determination in any respect.
> - **ADR-0003 is not modified.** Its Context, Decision, Consequences, and Status sections stand unedited as an accurate historical record of what was decided. This ADR supersedes only its `Assessment Report` example, going forward; readers of ADR-0003 should consult this ADR for that one claim.
>
> ---
>
> # Consequences
>
> **Benefits:**
>
> - Resolves the conflict between ADR-0003's example list and the Reporting subsystem's boundary — both already correct in substance and unchanged by this decision.
> - Establishes the platform's Runtime-entity ownership definition (ADR-0007) as the controlling standard against which any Accepted ADR's own examples should be checked going forward.
> - Fully closes the documentary question GOV-015 raised, without disturbing any of ADR-0003's other still-valid content.
>
> **Trade-offs:**
>
> - Exercises the superseding-ADR mechanism described in `docs/adrs/README.md`, applied here to an Accepted ADR rather than to a lower-tier technical specification.
>
> ---
>
> # Relationship to Other Specifications
>
> This decision is reflected in:
>
> - **ADR-0003** — superseded solely with respect to its `Assessment Report` example; every other decision in ADR-0003 remains fully in force and unchanged.
> - **ADR-0007** — the Runtime-entity ownership definition this decision applies; unaffected in substance, reaffirmed as the controlling standard.
> - **`docs/engineering/GOVERNANCE.md`**, Reporting Crate Boundary Rule — confirmed, not changed.
> - **`docs/architecture/Architecture.md`**, Platform Boundaries — confirmed, not changed.
> - **GOV-015** (`docs/engineering/GOVERNANCE.md`) — this ADR is the artifact GOV-015's Architectural Resolution recommended preparing.
>
> ---
>
> # Status
>
> Proposed. Not yet Accepted.

---

## What This Proposal Does Not Do

Per the Chief Architect's explicit Sprint 18 scope, this document:

- Does **not** create `docs/adrs/0011-*.md` as an accepted repository document.
- Does **not** modify ADR-0003.
- Does **not** modify `docs/engineering/GOVERNANCE.md`.
- Does **not** update `docs/governance/PROJECT_STATUS.md`.
- Does **not** update `docs/engineering/ENGINEERING_LOG.md`.
- Does **not** update `docs/governance/CHANGELOG.md`.
- Performs no implementation and touches no Rust source.
- Is not committed by its own creation — commit remains a separately authorized action.

---

## Recommended Workflow After This Proposal

Per the governance workflow this Sprint is bound to, and not performed here: Chief Architect review of the draft ADR-0011 text above. If approved, creating `docs/adrs/0011-*.md` as an accepted document is a separately authorized stage, followed by Repository Reconciliation (`GOVERNANCE.md`'s GOV-015 entry, `PROJECT_STATUS.md`, `ENGINEERING_LOG.md`, `CHANGELOG.md`, and a re-check of `PROJECT_HANDOFF_v1.1.md`'s own ADR-0003 citation) as a further, separately authorized stage.

**This document does not perform any of that. It ends at Proposal, awaiting Chief Architect review.**

---

## Revision Note

Following initial Chief Architect review ("Approved with Minor Revisions"), this document was revised to: replace implementation-level detail (concrete methods, fields, file paths) with architecture-level language, both in this proposal's own Background and in the draft ADR-0011 text, on the understanding that GOV-015 remains the authoritative evidence record and is not reproduced here; condense the historical narrative accordingly; remove repository-relative framing ("the first ADR... to supersede...") from the draft ADR so its text remains accurate regardless of future repository evolution; and remove forward-looking repository-reconciliation guidance from the draft ADR's own Consequences section — that guidance now appears only in this proposal's "Recommended Workflow After This Proposal" section, outside the draft ADR text itself. No architectural decision, scope, or conclusion was changed by this revision.

# ADR-0011: `AssessmentReport` Ownership Correction

| Property | Value |
|----------|-------|
| **ADR** | 0011 |
| **Title** | `AssessmentReport` Ownership Correction |
| **Status** | Accepted |
| **Project** | modIQ |
| **Date** | 2026-07-24 |
| **Supersedes** | ADR-0003, solely with respect to its inclusion of `Assessment Report` among Assessment-owned entities |

---

# Context

ADR-0003 ("Assessment as the Aggregate Root," Accepted) names `Assessment Report` as an example of an Assessment-owned entity. The platform's Reporting subsystem boundary treats `Assessment Report` as generated from Assessment's execution, not owned or mutated by Assessment itself.

This ADR resolves that discrepancy. It reflects a governance evaluation (GOV-015, `docs/engineering/GOVERNANCE.md`) that found the discrepancy to be an editorial imprecision in ADR-0003's original example list — `Assessment Report` was never actually owned by Assessment in the sense the platform's ownership definition for Runtime entities (ADR-0007) requires — not evidence of the architecture having changed. The Reporting subsystem boundary itself has been accurate throughout and is confirmed, not altered, by this decision. The full evidentiary record supporting this conclusion is maintained in GOV-015, not restated here.

---

# Decision

This ADR supersedes ADR-0003's Decision section **solely** with respect to its inclusion of `Assessment Report`. Effective upon acceptance:

- `Assessment Context`, `Assessment Subject`, `Evidence`, `Findings`, and `Recommendations` remain accurately described as Assessment-owned entities exactly as ADR-0003 states.
- **`Assessment Report` is not an Assessment-owned entity.** It is produced by the platform's Reporting subsystem from Assessment's already-public state, and is never held or mutated by Assessment itself — consistent with the Reporting subsystem's Crate Boundary Rule and the Architecture specification's Platform Boundaries, both of which are confirmed, not changed, by this decision.
- **`Assessment` remains the Runtime domain's sole aggregate root.** This ADR does not reopen, narrow, or otherwise affect that determination in any respect.
- **ADR-0003 is not modified.** Its Context, Decision, Consequences, and Status sections stand unedited as an accurate historical record of what was decided. This ADR supersedes only its `Assessment Report` example, going forward; readers of ADR-0003 should consult this ADR for that one claim.

---

# Consequences

**Benefits:**

- Resolves the conflict between ADR-0003's example list and the Reporting subsystem's boundary — both already correct in substance and unchanged by this decision.
- Establishes the platform's Runtime-entity ownership definition (ADR-0007) as the controlling standard against which any Accepted ADR's own examples should be checked going forward.
- Fully closes the documentary question GOV-015 raised, without disturbing any of ADR-0003's other still-valid content.

**Trade-offs:**

- Exercises the superseding-ADR mechanism described in `docs/adrs/README.md`, applied here to an Accepted ADR rather than to a lower-tier technical specification.

---

# Relationship to Other Specifications

This decision is reflected in:

- **ADR-0003** — superseded solely with respect to its `Assessment Report` example; every other decision in ADR-0003 remains fully in force and unchanged.
- **ADR-0007** — the Runtime-entity ownership definition this decision applies; unaffected in substance, reaffirmed as the controlling standard.
- **`docs/engineering/GOVERNANCE.md`**, Reporting Crate Boundary Rule — confirmed, not changed.
- **`docs/architecture/Architecture.md`**, Platform Boundaries — confirmed, not changed.
- **GOV-015** (`docs/engineering/GOVERNANCE.md`) — this ADR is the artifact GOV-015's Architectural Resolution recommended preparing.

---

# Status

Accepted.

Repository reconciliation (`docs/adrs/README.md`'s Current ADRs index, `GOVERNANCE.md`'s GOV-015 entry, `PROJECT_STATUS.md`, `ENGINEERING_LOG.md`, `CHANGELOG.md`, and `PROJECT_HANDOFF_v1.1.md`'s own ADR-0003 citation) is not performed by this document and remains a separately authorized stage.

# Architecture Decision Records (ADRs)

Architecture Decision Records (ADRs) document the significant architectural decisions made during the design and evolution of the modIQ platform.

Each ADR captures the context, rationale, decision, and consequences of a specific architectural choice.

Together, the ADRs provide a historical record of the platform's architectural evolution and serve as the authoritative source for understanding why key decisions were made.

---

## Purpose

The ADR process exists to:

- Preserve architectural intent
- Document engineering rationale
- Prevent architectural drift
- Support long-term maintainability
- Provide historical traceability

Architectural decisions should remain discoverable long after implementation details evolve.

---

## When to Create an ADR

An ADR should be created whenever a decision:

- Changes the platform architecture
- Introduces a significant engineering constraint
- Alters ownership boundaries
- Establishes a new architectural principle
- Affects multiple platform components
- Has long-term consequences for maintainability or extensibility

Implementation details that do not alter the architecture should not require an ADR.

---

## ADR Lifecycle

Architectural decisions follow this process:

1. Identify the architectural issue.
2. Evaluate alternatives.
3. Record the decision.
4. Review with project leadership.
5. Approve.
6. Implement.
7. Reference the ADR from implementation where appropriate.

---

## ADR Naming Convention

Architecture Decision Records use sequential numbering.

Examples:

```text
ADR-0001-Foundation-Freeze.md
ADR-0002-Domain-Model-Boundaries.md
ADR-0003-Assessment-Aggregate-Root.md
```

ADR numbers are permanent and should never be reused.

---

## Current ADRs

| ADR | Title | Status |
|-----|-------|--------|
| ADR-0001 | Foundation Freeze | Accepted |
| ADR-0002 | Domain Model Boundaries | Accepted |
| ADR-0003 | Assessment Aggregate Root | Accepted |
| ADR-0004 | Platform-First Version Profiles | Accepted |
| ADR-0005 | Deterministic Assessment Engine | Accepted |
| ADR-0006 | Documentation Release 1.0 Freeze | Accepted |

Future ADRs should continue the established numbering sequence.

---

## Relationship to the Engineering Specification

The Engineering Specification defines **what** the platform is.

ADRs explain **why** significant architectural decisions were made.

Implementation should follow the Engineering Specification while respecting the architectural intent documented by accepted ADRs.

---

## Engineering Notes

ADRs are historical engineering records.

Accepted ADRs should not be modified to reflect new decisions.

If an architectural decision changes, a new ADR should be created that supersedes or amends the previous decision while preserving the historical record.

When in doubt:

> Architecture evolves through new decisions, not by rewriting history.
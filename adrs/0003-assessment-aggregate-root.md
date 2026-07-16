# ADR-0003: Assessment as the Aggregate Root

| Property | Value |
|----------|-------|
| **ADR** | 0003 |
| **Title** | Assessment as the Aggregate Root |
| **Status** | Accepted |
| **Project** | modIQ |
| **Date** | 2026-07-16 |

---

# Context

The runtime domain of modIQ represents information that exists only while an Assessment is being performed or as the result of a completed Assessment.

During architectural design, it became necessary to define a single aggregate root responsible for coordinating the lifecycle of runtime entities and preserving clear ownership boundaries.

Without a defined aggregate root, runtime entities such as Evidence, Findings, Recommendations, and Assessment Reports risked becoming independently managed, leading to inconsistent ownership, reduced traceability, and increased architectural complexity.

A single runtime authority was required to ensure deterministic execution and maintain conceptual integrity throughout the assessment lifecycle.

---

# Decision

The **Assessment** is designated as the aggregate root of the runtime domain.

All runtime entities exist within the context of an Assessment and derive their lifecycle from it.

Examples of Assessment-owned entities include:

- Assessment Context
- Assessment Subject
- Evidence
- Findings
- Recommendations
- Assessment Report

Runtime entities shall not exist independently of an Assessment.

Reusable engineering knowledge remains outside the runtime domain and is governed separately by the Knowledge Model.

---

# Consequences

Benefits include:

- Clear ownership of runtime entities.
- Deterministic assessment lifecycle management.
- Improved traceability between Evidence, Findings, Recommendations, and Assessment Reports.
- Reduced coupling between runtime execution and reusable engineering knowledge.
- Consistent implementation of the runtime domain across future platform capabilities.

This decision establishes a stable conceptual model that supports future platform evolution without altering runtime ownership boundaries.

---

# Relationship to Other Specifications

This decision is reflected in:

- DataModel.md
- RuleEngine.md
- EngineAPI.md
- Sprint0.md
- Sprint1.md

Future runtime implementations should preserve Assessment as the aggregate root.

---

# Status

Accepted.

This decision is authoritative for Documentation Release 1.0.
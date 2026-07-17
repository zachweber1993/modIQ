# Data Model

> **The authoritative specification defining the runtime domain model of the modIQ platform.**

---

| Property | Value |
|----------|-------|
| **Document** | DataModel.md |
| **Version** | 1.0.0 |
| **Status** | Frozen |
| **Project** | modIQ |
| **Documentation Release** | 1.0 |
| **Owner** | Zach Weber |
| **Created** | 2026-07-15 |
| **Last Updated** | 2026-07-15 |

---

# Specification Authority

Authority:

- Vision.md
- Principles.md
- Glossary.md
- ProductSpecification.md
- Architecture.md

This document governs:

- KnowledgeModel.md
- RuleEngine.md
- EngineAPI.md

If a conflict exists between this document and a higher-level specification, the higher-level specification takes precedence.

---

# Purpose

This document defines the conceptual runtime data model of the modIQ platform.

It identifies the primary runtime entities, their responsibilities, and the relationships between them.

This specification intentionally avoids implementation details including databases, serialization formats, APIs, object models, and programming language constructs.

Its purpose is to establish a stable conceptual domain model that all implementations of modIQ should preserve.

---

# Relationship to Other Specifications

This document defines **runtime assessment data**.

It does not define:

- engineering knowledge
- rule implementation
- API contracts
- storage technologies
- persistence models

These concerns are addressed within their respective specifications.

---

# Data Modeling Principles

## Conceptual First

The Data Model describes business concepts rather than implementation structures.

Entities represent stable concepts within the runtime domain.

---

## Runtime Domain Only

This specification defines entities that exist because an Assessment is executed.

Reusable engineering knowledge is defined separately within KnowledgeModel.md.

---

## Assessment-Centric

The runtime domain is centered around the Assessment.

Every runtime entity exists within the context of a single Assessment.

Assessment is the aggregate root of the runtime domain.

---

## Stable Identity

Each entity represents a unique conceptual object.

Implementation identifiers are intentionally excluded from this specification.

---

## Immutability

Completed Assessments represent historical records.

Historical assessment results should remain reproducible.

If assessment behavior changes, a new Assessment should be performed rather than modifying previous Assessment results.

---

## Separation of Concerns

Runtime data should remain independent from engineering knowledge.

Assessment execution, knowledge representation, and reporting evolve independently while cooperating through clearly defined relationships.

---

# Domain Overview

The runtime domain represents information generated during an Assessment.

The primary aggregate is the Assessment itself.

Supporting entities provide context, evidence, conclusions, recommendations, and reporting.

Engineering knowledge that exists independently of any Assessment is intentionally excluded from this model.

---

# Runtime Domain

## Assessment

Assessment is the primary runtime entity of the platform.

An Assessment represents a deterministic evaluation of a single Assessment Subject using the modIQ Assessment Framework.

The Assessment coordinates evidence collection, rule execution, finding generation, recommendation generation, and Assessment Report production.

All runtime entities belong to exactly one Assessment.

---

## Assessment Subject

An Assessment Subject represents the artifact being evaluated.

Examples include:

- Farming Simulator mods
- Mod collections
- Savegames
- Maps
- Future supported assessment targets

Each Assessment evaluates exactly one Assessment Subject.

---

## Assessment Context

Assessment Context describes the environment in which an Assessment was performed.

Examples may include:

- Version Profile
- Assessment timestamp
- Assessment configuration
- Platform capabilities
- Execution environment

Assessment Context ensures Assessment results remain reproducible.

---

## Evidence

Evidence represents objective information collected during an Assessment.

Evidence serves as the factual basis for all Findings.

Every Finding should be traceable to supporting Evidence.

Evidence never represents opinion.

---

## Finding

A Finding represents a single conclusion derived from evaluated Evidence.

Findings communicate observations generated during an Assessment.

Multiple Findings contribute to the overall Assessment.

---

## Recommendation

A Recommendation represents actionable guidance generated from one or more Findings.

Recommendations exist to improve understanding and assist user decision making.

Recommendations are informative rather than mandatory.

---

## Assessment Report

The Assessment Report represents the structured output of an Assessment.

The Assessment Report organizes:

- Assessment summary
- Evidence
- Findings
- Recommendations
- Supporting explanations

The Assessment Report communicates Assessment results but performs no analysis.

---

## Version Profile

Version Profiles provide game-version context for an Assessment.

Version Profiles define the compatibility context within which an Assessment is performed.

They enable platform support for multiple Farming Simulator releases without architectural redesign.

Unlike the other entities described in this document, a Version Profile is not owned or generated by the Assessment. It is defined and maintained independently of any individual Assessment and is referenced by the Assessment for context, per VersionProfile.md.

---

# Entity Relationships

The conceptual runtime model is organized around the Assessment.

```text
Assessment
│
├── has one Assessment Subject
├── has one Assessment Context
├── collects many Evidence items
├── produces many Findings
├── produces many Recommendations
├── generates one Assessment Report
└── uses one Version Profile
```

All runtime entities other than the Version Profile exist within the lifecycle of a single Assessment. The Version Profile exists independently and is only referenced by the Assessment.

---

# Runtime Lifecycle

The conceptual lifecycle of runtime entities is:

```text
Assessment Created
        │
        ▼
Assessment Context Established
        │
        ▼
Evidence Collected
        │
        ▼
Findings Produced
        │
        ▼
Recommendations Generated
        │
        ▼
Assessment Report Produced
        │
        ▼
Assessment Completed
```

---

# Ownership

Runtime ownership is hierarchical.

Assessment owns all runtime entities generated during its execution, with the exception of the Version Profile, which the Assessment references but does not own.

Supporting entities other than the Version Profile do not exist independently of an Assessment.

This ownership model simplifies traceability and preserves the educational focus of the platform.

---

# Immutability

Completed Assessments represent historical observations.

Historical Assessments should remain unchanged after completion.

If new Rules, Knowledge, or platform capabilities become available, subsequent Assessments should generate new historical records rather than altering previous results.

---

# Future Evolution

The runtime model is intentionally platform-first.

Future Assessment Subjects, Version Profiles, and runtime capabilities should integrate into the existing model without requiring structural redesign.

Stable runtime concepts should evolve through extension rather than replacement.

---

# Relationship to Other Specifications

This specification defines runtime entities only.

Related specifications include:

- KnowledgeModel.md defines reusable engineering knowledge.
- RuleEngine.md defines deterministic assessment execution.
- EngineAPI.md defines subsystem interfaces.
- Architecture.md defines platform organization.

---

# Document Status

**Current Version:** 1.0.0

**Status:** Frozen

This specification establishes the authoritative conceptual runtime data model for the modIQ platform and serves as the foundation for the remaining technical specifications.
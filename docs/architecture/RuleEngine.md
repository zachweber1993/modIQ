# Rule Engine

> **The authoritative specification defining the conceptual execution model of the modIQ assessment engine.**

---

| Property | Value |
|----------|-------|
| **Document** | RuleEngine.md |
| **Version** | 1.0.1 |
| **Status** | Frozen, with a Documentation Release 2.1 cross-reference addition; amended 2026-08-01 for Engineering Alignment Reconciliation (Initiatives 1, 3) |
| **Project** | modIQ |
| **Documentation Release** | 1.0 (amended under 2.1) |
| **Owner** | Zach Weber |
| **Created** | 2026-07-16 |
| **Last Updated** | 2026-07-19 |

---

# Specification Authority

Authority:

- Vision.md
- Principles.md
- Glossary.md
- ProductSpecification.md
- Architecture.md
- DataModel.md
- KnowledgeModel.md

This document governs:

- EngineAPI.md

If a conflict exists between this document and a higher-level specification, the higher-level specification takes precedence.

---

# Purpose

This document defines the conceptual execution model of the modIQ Rule Engine.

The Rule Engine transforms reusable engineering knowledge and runtime evidence into deterministic, explainable assessment outcomes.

This specification intentionally excludes implementation details including programming languages, scripting systems, execution frameworks, serialization formats, and storage technologies.

Its purpose is to define **how** engineering knowledge is operationalized during an Assessment while remaining independent of implementation.

---

# Relationship to Other Specifications

The Rule Engine exists between the runtime domain and the knowledge domain.

It consumes:

- Runtime Evidence
- Engineering Knowledge

It produces:

- Findings
- Recommendations
- Explainable Assessment Results

The Rule Engine owns neither runtime data nor engineering knowledge.

It coordinates the interaction between them.

---

# Execution Principles

## Deterministic

Given identical runtime inputs and identical engineering knowledge, the Rule Engine should produce identical assessment outcomes.

---

## Evidence-Based

Rules evaluate runtime Evidence.

Rules never operate on assumptions or unsupported observations.

Every conclusion should be supported by collected Evidence.

---

## Knowledge-Driven

Rules represent executable expressions of engineering knowledge.

The Rule Engine operationalizes knowledge rather than creating it.

---

## Explainable

Every Finding should be explainable.

Every Recommendation should be traceable.

Every conclusion should be understandable.

The Rule Engine should never produce opaque or unsupported outcomes.

---

## Traceable

Assessment outcomes should maintain complete traceability between:

- Engineering Knowledge
- Rule
- Evidence
- Finding
- Recommendation

No assessment outcome should exist without supporting traceability.

---

## Version Aware

The Rule Engine evaluates knowledge within the context of the active Version Profile.

Version awareness should influence rule applicability without altering the conceptual execution model.

---

# Rule Engine Responsibilities

The Rule Engine performs six conceptual responsibilities.

## Rule Selection

Determine which Rules are applicable to the current Assessment based on the Assessment Context and Version Profile.

---

## Evidence Evaluation

Evaluate collected runtime Evidence against applicable Rules.

Evidence evaluation represents the core responsibility of the Rule Engine.

---

## Finding Generation

Produce Findings supported by evaluated Evidence.

Every Finding should be directly traceable to both the evaluated Rule and the supporting Evidence.

Any future mechanism exposing Findings before an Assessment completes shall preserve set-wide finalization — an Adopted Architectural Constraint from Engineering Alignment, Initiative 1 (AC-4). The Finding set Rules produce remains a single atomic batch; this responsibility is not altered by that constraint, only bound by it going forward.

---

## Recommendation Generation

Generate Recommendations from Findings. Associated Repair Recipes inform Recommendation content but do not independently trigger a Recommendation.

Recommendations should assist user understanding without automatically modifying content.

A Finding may stand without a paired Recommendation — Engineering Alignment, Initiative 3, Item 3 (Adopted). Pairing a Recommendation with a Finding is no longer mandatory; this responsibility generates a Recommendation only when one is warranted, not unconditionally alongside every Finding.

---

## Traceability Management

Maintain conceptual relationships between:

- Knowledge
- Rules
- Evidence
- Findings
- Recommendations

This traceability supports explainability, reproducibility, and user education.

---

## Explainability

Provide sufficient reasoning for every Assessment outcome.

Users should understand:

- what was evaluated
- why it was evaluated
- what evidence supported the conclusion
- how the conclusion was reached

---

# Conceptual Execution Lifecycle

The Rule Engine participates in the Assessment lifecycle as follows:

```text
Assessment Created
        │
        ▼
Assessment Context Established
        │
        ▼
Applicable Rules Selected
        │
        ▼
Evidence Evaluated
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

This lifecycle defines conceptual execution only.

Implementation details are intentionally excluded.

**Dispatch boundaries (Engineering Alignment, Initiative 1, AC-2 — Adopted Architectural Constraint).** Evidence Evaluated, Findings Produced, and Recommendations Generated are the architecturally coherent, already-deterministic dispatch boundaries within this lifecycle. Any future progressive-observation mechanism may observe only at boundaries already named here — never at an arbitrary, time-based, or percentage-based cadence. This confirms boundaries this lifecycle already established; it adds no new boundary and reorders nothing.

---

# Relationship to the Runtime Domain

The Rule Engine consumes runtime entities defined by DataModel.md.

These include:

- Assessment
- Assessment Context
- Assessment Subject
- Evidence

The Rule Engine produces:

- Findings
- Recommendations
- Assessment Report content

The Rule Engine does not own runtime entities.

Evidence is produced by the Evidence Collection subsystem (EvidenceCollection.md, ADR-0008), not by the Rule Engine. This document's boundary — the Rule Engine consumes Evidence, it never produces it — is unchanged by that subsystem's introduction; EvidenceCollection.md was written specifically to preserve it.

---

# Relationship to the Knowledge Domain

The Rule Engine consumes reusable engineering knowledge defined by KnowledgeModel.md.

Examples include:

- Rules
- Repair Recipes
- Engine Behaviors
- Compatibility Patterns
- Best Practices
- Known Issues

The Rule Engine does not modify engineering knowledge.

Knowledge evolves independently of runtime execution.

---

# Conceptual Boundaries

The Rule Engine does not define:

- Rule syntax
- Scripting languages
- Execution scheduling
- Programming interfaces
- Storage technologies
- APIs
- Database models

These concerns belong to future implementation specifications.

---

# Future Evolution

The Rule Engine is intentionally implementation independent.

Future execution optimizations, parallel processing, distributed execution, AI-assisted analysis, or plugin architectures should preserve the conceptual execution model defined by this specification.

The execution model should remain deterministic, explainable, and evidence-based regardless of implementation strategy.

---

# Document Status

**Current Version:** 1.0.1

**Status:** Frozen, with a Documentation Release 2.1 cross-reference addition

**Amended 2026-08-01 — Engineering Alignment Reconciliation.** Synchronizes already-adopted outcomes of Engineering Alignment Initiatives 1 and 3 into this specification: Finding Generation's set-wide-finalization constraint (Initiative 1, AC-4), the Conceptual Execution Lifecycle's confirmed dispatch boundaries for future progressive observation (Initiative 1, AC-2), and Recommendation Generation's now-optional Finding/Recommendation pairing (Initiative 3, Item 3). This amendment introduces no new architectural decision — every addition traces directly to an Adopted fact or Adopted Architectural Constraint already recorded in `INITIATIVE_1_ARCHITECTURAL_RESOLUTION.md` and `INITIATIVE_3_ARCHITECTURAL_RESOLUTION.md`. No responsibility, boundary, or lifecycle ordering was added, removed, or reordered — Initiative 1's own still-open questions (execution-phase signal granularity, D-1; the Report-evolution model, D-2) remain unresolved and are not addressed here.

This specification establishes the authoritative conceptual execution model for the modIQ assessment engine. Under Documentation Release 2.1 (Frozen, Technical Director approved), a cross-reference to EvidenceCollection.md was added to the Relationship to the Runtime Domain section, reinforcing (not altering) this document's existing Evidence-consumer boundary.
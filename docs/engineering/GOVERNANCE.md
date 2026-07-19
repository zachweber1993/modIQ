# modIQ Governance

Version: 1.0

---

# Purpose

This document defines the governance process for evolving the modIQ architecture after implementation has begun.

Architecture is considered a managed asset of the project. Once documented and implemented, it may only evolve through an intentional governance process.

Implementation must never silently redefine the architecture.

Engineering discoveries are expected, but architectural changes must always be documented, reviewed, and approved before becoming part of the platform.

---

# Governance Principles

1. Documentation is authoritative.
2. Architecture precedes implementation.
3. Engineering may expose documentation deficiencies.
4. Implementation does not redefine architecture.
5. Every architectural change must be documented.
6. Simplicity takes precedence over speculation.
7. Crate boundaries are protected.
8. Public APIs are treated as contracts.
9. Deterministic behavior is mandatory.
10. Governance decisions are repository artifacts.

---

# Governance Scope

This document governs:

- Architectural evolution
- Runtime domain boundaries
- Crate responsibilities
- Dependency hierarchy
- Public API evolution
- Runtime invariant evolution
- Documentation authority
- Engineering release governance
- Documentation release governance

This document does **not** govern:

- Product roadmap
- Feature prioritization
- Sprint planning
- Coding style
- Formatting conventions
- Development tooling
- Individual implementation tasks

---

# Documentation Authority

When two documents conflict, precedence shall be determined in the following order.

1. Vision
2. Product Specification
3. Architecture
4. Data Model
5. Principles
6. Architecture Decision Records (ADRs)
7. Governance
8. Engineering Release Documents
9. Engineering Log
10. Source Code

Implementation shall never be considered authoritative when it conflicts with higher-level documentation.

Instead, the discrepancy shall be recorded as a Governance Item and resolved through the governance process.

---

# Change Categories

All repository changes fall into one of four categories.

## Level 1 — Editorial

Examples:

- grammar
- spelling
- formatting
- wording improvements with no semantic change

Requires:

- no governance review

---

## Level 2 — Clarification

Examples:

- improved documentation wording
- clearer invariant descriptions
- terminology reconciliation

Requires:

- governance review
- documentation update

Implementation should remain unaffected.

---

## Level 3 — Behavioral

Examples:

- lifecycle modifications
- invariant changes
- API behavior changes
- assessment workflow revisions

Requires:

- governance approval
- documentation update
- implementation update
- engineering review

---

## Level 4 — Architectural

Examples:

- new crates
- aggregate redesign
- dependency changes
- ownership changes
- major public API redesign

Requires:

- ADR
- governance approval
- documentation release
- engineering implementation
- engineering release

---

# Architectural Review Process

Every architectural change follows the same lifecycle.

```text
Proposal

↓

Architecture Review

↓

Governance Decision

↓

Documentation Update

↓

Implementation

↓

Engineering Review

↓

Engineering Release
```

Implementation must never skip documentation review.

---

# Public API Policy

Every public API shall exist in one of the following states.

- Experimental
- Internal
- Stable
- Deprecated
- Removed

Public APIs are considered contracts.

Breaking changes require governance approval.

---

# Crate Boundary Rules

## Runtime Domain

Owns:

- Assessment
- Lifecycle
- Runtime state
- Evidence
- Findings
- Recommendations

Must never:

- evaluate Rules
- generate Reports
- own Knowledge
- perform orchestration

---

## Rule Engine

Owns:

- deterministic evaluation
- rule execution
- rule outcomes

May consume Runtime state.

Must never mutate Assessment directly.

---

## Reporting

Owns:

- AssessmentReport
- report formatting
- report summarization
- traceability output

Must never:

- evaluate rules
- mutate runtime state

Reports are reflections of Assessment state.

They never create new state.

---

## Knowledge Domain

Owns:

- reusable engineering knowledge
- Rule definitions
- Repair Recipes
- Best Practices
- Compatibility Patterns
- Known Issues

Must remain independent from any individual Assessment.

---

## Engine

Owns:

- orchestration
- execution flow
- composition of platform services

Must never:

- own runtime state
- implement business rules
- generate reports

---

## CLI

Owns:

- user interaction
- command execution
- platform entry point

Must never contain business logic.

---

# Governance Register

Governance Items track architectural questions discovered during engineering.

Each item receives a permanent identifier.

Each item shall remain open until resolved by a Documentation Release.

---

## GOV-001

Title

Assessment Report Generation Timing

Status

Open

Raised

Engineering Release v0.1.0-alpha

Description

Assessment reports are currently generated before Assessment completion.

Question

Should reports represent the Assessment immediately before completion or after completion?

Resolution

Pending Documentation Release 1.1

---

## GOV-002

Title

Runtime Invariant Reconciliation

Status

Open

Raised

Engineering Release v0.1.0-alpha

Description

Implementation enforces lifecycle behaviors that are not yet fully represented within RuntimeInvariants.md.

Resolution

Pending Documentation Release 1.1

---

## GOV-003

Title

Role of modiq-common

Status

Open

Raised

Engineering Release v0.1.0-alpha

Description

The shared crate currently contains no implemented shared types.

Question

Should modiq-common become the repository for shared platform types, or should its architectural purpose be reconsidered?

Resolution

Pending

---

## GOV-004

Title

Engine Service Granularity

Status

Open

Raised

Engineering Release v0.1.0-alpha

Description

AssessmentService currently orchestrates the entire pipeline directly.

Question

Should future orchestration continue through AssessmentService, or transition to specialized Engine services?

Resolution

Pending

---

## GOV-005

Title

New Finding Invariant — Evidence Reference Requirement

Status

Open

Raised

Sprint 2 (Runtime Domain Content Implementation)

Description

Finding now carries an `evidence_ids` field, but no invariant requires it to be non-empty or to reference Evidence that exists within the same Assessment. RuntimeInvariants.md defines no invariant of this kind.

Question

Should a new invariant require every Finding to reference at least one Evidence item, validated against the Assessment's own Evidence collection?

Resolution

Pending Documentation Release 2.1

---

## GOV-006

Title

INV-005 Refinement — Recommendation Finding Reference Requirement

Status

Open

Raised

Sprint 2 (Runtime Domain Content Implementation)

Description

Recommendation now carries a `finding_ids` field. INV-005 ("Recommendations SHALL only be produced from one or more Findings") is currently enforced only as a temporal precondition — that some Finding exists in the Assessment — not as a content-level requirement that the Recommendation reference specific, existing Finding(s).

Question

Should INV-005 be refined to require content-level reference validation, and should this be a wording change to INV-005 itself or a new, separate invariant?

Resolution

Pending Documentation Release 2.1

---

# Documentation Release Process

Architecture evolves through Documentation Releases.

A Documentation Release represents the authoritative state of the platform architecture.

Engineering Releases represent the implementation state of the repository.

The two are intentionally independent.

---

# Repository Development Cycle

The expected development cadence is:

```text
Documentation Release

↓

Sprint Planning

↓

Implementation

↓

Engineering Review

↓

Engineering Release

↓

Governance Review

↓

Documentation Release
```

This process ensures implementation continually validates the architecture while governance continuously reconciles engineering discoveries.

---

# Governance Objectives

Governance exists to preserve long-term architectural integrity.

Its objectives are to:

- Maintain deterministic behavior.
- Preserve architectural boundaries.
- Prevent implementation drift.
- Protect public API stability.
- Ensure documentation remains authoritative.
- Capture architectural decisions as permanent repository artifacts.
- Enable sustainable evolution of the platform without sacrificing clarity or consistency.
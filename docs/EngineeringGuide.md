# Engineering Guide

| Property | Value |
|----------|-------|
| **Document** | EngineeringGuide.md |
| **Project** | modIQ |
| **Purpose** | Engineering Implementation Guide |
| **Maintained By** | Project Maintainers |
| **Last Updated** | 2026-07-16 |

---

# Purpose

This document defines the engineering practices used to implement and maintain the modIQ platform.

It establishes the relationship between the Engineering Specification and production implementation while providing guidance for contributors throughout the software development lifecycle.

This document complements, but does not supersede, the Engineering Specification.

---

# Engineering Philosophy

Implementation should realize the Engineering Specification rather than redefine it.

Production code should preserve:

- Architectural intent
- Deterministic behavior
- Explainability
- Platform independence
- Clear ownership boundaries

Implementation decisions should remain consistent with the frozen Engineering Specification unless superseded through the project's architectural governance process.

---

# Relationship to the Engineering Specification

The Engineering Specification is the authoritative source of truth for the platform.

Engineers should implement the platform by reconciling implementation decisions against the specification rather than modifying the specification to match implementation.

When implementation reveals an architectural issue, the issue should be documented and resolved through the Architecture Decision Record (ADR) process and incorporated into a future Documentation Release where appropriate.

---

# Required Reading

Engineers should become familiar with the Engineering Specification before contributing to the codebase.

Recommended reading order:

1. Vision.md
2. Principles.md
3. Glossary.md
4. ProductSpecification.md
5. Architecture.md
6. DataModel.md
7. KnowledgeModel.md
8. RuleEngine.md
9. EngineAPI.md
10. Sprint0.md
11. Sprint1.md

---

# Repository Organization

The repository is organized into distinct areas of responsibility.

## Governance

Defines repository governance and engineering process.

Examples include:

- DocumentationRelease.md
- EngineeringGuide.md

---

## Engineering Specification

Defines the architecture of the platform.

Examples include:

- Vision.md
- Architecture.md
- DataModel.md
- RuleEngine.md

---

## Architecture Decision Records

Documents significant architectural decisions.

Examples include:

- Foundation Freeze
- Domain Model Boundaries
- Assessment Aggregate Root

---

## Implementation

Contains production source code and supporting assets.

Examples include:

- engine/
- clients/
- tests/
- sample_mods/
- scripts/

---

# Engineering Principles

Engineers should:

- Implement the specification rather than reinterpret it.
- Preserve deterministic behavior.
- Prefer simple solutions over unnecessary complexity.
- Maintain explainability throughout the assessment lifecycle.
- Respect architectural boundaries.
- Write tests for production functionality.
- Preserve readability and maintainability.

---

# Working with ADRs

Architecture Decision Records preserve significant architectural decisions.

Engineers should create a new ADR when:

- Introducing a significant architectural change.
- Altering platform boundaries.
- Changing ownership relationships.
- Modifying core architectural behavior.

Routine implementation decisions should not create ADRs.

---

# Coding Standards

Implementation should prioritize:

- Readability
- Maintainability
- Determinism
- Explicit behavior
- Testability
- Small, focused modules
- Clear naming

Implementation-specific coding standards may evolve independently of the Engineering Specification.

---

# Testing Philosophy

Testing should verify:

- Deterministic behavior
- Rule correctness
- Evidence traceability
- Assessment reproducibility
- Platform stability

Production functionality should be accompanied by appropriate automated tests.

---

# Definition of Done

Implementation is considered complete when:

- The feature satisfies the Engineering Specification.
- All automated tests pass.
- Architectural boundaries remain intact.
- Documentation has been updated where required.
- No unresolved architectural conflicts exist.
- Code has been reviewed.

---

# Engineering Workflow

Development should generally follow this sequence:

1. Review the relevant specification.
2. Identify affected architectural boundaries.
3. Implement the feature.
4. Write or update tests.
5. Verify consistency with the Engineering Specification.
6. Submit for review.

---

# Future Evolution

This guide is expected to evolve alongside implementation practices.

Unlike the Engineering Specification, this document is intended to be a living guide and may be updated as engineering practices mature.

Changes to this guide should improve engineering consistency without altering the architectural intent established by the Engineering Specification.
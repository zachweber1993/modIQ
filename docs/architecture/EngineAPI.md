# Engine API

> **The authoritative specification defining the conceptual service contract of the modIQ assessment engine.**

---

| Property | Value |
|----------|-------|
| **Document** | EngineAPI.md |
| **Version** | 1.1.0 |
| **Status** | Frozen, amended following GOV-004 resolution |
| **Project** | modIQ |
| **Documentation Release** | 1.0 (amended following GOV-004 resolution) |
| **Owner** | Zach Weber |
| **Created** | 2026-07-16 |
| **Last Updated** | 2026-07-20 |

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
- RuleEngine.md

If a conflict exists between this document and a higher-level specification, the higher-level specification takes precedence.

---

# Purpose

This document defines the conceptual service contract of the modIQ assessment engine.

The Engine API describes the capabilities exposed by the platform rather than implementation-specific interfaces.

It intentionally excludes implementation details including:

- REST endpoints
- GraphQL schemas
- RPC protocols
- CLI commands
- SDK interfaces
- Programming language constructs
- Serialization formats

Its purpose is to define the stable service available to consumers of the assessment engine regardless of implementation technology.

---

# Relationship to Other Specifications

The Engine API exposes capabilities derived from the platform architecture.

It does not redefine:

- Runtime entities
- Engineering knowledge
- Rule execution

Instead, it provides conceptual access to those capabilities through stable service boundaries.

---

# API Design Principles

## Capability-Oriented

The Engine API exposes capabilities rather than implementation technologies.

Consumers interact with conceptual services.

Implementations remain free to expose those services through REST, SDKs, plugins, CLIs, or other interfaces.

---

## Stable

Service contracts should remain stable as implementation evolves.

Future implementation changes should not require redesign of the conceptual API.

---

## Deterministic

Service behavior should preserve deterministic platform behavior.

Identical inputs should produce identical outputs when evaluated against identical engineering knowledge.

---

## Explainable

Services should expose sufficient information to support platform explainability.

Consumers should be able to understand both assessment outcomes and the reasoning supporting them.

---

## Platform Independent

The Engine API defines capabilities independently of deployment model, execution environment, or communication protocol.

---

# Engine Orchestration Model

The Engine API exposes one real conceptual service: the Assessment Service. An earlier draft of this specification described four additional intra-engine services (Knowledge Service, Rule Evaluation Service, Reporting Service, Version Profile Service) as separate conceptual objects mediating between the Assessment Service and each subsystem. Implementation evidence gathered across three Engineering Releases showed this mediation layer was never needed: three independent subsystems — the Rule Engine, Reporting, and Evidence Collection — were each, independently, composed by the Assessment Service through direct reference to that subsystem's own real capability, with no intervening service object, and no loss of correctness, explainability, or determinism. This specification now documents that architecture. See ADR-0010 (Engine Orchestration Simplification) and Governance Register item GOV-004 for the full evaluation and decision record.

---

# Assessment Service

## Responsibility

Coordinates the lifecycle of an Assessment.

## Capabilities

- Create Assessments
- Manage Assessment Context
- Execute Assessments
- Produce Assessment Reports

## Depends On

- Runtime Domain
- Rule Engine
- Evidence Collection
- Reporting
- Version Profiles (once implemented)
- Knowledge Base (once implemented)

---

# Subsystem Orchestration

The Assessment Service orchestrates each subsystem by direct composition: it invokes that subsystem's own real capability directly, rather than through an intermediate engine-local service object. This mirrors `Architecture.md`'s own System Overview, which depicts the Assessment Service delegating directly to Evidence Collection, Rule Engine, Knowledge Base, Version Profiles, Reporting, and Storage as subsystems — the shape this section now describes as implemented, not merely diagrammed.

Concretely, today:

- **Rule Engine** is invoked directly to evaluate Evidence and produce Findings and Recommendations.
- **Reporting** is invoked directly to generate the Assessment Report.
- **Evidence Collection** is invoked directly to produce Evidence from an Assessment Input.
- **Knowledge Domain** and **Version Profiles** remain unimplemented scaffolding. When each gains real content, the Assessment Service is expected to orchestrate it the same way — by direct composition, not through a dedicated engine-local service object.

The Assessment Service owns none of the state, logic, or output it orchestrates. Each subsystem remains solely responsible for its own domain.

---

# Engine Responsibilities

The Assessment Service:

- Owns orchestration, execution flow, and the composition of subsystem capabilities into a complete Assessment.
- Owns none of the runtime state it orchestrates — the Runtime Domain's Assessment aggregate remains the sole owner and sole mutator of Assessment state.
- Implements no business rules — rule evaluation belongs exclusively to the Rule Engine.
- Generates no reports itself — report generation belongs exclusively to Reporting.
- Performs no content inspection itself — Evidence production belongs exclusively to Evidence Collection.

---

# Dependency Boundaries

The Assessment Service depends downward on each subsystem it orchestrates. No subsystem depends on the Assessment Service. Nothing depends on the Assessment Service except applications built on the platform.

This preserves the platform's existing strictly-downward dependency rule (`Architecture.md`: Dependency Rules) without introducing any new dependency edge.

---

# Relationship Diagram

This specification does not maintain its own orchestration diagram. `Architecture.md`'s System Overview diagram is the single authoritative picture of how the Assessment Service relates to each subsystem — maintaining a second, separate diagram here, describing the same relationships differently, was a contributing factor to this specification drifting out of sync with real implementation (see ADR-0010). Consult `Architecture.md` directly.

---

# Conceptual Boundaries

The Engine API does not define:

- REST resources
- HTTP methods
- GraphQL operations
- RPC interfaces
- SDK implementations
- Plugin interfaces
- Programming language bindings

These concerns belong to future implementation phases.

---

# Future Evolution

The Engine API is intentionally implementation independent.

Future deployment models including:

- Local applications
- Cloud services
- Plugins
- Distributed execution
- AI-assisted tooling

should preserve the conceptual service contract defined by this specification.

Implementation technology may evolve without changing the conceptual capabilities exposed by the platform.

---

# Document Status

**Current Version:** 1.1.0

**Status:** Frozen, amended following GOV-004 resolution

This specification establishes the authoritative conceptual service contract for the modIQ assessment engine. It was amended following Technical Director approval of `PROPOSAL_GOV-004.md`, itself based on implementation evidence gathered in `PLATFORM_VALIDATION_GOV-004.md`: the four intra-engine services originally described alongside the Assessment Service (Knowledge Service, Rule Evaluation Service, Reporting Service, Version Profile Service) are retired, and the Assessment Service's direct-composition orchestration of each subsystem — validated by three independent subsystems across three Engineering Releases — is now documented as the approved architecture. See ADR-0010 and Governance Register item GOV-004 (Resolved). This amendment is recorded explicitly, as required by `docs/governance/DocumentationRelease.md`'s "accepted Architecture Decision Record requires modification" exception to Frozen-specification stability — it is not a silent rewrite.
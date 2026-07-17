# Engine API

> **The authoritative specification defining the conceptual service contract of the modIQ assessment engine.**

---

| Property | Value |
|----------|-------|
| **Document** | EngineAPI.md |
| **Version** | 1.0.0 |
| **Status** | Frozen |
| **Project** | modIQ |
| **Documentation Release** | 1.0 |
| **Owner** | Zach Weber |
| **Created** | 2026-07-16 |
| **Last Updated** | 2026-07-16 |

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

Its purpose is to define the stable services available to consumers of the assessment engine regardless of implementation technology.

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

# Service Overview

The Engine API exposes five conceptual services.

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
- Version Profiles

---

# Knowledge Service

## Responsibility

Provides access to reusable engineering knowledge.

## Capabilities

- Provide Rules
- Provide Repair Recipes
- Provide Engineering Knowledge
- Provide Best Practices
- Provide Known Issues
- Provide Compatibility Patterns

## Depends On

- modIQ Knowledge Base

---

# Rule Evaluation Service

## Responsibility

Coordinates deterministic rule evaluation.

## Capabilities

- Select applicable Rules
- Evaluate Evidence
- Generate Findings
- Generate Recommendations

## Depends On

- Rule Engine
- Runtime Evidence
- Engineering Knowledge

---

# Reporting Service

## Responsibility

Produces explainable Assessment output.

## Capabilities

- Generate Assessment Reports
- Generate Assessment Summaries
- Organize Findings
- Organize Recommendations
- Preserve Traceability

## Depends On

- Runtime Domain
- Rule Engine

---

# Version Profile Service

## Responsibility

Provides version-specific assessment context.

## Capabilities

- Resolve Version Profiles
- Determine applicable Rules
- Determine compatibility context
- Support multiple Farming Simulator releases

## Depends On

- Version Profiles
- Knowledge Base

---

# Service Relationships

The conceptual interaction between services is illustrated below.

```text
Assessment Service
        │
        ▼
Rule Evaluation Service
        │
        ├──────────────► Knowledge Service
        │
        ▼
Reporting Service
        ▲
        │
Version Profile Service
```

Each service provides a distinct conceptual responsibility.

Services cooperate through well-defined responsibilities rather than shared ownership.

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

**Current Version:** 1.0.0

**Status:** Frozen

This specification establishes the authoritative conceptual service contract for the modIQ assessment engine.
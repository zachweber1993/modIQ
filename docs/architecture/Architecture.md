# Architecture

> **The authoritative specification defining the conceptual organization of the modIQ platform.**

---

| Property | Value |
|----------|-------|
| **Document** | Architecture.md |
| **Version** | 1.0.0 |
| **Status** | Frozen |
| **Project** | modIQ |
| **Documentation Release** | 1.0 |
| **Owner** | Zach Weber |
| **Created** | 2026-07-15 |
| **Last Updated** | 2026-07-15 |

---

# Purpose

This document defines the high-level architecture of the modIQ platform.

It describes the major platform subsystems, their responsibilities, and the relationships between them.

This document intentionally avoids implementation details. It defines *how the platform is organized*, not *how individual components are implemented*.

---

# Specification Authority

**Authority**

- Vision.md
- Principles.md
- Glossary.md
- ProductSpecification.md

**Governed Specifications**

- DataModel.md
- KnowledgeModel.md
- RuleEngine.md
- EngineAPI.md
- Sprint0.md
- Sprint1.md

If a conflict exists between this document and a higher-level specification, the higher-level specification takes precedence.

---

# Architectural Goals

The architecture exists to support the long-term vision of modIQ as an explainable, evidence-based assessment platform.

The architecture should:

- support deterministic assessments
- preserve explainability
- separate knowledge from execution
- isolate game-specific behavior
- enable incremental evolution
- remain platform-first
- encourage modular development
- support future extensibility

---

# Architectural Principles

The platform architecture follows several guiding principles.

## Separation of Responsibilities

Each subsystem has a single, clearly defined responsibility.

Subsystems cooperate through well-defined interfaces rather than shared implementation.

## Domain Separation

The platform distinguishes between runtime assessment data and reusable engineering knowledge.

Runtime entities exist only within the context of an Assessment and are defined by the Data Model.

Reusable engineering knowledge exists independently of any individual Assessment and is defined by the Knowledge Model.

This separation allows assessment execution and accumulated engineering knowledge to evolve independently while maintaining clear architectural boundaries.

---

## Modular Composition

Capabilities should be composed from independent systems rather than tightly coupled implementations.

Individual components should evolve independently whenever practical.

---

## Explainable by Design

The architecture should preserve the ability to explain every assessment.

Explainability is considered a system capability rather than a reporting feature.

---

## Knowledge-Driven

Engineering knowledge should exist independently from assessment execution.

The platform should separate:

- knowledge
- reasoning
- reporting

Each evolves independently.

---

## Version Isolation

Game-specific behavior should remain isolated within Version Profiles.

Support for future Farming Simulator releases should not require architectural redesign.

---

# System Overview

modIQ is organized as a collection of cooperating platform services centered around the Assessment Service.

The Assessment Service coordinates the complete assessment lifecycle while delegating specialized responsibilities to supporting subsystems.

```
                        User
                          │
                          ▼
                  Assessment Service
                          │
     ┌───────────┬────────┼────────┬────────────┐
     ▼           ▼        ▼        ▼            ▼
Rule Engine   Version   Knowledge  Reporting  Storage
              Profiles    Base
                 │
                 ▼
          Extension Layer
```

The Assessment Service serves as the primary orchestration layer for the platform.

---

# Core Platform Components

## Assessment Service

The Assessment Service is the central orchestration subsystem.

Responsibilities include:

- creating Assessments
- managing Assessment Context
- executing Assessments
- producing Assessment Reports

The Assessment Service owns the assessment lifecycle but does not perform technical analysis directly.

The modIQ Assessment Framework (MAF) is not a subsystem. It is the assessment methodology — the standards, categories, rules, scoring methodology, and evidence requirements — that the Assessment Service applies during execution. See Glossary.md.

---

## Rule Engine

The Rule Engine performs deterministic assessment logic.

Responsibilities include:

- executing Rules
- evaluating Evidence
- producing Findings
- determining assessment outcomes

The Rule Engine consumes knowledge but does not own it.

---

## modIQ Knowledge Base (MKB)

The Knowledge Base represents accumulated engineering knowledge.

Responsibilities include:

- engine behaviors
- best practices
- validated community knowledge
- Repair Recipes
- supporting references
- version-aware guidance

The Knowledge Base provides information to the Rule Engine while remaining independent of assessment execution.

---

## Version Profiles

Version Profiles isolate game-specific behavior.

Responsibilities include:

- game version definitions
- supported features
- behavioral differences
- engine capabilities
- compatibility metadata

Version Profiles enable the platform to support multiple Farming Simulator releases without architectural changes.

---

## Reporting System

The Reporting System transforms assessment outputs into educational reports.

Responsibilities include:

- organizing Findings
- presenting Evidence
- generating explanations
- producing Assessment Reports

The Reporting System performs no analysis.

Its responsibility is presentation.

---

## Storage Layer

The Storage Layer provides persistence services.

Responsibilities include:

- assessment persistence
- knowledge storage
- configuration
- historical records
- cached resources

The architecture intentionally abstracts storage implementation.

---

## Extension Layer

The Extension Layer provides controlled extensibility.

Potential capabilities include:

- custom Rules
- plugins
- external integrations
- import/export
- future AI-assisted capabilities

Extensions should integrate through stable platform interfaces rather than modifying core architecture.

---

# Assessment Lifecycle

Every Assessment follows the same conceptual lifecycle.

```
Assessment Request

        │

        ▼

Assessment Service

        │

        ▼

Version Profile Selection

        │

        ▼

Rule Engine

        │

        ▼

Knowledge Base

        │

        ▼

Evidence Collection

        │

        ▼

Finding Generation

        │

        ▼

Assessment Report
```

Each stage has a clearly defined responsibility.

---

# Information Flow

Information moves through the platform in a single direction.

```
Input

↓

Evidence

↓

Rule Evaluation

↓

Findings

↓

Assessment Report

↓

User Understanding
```

This progression reflects the educational goals established by the Vision.

---

# Dependency Rules

Platform dependencies should remain directional.

Subsystems may depend upon lower-level services but should avoid circular dependencies.

General dependency flow:

```
Assessment Service

↓

Rule Engine

↓

Knowledge Base

↓

Version Profiles

↓

Storage
```

Presentation systems should consume assessment outputs without influencing assessment logic.

---

# Platform Boundaries

The architecture intentionally separates:

Assessment execution

Engineering knowledge

Version-specific behavior

Reporting

Persistence

User interfaces

Future implementations should preserve these boundaries whenever practical.

---

# Extensibility

The platform should evolve through extension rather than modification.

New capabilities should integrate by:

- introducing additional Rules
- extending the Knowledge Base
- adding Version Profiles
- implementing Extension interfaces

Core architectural responsibilities should remain stable.

---

# Architectural Constraints

The architecture intentionally excludes:

- automatic mod repair
- opaque scoring systems
- hidden assessment logic
- runtime memory analysis
- game modification
- implementation-specific technologies

These decisions preserve explainability and long-term maintainability.

---

# Future Evolution

Future platform capabilities may include:

- additional Farming Simulator releases
- expanded assessment domains
- collaborative knowledge contributions
- enhanced reporting
- AI-assisted learning
- third-party integrations

Future capabilities should integrate without requiring architectural redesign.

---

# Relationship to Other Specifications

This document defines platform organization.

Subsequent specifications refine this architecture.

- DataModel.md defines platform information.
- KnowledgeModel.md defines engineering knowledge representation.
- RuleEngine.md defines assessment execution.
- EngineAPI.md defines subsystem interfaces.
- Sprint planning documents define implementation sequencing.

---

# Document Status

**Current Version:** 1.0.0

**Status:** Frozen

This document is a foundational technical specification, frozen for Documentation Release 1.0 per ADR-0001 (Foundation Freeze).

Changes should preserve consistency with higher-level specifications and maintain stable subsystem boundaries.
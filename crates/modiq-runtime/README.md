# modiq-runtime

Runtime domain model for the modIQ platform.

---

## Purpose

`modiq-runtime` defines the conceptual runtime entities that exist during the execution of an Assessment.

These entities represent the state generated while evaluating an Assessment Subject and are the authoritative implementation of the runtime domain described in `DataModel.md`.

The runtime domain is centered around the **Assessment** aggregate.

---

## Responsibilities

This crate owns the runtime domain model, including:

- Assessment
- Assessment Subject
- Assessment Context
- Evidence
- Findings
- Recommendations
- Assessment Report

All runtime entities exist within the lifecycle of a single Assessment.

---

## Non-Responsibilities

This crate does **not** contain:

- Engineering knowledge
- Rule implementations
- Rule execution
- Version Profile definitions
- Reporting implementations
- CLI functionality
- Persistence
- Serialization
- API contracts

These concerns belong to their respective crates.

---

## Aggregate Ownership

`Assessment` is the aggregate root of the runtime domain.

Supporting runtime entities do not exist independently of an Assessment.

Ownership follows the hierarchy defined in `DataModel.md`.

---

## Design Principles

The runtime model is intentionally:

- Assessment-centric
- Deterministic
- Immutable after completion
- Independent of engineering knowledge
- Independent of persistence technologies
- Independent of presentation concerns

The runtime model should remain a direct implementation of the Engineering Specification.

---

## Dependencies

`modiq-runtime` should depend only on foundational platform crates.

It must not depend on:

- modiq-engine
- modiq-cli
- modiq-report

Future dependencies on `modiq-versioning` should occur only through well-defined public interfaces.

---

## Engineering Notes

The runtime domain should evolve only when the Engineering Specification evolves.

Implementation must preserve the conceptual model defined in `DataModel.md`.

Avoid introducing behavior that belongs to the Rule Engine, Knowledge Model, or Engine orchestration.

When in doubt:

> The runtime domain represents **what exists during an Assessment**, not **how an Assessment is performed**.
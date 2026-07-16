# modiq-engine

Assessment engine orchestration services for the modIQ platform.

---

## Purpose

`modiq-engine` exposes the conceptual services that coordinate deterministic engineering assessment.

Unlike the runtime, knowledge, versioning, and rule crates, this crate does not define domain concepts. Instead, it composes those domains into stable assessment capabilities.

This crate implements the conceptual service model defined in `EngineAPI.md`.

---

## Responsibilities

This crate owns the conceptual engine services, including:

- Assessment Service
- Knowledge Service
- Rule Evaluation Service
- Reporting Service
- Version Profile Service

These services expose the capabilities of the assessment platform while remaining independent of implementation technologies.

---

## Non-Responsibilities

This crate does **not** own:

- Runtime entities
- Engineering knowledge
- Rule implementations
- Version Profile definitions
- CLI functionality
- Persistence
- Serialization
- Network protocols

These concerns belong to their respective crates.

---

## Design Principles

The engine is intentionally:

- Capability-oriented
- Deterministic
- Explainable
- Platform-independent
- Stable

The engine coordinates platform components but does not redefine their responsibilities.

---

## Dependencies

`modiq-engine` composes services from:

- modiq-runtime
- modiq-knowledge
- modiq-versioning
- modiq-rules

It should remain independent of presentation concerns such as:

- modiq-report
- modiq-cli

---

## Engineering Notes

The engine is the orchestration layer of the modIQ platform.

It coordinates deterministic assessment by composing runtime state, engineering knowledge, version compatibility, and rule evaluation into stable assessment capabilities.

When in doubt:

> The engine coordinates platform services; it does not redefine platform domains.
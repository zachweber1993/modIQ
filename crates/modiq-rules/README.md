# modiq-rules

Deterministic rule evaluation engine for the modIQ platform.

---

## Purpose

`modiq-rules` defines the conceptual execution model that transforms runtime evidence and reusable engineering knowledge into deterministic assessment outcomes.

The Rule Engine operationalizes engineering knowledge while preserving determinism, explainability, and complete traceability.

This crate implements the conceptual Rule Engine defined in `RuleEngine.md`.

---

## Responsibilities

This crate is responsible for:

- Rule selection
- Evidence evaluation
- Finding generation
- Recommendation generation
- Traceability
- Explainability

The Rule Engine coordinates deterministic assessment but does not own the runtime domain or the engineering knowledge domain.

---

## Non-Responsibilities

This crate does **not** contain:

- Runtime assessment entities
- Engineering knowledge
- Version Profile definitions
- Assessment orchestration
- Report generation
- CLI functionality
- Persistence
- Serialization

These concerns belong to their respective crates.

---

## Design Principles

The Rule Engine is intentionally:

- Deterministic
- Evidence-based
- Knowledge-driven
- Explainable
- Traceable
- Version-aware

Given identical runtime inputs, engineering knowledge, and Version Profile, the Rule Engine should always produce identical assessment outcomes.

---

## Dependencies

`modiq-rules` consumes concepts from:

- modiq-runtime
- modiq-knowledge
- modiq-versioning

It should remain independent of:

- modiq-engine
- modiq-report
- modiq-cli

The Rule Engine coordinates these domains but does not own them.

---

## Engineering Notes

The Rule Engine represents **how** deterministic assessment is performed.

It consumes runtime Evidence, engineering Knowledge, and Version Profile context to produce Findings and Recommendations.

The Rule Engine should never introduce engineering knowledge, modify runtime state, or generate opaque conclusions.

Every assessment outcome should remain deterministic, explainable, and traceable.

When in doubt:

> The Rule Engine operationalizes engineering knowledge—it does not create it.
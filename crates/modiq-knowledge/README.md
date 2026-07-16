# modiq-knowledge

Engineering knowledge domain for the modIQ platform.

---

## Purpose

`modiq-knowledge` defines the reusable engineering knowledge that powers deterministic assessment within the modIQ platform.

Unlike the runtime domain, engineering knowledge exists independently of any individual Assessment and represents the long-term intellectual foundation of the platform.

This crate provides the conceptual implementation of the Knowledge Model defined in `KnowledgeModel.md`.

---

## Responsibilities

This crate owns reusable engineering knowledge, including:

- Rules
- Repair Recipes
- Engine Behaviors
- Compatibility Patterns
- Best Practices
- Known Issues
- Knowledge References

These entities describe engineering understanding rather than runtime execution.

---

## Non-Responsibilities

This crate does **not** contain:

- Runtime assessment state
- Assessment execution
- Rule evaluation
- Version Profile definitions
- Report generation
- CLI functionality
- Persistence
- Serialization
- Engine orchestration

These concerns belong to their respective crates.

---

## Knowledge Principles

The knowledge domain is intentionally:

- Independent of runtime execution
- Deterministic
- Explainable
- Version-aware
- Extensible
- Focused on long-term preservation of engineering knowledge

Knowledge should remain reusable across multiple Assessments and supported Farming Simulator versions.

---

## Dependencies

`modiq-knowledge` should depend only on foundational platform crates.

It must not depend on:

- modiq-runtime
- modiq-rules
- modiq-engine
- modiq-report
- modiq-cli

Future integration with `modiq-versioning` should occur only through well-defined public interfaces.

---

## Engineering Notes

Engineering knowledge represents **what the platform knows**, not **how the platform executes**.

The runtime domain consumes knowledge to produce Findings and Recommendations, but it never owns or modifies the knowledge itself.

The implementation of this crate should remain a direct translation of `KnowledgeModel.md`.

Avoid introducing runtime behavior, assessment logic, or executable rule evaluation into this crate.

When in doubt:

> The knowledge domain preserves engineering understanding; it does not perform engineering assessment.
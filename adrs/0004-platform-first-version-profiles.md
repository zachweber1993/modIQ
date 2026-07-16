# ADR-0004: Platform-First Version Profiles

| Property | Value |
|----------|-------|
| **ADR** | 0004 |
| **Title** | Platform-First Version Profiles |
| **Status** | Accepted |
| **Project** | modIQ |
| **Date** | 2026-07-16 |

---

# Context

modIQ is intended to support Farming Simulator mod assessment over multiple game releases.

Although Farming Simulator 25 is the initial implementation target, future versions of the Farming Simulator platform will introduce new engine behavior, file formats, validation requirements, and compatibility considerations.

Designing the architecture around a single game version would tightly couple the platform to FS25, making future support for additional releases require significant architectural changes.

A mechanism was required to isolate version-specific behavior while preserving a stable assessment architecture.

---

# Decision

The platform shall adopt a **platform-first architecture** centered around **Version Profiles**.

A Version Profile encapsulates all version-specific assessment knowledge required to evaluate a mod for a particular Farming Simulator release.

Version Profiles determine:

- Applicable Rules
- Compatibility constraints
- Validation behavior
- Supported file formats
- Version-specific engineering knowledge

The core assessment architecture shall remain independent of any individual Farming Simulator release.

Support for future platform versions shall be introduced by creating or extending Version Profiles rather than redesigning the platform architecture.

---

# Consequences

Benefits include:

- Separation of platform architecture from version-specific behavior.
- Simplified support for future Farming Simulator releases.
- Reduced architectural change when new game versions are introduced.
- Reuse of the Assessment Framework across multiple platform versions.
- Long-term maintainability through isolated version-specific knowledge.

This decision allows the platform to evolve as Farming Simulator changes while preserving a stable architectural foundation.

---

# Relationship to Other Specifications

This decision is reflected in:

- Vision.md
- ProductSpecification.md
- Architecture.md
- KnowledgeModel.md
- EngineAPI.md

Future platform support should extend Version Profiles rather than introducing game-specific architectural branches.

---

# Status

Accepted.

This decision is authoritative for Documentation Release 1.0.
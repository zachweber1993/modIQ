# modiq-common

Shared platform types used across multiple modIQ crates.

---

## Purpose

`modiq-common` provides foundational types that are genuinely shared across the platform and are not owned by any specific domain.

This crate exists to prevent duplication while preserving clear architectural boundaries.

It intentionally contains **no business logic**.

---

## Responsibilities

Examples of appropriate contents include:

- Shared error types
- Common identifier types
- Shared traits
- Cross-platform utility types

Only types required by multiple crates should be added.

---

## Non-Responsibilities

This crate does **not** contain:

- Assessment runtime models
- Engineering knowledge
- Rule evaluation
- Version Profiles
- Reporting
- Engine orchestration
- CLI functionality

These concerns belong to their respective crates.

---

## Design Principles

- Keep the crate intentionally small.
- Avoid becoming a general utility or "miscellaneous" library.
- Every addition must have clear architectural justification.
- Ownership should remain with the domain whenever possible.

If a type naturally belongs to another crate, it should remain there.

---

## Dependencies

`modiq-common` should have minimal external dependencies.

Other platform crates may depend on `modiq-common`, but `modiq-common` should avoid depending on higher-level platform crates.

---

## Engineering Note

As a general rule:

> If a type can live in its owning domain, it should.

Only promote a type into `modiq-common` when it is demonstrably shared across multiple domains and has no single owner.

Maintaining a small and stable `modiq-common` crate helps preserve the long-term architectural integrity of the modIQ platform.
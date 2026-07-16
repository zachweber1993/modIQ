# modiq-cli

Command-line application for the modIQ platform.

---

## Purpose

`modiq-cli` provides the primary command-line interface for interacting with the modIQ assessment engine.

The CLI is responsible for translating user input into engine requests and presenting assessment results.

It does not implement assessment logic.

---

## Responsibilities

This crate owns:

- Command parsing
- Application startup
- User interaction
- Console output
- Engine invocation

The CLI serves as the user-facing application layer of the platform.

---

## Non-Responsibilities

This crate does **not** contain:

- Runtime domain models
- Engineering knowledge
- Rule evaluation
- Version Profile resolution
- Assessment orchestration
- Report generation

These responsibilities belong to their respective crates.

---

## Design Principles

The CLI is intentionally:

- Thin
- Platform-independent
- User-focused
- Deterministic
- Presentation-oriented

The CLI should delegate assessment work to the engine rather than implementing business logic directly.

---

## Dependencies

The CLI should depend only on the public interfaces exposed by the platform.

Business logic should remain outside the application layer.

---

## Engineering Notes

The CLI is the entry point for users, not the owner of platform behavior.

When in doubt:

> Parse input. Invoke the engine. Display the results.
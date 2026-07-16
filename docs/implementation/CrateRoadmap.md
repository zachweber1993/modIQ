# Crate Roadmap

> **The authoritative implementation roadmap for all modIQ platform crates.**

---

| Property | Value |
|----------|-------|
| **Document** | CrateRoadmap.md |
| **Version** | 1.0.0 |
| **Status** | Active |
| **Project** | modIQ |
| **Owner** | Zach Weber |
| **Maintained By** | Technical Director |

---

# Purpose

This document tracks the engineering maturity of every crate within the modIQ platform.

It provides implementation visibility while preserving the architectural boundaries established by the Engineering Specification.

Each crate progresses independently through defined implementation stages while respecting platform dependencies.

---

# Crate Dependency Hierarchy

Implementation proceeds from the lowest-level platform crates upward.

```text
                 modiq-cli
                      │
                      ▼
                modiq-engine
          ┌─────────┼──────────┬──────────┐
          ▼         ▼          ▼          ▼
   modiq-runtime  modiq-knowledge  modiq-rules  modiq-versioning
          │                                 │
          └───────────────┬─────────────────┘
                          ▼
                    modiq-common
```

No crate should depend on a crate above it in this hierarchy.

---

# Crate Maturity Levels

| Level | Description |
|--------|-------------|
| L0 | Crate created |
| L1 | Architecture translated into module structure |
| L2 | Domain model implemented |
| L3 | Business logic implemented |
| L4 | Fully tested |
| L5 | Stable |

---

# Implementation Status

| Crate | Responsibility | Current Level | Status |
|--------|----------------|--------------|--------|
| modiq-common | Shared platform types | L1 | 🟡 Scaffolded |
| modiq-runtime | Runtime domain | L1 | 🟡 Scaffolded |
| modiq-knowledge | Engineering knowledge | L0 | ⚪ Not Started |
| modiq-versioning | Version Profiles | L0 | ⚪ Not Started |
| modiq-rules | Deterministic rule evaluation | L0 | ⚪ Not Started |
| modiq-engine | Assessment orchestration | L0 | ⚪ Not Started |
| modiq-report | Report generation | L0 | ⚪ Not Started |
| modiq-cli | Command line interface | L0 | ⚪ Not Started |

---

# Crate Completion Workflow

Every crate progresses through the same engineering lifecycle.

## Phase 0 — Scaffold

- Crate created
- Cargo.toml configured
- README.md created
- Module structure established

---

## Phase 1 — Architecture

- Engineering Specification reviewed
- Responsibilities confirmed
- Architectural boundaries verified
- Public module structure defined

---

## Phase 2 — Domain

- Domain entities implemented
- Traits established
- Public interfaces reviewed
- Documentation updated

---

## Phase 3 — Implementation

- Business logic implemented
- Error handling completed
- Integration completed

---

## Phase 4 — Validation

- Unit tests
- Integration tests
- Documentation review
- Technical review

---

## Phase 5 — Stable

The crate is considered complete for the current release.

Future enhancements should occur through new implementation work rather than architectural redesign.

---

# Engineering Rules

A crate may only begin implementation when all lower-level dependencies required by that crate have reached an appropriate maturity level.

Every implementation decision must remain traceable to the Engineering Specification.

No implementation may redefine architectural ownership.

Architectural conflicts must be resolved through the ADR process before implementation continues.

---

# Current Sprint

Sprint 0

Current engineering focus:

1. modiq-common ✅
2. modiq-runtime ✅
3. modiq-knowledge
4. modiq-versioning
5. modiq-rules
6. modiq-engine
7. modiq-report
8. modiq-cli

---

# Exit Criteria

Sprint 0 is complete when:

- All crates have been scaffolded.
- Module boundaries reflect the Engineering Specification.
- Workspace builds successfully.
- Tests execute successfully.
- CI passes.
- No production assessment functionality has been implemented beyond architectural scaffolding.

---

# Revision History

| Version | Date | Description |
|----------|------|-------------|
| 1.0.0 | 2026-07-16 | Initial crate implementation roadmap established. |
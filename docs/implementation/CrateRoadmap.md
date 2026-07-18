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
          ┌─────────┼──────────┬──────────┬──────────┐
          ▼         ▼          ▼          ▼          ▼
   modiq-runtime  modiq-knowledge  modiq-rules  modiq-versioning  modiq-report
          │                                 │
          └───────────────┬─────────────────┘
                          ▼
                    modiq-common
```

Additional direct dependencies:

- modiq-rules depends on modiq-runtime (consumes Evidence, Finding, Recommendation)
- modiq-report depends on modiq-runtime (consumes Assessment, Evidence, Finding, Recommendation)

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
| modiq-common | Shared platform types | L1 | ✅ Scaffolded |
| modiq-runtime | Runtime domain | L3 | 🔧 In Progress |
| modiq-knowledge | Engineering knowledge | L1 | ✅ Scaffolded |
| modiq-versioning | Version Profile management | L1 | ✅ Scaffolded |
| modiq-rules | Deterministic rule evaluation | L3 | 🔧 In Progress |
| modiq-engine | Assessment orchestration | L3 | 🔧 In Progress |
| modiq-report | Report representation and formatting | L3 | 🔧 In Progress |
| modiq-cli | Command-line interface | L1 | ✅ Scaffolded |

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

## Sprint 1

Sprint 0 has successfully completed.

The engineering foundation for the modIQ platform has been established.

Completed objectives include:

- Workspace scaffolding
- Repository organization
- Crate hierarchy
- Module hierarchy
- Documentation hierarchy
- Governance documentation
- Dependency map
- Clean workspace verification
- Engineering audit

Current engineering focus has shifted from architectural scaffolding to deterministic implementation.

The Assessment lifecycle, deterministic Rule evaluation, and Assessment Report generation have been implemented across `modiq-runtime`, `modiq-rules`, and `modiq-report` (see Implementation Status above). These are composed into one executable pipeline by `AssessmentService` in `modiq-engine`, demonstrated end-to-end by an integration test in `modiq-engine`.

---

# Exit Criteria

Sprint 1 will be considered complete when:

- Assessment lifecycle implemented
- Runtime domain operational
- Rule Engine executes deterministic evaluation
- Findings generated from objective evidence
- Recommendations generated from findings
- Assessment reports produced through the Engine API
- Unit tests passing
- Integration tests passing
- Workspace builds without warnings

---

# Revision History

| Version | Date | Description |
|----------|------|-------------|
| 1.0.0 | 2026-07-16 | Initial crate implementation roadmap established. |
| 1.1.0 | 2026-07-16 | Sprint 0 completed. All workspace crates scaffolded and engineering foundation established. |
| 1.2.0 | 2026-07-16 | modiq-runtime advanced to L3. Assessment aggregate, lifecycle transitions, and AssessmentError implemented. |
| 1.3.0 | 2026-07-18 | modiq-rules and modiq-report advanced to L3. Deterministic Rule evaluation, Assessment Report generation, and Recommendation ownership in modiq-runtime implemented; end-to-end pipeline demonstrated by integration test. |
| 1.4.0 | 2026-07-18 | modiq-engine advanced to L3. AssessmentService orchestrates the full pipeline (Runtime Domain, Rule Engine, Reporting); end-to-end integration test moved from modiq-report to modiq-engine to exercise the orchestration layer directly. |
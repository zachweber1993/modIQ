# modIQ Design Specification

Welcome to the official design specification for modIQ.

## Reading Order

1. Vision.md
2. Principles.md
3. Glossary.md
4. ProductSpecification.md
5. Architecture.md
6. DataModel.md
7. KnowledgeModel.md
8. VersionProfile.md
9. RuleEngine.md
10. EvidenceCollection.md
11. EngineAPI.md
12. Sprint0.md
13. Sprint1.md

## Purpose

These documents define the product before implementation begins.

All contributors should read them in order before making architectural or implementation decisions.

Approved RFCs supplement these documents.

## Specification Hierarchy

The documentation is hierarchical.

Each specification derives its authority from the documents that precede it in the Reading Order.

Higher-level specifications define architectural intent.

Lower-level specifications implement that intent without redefining it.

If a conflict exists between specifications, the higher-level specification takes precedence.

## Current Documentation Status

The constitutional specifications, platform architecture, technical specifications, and implementation planning documents have completed the Documentation Release 1.0 Final Review and are considered **Frozen**.

The review verified:

- Architectural consistency
- Cross-specification reconciliation
- Terminology consistency
- Documentation quality
- Implementation readiness

The review concluded with:

- Documentation Release 1.0 frozen
- Git tag `v0.1.0`
- Transition from documentation to implementation

Documentation Release 2.0 (governance and terminology reconciliation) and Documentation Release 2.1 (Evidence Collection subsystem boundary — new specification `EvidenceCollection.md`) have since been completed. The last frozen Engineering Release remains 0.5 (Sprint 5 complete, Phases 1–5, plus Closeout); Sprints 6 (CLI wiring, `modiq-report` scaffold retirement) and 7 (Multi-Source Evidence Collection: XML inspection) are both complete, implemented, committed, and pushed, but formal Engineering Release 0.6/0.7 records have not yet been produced — see `docs/engineering/SPRINT7_IMPLEMENTATION_REPORT.md`, `docs/engineering/REPOSITORY_CLOSEOUT_REPORT.md`, and `docs/governance/PROJECT_STATUS.md` for current status. The engineering methodology itself is now designated Version 1.0 — see `docs/engineering/PROJECT_HANDOFF_v1.0.md`, Section 5.
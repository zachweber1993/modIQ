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

Documentation Release 2.0 (governance and terminology reconciliation) and Documentation Release 2.1 (Evidence Collection subsystem boundary — new specification `EvidenceCollection.md`) have since been completed; Documentation Release 2.1 remains current — Sprint 8 required no Documentation Release amendment. The current Engineering Release is **0.8** (Sprint 8: Version Profile-aware compatibility checking), produced at its own Repository Closeout. Engineering Releases 0.6 (Sprint 6: CLI wiring, `modiq-report` scaffold retirement) and 0.7 (Sprint 7: Multi-Source Evidence Collection) were both produced retroactively, after a two-Sprint documentation gap (`docs/engineering/HISTORICAL_RELEASE_COMPLETION_REPORT.md`); Sprint 8 corrected that pattern rather than repeating it a third time, per `ENGINEERING_RELEASE_0.7.md`'s own Lessons Learned. See `docs/engineering/ENGINEERING_RELEASE_0.8.md` and `docs/governance/PROJECT_STATUS.md` for current status. The engineering methodology itself is designated Version 1.0, unchanged since Sprint 7 — see `docs/engineering/PROJECT_HANDOFF_v1.0.md`, Section 5.
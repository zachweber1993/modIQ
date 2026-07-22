# modIQ Project Handoff v1.0

> **The authoritative, role-independent onboarding document for the modIQ repository.**

---

| Property | Value |
|----------|-------|
| **Document** | PROJECT_HANDOFF_v1.0.md |
| **Project** | modIQ |
| **Purpose** | Canonical project handoff — describes the platform, its architecture, its governance, and its history independently of any individual role |
| **Audience** | Anyone onboarding to modIQ — engineering, architecture, or product — before reading a role-specific handoff |
| **Supersedes** | Nothing directly. `LEAD_ENGINEER_HANDOFF_v*.md` and `CHIEF_ARCHITECT_HANDOFF_v1.0.md` (the current role-specific supplement for that role; `TECHNICAL_DIRECTOR_HANDOFF_v2.0.md` through `v2.2.md` remain on disk as superseded historical record under the role's prior title) remain role-specific supplements; from this version forward, both assume the reader has already read this document and do not repeat what it covers. |
| **As of** | 2026-07-21, following Sprint 6 (CLI wiring, `modiq-report` scaffold retirement), implemented, reviewed, and merged; Repository Closeout in progress |
| **Branch** | `feature/runtime-implementation` |
| **HEAD** | `29657df` — "Merge feature/sprint6-cli: Sprint 6 (CLI wiring, modiq-report scaffold retirement)" |

---

# How to Use This Document

This is the first document anyone — a new engineer, a new Chief Architect session, a reviewer, a future contributor — should read about modIQ. It does not belong to Engineering or to the Chief Architect; it describes the project itself: why it exists, how it is organized, how decisions get made, what has been built, and what is still open.

After this document, read the role-specific supplement for your role (`LEAD_ENGINEER_HANDOFF_v3.0.md` or `CHIEF_ARCHITECT_HANDOFF_v1.0.md`). Those documents assume you have read this one and will not repeat its content.

Everything in this document is verified directly against the repository as of the commit above, not carried over from memory or from prior handoff documents.

---

# 1. Product Vision

modIQ exists to make every Farming Simulator mod understandable. Every assessment should answer three questions: *does this mod work? why? what can I do next?* The platform is an evidence-based assessment tool, not a scoring tool — its purpose is to explain, not merely to flag.

**Core beliefs** (`Vision.md`): understanding creates confidence; evidence is more valuable than opinion; transparent systems create stronger communities than opaque automation; every technical conclusion should be explainable; software should educate its users rather than simply produce results; preserving knowledge is as important as generating it.

**Target users** (`ProductSpecification.md`): players (want confidence before installing/troubleshooting), mod creators (want objective engineering feedback), server administrators (want to validate mod collections), and community contributors (future — contribute validated knowledge back).

**Non-goals, explicitly stated:** modIQ does not automatically rewrite mods, replace creators, replace the Farming Simulator editor, replace community discussion, make subjective quality judgments without evidence, or hide complexity behind opaque scoring. It explains, educates, and preserves knowledge — it does not decide for the user.

**Decision priority order** (`Principles.md`, when approaches conflict): Correctness → Explainability → Evidence → Determinism → User Understanding → Extensibility → Performance → Convenience. This ordering has been invoked, implicitly or explicitly, in essentially every architectural decision recorded in this repository's history — Determinism Before Intelligence and Evidence Over Assumption in particular show up by name repeatedly.

The initial implementation targets Farming Simulator 25, with the architecture deliberately designed to support future FS releases through Version Profiles rather than redesign (`Principles.md`: Platform Independence; ADR-0004).

---

# 2. Repository Organization

```
docs/
├── constitutional/     Vision, Principles, Glossary, ProductSpecification — Frozen, Documentation Release 1.0
├── architecture/       Architecture, DataModel, KnowledgeModel, RuleEngine, EvidenceCollection, EngineAPI, VersionProfile
├── adrs/                Architecture Decision Records, 0001–0010, sequentially numbered, never reused
├── governance/          GOVERNANCE.md's home is actually docs/engineering/ (see note below); PROJECT_STATUS.md, CHANGELOG.md, ROADMAP.md, EngineeringGuide.md, DocumentationRelease.md
├── engineering/         GOVERNANCE.md, ENGINEERING_LOG.md, Sprint plans, Engineering Release records, Proposals, Platform Validation records
├── implementation/      RuntimeInvariants.md, CrateRoadmap.md, AssessmentCreation.md
└── releases/            ENGINEERING_RELEASE_0.2.md only — a pre-existing, minor location inconsistency; every later Engineering Release lives in docs/engineering/

crates/                  Cargo workspace, 9 crates (below)
apps/sandbox/             Tauri desktop application — the only real end-to-end consumer of the platform, its own separate Cargo workspace and `Cargo.lock`
```

**A genuine location quirk, not a defect:** `GOVERNANCE.md` — despite being one of the most-referenced documents in this repository — lives at `docs/engineering/GOVERNANCE.md`, not under `docs/governance/`. `docs/governance/` instead holds the living status documents (`PROJECT_STATUS.md`, `CHANGELOG.md`, `ROADMAP.md`, `EngineeringGuide.md`, `DocumentationRelease.md`). This has been true since early in the project and every cross-reference in the repository already accounts for it; noted here only so a new reader isn't surprised.

## The Nine Workspace Crates

| Crate | Responsibility | Maturity | Depends On |
|---|---|---|---|
| `modiq-runtime` | Runtime Domain — Assessment aggregate, Evidence, Finding, Recommendation, lifecycle | L3, 82 tests | (leaf — no workspace dependencies) |
| `modiq-collection` | Evidence Collection — produces Evidence from real content | L2, 43 tests | `modiq-runtime` |
| `modiq-rules` | Rule Engine — deterministic Rule evaluation | L3, 15 tests | `modiq-runtime` |
| `modiq-report` | Reporting — `AssessmentReport` snapshot generation | L3, 3 tests | `modiq-runtime` |
| `modiq-engine` | Engine — `AssessmentService`, the sole orchestration boundary | L3, 16 unit + 3 integration tests | `modiq-runtime`, `modiq-rules`, `modiq-report`, `modiq-collection` |
| `modiq-knowledge` | Knowledge Domain — reusable engineering knowledge (Rules, Repair Recipes, Best Practices, etc.) | L1, pure scaffold, zero implementation since Sprint 0 | `modiq-runtime` |
| `modiq-versioning` | Version Profiles — game-version compatibility context | L1, pure scaffold, zero implementation since Sprint 0 | `modiq-runtime` |
| `modiq-cli` | CLI entry point | L2, 10 tests — wired to `modiq-engine` (Sprint 6): `Application` dispatches `assess`/`help`/`version`, `AssessCommand` calls `AssessmentService::execute_from_assessment_input` against a user-supplied path | `modiq-runtime`, `modiq-engine`, `modiq-report` |
| `modiq-common` | Shared platform types | L1, empty stub files, zero evidence it's needed after 6 Sprints | (none) |

Dependency direction is strictly downward; `modiq-runtime` is the leaf every other crate ultimately depends on. No circular dependency has ever existed. `apps/sandbox` and `modiq-cli` both depend on `modiq-engine` (and, transitively, everything below it) for orchestration, preserving `AssessmentService` as the sole orchestration entry point — but both also depend on `modiq-report` directly, since `modiq-engine` does not re-export `AssessmentReport`, the type both `execute` and `execute_from_assessment_input` return. This corrects a previously-stated claim in this document ("apps/sandbox... never depends on... modiq-report directly") that predates Sprint 6 and was already inaccurate; Sprint 6 surfaced it by producing a second, independent consumer with the identical dependency, not by introducing the dependency itself. Neither crate depends on `modiq-collection` or `modiq-rules` directly.

**Root workspace: 172 tests, zero ignored, zero flaky.** **Sandbox: 6 tests**, its own separate workspace, independently verified.

---

# 3. Architecture

## System Overview (`Architecture.md`)

modIQ is organized as cooperating platform subsystems centered on the **Assessment Service**, which orchestrates the complete Assessment lifecycle by direct composition of each subsystem — no intra-engine service indirection layer (see ADR-0010, and Section 6 below).

```
                                    User
                                      │
                                      ▼
                              Assessment Service
                                      │
     ┌───────────┬────────┬──────────┼────────┬────────────┐
     ▼           ▼        ▼          ▼        ▼            ▼
  Evidence    Rule Engine  Version   Knowledge  Reporting  Storage
 Collection               Profiles    Base
```

Of these, **Evidence Collection, Rule Engine, and Reporting are real and implemented.** **Version Profiles and Knowledge Base remain pure scaffolding** — real, architecturally-placed crates with zero implemented content, deliberately, per this platform's "capability before abstraction" discipline (Section 6). **Storage does not exist as a crate at all** — no persistence layer has ever been built; every Assessment is process-local and ephemeral today.

## The Two Domains

The platform separates two conceptual domains (ADR-0002 — filename `0002-domain-model-boundaries.md`, but the file's own internal title reads "ADR-0001," a known, flagged, uncorrected numbering mismatch that predates Engineering Release 0.2):

- **Runtime Domain** (`DataModel.md`) — information that exists because an Assessment is executed: `Assessment` (the sole aggregate root, ADR-0003), `Evidence`, `Finding`, `Recommendation`, `AssessmentReport`. Fully real, fully implemented.
- **Knowledge Domain** (`KnowledgeModel.md`) — reusable engineering knowledge independent of any Assessment: `Rule`, `Repair Recipe`, `Engine Behavior`, `Compatibility Pattern`, `Best Practice`, `Known Issue`, `Knowledge Reference`. Conceptually specified in full; zero implementation (`modiq-knowledge` is pure scaffold).

Runtime entities consume Knowledge Domain concepts only through **opaque references** (`RuleReference`, `RepairRecipeReference`) — Runtime stores an identifier, never the referenced domain's own type (ADR-0007). This is the structural mechanism that keeps the two domains decoupled even though only one is real today.

## The Rule Engine (`RuleEngine.md`)

Six conceptual responsibilities: Rule Selection, Evidence Evaluation, Finding Generation, Recommendation Generation, Traceability Management, Explainability. As of Sprint 5, `RuleEngine::evaluate` fulfills all six **inline**, dispatching across two concrete Rules (`EvidencePresenceRule`, `StructuralDuplicationRule`) via fixed, explicit declaration order — no trait, no registry, no per-responsibility service object (GOV-004, GOV-012; see Section 6). Execution principles: Deterministic, Evidence-Based, Knowledge-Driven, Explainable, Traceable, Version Aware (the last not yet realized — Rule Selection today is Evidence-Category-only, since no Version Profile capability exists to select against).

## Evidence Collection (`EvidenceCollection.md`)

Owns inspecting an Assessment Subject's actual content and producing `Evidence`. Two real Collectors exist: `EvidenceCollector` (filesystem discovery, Sprint 3) and `ArchiveCollector` (ZIP archive discovery, Sprint 4), both reachable from `AssessmentService::execute_from_assessment_input` via one explicit, inline `.zip`-suffix routing check. The **Collector Contract** (inputs: Assessment Input + Assessment Context; outputs: Evidence or a categorized failure; guarantees: determinism, factual accuracy, boundary respect) is the stable interface any future Collector (XML, Lua, manifest, dependency inspection — all named, none started) is expected to satisfy.

**Collection Outcomes** (GOV-010): four, and only four — Invalid Input, Inaccessible Input, Unsupported Input, Empty Collection. Extended for the archive case (GOV-011) without a fifth outcome.

## Reporting (`modiq-report`)

`AssessmentReport::generate` produces a pure, read-only snapshot of an `Assessment` (evidence, findings, recommendations, status, id) — no analysis, no formatting beyond what Runtime already carries. Four scaffold types (`FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, `ReportFormatter`) have existed, unused, since Sprint 0; Sprint 5 Phase 4 investigated and recommended their retirement (Section 9).

## Version Profiles and Knowledge Base

Both fully specified (`VersionProfile.md` — note: the file is named `VersionProfile.md` singular, but its own internal title reads "Version Profiles," another minor, harmless naming inconsistency; `KnowledgeModel.md`), both zero-implementation. Neither has a forcing function yet: Rule Selection has never needed Version-Profile-awareness, and two real Rules have never needed shared Knowledge content. Both are correctly, deliberately deferred, not neglected.

## Dependency and Documentation Authority Rules

Specifications form a strict authority hierarchy: `Vision.md` is highest; every other constitutional and architecture document derives from it; if a conflict exists, the higher-level document wins (stated identically in every specification's own "Specification Authority" section). Frozen specifications (Documentation Release 1.0: `Vision.md`, `Principles.md`, `Glossary.md`, `ProductSpecification.md`, `Architecture.md`, plus later technical specs frozen under their own releases) may only change for a demonstrated architectural defect, a documented contradiction, or an accepted ADR — never for routine implementation convenience (ADR-0001).

---

# 4. Governance

## Project Structure

Decisions flow through a fixed chain, unchanged in substance across every Sprint to date, only in role title:

```
Project Owner
     ↓
Chief Architect
     ↓
Lead Engineer
     ↓
Implementation
     ↓
Architectural Conformance Review
     ↓
Repository Closeout
```

The Project Owner sets priorities, approves the roadmap, and selects sprint objectives. The Chief Architect owns architecture, governance, and sprint scope authorization (`CHIEF_ARCHITECT_HANDOFF_v1.0.md`); the Lead Engineer owns implementation, testing, and documentation synchronization within that authorized scope (`LEAD_ENGINEER_HANDOFF_v3.0.md`). Every architectural decision and governance resolution on record originated from the Chief Architect role, never from independent engineering judgment.

## The Governance Register (`GOVERNANCE.md`, docs/engineering/)

Every architectural question discovered during engineering receives a permanent identifier and stays open until resolved by a Documentation Release. **13 items exist today; 8 Resolved, 5 Open:**

| Item | Title | Status |
|---|---|---|
| GOV-001 | Assessment Report Generation Timing | Open |
| GOV-002 | Runtime Invariant Reconciliation | Open |
| GOV-003 | Role of `modiq-common` | Open |
| GOV-004 | Engine Service Granularity | Resolved |
| GOV-005 | Finding minimum Evidence reference | Resolved (cardinality only) |
| GOV-006 | Recommendation minimum Finding reference | Resolved (cardinality only) |
| GOV-007 | Evidence Collection boundary-proving | Resolved |
| GOV-008 | AssessmentService Public API Evolution | Open |
| GOV-009 | Assessment Input Ownership | Resolved |
| GOV-010 | Collection Error Model | Resolved |
| GOV-011 | Archive Collection Model | Resolved |
| GOV-012 | Rule Evaluation Model | Resolved |
| GOV-013 | FindingSeverity Severity/Kind Conflation | **Open, deliberately** |

GOV-013 is worth understanding as a category, not just an entry: it was opened not because implementation broke something, but because *writing a precise specification* (Sprint 5's `FindingSeverity` definitions) surfaced that `BestPractice` classifies Finding *kind*, not *severity*, unlike `Error`/`Warning`/`Informational`. The Chief Architect declined to restructure the Runtime type from two Rules' worth of evidence and left it Open by design — see Section 9.

## Change Categories (`GOVERNANCE.md`)

Four levels: **Level 1 (Editorial)** — no approval needed. **Level 2 (Clarification)** — lightweight review. **Level 3 (Behavioral)** — defines valid/invalid input or new failure categories; requires governance approval before enforcement code is written (this is where most Collection Outcome and Rule Evaluation Model decisions have landed — GOV-009 through GOV-013). **Level 4 (Architectural)** — changes ownership boundaries or introduces a new architectural principle; requires an ADR.

## Crate Boundary Rules (`GOVERNANCE.md`)

Every crate has an explicit "Owns" / "Must never" pair. The two most load-bearing:
- **Rule Engine:** owns deterministic evaluation, rule execution, rule outcomes; must never mutate Assessment directly. `RuleEngine::evaluate` fulfills selection/evaluation/generation/traceability *inline*, without delegating to separate internal types — explicitly the approved pattern (ADR-0010, GOV-004).
- **Evidence Collection:** owns inspection and Evidence production; must never evaluate Evidence, produce Findings/Recommendations, mutate Assessment, own orchestration, or acquire its own Assessment Input.

## ADRs (`docs/adrs/`)

Ten, Accepted, never reused, never rewritten (`docs/adrs/README.md`: "Architecture evolves through new decisions, not by rewriting history"). Created only for decisions that change architecture, alter ownership boundaries, or establish a new principle — not for routine implementation. Full list and what each decided is in Section 6.

## Documentation Releases

Distinct from Engineering Releases: a Documentation Release freezes the *specification*; an Engineering Release records the *implementation state*. Three Documentation Releases to date: 1.0 (Foundation Freeze), 2.0 (governance/terminology reconciliation), 2.1 (Evidence Collection subsystem boundary — still current). `DataModel.md` was separately amended to v1.1.0 within Documentation Release 2.1's own umbrella during Sprint 5, for Finding Severity definitions — frozen documents can be amended in place when the amendment is recorded explicitly, per `DocumentationRelease.md`'s own exception clause; this has happened repeatedly (`EvidenceCollection.md` alone has been amended four times since its own freeze).

---

# 5. Engineering Workflow

**Engineering Methodology Version: 1.0** — declared following Sprint 7 and the Engineering Workflow Consolidation. Distinct from a Documentation Release (which freezes the platform's *specification*): this versions the engineering *methodology* itself — the workflow below and the philosophy behind it — as a stable architectural artifact, demonstrated across multiple completed engineering cycles (Sprints 5 through 7) rather than declared in advance of them. Future changes to it follow the same evidence-based evolution process used throughout this repository: a concrete forcing function, evaluated and resolved, before the methodology itself changes — not routine amendment.

This section is the repository's single canonical source for the engineering workflow — project-wide, not owned by either role. `CHIEF_ARCHITECT_HANDOFF_v1.0.md` and `LEAD_ENGINEER_HANDOFF_v3.0.md` reference this section rather than restating it; where either document once carried its own full copy, that copy has been reduced to role-specific elaboration. Future workflow changes should update this section first (`ENGINEERING_WORKFLOW_CONSOLIDATION_STUDY.md`, `ENGINEERING_WORKFLOW_CONSOLIDATION_REPORT.md`).

## Engineering Philosophy

The workflow below is a process description. What makes it work is a set of principles this project's own history has validated repeatedly, independent of any single Sprint — stated here once, in prose, rather than as a checklist to copy:

A capability is defined before its implementation is designed — Sprint 7's own charter ("determine what new capability should exist — not how to implement it") generalizes a discipline this project already practiced informally since Sprint 1. Architecture evolves from implementation evidence, not from anticipation of it: every accepted architectural model on record — direct engine composition (ADR-0010, GOV-004), fixed-order Rule dispatch (GOV-012), inline Collector composition (this Sprint) — was decided only after a concrete case existed to test it against, never in advance of one. Evidence precedes evaluation, structurally as well as procedurally: Collectors observe and Rules evaluate, in that order, and Governance itself only ever resolves a question once an Investigation has produced evidence to resolve it from, never from intuition alone. Refinement is preferred over premature abstraction — the platform's own repeated "capability before abstraction" test (does a second concrete case actually exist, right now, that needs this?) has been applied to Collector dispatch, Rule dispatch, and Collector composition alike, and every time an abstraction was deferred rather than built early, the deferral held. Architectural boundaries are preserved, not routed around for convenience — a crate's "Owns" and "Must never" pair (`GOVERNANCE.md`) is enforced at review time, not treated as a suggestion. Every Sprint should increase product capability, statable in one sentence — "after this Sprint, modIQ can now..." — not merely advance infrastructure for its own sake; Sprint 7's own Capability Success Criteria is the standing model for this, expected of every Sprint going forward, not a one-off. And every engineering concern has exactly one authoritative source, never several hand-synchronized copies — this workflow section's own existence is the direct, repository-native consequence of that principle being applied to the workflow's own documentation, after distributed copies of it had already drifted out of sync with each other.

## The Permanent Engineering Workflow

The canonical Sprint lifecycle, terminology unified project-wide as of this consolidation — superseding "Sprint Planning," "Authorization," "Chief Architect Review"/"Architecture Review" (previously used, ambiguously, for two different stages), and "Sprint Closeout" wherever those terms appeared for the same stages elsewhere in this repository:

```
Capability Definition
     ↓
Architecture Evaluation
     ↓
Architectural Resolution
     ↓
Implementation Authorization
     ↓
Implementation
     ↓
Validation
     ↓
Implementation Report
     ↓
Architectural Conformance Review
     ↓
Commit
     ↓
Merge
     ↓
Repository Closeout
```

**Capability Definition** establishes what new capability the work should provide — grounded in product specification and repository evidence — before any architecture is designed (Sprint 7's own charter: "do not begin by asking how to implement X; determine what new capability X should provide"). **Architecture Evaluation** is where a genuine architectural question exists (not every Sprint has one): concrete design alternatives are evaluated against repository evidence and established principles, and one is recommended — not yet approved. **Architectural Resolution** marks every open architectural question Accepted, Rejected, or Deferred; nothing may carry forward silently into implementation. **Implementation Authorization** confirms every remaining precondition (dependency selection, final scope) is satisfied and gives explicit, recorded permission to begin — distinct from Resolution, since narrower non-architectural decisions can still sit between the two, as Sprint 7's own dependency selection did. **Implementation** is the Lead Engineer's work against the authorized scope only. **Validation** is `cargo fmt`/`check`/`test`, both workspaces, zero warnings — run continuously as work proceeds, not solely as one discrete gate after Implementation finishes, and confirmed complete before a phase is reported. **Implementation Report** is the Lead Engineer's standard report. **Architectural Conformance Review** confirms the actual implementation matches what was resolved and authorized — no undisclosed scope expansion, no unauthorized abstraction; this should produce its own short recorded statement, not remain a passing remark at the top of a later session. **Commit** and **Merge** are separately authorized steps, never assumed to carry forward from a prior authorization. **Repository Closeout** reconciles the repository's authoritative documentation with the implementation following Sprint completion and integration — administrative consistency only, and the final stage before a Sprint is considered complete, not merely implemented.

## Historical Background

`EngineeringGuide.md`'s own, earlier articulation of the same underlying discipline, retained here for history rather than deleted, per this project's own convention of amending rather than silently erasing prior text. Superseded by the workflow above wherever the two differ in stage granularity or terminology:

1. Review the relevant specification.
2. Identify affected architectural boundaries.
3. Implement the feature.
4. Write or update tests.
5. Verify consistency with the Engineering Specification.
6. Produce a standard implementation report.
7. Submit for review.

In practice, for anything touching a governance question, a design proposal (or a lighter-weight "preparation" document for narrower questions) precedes implementation — this has held for GOV-007 through GOV-013 without exception, and has never once required post-implementation architectural rework.

## Every Implementation Task Concludes With

`cargo fmt`, `cargo check --workspace`, `cargo test --workspace` — all three, every phase, both the root workspace and, independently, `apps/sandbox/src-tauri`'s own separate workspace. Zero warnings is the standing bar, not just zero errors.

## Real-I/O Testing Discipline

Established at Sprint 3 Phase 5 (the first code needing OS-level test fixtures — temp directories, symbolic links) and reused, not reinvented, at every subsequent I/O-touching phase (Sprint 4's archive fixtures, byte-level fixture construction once `ZipWriter` started rejecting duplicate filenames at write time). No mocking of real I/O; real, checked-in or test-constructed fixtures throughout.

## Determinism-by-Content, Not Identity

Universal convention: every Runtime entity's own identity (`AssessmentId`, `EvidenceId`, `FindingId`, `RecommendationId`) is freshly assigned per construction/evaluation, by design. Determinism is judged by *content and order*, never by incidental identity. Applied without exception since Sprint 2.

## Public API Changes

Governed by `GOVERNANCE.md`'s Public API Policy: breaking changes require governance approval. `AssessmentService` has exactly two public entry points (`execute`, `execute_from_assessment_input`), added additively — `execute_from_assessment_input` was introduced alongside `execute`, never replacing it, specifically to avoid resolving GOV-008 as a side effect of unrelated work. GOV-008 has now been open across three Sprints (3, 4, 5).

---

# 6. Architectural Principles Learned

These are not restated from the specifications — they are patterns this project's own history has validated, repeatedly, under real implementation pressure. A new contributor should treat these as load-bearing, not optional style preferences.

**1. Capability before abstraction.** Stated once (Chief Architect principle, first applied to the Rule trait question), then independently re-validated at least six times: Collector dispatch (GOV-004), the Rule Engine's own internal dispatch (GOV-012), Reporting's scaffold-retirement question (Sprint 5 Phase 4), and the original EngineAPI/`modiq-rules` service-object retirement (ADR-0010) itself. The test every time: does a *second concrete case* actually exist and actually need the abstraction? If not, the abstraction does not get built, no matter how architecturally clean it would be. Three independent subsystems converging unprompted on direct composition (ADR-0010) is treated as *stronger* evidence than any single design argument.

**2. A concrete forcing function, not a hypothetical one, justifies a model change.** Applied to GOV-004 (three Engineering Releases of non-use before retirement), and applied again, freshly, to GOV-013 (the Chief Architect explicitly declined to restructure `FindingSeverity` from two Rules' worth of evidence, deferring to whenever a third Rule actually needs the distinction).

**3. Proposal-then-implement, not implement-then-reconcile.** Every governance-relevant capability (Evidence Collection's boundary, the filesystem Collector, GOV-011's archive policies, GOV-012's Rule Evaluation Model) was designed in a proposal document, approved, and *then* implemented — never the reverse. Zero instances of post-implementation architectural rework are on record as a result.

**4. Specification-writing is itself a source of architectural findings**, not just implementation or testing. GOV-013 is the first item on this project's Governance Register to originate from the act of writing a precise definition (`FindingSeverity`'s semantics) rather than from a bug, a test failure, or an unused-code audit. Expect this again wherever previously-implicit semantics get written down for the first time.

**5. Determinism is verified directly, never assumed.** Every ordering claim (archive entry order, Rule dispatch order, directory traversal order) has its own direct test proving it survives varied input arrangement, not just repeated identical calls. Sprint 5 Phase 5 exists specifically because a repeated-identical-input test had quietly been standing in for an arrival-order-independence test that didn't yet exist.

**6. Evidence-first, single-channel reporting.** When a genuinely new kind of fact needed representing (Sprint 4's duplicate-entry detection), the option of a second, out-of-band reporting channel was explicitly considered and rejected — specifically because it risked reopening GOV-008. Every fact this platform surfaces to a Rule flows through `Evidence`; every fact a Rule concludes flows through `Finding`/`Recommendation`. No parallel channel has ever been introduced.

**7. Runtime Entity Design Pattern (ADR-0007) is uniform, not per-entity.** Aggregate ownership, stable identity, value-object fields, opaque cross-domain references, constructor validation, identity-based equality, aggregate-owned relationship resolution, governance-controlled invariants. Every Runtime entity added since Sprint 2 has followed this pattern without a fresh design conversation.

**8. Frozen does not mean immutable — it means change is recorded, not silent.** Multiple Frozen specifications (`Architecture.md`, `EngineAPI.md`, `EvidenceCollection.md`, `DataModel.md`) have been amended in place, repeatedly, each time with the amendment and its rationale stated directly in the document rather than hidden in a diff.

**9. Documentation staleness between checkpoints is a real, recurring, still-unsolved pattern.** `PROJECT_STATUS.md`/`CHANGELOG.md` have gone stale mid-Sprint at every closeout checked so far (Sprint 3, 4, 5, 6 — four for four). Sprint 6 extended the pattern for the first time to this document and its two role-specific companions (`CHIEF_ARCHITECT_HANDOFF_v1.0.md`, `LEAD_ENGINEER_HANDOFF_v3.0.md`), not just `PROJECT_STATUS.md`/`CHANGELOG.md`. Reconciliation at Sprint close has been reliable; preventing staleness *between* closeouts has not yet been solved, and is currently tracked as an engineering workflow refinement goal ("keep `PROJECT_STATUS.md` current at meaningful milestones"), not a mandatory per-phase rule (the Chief Architect specifically declined the latter at Sprint 5 Closeout).

---

# 7. Sprint History

| Sprint | Delivered | Test Growth |
|---|---|---|
| **Sprint 0** | Workspace scaffolding, nine-crate structure, governance foundation | — |
| **Sprint 1** | First complete deterministic pipeline: Assessment creation → Evidence → Rule evaluation → Findings/Recommendations → Report → completion. Tagged `v0.1.0-alpha`. | → 55 |
| **Sprint 2** | Real field content for Evidence/Finding/Recommendation; identity, constructor validation; Runtime Entity Design Pattern (ADR-0007) established. GOV-005/006 opened. | → 97 |
| **Sprint 3** | Evidence Collection subsystem from architectural boundary (ADR-0008/0009, Documentation Release 2.1) through its first real Collector (filesystem discovery, GOV-009/010). Nine crates. | → 112 |
| *(Platform Validation Phase 1)* | GOV-004 resolved (direct composition confirmed, EngineAPI/`modiq-rules` service scaffolding retired); GOV-008 evaluated and deliberately deferred. | — |
| **Sprint 4** | Second real Collector: `ArchiveCollector` (ZIP), across Phases 1–3D. GOV-011 resolved (malformed/duplicate/traversal/resource-limit policies). `EvidenceCategory::StructuralDuplication` added. Explicit `.zip`-routing wired into `AssessmentService`. | 112 → 150 |
| **Sprint 5** | Assessment intelligence layer, deliberately *not* a third Collector. GOV-012 resolved (multi-Rule dispatch shape). GOV-013 opened (deliberately). First `FindingSeverity` definitions (`DataModel.md` v1.1.0). Second real Rule (`StructuralDuplicationRule`) and real multi-Rule `RuleEngine::evaluate`. Reporting scaffold-retirement investigated, not acted on. | 150 → 162 |
| **Sprint 6** | `modiq-cli` wired to `modiq-engine` for the first time since Sprint 0 (`Application` dispatch, `AssessCommand` calling `execute_from_assessment_input` against a real user-supplied path, three-tier exit-code convention). `modiq-report`'s four scaffold types retired under explicit authorization; `AssessmentReport` unchanged. No change to `AssessmentService`'s public entry points. Implemented, reviewed, and merged into `feature/runtime-implementation`; formal Engineering Release 0.6 record not yet produced. | 162 → 172 |

Full detail for each Sprint lives in its own Engineering Release (`ENGINEERING_RELEASE_v0.1.0-alpha.md`, `docs/releases/ENGINEERING_RELEASE_0.2.md`, `ENGINEERING_RELEASE_0.3.md`, `_0.4.md`, `_0.5.md`) — each one is also that Sprint's retrospective and completion report combined, per this project's own established convention (no separate retrospective files; one existed only briefly in error — see Section 11). Sprint 6 does not yet have its own Engineering Release record; `docs/engineering/SPRINT6_IMPLEMENTATION_PLAN.md` and `docs/engineering/POST_SPRINT6_REPOSITORY_ASSESSMENT.md` are its record until one is produced.

**No Engineering Release has ever been git-tagged.** `v0.1.0`, `v0.1.0-alpha`, `v0.2.0`, `v0.2.0-alpha`, `v0.3.0` exist as tags, but predate-and-collide oddly with the Engineering Release numbering (a pre-existing git tag hygiene issue, flagged at every release since 0.3, never resolved). `v0.4.0` and `v0.5.0` are both currently available, untagged.

---

# 8. Current Repository State

- **Branch:** `feature/runtime-implementation`, tracking `origin/feature/runtime-implementation`, in sync (no ahead/behind).
- **HEAD:** `29657df` — Sprint 6's merge commit, following implementation on `feature/sprint6-cli` (`397707f`), committed and pushed.
- **Working tree:** clean.
- **Tests:** 172/172 root workspace, 6/6 Sandbox, zero warnings in either, verified directly while preparing this document (not carried over from any prior report).
- **Crate maturity:** see Section 2's table. `modiq-collection`, `modiq-rules`, and now `modiq-cli` (L2, Sprint 6) are where real capability has been built; `modiq-knowledge`, `modiq-versioning`, `modiq-common` remain L1 scaffolding, correctly deferred per Section 6's Principle 1 and 2.

---

# 9. Current Investigations (Open, Unresolved, or Recommended-but-Not-Acted-On)

- **GOV-013 (FindingSeverity Severity/Kind Conflation)** — Open by design. Will only move when a third Rule genuinely needs to express Finding *kind* independent of *severity*. Do not resolve this speculatively; wait for the forcing function. Unaffected by Sprint 6, which did not touch `modiq-rules`.
- **GOV-008 (AssessmentService Public API Evolution)** — Open across four Sprints, now including Sprint 6. `modiq-cli` became a second real consumer of `execute_from_assessment_input`, reusing it exactly as designed — this generated no new evidence toward resolving GOV-008, and was not expected to (confirmed explicitly during Sprint 6 scoping). The two-entry-point additive pattern remains the deliberate stopgap.
- **GOV-001, GOV-002, GOV-003** — all Open since Engineering Release v0.1.0-alpha, all still pending Documentation Release 1.1 or further evidence, none blocking current work.
- **Reporting scaffold retirement — Resolved at Sprint 6.** `FindingSummary`/`RecommendationSummary`/`TraceabilityReport`/`ReportFormatter` were deleted under Sprint 6's explicit Chief Architect authorization. No longer an open item.
- **A new, minor architectural item, named but not yet a Governance Register entry:** `modiq-engine` does not re-export `AssessmentReport`, so both real consumers of `AssessmentService` (the Sandbox and, since Sprint 6, `modiq-cli`) independently depend on `modiq-report` directly just to name the type. Two data points — below this project's own usual three-point convergent-evidence bar (see Section 6, Principle 1's GOV-004 example) — tracked for a future third occurrence, not acted on.
- **Documentation-staleness workflow refinement** — tracked as a goal (keep `PROJECT_STATUS.md` current at meaningful milestones), explicitly *not* a mandatory per-phase rule. Four consecutive Sprints (3, 4, 5, 6) have now shown the pattern recurring between closeouts even with reconciliation happening reliably at closeout itself — Sprint 6 is the first instance where the staleness reached this document and its two role-specific companions, not just `PROJECT_STATUS.md`/`CHANGELOG.md`.
- **Known, harmless documentation inconsistencies** (see Section 11) — flagged, not fixed, since fixing them wasn't in scope for whichever session first noticed each one, and none affects architecture or implementation.

---

# 10. Roadmap

**Not yet scoped for Sprint 7.** Of the three candidates on record at Sprint 5 Closeout, two are now done:

1. **XML inspection** — the next Evidence Collector in `PROPOSAL_FILESYSTEM_COLLECTION.md`'s own original sequencing (filesystem → archive → XML). Explicitly deferred through Sprint 5 specifically so it would build on a mature Rule Engine rather than drive its design — that condition is satisfied and unchanged by Sprint 6. **The sole remaining item from the original three-candidate list.**
2. ~~**CLI wiring**~~ — **Done, Sprint 6.** `modiq-cli`'s `AssessCommand` now calls `AssessmentService::execute_from_assessment_input` directly, reusing the Sandbox's own thin-client pattern rather than reinventing it, exactly as this entry anticipated.
3. ~~**Acting on the Reporting scaffold-retirement recommendation**~~ — **Done, Sprint 6.** The four scaffold types are deleted.

**Longer-term, per `Vision.md`:** cross-version compatibility analysis, community-validated knowledge, Repair Recipe libraries, intelligent pattern recognition — all depend on Knowledge Domain and Version Profile integration eventually happening, neither of which has a forcing function yet (Section 6, Principles 1–2).

---

# 11. Known Documentation Inconsistencies

Recorded for transparency, not urgency — none affects architecture or implementation, and per this project's own discipline, accepted ADRs and Frozen specs are not silently rewritten to fix cosmetic issues.

- **ADR-0002**'s own file title reads "ADR-0001" internally, despite its filename (`0002-domain-model-boundaries.md`) and every external reference treating it as ADR-0002. Already flagged in `docs/adrs/README.md` itself.
- **`Architecture.md`**'s header table currently reads Version 1.1.1, while its own Document Status footer reads "Current Version: 1.1.2" — a one-version mismatch between the two places this document states its own version.
- **`VersionProfile.md`** (the filename) internally titles itself "Version Profiles" (plural) in its own metadata table's `Document` field, which literally reads `VersionProfiles.md`.
- **A `SPRINT4_RETROSPECTIVE.md` reference** existed in both `PROJECT_STATUS.md` and `CHANGELOG.md` for one Sprint (referencing a file that was never created — Sprint 4's retrospective has always lived inside `ENGINEERING_RELEASE_0.4.md` itself). Found and corrected during Sprint 5 Closeout; mentioned here only so its history is traceable.
- **`docs/governance/ROADMAP.md` and `EngineeringGuide.md`** are stale since Sprint 0/1 (2026-07-16) — `ROADMAP.md`'s own phase model (Phase 3 = Sprint 1, Phase 4 = MVP, Phase 5 = Alpha...) has never tracked actual Sprint numbering past Sprint 1. Flagged at Sprint 4 and 5 Closeout, deliberately not fixed either time (larger content judgment than either closeout's own scope covered).
- **`docs/releases/ENGINEERING_RELEASE_0.2.md`** lives in a different directory than every later Engineering Release (`docs/engineering/`) — a location inconsistency, not a content one.

---

# 12. Where to Go Next

- **New to the project entirely?** Read `Vision.md` → `Principles.md` → `Glossary.md` → `ProductSpecification.md` → `Architecture.md`, in that order (`EngineeringGuide.md`'s own Required Reading list, still accurate).
- **Picking up implementation work?** Read `LEAD_ENGINEER_HANDOFF_v3.0.md` next — it assumes this document and covers current branch state, immediate priorities, and engineering expectations without repeating what's here.
- **Reviewing architecture or governance?** Read `GOVERNANCE.md` (`docs/engineering/`) in full, then the ADRs in numeric order.
- **Want the full history of any single Sprint?** Its own `ENGINEERING_RELEASE_*.md` is the complete, self-contained record — retrospective and completion report included.

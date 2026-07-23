# modIQ Project Handoff v1.1

> **The authoritative, role-independent onboarding document for the modIQ repository.**

---

| Property | Value |
|----------|-------|
| **Document** | PROJECT_HANDOFF_v1.1.md |
| **Project** | modIQ |
| **Purpose** | Canonical project handoff — describes the platform, its architecture, its governance, and its history independently of any individual role |
| **Audience** | Anyone onboarding to modIQ — engineering, architecture, or product — before reading a role-specific handoff |
| **Supersedes** | `PROJECT_HANDOFF_v1.0.md` (Sprint 6-era). `LEAD_ENGINEER_HANDOFF_v3.0.md` and `CHIEF_ARCHITECT_HANDOFF_v1.0.md`/`v1.1.md` (the current role-specific supplements) remain role-specific companions; both assume the reader has already read this document and do not repeat what it covers. |
| **As of** | 2026-07-23, following Sprint 12 (Capability Scaling Architecture: Capability Identity procedure derived, adversarially reconciled, validated); Repository Closeout complete, tagged, pushed |
| **Branch** | `feature/runtime-implementation` |
| **HEAD** | `681dd2e` — "Sprint 12: engineering release and closeout" |

---

# How to Use This Document

This is the first document anyone — a new engineer, a new Chief Architect session, a reviewer, a future contributor — should read about modIQ. It does not belong to Engineering or to the Chief Architect; it describes the project itself: why it exists, how it is organized, how decisions get made, what has been built, and what is still open.

After this document, read the role-specific supplement for your role (`LEAD_ENGINEER_HANDOFF_v3.0.md` or the current `CHIEF_ARCHITECT_HANDOFF_v1.*.md`). Those documents assume you have read this one and will not repeat its content.

Everything in this document is verified directly against the repository as of the commit above, not carried over from memory or from prior handoff documents. Per this project's own standing discipline, conversation context that was never committed to the repository as a file is treated as non-authoritative and is not reflected here, however substantive it may have been.

---

# 1. Product Vision

modIQ exists to make every Farming Simulator mod understandable. Every assessment should answer three questions: *does this mod work? why? what can I do next?* The platform is an evidence-based assessment tool, not a scoring tool — its purpose is to explain, not merely to flag.

**Core beliefs** (`Vision.md`): understanding creates confidence; evidence is more valuable than opinion; transparent systems create stronger communities than opaque automation; every technical conclusion should be explainable; software should educate its users rather than simply produce results; preserving knowledge is as important as generating it.

**Target users** (`ProductSpecification.md`): players (want confidence before installing/troubleshooting), mod creators (want objective engineering feedback), server administrators (want to validate mod collections), and community contributors (future — contribute validated knowledge back).

**Non-goals, explicitly stated:** modIQ does not automatically rewrite mods, replace creators, replace the Farming Simulator editor, replace community discussion, make subjective quality judgments without evidence, or hide complexity behind opaque scoring. It explains, educates, and preserves knowledge — it does not decide for the user.

**Decision priority order** (`Principles.md`, when approaches conflict): Correctness → Explainability → Evidence → Determinism → User Understanding → Extensibility → Performance → Convenience. This ordering has been invoked, implicitly or explicitly, in essentially every architectural decision recorded in this repository's history — Determinism Before Intelligence and Evidence Over Assumption in particular show up by name repeatedly, most recently in Sprint 11's own `FindingSeverity::Error` reasoning and Sprint 12's own adversarial-verification discipline.

The initial implementation targets Farming Simulator 25, with the architecture deliberately designed to support future FS releases through Version Profiles rather than redesign (`Principles.md`: Platform Independence; ADR-0004) — a dimension now real, not merely specified (Section 3).

---

# 2. Repository Organization

```
docs/
├── constitutional/     Vision, Principles, Glossary, ProductSpecification — Frozen, Documentation Release 1.0
├── architecture/       Architecture, DataModel, KnowledgeModel, RuleEngine, EvidenceCollection, EngineAPI, VersionProfile
├── adrs/                Architecture Decision Records, 0001–0010, sequentially numbered, never reused
├── governance/          GOVERNANCE.md's home is actually docs/engineering/ (see note below); PROJECT_STATUS.md, CHANGELOG.md, ROADMAP.md, EngineeringGuide.md, DocumentationRelease.md
├── engineering/         GOVERNANCE.md, ENGINEERING_LOG.md, Sprint plans, Architectural Resolutions, Engineering Release records, Proposals, Platform Validation and Capability Prioritization records
├── implementation/      RuntimeInvariants.md, CrateRoadmap.md, AssessmentCreation.md, SPRINT0.md–SPRINT12.md
└── releases/            ENGINEERING_RELEASE_0.2.md only — a pre-existing, minor location inconsistency; every later Engineering Release lives in docs/engineering/

fixtures/                New since Sprint 10: fixtures/runtime-logs/ — a permanent, top-level, provenance-tracked corpus of real, captured Farming Simulator runtime logs, deliberately separate from apps/sandbox/src-tauri/fixtures/'s own unrelated synthetic-fixture convention. Governed entirely by its own README.md and TEMPLATE.md, not by any workspace crate.
crates/                  Cargo workspace, 9 crates (below)
apps/sandbox/             Tauri desktop application — the only real end-to-end consumer of the platform, its own separate Cargo workspace and `Cargo.lock`
```

**A genuine location quirk, not a defect:** `GOVERNANCE.md` — despite being one of the most-referenced documents in this repository — lives at `docs/engineering/GOVERNANCE.md`, not under `docs/governance/`. `docs/governance/` instead holds the living status documents (`PROJECT_STATUS.md`, `CHANGELOG.md`, `ROADMAP.md`, `EngineeringGuide.md`, `DocumentationRelease.md`). This has been true since early in the project and every cross-reference in the repository already accounts for it; noted here only so a new reader isn't surprised.

## The Nine Workspace Crates

| Crate | Responsibility | Maturity | Depends On |
|---|---|---|---|
| `modiq-runtime` | Runtime Domain — Assessment aggregate, Evidence, Finding, Recommendation, lifecycle | L3, 84 tests | (leaf — no workspace dependencies) |
| `modiq-collection` | Evidence Collection — produces Evidence from real content. Four real Collectors: `EvidenceCollector` (filesystem, Sprint 3), `ArchiveCollector` (ZIP, Sprint 4), `XmlCollector` (manifest content, Sprint 7), `RuntimeLogCollector` (runtime event content, Sprint 11) | L2, 70 tests | `modiq-runtime` |
| `modiq-rules` | Rule Engine — deterministic Rule evaluation. Four real Rules dispatched in fixed declaration order: `EvidencePresenceRule`, `StructuralDuplicationRule`, `VersionCompatibilityRule` (Sprint 8), `RuntimeLoadFailureRule` (Sprint 11) | L3, 36 tests | `modiq-runtime`, `modiq-versioning` (Sprint 8), `modiq-knowledge` (Sprint 9) |
| `modiq-report` | Reporting — `AssessmentReport` snapshot generation | L3, 3 tests | `modiq-runtime` |
| `modiq-engine` | Engine — `AssessmentService`, the sole orchestration boundary | L3, 23 unit + 3 integration tests | `modiq-runtime`, `modiq-rules`, `modiq-report`, `modiq-collection`, `modiq-versioning` |
| `modiq-knowledge` | Knowledge Domain — reusable engineering knowledge | L2, 5 tests — first real content since Sprint 0 (Sprint 9): `RepairRecipe` with real fields, an infallible constructor, one named, authored recipe. Six remaining Knowledge Model categories (`Rule`, `Engine Behavior`, `Compatibility Pattern`, `Best Practice`, `Known Issue`, `Knowledge Reference`) remain unimplemented, deliberately | `modiq-runtime` |
| `modiq-versioning` | Version Profiles — game-version compatibility context | L2, 4 tests — first real content since Sprint 0 (Sprint 8): `GameVersion`/`VersionProfile`, a single hardcoded `VersionProfile::fs25()`. `Capability`/`Compatibility` remain unimplemented, deliberately | `modiq-runtime` |
| `modiq-cli` | CLI entry point | L2, 10 tests — wired to `modiq-engine` (Sprint 6): `Application` dispatches `assess`/`help`/`version`, `AssessCommand` calls `AssessmentService::execute_from_assessment_input` against a user-supplied path | `modiq-runtime`, `modiq-engine`, `modiq-report` |
| `modiq-common` | Shared platform types | L1, empty stub files, zero evidence it's needed after twelve Sprints | (none) |

Dependency direction is strictly downward; `modiq-runtime` is the leaf every other crate ultimately depends on. No circular dependency has ever existed. `apps/sandbox` and `modiq-cli` both depend on `modiq-engine` (and, transitively, everything below it) for orchestration, preserving `AssessmentService` as the sole orchestration entry point — but both also depend on `modiq-report` directly, since `modiq-engine` does not re-export `AssessmentReport` (Section 9). Neither crate depends on `modiq-collection` or `modiq-rules` directly.

**Root workspace: 238 tests, zero ignored, zero flaky.** **Sandbox: 7 tests**, its own separate workspace, independently verified. (`modiq-cli` 10, `modiq-collection` 70, `modiq-common` 0, `modiq-engine` 23 unit + 3 integration, `modiq-knowledge` 5, `modiq-report` 3, `modiq-rules` 36, `modiq-runtime` 84, `modiq-versioning` 4.)

---

# 3. Architecture

## System Overview (`Architecture.md`)

modIQ is organized as cooperating platform subsystems centered on the **Assessment Service**, which orchestrates the complete Assessment lifecycle by direct composition of each subsystem — no intra-engine service indirection layer (see ADR-0010, Section 6 below).

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

**Evidence Collection, Rule Engine, Reporting, Version Profiles, and Knowledge are all real and implemented today** — the last two only since Sprint 8 and Sprint 9 respectively, each with deliberately minimum-viable content (one hardcoded `VersionProfile`, one authored `RepairRecipe`), not the full conceptual model either specification describes. **Storage does not exist as a crate at all** — no persistence layer has ever been built; every Assessment is process-local and ephemeral today.

## The Two Domains

The platform separates two conceptual domains (ADR-0002 — filename `0002-domain-model-boundaries.md`, but the file's own internal title reads "ADR-0001," a known, flagged, uncorrected numbering mismatch that predates Engineering Release 0.2):

- **Runtime Domain** (`DataModel.md`) — information that exists because an Assessment is executed: `Assessment` (the sole aggregate root, ADR-0003), `Evidence`, `Finding`, `Recommendation`, `AssessmentReport`. Fully real, fully implemented.
- **Knowledge Domain** (`KnowledgeModel.md`) — reusable engineering knowledge independent of any Assessment: `Rule`, `Repair Recipe`, `Engine Behavior`, `Compatibility Pattern`, `Best Practice`, `Known Issue`, `Knowledge Reference`. Conceptually specified in full; one category (`Repair Recipe`) real since Sprint 9, the remaining six still scaffold.

Runtime entities consume Knowledge Domain and Version Profile concepts only through **opaque references** (`RuleReference`, `RepairRecipeReference`, `VersionProfileReference`) — Runtime stores an identifier, never the referenced domain's own type (ADR-0007). This pattern has now been independently exercised three times (Sprint 2, Sprint 8, Sprint 9) with zero modification required, confirming it generalizes rather than merely working once.

## The Rule Engine (`RuleEngine.md`)

Six conceptual responsibilities: Rule Selection, Evidence Evaluation, Finding Generation, Recommendation Generation, Traceability Management, Explainability. `RuleEngine::evaluate` fulfills all six **inline**, dispatching across four concrete Rules (`EvidencePresenceRule`, `StructuralDuplicationRule`, `VersionCompatibilityRule`, `RuntimeLoadFailureRule`) via fixed, explicit declaration order — no trait, no registry, no per-responsibility service object (GOV-004, GOV-012). Execution principles: Deterministic, Evidence-Based, Knowledge-Driven, Explainable, Traceable, Version Aware — the last now real (Sprint 8: `RuleEngine::evaluate` accepts the active `VersionProfile` directly).

## Evidence Collection (`EvidenceCollection.md`)

Owns inspecting an Assessment Subject's actual content and producing `Evidence`. Four real Collectors exist, composed along two independent axes: a mutually-exclusive structural axis (`EvidenceCollector` for filesystem input, `ArchiveCollector` for `.zip` input, chosen by one explicit routing check) and an additive content-inspecting axis (`XmlCollector`, `RuntimeLogCollector`, both invoked unconditionally alongside the structural Collector, per the approved Collector Composition Architecture, Sprint 7). The **Collector Contract** (inputs: Assessment Input + Assessment Context; outputs: Evidence or a categorized failure; guarantees: determinism, factual accuracy, boundary respect) is the stable interface every Collector satisfies.

**Collection Outcomes** (GOV-010): four, and only four — Invalid Input, Inaccessible Input, Unsupported Input, Empty Collection. Extended for the archive case (GOV-011) without a fifth outcome.

**Capability Scaling Architecture (Sprint 12).** An explicit, historically-derived decision procedure now governs when a new capability requires a new Collector versus extending an existing one, and separately, whether it requires a new `EvidenceCategory` — the two are independent dimensions, not one cascading test (full procedure: `docs/engineering/SPRINT12_ARCHITECTURAL_RESOLUTION.md`). This does not change the Collector Contract itself; it governs how future Collectors are classified before they are built.

## Reporting (`modiq-report`)

`AssessmentReport::generate` produces a pure, read-only snapshot of an `Assessment` (evidence, findings, recommendations, status, id) — no analysis, no formatting beyond what Runtime already carries. The four scaffold types present through Sprint 5 (`FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, `ReportFormatter`) were retired at Sprint 6 under explicit Chief Architect authorization; the crate has contained only `AssessmentReport` since.

## Version Profiles and Knowledge Base

Both now real, both deliberately minimum-viable, neither the full conceptual model its own specification describes:

- **Version Profiles** (`VersionProfile.md`) — `modiq-versioning` gained `GameVersion`/`VersionProfile` and a single hardcoded `VersionProfile::fs25()` at Sprint 8, consumed directly by `AssessmentService` and by `VersionCompatibilityRule`. No profile-selection mechanism exists — every Assessment runs against the same profile, since no second profile yet exists for a caller to choose between.
- **Knowledge Base** (`KnowledgeModel.md`) — `modiq-knowledge` gained `RepairRecipe` (fields, constructor, one named, authored recipe) at Sprint 9, consumed directly by `VersionCompatibilityRule`'s own `Recommendation`. `RuntimeLoadFailureRule` (Sprint 11) does not yet have a paired Repair Recipe — its Recommendation is inline-authored, mirroring how `VersionCompatibilityRule` itself operated before Sprint 9.

Both remain correctly, deliberately unexpanded beyond this — neither has a forcing function yet for its own remaining conceptual breadth.

## Dependency and Documentation Authority Rules

Specifications form a strict authority hierarchy: `Vision.md` is highest; every other constitutional and architecture document derives from it; if a conflict exists, the higher-level document wins (stated identically in every specification's own "Specification Authority" section). Frozen specifications (Documentation Release 1.0, plus later technical specs frozen under their own releases) may only change for a demonstrated architectural defect, a documented contradiction, or an accepted ADR — never for routine implementation convenience (ADR-0001).

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

The Project Owner sets priorities, approves the roadmap, and selects sprint objectives. The Chief Architect owns architecture, governance, and sprint scope authorization; the Lead Engineer owns implementation, testing, and documentation synchronization within that authorized scope. Every architectural decision and governance resolution on record originated from the Chief Architect role, never from independent engineering judgment.

## The Governance Register (`GOVERNANCE.md`, docs/engineering/)

Every architectural question discovered during engineering receives a permanent identifier and stays open until resolved by a Documentation Release. **13 items exist today; 8 Resolved, 5 Open — unchanged in count since Engineering Release 0.8, across six further Sprints (7 through 12):**

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

**GOV-013 is worth understanding as a category, not just an entry:** it was opened not because implementation broke something, but because *writing a precise specification* (Sprint 5's `FindingSeverity` definitions) surfaced that `BestPractice` classifies Finding *kind*, not *severity*. Sprint 11's own `FindingSeverity::Error` assignment (the platform's first real use of that variant) was recorded as new evidence relevant to a future GOV-013 review — deliberately not treated as grounds to reopen it now (Section 7).

**No new Governance Register item and no new ADR were opened across Sprints 7 through 12** — every capability shipped in that span (XML inspection, Version Profile activation, Repair Guidance, Runtime Fixture acquisition, Runtime Evidence Processing, Capability Scaling Architecture) either applied and extended already-approved architecture, or — where a genuine new question existed (Collector Composition, Sprint 7) — resolved it as its own dedicated Architecture Evaluation without requiring a permanent Register entry, following the same "implementation evidence, not routine amendment" discipline GOV-012/013 themselves were held to.

## Change Categories (`GOVERNANCE.md`)

Four levels: **Level 1 (Editorial)** — no approval needed. **Level 2 (Clarification)** — lightweight review. **Level 3 (Behavioral)** — defines valid/invalid input or new failure categories; requires governance approval before enforcement code is written. **Level 4 (Architectural)** — changes ownership boundaries or introduces a new architectural principle; requires an ADR.

## Crate Boundary Rules (`GOVERNANCE.md`)

Every crate has an explicit "Owns" / "Must never" pair. The two most load-bearing:
- **Rule Engine:** owns deterministic evaluation, rule execution, rule outcomes; must never mutate Assessment directly. `RuleEngine::evaluate` fulfills selection/evaluation/generation/traceability *inline* — the approved pattern (ADR-0010, GOV-004), now exercised by four Rules without needing to change shape.
- **Evidence Collection:** owns inspection and Evidence production; must never evaluate Evidence, produce Findings/Recommendations, mutate Assessment, own orchestration, or acquire its own Assessment Input. Now exercised by four Collectors along two independent composition axes (Section 3), without needing to change shape.

## ADRs (`docs/adrs/`)

Ten, Accepted, never reused, never rewritten. Created only for decisions that change architecture, alter ownership boundaries, or establish a new principle — not for routine implementation. No ADR has been added since ADR-0010 (Sprint 6-era); Sprints 7 through 12 each applied existing ADRs rather than requiring new ones.

## Documentation Releases

Distinct from Engineering Releases: a Documentation Release freezes the *specification*; an Engineering Release records the *implementation state*. Three Documentation Releases to date: 1.0 (Foundation Freeze), 2.0 (governance/terminology reconciliation), 2.1 (Evidence Collection subsystem boundary — still current, unchanged through Sprint 12). `DataModel.md` was separately amended to v1.1.0 during Sprint 5 for Finding Severity definitions; frozen documents can be amended in place when the amendment is recorded explicitly, per `DocumentationRelease.md`'s own exception clause — this has continued (`RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md`, an engineering-level architecture document rather than a Documentation Release specification, was itself amended twice within Sprint 11 alone, each amendment recorded explicitly).

---

# 5. Engineering Workflow

**Engineering Methodology Version: 1.0** — declared following Sprint 7, unchanged through Sprint 12. Distinct from a Documentation Release: this versions the engineering *methodology* itself as a stable architectural artifact. Future changes to it follow the same evidence-based evolution process used throughout this repository — a concrete forcing function, evaluated and resolved, before the methodology itself changes.

This section is the repository's single canonical source for the engineering workflow — project-wide, not owned by either role.

## Engineering Philosophy

The workflow below is a process description. What makes it work is a set of principles this project's own history has validated repeatedly:

A capability is defined before its implementation is designed. Architecture evolves from implementation evidence, not from anticipation of it. Evidence precedes evaluation, structurally as well as procedurally. Refinement is preferred over premature abstraction — the platform's own "capability before abstraction" test has now been applied to Collector dispatch, Rule dispatch, Collector composition, *and*, as of Sprint 12, the very question of when a new capability needs a new architectural participant at all, and every time an abstraction was deferred rather than built early, the deferral held. Architectural boundaries are preserved, not routed around for convenience. Every Sprint should increase product capability, statable in one sentence. And every engineering concern has exactly one authoritative source, never several hand-synchronized copies.

**A further principle, confirmed rather than newly discovered across Sprints 10 through 12: dedicated adversarial verification — checking a finished artifact against the full body of evidence specifically to attempt to disprove it, not confirm it — finds real errors that good-faith review, applied in good faith at the time, does not.** This has now held at three separate stages: a not-yet-implemented resolution reviewed before coding began (Sprint 8's `VersionProfileReference` refinement, Sprint 9's Repair Recipe authorship conflation), a finished implementation checked against its own architecture document (Sprint 11), and a purely retrospective, historically-derived procedure checked against the very decisions it claims to describe (Sprint 12 — validating a derived model only against summary Sprint labels, rather than every separable decision a label may compress, is itself a real failure mode this project's history has now demonstrated once, worth checking for deliberately in any future historical-evidence derivation).

## The Permanent Engineering Workflow

The canonical Sprint lifecycle, terminology unified project-wide:

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

**Capability Definition** establishes what new capability the work should provide — grounded in product specification and repository evidence — before any architecture is designed. **Architecture Evaluation** is where a genuine architectural question exists: concrete design alternatives are evaluated against repository evidence and established principles, and one is recommended — not yet approved. **Architectural Resolution** marks every open architectural question Accepted, Rejected, or Deferred; nothing may carry forward silently into implementation. **Implementation Authorization** confirms every remaining precondition is satisfied and gives explicit, recorded permission to begin. **Implementation** is the Lead Engineer's work against the authorized scope only. **Validation** is `cargo fmt`/`check`/`test`, both workspaces, zero warnings — run continuously, not solely as one discrete gate. **Implementation Report** is the Lead Engineer's standard report. **Architectural Conformance Review** confirms the actual implementation matches what was resolved and authorized. **Commit** and **Merge** are separately authorized steps. **Repository Closeout** reconciles the repository's authoritative documentation with the implementation.

**Sprint 12 established one further, standing precondition on this workflow, ahead of Capability Definition itself:** a new capability does not proceed to Capability Definition until it has been classified through the Capability Identity procedure (full procedure: `docs/engineering/SPRINT12_ARCHITECTURAL_RESOLUTION.md`) — whether it represents an enrichment of something that already exists, an expansion within an existing architectural axis, or the introduction of an entirely new one. This does not replace Capability Definition; it is now the explicit gate immediately preceding it, so that "does this need a new Collector or Rule at all" is answered from a historically-derived procedure rather than re-argued from first principles each time. This scope is deliberate, not incidental: an already-specified-but-dormant architectural subsystem (`Architecture.md`'s own System Overview) receiving its first real content is not a "new Collector or Rule" question, and does not require classification through this procedure. `SPRINT8_ARCHITECTURAL_RESOLUTION.md` §8 names this shape of work Architectural Activation, applied there to Version Profiles' first real content; Sprint 9 followed the same architectural pattern for Knowledge Base.

## Historical Background

`EngineeringGuide.md`'s own, earlier articulation of the same underlying discipline, retained here for history rather than deleted:

1. Review the relevant specification.
2. Identify affected architectural boundaries.
3. Implement the feature.
4. Write or update tests.
5. Verify consistency with the Engineering Specification.
6. Produce a standard implementation report.
7. Submit for review.

In practice, for anything touching a governance question, a design proposal (or a lighter-weight "preparation" document) precedes implementation — this has held for GOV-007 through GOV-013 without exception, and has never once required post-implementation architectural rework.

## Every Implementation Task Concludes With

`cargo fmt`, `cargo check --workspace`, `cargo test --workspace` — all three, every phase, both the root workspace and `apps/sandbox/src-tauri`'s own separate workspace. Zero warnings is the standing bar.

## Real-I/O Testing Discipline

Established at Sprint 3 Phase 5 and reused, not reinvented, at every subsequent I/O-touching phase (Sprint 4's archive fixtures, Sprint 10's real, captured runtime log fixtures, Sprint 11's Collector/Rule tests constructed against realistic, evidence-grounded content). No mocking of real I/O; real, checked-in or test-constructed fixtures throughout.

## Determinism-by-Content, Not Identity

Universal convention: every Runtime entity's own identity is freshly assigned per construction/evaluation, by design. Determinism is judged by *content and order*, never by incidental identity. Applied without exception since Sprint 2, through all four Rules and all four Collectors.

## Public API Changes

Governed by `GOVERNANCE.md`'s Public API Policy: breaking changes require governance approval. `AssessmentService` has exactly two public entry points (`execute`, `execute_from_assessment_input`), unchanged in signature since introduction, across Sprints 7 through 12, each of which could have pressured a change and did not. GOV-008 remains open, now across nine Sprints.

---

# 6. Architectural Principles Learned

These are not restated from the specifications — they are patterns this project's own history has validated, repeatedly, under real implementation pressure.

**1. Capability before abstraction.** Re-validated repeatedly since first stated: Collector dispatch (GOV-004), the Rule Engine's own internal dispatch (GOV-012), Reporting's scaffold-retirement, the original EngineAPI retirement (ADR-0010), and Collector Composition itself (Sprint 7's own extraction threshold, still not crossed at four Collectors — two content-Collectors composed additively, not yet three). **Sprint 12 gave this principle a concrete, historically-derived decision tool** — a three-axis model (Collection, Evidence, Interpretation) plus an orthogonal Introduction check — for the specific question "does this new capability need a new architectural participant at all," rather than leaving that judgment purely intuitive.

**2. A concrete forcing function, not a hypothetical one, justifies a model change.** Applied to GOV-004, and to GOV-013 (still deferred, now with Sprint 11's own `FindingSeverity::Error` use recorded as evidence for a future review, not acted on prematurely).

**3. Proposal-then-implement, not implement-then-reconcile.** Every governance-relevant capability was designed in a proposal document, approved, and *then* implemented. Zero instances of post-implementation architectural rework are on record — including through Sprints 7 through 12.

**4. Specification-writing, and historical-evidence derivation, are themselves sources of architectural findings.** GOV-013 originated from writing a precise definition. Sprint 12's own Capability Identity procedure originated from — and was then corrected by — deriving a model from historical decisions and checking it against every one of them, not just the ones a summary label made visible.

**5. Determinism is verified directly, never assumed.** Every ordering claim has its own direct test proving it survives varied input arrangement — including, since Sprint 7 and Sprint 11, Collector composition order (structural, then XML, then runtime log) and multi-Rule dispatch order.

**6. Evidence-first, single-channel reporting.** Every fact this platform surfaces to a Rule flows through `Evidence`; every fact a Rule concludes flows through `Finding`/`Recommendation`. No parallel channel has ever been introduced, across nine Sprints of capability growth since this was first stated.

**7. Runtime Entity Design Pattern (ADR-0007) is uniform, not per-entity.** Every Runtime entity and every new opaque reference type (`VersionProfileReference`, Sprint 8) has followed this pattern without a fresh design conversation.

**8. Frozen does not mean immutable — it means change is recorded, not silent.** Multiple Frozen specifications have been amended in place, repeatedly, each with the amendment and its rationale stated directly. `RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md`'s own two in-Sprint amendments (Sprint 11) are the most recent instance of the same discipline applied to an engineering-level architecture document.

**9. Documentation staleness between checkpoints was a real, recurring pattern — and has been solved by discipline, not merely acknowledged.** Stale at every closeout checked through Sprint 6 (four for four). **Every Sprint since Sprint 8 has produced its own Engineering Release and full documentation reconciliation at its own close, without exception** — five consecutive Sprints (8 through 12) now break the pattern the earlier version of this document could only flag as unsolved. This is worth stating plainly: the risk was real, and it was closed by consistently applying the same closeout discipline, not by a process change.

**10. Adversarial verification is a distinct, valuable discipline from good-faith review, and applies to documentation as much as to code.** Sprint 11 confirmed this for finished implementation checked against architecture; Sprint 12 confirmed it again for a purely retrospective procedure checked against the historical record it claims to describe, with zero implementation involved at any point. Three data points now support treating deliberate, adversarial re-checking — not just careful drafting — as its own necessary step before an artifact is trusted, whether that artifact is code, an architecture document, or a planning document.

---

# 7. Sprint History

| Sprint | Delivered | Test Growth |
|---|---|---|
| **Sprint 0** | Workspace scaffolding, nine-crate structure, governance foundation | — |
| **Sprint 1** | First complete deterministic pipeline. Tagged `v0.1.0-alpha`. | → 55 |
| **Sprint 2** | Real field content for Evidence/Finding/Recommendation; Runtime Entity Design Pattern (ADR-0007) established. | → 97 |
| **Sprint 3** | Evidence Collection subsystem boundary (ADR-0008/0009, Documentation Release 2.1) through its first real Collector. | → 112 |
| *(Platform Validation Phase 1)* | GOV-004 resolved; GOV-008 evaluated and deliberately deferred. | — |
| **Sprint 4** | Second real Collector: `ArchiveCollector` (ZIP). GOV-011 resolved. `EvidenceCategory::StructuralDuplication` added. | 112 → 150 |
| **Sprint 5** | Assessment intelligence layer. GOV-012 resolved. GOV-013 opened, deliberately. Second real Rule; multi-Rule `RuleEngine::evaluate`. | 150 → 162 |
| **Sprint 6** | `modiq-cli` wired to `modiq-engine`. `modiq-report`'s four scaffold types retired. | 162 → 172 |
| **Sprint 7** | Third real Collector: `XmlCollector` — first content-inspecting Collector, first multi-Collector Assessment. Collector Composition Architecture approved. Engineering Methodology Version 1.0 declared. | 172 → 187 |
| **Sprint 8** | Version Profile-aware compatibility checking. `modiq-versioning` real for the first time (`VersionProfile::fs25()`). Third Rule (`VersionCompatibilityRule`). `VersionProfileReference` (ADR-0007's pattern, third instance). | 187 → 205 |
| **Sprint 9** | Repair Guidance. `modiq-knowledge` real for the first time (`RepairRecipe`). `RepairRecipeReference` populated for the first time since Sprint 2. | 205 → 210 |
| **Sprint 10** | Runtime Fixture Corpus Acquisition — no Rust source touched. Three real, captured, normalized Farming Simulator runtime log fixtures (`fixtures/runtime-logs/`), grounding all subsequent Runtime Log Interpretation work. | 210 (unchanged) |
| **Sprint 11** | Runtime Evidence Processing Architecture and Implementation. Fourth Collector (`RuntimeLogCollector`) and fourth Rule (`RuntimeLoadFailureRule`) — the platform's first event-based Evidence source, first real use of `FindingSeverity::Error`. A mid-Sprint architectural reconciliation (adversarial verification found and corrected a genuine documentation inconsistency, touching no code). | 210 → 238 |
| **Sprint 12** | Capability Scaling Architecture — architecture-only, no Rust source touched. An explicit Capability Identity procedure derived from seven historical decisions, itself found to contain a contradiction during its own adversarial verification, reconciled to a corrected three-axis model. Direct consequence: Sprint 11 reclassified from Capability Introduction to Capability Expansion (architectural classification only — Sprint 11's own product significance is unchanged). | 238 (unchanged) |

Full detail for each Sprint lives in its own Engineering Release. Every Sprint since Sprint 8 has produced its own release at Sprint close (`ENGINEERING_RELEASE_0.8.md` through `_1.2.md`); Engineering Releases 0.6 and 0.7 were produced retroactively, after a two-Sprint gap, and that gap has not recurred since (Section 6, Principle 9).

**Git tags now exist for Sprints 10, 11, and 12** (`sprint10-complete`, `sprint11-complete`, `sprint12-complete`), pushed to the primary remote — the first Sprints in this project's history to be tagged at all. `v0.1.0`, `v0.1.0-alpha`, `v0.2.0`, `v0.2.0-alpha`, `v0.3.0` remain as pre-existing tags that predate and collide oddly with Engineering Release numbering — a known, still-unresolved git tag hygiene issue, unaffected by the new `sprintN-complete` convention.

---

# 8. Current Repository State

- **Branch:** `feature/runtime-implementation`, tracking `origin/feature/runtime-implementation`, in sync (no ahead/behind).
- **HEAD:** `681dd2e` — "Sprint 12: engineering release and closeout."
- **Working tree:** clean.
- **Tests:** 238/238 root workspace, 7/7 Sandbox, zero warnings in either, verified directly while preparing this document.
- **Crate maturity:** see Section 2's table. `modiq-collection` and `modiq-rules` (four real participants each), `modiq-knowledge` and `modiq-versioning` (L2, minimum-viable real content since Sprints 8–9), and `modiq-cli` (L2) all carry real, tested capability. `modiq-common` remains L1, correctly deferred, zero forcing function across twelve Sprints.

---

# 9. Current Investigations (Open, Unresolved, or Recommended-but-Not-Acted-On)

- **GOV-013 (FindingSeverity Severity/Kind Conflation)** — Open by design. Sprint 11's own `FindingSeverity::Error` assignment is recorded as new, relevant evidence — deliberately not treated as grounds to resolve this now. Will only move when a genuine, demonstrated need arises to express Finding *kind* independent of *severity*.
- **GOV-008 (AssessmentService Public API Evolution)** — Open across nine Sprints. Six further capabilities (Sprints 7 through 12) each reused the existing two-entry-point pattern without generating new pressure toward resolving it.
- **GOV-001, GOV-002, GOV-003** — all Open since Engineering Release v0.1.0-alpha, still pending, none blocking current work.
- **`modiq-engine` does not re-export `AssessmentReport`** — named at Sprint 6 with two data points (Sandbox, `modiq-cli`); no third consumer has been introduced since, so this remains below this project's own usual three-point convergent-evidence bar and remains un-actioned, not neglected.
- **Referential integrity for Finding/Recommendation references** (the GOV-005/GOV-006 cardinality-only follow-up) — remains unassigned to its own Governance Register item, unaffected by Sprints 7 through 12.
- **The Rule Composition question for a second recognized fact within an already-interpreted `EvidenceCategory`** (Sprint 12's own named limitation) — no historical instance yet exists; the Interpretation Axis's own judgment test is a disciplined extrapolation for this case, not a confirmed data point.
- **A documentation citation drift, found and worth correcting:** `COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`, `SPRINT7_CAPABILITY_AND_IMPLEMENTATION_PLAN.md`, and `POST_SPRINT8_CAPABILITY_PRIORITIZATION_STUDY.md` all attribute the phrase "the platform's highest-risk future collector" to `EvidenceCollection.md`, where it does not currently appear. The phrase actually originates in `PROPOSAL_FILESYSTEM_COLLECTION.md`. Recorded here for transparency (Section 11); harmless, not yet corrected.
- **No new capability proposal has yet been evaluated and committed to the repository since Sprint 12's own Capability Identity procedure was established.** Per Sprint 12's own standing rule, Sprint 13 does not exist until one has (Section 10).

---

# 10. Roadmap

**Not yet scoped for Sprint 13, by explicit design.** Sprint 12 established that a future Sprint does not begin until a specific capability proposal has been classified through the Capability Identity procedure (full procedure: `docs/engineering/SPRINT12_ARCHITECTURAL_RESOLUTION.md`) and committed to the repository as such — no capability proposal has reached that state yet. This document deliberately does not name or assume one; doing so here would itself violate the discipline Sprint 12 exists to enforce.

**Longer-term, per `Vision.md`:** cross-version compatibility analysis, community-validated knowledge, Repair Recipe libraries beyond the one that exists, intelligent pattern recognition — all depend on the Knowledge Domain and Version Profile support deepening beyond their current minimum-viable state, neither of which has a forcing function yet (Section 6, Principles 1–2).

---

# 11. Known Documentation Inconsistencies

Recorded for transparency, not urgency — none affects architecture or implementation, and per this project's own discipline, accepted ADRs and Frozen specs are not silently rewritten to fix cosmetic issues.

- **ADR-0002**'s own file title reads "ADR-0001" internally, despite its filename and every external reference treating it as ADR-0002.
- **`Architecture.md`**'s header table and its own Document Status footer disagree by one version number.
- **`VersionProfile.md`** (the filename) internally titles itself "Version Profiles" (plural) in its own metadata table.
- **`docs/governance/ROADMAP.md` and `EngineeringGuide.md`** remain stale since Sprint 0/1 — flagged repeatedly, still not corrected, deliberately out of scope for the sessions that noticed it.
- **`docs/releases/ENGINEERING_RELEASE_0.2.md`** lives in a different directory than every later Engineering Release.
- **A citation drift, found this session:** three separate engineering documents (`COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`, `SPRINT7_CAPABILITY_AND_IMPLEMENTATION_PLAN.md`, `POST_SPRINT8_CAPABILITY_PRIORITIZATION_STUDY.md`) attribute the phrase "the platform's highest-risk future collector" to `EvidenceCollection.md`, where it does not appear in the document's current text. The phrase's real origin is `PROPOSAL_FILESYSTEM_COLLECTION.md`, line 133. A minor, repeated mis-citation, not fabricated evidence — worth correcting at each citing document's own next revision, not urgent enough to warrant its own session.

---

# 12. Where to Go Next

- **New to the project entirely?** Read `Vision.md` → `Principles.md` → `Glossary.md` → `ProductSpecification.md` → `Architecture.md`, in that order.
- **Picking up implementation work?** Read `LEAD_ENGINEER_HANDOFF_v3.0.md` next.
- **Reviewing architecture or governance?** Read `GOVERNANCE.md` (`docs/engineering/`) in full, then the ADRs in numeric order, then `docs/engineering/RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md` and `docs/engineering/SPRINT12_ARCHITECTURAL_RESOLUTION.md` for the two most recent architectural decisions.
- **Acting as Chief Architect?** Read the current `CHIEF_ARCHITECT_HANDOFF_v1.*.md` next — it assumes this document and does not repeat it.
- **Want the full history of any single Sprint?** Its own `ENGINEERING_RELEASE_*.md` is the complete, self-contained record.

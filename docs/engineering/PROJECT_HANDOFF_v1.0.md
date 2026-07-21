# modIQ Project Handoff v1.0

> **The authoritative, role-independent onboarding document for the modIQ repository.**

---

| Property | Value |
|----------|-------|
| **Document** | PROJECT_HANDOFF_v1.0.md |
| **Project** | modIQ |
| **Purpose** | Canonical project handoff — describes the platform, its architecture, its governance, and its history independently of any individual role |
| **Audience** | Anyone onboarding to modIQ — engineering, architecture, or product — before reading a role-specific handoff |
| **Supersedes** | Nothing directly. `LEAD_ENGINEER_HANDOFF_v*.md` and `TECHNICAL_DIRECTOR_HANDOFF_v*.md` remain role-specific supplements; from this version forward, both assume the reader has already read this document and do not repeat what it covers. |
| **As of** | 2026-07-21, following Sprint 5 Closeout (Engineering Release 0.5) |
| **Branch** | `feature/runtime-implementation` |
| **HEAD** | `fbef863` — "feat: implement Sprint 5 assessment intelligence layer (Phases 1-5)" |

---

# How to Use This Document

This is the first document anyone — a new engineer, a new Technical Director session, a reviewer, a future contributor — should read about modIQ. It does not belong to Engineering or to the Technical Director; it describes the project itself: why it exists, how it is organized, how decisions get made, what has been built, and what is still open.

After this document, read the role-specific supplement for your role (`LEAD_ENGINEER_HANDOFF_v3.0.md` or the equivalent Technical Director handoff, once produced). Those documents assume you have read this one and will not repeat its content.

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
| `modiq-cli` | CLI entry point | L1, scaffolded, not wired to `modiq-engine` | `modiq-runtime`, `modiq-engine` |
| `modiq-common` | Shared platform types | L1, empty stub files, zero evidence it's needed after 5 Sprints | (none) |

Dependency direction is strictly downward; `modiq-runtime` is the leaf every other crate ultimately depends on. No circular dependency has ever existed. `apps/sandbox` depends on `modiq-engine` (and, transitively, everything below it) — it never depends on `modiq-collection`, `modiq-rules`, or `modiq-report` directly, preserving `AssessmentService` as the sole point of entry.

**Root workspace: 162 tests, zero ignored, zero flaky.** **Sandbox: 6 tests**, its own separate workspace, independently verified.

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

GOV-013 is worth understanding as a category, not just an entry: it was opened not because implementation broke something, but because *writing a precise specification* (Sprint 5's `FindingSeverity` definitions) surfaced that `BestPractice` classifies Finding *kind*, not *severity*, unlike `Error`/`Warning`/`Informational`. The Technical Director declined to restructure the Runtime type from two Rules' worth of evidence and left it Open by design — see Section 9.

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

## The Proposal → Governance → Documentation → Implementation → Verification Cycle (`EngineeringGuide.md`)

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

**1. Capability before abstraction.** Stated once (Technical Director principle, first applied to the Rule trait question), then independently re-validated at least six times: Collector dispatch (GOV-004), the Rule Engine's own internal dispatch (GOV-012), Reporting's scaffold-retirement question (Sprint 5 Phase 4), and the original EngineAPI/`modiq-rules` service-object retirement (ADR-0010) itself. The test every time: does a *second concrete case* actually exist and actually need the abstraction? If not, the abstraction does not get built, no matter how architecturally clean it would be. Three independent subsystems converging unprompted on direct composition (ADR-0010) is treated as *stronger* evidence than any single design argument.

**2. A concrete forcing function, not a hypothetical one, justifies a model change.** Applied to GOV-004 (three Engineering Releases of non-use before retirement), and applied again, freshly, to GOV-013 (the Technical Director explicitly declined to restructure `FindingSeverity` from two Rules' worth of evidence, deferring to whenever a third Rule actually needs the distinction).

**3. Proposal-then-implement, not implement-then-reconcile.** Every governance-relevant capability (Evidence Collection's boundary, the filesystem Collector, GOV-011's archive policies, GOV-012's Rule Evaluation Model) was designed in a proposal document, approved, and *then* implemented — never the reverse. Zero instances of post-implementation architectural rework are on record as a result.

**4. Specification-writing is itself a source of architectural findings**, not just implementation or testing. GOV-013 is the first item on this project's Governance Register to originate from the act of writing a precise definition (`FindingSeverity`'s semantics) rather than from a bug, a test failure, or an unused-code audit. Expect this again wherever previously-implicit semantics get written down for the first time.

**5. Determinism is verified directly, never assumed.** Every ordering claim (archive entry order, Rule dispatch order, directory traversal order) has its own direct test proving it survives varied input arrangement, not just repeated identical calls. Sprint 5 Phase 5 exists specifically because a repeated-identical-input test had quietly been standing in for an arrival-order-independence test that didn't yet exist.

**6. Evidence-first, single-channel reporting.** When a genuinely new kind of fact needed representing (Sprint 4's duplicate-entry detection), the option of a second, out-of-band reporting channel was explicitly considered and rejected — specifically because it risked reopening GOV-008. Every fact this platform surfaces to a Rule flows through `Evidence`; every fact a Rule concludes flows through `Finding`/`Recommendation`. No parallel channel has ever been introduced.

**7. Runtime Entity Design Pattern (ADR-0007) is uniform, not per-entity.** Aggregate ownership, stable identity, value-object fields, opaque cross-domain references, constructor validation, identity-based equality, aggregate-owned relationship resolution, governance-controlled invariants. Every Runtime entity added since Sprint 2 has followed this pattern without a fresh design conversation.

**8. Frozen does not mean immutable — it means change is recorded, not silent.** Multiple Frozen specifications (`Architecture.md`, `EngineAPI.md`, `EvidenceCollection.md`, `DataModel.md`) have been amended in place, repeatedly, each time with the amendment and its rationale stated directly in the document rather than hidden in a diff.

**9. Documentation staleness between checkpoints is a real, recurring, still-unsolved pattern.** `PROJECT_STATUS.md`/`CHANGELOG.md` have gone stale mid-Sprint at every closeout checked so far (Sprint 3, 4, 5 — three for three). Reconciliation at Sprint close has been reliable; preventing staleness *between* closeouts has not yet been solved, and is currently tracked as an engineering workflow refinement goal ("keep `PROJECT_STATUS.md` current at meaningful milestones"), not a mandatory per-phase rule (the Technical Director specifically declined the latter at Sprint 5 Closeout).

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

Full detail for each Sprint lives in its own Engineering Release (`ENGINEERING_RELEASE_v0.1.0-alpha.md`, `docs/releases/ENGINEERING_RELEASE_0.2.md`, `ENGINEERING_RELEASE_0.3.md`, `_0.4.md`, `_0.5.md`) — each one is also that Sprint's retrospective and completion report combined, per this project's own established convention (no separate retrospective files; one existed only briefly in error — see Section 11).

**No Engineering Release has ever been git-tagged.** `v0.1.0`, `v0.1.0-alpha`, `v0.2.0`, `v0.2.0-alpha`, `v0.3.0` exist as tags, but predate-and-collide oddly with the Engineering Release numbering (a pre-existing git tag hygiene issue, flagged at every release since 0.3, never resolved). `v0.4.0` and `v0.5.0` are both currently available, untagged.

---

# 8. Current Repository State

- **Branch:** `feature/runtime-implementation`, tracking `origin/feature/runtime-implementation`, in sync (no ahead/behind).
- **HEAD:** `fbef863` — Sprint 5's full implementation and closeout, committed and pushed as one commit.
- **Working tree:** clean.
- **Tests:** 162/162 root workspace, 6/6 Sandbox, zero warnings in either, verified directly while preparing this document (not carried over from any prior report).
- **Crate maturity:** see Section 2's table. `modiq-collection` and `modiq-rules` are where real capability has been built (L2/L3 respectively, each recently extended); `modiq-knowledge`, `modiq-versioning`, `modiq-cli`, `modiq-common` remain L1 scaffolding, correctly deferred per Section 6's Principle 1 and 2.

---

# 9. Current Investigations (Open, Unresolved, or Recommended-but-Not-Acted-On)

- **GOV-013 (FindingSeverity Severity/Kind Conflation)** — Open by design. Will only move when a third Rule genuinely needs to express Finding *kind* independent of *severity*. Do not resolve this speculatively; wait for the forcing function.
- **GOV-008 (AssessmentService Public API Evolution)** — Open across three Sprints. The two-entry-point additive pattern (`execute` / `execute_from_assessment_input`) remains the deliberate stopgap. No implementation evidence yet judged sufficient to resolve it (Platform Validation Phase 1 reviewed and declined to resolve it).
- **GOV-001, GOV-002, GOV-003** — all Open since Engineering Release v0.1.0-alpha, all still pending Documentation Release 1.1 or further evidence, none blocking current work.
- **Reporting scaffold retirement** — Sprint 5 Phase 4 recommended retiring `FindingSummary`/`RecommendationSummary`/`TraceabilityReport`/`ReportFormatter` (zero construction sites, ever; the Sandbox's own rendering needed no changes to display Sprint 5's new severity). Recommendation accepted as eligible, **pending formal governance approval** — not yet acted on. A real, scoped, low-risk item for a near-future sprint.
- **Documentation-staleness workflow refinement** — tracked as a goal (keep `PROJECT_STATUS.md` current at meaningful milestones), explicitly *not* a mandatory per-phase rule. Three consecutive Sprints (3, 4, 5) have shown the pattern recurring between closeouts even with reconciliation happening reliably at closeout itself.
- **Known, harmless documentation inconsistencies** (see Section 11) — flagged, not fixed, since fixing them wasn't in scope for whichever session first noticed each one, and none affects architecture or implementation.

---

# 10. Roadmap

**Not yet scoped for Sprint 6.** Three candidates are on record, none committed to:

1. **XML inspection** — the next Evidence Collector in `PROPOSAL_FILESYSTEM_COLLECTION.md`'s own original sequencing (filesystem → archive → XML). Explicitly deferred through Sprint 5 specifically so it would build on a mature Rule Engine rather than drive its design — that condition is now satisfied.
2. **CLI wiring** — `modiq-cli`'s `AssessCommand` scaffold, wired to `modiq-engine`. Independent, low-risk, and more de-risked every Sprint the Sandbox's own thin-client pattern (`AssessmentService::execute_from_assessment_input`, called directly, no reimplementation) is reused rather than reinvented.
3. **Acting on the Reporting scaffold-retirement recommendation** — small, scoped, low-risk, currently unscheduled.

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

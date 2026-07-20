# ADR-0010: Engine Orchestration Simplification

| Property | Value |
|----------|-------|
| **ADR** | 0010 |
| **Title** | Engine Orchestration Simplification |
| **Status** | Accepted |
| **Project** | modIQ |
| **Date** | 2026-07-20 |

---

# Context

`EngineAPI.md`, frozen at Documentation Release 1.0, described the Engine API as exposing five conceptual services: an Assessment Service, and four supporting services — Knowledge Service, Rule Evaluation Service, Reporting Service, Version Profile Service — each with its own Responsibility, Capabilities, and Depends On sections, cooperating through a Service Relationships diagram showing the Assessment Service coordinating the other four.

At Sprint 0, before any real subsystem existed, this model was scaffolded directly into source: `modiq-engine` received four one-line unit structs (`KnowledgeService`, `ReportingService`, `RuleEvaluationService`, `VersionProfileService`), and `modiq-rules` received a mirrored set (`RuleSelector`, `EvidenceEvaluator`, `Explainability`, `Traceability`) reflecting `RuleEngine.md`'s six conceptual responsibilities. Both sets of scaffolding were reasonable at the time: `EngineAPI.md` was the only description of the engine's shape that existed, and building toward it was the only available interpretation of "architecture translated into module structure" (`CrateRoadmap.md`'s own Phase 1 definition).

GOV-004 was raised at the platform's first Engineering Release, asking directly whether `AssessmentService` should continue orchestrating the pipeline itself, or transition to these specialized services. It remained open through three Engineering Releases while `AssessmentService` was, in fact, built out and repeatedly extended — first for the Rule Engine and Reporting (Sprint 1), then for Evidence Collection (Sprint 3) — without the question ever being revisited.

`PLATFORM_VALIDATION_GOV-004.md` (Platform Validation cycle, following Engineering Release 0.3) evaluated the resulting three Sprints of implementation evidence directly. `PROPOSAL_GOV-004.md` translated the Technical Director's review of that evidence into an architectural direction. This ADR records the resulting decision as a durable architectural artifact, per `docs/adrs/README.md`'s guidance that decisions altering ownership boundaries and affecting multiple platform components require an ADR rather than living only in a proposal document.

---

# Decision

## Why the internal service model originally existed

`EngineAPI.md` was written before any real subsystem existed to test it against. Its five-service model was a reasonable conceptual elaboration of `Architecture.md`'s System Overview — which already showed the Assessment Service delegating to supporting subsystems — describing that delegation at a finer grain: one dedicated service per subsystem, each with its own capability contract. The Sprint 0 scaffolding built directly against this model, in both `modiq-engine` and `modiq-rules`, on the reasonable assumption that a Frozen specification's described shape was the shape to build toward.

## What implementation demonstrated

Across three Engineering Releases, three independently introduced real subsystems — the Rule Engine (Sprint 1), Reporting (Sprint 1), and Evidence Collection (Sprint 3) — were each wired into `AssessmentService` by direct instantiation of that subsystem's own real type, at different times, under different immediate pressures, with no coordination between those individual engineering decisions beyond consistent judgment applied each time. None of the four `EngineAPI` stub services, and none of the four mirrored `modiq-rules` stub submodules, was ever constructed, called, or tested, in any commit, from the moment they were created. `RuleEngine::evaluate` — the one real, tested Rule in the platform — independently arrived at the same shape: it fulfills selection, evaluation, Finding/Recommendation generation, and traceability inline, without delegating to the four dedicated types built for exactly that purpose.

Separately, `Architecture.md` — the higher-authority specification, which `EngineAPI.md` itself names as taking precedence in the event of conflict — was found, on direct re-reading, to already describe the shape implementation converged on: its System Overview depicts the Assessment Service delegating directly to Evidence Collection, Rule Engine, Knowledge Base, Version Profiles, Reporting, and Storage as subsystems, not as a further layer of intra-engine service objects distinct from those subsystems' own types.

## Why it was retired

No concrete forcing function for the four-service model arrived in three Engineering Releases: nothing in the platform has ever needed to swap a Rule Engine implementation, expose `modiq-engine` over a remote or process boundary, support a plugin surface, or mediate access to Knowledge or Version Profile data through a dedicated service object. Continuing to carry unused scaffolding without such a case would contradict this platform's own repeatedly validated engineering discipline — see below. Three independent subsystems choosing the same alternative, unprompted, is stronger evidence for retiring the untaken path than a single implementation attempt would have been.

## Why `AssessmentService` remains the orchestration boundary

`AssessmentService` already does, structurally, exactly what `Architecture.md`'s System Overview always described: it coordinates the Assessment lifecycle and delegates specialized responsibility to each supporting subsystem, owning none of the state, rule logic, or report-generation logic it orchestrates (`GOVERNANCE.md`, Engine Crate Boundary Rules). Retiring the internal service model does not change this boundary in any way — it removes an unused indirection layer that sat inside it, not the boundary itself. `AssessmentService`'s public methods (`execute`, `execute_from_assessment_input`) are unaffected by this decision.

## Why this aligns with the project's engineering philosophy

"A capability should justify an abstraction, not the reverse" has already been tested twice under real pressure to build the abstraction anyway — a Rule trait, a Collector trait — and declined both times, with no loss of correctness. The internal service model is the same pattern, discovered later only because it predates the discipline that would now reject it: an abstraction built ahead of any concrete need, never activated by three subsequent, independent opportunities to activate it. Retiring it is not a new judgment; it is the same judgment this project has already made twice, applied consistently to scaffolding that happened to be older than the discipline itself.

---

# Consequences

Benefits:

- Removes eight structurally identical, permanently inert files across two crates, and the scaffold/reality divergence risk they created for any future contributor reading `EngineAPI.md` literally.
- Reconciles `EngineAPI.md` with `Architecture.md`, closing a latent disagreement between a higher- and lower-authority specification that had existed, unexamined, since Documentation Release 1.0.
- Resolves GOV-004 explicitly, after three Engineering Releases open, consistent with this platform's preference for explicit resolution over indefinite accumulation.

Trade-offs:

- Removes a design option — an intra-engine service indirection layer — that has never been proven unnecessary in any scenario requiring it, only unnecessary in the scenarios the platform has faced so far. If a future scenario genuinely needs that seam (an alternate Rule Engine implementation, a remote boundary, a plugin surface), it will need to be reintroduced rather than already existing.
- This is the first substantive amendment to `EngineAPI.md` since Documentation Release 2.0, and the first time this platform has reconciled a Frozen conceptual specification against three Sprints of accumulated implementation evidence rather than against a single implementation attempt.
- Deletion of the eight stub types is not performed by this ADR. Until that implementation work is separately authorized and completed, the source and the amended specification will briefly disagree in the opposite direction — the code will contain scaffolding the specification no longer describes — rather than the reverse.

---

# Relationship to Other Specifications

This decision is reflected in:

- `docs/architecture/EngineAPI.md` — amended (v1.1.0) to describe the Assessment Service and direct subsystem composition; the four retired services and the Service Relationships diagram removed.
- `docs/architecture/Architecture.md` — unchanged in substance; a cross-reference to this ADR added to the Assessment Service component description, confirming its System Overview diagram as the accurate, higher-authority description this ADR validates rather than supersedes.
- `GOVERNANCE.md` — GOV-004 marked Resolved; the Engine and Rule Engine Crate Boundary Rules entries cross-reference this ADR.
- `docs/implementation/CrateRoadmap.md` — `modiq-engine` and `modiq-rules` maturity entries note the retirement.
- `PLATFORM_VALIDATION_GOV-004.md` and `PROPOSAL_GOV-004.md` — the evaluation and proposal this ADR formalizes; retained as historical records, not superseded or rewritten.
- ADR-0007 (Runtime Entity Design Pattern) — unaffected; this decision concerns engine orchestration, not Runtime entity design.

---

# Status

Accepted as an architectural and governance decision.

Deletion of the eight retired stub types (`modiq-engine`: `knowledge_service.rs`, `reporting_service.rs`, `rule_evaluation_service.rs`, `version_profile_service.rs`; `modiq-rules`: `selector.rs`, `evaluator.rs`, `explainability.rs`, `traceability.rs`) is **not** authorized by this ADR and requires separate Technical Director review and approval before any implementation begins.

# Capability Definition — C1: RecommendationStep Presentation

| Property | Value |
|---|---|
| **Document** | CAPABILITY_DEFINITION_C1_RECOMMENDATION_STEP_PRESENTATION.md |
| **Project** | modIQ |
| **Type** | Capability Definition — defines the capability only. Not an Architecture Evaluation, Architectural Resolution, Implementation Authorization, Sprint Plan, or roadmap. |
| **Origin** | `docs/engineering/CAPABILITY_PORTFOLIO_ASSESSMENT.md` (`ca11328`), which classified this candidate ("C1") Category A and recommended it as the first post-foundation engineering milestone. Treated as fixed, unreopened. |

---

## Purpose

This document answers one question: what capability is being added to the platform? It does not decide how that capability is implemented, whether it requires a new Architecture Evaluation, what Rust or TypeScript types would carry it, what the transport or serialization shape would be, what a Sprint Plan would contain, or in what order any of that work would occur. Those determinations belong to whatever repository artifact follows this one.

---

## Repository Context

`docs/engineering/PLATFORM_FOUNDATION_V1_DECLARATION.md` and `docs/engineering/POST_FOUNDATION_ENGINEERING_CAPABILITY_ASSESSMENT.md` are treated as settled: stable Runtime, Knowledge, Rule, Collector, Storage, and Frontend architecture, and a mature implementation lifecycle. `docs/engineering/CAPABILITY_PORTFOLIO_ASSESSMENT.md` surveyed twelve capability candidates against that settled architecture and identified this one ("C1") as the strongest — lowest architectural risk, real and traceable value, and dependent on no unresolved governance or architecture question. This document is the first artifact produced for that candidate.

The capability sits at the end of a specific, three-Sprint lineage: Sprint 22 gave `Finding` and `Evidence` real field anatomy and exposed it through `apps/console` at Sprint 23; Sprint 24 gave `Recommendation` a comparable real-content addition — `RecommendationStep`, a Runtime-owned, structured projection of `RepairRecipe`'s per-step guidance — but did not extend `apps/console` to present it, naming that explicitly, in its own Implementation Report, as "a real, evidence-backed future opportunity, requiring its own, separate Implementation Authorization."

---

## Capability Statement

**The platform's production interaction layer (`apps/console`) presents the structured, per-step repair guidance the Runtime already attaches to a Recommendation, in addition to the flat guidance text it already presents today.**

Confirmed directly, this session, against current source: `Recommendation` carries `repair_steps: Vec<RecommendationStep>` (`crates/modiq-runtime/src/assessment/recommendation.rs`), each step exposing a `kind` (`RecommendationStepKind`: `XmlChange`, `LuaChange`, `DependencyInstallation`, `AssetReplacement`, `VersionUpdate`) and an `instruction` (`crates/modiq-runtime/src/assessment/recommendation_step.rs`). `apps/console`'s current transport (`apps/console/src-tauri/src/assessment.rs`) and TypeScript mirror (`apps/console/src/engine/types.ts`) carry only `recommendation: string | null` — the flat `action` text. `apps/console/src/workspace/Reviewing.tsx` renders exactly that one string ("Recommendation: {finding.recommendation}") and nothing else. The structured content Sprint 24 built has no path to any user today.

---

## User Value

A user viewing a Finding whose Recommendation was informed by a Repair Recipe (currently, `VersionCompatibilityRule`'s declared-version-mismatch guidance) sees one undifferentiated sentence. The Runtime already knows, per step, what *kind* of change that step describes — an XML edit, a Lua change, a dependency installation, an asset replacement, or a version update — a distinction `RuleEngine.md`'s own Recommendation Generation section already specifies ("Associated Repair Recipes inform Recommendation content") but that no consumer currently surfaces. The user-visible problem this capability solves is exactly that gap: real, already-computed structure exists and is currently invisible.

---

## Existing Repository Foundation

- **`RecommendationStep`/`RecommendationStepKind`** (`modiq-runtime`, Sprint 24) — the structured content itself, populated by `VersionCompatibilityRule` at construction time from real `RepairRecipe` content, verified zero-leak across the Runtime/Knowledge boundary.
- **`PersistedRecommendation`'s `repair_steps` mirror** (`modiq-storage`, Sprint 24) — confirms the content survives persistence, though this capability does not require persistence to be involved.
- **`RuleEngine.md`'s own specification** establishes that Repair Recipes inform Recommendation generation — a Runtime-layer principle already satisfied since the flat `Recommendation.action` implementation (Sprint 8/9), and unrelated to frontend presentation. `RuleEngine.md` does not establish or foreshadow structured presentation of `RecommendationStep` content in `apps/console`; this capability's own presentation grounding comes instead from Sprint 24's implementation, Sprint 24's own Implementation Report, and the confirmed current transport and `Reviewing.tsx` presentation gap.
- **The Sprint 22 → Sprint 23 precedent** — the same shape of work (Runtime gains structured content; `apps/console` is later extended to present it) has been performed once already, with the second instance requiring, per `ENGINEERING_RELEASE_1.8.md` §6, "zero reinterpretation" of the governing frontend architecture.
- **`DataModel.md`'s own Recommendation section**, read directly for this document, still describes only `action`-level content and does not yet mention `RepairRecipe`-derived structure at all — the specification-level documentation has not caught up to Sprint 24's implementation, a documentation-currency fact relevant to, but not resolved by, this capability.

---

## Existing Architectural Principles

- **ADR-0007 (Runtime Entity Design Pattern).** `RecommendationStep` is a plain, infallibly-constructed value with no identity — this capability presents that value's own already-fixed content; it does not reinterpret, recompute, or infer anything about it.
- **`FrontendArchitecture.md`'s Presentation Without Inference.** The consumer presents what the engine produces; it does not evaluate or conclude. Presenting `repair_steps` is squarely presentation of already-produced Runtime content.
- **`FrontendArchitecture.md`'s Explainable Continuity.** Whatever is presented must be traceable to something the engine actually produced — `repair_steps` is exactly that, with no consumer-introduced fact.
- **`FrontendArchitecture.md`'s Consumer-Owned State.** Presentation state remains the consumer's own; this capability does not touch Assessment state, Workspace state, or navigation state.
- **`FrontendArchitecture.md`'s Boundary Enforcement.** `assessment.rs` remains the only module depending on a `modiq-*` crate; `engine/index.ts` remains the only file calling `invoke`.

---

## Existing Repository Patterns

- **The Transport Mirror pattern** — `apps/console`'s Rust and TypeScript DTOs mirror Runtime's already-public getters field-for-field, demonstrated at Sprint 21, 22, and 23.
- **The existing single expansion-layer presentation pattern** — Sprint 23 absorbed new per-Finding and per-Evidence content into the presentation layer's already-existing expansion, without introducing a new navigation level.
- **The `Debug`-format string convention** already used for `severity`, `mod_health_dimension`, and `status` — an established precedent for presenting a Runtime enum as text.

---

## Architectural Boundaries

The following must remain unchanged by this capability, as a direct consequence of already-adopted architecture:

- Boundary Enforcement's two named crossing points (`assessment.rs`; `engine/index.ts`).
- `Recommendation.action`'s own meaning and content — unchanged, supplemented only, per Sprint 24's own Authorization.
- `RepairRecipeReference` — remains permanently unresolved; this capability presents `repair_steps`, not a resolution mechanism for the reference.
- The Runtime/Knowledge boundary — no `modiq-knowledge` type may be referenced outside its own crate or the one existing authorized seam (`modiq-rules`); this capability is presentation-only and touches neither crate.
- The three Reserved Frontend responsibilities (`FrontendArchitecture.md`) — Workspace-state derivation, post-completion Report supplementation, and the concrete request/response payload contract — none is implicated by presenting already-transported-in-principle Runtime content.

---

## Explicit Exclusions

Intentionally outside this capability's own scope:

- Any Rust type, TypeScript interface, serialization shape, or transport design.
- Any UI layout, visual design, or interaction design beyond what `FrontendArchitecture.md` already governs.
- Any Sprint Plan, phase breakdown, or sequencing.
- Any change to `modiq-runtime`, `modiq-rules`, `modiq-storage`, or any other `modiq-*` crate — Sprint 24's own domain and persistence work is complete and frozen.
- Any resolution mechanism for `RepairRecipeReference`.
- `apps/sandbox` or `modiq-cli` parity (the Capability Portfolio Assessment's own C4 and C3, separately deferred candidates).
- `Confidence`, cross-mod dependency resolution, or any other Capability Portfolio Assessment candidate.
- Any correction to `DataModel.md`'s own currently-incomplete Recommendation section.

---

## Capability Success Criteria

A user viewing a Finding whose Recommendation carries a non-empty `repair_steps` can see each step's `kind` and `instruction`, distinguishably from the existing flat `action` text, sourced from real Runtime content with no fact introduced by the consumer.

---

## What Success Does Not Require

- It does not require every Recommendation to carry `repair_steps` — only `VersionCompatibilityRule` currently populates it; every other Rule's Recommendation carries an empty `Vec`, and presenting "no structured steps" is itself a valid, expected outcome, not a gap to fill.
- It does not require resolving `RepairRecipeReference`.
- It does not require any new navigation, region, or interaction mechanism.
- It does not require `apps/sandbox` or `modiq-cli` to change.
- It does not require `DataModel.md` to be amended.

---

## Repository Impact

At the capability-definition level, not the implementation level: this capability implicates `apps/console` only. No `modiq-*` crate, no `apps/sandbox`, no `Cargo.toml`, no ADR, and no Governance Register item is implicated by the capability itself, matching the Capability Portfolio Assessment's own Category A classification for this candidate.

---

## Next Required Repository Artifact

Repository precedent offers two structurally different paths for the artifact that follows a Capability Definition, distinguished by whether the specific work still raises an architectural question `FrontendArchitecture.md` does not already, non-discretionarily, answer: Sprint 21's presentation work proceeded through a full Architecture Evaluation and Architectural Resolution; Sprint 23's — the closest precedent to this capability, extending existing transport with already-produced Runtime content — proceeded directly to an Implementation Authorization, because Consumer-Owned State already covered it in full.

Whether this capability matches Sprint 23's precedent closely enough to take the same path, or instead raises a question `FrontendArchitecture.md` does not yet answer, is not decided by this document — that determination, and whichever of an Architecture Evaluation or an Implementation Authorization it leads to, is the next required repository artifact.

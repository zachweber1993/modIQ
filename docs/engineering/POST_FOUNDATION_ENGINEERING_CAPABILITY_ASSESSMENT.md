# Post-Foundation Engineering Capability Assessment

| Property | Value |
|---|---|
| **Document** | POST_FOUNDATION_ENGINEERING_CAPABILITY_ASSESSMENT.md |
| **Project** | modIQ |
| **Type** | Repository Capability Assessment — not an Architecture Evaluation, Sprint Plan, Capability Definition, or roadmap. |
| **Author** | Chief Architect / Technical Director |
| **Origin** | `docs/engineering/PLATFORM_FOUNDATION_V1_DECLARATION.md` (`5ab2224`), treated as fixed, unreopened determination. |

---

## Purpose

This document determines what classes of engineering work the repository can now undertake as a direct consequence of Platform Foundation Version 1 having been declared complete. It classifies, from repository evidence alone, which architectural questions no longer need to be revisited, which implementation patterns are now reusable, which categories of capability can be built against existing architecture, and which categories still require architecture first.

This is not a priority ordering, a roadmap, a Sprint recommendation, or a Capability Definition for any specific candidate. It records what is now architecturally possible, not what should happen next.

---

## Repository Evidence Reviewed

- `docs/engineering/PLATFORM_FOUNDATION_V1_DECLARATION.md` and its own Findings (F1–F14)
- `docs/adrs/0001` through `0011`
- `docs/engineering/GOVERNANCE.md` — full Governance Register (GOV-001 through GOV-017), all nine Crate Boundary Rules
- `docs/architecture/Architecture.md`, `RuleEngine.md`, `EvidenceCollection.md`, `EngineAPI.md`, `DataModel.md`, `KnowledgeModel.md`, `FrontendArchitecture.md`
- `docs/engineering/SPRINT12_ARCHITECTURAL_RESOLUTION.md` (Capability Identity: the three-axis classification model and the Introduction/Expansion/Enrichment test, re-validated against seven historical decisions)
- `docs/governance/PROJECT_STATUS.md` — Engineering Alignment Program and Frontend Architecture narrative sections
- Current repository source: crate dependency graph (`Cargo.toml`, every crate's own `[dependencies]`), `crates/modiq-rules/src/rules/`, `crates/modiq-collection/src/collection/` (Collector count and composition style)

---

## Permanently Settled Foundations

Questions future work no longer needs to revisit, because each has been resolved, exercised across multiple independent Sprints, and never reopened:

- **Assessment as sole aggregate root** (ADR-0003). Structurally unchanged since its adoption; ADR-0011 corrected only an unrelated documentation example, not the role itself.
- **Runtime entity/value design** (ADR-0007). Identity-bearing entities use fallible constructors and generated identity; opaque or nested values use infallible constructors and no identity. Unmodified since Sprint 2, governing every new Runtime type added since, including Sprint 24's `RecommendationStep`.
- **Crate dependency direction and independence.** `modiq-knowledge` has zero outbound dependencies and exactly one inbound dependency (`modiq-rules`); `modiq-runtime` has zero dependency on `modiq-knowledge`; Reporting performs no analysis; Storage sits strictly downstream and never feeds back upstream. All nine current Crate Boundary Rules have held, unmodified, since Sprint 13.
- **Engine orchestration model** (ADR-0010, GOV-004, Resolved). Direct composition of each subsystem's own real type; no engine-local service-object layer. Unrevisited since Sprint 3.
- **Rule Engine composition model** (GOV-012, Resolved). `RuleEngine::evaluate` returns `Vec<RuleOutcome>`; fixed, explicit declaration order; independent composition with no suppression model. Exercised by four Rules without amendment since Sprint 5.
- **Collector composition model** (`EvidenceCollection.md`, GOV-009/010/011). Inline, deterministic routing; no registry, dispatcher, or trait. Two composition styles — mutually-exclusive and additive — both confirmed, twice each, across four Collectors.
- **The Runtime/Knowledge crossing mechanism.** Exactly one legitimate seam (`modiq-rules`), exercised twice (`RepairRecipeReference`, Sprint 2; `RecommendationStep`, Sprint 24), zero-leak both times.
- **The Storage mirror mechanism.** A one-way, Storage-owned representation built from a Runtime type's already-public getters, never reconstructing a live Runtime value. Exercised three times (Sprint 13, 22, 24).
- **Frontend Boundary Enforcement and Consumer-Owned State** (Initiative 5 Architectural Resolution; `FrontendArchitecture.md`). The consumer never evaluates, infers, or mutates Assessment state; presentation, interaction, and navigation state are consumer-owned. Adopted once, exercised twice (Sprint 21, 23) without reinterpretation.
- **Capability Identity classification** (Sprint 12, corrected). Three independent axes (Collection, Evidence, Interpretation) plus an orthogonal Introduction/Expansion/Enrichment test, re-validated against seven historical decisions with zero contradiction.
- **The implementation lifecycle's own shape.** Architecture Evaluation → Architectural Resolution → Implementation Authorization → Sprint Plan → Phased Implementation → Engineering Release/Closeout, exercised across every major decision since Sprint 3; a dedicated Implementation Audit stage was added, and demonstrated once, at Sprint 24.
- **Evidence-based governance escalation.** An item opens on real tension, resolves only when evidence supports a decision, and otherwise stays explicitly Open and revisited rather than forced closed. Demonstrated across all 17 Governance Register items.

---

## Established Engineering Patterns

Patterns demonstrated by repeated repository history, available for reuse without re-deriving them:

- **Adding Runtime field content.** ADR-0007's constructor-validated shape, applied to `Finding`/`Evidence` (Sprint 22) and `Recommendation` (Sprint 24) independently.
- **Adding a Collector.** Collector Contract compliance (deterministic, factually accurate, boundary-respecting) plus a Collection-Axis classification (Enrichment if the existing mechanism suffices, Expansion if not) and a composition-style choice (mutually-exclusive or additive), both already twice-confirmed. Demonstrated at Sprint 3, 4, 7, 11.
- **Adding a Rule.** The Interpretation Axis's two-stage filter — category match, then content-shape match — used internally by every category-specific Rule to date (`StructuralDuplicationRule`, `VersionCompatibilityRule`, `RuntimeLoadFailureRule`), composing independently under GOV-012's fixed dispatch order. Demonstrated at Sprint 5, 8, 9, 11.
- **Projecting Knowledge into Runtime.** An opaque reference (`RepairRecipeReference`) or a structured, independently-typed projection (`RecommendationStep`), populated only at the one legitimate seam. Demonstrated at Sprint 2 and Sprint 24.
- **Mirroring into Storage.** A Storage-owned type populated one-way from already-public getters. Demonstrated at Sprint 13, 22, 24.
- **Mirroring into transport.** `apps/console`'s Rust and TypeScript DTOs mirror Runtime's already-public getters field-for-field, never transporting a live Runtime type. Demonstrated at Sprint 21, 22, 23.
- **Extending frontend presentation.** New transported fields absorbed into an existing expansion layer without a new navigation level. Demonstrated once, at Sprint 23.
- **The full architecture-to-implementation lifecycle**, including, since Sprint 24, an independent Implementation Audit that re-derives claims from source rather than trusting phase reports.
- **Deferring under insufficient evidence**, rather than forcing a decision. Demonstrated repeatedly (GOV-001, GOV-008, GOV-013).

---

## Capability Classes Now Enabled

Categories of work with an already-adopted, exercised construction pattern to build from — classification, not endorsement, and not a determination that any future instance is exempt from Architecture Evaluation. Whether a specific future instance still requires its own Architecture Evaluation, Architectural Resolution, or Capability Definition remains a case-by-case determination under the existing Architectural Review Process (`GOVERNANCE.md`) and, where applicable, Capability Identity classification (Sprint 12). This section identifies only that a construction pattern already exists to build against — future work begins from existing precedent rather than a blank architectural foundation — not that the decision to build is itself pre-authorized:

- **Richer assessment output within existing entities.** ADR-0007's constructor-validated pattern for adding field-level content to `Finding`, `Evidence`, or `Recommendation` is settled and reusable, demonstrated at Sprint 22 and Sprint 24 — each of which nonetheless underwent its own Architecture Evaluation and Architectural Resolution before implementation. The pattern settles how such content is constructed once authorized; it does not establish that authorizing new content bypasses Architecture Evaluation. Bounded by the Aggregate-Owned Relationship Resolution's own permissive model (see below) — this category covers field-content construction, not new cross-entity validation.
- **Additional engineering knowledge.** New `RepairRecipe` content, new `RepairStep` instructions, or richer Rule-side interpretation within an already-recognized `EvidenceCategory`, since `modiq-knowledge` requires no crate or boundary change to accept new authored content (Sprint 9, Sprint 22 precedent).
- **New Rule families that classify as Expansion or Enrichment.** A Rule interpreting an already-recognized category via the established two-stage filter, or a new judgment over a dormant category not yet interpreted, composing independently under GOV-012's existing dispatch model. The Sprint 12 Rule Guidance names one specific, real limit on this: no historical decision has yet tested two Rules sharing one already-interpreted category, so that specific configuration is "strong guidance, not settled precedent."
- **New Collectors that classify as Expansion or Enrichment.** A new inspection mechanism composing via one of the two already-confirmed styles (mutually-exclusive or additive), so long as the Collector Composition Architecture's own five-condition extraction threshold is not crossed.
- **Persistence enhancements via the Storage Mirror pattern.** The one-way, already-public-getter-sourced mirror shape is settled and reusable — demonstrated at Sprint 13 (founding, its own Architecture Evaluation), and extended at Sprint 22 and Sprint 24, each within a broader Architecture Evaluation covering the Runtime change being mirrored. The pattern settles how a new mirror is constructed once the Runtime content it mirrors is authorized; it does not itself exempt that authorization from Architecture Evaluation.
- **Presentation extensions within Consumer-Owned State's already-adopted scope.** New transported fields, rendered within the existing expansion layer, so long as none of the three named Reserved responsibilities (Workspace-state derivation, post-completion Report supplementation, the concrete request/response payload contract) is touched.
- **Reporting-layer formatting or organization changes.** `modiq-report`'s own boundary ("performs no analysis... its responsibility is presentation") permits reorganizing or reformatting already-produced Findings and Recommendations without new architecture.
- **User workflow work within `apps/console`'s already-realized responsibilities.** Application Shell composition, and navigation realized under the Single Continuous Assessment principle already adopted and exercised once (Sprint 21).

---

## Capability Classes Still Requiring Architecture

Categories that cross an architectural boundary the repository has not yet settled, each supported by direct evidence:

- **A new domain or subsystem.** The Extension Layer (`Architecture.md`) has zero implementation and no composition or dispatch relationship has ever been exercised for it; under Sprint 12's own Introduction test, any first realization of it is Capability Introduction, requiring a founding Architecture Evaluation.
- **Assessment lifecycle changes.** Initiative 1 (Progressive Execution Observability) and Initiative 2 (Reentrant Assessment Lifecycle) both remain unresolved at the Architectural Resolution stage or beyond — Initiative 2's own Governance Reconciliation found its central question "cannot presently be reconciled under the repository's current governance model." Any progressive-visibility or reentrancy work requires that resolution first.
- **New orchestration or dispatch machinery.** Introducing a registry, trait, plugin mechanism, or coordinator for Rules or Collectors — explicitly excluded by ADR-0010, GOV-004, and `EvidenceCollection.md`'s own Collector Contract — or crossing the Collector Composition Architecture's five-condition extraction threshold.
- **Cross-entity relationship validation.** GOV-005 and GOV-006 both resolved cardinality only; referential-integrity validation (whether a referenced id actually resolves) was explicitly excluded from each resolution and "remains open, to be raised as a separate future governance item" — a governance action that has not yet occurred. Enforcing it would change Aggregate-Owned Relationship Resolution's own permissive model (ADR-0007).
- **`AssessmentService`'s public API shape.** GOV-008 remains Open; any change to the two public entry points, `AssessmentInput`, `AssessmentReport`, or the public error model requires that resolution first.
- **Governance-verification standards.** GOV-016 (the evidentiary standard for treating a governance decision as repository fact) is itself unresolved at the constitutional level and must proceed through its own Architecture Evaluation and Architectural Resolution before any dependent principle is adopted.
- **Confidence as a first-class concept.** Its Architectural Resolution adopted scope and one timing constraint but left concrete representation and computation mechanism "genuinely undesigned" — both require dedicated design work before implementation, not extension of any existing pattern.
- **Lua Analysis.** GOV-014 requires its own governance resolution (fixture provenance, licensing, storage policy) before an Architecture Evaluation for Lua Analysis may even be authorized.
- **`AssessmentSubject`/`AssessmentContext` real content, and Report Identity.** Both remain zero-field placeholder types; Item 6a's own connection to GOV-002's prior "require no action" disposition is named as a candidate for reconsideration, not decided.

---

## Repository Precedent

**Architectural precedent** — binding shape for future work unless new evidence justifies deviation:
ADR-0007's entity/value pattern; the single-seam Runtime Projection pattern; the one-way Storage Mirror pattern; the field-for-field Transport Mirror pattern; the Rule Engine's fixed-order, no-suppression composition model; the Collector Contract and its two confirmed composition styles; Frontend's Boundary Enforcement and Consumer-Owned State.

**Engineering precedent** — standard practice for how work is designed and verified:
Capability Identity's three-axis classification, applied before implementation begins, not after; real-fixture-only testing with no mocking; per-phase validation gates within a Sprint Plan; deferring a decision under insufficient evidence rather than forcing one; re-deriving verification claims at closeout rather than trusting earlier phase reports, now including a dedicated Implementation Audit stage.

**Repository process precedent** — how decisions and their records are produced:
The Architecture Evaluation → Architectural Resolution → Implementation Authorization → Sprint Plan → Implementation → Engineering Release/Closeout lifecycle; naming an excluded or reserved responsibility explicitly rather than omitting it silently; naming a discovered documentation inconsistency and deferring its correction to its own dedicated pass rather than fixing it inline. A standalone Milestone Declaration, reviewed by a dedicated Falsification Review before becoming permanent history, is now part of repository history as its own artifact type distinct from a Sprint or an Engineering Release — a validation sequence available to follow should a comparable milestone ever be declared again, not an expected recurring Sprint-closeout activity (see Non-Precedent Activities, below).

---

## Non-Precedent Activities

Repository events that remain exceptional, not templates for routine future practice:

- **Retroactive Sprint recognition.** Sprint 20 was implemented and committed before being recognized as a Sprint at all. This is recorded as a one-time historical condition, not a pattern for building work ahead of its own Sprint scoping.
- **The Sprint 15–19 governance-cycle labeling, and its own still-unresolved contradiction** between `PROJECT_STATUS.md` and `CHANGELOG.md`. This is named repository debt awaiting its own dedicated reconciliation, not a numbering convention to be repeated.
- **`modiq-common`'s retirement (GOV-003).** A one-time removal of a pre-provisioned, always-empty crate. Not a precedent for routinely pre-provisioning or speculatively retiring crates.
- **ADR-0011's ownership correction.** A superseding-ADR mechanism used once, to correct a documentation example inside an already-Accepted ADR. Not a general license to reopen accepted architecture; the correction mechanism itself (supersede, don't amend in place) is process precedent, but the act of correcting ADR-0003 is not repeated practice.
- **GOV-017's convergent-evidence analysis.** Established that a shared crate dependency is not, by itself, convergent evidence unless it arises from consuming another crate's API rather than declaring a type in one's own. This reasoning is available for reuse; it did not lower or alter GOV-004's own three-point convergence bar, which remains the standing rule.
- **The Repository Falsification Review and Platform Foundation Declaration process.** Fact: this process has occurred exactly once, at a determined architectural inflection point. Interpretation, not repository fact: this reads as an exceptional, milestone-scoped mechanism rather than a routine Sprint-closeout activity, consistent with the Declaration's own Purpose language ("a significant architectural milestone"). Repository history establishes only that it has occurred once; it does not itself establish that the process must never recur.
- **Foundation-era naming without foundation-era exercise.** The Extension Layer and Confidence are named in adopted specifications but have never been built against; their eventual first realization is Capability Introduction under Sprint 12's own test, not an extension of any pattern this assessment lists as established.

---

## Overall Assessment

The repository's settled architecture now supports a defined, evidence-bounded set of capability classes — richer entity content, additional Knowledge authoring, Expansion/Enrichment-classified Rules and Collectors, Storage and transport mirroring, bounded presentation extension, and Reporting-layer formatting — each traceable to a pattern exercised more than once and never contradicted. An equally defined set of categories remains gated behind unresolved architecture: a new domain, lifecycle reentrancy or progressive observability, new orchestration machinery, cross-entity referential integrity, the public API's own evolution, governance-verification standards, Confidence, Lua Analysis, and `Assessment`'s own still-empty input types. Neither list is exhaustive of every possible future question; both are exhaustive of what current repository evidence actually classifies.

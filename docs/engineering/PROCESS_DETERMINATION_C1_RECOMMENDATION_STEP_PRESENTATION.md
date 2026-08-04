# Process Determination — C1: RecommendationStep Presentation

| Property | Value |
|---|---|
| **Document** | PROCESS_DETERMINATION_C1_RECOMMENDATION_STEP_PRESENTATION.md |
| **Project** | modIQ |
| **Type** | Repository Process Determination — decides which repository artifact follows the Capability Definition. It does not authorize implementation, evaluate architecture, design a transport shape, or define a Sprint. |
| **Origin** | `docs/engineering/CAPABILITY_DEFINITION_C1_RECOMMENDATION_STEP_PRESENTATION.md` (`d5b8735`), §"Next Required Repository Artifact," which states this determination explicitly has not yet been made and names it as the next required artifact. |

---

## Purpose

This document answers one question, and only one: does C1 follow Sprint 23's precedent (Implementation Authorization directly, no Architecture Evaluation or Architectural Resolution) or Sprint 21's precedent (Architecture Evaluation and Architectural Resolution first)? Both paths are real, committed repository precedent, and the Capability Definition names both without choosing between them.

This determination is answered by applying the same test the repository already used, once, to make this exact choice — not a newly invented one.

---

## Sprint 21 and Sprint 23, Compared

`PROJECT_HANDOFF_v1.1.md` §5 defines one canonical engineering workflow (Capability Definition → Architecture Evaluation → Architectural Resolution → Implementation Authorization → …). The repository does not formally recognize two standing procedural tracks. Sprint 21 and Sprint 23 are each a historically documented, individually justified application of that one workflow under different circumstances; they are compared here because each is the closest committed precedent for a different set of circumstances, not because either constitutes a separate governance-recognized path.

**Sprint 21** (`FrontendArchitecture.md` (Approved) → Frontend Implementation Readiness Assessment → Frontend Architecture Architectural Resolution → `FRONTEND_IMPLEMENTATION_AUTHORIZATION.md`) required Architecture Evaluation and Architectural Resolution because the work itself established the engine/consumer boundary mechanism, region composition, the navigation model, and Consumer-Owned State as adopted architecture for the first time. There was no governing specification yet to check the work against — the work *was* the specification being settled.

**Sprint 23** (`FRONTEND_PRESENTATION_IMPLEMENTATION_AUTHORIZATION.md`, directly, no Evaluation or Resolution) proceeded without either stage, justified by a two-part test stated explicitly in that document's own §7:

1. **No genuinely open design question exists.** The governing principle is already *Adopted*, not deferred — `FrontendArchitecture.md`'s Consumer-Owned State and Explainable Continuity already, non-discretionarily, settle what a consumer may display about an already-completed Assessment, provided the content is already engine-produced.
2. **No alternative is being weighed.** The document authorizes exposing already-produced data; it does not decide how the engine/consumer boundary itself should work.

Sprint 23's document states the consequence of both prongs holding directly: *"an Architecture Evaluation exists in this repository's vocabulary to gather and classify evidence toward a genuinely open design question. There is none here."*

---

## A Lighter Historical Route, Considered and Not Applicable

`FRONTEND_PRESENTATION_IMPLEMENTATION_AUTHORIZATION.md` line 124 records that Sprint 23's own precedent review considered and rejected a third, lighter route it names as "the lighter Sprint 9/Sprint 20 path" — its full reasoning was not separately committed, but its existence is repository-acknowledged, so it is examined here rather than silently omitted a second time.

Checked directly against primary source: Sprint 9 (`SPRINT9_CAPABILITY_DEFINITION.md`, `SPRINT9_ARCHITECTURAL_RESOLUTION.md`) proceeded from Capability Definition straight to Architectural Resolution, with no standalone Architecture Evaluation and no standalone Implementation Authorization document. Sprint 20 (commit `ebc10c5`) is the specific subject of `GOVERNANCE_OBSERVATION_EBC10C5_AUTHORIZATION_EVIDENCE.md`, which found no independent repository evidence corroborating that commit's own claimed Chief Architect authorization, and internally contradictory authorization-status statements within the same commit — a documented governance irregularity, not neutral precedent.

Neither instance is applicable to C1. C1 already has a committed, reviewed Capability Definition, and that document's own "Next Required Repository Artifact" section frames the open choice as Architecture Evaluation versus Implementation Authorization — not as an option to bypass both. Sprint 23 remains the governing comparison for C1.

---

## Applying the Test to C1

**Prong 1 — is there a genuinely open design question, or is the governing principle already Adopted?**

- `repair_steps: Vec<RecommendationStep>` is already engine-produced content (`crates/modiq-runtime/src/assessment/recommendation.rs`), populated by `VersionCompatibilityRule` at construction time — not a fact the consumer would introduce. Explainable Continuity is satisfied by construction, identically to Sprint 23's Evidence fields.
- Consumer-Owned State already, non-discretionarily, settles that what an already-complete Report displays is the consumer's own decision, provided nothing is inferred. Presenting `kind`/`instruction` per step reorganizes and formats already-produced content; it concludes nothing the engine did not already conclude.
- Checked directly against `FrontendArchitecture.md`'s Reserved Responsibilities: Initiative 1 (Assessing/Reviewing derivation) and Initiative 2 (post-completion/reentrant presentation) both concern *when* or *whether* a Report is live or evolving. C1 concerns only *what* an already-completed Report displays — identical to Sprint 23's own basis for finding neither reservation applicable.
- GOV-008 reserves the request/response contract's concrete payload shape. Sprint 23 already established that ordinary field extension to the existing provisional `ReportSummary`/`FindingSummary` DTOs is "anticipated evolution," not a new architectural event, citing `assessment.rs`'s own header comment. Nothing about C1 distinguishes it from that same characterization.

One distinction was checked: `repair_steps` would require a *nested* DTO (an array of `{kind, instruction}` objects), not a flat scalar field like Sprint 22/23's `mod_health_dimension` or `status`. `EvidenceSummary`, established at Sprint 21 under the same governing architecture, is repository precedent for transporting a nested, structured object as a new field — it is not precedent for the more specific transformation C1 would need, which is evolving an existing scalar field (`FindingSummary.recommendation: Option<String>`, currently populated from `Recommendation::action()` alone) into a richer presentation structure. That specific transformation has no direct prior instance and is a transport-shape design question. Designing that shape is out of scope for this document (see "What This Document Does Not Do") and belongs to the Implementation Authorization or Implementation Plan; it does not reopen Prong 1, since Consumer-Owned State and Explainable Continuity govern what the consumer may display, independent of the DTO's internal shape.

A second distinction was checked: `RecommendationStepKind` is a five-variant enum with no prior presentation precedent of its own. This is also already covered — the `Debug`-format string convention already used for `severity`, `mod_health_dimension`, and `status` is established repository precedent (named directly in `CAPABILITY_DEFINITION_C1...md` §"Existing Repository Patterns") for presenting a Runtime enum as text, with no new rendering mechanism required.

**Prong 2 — is any alternative being weighed?**

No. Nothing about C1 proposes a different shape for the engine/consumer boundary, a different state-ownership model, or a different navigation mechanism. It exposes already-computed data through the already-adopted Transport Mirror pattern (Sprint 21, 22, 23) and the already-adopted single-expansion-layer presentation pattern (Sprint 23). There is no decision here to disposition between alternatives — only whether to extend an existing, already-authorized mechanism.

---

## Determination

**C1 matches Sprint 23's path on both prongs of the test.** No genuinely open design question exists — the governing principle (`FrontendArchitecture.md`'s Consumer-Owned State and Explainable Continuity) is Adopted, not deferred, and is satisfied by construction. No alternative is being weighed.

**The next required repository artifact is an Implementation Authorization**, not an Architecture Evaluation or Architectural Resolution.

---

## What This Document Does Not Do

- It does not draft the Implementation Authorization itself.
- It does not define implementation boundaries, participating crates, exclusions, invariants, verification gates, or completion criteria — those belong to the Implementation Authorization.
- It does not design a transport shape, a DTO, a UI treatment, or a Sprint Plan.
- It does not reopen, revise, or reinterpret `FrontendArchitecture.md`, Initiative 5's Architectural Resolution, or any other adopted determination — every conclusion above traces to a determination already Adopted before this document existed.

---

## Note on This Document's Own Standing

Repository precedent for this exact stage (Sprint 23's own path-selection reasoning) was performed as an uncommitted "Procedural Validation," cited by title only in the Implementation Authorization that followed it, and never separately committed as its own file — the same pattern Sprint 24's Architecture Evaluation and Architectural Resolution followed. This document is committed as its own standalone artifact instead, per explicit direction for this milestone. This is a higher evidentiary bar than repository precedent strictly required for this stage, not a deviation from it.

---

## Next Required Repository Artifact

An Implementation Authorization for C1 — RecommendationStep Presentation, following the same procedural shape `FRONTEND_PRESENTATION_IMPLEMENTATION_AUTHORIZATION.md` followed for Sprint 23.

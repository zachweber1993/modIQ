# Engineering Release 0.9

| Property | Value |
|---|---|
| **Release** | 0.9 |
| **Documentation Release** | 2.1 (Frozen — unchanged this Sprint; no specification amendment) |
| **Milestone** | Sprint 9 complete (Repair Guidance: minimum viable `modiq-knowledge` activation; Repository Closeout) |
| **Scope** | The platform's first real Knowledge Domain implementation; the first `RepairRecipeReference` ever populated with `Some(...)` instead of `None`; a second, independent confirmation that ADR-0007's Opaque Runtime References pattern generalizes without modification |
| **Predecessor** | `docs/engineering/ENGINEERING_RELEASE_0.8.md` (Sprint 8) |
| **Governing ADRs** | None new — this Sprint applies ADR-0007's existing Opaque Runtime References pattern rather than establishing a new durable principle, consistent with `SPRINT9_ARCHITECTURAL_RESOLUTION.md`'s own explicit conclusion that no decision here extends beyond Sprint 9's scope |
| **Governing Plan** | `SPRINT9_CAPABILITY_DEFINITION.md`, `SPRINT9_ARCHITECTURAL_RESOLUTION.md`, `SPRINT9_REPOSITORY_REVIEW.md` |

---

## Executive Summary

Engineering Release 0.8 closed by naming Repair Guidance as the clearest concrete Sprint 9 candidate — `RepairRecipeReference` had existed, real and tested, since Sprint 2, wired through `Recommendation::new`'s constructor, and had never once been populated with anything but `None`; `modiq-knowledge` had gone eight Sprints with zero implementation and zero forcing function.

Sprint 9 did not begin with implementation. Capability Definition (`SPRINT9_CAPABILITY_DEFINITION.md`) verified `modiq-knowledge`'s zero-implementation state directly against source, confirmed no repository drift since Sprint 8's own closeout, and scoped the capability to its smallest complete form: one existing Rule, one named Repair Recipe, no Knowledge Base access mechanism. Architectural Resolution (`SPRINT9_ARCHITECTURAL_RESOLUTION.md`) resolved five open questions — which Rule, where a `RepairRecipe` is retrieved, its minimum field shape, whether `GOVERNANCE.md` needed amendment, and whether `RepairRecipeReference` needed to change. During Chief Architect review of that resolution, a conflation was found and corrected: the initial draft had the consuming Rule both retrieve *and author* Repair Recipe content, which would have made `modiq-rules` the author of engineering knowledge rather than its consumer — a direct violation of `modiq-knowledge`'s own README boundary. The resolution was revised in place, before implementation began, so that `modiq-knowledge` authors the specific recipe as a named constructor and `modiq-rules` only calls it.

`RepairRecipe` gained real content for the first time since Sprint 0: `identifier`/`guidance` fields, an infallible constructor, and one named, authored recipe, `RepairRecipe::version_compatibility_declared_version_mismatch()`, mirroring `VersionProfile::fs25()`'s exact precedent. `VersionCompatibilityRule` now calls this recipe instead of authoring guidance text inline, wiring a real `Some(RepairRecipeReference)` into its `Recommendation`. `RuleEngine::evaluate` and `AssessmentService`'s two public entry points required **zero signature change** — the capability's entire footprint is one new dependency edge, `modiq-rules` → `modiq-knowledge`.

---

## Scope of Sprint 9

### Delivered

- **Capability Definition.** `SPRINT9_CAPABILITY_DEFINITION.md` — the capability (activating `modiq-knowledge`'s first real content, consumed by exactly one existing Rule through the already-existing `RepairRecipeReference` seam), its rationale (`Vision.md`'s own least-served founding Assessment question, "what can I do next?"), and five Architectural Considerations named explicitly for Architectural Resolution to answer, not assumed.
- **Architectural Resolution.** `SPRINT9_ARCHITECTURAL_RESOLUTION.md` resolved all five questions against repository evidence (the `RuleReference`-vs-`VersionProfile` contrast already present in `version_compatibility_rule.rs`; `VersionProfile::fs25()`'s own "authored in the domain crate, called by consumers" precedent; `modiq-knowledge`'s own README boundary statement) — each with a decision, rationale, repository precedent, alternatives considered, and why each alternative was rejected. Amended in place following Chief Architect review to correct the Question 2 conflation described above.
- **Implementation.** `modiq-knowledge` (`RepairRecipe`: fields, constructor, accessors, named recipe); `modiq-rules` (`Cargo.toml` new dependency; `VersionCompatibilityRule` calling the named recipe instead of authoring guidance inline; two existing tests updated, not left stale); `modiq-engine` (one end-to-end test assertion added, no production code changed).
- **Validation.** `cargo fmt`, `cargo check --workspace`, `cargo clippy --workspace --all-targets`, `cargo test --workspace`, both workspaces, zero warnings, performed both during implementation and again independently during Repository Review.
- **Repository Review.** `SPRINT9_REPOSITORY_REVIEW.md` — independent re-verification of every resolved decision against the actual diffs (not against the implementation session's own claims), confirming zero deviation, recommending Approve for Commit.
- **Commit.** Six implementation files, dependency updates, and associated tests, plus (by explicit Chief Architect authorization) this Sprint's own planning/decision record, committed together as `21eb7eb` on `feature/runtime-implementation`.
- **Repository Closeout.** Full re-validation, documentation synchronization, this Engineering Release produced at Sprint 9's own Closeout (continuing Sprint 8's own corrected practice), a second commit for the Closeout documentation itself — no separate Sprint 9 feature branch, mirroring Sprint 7's and Sprint 8's own precedent.

### Deliberately Not Delivered

- `modiq-knowledge`'s remaining six Knowledge Model categories (`Rule`, `Engine Behavior`, `Compatibility Pattern`, `Best Practice`, `Known Issue`, `Knowledge Reference`) — one minimum-viable `RepairRecipe`, by explicit scope.
- A second Rule consuming Repair Guidance — `StructuralDuplicationRule` remains without one; `VersionCompatibilityRule` was the sole Sprint 9 consumer.
- Any Knowledge Base access mechanism (lookup, registry, query) — a single named, hardcoded recipe, exactly as minimal as `VersionProfile::fs25()`.
- Any change to `RuleEngine::evaluate`'s signature, `AssessmentService`'s two public entry points, or `RepairRecipeReference`'s shape.
- A new Governance Register item, ADR, or `modiq-knowledge` Crate Boundary Rule — none was needed; the Knowledge Domain boundary section in `GOVERNANCE.md` already named Repair Recipes explicitly before this Sprint began.

---

## Major Architectural Accomplishments

- **A design conflation was caught and corrected during Chief Architect review of the Architectural Resolution, before any implementation code was written.** The initial draft answered "where is a `RepairRecipe` retrieved" and "where is it authored" as if they were the same question; they are not, and the Rule-internal-authorship answer would have violated `modiq-knowledge`'s own README boundary. The resolution document was revised in place — decision, rationale, alternatives, and every downstream section — rather than patched with a caveat, so implementation proceeded against one internally consistent record.
- **ADR-0007's Opaque Runtime References pattern was confirmed to generalize a third time, with zero modification.** `RepairRecipeReference`, real and untouched since Sprint 2, required no shape change to accommodate its first real content — the same pattern `RuleReference` and `VersionProfileReference` already established.
- **The `VersionProfile::fs25()` precedent (Sprint 8) was recognized and correctly applied to a second domain.** A named, self-contained, domain-authored constructor — not a caller-supplied literal — is now a repeated, confirmed shape for "give a scaffold crate its first real content," not a one-off pattern specific to Version Profiles.
- **The smallest architecture capable of delivering the capability required zero change to any public API above `modiq-rules`.** No new `RuleEngine::evaluate` parameter, no new `AssessmentService` construction responsibility, no new crate — the smallest public-surface footprint of any capability shipped since Sprint 3.
- **A four-stage Capability Definition → Architectural Resolution → Implementation → Repository Review sequence was completed for the third time** (following Sprint 7's establishment and Sprint 8's second use), each stage producing its own reviewed, evidence-grounded document.

---

## Major Implementation Accomplishments

- `modiq-knowledge` gained real content for the first time since Sprint 0 — an eight-Sprint-old scaffold crate becoming real, tested code.
- `RepairRecipeReference` was populated with `Some(...)` for the first time since its introduction at Sprint 2 — the platform's oldest-standing dormant scaffolding finally activated.
- `modiq-rules` gained its second new dependency (`modiq-knowledge`, alongside Sprint 8's `modiq-versioning`) without any change to `VersionCompatibilityRule::evaluate`'s public signature.
- The root workspace test suite grew from 205 to 210 tests: `modiq-knowledge` 0 → 5 (its first tests ever), `modiq-rules` and `modiq-engine` unchanged in count (both extended in place: a stale `None` assertion corrected to the real `Some(...)`, a determinism assertion extended, an end-to-end pipeline assertion added).
- The Sandbox's own separate test suite required zero modification and remains unchanged at 7/7 — confirming `AssessmentService`'s public entry points were genuinely untouched, the same outcome Sprint 8 achieved.
- `cargo clippy --workspace --all-targets` run for the first time as part of this Sprint's own Repository Review, surfacing exactly one warning — pre-existing, in a file this Sprint did not touch — confirming zero new lint debt from this Sprint's own changes.

---

## Governance Completed

No Governance Register item was opened and no ADR was created this Sprint — unlike Sprint 8, no gap was found to fill:

| Item | Status | Notes |
|---|---|---|
| `modiq-knowledge` Crate Boundary Rules | Already adequate, confirmed | `GOVERNANCE.md`'s Knowledge Domain section already named "Repair Recipes" explicitly and already stated the crate "must remain independent from any individual Assessment" — verified directly, not assumed, at both Architectural Resolution and Repository Review. |
| Repair Recipe retrieval/authorship split | Recorded as a Sprint-specific Architectural Resolution decision (`SPRINT9_ARCHITECTURAL_RESOLUTION.md`), not a Governance Register item | Mirrors Sprint 7's and Sprint 8's own treatment of comparably-scoped decisions — implementation evidence precedes formal governance codification, not the reverse. |

**Still open:** GOV-001, GOV-002, GOV-003, GOV-008 (now aging across seven Sprints, unaffected by this Sprint — `AssessmentService`'s entry points required no signature change, generating no new evidence toward it), and GOV-013 (untouched — no Rule's severity/kind classification changed). Total Governance Register size: 13 items, 8 Resolved, 5 Open — unchanged in count from Engineering Release 0.8.

---

## Documentation Completed

- **Three new Sprint-specific documents**: `SPRINT9_CAPABILITY_DEFINITION.md`, `SPRINT9_ARCHITECTURAL_RESOLUTION.md` (amended in place following Chief Architect review, not superseded by a second document), `SPRINT9_REPOSITORY_REVIEW.md`.
- **Repository Closeout**: `PROJECT_STATUS.md` (header fields, new `## Sprint 9 — Complete` section, Current Focus and Governance Status notes updated), `CHANGELOG.md` (new `# [Sprint 9]` entry), `ENGINEERING_LOG.md` (Sprint 9 Closeout entry, plus a new "Engineering Methodology Observations" sub-entry recording the Architectural Resolution correction as a second, independent data point for the "dedicated review stages find real errors" observation Sprint 8 first recorded), `CrateRoadmap.md` (Implementation Status table — `modiq-knowledge` L1 → L2 — dependency-edge note, Sprint 9 narrative section, revision history entry 1.21.0), `docs/README.md` (Engineering Release cross-reference updated to 0.9).

---

## Testing Growth

| Milestone | Root Workspace Tests | Sandbox Tests |
|---|---|---|
| Engineering Release 0.8 | 205 | 7 |
| End of Sprint 9 implementation | 210 | 7 |
| Engineering Release 0.9 (this release) | **210** | **7** |

By crate, at this release: `modiq-runtime` 84, `modiq-collection` 57, `modiq-engine` 19 unit + 3 integration, `modiq-report` 3, `modiq-rules` 25, `modiq-cli` 10, `modiq-versioning` 4, `modiq-knowledge` 5. Zero tests ignored or flaky anywhere in the workspace, verified by direct execution both continuously during implementation and again independently during Repository Review and this Closeout.

---

## Repository Maturity Assessment

| Area | Classification | Basis |
|---|---|---|
| Workspace organization | **Stable** | Nine crates, unchanged in count since Engineering Release 0.3 — the new capability was added within existing crates. |
| Crate boundaries | **Stable, and confirmed rather than merely preserved.** `modiq-knowledge`'s own README boundary ("knowledge is authored here, not by Rules") was directly tested by this Sprint's own initial design error and held — the correction, not a boundary exception, is what shipped. |
| Dependency direction | **Stable, one new internal edge, zero new external dependency.** `modiq-rules` → `modiq-knowledge`, a sibling-to-sibling edge, confirmed via `cargo tree` to introduce no cycle. |
| Documentation consistency | **Maintained.** Engineering Release produced at Sprint 9's own Closeout, continuing the corrected practice Sprint 8 established; no new documentation staleness found during this Sprint's own Repository Review. |
| Engineering workflow | **Exercised for the third time, and demonstrated its value directly.** The Architectural Resolution review step — not a formality — caught a real design error before implementation, the second independent instance of this class of finding (after Sprint 8's `VersionProfileReference` refinement). |
| Governance and evidentiary discipline | **Reinforced.** No governance gap was found this Sprint, in contrast with Sprint 8 — direct evidence that `GOVERNANCE.md`'s Knowledge Domain section was already correctly scoped ahead of this Sprint's own forcing function arriving. |

---

## Crate Maturity Review

| Crate | Maturity | Remaining Work | Architectural Confidence | Priority |
|---|---|---|---|---|
| `modiq-knowledge` | **L1 → L2**, 0 → 5 tests | Six remaining Knowledge Model categories unimplemented; no second Repair Recipe yet | High — the crate's first real content required no redesign of its already-specified conceptual model (`KnowledgeModel.md`) | Medium — real, but minimum-viable; no urgency beyond this Sprint's own scope |
| `modiq-rules` | L3, 25 tests, unchanged in count | A fourth Rule; `StructuralDuplicationRule` still has no Repair Recipe of its own | High — the second new dependency required no change to any Rule's public signature | Medium — the Rule Engine's own depth continues to grow incrementally, as designed |
| `modiq-runtime` | L3, 84 tests, unchanged | `Display`/`Serialize` for identity/enum types — flagged for a ninth consecutive release | High — `RepairRecipeReference` required zero modification, confirming ADR-0007's pattern a third time | Low — stable |
| `modiq-collection` | L2, 57 tests, unchanged | Dependency *resolution*, Lua/localization/texture/store-asset collectors, all still deferred | High — unaffected by this Sprint | Medium — unchanged from Engineering Release 0.8's own assessment |
| `modiq-engine` | L3, 19 unit tests, 3 integration, unchanged in count | The `AssessmentReport` re-export gap (Engineering Release 0.6) remains untouched | High — zero production-code change this Sprint, confirmed directly by the diff | Low — stable |
| `modiq-versioning` | L2, 4 tests, unchanged | `Capability`/`Compatibility` remain unimplemented; a second Version Profile has no forcing function yet | High — unaffected by this Sprint | Medium — unchanged from Engineering Release 0.8 |
| `modiq-report` | L3, 3 tests, unchanged | `AssessmentReport` does not yet expose which Version Profile was active — a real, named Known Limitation | High | Low — small, low-risk future addition |
| `modiq-cli`, `apps/sandbox` | L2 / real, respectively — both unchanged in maturity level and source code | None blocking | High — both already render a Recommendation's `repair_recipe_reference()` generically, with zero code change required | Low — stable |
| `modiq-common` | L1, empty stub files, unchanged since Sprint 0 | Undecided | Low — zero evidence of need after nine Sprints; `modiq-knowledge`'s own new content confirms this again (it depends on nothing, including `modiq-common`) | Low — do not invent a use for it |

---

## Technical Debt Review

### Intentional Technical Debt
*(deliberate, documented, correct to leave as-is for now)*

- Exactly one named `RepairRecipe` exists — deliberately, mirroring `VersionProfile::fs25()`'s own "one real value, no selection mechanism yet" acceptance.
- No Knowledge Base access mechanism (lookup, registry, query) — building one now would be speculative extensibility with no second consumer to justify it.
- `StructuralDuplicationRule` has no Repair Recipe of its own — by explicit Sprint 9 scope, not oversight.

### Future Capabilities
*(deferred by design, not yet started, no urgency implied)*

- A second Rule (most likely `StructuralDuplicationRule`) consuming its own named Repair Recipe.
- `modiq-knowledge`'s remaining six categories, each awaiting its own forcing function — none should be built speculatively, per this project's own repeatedly-applied "capability before abstraction" discipline.
- Runtime Log Interpretation — named in the frozen roadmap (`SPRINT_ROADMAP_UPDATE_v1.md`) as the Sprint 10 candidate, contingent on this Sprint's own successful closeout (now complete).

### Known Limitations
*(real, current, will not resolve without deliberate work)*

- Missing `Display`/`Serialize` for Runtime identity/enum types — now flagged in nine consecutive release records.
- `AssessmentReport` does not expose which Version Profile was active (Sprint 8's own named limitation, unaffected by this Sprint).
- Git tag hygiene remains unresolved: `v0.4.0` through `v0.8.0` are all available untagged; `v0.9.0` is now also available.

### Deferred Enhancements
*(nice-to-have, no urgency, no forcing function yet)*

- Whether `docs/implementation/ImplementationWorkflow.md` should be deleted outright rather than archived (named at Sprint 7, still open).
- Whether `EngineeringGuide.md`'s remaining "Technical Director" reference should be reconciled (named at Sprint 7, still open).
- `CrateRoadmap.md`'s "Exit Criteria" section still has no entry for any Sprint since Sprint 2.
- `docs/governance/ROADMAP.md`'s Phase-based roadmap remains stale relative to actual repository state (last describes "Phase 3 — Sprint 1, In Progress") — a pre-existing staleness predating this Sprint, confirmed out of scope rather than corrected here, since Sprint 9 did not cause it and correcting it was not evaluated against fresh evidence this session.

---

## Sprint 9 Retrospective

**What went well.** The Architectural Resolution review step did exactly the job this project's workflow reserves it for: it caught a genuine design error — Rule-authored knowledge content — before a single line of implementation code existed, and the correction was made by revising the resolution document itself rather than patching around the error.

**What surprised us.** How directly analogous this Sprint's own design error was to a precedent already sitting in the repository (`VersionProfile::fs25()`'s "authored in the domain crate, called by its consumer" shape) — the correction did not require inventing a new pattern, only recognizing that an existing one already applied and had been overlooked in the first draft.

**Architectural decisions validated by implementation.** All five Architectural Resolution decisions were confirmed by implementation with zero deviation — Repository Review's own independent re-verification, re-run from source rather than trusting the implementation session's account, found nothing to correct.

**Areas requiring future attention.** GOV-008 has now aged across seven Sprints. `modiq-knowledge`'s remaining six categories each still lack a forcing function. `StructuralDuplicationRule` remains the clearest candidate for a second Repair Recipe consumer, should Sprint 10 or a later Sprint choose to extend this capability rather than begin an unrelated one.

---

## Remaining Risks

- **GOV-008 aging across seven Sprints untouched** — still functioning as a stopgap, still not resolved either way.
- **One named Repair Recipe is not yet a realistic test of whether the "domain-authored, Rule-called" pattern generalizes to a second consumer.** A second Rule or a second recipe would be the first real evidence either way.
- **`modiq-knowledge`'s remaining six Knowledge Model categories could, if approached without care, repeat the same authorship conflation this Sprint's own initial draft made** — Sprint 9's own corrected resolution is now the concrete precedent to reference, not merely a written principle.
- **`docs/governance/ROADMAP.md`'s staleness, though pre-existing and out of this Sprint's scope, continues to accumulate** — a reminder that documentation drift between Closeouts remains a real, standing risk (`PROJECT_HANDOFF_v1.0.md`, Section 6, Principle 9) beyond what any single Sprint's diligence resolves.

---

## Lessons Learned

- **A dedicated Architectural Resolution review step, exercised for its own sake rather than as a formality, found a real error a single drafting pass did not.** This is the second independent instance of this class of finding, following Sprint 8's `VersionProfileReference` refinement — two data points now support treating this review step as consistently valuable, not merely a process nicety.
- **Correcting a resolution document in place, rather than appending a caveat, keeps the repository's own record of "what was decided" singular and unambiguous.** Every downstream section (Type Design, Data Flow, Implementation Plan, Risks) was revised alongside the corrected decision itself, so implementation had one coherent document to follow, not a decision plus a patch to reconcile against it.
- **An existing precedent (`VersionProfile::fs25()`) generalized cleanly to a second domain (`modiq-knowledge`) once correctly recognized** — further evidence that this repository's "give a scaffold crate its first real content via one named, domain-authored constructor" pattern is a durable, reusable shape, not specific to Version Profiles.
- **Producing the Engineering Release at Sprint close remains achievable on a second consecutive Sprint** — Engineering Release 0.7's own named risk (a two-Sprint-running late-production pattern) continues not to recur.

---

## Engineering Metrics

| Metric | Value |
|---|---|
| Workspace crates | 9 (unchanged since Engineering Release 0.3) |
| Governance items | 13 total — 8 Resolved, 5 Open — unchanged in count from Engineering Release 0.8 |
| Documentation Release | 2.1, Frozen — unchanged this Sprint |
| Engineering Release | 0.9 (this document, produced at Sprint 9's own Closeout) — not tagged; `v0.9.0` available |
| Root workspace tests | 210 (up from 205 at Engineering Release 0.8) |
| Sandbox tests | 7 (unchanged) |
| Engineering Methodology Version | 1.0 — unchanged this Sprint |
| Major milestones completed | Documentation Releases 1.0, 2.0, 2.1; Sprint 0–8; Engineering Releases v0.1.0-alpha, 0.2–0.8; Platform Validation Phase 1; Sprint 9 (Repair Guidance); Sprint 9 Closeout |
| Implementation readiness | Fully ready for the next engineering objective. |

---

## Repository Timeline

```
Documentation Release 1.0
        ↓
Sprint 0
        ↓
Sprint 1
        ↓
Engineering Release v0.1.0-alpha
        ↓
Sprint 2
        ↓
Engineering Release 0.2
        ↓
Sprint 3 (Phases 1–5)
        ↓
Documentation Release 2.1
        ↓
Engineering Release 0.3
        ↓
Platform Validation Phase 1 — GOV-004 resolved; GOV-008 deferred
        ↓
Sprint 4 (Phases 1–3D, plus Closeout)
        ↓
Engineering Release 0.4
        ↓
Sprint 5 (Phases 1–5, plus Closeout)
        ↓
Engineering Release 0.5
        ↓
Sprint 6 — CLI wiring; modiq-report scaffold retirement; Technical Director → Chief Architect
        ↓
Sprint 6 Closeout
        ↓
Engineering Release 0.6
        ↓
Sprint 7 — Multi-Source Evidence Collection (XML inspection); Engineering Workflow Consolidation
        ↓
Sprint 7 Closeout — Engineering Methodology Version 1.0 declared
        ↓
Engineering Release 0.7
        ↓
Sprint 8 — Version Profile-aware compatibility checking
        ↓
Sprint 8 Closeout
        ↓
Engineering Release 0.8
        ↓
Sprint 9 — Repair Guidance (minimum viable modiq-knowledge activation)
        ↓
Sprint 9 Closeout
        ↓
Engineering Release 0.9
        ↓
Sprint 10 (not yet scoped for implementation; Runtime Log Interpretation named as the frozen roadmap's candidate)
```

---

## Recommendation

**Sprint 10 is ready to be scoped.** The platform's first real Knowledge Domain implementation is live, tested, and validated end to end; `RepairRecipeReference` — dormant since Sprint 2 — is now real; both workspaces are fully green.

**What Sprint 10 should contain is not decided here**, consistent with this project's own capability-first discipline. The frozen roadmap (`SPRINT_ROADMAP_UPDATE_v1.md`) already names Runtime Log Interpretation as the Sprint 10 candidate, its own architectural workflow question (bundled submission, Option A) already resolved by prior Chief Architect decision — but this release record treats that as an input to Sprint 10 Capability Definition to confirm against fresh evidence, not a decision this document re-makes.

**One process item continues to deserve attention.** GOV-008 has now aged across seven Sprints. This release does not recommend resolving it reactively, but does recommend that Sprint 10 scoping explicitly ask whether its own capability is likely to finally produce the evidence needed — rather than let it continue by default, as every Sprint since Sprint 3 has.

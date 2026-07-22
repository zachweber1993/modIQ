# Engineering Release 0.7

| Property | Value |
|----------|-------|
| **Release** | 0.7 |
| **Documentation Release** | 2.1 (Frozen — unchanged this Sprint; no specification amendment) |
| **Milestone** | Sprint 7 complete (Multi-Source Evidence Collection: XML inspection; Engineering Workflow Consolidation; plus Closeout) |
| **Scope** | The platform's first content-inspecting Collector (`XmlCollector`); the first Assessment in this platform's history to run more than one Collector; nine independent workflow descriptions across the repository consolidated into one canonical source; the engineering methodology itself designated Version 1.0 |
| **Predecessor** | `docs/engineering/ENGINEERING_RELEASE_0.6.md` (Sprint 6) |
| **Governing ADRs** | None — the approved Collector composition model applies existing principles (capability before abstraction, the Collector Contract's existing boundaries) rather than establishing a new durable one, per `COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`'s own explicit reasoning |
| **Governing Plan** | `SPRINT7_CAPABILITY_AND_IMPLEMENTATION_PLAN.md`, `COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`, `SPRINT7_IMPLEMENTATION_AUTHORIZATION.md` |

---

## Executive Summary

Engineering Release 0.6 closed with one candidate remaining from the original three-item Sprint 6 roadmap: XML inspection. Sprint 7 did not begin by asking how to implement it. Its own charter asked a different question first — "what new capability should XML inspection provide to modIQ?" — generalizing, for the first time as an explicit Sprint mission, a discipline this project had practiced informally since Sprint 1.

That question, grounded directly in `ProductSpecification.md`'s explicitly named "Dependency identification" objective, led to a real architectural question no prior Sprint had faced: every Collector composition decision to date (filesystem vs. archive) had been mutually exclusive, exactly one Collector per Assessment. XML inspection needed to run *alongside* the structural Collector, not instead of it — the first time this platform needed more than one Collector to participate in a single Assessment. Rather than resolve this informally, Sprint 7 was preceded by a dedicated Architecture Evaluation (`COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`), evaluating three named alternatives against repository evidence before any code was written. The approved architecture: Collectors remain fully independent and never consume one another's output; composition is direct and inline inside `AssessmentService`; no dedicated coordinator component is introduced, on the same "capability before abstraction" evidentiary standard that has governed every comparable question on this platform since GOV-004.

`XmlCollector` now runs alongside the existing structural Collector for every Assessment, producing `XmlInspection` Evidence — manifest presence, well-formedness, and declared `<dependency>` elements — with a missing manifest itself recorded as Evidence rather than silently treated as Empty Collection. No Rule Engine change; `modiq-rules` is untouched.

This Sprint also completed a second, largely independent body of work: a consolidation of the repository's own engineering workflow documentation. A study found nine independent, drifted workflow descriptions across the repository — including one document (`docs/implementation/ImplementationWorkflow.md`) that had gone unnoticed and unreferenced since before the Chief Architect rename. All were reconciled into one canonical source, terminology was unified project-wide, and the engineering methodology itself was designated **Version 1.0** — the first time this project's own process, rather than its product architecture, has been versioned as a stable artifact.

---

## Scope of Sprint 7

### Delivered

- **Capability Definition.** `SPRINT7_CAPABILITY_AND_IMPLEMENTATION_PLAN.md` — the capability (reading a mod's own declared self-description), its rationale (grounded in `ProductSpecification.md`'s named objectives), what new evidence becomes available, and what deliberately remains out of scope (true dependency *resolution*, Lua inspection, Version Profile integration, any Knowledge Domain interaction — the last architecturally prohibited for a Collector, not merely deferred), all established before any architecture was designed.
- **Architecture Evaluation.** `COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md` evaluated three named alternatives — expanding an existing Collector, allowing Collectors to depend on each other, and a dedicated coordinator component — against the frozen `EvidenceCollection.md` Collector Contract and `Architecture.md`'s Extensibility principle. The first two were rejected on direct textual and contractual grounds; the third's underlying principle was approved, its literal "coordinator component" form rejected as premature, with a five-condition threshold recorded for when that judgment should be revisited.
- **Architectural Resolution.** Every question the Architecture Evaluation raised was explicitly marked Accepted, Rejected, or Deferred in a Chief Architect Decision Record, added to both the architecture proposal and the capability plan — the first time this project has performed this stage as an explicitly named, artifact-producing step rather than an implicit part of review.
- **Implementation Authorization.** `SPRINT7_IMPLEMENTATION_AUTHORIZATION.md` resolved the remaining non-architectural preconditions: Rule Engine scope (Collector-only this Sprint, no new Rule), the absence-as-evidence question (a missing manifest is recorded as Evidence), Governance Register timing (deferred), and the XML parsing dependency (`roxmltree`, selected for its read-only, query-oriented API matching this Collector's bounded queries, and for a genuine safety property: no DTD or external-entity support, removing XML's classic adversarial-input attack surface entirely — relevant since `modDesc.xml` comes from the same kind of untrusted, community-submitted content this platform already treated carefully for the `zip` crate).
- **Implementation.** `XmlCollector` (`crates/modiq-collection/src/collection/xml_collector.rs`) locates a file or archive entry named exactly `modDesc.xml` at an Assessment Input's root, independently of the structural Collector, and produces `XmlInspection` Evidence: a well-formedness confirmation plus one item per declared `<dependency>` element, a "not well-formed" observation, or an absence observation — never a `CollectionError` for the manifest's own content, only for the Assessment Input's root being unreachable at all. Wired into `AssessmentService::execute_from_assessment_input` as a second, independent, inline Collector invocation alongside the existing structural one — two lines added to an existing method, no new type, trait, or module.
- **Validation.** `cargo fmt`/`check`/`test`, continuously during implementation and as a final comprehensive pass, both workspaces, zero warnings.
- **Implementation Report.** `SPRINT7_IMPLEMENTATION_REPORT.md`, including a new Architectural Validation section recording that every prediction made during Architecture Evaluation was confirmed by implementation, none disproven.
- **Post-Implementation Refinement.** The Implementation Report was revised following Chief Architect review: dependency-extraction status reworded to distinguish implementation-complete from behaviorally-validated (no real Farming Simulator `modDesc.xml` sample exists in this repository to validate against), and the governance recommendation corrected — Sprint 7 is the *first* implementation evidence for the approved Collector composition architecture, not a "second data point" as an earlier draft of the same report had stated.
- **Architectural Conformance Review.** Confirmed by the Chief Architect that the implementation matched what was resolved and authorized, with no undisclosed scope expansion.
- **Engineering Workflow Consolidation — Study.** `ENGINEERING_WORKFLOW_CONSOLIDATION_STUDY.md` located nine independent workflow-shaped descriptions across the repository, found concrete evidence of drift between them (a cross-reference written specifically to prevent staleness had itself gone stale within two sessions), and recommended consolidating into `PROJECT_HANDOFF_v1.0.md` rather than either leaving them distributed or creating a tenth, new document.
- **Engineering Workflow Consolidation — Implementation.** `PROJECT_HANDOFF_v1.0.md` Section 5 rewritten as the single canonical workflow: an Engineering Philosophy section stated in prose, the unified eleven-stage sequence (Capability Definition → Architecture Evaluation → Architectural Resolution → Implementation Authorization → Implementation → Validation → Implementation Report → Architectural Conformance Review → Commit → Merge → Repository Closeout), and the prior `EngineeringGuide.md` cycle preserved as explicitly labeled history. `CHIEF_ARCHITECT_HANDOFF_v1.0.md` and `LEAD_ENGINEER_HANDOFF_v3.0.md` reduced from full copies to references plus role-specific elaboration. `docs/implementation/ImplementationWorkflow.md` archived with an explicit non-normative notice, original content preserved underneath. The Implementation Report template reconciled with practice actually demonstrated across Sprints 6 and 7 — six sections added, none removed.
- **Engineering Methodology Version 1.0** declared in `PROJECT_HANDOFF_v1.0.md` Section 5, recorded as project history rather than governance.
- **Repository Closeout.** Full validation, two commits (`277aefd` for the Sprint's engineering work, `da2d13b` for the closeout documentation), both pushed directly to `feature/runtime-implementation` — no separate Sprint 7 feature branch was created this cycle, unlike Sprint 6's `feature/sprint6-cli`.

### Deliberately Not Delivered

- Dependency *resolution* (cross-referencing a declared dependency against other mods or an environment) — `XmlCollector` identifies declared dependencies as facts about one mod's own manifest; resolving them against a broader collection is a distinct, deferred, future capability.
- Any Rule Engine change — `modiq-rules` untouched; the new `XmlInspection` Evidence is picked up only by the existing, category-agnostic `EvidencePresenceRule`.
- Version Profile integration — `modiq-versioning` untouched, despite this Sprint producing the first real data (declared FS version, declared dependencies) a future Version-Profile-aware capability could consume.
- Knowledge Domain interaction of any kind — architecturally prohibited for a Collector, not a scope choice available to this Sprint.
- A dedicated `CollectionCoordinator` component — the underlying independent-Collector principle was approved; its literal, separately-named component form was not, pending a concrete threshold this Sprint did not meet.
- A new Governance Register item for the Collector Composition Model — deferred explicitly, pending a second independent content Collector beyond `XmlCollector`.
- Deletion (as opposed to archival) of `docs/implementation/ImplementationWorkflow.md`, and reconciliation of `EngineeringGuide.md`'s remaining "Technical Director" reference — both named as open questions by the consolidation study, neither resolved by this Sprint's own authorized scope.

---

## Major Architectural Accomplishments

- **The platform's first genuinely novel Collector-side architectural question was resolved on direct textual and contractual evidence, not intuition.** `Architecture.md`'s own Extensibility principle ("extending Evidence Collection with additional Collectors") directly rejected expanding an existing Collector; `EvidenceCollection.md`'s Collector Contract ("a Collector receives nothing else") directly rejected inter-Collector dependency. Neither rejection required new judgment — both were already written down, waiting to be applied.
- **"Capability before abstraction" was applied to Collector composition itself, for the first time, rather than only to Collector or Rule dispatch.** A literal coordinator component — the shape that would have looked most "complete" — was declined for exactly two participating Collectors, with a concrete, five-condition threshold recorded for when that judgment should be revisited, mirroring the same discipline that retired the original `EngineAPI` service layer.
- **The platform ran more than one Collector for a single Assessment for the first time in its history.** Composition proved to require only two lines inside `AssessmentService`'s existing method body — direct evidence the "capability before abstraction" prediction was correct, not merely convenient.
- **Architectural Resolution was performed as an explicit, artifact-producing stage for the first time**, rather than an implicit part of review — every question raised during Architecture Evaluation was marked Accepted, Rejected, or Deferred in a recorded Decision Record, a discipline this Sprint's own practice is what led to it being formalized as a permanent workflow stage.
- **A genuine post-implementation correction was made and recorded, not silently absorbed.** An early draft of this Sprint's own Implementation Report described Sprint 7 as generating a "second data point" toward the approved architecture; Post-Implementation Refinement corrected this explicitly — it is the *first* implementation evidence, since no independent prior data point actually existed.

---

## Major Implementation Accomplishments

- `modiq-collection` gained its third real Collector and its first content-inspecting one — every prior Collector (`EvidenceCollector`, `ArchiveCollector`) discovered structural presence only; `XmlCollector` is the first to read and interpret file content.
- The root workspace test suite grew from 172 to 187 tests: `modiq-collection` +13 (covering directory/archive × well-formed-with-dependencies/well-formed-without/malformed/absent, plus the manifest-file-as-input case, plus archive open failures, plus determinism), `modiq-engine` +2 (a full-pipeline "manifest found" case and a new multi-Collector evidence-ordering determinism test — the first determinism claim this platform has needed to prove *across* two independently-invoked Collectors, not only within one).
- The Sandbox's own separate test suite grew from 6 to 7, confirming the new Evidence source reaches the platform's other real consumer without any Sandbox code change beyond test expectations.
- `EvidenceCategory::XmlInspection` — declared in `modiq-runtime` before Sprint 1, exercised only by unit tests for six Sprints — became real production output for the first time.
- Manual, end-to-end verification of the real `modiq-cli` binary against a real `modDesc.xml` fixture, both the dependency-found and manifest-absent cases — not only asserted in unit tests.

---

## Governance Completed

No Governance Register item was opened and no ADR was created this Sprint, both by explicit Chief Architect decision recorded in `SPRINT7_IMPLEMENTATION_AUTHORIZATION.md` and reaffirmed in Post-Implementation Refinement:

| Item | Status | Notes |
|---|---|---|
| Collector Composition Model | Deferred, deliberately | The approved architecture is recorded in `COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md` Section 14 as a Chief Architect Decision Record — binding, but not yet a Governance Register item. To be opened once a second independent content Collector provides convergent evidence, mirroring GOV-004's own three-point evidentiary standard. |

**Still open:** GOV-001, GOV-002, GOV-003, GOV-008 (now aging across five Sprints, unaffected by this Sprint — `AssessmentService`'s public entry points are unchanged), and GOV-013 (untouched — `modiq-rules` was not touched this Sprint). Total Governance Register size: 13 items, 8 Resolved, 5 Open — unchanged in count from Engineering Release 0.6.

---

## Documentation Completed

- **Four new Sprint-specific documents**: `SPRINT7_CAPABILITY_AND_IMPLEMENTATION_PLAN.md`, `COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`, `SPRINT7_IMPLEMENTATION_AUTHORIZATION.md`, `SPRINT7_IMPLEMENTATION_REPORT.md` — the fullest Capability-Definition-through-Implementation-Report documentation trail of any Sprint to date.
- **Two new workflow-consolidation documents**: `ENGINEERING_WORKFLOW_CONSOLIDATION_STUDY.md` and `ENGINEERING_WORKFLOW_CONSOLIDATION_REPORT.md`.
- **`PROJECT_HANDOFF_v1.0.md` Section 5 rewritten as the repository's single canonical engineering workflow** — an Engineering Philosophy section, the unified eleven-stage sequence, and the prior cycle preserved as explicit history. The Project Structure diagram in the same document was also corrected during this work, after the consistency validation found it still used two terms the new canonical vocabulary had just retired.
- **`CHIEF_ARCHITECT_HANDOFF_v1.0.md` and `LEAD_ENGINEER_HANDOFF_v3.0.md`** reduced from full workflow copies to references plus role-specific elaboration; Sections 6 and 9 of the former gained one-line notes distinguishing their own, different purposes from the canonical workflow rather than being rewritten.
- **`docs/governance/EngineeringGuide.md`**'s own "Engineering Workflow" section reduced to a reference. **`GOVERNANCE.md`**'s Repository Development Cycle gained a one-line note distinguishing its Documentation-Release granularity from the canonical per-Sprint workflow, preserved otherwise.
- **`docs/implementation/ImplementationWorkflow.md`** archived: an explicit non-normative notice added, original eight-stage content (predating the Chief Architect rename entirely) preserved unedited beneath it.
- **`docs/governance/templates/ImplementationReportTemplate.md`** revised: six sections added (Capability Summary, Repository Impact, Architectural Validation, Governance Observations, Implementation Constraints, Recommendations), reflecting practice actually demonstrated in Sprints 6 and 7; no prior section removed.
- **Repository Closeout**: `PROJECT_STATUS.md`, `CHANGELOG.md`, `ENGINEERING_LOG.md`, and `docs/README.md` reconciled against this Sprint's actual state, including the Version 1.0 methodology designation.

---

## Testing Growth

| Milestone | Root Workspace Tests | Sandbox Tests |
|---|---|---|
| Engineering Release 0.6 | 172 | 6 |
| End of Sprint 7 implementation | 187 | 7 |
| Engineering Release 0.7 (this release) | **187** | **7** |

By crate, at this release: `modiq-runtime` 82, `modiq-collection` 56 (43 + 13 new for `XmlCollector`), `modiq-engine` 18 unit + 3 integration (16 + 2 new), `modiq-report` 3, `modiq-rules` 15, `modiq-cli` 10. Zero tests ignored or flaky anywhere in the workspace, verified by direct execution both continuously during implementation and again while preparing this release.

---

## Repository Maturity Assessment

| Area | Classification | Basis |
|---|---|---|
| Workspace organization | **Stable** | Nine crates, unchanged in count since Engineering Release 0.3 — the new capability was added entirely within `modiq-collection`, not as a new crate. |
| Crate boundaries | **Stable, tested under new pressure for the first time.** `XmlCollector` satisfies the Collector Contract's "receives nothing else" guarantee by construction, verified directly — the first real test of that specific guarantee under a multi-Collector scenario. |
| Dependency direction | **Stable, one new external dependency.** `roxmltree`, authorized explicitly, added only to `modiq-collection`. |
| Documentation consistency | **Substantially improved, following the most severe staleness finding on record (Engineering Release 0.6).** Nine independent workflow descriptions reduced to one; the specific staleness pattern named at four consecutive prior closeouts was not repeated at this Closeout — verified directly, not assumed. |
| Engineering workflow | **Formalized as a versioned artifact for the first time.** Engineering Methodology Version 1.0 — the process itself, not only the product architecture, is now a named, stable, historically-grounded artifact. |
| Governance and evidentiary discipline | **Reinforced, including through self-correction.** The convergent-evidence standard (GOV-004's three-point bar) was applied to a new question (Collector Composition) and explicitly not lowered for convenience; a report's own overstated claim about evidentiary status was caught and corrected before being allowed to stand. |

---

## Crate Maturity Review

| Crate | Maturity | Remaining Work | Architectural Confidence | Priority |
|---|---|---|---|---|
| `modiq-runtime` | L3, 82 tests, unchanged this Sprint | `Display`/`Serialize` for identity/enum types — flagged for a seventh consecutive release | High | Low — stable |
| `modiq-collection` | **L2 → real third Collector added**, 43 → 56 tests | Dependency *resolution* (as opposed to identification), Lua/localization/texture/store-asset collectors, all still deferred | High — the Collector Contract held under genuinely new pressure without needing amendment | **High** — the platform's evaluative and observational value proposition continues to concentrate here |
| `modiq-rules` | L3, 15 tests, unchanged this Sprint | A third Rule; `XmlInspection` Evidence is currently interpreted only generically, by `EvidencePresenceRule` | High — untouched by this Sprint's work, deliberately | Medium — the first concrete candidate (a Rule for `XmlInspection` Evidence) now exists but was deliberately not authorized this Sprint |
| `modiq-engine` | L3, 16 → 18 unit tests, 3 integration tests | The `AssessmentReport` re-export gap (Engineering Release 0.6) remains untouched — `XmlCollector` never needed to name `AssessmentReport`, so this Sprint neither worsened nor resolved it | High — the composition change required no public signature change, confirmed directly | Low — stable |
| `modiq-report` | L3, 3 tests, unchanged since Sprint 6 | None identified | High | Low — stable |
| `modiq-cli`, `apps/sandbox` | L2 / real, respectively — both unchanged in maturity level, both gained real new output | None blocking | High — both already rendered the new Evidence category generically, with zero code change required beyond test expectations | Low — stable |
| `modiq-knowledge` | L1, pure scaffold, unchanged since Sprint 0 | Everything; seven Sprints running with zero forcing function, including through a Sprint that added new Evidence content and still didn't need it | Low | Low — correctly deferred |
| `modiq-versioning` | L1, pure scaffold, unchanged since Sprint 0 | Everything; this Sprint produced the first real data (declared FS version) a future Version-Profile-aware capability could consume, but did not build that capability | Low — untested, but now has a named, concrete future consumer | Low — no forcing function yet, but the clearest one on record |
| `modiq-common` | L1, empty stub files, unchanged since Sprint 0 | Undecided | Low — zero evidence of need after seven Sprints | Low — do not invent a use for it |

---

## Technical Debt Review

### Intentional Technical Debt
*(deliberate, documented, correct to leave as-is for now)*

- The Collector Composition Model Governance Register item — deliberately deferred pending a second independent content Collector.
- No dedicated `CollectionCoordinator` — evaluated and declined, with a concrete, named threshold for revisiting the decision, mirroring `EngineAPI`'s own retirement precedent.
- No Rule Engine interpretation of `XmlInspection` Evidence beyond the generic `EvidencePresenceRule` — deliberately deferred this Sprint, per explicit Chief Architect direction.
- The `modiq-engine` → `AssessmentReport` re-export gap (Engineering Release 0.6) — unchanged, still below the three-point convergent-evidence bar.

### Future Capabilities
*(deferred by design, not yet started, no urgency implied)*

- True dependency resolution (cross-referencing declared dependencies against other mods or an environment).
- Version Profile-aware compatibility checking — the clearest concrete beneficiary of this Sprint's own work, and the first genuine forcing function `modiq-versioning` has had in seven Sprints.
- Lua, localization, texture, and store-asset inspection — all evaluated against the approved Collector Composition architecture during this Sprint's own Architecture Evaluation and found to fit it, none begun.
- A Rule interpreting `XmlInspection` Evidence specifically.
- Knowledge Domain integration.

### Known Limitations
*(real, current, will not resolve without deliberate work)*

- Missing `Display`/`Serialize` for Runtime identity/enum types — now flagged in seven consecutive release records.
- Dependency extraction's interpretation of `<dependency>` element convention is implemented but not behaviorally validated against a real Farming Simulator `modDesc.xml` sample — none exists in this repository.
- `EngineeringGuide.md` still references "Technical Director" in its External Dependencies section — named by the workflow consolidation study, not resolved by this Sprint's own authorized scope.
- No formal `ENGINEERING_RELEASE_0.6.md` or `_0.7.md` existed at the time of either Sprint's own Closeout — both gaps closed by this document and its predecessor, produced retroactively but accurately.
- Git tag hygiene remains unresolved: `v0.4.0` through `v0.7.0` are all available untagged.

### Deferred Enhancements
*(nice-to-have, no urgency, no forcing function yet)*

- Whether `docs/implementation/ImplementationWorkflow.md` should be deleted outright rather than archived.
- Whether `EngineeringGuide.md`'s non-workflow content should be reconciled in place or relocated alongside the now-consolidated workflow.
- `CrateRoadmap.md`'s "Exit Criteria" section still has no entry for any Sprint since Sprint 2.

---

## Sprint 7 Retrospective

**What went well.** The capability-first mission — asking what XML inspection should provide before asking how to build it — produced a materially stronger justification for the work than a purely technical framing would have, grounded directly in a named product objective rather than an inferred one. The Architecture Evaluation → Architectural Resolution sequence, performed as explicit, artifact-producing stages for the first time, left zero open architectural ambiguity by the time Implementation Authorization was reached — verified directly, not assumed.

**What surprised us.** How large a "documentation-only" workflow consolidation task actually was: nine independent, drifted descriptions, not the handful a first read of the repository would suggest, including one (`ImplementationWorkflow.md`) that had gone entirely unnoticed until this Sprint's own systematic search found it.

**Architectural decisions validated by implementation.** Every prediction recorded during Architecture Evaluation was confirmed, not merely asserted — recorded explicitly in this Sprint's own Implementation Report as a new, permanent report section, specifically because it is the place a disproven prediction would have been recorded had one occurred.

**Areas requiring future attention.** GOV-008 has now aged across five Sprints untouched. The Collector Composition Model remains one independent data point short of this project's own usual convergent-evidence threshold. Dependency extraction's real-world behavioral validation remains genuinely open, not merely a formality.

---

## Remaining Risks

- **GOV-008 aging across five Sprints untouched** — still functioning as a stopgap, still not resolved either way.
- **Dependency extraction's interpretation is unvalidated against real production data** — the one piece of this Sprint's behavior not grounded in something checked directly against this repository or a real fixture.
- **The Collector Composition Model operates on one data point** — real, evidenced, and consistent with prediction, but this project's own discipline has historically wanted convergent evidence from independent cases before treating a pattern as settled.
- **Two Engineering Releases (0.6 and 0.7) were both produced after their own Sprints had already concluded**, not at or near their own Closeouts — this document is itself the closing instance of a pattern worth not repeating a third time.

---

## Lessons Learned

- **A capability-first mission changes what "done" means, not just what gets built first.** Framing this Sprint around "what capability should exist" rather than "how do we implement XML inspection" produced a Capability Success Criteria statement ("after this Sprint, modIQ can now...") that is directly falsifiable against real product objectives — a stronger check than "the code compiles and tests pass" alone, and worth carrying forward as standing practice, not a one-Sprint experiment.
- **Formalizing a workflow stage after it has already been demonstrated, rather than designing it in advance, produced a stage (Architectural Resolution) that fit real practice immediately**, with no post-hoc adjustment needed once it became permanent. The same "capability before abstraction" discipline this project applies to code applies equally well to process.
- **Distributed documentation drifts even when someone is actively trying to prevent it.** A cross-reference written at Sprint 6 Closeout specifically to prevent staleness had already gone stale by the time Sprint 7's own consolidation study looked at it, two sessions later. Consolidation to one canonical source, not better hand-synchronization discipline, is the fix this project's own evidence now supports.
- **Engineering Release records should be produced at or near their own Sprint's Closeout.** Both this document and its immediate predecessor were written after the fact, once the gap had grown to two Sprints. The retroactive record is accurate — grounded in real, contemporaneous evidence (implementation reports, closeout reports, status trackers) rather than reconstructed from memory — but the gap itself is a process risk worth not letting recur a third time.

---

## Engineering Metrics

| Metric | Value |
|---|---|
| Workspace crates | 9 (unchanged since Engineering Release 0.3) |
| Governance items | 13 total — 8 Resolved, 5 Open — unchanged in count from Engineering Release 0.6 |
| Documentation Release | 2.1, Frozen — unchanged this Sprint |
| Engineering Release | 0.7 (this document, produced retroactively, alongside 0.6) — not tagged; `v0.7.0` available |
| Root workspace tests | 187 (up from 172 at Engineering Release 0.6) |
| Sandbox tests | 7 (up from 6) |
| Engineering Methodology Version | **1.0** — declared this Sprint |
| Major milestones completed | Documentation Releases 1.0, 2.0, 2.1; Sprint 0–6; Engineering Releases v0.1.0-alpha, 0.2–0.6; Platform Validation Phase 1; Sprint 7 (Multi-Source Evidence Collection, Engineering Workflow Consolidation); Sprint 7 Closeout |
| Implementation readiness | Fully ready for the next engineering objective. Implementation and refinement committed as `277aefd`, Closeout documentation as `da2d13b`, both on `feature/runtime-implementation` directly and pushed — no separate Sprint branch existed this cycle. |

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
Sprint 8 (not yet scoped)
```

---

## Recommendation

**Sprint 8 is ready to be scoped.** The platform's first multi-Collector Assessment is real, tested, and validated; the engineering workflow itself is now consolidated and versioned; both workspaces are fully green.

**What Sprint 8 should contain is not decided here**, consistent with this project's own capability-first discipline, now a formally documented principle rather than an informal habit. `REPOSITORY_CLOSEOUT_REPORT.md` (Sprint 7) names Version Profile-aware compatibility checking as the clearest concrete candidate — this Sprint's own work is the first real data (declared FS version, declared dependencies) such a capability would have to consume — but this release record treats that as a recommendation for Sprint 8 scoping to weigh, not a decision already made.

**Two process items deserve attention before or early in Sprint 8.** First: produce Engineering Release records at or near each Sprint's own Closeout going forward — both this document and Engineering Release 0.6 were written after the fact, and this release's own Lessons Learned names the gap as a pattern not to repeat a third time. Second: the Collector Composition Model remains one independent data point short of this project's own usual convergent-evidence threshold for a Governance Register item — if Sprint 8 scopes a second content Collector, that threshold question should be revisited with real evidence in hand, not decided in the abstract.

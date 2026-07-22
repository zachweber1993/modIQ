# Engineering Release 0.8

| Property | Value |
|---|---|
| **Release** | 0.8 |
| **Documentation Release** | 2.1 (Frozen — unchanged this Sprint; no specification amendment) |
| **Milestone** | Sprint 8 complete (Version Profile-aware compatibility checking; Repository Closeout) |
| **Scope** | The platform's first real Version Profile implementation; the first Rule Engine capability to consult game-version context; the first Runtime entity relationship to extend ADR-0007's Opaque Runtime References pattern beyond Knowledge Domain references |
| **Predecessor** | `docs/engineering/ENGINEERING_RELEASE_0.7.md` (Sprint 7) |
| **Governing ADRs** | None new — this Sprint applies ADR-0007's existing Opaque Runtime References pattern and ADR-0010/GOV-004's existing direct-composition pattern rather than establishing new durable principles, per explicit Chief Architect decision (Decision 6) |
| **Governing Plan** | `SPRINT8_INITIALIZATION_REPORT.md`, `SPRINT8_CAPABILITY_AND_IMPLEMENTATION_PLAN.md`, `SPRINT8_ARCHITECTURAL_RESOLUTION.md`, `SPRINT8_IMPLEMENTATION_AUTHORIZATION.md` |

---

## Executive Summary

Engineering Release 0.7 closed with Version Profile-aware compatibility checking named as the clearest concrete Sprint 8 candidate — `modiq-versioning` had gone seven Sprints with zero implementation and zero forcing function, and Sprint 7's own `XmlCollector` work had just produced the first real data (a mod's declared `modDesc.xml` content) such a capability would need to consume.

Sprint 8 did not begin with implementation. It began with a full repository orientation (`SPRINT8_INITIALIZATION_REPORT.md`) verifying that orientation directly against source — including discovering, by direct inspection, that `XmlCollector` did **not** yet extract a mod's declared `descVersion` despite prior documentation implying it did. Capability Definition (`SPRINT8_CAPABILITY_AND_IMPLEMENTATION_PLAN.md`) then surfaced two genuine architectural forks the frozen specification itself left ambiguous: where a Version Profile attaches to `Assessment` (`DataModel.md`'s Entity Relationships diagram versus its Assessment Context prose), and where version-aware evaluation should occur in the pipeline. Both were evaluated with alternatives and a recommendation, explicitly not decided, in a dedicated Architecture Evaluation (`SPRINT8_ARCHITECTURAL_RESOLUTION.md`) — six architectural decisions in total. The Chief Architect resolved all six; Implementation Authorization (`SPRINT8_IMPLEMENTATION_AUTHORIZATION.md`) validated those decisions against fresh repository evidence, surfacing a further refinement (an opaque `VersionProfileReference` rather than a direct dependency, preserving `modiq-runtime`'s leaf status) before implementation began.

`modiq-versioning` gained its first real content since Sprint 0: a minimal `GameVersion`/`VersionProfile` pair, with a single hardcoded `VersionProfile::fs25()` recognizing `descVersion` 93. `XmlCollector` was extended to extract a mod's declared `descVersion` as a purely factual `XmlInspection` Evidence item. A new Rule, `VersionCompatibilityRule`, evaluates that declared value against the active Version Profile inside the Rule Engine, producing a `Warning` Finding when unrecognized. `Assessment` records which profile governed it through a new opaque `VersionProfileReference`, extending ADR-0007's established pattern to a domain relationship for the first time. `AssessmentService`'s two public entry points required **zero signature change** — an implementation simplification the Chief Architect accepted in place of the originally anticipated new additive entry point.

---

## Scope of Sprint 8

### Delivered

- **Capability Definition.** `SPRINT8_CAPABILITY_AND_IMPLEMENTATION_PLAN.md` — the capability (comparing a mod's declared version against what the platform recognizes), its rationale (`ProductSpecification.md`'s named "Compatibility Verification" Player objective), a direct correction of a prior session's inaccurate claim that declared-version Evidence already existed, and the two architectural forks requiring resolution, all established before any architecture was designed.
- **Architecture Evaluation.** `SPRINT8_ARCHITECTURAL_RESOLUTION.md` evaluated six architectural decisions — Version Profile ownership, extraction location, version-aware evaluation location, Assessment construction, crate dependencies, and governance timing — against repository evidence (ADR-0003's own ownership list omitting Version Profile, corroborating `DataModel.md`'s explicit ownership exception; `ADR-0007`'s Opaque Runtime References precedent; `ADR-0009`/GOV-008's anticipated-breaking-change precedent), with a recommendation each, none decided.
- **Architectural Resolution.** All six decisions explicitly approved by the Chief Architect, recorded as final for Sprint 8.
- **Implementation Authorization.** `SPRINT8_IMPLEMENTATION_AUTHORIZATION.md` validated all six decisions against fresh repository evidence, surfacing one refinement not previously identified: `Assessment` holding the real `VersionProfile` type directly would have required `modiq-runtime`'s first-ever workspace dependency, breaking its unbroken leaf status since Sprint 0 and bypassing ADR-0007's own Opaque Runtime References pattern — an opaque `VersionProfileReference` was recommended instead, and adopted.
- **Implementation.** `modiq-versioning` (`GameVersion`, `VersionProfile`, `fs25()`); `modiq-runtime` (`VersionProfileReference`, `Assessment`'s new constructor parameter and accessor); `modiq-collection` (`XmlCollector`'s `descVersion` extraction); `modiq-rules` (`VersionCompatibilityRule`, `RuleEngine::evaluate`'s new parameter); `modiq-engine` (`AssessmentService` internally defaulting to `VersionProfile::fs25()`); `modiq-report` (test call sites only). `modiq-cli` and `apps/sandbox` required zero change.
- **Validation.** `cargo fmt`/`check`/`test`, continuously through seven implementation phases and as a final comprehensive pass, both workspaces, zero warnings.
- **Implementation Report.** `SPRINT8_IMPLEMENTATION_REPORT.md`, including an Architectural Validation section recording every Decision's outcome against implementation evidence and the one refinement from the authorized plan (Section 5/7 of that report).
- **Implementation Deviation Record.** `SPRINT8_IMPLEMENTATION_DEVIATIONS.md` — every meaningful difference between `SPRINT8_IMPLEMENTATION_AUTHORIZATION.md`'s planned approach and actual implementation, with repository evidence and engineering rationale for each, per explicit Chief Architect request that this become permanent engineering history.
- **Architectural Conformance Review.** Confirmed by the Chief Architect that the implementation matched what was resolved and authorized, with the two named refinements (`VersionProfileReference`; unchanged `AssessmentService` APIs) explicitly accepted as part of Sprint 8, not treated as undisclosed scope expansion.
- **Repository Closeout.** Full validation, documentation synchronization, this Engineering Release produced at Sprint 8's own Closeout (not retroactively), one commit, pushed directly to `feature/runtime-implementation` — no separate Sprint 8 feature branch, mirroring Sprint 7's own precedent.

### Deliberately Not Delivered

- Exhaustive Farming Simulator version knowledge — one hardcoded FS25 profile, recognizing exactly one `descVersion` value, by explicit scope.
- A profile-selection mechanism — no configuration, no persistence, no UI; the single default profile is constructed internally by `AssessmentService`.
- General-purpose Rule Selection filtering (`RuleEngine.md`'s original, broader "Version Aware" principle) — the narrower "one Rule consumes the Version Profile directly" path was implemented instead, per Decision 3.
- Knowledge Domain interaction of any kind — `modiq-knowledge` untouched, zero forcing function created by this Sprint's own work.
- A new additive `AssessmentService` entry point — anticipated at Implementation Authorization, found unnecessary at implementation (Section 5, `SPRINT8_IMPLEMENTATION_REPORT.md`; full record in `SPRINT8_IMPLEMENTATION_DEVIATIONS.md`).
- A new Governance Register item, ADR, or `modiq-versioning` Crate Boundary Rule — deferred explicitly (Decision 6), mirroring Sprint 7's own treatment of the Collector Composition Model.

---

## Major Architectural Accomplishments

- **A textual ambiguity in already-Frozen specifications was identified and resolved through convergent evidence, not intuition.** `DataModel.md`'s Entity Relationships diagram and its Assessment Context prose did not obviously agree on where Version Profile attaches; `ADR-0003`'s own ownership list independently omitting Version Profile — discovered during Architecture Evaluation, not assumed — corroborated the diagram's reading over the prose's, the same convergent-evidence standard GOV-004's resolution established.
- **ADR-0007's Opaque Runtime References pattern was extended to a new category of relationship for the first time.** Every prior use (`RuleReference`, `RepairRecipeReference`) referenced the Knowledge Domain specifically; `VersionProfileReference` is the first opaque reference to a domain outside Runtime and Knowledge both, confirming the pattern generalizes rather than being Knowledge-Domain-specific.
- **A planned implementation consequence was avoided through validation, not discovered as a regret afterward.** Implementation Authorization's own validation pass caught that a literal reading of "Assessment owns a Version Profile" would have broken `modiq-runtime`'s unbroken, Sprint-0-era leaf status — surfaced and resolved before any code was written, exactly the kind of pre-implementation validation this Sprint's own Lessons Learned (below) generalizes into a broader observation.
- **"Capability before abstraction" was applied to a new additive `AssessmentService` entry point and, for the first time in this project's history, resulted in *not building* API surface that a prior planning document had explicitly anticipated** — recorded as a deviation (`SPRINT8_IMPLEMENTATION_DEVIATIONS.md`) rather than silently substituted, since no second Version Profile exists yet to justify it.
- **A four-stage Capability Definition → Architecture Evaluation → Architectural Resolution → Implementation Authorization sequence was completed for the second time**, following Sprint 7's own establishment of Architectural Resolution as an explicit, artifact-producing stage — confirming that stage's durability as a repeatable part of the canonical workflow, not a one-Sprint experiment.

---

## Major Implementation Accomplishments

- `modiq-versioning` gained real content for the first time since Sprint 0 — a seven-Sprint-old scaffold crate becoming real, tested code.
- `modiq-rules` gained its third Rule and its first dependency on a crate other than `modiq-runtime`; `modiq-engine` gained its first dependency on `modiq-versioning`. Both are new dependency edges in a graph that had not changed shape since Sprint 3's introduction of `modiq-collection`.
- The root workspace test suite grew from 187 to 205 tests: `modiq-versioning` 0 → 4 (its first tests ever), `modiq-rules` 15 → 25 (+10: `VersionCompatibilityRule`'s own 8, plus 2 new `RuleEngine` dispatch tests), `modiq-runtime` 82 → 84 (+2), `modiq-collection` 56 → 57 (+1), `modiq-engine` 18 → 19 unit (+1), 3 integration unchanged.
- The Sandbox's own separate test suite required zero modification and remains unchanged at 7/7 — the first Sprint since at least Sprint 4 where the Sandbox needed no update at all, confirming `AssessmentService`'s public entry points were genuinely untouched.
- Manual, end-to-end verification of the real `modiq-cli` binary against both an unsupported-declared-version fixture and a supported one — not only asserted in unit tests.
- `modiq-runtime`'s zero-workspace-dependency leaf status, unbroken since Sprint 0, was preserved through this Sprint's own implementation choice, not merely by accident of scope.

---

## Governance Completed

No Governance Register item was opened and no ADR was created this Sprint, both by explicit Chief Architect decision (Decision 6, `SPRINT8_ARCHITECTURAL_RESOLUTION.md`/`SPRINT8_IMPLEMENTATION_AUTHORIZATION.md`), mirroring Sprint 7's own treatment of the Collector Composition Model:

| Item | Status | Notes |
|---|---|---|
| Version Profile Ownership / Assessment Construction / Version-aware Evaluation | Deferred, deliberately | Recorded as a Sprint-specific Chief Architect authorization (`SPRINT8_IMPLEMENTATION_AUTHORIZATION.md`, this document) — binding for Sprint 8, not yet a Governance Register item. |
| `modiq-versioning` Crate Boundary Rule (`GOVERNANCE.md`) | Open, unaddressed | Named during Sprint 8 planning; left open by explicit decision. |

**Still open:** GOV-001, GOV-002, GOV-003, GOV-008 (now aging across six Sprints, unaffected by this Sprint — both `AssessmentService` entry points required no signature change, generating no new evidence toward it), and GOV-013 (untouched — `modiq-rules`'s existing two Rules were not modified, only extended with a third). Total Governance Register size: 13 items, 8 Resolved, 5 Open — unchanged in count from Engineering Release 0.7.

---

## Documentation Completed

- **Four new Sprint-specific documents**: `SPRINT8_INITIALIZATION_REPORT.md`, `SPRINT8_CAPABILITY_AND_IMPLEMENTATION_PLAN.md`, `SPRINT8_ARCHITECTURAL_RESOLUTION.md`, `SPRINT8_IMPLEMENTATION_AUTHORIZATION.md`.
- **`SPRINT8_IMPLEMENTATION_REPORT.md`** — the standard Lead Engineer implementation report.
- **`SPRINT8_IMPLEMENTATION_DEVIATIONS.md`** (new) — a dedicated, permanent record of every meaningful difference between the authorized plan and actual implementation, requested explicitly by the Chief Architect as its own document rather than folded into the Implementation Report alone.
- **Repository Closeout**: `PROJECT_STATUS.md` (header fields, new `## Sprint 8 — Complete` section, Current Focus and Governance Status notes updated), `CHANGELOG.md` (new `# [Sprint 8]` entry), `ENGINEERING_LOG.md` (Sprint 8 Closeout entry, plus a new "Engineering Methodology Observations" section recording the phased-execution-with-validation-gates observation as history for future consideration, not an adopted process change), `CrateRoadmap.md` (Implementation Status table, dependency-edge notes, Sprint 7 and 8 narrative sections — Sprint 7's own entry had never been added, corrected here alongside Sprint 8's), `docs/README.md` (Engineering Release cross-reference, also correcting a stale claim that Engineering Release 0.6/0.7 remained unproduced).

---

## Testing Growth

| Milestone | Root Workspace Tests | Sandbox Tests |
|---|---|---|
| Engineering Release 0.7 | 187 | 7 |
| End of Sprint 8 implementation | 205 | 7 |
| Engineering Release 0.8 (this release) | **205** | **7** |

By crate, at this release: `modiq-runtime` 84, `modiq-collection` 57, `modiq-engine` 19 unit + 3 integration, `modiq-report` 3, `modiq-rules` 25, `modiq-cli` 10, `modiq-versioning` 4. Zero tests ignored or flaky anywhere in the workspace, verified by direct execution both continuously during implementation and again while preparing this release.

---

## Repository Maturity Assessment

| Area | Classification | Basis |
|---|---|---|
| Workspace organization | **Stable** | Nine crates, unchanged in count since Engineering Release 0.3 — the new capability was added within existing crates, not a new one. |
| Crate boundaries | **Stable, extended under new pressure for the first time.** ADR-0007's Opaque Runtime References pattern, previously exercised only for Knowledge Domain references, was successfully applied to a Version domain reference — the pattern generalized without needing amendment. |
| Dependency direction | **Stable, two new internal edges, zero new external dependency.** `modiq-engine`/`modiq-rules` → `modiq-versioning`, both justified by direct implementation need; `modiq-runtime` confirmed still dependency-free. |
| Documentation consistency | **Substantially improved.** The two-Sprint-running late-Engineering-Release pattern named at Engineering Release 0.7's own Lessons Learned was not repeated — this release was produced at Sprint 8's own Closeout. A stale `docs/README.md` claim (that Engineering Release 0.6/0.7 remained unproduced, though both had in fact been completed retroactively before this Sprint began) was also found and corrected. |
| Engineering workflow | **Exercised for the second time, unchanged.** The four-stage Capability Definition → Architecture Evaluation → Architectural Resolution → Implementation Authorization sequence, established as a permanent stage at Sprint 7, was followed again without modification. |
| Governance and evidentiary discipline | **Reinforced.** The convergent-evidence standard was applied to a specification-reading question (Version Profile ownership) for the first time, not only to a code-pattern question as in prior Sprints. |

---

## Crate Maturity Review

| Crate | Maturity | Remaining Work | Architectural Confidence | Priority |
|---|---|---|---|---|
| `modiq-versioning` | **L1 → L2**, 0 → 4 tests | `Capability`/`Compatibility` remain unimplemented; a second Version Profile (e.g. FS22) has no forcing function yet | High — the crate's first real content required no redesign of its already-specified conceptual model (`VersionProfile.md`) | Medium — real, but minimum-viable; no urgency beyond this Sprint's own scope |
| `modiq-rules` | L3, 15 → 25 tests | A fourth Rule; general Rule Selection filtering remains deliberately unbuilt | High — the third Rule required no change to `RuleEngine::evaluate`'s dispatch shape beyond one new parameter and one new `if let` | Medium — the Rule Engine's own depth continues to grow incrementally, as designed |
| `modiq-runtime` | L3, 82 → 84 tests | `Display`/`Serialize` for identity/enum types — flagged for an eighth consecutive release | High — `VersionProfileReference` followed `ADR-0007`'s pattern exactly, no new design question | Low — stable |
| `modiq-collection` | L2, 56 → 57 tests | Dependency *resolution*, Lua/localization/texture/store-asset collectors, all still deferred | High — `descVersion` extraction required no change to the Collector Contract | Medium — unchanged from Engineering Release 0.7's own assessment |
| `modiq-engine` | L3, 18 → 19 unit tests, 3 integration | The `AssessmentReport` re-export gap (Engineering Release 0.6) remains untouched — this Sprint neither worsened nor resolved it | High — the new dependency and internal defaulting required no public signature change, confirmed directly by zero Sandbox/`modiq-cli` modification | Low — stable |
| `modiq-report` | L3, 3 tests, unchanged | `AssessmentReport` does not yet expose which Version Profile was active — a real, named Known Limitation | High | Low — small, low-risk future addition |
| `modiq-cli`, `apps/sandbox` | L2 / real, respectively — both unchanged in maturity level and, uniquely this Sprint, unchanged in source code at all | None blocking | High — both already rendered the new Finding generically, with zero code change required | Low — stable |
| `modiq-knowledge` | L1, pure scaffold, unchanged since Sprint 0 | Everything; eight Sprints running with zero forcing function | Low | Low — correctly deferred |
| `modiq-common` | L1, empty stub files, unchanged since Sprint 0 | Undecided | Low — zero evidence of need after eight Sprints; confirmed directly this Sprint (`modiq-versioning` deliberately did not depend on it) | Low — do not invent a use for it |

---

## Technical Debt Review

### Intentional Technical Debt
*(deliberate, documented, correct to leave as-is for now)*

- No new additive `AssessmentService` entry point — evaluated and declined this Sprint specifically, pending a real second Version Profile need.
- No general-purpose Rule Selection filtering — deliberately deferred; one Rule consuming the Version Profile directly was sufficient for this Sprint's own scope.
- The `modiq-versioning` Crate Boundary Rules gap in `GOVERNANCE.md` — deliberately deferred (Decision 6).
- The `DECLARED_DESC_VERSION_PREFIX` string-format coupling between `modiq-collection` and `modiq-rules` — a deliberate, documented data-format convention rather than a shared-crate dependency, per Decision 5's "no speculative dependency" instruction and GOV-003's continued Open status.

### Future Capabilities
*(deferred by design, not yet started, no urgency implied)*

- A second Version Profile (e.g. FS22) — the clearest concrete forcing function for both a profile-selection mechanism and a fresh look at whether an additive `AssessmentService` entry point is now justified.
- A positive-compatibility Finding (confirming a recognized declared version, not only flagging an unrecognized one).
- `AssessmentReport` exposing the active Version Profile directly.
- Version-aware Rule Selection as `RuleEngine.md` originally specified it, more broadly than this Sprint's single-Rule implementation.
- Knowledge Domain integration, unaffected by this Sprint.

### Known Limitations
*(real, current, will not resolve without deliberate work)*

- Missing `Display`/`Serialize` for Runtime identity/enum types — now flagged in eight consecutive release records.
- Exactly one Version Profile exists (FS25, `descVersion` 93 only) — by explicit scope, not oversight.
- `AssessmentReport` does not expose which Version Profile was active — discoverable only indirectly, through a `VersionCompatibilityRule` Finding's own description text, and only when one fires.
- An unparseable declared `descVersion` value is silently ignored by `VersionCompatibilityRule`, not reported as its own kind of Finding.
- Git tag hygiene remains unresolved: `v0.4.0` through `v0.8.0` are all available untagged.

### Deferred Enhancements
*(nice-to-have, no urgency, no forcing function yet)*

- Whether `docs/implementation/ImplementationWorkflow.md` should be deleted outright rather than archived (named at Sprint 7, still open).
- Whether `EngineeringGuide.md`'s remaining "Technical Director" reference should be reconciled (named at Sprint 7, still open).
- `CrateRoadmap.md`'s "Exit Criteria" section still has no entry for any Sprint since Sprint 2.

---

## Sprint 8 Retrospective

**What went well.** The four-stage Capability Definition → Architecture Evaluation → Architectural Resolution → Implementation Authorization sequence, established at Sprint 7, proved durable on its second real use — every architectural question was resolved before implementation began, and Implementation Authorization's own validation pass caught a real, previously-unidentified consequence (the `modiq-runtime` leaf-status risk) before it became a regret discovered mid-implementation.

**What surprised us.** How much a validation-focused Implementation Authorization session could still find after three prior sessions of careful evaluation — the `VersionProfileReference` refinement was not visible until the fourth pass specifically looked for it, reinforcing that repeated, independent verification finds things a single careful pass does not.

**Architectural decisions validated by implementation.** All six Chief Architect Decisions were confirmed by implementation with no conflicts; one implementation detail (the additive-entry-point question) was resolved differently than anticipated, recorded explicitly (`SPRINT8_IMPLEMENTATION_DEVIATIONS.md`) rather than silently reconciled — this Sprint's own Architectural Validation section is the second instance of this project's now-standing "record predictions and their outcomes" discipline, first introduced at Sprint 7.

**Areas requiring future attention.** GOV-008 has now aged across six Sprints untouched. The `modiq-versioning` Crate Boundary Rules gap remains open. A second Version Profile remains the clearest concrete forcing function for several deferred questions at once.

---

## Remaining Risks

- **GOV-008 aging across six Sprints untouched** — still functioning as a stopgap, still not resolved either way, and now further reinforced by this Sprint's own zero-signature-change outcome.
- **`modiq-versioning`'s minimum-viable single profile is not yet a realistic test of the architecture at scale.** A second profile would be the first real evidence of whether the current design (a hardcoded `fs25()` factory, application-layer-supplied) generalizes or needs revisiting.
- **The `DECLARED_DESC_VERSION_PREFIX` string-format coupling between two independent crates** is a real, if minor, fragility — a silent drift between the two copies would silently stop `VersionCompatibilityRule` from recognizing declared versions, with no compiler error to catch it.
- **The Sprint 8 documentation history itself revealed a pre-existing staleness** (`docs/README.md`'s claim that Engineering Release 0.6/0.7 remained unproduced) that had gone unnoticed since Engineering Release 0.7's own Closeout — a reminder that documentation drift between Closeouts remains a real, standing risk (`PROJECT_HANDOFF_v1.0.md`, Section 6, Principle 9), not fully solved by any single Sprint's diligence.

---

## Lessons Learned

- **Validation sessions find things earlier sessions did not, even on the same material.** The `VersionProfileReference` refinement existed as a real design question from the moment Decision 1 was first evaluated, but was only surfaced at Implementation Authorization — the fourth pass over the same architectural territory. This is direct evidence for treating "validate the decisions against the repository" as a genuine, distinct stage with its own value, not a formality between Architectural Resolution and Implementation.
- **Recording a plan deviation explicitly, immediately, is cheaper than discovering it silently later.** The unchanged-`AssessmentService`-APIs deviation was caught and recorded in the same session it was decided, in both the Implementation Report and a dedicated `SPRINT8_IMPLEMENTATION_DEVIATIONS.md` — avoiding any future session needing to reverse-engineer why the actual code doesn't match the authorized plan.
- **Phased implementation with a validation gate after each phase is worth naming as an explicit engineering observation**, not only practicing informally — recorded in `ENGINEERING_LOG.md`'s new "Engineering Methodology Observations" section as a single data point, pending convergent evidence from a future Sprint before it becomes an adopted expectation.
- **Producing the Engineering Release at Sprint close, not after a gap, is achievable and should remain the standing expectation** — this release exists as direct proof the two-Sprint-running pattern named at Engineering Release 0.7 was not structural, but simply not yet corrected.

---

## Engineering Metrics

| Metric | Value |
|---|---|
| Workspace crates | 9 (unchanged since Engineering Release 0.3) |
| Governance items | 13 total — 8 Resolved, 5 Open — unchanged in count from Engineering Release 0.7 |
| Documentation Release | 2.1, Frozen — unchanged this Sprint |
| Engineering Release | 0.8 (this document, produced at Sprint 8's own Closeout) — not tagged; `v0.8.0` available |
| Root workspace tests | 205 (up from 187 at Engineering Release 0.7) |
| Sandbox tests | 7 (unchanged) |
| Engineering Methodology Version | 1.0 — unchanged this Sprint |
| Major milestones completed | Documentation Releases 1.0, 2.0, 2.1; Sprint 0–7; Engineering Releases v0.1.0-alpha, 0.2–0.7; Platform Validation Phase 1; Sprint 8 (Version Profile-aware compatibility checking); Sprint 8 Closeout |
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
Sprint 9 (not yet scoped)
```

---

## Recommendation

**Sprint 9 is ready to be scoped.** The platform's first real Version Profile is live, tested, and validated end-to-end through the real `modiq-cli` binary; `modiq-runtime`'s architectural leaf status was preserved under genuine new pressure; both workspaces are fully green.

**What Sprint 9 should contain is not decided here**, consistent with this project's own capability-first discipline. Natural candidates surfaced by this Sprint's own work, none selected: a second Version Profile (the clearest forcing function for several deferred questions — profile selection, the `AssessmentService` entry-point question, and whether `modiq-versioning`'s current design generalizes); a fourth Rule; or closing the `AssessmentReport` Version Profile visibility gap. This release record treats these as candidates for Sprint 9 scoping to weigh, not decisions already made.

**One process item continues to deserve attention.** GOV-008 has now aged across six Sprints. This release does not recommend resolving it reactively, but does recommend that Sprint 9 scoping explicitly ask whether its own capability is likely to finally produce the evidence needed — rather than let it continue by default, as every Sprint since Sprint 3 has.

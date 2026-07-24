# PROJECT_STATUS

| Property | Value |
|----------|-------|
| **Project** | modIQ |
| **Current Release** | Engineering Release 1.3 — `docs/engineering/ENGINEERING_RELEASE_1.3.md`, produced at Sprint 13's own Repository Closeout |
| **Repository Status** | Implementation Ready — Sprint 13 (Storage Architectural Activation) complete; `modiq-storage` implemented, tested, and wired end to end through `modiq-cli` and `apps/sandbox`; `RuntimeLogCollector` and `RuntimeLoadFailureRule` remain implemented, tested, and unchanged since Sprint 11 |
| **Current Milestone** | Sprint 13 — Complete (Storage Architectural Activation); Repository Closeout complete. INV-001 (Lua Analysis Capability Investigation) remains Complete, Recommendation A, unaffected; see below. |
| **Overall Status** | Active Implementation — Storage subsystem real for the first time; Lua Analysis still awaiting additional evidence before further architectural evaluation |
| **Current Phase** | Post-Sprint 13 Repository Closeout complete; GOV-001 (Assessment Report Generation Timing) Architecture Evaluation complete — no Architectural Resolution performed, returned to Open under a narrower question, pending a future forcing function; GOV-014 (Lua Fixture Acquisition Governance) open, unaffected by Sprint 13; Sprint 14 not yet scoped and not yet started |
| **Engineering Methodology Version** | 1.0 — unchanged |
| **Last Updated** | 2026-07-23 |

---

# Current Milestone

## Sprint 1 — Complete

Sprint 0 is complete. The engineering foundation was established, including workspace scaffolding, repository governance, crate hierarchy, module hierarchy, documentation hierarchy, architecture verification, and a clean workspace build.

Sprint 1 delivered the first complete, deterministic Assessment pipeline: Assessment creation, Evidence collection, deterministic Rule evaluation, Finding and Recommendation generation, Assessment Report generation, and Assessment completion, composed into one executable operation (`AssessmentService::execute`) and exercised end to end by integration tests. The repository was tagged `v0.1.0-alpha`. Full record: `ENGINEERING_RELEASE_v0.1.0-alpha.md`, `HANDOFF_SPRINT1.md`.

## Sprint 2 — Complete

Sprint 2 gave Evidence, Finding, and Recommendation — content-free in Sprint 1 — real field content, identity, and constructor validation, and extended `Assessment` with relationship-resolution methods (`evidence_by_id`, `finding_by_id`, `evidence_for_finding`, `findings_for_recommendation`). No Rule Engine, Reporting, or Engine orchestration behavior changed; no Knowledge Domain integration began. Two content-level invariants were identified and deliberately left unenforced, recorded as Governance Register items GOV-005 and GOV-006. The workspace test suite grew from 55 to 97 tests. Full record: `docs/releases/ENGINEERING_RELEASE_0.2.md`, `SPRINT2_IMPLEMENTATION_PLAN.md`, ADR-0007.

## Sprint 3 — Complete

Sprint 3 delivered the Evidence Collection subsystem, from architectural boundary through its first real capability, across five phases: Sandbox real-pipeline integration (Phase 1); GOV-005/GOV-006 minimum reference cardinality (Phase 2, INV-013/INV-014); the Evidence Collection Subsystem Boundary architecture (Phase 3, ADR-0008, ADR-0009, Documentation Release 2.1); a minimal `modiq-collection` crate proving the boundary (Phase 4, GOV-007); and the first real, filesystem-backed collector (Phase 5, GOV-009/GOV-010, deterministic traversal, the four-outcome Collection Error Model, Collection Atomicity, the Symbolic Link Policy). The workspace grew to nine crates; the root test suite grew from 97 to 112 tests. Full record: `ENGINEERING_RELEASE_0.3.md`.

## Platform Validation Phase 1 — Complete

Following Engineering Release 0.3, a Platform Validation cycle reviewed accumulated architectural assumptions against three Sprints of implementation evidence (`PROPOSAL_PLATFORM_VALIDATION_REVIEW.md`). GOV-004 (Engine Service Granularity) was evaluated, approved, and implemented: `AssessmentService` direct subsystem composition is the confirmed engine architecture, and the unused internal `EngineAPI` service model (and its `modiq-rules` mirror) was removed. GOV-008 (AssessmentService Public API Evolution) was evaluated and deferred: implementation evidence was found insufficient to resolve it, no architectural change was authorized, and the current `AssessmentService` execution contract remains the approved platform boundary pending future implementation evidence. Architecture validated for continued roadmap execution. Full record: `docs/engineering/ENGINEERING_LOG.md`.

**Next implementation milestone (superseded by Sprint 4, below):** ZIP / Archive Evidence Collection, the platform's second real Evidence Collector — see `docs/engineering/PROPOSAL_ZIP_EVIDENCE_COLLECTION.md`. CLI wiring remains a separately available, low-risk parallel track, not yet scoped in detail.

## Sprint 4 — Complete

Sprint 4 delivered the platform's second real Evidence Collector, archive-based, across four implementation phases plus a governance-resolution and boundary-proving cycle. GOV-011 (Archive Collection Model) was resolved in its entirety (`PROPOSAL_GOV-011.md`), informed by Phase 2's empirical investigation of the `zip` crate (v8.6.0) — the platform's first archive-parsing dependency and first genuinely adversarial input surface. Phase 3A implemented deterministic archive structural enumeration (`ArchiveReader`); Phase 3B transformed that structure into real Evidence (`ArchiveEvidenceBuilder`), reusing `EvidenceCategory::FileStructureAnalysis` unchanged; Phase 3C assembled both with GOV-011's three remaining policies (resource limits, the Archive Traversal Boundary Policy, and the Duplicate Archive Entry Policy) into one real Collector (`ArchiveCollector`), introducing one new closed-set category, `EvidenceCategory::StructuralDuplication`, following a dedicated Architecture Review (`PROPOSAL_GOV-011_DUPLICATE_REPRESENTATION.md`); Phase 3D wired `ArchiveCollector` into `AssessmentService` via one explicit, inline routing check, completing the platform's first end-to-end archive assessment path. No collector-dispatch abstraction was introduced at any phase. A Sprint 4 Closeout reconciled repository documentation, validated the archive path through the Sandbox application against a checked-in fixture, and produced an engineering retrospective (recorded inside `ENGINEERING_RELEASE_0.4.md` itself, per this project's established pattern of housing a release's retrospective in its own release document rather than a separate file). The workspace grew from 112 to 150 root tests (Sandbox: 3 to 6); nine workspace crates, unchanged in count. Full record: `docs/engineering/ENGINEERING_RELEASE_0.4.md`.

## Sprint 5 — Complete

Sprint 5 deliberately did not add a third Evidence Collector; per Technical Director direction, it strengthened the platform vertically instead, building the assessment intelligence layer — the Rule Engine's own depth — before widening Evidence Collection further. GOV-012 (Rule Evaluation Model) was resolved: `RuleEngine::evaluate` returns `Vec<RuleOutcome>`, Rules are dispatched in fixed, explicit declaration order, and compose independently with no suppression model. GOV-013 (FindingSeverity Severity/Kind Conflation) was opened, deliberately Open, not Resolved — the Technical Director accepted a real architectural tension (`BestPractice` classifies Finding *kind*, not severity, unlike `Error`/`Warning`/`Informational`) as provisionally accepted rather than deciding a Runtime model change from two Rules alone, to be revisited once more concrete Rules exist. `DataModel.md` was amended to v1.1.0 with the platform's first specification-level Finding Severity definitions. `modiq-rules` gained its second concrete Rule, `StructuralDuplicationRule` (Phase 2), and the original Sprint 1 Rule was extracted into its own unit, `EvidencePresenceRule`, both dispatched by the new multi-Rule `RuleEngine::evaluate` (Phase 3) — no trait, registry, or dispatch abstraction introduced. A dedicated investigation (Phase 4) recommended retiring `modiq-report`'s four unused scaffold types (`FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, `ReportFormatter`), using the same evidentiary method GOV-004 used, but no action was taken this Sprint. Phase 5 closed one determinism-testing gap (Rule outcome order confirmed independent of Evidence arrival order, not just repeated-identical-input order). A Sprint 5 Closeout reconciled repository documentation and produced an engineering retrospective (inside `ENGINEERING_RELEASE_0.5.md`). The workspace grew from 150 to 162 root tests (Sandbox unchanged at 6); nine workspace crates, unchanged in count. Full record: `docs/engineering/ENGINEERING_RELEASE_0.5.md`.

## Sprint 6 — Complete (Implementation and Merge)

Sprint 6 closed two of the three roadmap candidates named at Sprint 5 Closeout. `modiq-cli` was wired to `modiq-engine` for the first time since Sprint 0: `Application` dispatches `assess`/`help`/`version` by one direct match (no command trait or registry); `AssessCommand` calls `AssessmentService::execute_from_assessment_input` against a real, user-supplied path — the same entry point the Sandbox already calls, reused rather than reinvented — mapping the result to a three-tier exit-code convention (0 success, 1 execution failure, 2 invalid usage). `modiq-report`'s four scaffold types recommended for retirement at Sprint 5 Phase 4 (`FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, `ReportFormatter`) were deleted under this Sprint's explicit Chief Architect authorization; `AssessmentReport`, the crate's real, tested content, is unchanged. No change to `AssessmentService`'s public entry points, `AssessmentInput`, or the public error model. `modiq-cli` advanced from L1 to L2. The workspace grew from 162 to 172 root tests (`modiq-cli` 0 → 10; `modiq-report` unchanged at 3, confirming the deletion had zero test-coverage impact); Sandbox reverified unchanged at 6/6. Implementation was committed (`397707f`) on `feature/sprint6-cli`, reviewed, and merged into `feature/runtime-implementation` (`29657df`) with a full post-merge revalidation. Full record: `docs/engineering/SPRINT6_IMPLEMENTATION_PLAN.md` (including its Authorization Record), `docs/engineering/POST_SPRINT6_REPOSITORY_ASSESSMENT.md`, and `docs/engineering/ENGINEERING_RELEASE_0.6.md` (produced retroactively alongside `_0.7.md`, after Sprint 7's own close — see `HISTORICAL_RELEASE_COMPLETION_REPORT.md`).

## Sprint 7 — Complete (Multi-Source Evidence Collection: XML Inspection)

Sprint 7 closed the sole remaining candidate from the original three-item Sprint 6 roadmap. Rather than beginning with implementation, Sprint 7 began with a capability question — "what new capability should XML inspection provide?" — and, when that surfaced the platform's first need for more than one Collector to run per Assessment, was preceded by a dedicated Architecture Evaluation (`COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`) before any code was written. The approved architecture: Collection is a phase of an Assessment, not a standalone component; Collectors remain fully independent and never consume one another's output; composition is direct and inline inside `AssessmentService`, with no `CollectionCoordinator` introduced. `XmlCollector` now runs alongside the existing structural Collector for every Assessment, producing `XmlInspection` Evidence — manifest presence, well-formedness, and declared dependencies — with a missing manifest itself recorded as Evidence rather than silently treated as Empty Collection. No Rule Engine change; `modiq-rules` is untouched. New dependency: `roxmltree`. The workspace grew from 172 to 187 root tests (Sandbox 6 → 7). Implementation committed and pushed directly to `feature/runtime-implementation` (`277aefd`) — no separate Sprint branch was created this cycle, unlike Sprint 6.

Sprint 7 also consolidated the repository's engineering workflow: nine independent, drifted workflow descriptions (`ENGINEERING_WORKFLOW_CONSOLIDATION_STUDY.md`) reduced to one canonical source (`PROJECT_HANDOFF_v1.0.md`, Section 5), terminology unified project-wide, `ImplementationWorkflow.md` archived as non-normative history, and the Implementation Report template reconciled with demonstrated Sprint 6/7 practice (`ENGINEERING_WORKFLOW_CONSOLIDATION_REPORT.md`). The engineering methodology is now designated **Version 1.0** — exercised across multiple completed Sprints, consolidated into one canonical process, and expected to evolve only through the same evidence-based process used throughout this repository, not routine amendment.

Full record: `SPRINT7_CAPABILITY_AND_IMPLEMENTATION_PLAN.md`, `COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`, `SPRINT7_IMPLEMENTATION_AUTHORIZATION.md`, `SPRINT7_IMPLEMENTATION_REPORT.md`, `ENGINEERING_WORKFLOW_CONSOLIDATION_STUDY.md`, `ENGINEERING_WORKFLOW_CONSOLIDATION_REPORT.md`, and `ENGINEERING_RELEASE_0.7.md`. `ENGINEERING_RELEASE_0.6.md` and `_0.7.md` were both produced retroactively, after this Sprint's own close — see `HISTORICAL_RELEASE_COMPLETION_REPORT.md` for how, and for the process risk this gap itself represents.

**Next implementation milestone (superseded by Sprint 8, below):** Version Profile-aware compatibility checking — the candidate `REPOSITORY_CLOSEOUT_REPORT.md` (Sprint 7) named as the clearest concrete beneficiary of Sprint 7's own work.

## Sprint 8 — Complete (Version Profile-aware Compatibility Checking)

Sprint 8 activated the Version Profile architectural dimension — specified since Documentation Release 1.0 (`VersionProfile.md`, ADR-0004) but unimplemented through seven Sprints — as the platform's first capability directly answering `ProductSpecification.md`'s named Player objective, "Compatibility Verification." Preceded by the full Capability Definition → Architecture Evaluation → Architectural Resolution → Implementation Authorization sequence: `SPRINT8_INITIALIZATION_REPORT.md` (repository orientation), `SPRINT8_CAPABILITY_AND_IMPLEMENTATION_PLAN.md` (capability definition and repository evidence, including a direct correction of a prior session's inaccurate "declared FS version already exists" framing), `SPRINT8_ARCHITECTURAL_RESOLUTION.md` (six architectural decisions evaluated, none resolved), and `SPRINT8_IMPLEMENTATION_AUTHORIZATION.md` (decisions validated against fresh repository evidence, implementation roadmap, Builder-pattern evaluation).

`modiq-versioning` gained its first real content since Sprint 0: a minimal `GameVersion`/`VersionProfile` pair, with a single hardcoded `VersionProfile::fs25()` recognizing `descVersion` 93 — deliberately not exhaustive version knowledge. `XmlCollector` (Sprint 7) was extended to extract a mod's declared `descVersion` as a purely factual `XmlInspection` Evidence item, preserving the Collector Contract's observational boundary exactly. A new Rule, `VersionCompatibilityRule`, evaluates that declared value against the active Version Profile inside the Rule Engine — never upstream of it — producing a `Warning` Finding when unrecognized. `Assessment` records which Version Profile governed it through a new opaque `VersionProfileReference`, extending ADR-0007's established Opaque Runtime References pattern to a domain relationship for the first time; `modiq-runtime` gained no new dependency and remains the platform's sole dependency-free leaf, unbroken since Sprint 0. `AssessmentService`'s two public entry points (`execute`, `execute_from_assessment_input`) required **zero signature change** — both now execute every Assessment against `VersionProfile::fs25()` internally, an implementation simplification the Chief Architect accepted as part of this Sprint rather than the anticipated new additive entry point, since no second Version Profile yet exists for a caller to select between.

The workspace grew from 187 to 205 root tests (`modiq-versioning` 0 → 4, its first tests ever; `modiq-rules` 15 → 25; `modiq-runtime` 82 → 84; `modiq-collection` 56 → 57; `modiq-engine` 18 → 19 unit); Sandbox unchanged at 7/7, requiring zero source modification. Two new internal dependency edges only (`modiq-engine`/`modiq-rules` → `modiq-versioning`), no new external dependency, no breaking public API change. Full record: `SPRINT8_IMPLEMENTATION_REPORT.md`, `SPRINT8_IMPLEMENTATION_DEVIATIONS.md`, and `ENGINEERING_RELEASE_0.8.md`.

## Sprint 9 — Complete (Repair Guidance: Minimum Viable `modiq-knowledge` Activation)

Sprint 9 activated `modiq-knowledge` — real content since Sprint 0 — through the platform's first Repair Guidance capability, directly answering `Vision.md`'s own least-served founding Assessment question, "what can I do next?" Preceded by the full Capability Definition → Architectural Resolution → Implementation → Repository Review sequence, each producing its own reviewed document: `SPRINT9_CAPABILITY_DEFINITION.md`, `SPRINT9_ARCHITECTURAL_RESOLUTION.md`, `SPRINT9_REPOSITORY_REVIEW.md`. During Architectural Resolution review, the Chief Architect identified and corrected a conflation in the initial draft's Question 2 (where a `RepairRecipe` is retrieved versus where its content is authored) before implementation began — the resolution was revised so that `modiq-knowledge`, not the consuming Rule, authors engineering knowledge, consistent with the crate's own README boundary.

`RepairRecipe` gained real content: `identifier`/`guidance` fields, an infallible constructor, and one named, authored recipe, `RepairRecipe::version_compatibility_declared_version_mismatch()`, mirroring `VersionProfile::fs25()`'s exact precedent (a specific minimum-viable value authored inside the domain crate itself). `VersionCompatibilityRule` now calls this recipe rather than authoring guidance text inline, wiring a real `Some(RepairRecipeReference)` into its `Recommendation` in place of `None`. `RepairRecipeReference` — real, tested, and unused since Sprint 2 — required no modification, its third proven instance of ADR-0007's Opaque Runtime References pattern. `RuleEngine::evaluate` and `AssessmentService`'s two public entry points required **zero signature change**: the capability's entire footprint is one new dependency edge, `modiq-rules` → `modiq-knowledge`, mirroring Sprint 8's own `modiq-rules` → `modiq-versioning` edge.

The workspace grew from 205 to 210 root tests (`modiq-knowledge` 0 → 5, its first tests ever; `modiq-rules` and `modiq-engine` unchanged in count, both extended in place); Sandbox unchanged at 7/7. No new Governance Register item, no ADR, no `GOVERNANCE.md` amendment — the Knowledge Domain boundary already named Repair Recipes explicitly; no gap was found, unlike Sprint 8's `modiq-versioning` boundary gap. Full record: `SPRINT9_CAPABILITY_DEFINITION.md`, `SPRINT9_ARCHITECTURAL_RESOLUTION.md`, `SPRINT9_REPOSITORY_REVIEW.md`, and `ENGINEERING_RELEASE_0.9.md`.

**Next implementation milestone:** not yet scoped for implementation. The frozen roadmap (`SPRINT_ROADMAP_UPDATE_v1.md`) names Runtime Log Interpretation as the Sprint 10 candidate, contingent on Sprint 9's own successful closeout (complete) — Sprint 10 Capability Definition confirmed this against fresh repository evidence, per this project's own standing discipline.

## Sprint 10 — Complete (Runtime Fixture Corpus Acquisition)

**Sprint 10, as scoped and executed, delivered Capability Definition and a real evidentiary foundation for Runtime Log Interpretation — not the capability's own implementation.** No Rust source was touched; the root workspace test suite remains unchanged at 210/210 (Sandbox 7/7). This is a deliberate, explicit scope boundary, not an incomplete Sprint: `SPRINT10_CAPABILITY_DEFINITION.md`, Section 11 established that no architectural or implementation decision for this capability may assume the structure, wording, stability, or formatting of a real Farming Simulator runtime log — acquiring and validating representative logs was named as the first engineering activity that requirement demands, and this Sprint completed exactly that activity.

`SPRINT10_CAPABILITY_DEFINITION.md` scoped the capability to recognizing one class of signal (a mod failed to load), grounded in `Vision.md`'s own named question, "why does it fail to load?" A dedicated, permanent, provenance-tracked fixture corpus (`fixtures/runtime-logs/`) was designed and built — separate from `apps/sandbox/src-tauri/fixtures/`'s own, unrelated synthetic-fixture convention — with a consistent per-fixture metadata schema (`TEMPLATE.md`). All three of the corpus's initial fixtures were captured, normalized, and integrated as real evidence: `clean-base-game` (a verified mod-free baseline — reclassified once, after acquisition revealed a new savegame does not clear the global mods directory's own enumeration), `single-compatible-mod` (a successful third-party mod load, the negative control), and `single-incompatible-mod` (a real, fully evidenced failure: a mod's declared `descVersion` exceeding the runtime's own recognized version, rejected at modDesc validation before registration or asset loading ever began). No mod archive was stored in the repository at any point — only runtime logs and independently verified factual metadata about the mods that produced them.

Real acquisition evidence twice revealed genuine gaps in the corpus's own documentation, both formalized before the fixture that exposed them was integrated, not worked around silently: **Installation State versus Savegame State** (a runtime log enumerates the global mods directory regardless of which savegame is active — `TEMPLATE.md`'s schema was split into `Installed Mods (global)` and `Savegame Mod State`), and **Warning Categorization** (every warning a fixture's log contains is now classified as a `Base-game warning`, a `Fixture warning`, or a `Fixture-affecting warning` — attribution-based categories, not a severity scale, consistent with this project's evidence-first discipline). A **Runtime Log Normalization** policy was also formalized, requiring that personally identifying or machine-specific information be replaced with a fixed, deterministic placeholder before any fixture is committed, without altering parser-relevant content, runtime semantics, ordering, line counts, or formatting.

No crate, test, Collector, Rule, Evidence category, Governance Register item, or ADR was touched. Full record: `SPRINT10_CAPABILITY_DEFINITION.md`, `SPRINT10_RUNTIME_LOG_FIXTURE_PREPARATION.md`, and `ENGINEERING_RELEASE_1.0.md`.

**Next implementation milestone (superseded by Sprint 11, below):** Runtime Log Interpretation's own Architectural Resolution and implementation, now grounded in three real, captured fixtures rather than assumption.

## Sprint 11 — Complete (Runtime Evidence Processing Architecture)

Sprint 11 defined and then implemented the Runtime Evidence Processing Architecture — the first Sprint since Sprint 10's own fixture acquisition to touch Runtime Log Interpretation at all. `RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md` resolved, grounded directly in Sprint 10's three real fixtures: where a runtime observation enters the pipeline (a second content-Collector, composed inline alongside `XmlCollector`, per the already-approved Collector Composition Architecture — its five-condition extraction threshold not yet met by a second participant); `EvidenceCategory::RuntimeLogs`'s content shape (a factual, template-bounded capture, mirroring `XmlCollector`'s Content Extraction discipline); how the evidence is interpreted (a new Rule, fourth in GOV-012's fixed dispatch order, `FindingSeverity::Error` — the platform's first real use of that variant, reasoned directly from `DataModel.md`'s own Finding Severity definitions rather than deferred to the game engine's own log wording); and the Runtime Interpretation Decision Matrix (four rows, each citing real fixture evidence).

Implementation followed across four independently reviewed milestones: `RuntimeLogCollector` (`modiq-collection`), its wiring into `AssessmentService::execute_from_assessment_input`, `RuntimeLoadFailureRule` (`modiq-rules`), and its dispatch integration into `RuleEngine::evaluate`. A missing or unrecognized runtime log is Legitimate Absence, never a recorded fact — a deliberate, reasoned divergence from `XmlCollector`'s own missing-manifest precedent, since a log's ordinary case is absence, unlike a manifest's. `AssessmentService`'s two public entry points and `RuleEngine::evaluate`'s parameter shape both required zero change.

A dedicated, adversarial engineering verification pass — conducted specifically to attempt to disprove architectural consistency rather than confirm it — found a genuine internal contradiction between the architecture document's own Architectural Invariants section and the Collector actually implemented against an earlier section of the same document. Implementation was halted and the contradiction reported, per this project's standing discipline, rather than resolved unilaterally either way. Chief Architect review confirmed the implementation was correct and revised the architecture document (v1.1.0 → v1.2.0) to describe the Collector-recognizes-then-Evidence-exists model the implementation already realized — no Rust source was touched in that reconciliation.

The workspace grew from 210 to 238 root tests (`modiq-collection` 57 → 70; `modiq-rules` 25 → 36; `modiq-engine` 19 → 23 unit); Sandbox unchanged at 7/7. No crate added, no public entry point changed, no new `EvidenceCategory` variant, no new external or internal dependency edge, no new Governance Register item, no ADR. Full record: `SPRINT11.md`, `RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md` (v1.2.0), and `ENGINEERING_RELEASE_1.1.md`.

**Next milestone (superseded by Sprint 12, below):** an explicit Capability Identity procedure was named as necessary before any further runtime capability could responsibly be scoped, so the platform's first capability-scaling questions would not have to be re-derived, from first principles, under future implementation pressure.

## Sprint 12 — Complete (Capability Scaling Architecture)

Sprint 12 was architecture-only, by explicit charter — no Rust source, test, or fixture was in scope, and none was touched. Its objective was to derive an explicit Capability Identity decision procedure from this platform's own completed engineering history (not from theory), distinguishing Capability Enrichment, Capability Expansion, and Capability Introduction, and to resolve Collector Composition and Rule Composition strictly as its consequences — replacing a prior draft objective that would have asked those two questions directly, which Sprint 11's own Chief Architect review found presupposed an unstated classification.

The first draft of that procedure (`SPRINT12.md` v1.0.0) was itself found, during this Sprint's own adversarial verification pass, to contain a genuine contradiction: it conflated `EvidenceCategory` novelty with Collector novelty into a single cascading test. Sprint 4 Phase 3C (`EvidenceCategory::StructuralDuplication`, GOV-011) falsifies that conflation directly — a genuinely new category, produced by *extending* the already-existing `ArchiveCollector`, not by adding a new one. Per this project's standing discipline, the contradiction was reported rather than silently resolved; on Chief Architect confirmation, the procedure was corrected to three independent axes — **Collection Axis**, **Evidence Axis**, **Interpretation Axis** — plus one orthogonal check reserving "Capability Introduction" for capabilities that require inventing composition or dispatch machinery the platform has never exercised before, independent of how novel any single axis is.

The direct, most consequential outcome of that correction: **Sprint 11 is reclassified from Capability Introduction to Capability Expansion**, because it reused Sprint 7's own already-proven additive-composition and GOV-012's own fixed-order dispatch machinery, rather than inventing new machinery — recorded with an explicit, repeated distinction between architectural classification and product significance, so this reclassification is never read as diminishing Sprint 11's own real, considerable product milestone (the platform's first event-based Evidence source and first use of `FindingSeverity::Error`). A new, previously-implicit piece of Collector guidance was also recorded: mutually-exclusive routing versus additive composition, discriminated by whether two Collectors' own applicability conditions can co-occur on the same Assessment Input — independent of, and not a replacement for, the Collector Composition Architecture's own extraction threshold, which remains unchanged.

All seven historical decisions evaluated (Sprint 3, 4, 4 Phase 3C, 7, 8, 9, 11) classify without contradiction under the corrected model. No crate, dependency, test, fixture, `EvidenceCategory`, Governance Register item, or ADR was touched at any point. Full record: `SPRINT12.md` (v1.1.0), `SPRINT12_ARCHITECTURAL_RESOLUTION.md`, and `ENGINEERING_RELEASE_1.2.md`.

**Next milestone:** not yet scoped. The Capability Proposal template (Observation, Evidence Source, Evidence Sufficiency, Expected User Value, Capability Identity Assessment, Architectural Consequence) is now the standing entry point for whichever future capability is proposed next — a second runtime-log signature, Lua analysis, asset validation, dependency resolution, and performance observations all remain named, unscoped candidates.

## Post-Sprint 12: INV-001 (Lua Analysis Capability Investigation) — Complete

**Not Sprint 13. Not architecture. Not implementation.** An evidence-acquisition investigation into whether a real, bounded, deterministic Lua Analysis capability exists for modIQ to recognize, following the accepted conclusion of an earlier capability proposal that evidence was insufficient for Architecture Evaluation. Full record: `docs/engineering/INV-001_LUA_ANALYSIS_CAPABILITY.md`.

**Findings:** no real Farming Simulator mod Lua script has ever been examined by this project, and none can be obtained without a human performing the same real, licensed acquisition Sprint 10 required for runtime log fixtures. Of the candidate defect classes named at the investigation's outset, only Lua syntax validity passes every determinism and Collector-boundary test — every broader target either fails determinism outright (Lua's own dynamic scoping defeats naive "undefined global" detection) or risks the Collector/Rule observational boundary. A safe, mature, parse-only Lua dependency (`full_moon`) was identified and execution-capable alternatives (`mlua`/`rlua`) were correctly ruled out, independent of any sample acquisition.

**Recommendation A selected: evidence remains insufficient; further investigation required.** Architecture Evaluation has **not** been authorized. Sprint Planning has **not** begun. The concrete next step — human-performed acquisition of real, licensed Lua scripts, mirroring Sprint 10's own precedent — is itself gated on **GOV-014 (Lua Fixture Acquisition Governance)**, opened as a direct consequence of this investigation (`GOVERNANCE.md`): provenance, licensing, storage, and acquisition-governance questions specific to real third-party source code have not yet been resolved, and Sprint 10's own runtime-log fixture policies do not automatically transfer to a structurally different kind of artifact.

**No Rust source, test, fixture, or ADR was touched.** One Governance Register item (GOV-014) was opened as an explicit, separate governance action — the Register now totals 14 items, 8 Resolved, 6 Open.

## Sprint 13 — Complete (Storage Architectural Activation)

Sprint 13 gave the Storage subsystem — named in `Architecture.md`'s own System Overview since the document's own baseline, never implemented — its first real content, following a distinct capability lineage from every prior Sprint since Sprint 8: not a Capability Identity classification (the Sprint 12 procedure), but a rediscovered, older one.

**Investigation and governance reconciliation.** `INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md` established Storage as a valid, well-evidenced capability candidate (`Vision.md`'s "Historical knowledge preservation," `Principles.md`'s "Knowledge Preservation," `ProductSpecification.md`'s MKB content list and Assessment Workflow step 7) — but found the Sprint 12 Capability Identity procedure could not classify it: none of its three axes or its Introduction test apply to a subsystem-level candidate, only to Collector/Rule-shaped ones. `GOVERNANCE_OBSERVATION_SUBSYSTEM_ACTIVATION.md` generalized this finding and, on Chief Architect review, surfaced that the repository already contained the answer: `SPRINT8_ARCHITECTURAL_RESOLUTION.md` §8 had already named and applied **Architectural Activation** to Version Profiles' own first real content, a classification Sprint 12's own historical derivation never checked itself against. `PROJECT_HANDOFF_v1.1.md` §5 was amended with a two-sentence scope clarification: the Capability Identity gate applies to Collector/Rule-shaped candidates specifically; subsystem-level activation follows Architectural Activation directly to Architecture Evaluation instead — introducing no new procedure, taxonomy, or architectural decision, only reconnecting two already-existing ones.

**Architecture Evaluation, Resolution, and Authorization.** `STORAGE_ARCHITECTURE_EVALUATION.md` evaluated four decisions with alternatives: the persisted domain object (`AssessmentReport`, not `Assessment` itself or individually addressable Evidence/Finding/Recommendation records), subsystem ownership (a new `modiq-storage` crate — the one respect in which this activation differs from Sprint 8/9's own precedent, since no such crate existed even as an empty scaffold), the lifecycle boundary (strictly downstream of Reporting, agnostic to GOV-001's still-Open timing question), and impact on `AssessmentService`'s public entry points (none, mirroring how `modiq-cli`/`apps/sandbox` already depend on `modiq-report` directly). All four were Accepted without modification. `STORAGE_IMPLEMENTATION_AUTHORIZATION.md` translated the Resolution into an engineering envelope — deliberately narrower in scope than Sprint 8's own equivalent document, excluding Sprint sequencing entirely, a scope difference recorded explicitly as reflecting the repository's own evolved workflow separation, not any difference in Storage's own requirements.

**A design conflict, found and resolved before implementation.** Preparing Phase 1 surfaced that `AssessmentReport` and its nested Runtime types (`Evidence`, `Finding`, `Recommendation`, and their identifier types) have no `Serialize`/`Deserialize` derive and, more fundamentally, that `AssessmentId`/`EvidenceId`/`FindingId` expose no accessor and are generated from a process-local counter that restarts at 1 on every invocation — meaning Runtime identity is not a durable concept across a process boundary at all, independent of any storage design. Rather than modify `modiq-runtime`/`modiq-report` to add this capability (which the Sprint Plan's own scope excluded), `STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md` resolved this within `modiq-storage`'s own boundary: Storage defines and owns its own persisted representation of a report's content — the fourth instance of ADR-0007's Opaque Runtime References pattern, populated from `AssessmentReport`'s already-public getters, with Finding→Evidence and Recommendation→Finding cross-references preserved as positions within the persisted report itself rather than the original process-local identifiers. Faithfulness is judged by content and order, consistent with this platform's own standing determinism convention ("judged by content and order, never by incidental identity").

**Implementation, across three phases, each independently validated.** Phase 1 gave `modiq-storage` its first real content: `PersistedAssessmentReport` and its nested types, `ReportKey` (an opaque identifier Storage mints itself, since `AssessmentId` cannot serve this role), and `ReportStore` (real filesystem-backed write/read, no new external dependency). Phase 2 wired `modiq-cli`: `AssessCommand` now stores a successful assessment's report, reporting the key; a new `retrieve` command reads a report back independent of running a new Assessment; a storage failure is reported as a warning, never as a change to the assessment's own exit code. Phase 3 wired `apps/sandbox` identically — `create_assessment` reports a `stored_report_key`; a new `retrieve_report` Tauri command mirrors `modiq-cli`'s own. Both integration phases were verified with a genuine cross-process round trip, not only in-test coverage — `modiq-cli`'s own real binary, run as two separate process invocations. `modiq-runtime`, `modiq-report`, `modiq-engine`, `modiq-rules`, `modiq-versioning`, and `modiq-knowledge` are unmodified throughout; `AssessmentService`'s two public entry points are untouched.

The root workspace grew from 238 to 253 tests (`modiq-storage` 0 → 10, its first tests ever; `modiq-cli` 10 → 15); Sandbox grew from 7 to 9. `.gitignore` gained `.modiq-storage/`, since both `modiq-cli`'s and the Sandbox's own default storage locations can now write real files during normal use. `GOVERNANCE.md` gained `modiq-storage`'s own Crate Boundary Rule pair. No Governance Register item and no ADR were opened this Sprint — the governance reconciliation clarified an existing rule's scope rather than establishing a new one. Full record: `INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md`, `GOVERNANCE_OBSERVATION_SUBSYSTEM_ACTIVATION.md`, `STORAGE_ARCHITECTURE_EVALUATION.md`, `STORAGE_IMPLEMENTATION_AUTHORIZATION.md`, `STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md`, `STORAGE_SPRINT_PLAN.md`, and `ENGINEERING_RELEASE_1.3.md`.

**Next milestone:** not yet scoped. Storage's own minimum-viable slice deliberately leaves two product-forcing-functions undecided — cross-mod collection validation and MKB accumulation from real Assessments — either of which could motivate the next capability, alongside Lua Analysis (still gated on GOV-014 and human fixture acquisition) and Extension Layer, the platform's other remaining dormant System Overview subsystem.

## Post-Sprint 13: GOV-001 (Assessment Report Generation Timing) Architecture Evaluation — Complete

**Not Sprint 14. Not an Architectural Resolution.** `SPRINT_14_PROPOSAL.md` (Candidate A) recommended resolving GOV-001 on the strength of a forcing function `modiq-storage`'s own Sprint 13 activation created: a manually smoke-tested, durably persisted report was observed showing `status: EvaluatingRules`, not `Completed`. Before any Architectural Resolution, the Chief Architect directed a determination of `AssessmentReport`'s own semantic contract, and whether its current generation point satisfies it.

**Finding:** No inconsistency exists between specification and implementation. `DataModel.md`'s own canonical Runtime Lifecycle diagram documents "Assessment Report Produced" as the step immediately preceding "Assessment Completed" — the specification does not merely permit pre-completion generation, it specifies it as the intended sequence. Both `AssessmentService::execute` and `execute_from_assessment_input` call `AssessmentReport::generate` exactly once, immediately before `Assessment::complete()`, exactly matching the documented sequence — enforced by the existing `execute_reflects_state_at_report_generation_prior_to_completion` test. `Assessment::complete()` itself was confirmed to be a pure status-field transition; `modiq-report`'s own `generate_after_completion_matches_generate_before_completion` test already proves Evidence, Findings, and Recommendations are identical before and after completion.

**Disposition:** no Architectural Resolution was performed or required. GOV-001 was narrowed, not closed: because both public entry points generate exactly once, always before completion, and never again afterward, `AssessmentStatus::Completed` is a value no report produced by either real entry point can ever hold in practice. The item's original question ("before or after completion?") is already answered by the specification and satisfied by the implementation; the real, narrower, still-open question is whether leaving `Completed` permanently unreachable by any real report was an intended consequence of that documented ordering, or simply never consequential enough to examine while reports were ephemeral. Returned to Open under this narrower framing, deliberately, mirroring GOV-008's and GOV-013's own standing treatment — no forcing function currently requires deciding the status field's own semantics; none is manufactured here. Full record: `docs/engineering/ENGINEERING_LOG.md` ("GOV-001 Architecture Evaluation — Post-Sprint 13, Not Resolved").

---

# Documentation Progress

## Phase 1 — Foundation

### Repository

| Item | Status |
|------|--------|
| Repository Foundation | ✅ Complete |
| Documentation Standards | ✅ Complete |
| Governance | ✅ Complete |

### Constitutional Specifications

| Document | Status |
|----------|--------|
| Vision.md | ✅ Frozen |
| Principles.md | ✅ Frozen |
| Glossary.md | ✅ Frozen |
| ProductSpecification.md | ✅ Frozen |

### Platform Architecture

| Document | Status |
|----------|--------|
| Architecture.md | ✅ Frozen |

---

## Phase 2 — Technical Specifications

| Document | Status |
|----------|--------|
| DataModel.md | ✅ Frozen |
| KnowledgeModel.md | ✅ Frozen |
| RuleEngine.md | ✅ Frozen |
| EngineAPI.md | ✅ Frozen |

---

## Phase 3 — Implementation Planning

| Document | Status |
|----------|--------|
| Sprint0.md | ✅ Frozen |
| Sprint1.md | ✅ Frozen |

---

# Documentation Release Status

| Property | Value |
|----------|-------|
| **Current Release** | Documentation Release 2.1 |
| **Status** | ✅ Complete (Frozen) |
| **Constitutional Layer** | ✅ Frozen |
| **Architecture Layer** | ✅ Frozen (amended under 2.1 — Evidence Collection named a Core Platform Component; Assessment Lifecycle diagram ordering corrected) |
| **Technical Layer** | ✅ Frozen (new: `EvidenceCollection.md`) |
| **Governance Documentation** | ✅ Reconciled |
| **Repository Audit** | ✅ Passed |
| **Implementation Readiness** | ✅ Verified |
| **Previous Release** | Documentation Release 2.0 |

Governance documentation (EngineeringGuide.md, CHANGELOG.md, ROADMAP.md, PROJECT_STATUS.md, and related tracking documents) is reconciled with current repository state as of this release. Unlike the constitutional and architecture layers, governance documentation remains a living body of documentation per EngineeringGuide.md and is not designated Frozen — and, as `ENGINEERING_RELEASE_0.3.md`'s Repository Health section notes directly, has a documented history of falling behind between releases despite that.

---

# Current Focus

Documentation Releases 1.0, 2.0, and 2.1 have all concluded; Documentation Release 2.1 (Evidence Collection subsystem boundary) is the current one, amended three times since its own freeze to record GOV-011's resolution and its Sprint 4 Phase 3C/3D implementation (`EvidenceCollection.md` v1.2.0 → v1.4.0). `DataModel.md` was separately amended to v1.1.0 during Sprint 5 to add Finding Severity definitions (GOV-012/GOV-013). Engineering Release 0.4 froze Sprint 4 (Phases 1–3D, plus Closeout); Engineering Release 0.5 froze Sprint 5 (Phases 1–5, plus Closeout). Platform Validation Phase 1 closed before Sprint 4 began, confirming the engine architecture (GOV-004) and deferring GOV-008 pending future evidence — GOV-008 remains open, untouched by Sprints 4, 5, and 6 (Sprint 6 reused `AssessmentService`'s existing entry points exactly as designed and generated no new evidence toward it). Sprint 6 is complete: `modiq-cli` is wired to `modiq-engine`, and `modiq-report`'s four scaffold types are retired. XML inspection is the sole remaining candidate from the original three-item Sprint 6 roadmap, not yet scoped for Sprint 7.

Sprint 7 (Multi-Source Evidence Collection: XML inspection), Sprint 8 (Version Profile-aware compatibility checking), Sprint 9 (Repair Guidance), Sprint 10 (Runtime Fixture Corpus Acquisition), Sprint 11 (Runtime Evidence Processing Architecture and Implementation), and Sprint 12 (Capability Scaling Architecture) are all complete since this paragraph was last current. None required a Documentation Release amendment; Documentation Release 2.1 remains the current release. GOV-008 remains open, unaffected by Sprints 9 through 12 — none changed `AssessmentService`'s two public entry points. Sprint 9 gave `modiq-knowledge` its first real content since Sprint 0; Sprint 10 gave the repository its first real, external, provenance-tracked evidence corpus (`fixtures/runtime-logs/`); Sprint 11 activated that corpus's use; Sprint 12 gave the repository its first explicit, historically-validated Capability Identity procedure, found — through the same adversarial verification discipline Sprint 11 itself established — to require one correction before it could be trusted, and reclassified Sprint 11 from Capability Introduction to Capability Expansion as a direct consequence.

**Following Sprint 12's own closeout, INV-001 (Lua Analysis Capability Investigation) completed as a non-Sprint evidence-acquisition activity** — no architecture, no implementation, no Sprint 13 work. It found that a Lua Analysis capability's only currently-deterministic recognition target (syntax validity) cannot be evaluated further without real, human-acquired Lua script samples, and opened GOV-014 (Lua Fixture Acquisition Governance) to resolve the provenance, licensing, storage, and acquisition-governance questions that acquisition would raise, before any such acquisition begins. Full record: `docs/engineering/INV-001_LUA_ANALYSIS_CAPABILITY.md`.

**Sprint 13 (Storage Architectural Activation) is complete.** It followed a capability lineage distinct from INV-001's own: not Sprint 12's Capability Identity procedure, but Sprint 8's own "Architectural Activation" precedent, rediscovered and reconnected to the standing workflow via a two-sentence `PROJECT_HANDOFF_v1.1.md` §5 amendment. `modiq-storage` is now real — a durable, cross-process-verified write/read capability for `AssessmentReport`'s content, wired through both `modiq-cli` and `apps/sandbox` — without any change to `modiq-runtime`, `modiq-report`, `modiq-engine`, `modiq-rules`, `modiq-versioning`, `modiq-knowledge`, or `AssessmentService`'s two public entry points. Full record: `docs/engineering/ENGINEERING_RELEASE_1.3.md` and the Sprint 13 milestone entry above.

**Following Sprint 13's own closeout, GOV-001 (Assessment Report Generation Timing) underwent an Architecture Evaluation** — not Sprint 14, not an Architectural Resolution — directed by the Chief Architect on the strength of the forcing function Storage's own durable persistence created. The evaluation found no inconsistency between `DataModel.md`'s specified Runtime Lifecycle and the implementation's actual behavior; no Architectural Resolution was performed or required. GOV-001 was narrowed and returned to Open, mirroring GOV-008's and GOV-013's own standing treatment. Full record: `docs/engineering/ENGINEERING_LOG.md` and the Post-Sprint 13 milestone entry above.

Implementation should remain consistent with the frozen engineering specification.

---

# Implementation Policy

The documentation contained within this repository is the authoritative engineering specification for modIQ.

Implementation should reconcile all engineering decisions against the frozen specifications.

Architectural changes should be introduced through Architecture Decision Records (ADRs) and incorporated into future Documentation Releases rather than modifying Documentation Release 1.0.

---

## Governance Status

Status: Frozen for Sprint 6, carried into Sprints 7 through 13 (all eight complete); baseline carries forward unchanged into Sprint 14, which has not begun. Engineering Methodology Version 1.0 declared following Sprint 7 — a versioning of the workflow itself, distinct from this governance baseline. Sprint 10 introduced no Governance Register item and no ADR. Sprint 11 likewise introduced none. GOV-013 (FindingSeverity Severity/Kind Conflation) remains Open, deliberately not reopened by Sprint 11's own `FindingSeverity::Error` assignment. Sprint 12 also introduced no new Governance Register item and no ADR — its own Capability Identity procedure applies and extends already-approved architecture rather than establishing a new durable principle requiring one. Following Sprint 12's own closeout, INV-001 (Lua Analysis Capability Investigation) opened one new Governance Register item — GOV-014 (Lua Fixture Acquisition Governance), Open — as an explicit, separate governance action, not Sprint work. **Sprint 13 introduced no new Governance Register item and no ADR either — its own governance work amended `PROJECT_HANDOFF_v1.1.md` §5 with a two-sentence scope clarification, reconnecting Sprint 8's pre-existing "Architectural Activation" classification to the standing workflow, rather than establishing anything new.** The Governance Register remains at **14 items, 8 Resolved, 6 Open** — unchanged by Sprint 13. The `modiq-versioning` Crate Boundary Rules gap named during Sprint 8 planning remains open, unaffected by Sprints 9 through 13. `modiq-storage` gained its own Crate Boundary Rule pair this Sprint, recorded directly, not left as a gap the way `modiq-versioning`'s own has been. **Post-Sprint 13, GOV-001's own Description, Question, and Resolution fields were updated** to record a completed Architecture Evaluation (no Architectural Resolution performed) and a narrower open question; the Register's own count remains 14 items, 8 Resolved, 6 Open — GOV-001 stays counted among the Open items, unchanged in count, changed in precision.

The project's governance baseline is established.

Authoritative governance documents:

- PROJECT_HANDOFF_v1.1.md
- CHIEF_ARCHITECT_HANDOFF_v1.1.md
- LEAD_ENGINEER_HANDOFF_v3.0.md

Future governance changes must be justified by implementation evidence,
architectural evolution, or approved governance decisions.

Sprint 14 begins under this governance baseline.

---

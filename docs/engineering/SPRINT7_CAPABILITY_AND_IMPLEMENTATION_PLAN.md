# Sprint 7 Capability & Implementation Plan

| Property | Value |
|----------|-------|
| **Document** | SPRINT7_CAPABILITY_AND_IMPLEMENTATION_PLAN.md |
| **Project** | modIQ |
| **Purpose** | Capability-first definition and implementation plan for Sprint 7 — for Chief Architect review and authorization |
| **Prepared by** | Lead Engineer, on `feature/runtime-implementation` |
| **Repository baseline** | `feature/runtime-implementation`, HEAD `af65bf0` |
| **Status** | Collection architecture approved (`COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`, Section 14) and integrated below. Remaining open questions (Rule Engine timing, dependency selection, absence-as-evidence) are unchanged by that approval. No code changed. No commits created. No branch created. Awaiting final Chief Architect authorization before implementation begins. |

---

# Capability Definition

## What problem does XML inspection solve?

Every Evidence category real today — `FileStructureAnalysis`, `StructuralDuplication` — is purely structural: *something exists*, or *something couldn't be fully resolved*. Neither says anything about what a mod actually **is**. Verified directly against a real assessment run this session (`modiq-cli assess <dir>`): the only Finding a real Assessment can currently produce is "Evidence was collected for this Assessment" — true, but content-free. modIQ cannot yet answer any version of the question `Vision.md` exists to answer: *does this mod work, and why?*

`modDesc.xml` is Farming Simulator's mod manifest — the one file every loadable mod is required to have, declaring its name, description, targeted game version, and declared dependencies. It is the first place a mod says anything about itself in a structured, machine-readable form. XML inspection is the capability to read that self-description and turn it into Evidence.

## Why is this the correct next capability?

Grounded directly in the frozen `ProductSpecification.md`, not inferred:

- **"Dependency identification"** is named explicitly as a Player objective. Nothing implemented today can identify a single declared dependency — Evidence Collection has never read the content of any file, only confirmed files exist.
- **"XML validation"** and **"Dependency inspection"** are both named, separately, as Assessment Engine responsibilities — confirming this project's own product specification already treats them as two distinct capabilities, not one bundled effort. This directly shapes scope, below.
- This is also the platform's own stated architectural sequencing: `EvidenceCollection.md`'s Future Evolution section names "structured-text inspection" and "manifest inspection" as the next kind of Collector after archive traversal, and its precondition — a Rule Engine mature enough not to be driven by this Collector's design — has held since Sprint 5.

## What new evidence becomes available?

Structural, factual observations about `modDesc.xml`'s content — not interpretation of it (`EvidenceCollection.md`'s Non-Responsibilities apply to this Collector exactly as to every other): whether the file exists at all, whether it is well-formed XML, whether required elements/attributes are present, and what dependency declarations it names. `EvidenceCategory::XmlInspection` already exists in `modiq-runtime` — declared, unused, since before Sprint 1 — and needs no change to accommodate this.

## How will that evidence improve an Assessment Report?

Directly, without any change to `AssessmentReport`'s shape: new `XmlInspection` Evidence flows through the same Evidence list every consumer (Sandbox, `modiq-cli`) already renders. Whether it also produces a *new, distinguishable Finding* (as opposed to being folded into the existing generic "Evidence was collected" Finding) is a real scope decision — addressed in Architectural Design and flagged as a Chief Architect Review Question below, not assumed.

## Which future capabilities depend on this work?

- **Version Profile-aware compatibility checking.** `modDesc.xml` declares the FS version(s) a mod targets. `modiq-versioning` has gone six Sprints with zero implementation and zero forcing function (`LEAD_ENGINEER_HANDOFF_v3.0.md`, Open Engineering Risks) — this is the first capability that would give it real data to act on. Not this Sprint's scope; named because it's the clearest concrete future dependency on this work.
- **True dependency resolution** — checking a declared dependency against a broader mod collection (Server Administrators' "Validate complete mod collections," "Identify conflicting mods"). Requires multi-mod, cross-Assessment awareness modIQ does not have today. XML inspection produces the raw declared-dependency facts a future capability like this would consume; it does not itself resolve anything.
- **Knowledge Base-backed Repair Recipes** for common manifest mistakes (missing version, malformed dependency declaration) — depends on Knowledge Domain integration, still zero-implementation, still correctly deferred.

## What deliberately remains out of scope

- **Lua script inspection** — `EvidenceCollection.md` names this as a separate future Collector ("the platform's highest-risk future collector"), not this one.
- **True dependency resolution** (cross-referencing declared dependencies against other mods or an environment) — a distinct, future, multi-mod capability, not single-Assessment XML inspection.
- **Version Profile integration** — `modiq-versioning` remains untouched; this Sprint produces evidence a future Version Profile capability could consume, it does not build that capability.
- **Knowledge Domain interaction of any kind** — architecturally prohibited for a Collector by `EvidenceCollection.md` itself (Relationship to Other Specifications: "Evidence Collection does not consume Rules, Findings, Recommendations, or Engineering Knowledge, and does not produce them"). Not a scope choice — a boundary already frozen.
- **Any UI or CLI-specific XML-aware rendering** beyond what already exists generically (`{:?}` formatting, unchanged).

---

# Repository Assessment

Reviewed directly against the current tree, not assumed from prior sessions:

- `EvidenceCategory::XmlInspection` exists in `crates/modiq-runtime/src/assessment/evidence_category.rs`, exercised only by `Evidence`'s own unit tests (`evidence.rs`) — real test fixtures already model the exact shape this Collector would produce (`Evidence::with_location(EvidenceCategory::XmlInspection, "invalid moddesc version attribute", "modDesc.xml:3")`), evidence this was anticipated architecturally, never scoped for implementation.
- `Evidence`'s shape (`category`, `description: String`, `location: Option<String>`) needs no change — every fact this Collector would produce fits the existing constructor.
- `modiq-collection`'s two real Collectors (`EvidenceCollector`, `ArchiveCollector`) both follow an identical shape: a unit struct, a `collect` method, real-I/O tests. A third Collector has a template to follow, not a design to invent from nothing.
- **`EvidenceCollection.md`'s Collector Contract states a Collector "receives an Assessment Input... and Relevant Assessment Context. A Collector receives nothing else."** This is the single most consequential fact this review surfaced — see Architectural Decisions below for why.

**Confirming XML inspection remains the highest-value next objective:** yes, on the same evidence the Post-Sprint 6 Assessment already established — it is the sole remaining item from the original three-candidate Sprint 6 roadmap, its precondition (Rule Engine maturity) has held since Sprint 5, and it is now additionally the only path to the explicitly-named "Dependency identification" product objective. Nothing discovered in this review suggests a better-value alternative. `modiq-knowledge`, `modiq-versioning`, and `modiq-common` remain correctly deferred — none has a forcing function XML inspection doesn't itself help create.

---

# Architectural Design

## Participating crates

| Crate | Change |
|---|---|
| `modiq-collection` | New `XmlCollector` (or equivalent), same shape as `EvidenceCollector`/`ArchiveCollector` |
| `modiq-runtime` | None expected — `EvidenceCategory::XmlInspection` and `Evidence` already accommodate this |
| `modiq-engine` | Orchestration change — see below, the central open question |
| `modiq-rules` | Possibly a new Rule — open question, see below |
| `modiq-report` | None expected — `AssessmentReport`'s existing Evidence/Finding/Recommendation lists already carry this |
| `modiq-cli`, `apps/sandbox` | None expected — both already render whatever Evidence/Findings exist generically |

## Collector composition — **RESOLVED, architecture approved**

See `COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`, Section 14 (Chief Architect Decision Record), for the full study and approved decision. Summary: Collection is a phase of an Assessment, not a standalone component. Multiple independent Collectors may participate in one Assessment; Collectors never invoke, consume, or depend on one another. `AssessmentService` temporarily owns orchestration — determining participants and aggregating Evidence — as direct inline composition, no `CollectionCoordinator` component. XML inspection is **the first implementation of Multi-Source Evidence Collection**, not a special case of its own; the architecture below is general.

The analysis that followed this question to its resolution is preserved below, unaltered, as the record of how it was reached.

### Original framing (historical — see resolution above)

Every Collector composition decision to date has been **mutually exclusive**: a location either ends in `.zip` (routed to `ArchiveCollector`) or it doesn't (routed to `EvidenceCollector`) — exactly one Collector runs per Assessment. XML inspection does not fit that shape. A mod directory can simultaneously need filesystem discovery (structure) **and** manifest inspection (content) — these are complementary, not alternative. This is the first time this platform has needed more than one Collector to run for a single Assessment.

The Collector Contract makes the design space narrower than it might first appear: a Collector "receives nothing else" beyond an Assessment Input and Assessment Context — explicitly **not** Evidence another Collector already produced. This rules out the seemingly-convenient design where `XmlCollector` operates on a `modDesc.xml` location `EvidenceCollector` already discovered; that would make one Collector consume another's output, a boundary `EvidenceCollection.md` does not permit.

The design this review recommends, consistent with that boundary: `XmlCollector` receives the **same** Assessment Input as the structural Collector and independently determines, on its own, whether a `modDesc.xml` exists at that input's root (a file directly, or an archive entry at the root, depending on input kind) — mirroring how `ArchiveCollector` already independently interprets the same kind of input `EvidenceCollector` would otherwise receive, rather than consuming `EvidenceCollector`'s output. `modiq-engine`'s orchestration would invoke both the structural Collector and `XmlCollector` for the same Assessment Input, aggregating both Collectors' Evidence before Rule evaluation — the same "each Rule evaluated independently, no suppression" discipline GOV-012 established for Rules, applied for the first time to Collectors.

**This was a genuine architectural decision, not an implementation detail** — it was raised as the first Chief Architect Review Question below and is now resolved, exactly as recommended here, per the Decision Record referenced above.

## Data flow

```
Application supplies one Assessment Input
        │
        ▼
AssessmentService routes: structural Collector selection unchanged
(EvidenceCollector or ArchiveCollector, by existing .zip check)
        │                                   │
        ▼                                   ▼
Structural Collector runs           XmlCollector runs
(independently, same Input)         (independently, same Input)
        │                                   │
        └───────────────┬───────────────────┘
                         ▼
      Combined Evidence added to the Assessment
                         │
                         ▼
       Rule Engine evaluates the full Evidence set
                         │
                         ▼
              AssessmentReport (unchanged shape)
```

## Ownership boundaries

Unchanged, checked directly against `GOVERNANCE.md`'s Crate Boundary Rules: `modiq-collection` still only produces Evidence, never evaluates it. `modiq-engine` still only orchestrates, never implements Collector or Rule logic. No boundary in `GOVERNANCE.md` needs amendment — this Sprint adds a second Collector invocation, not a new kind of ownership.

## Public API impact

None to `AssessmentService`'s existing entry points — `execute` and `execute_from_assessment_input` keep their current signatures. The change is entirely inside `execute_from_assessment_input`'s own body (invoking a second Collector), not its public contract. GOV-008 remains untouched.

## AssessmentService interaction

`execute_from_assessment_input` gains a second Collector invocation alongside its existing routing check, aggregating both Collectors' Evidence into the single `Vec<Evidence>` already passed to `execute`. No change to `execute` itself.

## Rule Engine interaction

Two paths, not decided here — the second Chief Architect Review Question:

1. **Collector-only this Sprint.** `EvidencePresenceRule` (category-agnostic, fires on any Evidence) already picks up `XmlInspection` Evidence with zero `modiq-rules` change, folded into the existing generic Finding. Mirrors the Sprint 4 → Sprint 5 precedent (`StructuralDuplication` category shipped a full Sprint before `StructuralDuplicationRule` existed to interpret it specifically).
2. **A minimal new Rule this Sprint**, filtering on `EvidenceCategory::XmlInspection` specifically, following `StructuralDuplicationRule`'s exact shape (one `if let` branch in `RuleEngine::evaluate`, one Finding/Recommendation pair). This is what actually makes the new Evidence *change* a Report's Findings, not just its Evidence list — more directly answering this Sprint's own "how will the Report improve" question, at the cost of breaking the Collector/Rule sprint-separation precedent.

## Knowledge model interaction

None. Architecturally prohibited for a Collector by `EvidenceCollection.md`'s own frozen boundary (quoted above). Not a design choice available to this Sprint.

## AssessmentReport impact

None to its shape. `evidence()`, `findings()`, `recommendations()` already carry whatever this Sprint produces — consistent with this project's own "Evidence-first, single-channel reporting" principle (`PROJECT_HANDOFF_v1.0.md`, Section 6, Principle 6): no second reporting channel gets introduced for this new kind of fact, exactly as was true when `StructuralDuplication` was introduced.

## Testing strategy

Real I/O, real fixtures, no mocks — the established, unbroken discipline. A real, checked-in or test-constructed `modDesc.xml` (well-formed, malformed, and absent cases), mirroring `EvidenceCollector`'s and `ArchiveCollector`'s own `TempDir`-based real-filesystem tests. Determinism verified directly per `EvidenceCollection.md`'s Collector Contract guarantee, not assumed. If the composition question (above) resolves toward running two Collectors per Assessment, a new determinism claim exists — the *combined* Evidence list's ordering across the two Collectors — and needs its own direct test, per Sprint 5 Phase 5's own lesson (a determinism claim not directly tested is not verified).

---

# Implementation Phases

Phased, mirroring `SPRINT4_IMPLEMENTATION_PLAN.md`'s and `SPRINT5_IMPLEMENTATION_PLAN.md`'s own shape — narrow enough that each phase can be separately authorized, not a single blanket approval.

## Phase 1 — Design Preparation and Governance

**Objective:** Composition architecture is already resolved (`COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`, Section 14) — this phase now covers only the Rule Engine question (flagged below) and selecting/authorizing an XML parsing dependency, plus opening a Governance Register item recording the approved Collector Composition Model, still outstanding per that proposal's own Chief Architect Review Questions 2–3.
**Files/crates likely affected:** None — proposal document(s) only, no code.
**Risks:** The external-dependency decision, if deferred past this phase, risks becoming a mid-Sprint blocker rather than a resolved precondition (every prior Collector settled its dependency question early; this Sprint should not be the first exception).
**Validation:** N/A — no code this phase.
**Expected repository outcome:** Remaining open questions resolved, an authorized dependency, no implementation yet.

## Phase 2 — `XmlCollector`: Discovery and Well-Formedness

**Objective:** A real `XmlCollector` that locates `modDesc.xml` within a given Assessment Input and reports whether it exists and is well-formed XML — the minimal, real, testable slice, mirroring how `ArchiveReader` (Sprint 4 Phase 3A) was built and tested before anything consumed it.
**Files/crates likely affected:** `modiq-collection` (new module), `modiq-collection/Cargo.toml` (new XML dependency).
**Risks:** Scope discipline — the temptation to parse further than "does this exist and parse as XML" in this phase specifically, before the content-extraction boundary (Phase 3) is even scoped.
**Validation:** `cargo fmt`/`check`/`test` on `modiq-collection` alone; real fixtures (well-formed, malformed, absent).
**Expected repository outcome:** A real, independently tested Collector, not yet reachable from `AssessmentService` — the same "real and tested before wired in" pattern every prior Collector has followed.

## Phase 3 — Content Extraction

**Objective:** Extend `XmlCollector` to produce `XmlInspection` Evidence for specific, named facts (missing/present required elements, declared dependency elements) — the actual content-level capability, scoped per Phase 1's resolved design.
**Files/crates likely affected:** `modiq-collection`.
**Risks:** This is where "discovering content exists" most risks drifting into "interpreting what it means" (Evidence Collection's own Non-Responsibility) — each fact extracted needs to be checked against that boundary individually, not assumed safe because it came from Phase 2's foundation.
**Validation:** `cargo fmt`/`check`/`test` on `modiq-collection`; real fixtures covering each extracted fact.
**Expected repository outcome:** `XmlCollector` producing real, factual `XmlInspection` Evidence, still not yet wired into `AssessmentService`.

## Phase 4 — `AssessmentService` Composition

**Objective:** Wire `XmlCollector` into `execute_from_assessment_input`, per the approved composition architecture (independent invocation alongside the structural Collector, composed inline, Evidence aggregated — no `CollectionCoordinator`).
**Files/crates likely affected:** `modiq-engine`.
**Risks:** The first-of-its-kind multi-Collector aggregation ordering needs its own explicit determinism test (Testing Strategy, above) — the single highest risk of this phase being reported "done" without actually being proven.
**Validation:** `cargo fmt`/`check`/`test` on `modiq-engine`, including a fresh determinism test for combined-Evidence ordering.
**Expected repository outcome:** The platform's first end-to-end multi-Collector Assessment path.

## Phase 5 — Rule Engine (conditional on Phase 1's resolution)

**Objective:** If Phase 1 resolved toward a dedicated Rule: implement it, following `StructuralDuplicationRule`'s exact shape.
**Files/crates likely affected:** `modiq-rules`.
**Risks:** None beyond the ordinary — this phase only exists if explicitly authorized as in-scope.
**Validation:** `cargo fmt`/`check`/`test` on `modiq-rules`.
**Expected repository outcome:** `XmlInspection` Evidence produces a distinguishable Finding, not just a generic one.

## Phase 6 — Closeout

**Objective:** Full-workspace validation, Sandbox revalidation, documentation sync, mirroring every prior Sprint's own closeout.
**Files/crates likely affected:** Documentation only.
**Risks:** The now four-for-four documentation-staleness pattern — worth a deliberate check this time, not a repeat.
**Validation:** `cargo fmt`/`check`/`test`, both workspaces, zero warnings.
**Expected repository outcome:** A Sprint 7 that closes administratively complete, not just implementation-complete — including, per the new convention below, its own "After this Sprint, modIQ can now..." statement.

---

# Validation Strategy

Unchanged from this project's standing discipline: `cargo fmt`, `cargo check --workspace`, `cargo test --workspace`, both the root workspace and `apps/sandbox/src-tauri` independently, zero warnings, every phase. Real I/O, real fixtures, no mocks. Every determinism claim — including the new multi-Collector aggregation ordering this Sprint introduces for the first time — gets its own direct test, not one inferred from a repeated-identical-input test.

---

# Risks

- **The composition question is a genuine architectural fork, not a detail** — building Phase 4 before it's explicitly resolved risks the exact "implementation-led architecture" failure mode `CHIEF_ARCHITECT_HANDOFF_v1.0.md` Section 8 warns against.
- **External XML-parsing dependency** — certain to be needed, not yet selected or authorized; this Sprint should not be the first to let a dependency decision slip past its own design phase.
- **Scope drift toward Knowledge Domain or dependency resolution** — flagged twice already in this project's own history (`SPRINT6_PLANNING_DOCUMENT.md`, `POST_SPRINT6_REPOSITORY_ASSESSMENT.md`) as the most likely failure mode for exactly this Collector. This plan's explicit out-of-scope list exists specifically to make that drift visible if it starts happening.
- **Multi-Collector determinism is genuinely new territory** — no prior Sprint has had to prove Evidence ordering *across* two independently-invoked Collectors, only within one.

---

# Chief Architect Review Questions

1. ~~**Composition:** does `XmlCollector` run independently, alongside the structural Collector, for every Assessment Input — or is there a preferred alternative?~~ — **RESOLVED.** See `COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`, Section 14, and the Chief Architect Decision Record below.
2. **Rule Engine scope (still open):** should Sprint 7 include a minimal new Rule for `XmlInspection` Evidence (Phase 5), or should Collector and Rule work be split across Sprints again, mirroring the Sprint 4 → 5 precedent? This directly determines whether `modiq-rules` is touched at all. Unaffected by the composition approval.
3. **External dependency (still open):** which XML parsing crate, if any preference exists, and is its addition authorized as part of Sprint 7's scope (per the standing "no new external dependency without explicit authorization" rule)?
4. **Absence-as-evidence (still open):** when `modDesc.xml` does not exist at all, should that be recorded as its own `XmlInspection` Evidence item, or treated as ordinary Empty Collection? This wasn't previously a live question, since no prior Collector inspects for one specific, expected file.

---

# Chief Architect Decision Record (Sprint 7)

**This is a Sprint-specific decision record, not a repository ADR** — mirrors `COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`'s own Section 14, scoped to what it means for this Sprint's implementation specifically.

**Approved architecture:** `AssessmentService` temporarily owns collection orchestration for Sprint 7 — determining which Collectors participate in an Assessment and aggregating their Evidence, implemented as direct inline composition. Collectors remain fully independent; no Collector consumes another Collector's output; no `CollectionCoordinator` component, service, or abstraction is introduced. XML inspection is the first implementation of Multi-Source Evidence Collection, not a bespoke mechanism of its own.

**Rationale:** "Capability before abstraction," applied to Collector composition for the first time, the same discipline already governing Rule dispatch (GOV-012) and Collector selection itself (GOV-004's own precedent). Two participating Collectors does not yet justify a dedicated orchestration component; a concrete, five-condition threshold (Decision Record, `COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md` Section 14) exists for when it would.

**Implementation implications for Sprint 7 specifically:** Phase 4 (`AssessmentService` Composition) is now unblocked in its design, though still gated on Phase 1's remaining open items (dependency selection, Rule Engine scope) and on explicit Chief Architect authorization to begin. No phase structure changed; only Phase 1 and Phase 4's own descriptions were narrowed to reflect what's already resolved.

---

# Capability Success Criteria

**After this Sprint, modIQ can now** read a mod's own declared self-description — for the first time, an Assessment can say something about what a mod *is*, not only that it *exists*.

Concretely, if Sprint 7 completes as scoped:

- **A user can** run an Assessment against a mod and see, in the Evidence list, real facts drawn from that mod's own manifest — not just confirmation that files were found.
- **The Assessment Report becomes more useful** by carrying its first genuinely content-derived Evidence category (`XmlInspection`, real for the first time since it was declared before Sprint 1) — and, if Phase 5 is authorized, its first Finding that isn't the generic "Evidence was collected" observation every Assessment has produced since Sprint 1.
- **New evidence inside the pipeline:** declared metadata and dependency facts from `modDesc.xml` — the direct, evidence-based mechanism behind the Product Specification's explicitly named "Dependency identification" objective, unreachable by any capability that exists today.

Per your request, this statement — or its Sprint 7 equivalent, updated to reflect what was actually built — should be the closing line of Sprint 7's own Closeout, not just this plan.

---

Awaiting Chief Architect review and authorization. No implementation, documentation change, commit, or branch has been made.

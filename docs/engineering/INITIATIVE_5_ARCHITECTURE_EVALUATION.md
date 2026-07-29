# Initiative 5 Architecture Evaluation — Production Interaction Layer Definition

| Property | Value |
|---|---|
| **Document** | INITIATIVE_5_ARCHITECTURE_EVALUATION.md |
| **Project** | modIQ |
| **Initiative Addressed** | Engineering Alignment Program, Initiative 5 — Production Interaction Layer Definition (`docs/engineering/ENGINEERING_ALIGNMENT_PROGRAM.md` §5–6) |
| **Purpose** | Determine the architectural boundary between the deterministic engine and the future production interaction layer: what constitutes it, the engine/consumer responsibility split, ownership of workspace/navigation/interaction/presentation/assessment state, the shape of the boundary-crossing mechanism, and the relationship to `apps/sandbox` — not implementation technology. |
| **Origin** | Chief Architect authorization — the first Architecture Evaluation opened under the Engineering Alignment Program, per its own recommended evaluation order (§6: this initiative first). |
| **Status** | **Architecture Evaluation only. No Architectural Resolution has been performed. No ADR, crate, API, payload, event, or implementation technology has been created, designed, or chosen.** |

---

## 1. Evaluation Scope

Per `ENGINEERING_ALIGNMENT_PROGRAM.md` §5 (Initiative 5), this evaluation addresses architectural boundary, ownership, and mechanism *shape* — not technology, not concrete payload design. This document does exactly that, and only that.

**Deliberately excluded**, per explicit instruction and consistent with `STORAGE_ARCHITECTURE_EVALUATION.md`'s own precedent for how a new architectural boundary is evaluated before any implementation question is touched:

- No implementation technology — React, Tauri, Electron, web, desktop, or any other framework is neither named nor implied as a choice.
- No API, method signature, event, or payload shape.
- No UI mockup or visual design of any kind.
- No modification to any Product Design or Interaction Design artifact — both are treated as frozen constraints throughout.
- No modification to any existing crate, Crate Boundary Rule, or dependency edge.
- No Sprint Planning, no implementation plan.
- No resolution of GOV-008 (AssessmentService Public API Evolution) — that remains Initiative 1's own territory; this evaluation identifies where the two initiatives meet without resolving either on the other's behalf.

---

## 2. Repository Evidence

Reviewed directly this session, not assumed from summary or memory:

- **`docs/architecture/Architecture.md`** — read in full: System Overview (diagram and Core Platform Components), Dependency Rules, Platform Boundaries, Extensibility.
- **`docs/engineering/GOVERNANCE.md`** — read in full: Crate Boundary Rules (all nine entries, most directly CLI and Storage), the Governance Register (GOV-008 specifically).
- **ADRs**, read in full: `0002-domain-model-boundaries.md` (internally titled "ADR-0001," a known, disclosed filename/numbering mismatch per `docs/adrs/README.md`'s own Engineering Notes — cited by its filename number here for consistency with every other cross-reference in the repository), ADR-0004 (Platform-First Version Profiles), ADR-0005 (Deterministic Assessment Engine), ADR-0007 (Runtime Entity Design Pattern), ADR-0008 (Evidence Collection Subsystem Boundary), ADR-0009 (AssessmentService Public API Evolution), ADR-0010 (Engine Orchestration Simplification).
- **`docs/implementation/RuntimeInvariants.md`** — read in full, all fourteen invariants.
- **`crates/modiq-runtime/src/assessment/assessment.rs`** — re-confirmed directly: no external mutation path exists for `Assessment`-owned state anywhere in the workspace.
- **Root `Cargo.toml`** — confirmed the nine-crate workspace membership list; `apps/sandbox` is not among them.
- **`apps/sandbox/README.md`** and **`apps/sandbox/src-tauri/Cargo.toml`** — confirmed the Sandbox's own explicit, first-party declaration that it is "not production software" and "intentionally its own Cargo workspace root... not a member of the root modIQ workspace."
- **`apps/sandbox/src-tauri/src/lib.rs`** — read in full: both Tauri commands (`create_assessment`, `retrieve_report`), their DTO types, and their own source comments citing `STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md` as precedent for building a representation from already-public getters.
- **`crates/modiq-cli/src/app.rs`** and **`commands/assess.rs`** — read in full: dispatch, entry point, and output-formatting behavior.
- **The complete Product Design and Interaction Design corpus** — already read in full during the Engineering Reconciliation this session; re-consulted here specifically for Navigation & Workspace Behavior, Assessment Intake & Upload, and Workspace Evolution's own state model.
- **`docs/engineering/ENGINEERING_ALIGNMENT_PROGRAM.md`** — this initiative's own governing scope definition (§5).
- **Classification precedent** — `STORAGE_ARCHITECTURE_EVALUATION.md` §9, `GOVERNANCE_OBSERVATION_SUBSYSTEM_ACTIVATION.md`, `SPRINT8_ARCHITECTURAL_RESOLUTION.md` §8, and `SPRINT12.md`'s Capability Identity procedure — consulted to test whether this initiative fits either existing classification.

---

## 3. Current Architecture

**`Architecture.md`'s own System Overview diagram places "User" outside the subsystem row entirely:**

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

Six boxes sit beneath Assessment Service — the subsystems it composes. "User" sits above it, on the calling side. This is the diagram's own structure, not an interpretation of it.

**The Dependency Rules section's own directional-flow diagram terminates at Storage.** Immediately after it, as a standalone line — never drawn into the flow diagram itself — `Architecture.md` states: "Presentation systems should consume assessment outputs without influencing assessment logic." The very next section, Platform Boundaries, names **"User interfaces"** as one of six things "the architecture intentionally separates" — at the same structural level as Assessment execution, Reporting, and Persistence.

**No Crate Boundary Rule category exists for presentation, interaction, or UI.** `GOVERNANCE.md`'s nine Crate Boundary Rules entries are: Runtime Domain, Rule Engine, Evidence Collection, Reporting, Knowledge Domain, Engine, CLI, Storage. The closest existing analog is **CLI**, whose own rule already reads: "Owns: user interaction, command execution, platform entry point. Must never contain business logic." — a real, governed answer to a narrower version of this initiative's own question, already living in the repository.

**Two consumers exist today, with asymmetric governance status:**

- **`modiq-cli`** — a root workspace member (confirmed against `Cargo.toml`), governed by its own Crate Boundary Rule, calling `AssessmentService` in-process, synchronously, dispatching by one direct match on the first argument, and formatting `AssessmentReport`'s already-public getters into a plain string. No business logic; no reimplementation of anything the engine already does.
- **`apps/sandbox`** — explicitly *not* a root workspace member (its own `src-tauri/Cargo.toml` states this directly), explicitly self-declared non-production, with no Crate Boundary Rule of any kind. It communicates via two Tauri IPC commands, each a single synchronous call that runs the entire pipeline and returns one complete DTO. Its DTOs (`EvidenceEntry`, `FindingEntry`, `RecommendationEntry`, `AssessmentSummary`) are built from `AssessmentReport`'s already-public getters — never from `modiq-runtime` types directly — and its own source comments cite `modiq-storage`'s representation-boundary precedent as the model for doing so.

**Assessment state ownership is already fixed, not open to this evaluation.** INV-006 ("Assessment SHALL be the sole owner of runtime assessment state") and INV-009 ("No external component SHALL directly mutate Assessment-owned collections") are both currently enforced with zero exceptions — confirmed directly against `assessment.rs`: no public mutation path exists outside `Assessment`'s own methods, for any caller.

**No engine-side concept of workspace, navigation, interaction, or presentation state exists anywhere in the codebase.** These four terms appear only in the Product Design and Interaction Design corpus. `AssessmentStatus` (four execution phases) is the only lifecycle-adjacent state the engine holds, and the Engineering Reconciliation already identified it as distinct from — not a synonym for — Workspace Evolution's own three-state model (Initiative 1's own territory).

---

## 4. Decision 1 — Classification of the Production Interaction Layer

**Question:** does a production interaction layer represent an existing System Overview subsystem, a new subsystem, or another architectural classification?

**Alternatives considered:**

- **Folding it into an existing subsystem** (most plausibly Reporting, whose own charter says "responsibility is presentation"). **Rejected.** Reporting's presentation is an engine-internal transformation — Assessment state into `AssessmentReport` — performed *inside* the pipeline, composed *by* `AssessmentService` (ADR-0010, GOV-004: direct composition of each subsystem's own real type). A production interaction layer stands on the opposite side of that same relationship: it calls the engine, the engine never calls it. These are not the same shape of thing wearing different names — they are different directions of dependency.
- **A new subsystem, via the Architectural Activation precedent** (Sprint 8: Version Profiles; Sprint 9: Knowledge Base; Sprint 13: Storage). **Rejected.** Every prior instance of Architectural Activation gave real content to a box `Architecture.md`'s own System Overview diagram had already drawn as engine-internal and dormant. "User" is drawn in that same diagram, but outside the subsystem row, above Assessment Service — on the calling side of the boundary Platform Boundaries separately names. There is no dormant "Presentation" or "Interaction Layer" box in the diagram to activate.
- **A consumer/client, structurally analogous to `modiq-cli`.** **Recommended.** Same directional relationship CLI already has (consumer calls the engine's public boundary; the engine never calls it back). Same governing constraint ("must never contain business logic"; "presentation systems... without influencing assessment logic"). Different only in the *richness* of responsibility CLI's own trivial text output never had to resolve — workspace, navigation, interaction, and presentation state (Decision 4), none of which a formatted string requires.

**Recommendation:** the production interaction layer is a **consumer**, not a subsystem — neither an existing one nor a new one in the System-Overview/Architectural-Activation sense.

---

## 5. Decision 2 — Responsibility Split

Stated in `GOVERNANCE.md`'s own "Owns / Must never" convention, extending the CLI entry's shape rather than replacing it:

**Engine (unchanged by this evaluation):** Evidence Collection, Rule Evaluation, Finding/Recommendation/Report generation, lifecycle enforcement (INV-001–014), determinism (ADR-0005). Every one of these remains exactly where it already is.

**Consumer owns:**
- Presenting whatever the engine's public boundary already returns.
- Translating user action into calls against that boundary.
- Everything named in Decision 4 (workspace, navigation, interaction, and presentation state) — none of which the engine can own without violating its own existing boundaries (Decision 3, Decision 4).

**Consumer must never** (mirroring CLI's own rule and `Architecture.md`'s existing Dependency Rules language exactly):
- Contain business logic.
- Evaluate Evidence, or generate Findings, Recommendations, or Reports.
- Mutate Assessment state directly, or become an alternate source of truth for it.

---

## 6. Decision 3 — Ownership of Assessment State

**Not a new decision — a confirmed, pre-existing constraint.** INV-006 and INV-009 already settle this: `Assessment` is the sole owner of runtime assessment state, and no external component may mutate it directly. This holds regardless of anything else this evaluation determines. Any future consumer, however architected, can only ever hold a read-only representation of Assessment state — an `AssessmentReport`, or whatever shape Initiative 1 eventually exposes — never the state itself. This evaluation confirms this boundary; it does not have discretion over it.

---

## 7. Decision 4 — Ownership of Workspace, Navigation, Interaction, and Presentation State

**Navigation, interaction, and presentation state: consumer-owned, by elimination.** Three converging facts, none newly asserted here: (1) Interaction Design itself repeatedly and explicitly declines to assign this ownership rather than assuming it — Finding Presentation §9 leaves whether a Finding's expansion state persists across a visit as "a state-persistence question this document is not positioned to resolve," and Navigation & Workspace Behavior's own scope explicitly excludes "interface components." (2) No Runtime type has any field for it, and adding one would violate ADR-0007's own boundary — Runtime entities "hold data, not behavior that reaches outside the aggregate." (3) `Architecture.md`'s own existing Dependency Rules language already assigns presentation to the consumer side, unconditionally. Nothing about this determination is new; it is what those three already-settled facts jointly imply.

**Workspace state is a genuinely different case — consumer-owned, but derived, not invented.** Workspace Evolution's three states are Intake, Assessing, and Reviewing. Intake has **no corresponding engine state at all** — the engine's own lifecycle machine (`AssessmentStatus`) does not begin until `Assessment::new` is called, and there is no engine-side representation of "nothing has been submitted yet." Assessing and Reviewing, by contrast, are consumer-side interpretations of engine-owned execution phase — but *which* engine signal they correspond to, and how it is exposed, is exactly Initiative 1's own open question, not this evaluation's to answer. **Recommendation:** workspace state is owned by the consumer, but it is not an independent fact the consumer invents — for Assessing/Reviewing specifically, it is derived from whatever execution-phase signal Initiative 1's own resolution exposes; for Intake specifically, it has no engine analog and is purely consumer-side.

---

## 8. Decision 5 — Mechanism Shape (How the Boundary Is Crossed)

**Baseline recommendation: request/response.** Every existing precedent — `modiq-cli`'s in-process synchronous call, `apps/sandbox`'s cross-process synchronous Tauri IPC call — is request/response, with zero counter-examples anywhere in the platform's history. This is also the only shape consistent with `Architecture.md`'s own one-directional Dependency Rules diagram, which never depicts a reverse (engine-to-consumer) edge.

**One sub-question is explicitly not resolved here, and cannot be without presupposing Initiative 1's own outcome.** `ENGINEERING_ALIGNMENT_PROGRAM.md` §5 already records this exact dependency, in the other direction: "Initiative 5 establishes the architectural boundary and consumption mechanism... that [Initiative 1's] own observability mechanism must be designed to conform to." This evaluation fixes the *baseline* — request/response is the shape the boundary takes today, and remains sufficient if Initiative 1 concludes the engine never exposes intermediate state at all. What is not decided here is whether a **supplementary** signal (a notification, a poll-trigger, or some other mechanism this evaluation does not name) is additionally required once — and only once — Initiative 1 determines the engine *does* expose intermediate state. Resolving that now would mean guessing Initiative 1's own answer before it has one.

**This is a deliberately partial Decision**, consistent with this evaluation's own instruction not to force a resolution where the evidence does not yet support one.

---

## 9. Decision 6 — Relationship to `apps/sandbox`

**`apps/sandbox`, as it exists today, cannot be the production interaction layer.** Three disqualifying facts, each already established, none newly asserted: it is not a root workspace member (`src-tauri/Cargo.toml`'s own explicit declaration); it is self-declared non-production (`README.md`); and it deliberately has no real Assessment Subject input mechanism — file selection, drag-and-drop, and any other real upload workflow are explicitly out of scope by its own cited precedent (`PROPOSAL_FILESYSTEM_COLLECTION.md`, Sandbox Interaction). Becoming the production interaction layer would require joining the root workspace, gaining a real input mechanism, and taking on the full responsibility set named in Decisions 2 and 4 — an architectural transformation of what the Sandbox is, not a continuation of its current, deliberately minimal charter.

**Its *pattern*, not its identity, is validated precedent worth naming.** Synchronous request/response over an IPC boundary (converging directly with Decision 5's own baseline) and building consumer-facing DTOs from already-public engine getters rather than serializing Runtime types directly — a pattern the Sandbox's own source comments explicitly model on `modiq-storage`'s representation-boundary precedent. This is now a second, informal instance of that same shape (Storage's `PersistedAssessmentReport` being the first), worth the Chief Architect weighing as convergent evidence — without this evaluation treating it as a technology endorsement, since concrete representation and payload design remain out of scope here.

---

## Governance Observation — Absence of a Boundary-Formalization Classification

| Property | Value |
|---|---|
| **Arising from** | Decision 1 (Classification of the Production Interaction Layer), above |
| **Nature** | An **Observation**, per this project's own Decision Framework (Observation → Evidence → Investigation → Governance) — it names and evidences a governance-vocabulary gap; it does not investigate options, recommend a procedure, or decide anything, mirroring `GOVERNANCE_OBSERVATION_SUBSYSTEM_ACTIVATION.md`'s own precedent for exactly this kind of finding. |
| **Status** | **Observation only. No Governance Register item opened. No ADR created. No Architectural Resolution performed.** |

### Observation

Decision 1 found that a production interaction layer fits neither of this repository's two existing architectural classification procedures — Sprint 12's Capability Identity procedure (Collector/Rule-shaped work only) and the Sprint 8/13 Architectural Activation precedent (dormant subsystems `Architecture.md`'s own System Overview diagram already draws as engine-internal). Generalized beyond this initiative specifically: **the repository's existing governance vocabulary currently has no classification procedure for formalizing an external architectural boundary** — work that specifies the client side of a relationship `Architecture.md` already names (Platform Boundaries: "User interfaces") but has never before required a governance procedure, because nothing has previously required formalizing it. This is not a defect in either existing procedure — both continue to correctly classify the work they were derived to classify. It is an absence: a third shape of architectural question this repository has now encountered for the first time, for which no procedure yet exists at all.

### Evidence

Restated from Decision 1 and Section 3 above, not re-derived: `SPRINT12_ARCHITECTURAL_RESOLUTION.md`'s three-axis Capability Identity procedure was validated exclusively against Collector/Rule-shaped historical decisions, and `STORAGE_ARCHITECTURE_EVALUATION.md` §9 already found even a subsystem-level candidate (Storage) falls outside it. Architectural Activation (`SPRINT8_ARCHITECTURAL_RESOLUTION.md` §8) activates a box `Architecture.md`'s own System Overview diagram already draws as engine-internal and dormant — Version Profiles, Knowledge Base, and Storage each occupied such a box before their own activation. "User" is drawn in the same diagram, but outside that row, on the calling side of the same boundary Platform Boundaries names — never a dormant internal box waiting to be activated. No third procedure exists anywhere in `GOVERNANCE.md`, any Governance Register item, or any ADR for classifying work that formalizes an external boundary rather than activating an internal one.

### Not Resolved Here

This document does not recommend a name, a procedure, or a disposition for this gap — only that it exists, and that it is not specific to Initiative 5. Any future initiative that formalizes another external boundary this platform has named but never specified would face the same, already-present absence. Whether this warrants its own named procedure (mirroring how Architectural Activation and Capability Identity were each named once, prospectively, inside the evaluation that first needed one), extension of an existing procedure, or another disposition entirely is left for the Chief Architect's own judgment.

### Explicit Non-Scope

- No recommendation for how this gap should be closed, named, or governed is made or implied.
- No Governance Register item is opened by this observation.
- No ADR is created.
- No Architectural Resolution is performed — Decisions 1 through 6 above remain independently ready for resolution regardless of how, or whether, this observation is later acted on.

---

## 10. Explicit Non-Goals

- No API, event, or payload design of any kind.
- No implementation technology or framework selection.
- No UI mockup or visual design.
- No modification to any Product Design or Interaction Design artifact.
- No modification to any existing crate, Crate Boundary Rule, or dependency edge.
- No resolution of GOV-008 — remains Initiative 1's own territory.
- No resolution of Decision 5's supplementary-signal sub-question.
- No Sprint scope, no implementation plan, no ADR.

---

## 11. Chief Architect Recommendation

**Decisions 1, 2, 3, 4, and 6 are ready for Architectural Resolution** — each is grounded in evidence this evaluation found convergent and, in Decision 3's case, already architecturally settled. **Decision 5 is only partially ready**: its baseline (request/response) is resolvable now; its supplementary-signal sub-question is not, and should remain explicitly open, tracked against Initiative 1's own eventual resolution rather than guessed at here. **The Governance Observation above requires explicit Chief Architect judgment** on how, or whether, to formalize a classification procedure for external-boundary work — independent of Decisions 1 through 6, none of which depend on that judgment to proceed to their own Architectural Resolution.

**No Architectural Resolution has been performed. No ADR, crate, API, payload, or technology has been created, chosen, or implied as a choice.** This evaluation's responsibility ends here, awaiting explicit Chief Architect authorization before the next stage of the governance workflow begins.

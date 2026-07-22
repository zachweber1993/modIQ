# Sprint 10 Capability Definition

| Property | Value |
|---|---|
| **Document** | SPRINT10_CAPABILITY_DEFINITION.md |
| **Project** | modIQ |
| **Purpose** | Capability Definition for Sprint 10 — Runtime Log Interpretation (minimum viable evidence source) — for Chief Architect review |
| **Prepared by** | Lead Engineer (Sonnet 5), on `feature/runtime-implementation` |
| **Repository baseline** | `feature/runtime-implementation`, HEAD `b7cb4a6` |
| **Status** | **Capability Definition only. Implementation has NOT been authorized.** No code changed, no documentation changed, no commits, no branch, no Governance Register item, no ADR. Awaiting Chief Architect review before Sprint 10 Architectural Resolution can begin. |

---

# 0. Repository Verification

Verified directly against source this session, not carried forward from any prior session's own account.

| Check | Result |
|---|---|
| Working tree | `git status` — clean, except the untracked `PROJECT_MILESTONE_REVIEW_SPRINT9.md` produced by the prior Milestone Review session |
| Current branch | `feature/runtime-implementation`, in sync with `origin` (0 ahead / 0 behind) |
| Latest commit | `b7cb4a6` — "docs: complete Sprint 9 repository closeout" |
| All tests pass | `cargo test --workspace` — **210/210**, matching `ENGINEERING_RELEASE_0.9.md`'s own recorded figure exactly; no drift since Sprint 9 closeout |
| Frozen planning baseline re-read | `SPRINT9_RUNTIME_LOG_ARCHITECTURE_EVALUATION.md` and `SPRINT_ROADMAP_UPDATE_v1.md` re-read in full this session, not recalled from earlier conversation |
| `EvidenceCategory::RuntimeLogs` | Confirmed still real and still unused: defined in `crates/modiq-runtime/src/assessment/evidence_category.rs`, produced by zero Collectors, exactly as every prior planning document states |
| Real log sample availability | Checked directly this session: no runtime log fixture, sample, or format description exists anywhere in this repository (`find`/`grep` across `docs/`, `crates/`, and `apps/`, excluding `node_modules`/`target` build artifacts, returns nothing). This is a material finding for scope, addressed in Section 11. |

No repository change has occurred since Sprint 9 Closeout that the frozen planning documents did not already anticipate. Sprint planning, roadmap ordering, and the resolved Runtime Log workflow decision (below) are treated as closed inputs and not reopened.

---

# 1. Executive Summary

Sprint 10 activates Runtime Log Interpretation — one of six Assessment Engine responsibilities `ProductSpecification.md` names explicitly, and the platform's first Evidence source describing an *attempt* to run a mod rather than a *fact about the mod's own package*. The architectural workflow question this capability originally raised (bundled submission versus a second Assessment Input versus a standalone Assessment) was already evaluated and resolved during Sprint 9's own Architecture Evaluation and Chief Architect decision, recorded in `SPRINT_ROADMAP_UPDATE_v1.md`: a bundled submission, read by a new source of Evidence against the same Assessment Input every existing source already inspects, with no second input and no cross-Assessment correlation. That resolution is treated here as a settled precondition, not reopened.

This document defines the capability itself, deliberately smaller than "interpret runtime logs" in the abstract: recognizing exactly one well-defined class of runtime log signal — that the game's own log output indicates a mod failed to load — as objective, traceable Evidence, and producing a Finding stating so. This is the smallest complete capability this platform's own history would recognize as "runtime log interpretation" at all, mirroring every prior Sprint's own smallest-real-slice discipline (one hardcoded Version Profile at Sprint 8, one named Repair Recipe at Sprint 9).

A real constraint surfaced directly this session, not previously named in the frozen planning baseline: **no real Farming Simulator runtime log sample exists anywhere in this repository.** This bears directly on how confidently the "one class of signal" can be scoped before Architectural Resolution, and is named explicitly in Section 11 for Chief Architect attention.

---

# 2. Problem Statement

`Vision.md`'s own "The Problem" section names *"Why does it fail to load?"* directly among the small set of fundamental questions this entire platform exists to help users answer — stated in the same breath as compatibility, dependencies, and performance. The same section states plainly how that question is answered *today*: *"through trial-and-error, community discussions, runtime log interpretation, or personal experience... inconsistent, time-consuming, and frequently unsupported by objective evidence."*

This is worth stating precisely, because it is a genuinely unusual case among this platform's capabilities: **"runtime log interpretation" is not a new idea this platform introduces — it is the exact informal, ad hoc practice `Vision.md` names as part of the problem.** Players and creators already read runtime logs today, by hand, inconsistently, without shared method. The capability is not to invent log-reading; it is to replace an unreliable, manual, tribal-knowledge practice with a deterministic, evidence-based one — the same transformation this platform has already made for compatibility (Sprint 8) and, more narrowly, for a mod's declared structure (Sprints 3, 4, 7).

`ProductSpecification.md`'s Assessment Engine section lists six named responsibilities: XML validation, Lua analysis, dependency inspection, asset verification, version compatibility, and runtime log interpretation. Of these, XML validation, dependency inspection (declaration only, not resolution), and version compatibility are real today; Lua analysis and asset verification remain unstarted. Runtime log interpretation would be the fourth of six to gain any real implementation — not the platform's last gap, but a concrete, specification-named one that has waited since Documentation Release 1.0.

---

# 3. Capability Definition

**What Runtime Log Interpretation means for this platform:** recognizing objective, verifiable facts within the textual output Farming Simulator itself produces when it attempts to load or run a mod, and recording those facts as Evidence — never opinion, never inferred root cause beyond what the log's own content states. This mirrors `DataModel.md`'s own standing definition of Evidence exactly ("objective information collected during an Assessment... Evidence never represents opinion") and the Collector Contract's existing observational boundary (`EvidenceCollection.md`) — this capability introduces a new *source* of Evidence, not a new *kind* of claim the platform is permitted to make.

**Distinguishing existing evidence sources from Runtime Log evidence.** Every Evidence source this platform has today — filesystem structure, archive structure, `modDesc.xml` inspection — describes facts about the mod's own package: what files it contains, whether its manifest is well-formed, what version and dependencies it declares about itself. These are all facts a mod carries *at rest*, true whether or not the mod is ever actually loaded. Runtime Log evidence is a different kind of fact entirely: it describes what happened during an actual attempt to load or run the mod — forensic material about an event, not a description of an artifact. No Evidence source this platform has ever produced has described an *outcome* rather than a *structure*; this is the first.

**How Runtime Log evidence complements, rather than replaces, the existing model.** The existing package-level Evidence sources answer "what is this mod, and is it well-formed, and does it correctly declare its own compatibility?" Runtime Log evidence answers a question none of them can: "what actually happened when this mod was used?" Both kinds of Evidence flow into the same, entirely unchanged Rule Engine, which already consumes a single flat `&[Evidence]` slice regardless of which source produced any given item — a Rule is already free, today, with no new mechanism, to reason about a runtime-log-observed failure alongside a package-level fact from the same Assessment (a declared dependency, a declared version) if a future Rule chooses to. This capability does not require building that correlation; it activates a source of Evidence the existing architecture was already structurally prepared to receive.

---

# 4. User Value

- **Players** — directly. `Vision.md`'s own named question, "why does it fail to load?", currently answered only through trial-and-error and informal log-reading, gains its first real, evidence-based answer: a mod bundled with a log showing it failed to load produces a Finding stating so, traceable to the log content itself, without the player needing to read or interpret raw log text.
- **Mod Creators** — directly, in the same way version compatibility already serves them (Sprint 8): objective, repeatable feedback during their own testing, replacing "does this log line mean something is wrong" guesswork with a stated Finding.
- **Server Administrators** — secondarily, consistent with `ProductSpecification.md`'s named objective to "reduce runtime instability": a load-failure signal is exactly the kind of fact that matters most before a mod is deployed to a shared server.

This directly realizes `Vision.md`'s own belief that "software should educate its users rather than simply produce results" — for a question the specification itself singles out by name, not a generic capability chosen for convenience.

---

# 5. Engineering Value

- **Activates `EvidenceCategory::RuntimeLogs`** — real and defined since before Sprint 1, produced by zero Collectors across nine Sprints. Of the five `EvidenceCategory` variants originally defined but never produced (`LuaAnalysis`, `RuntimeLogs`, `AssetValidation`, `DependencyResolution`, `PerformanceObservations`), this Sprint activates the second (after none activated at Sprints 8 or 9, both of which worked in adjacent domains — Version Profiles and Knowledge — rather than Evidence Collection itself).
- **Tests whether the platform's existing Evidence/Rule model generalizes to a genuinely new kind of fact** — an *event* rather than a *structure* — without requiring the Assessment model, the Rule Engine's dispatch shape, or `AssessmentService`'s public entry points to change at all. This is a meaningful generality test in the same spirit as Sprint 8's confirmation that ADR-0007's Opaque Reference pattern generalizes beyond the Knowledge Domain: this Sprint asks whether the Evidence/Finding model generalizes beyond package-structure facts, using the already-resolved bundled-submission shape rather than reopening that question.
- **Closes the sole remaining open question from Sprint 9's own Runtime Log Architecture Evaluation** that was itself capability-shaped, not architecture-shaped — the specific minimum-viable log signal to target — completing the sequence that evaluation explicitly deferred to this stage.

---

# 6. Existing Platform Relationship

Established as resolved, pre-existing input to this Sprint, not decided here:

- **Workflow:** a bundled submission — the log is reached the same way `XmlCollector` already reaches `modDesc.xml`, against the same Assessment Input every existing Collector inspects. No second Assessment Input, no standalone Runtime Log Assessment, no cross-Assessment correlation mechanism (`SPRINT_ROADMAP_UPDATE_v1.md`, Section 2).
- **`Assessment`, `AssessmentSubject`, `AssessmentContext`:** unaffected. The Assessment Subject remains the mod; the log is supporting material within the same submission, not a second subject.
- **`RuleEngine::evaluate`:** unaffected in shape — it already consumes a flat, provenance-blind `Evidence` slice and requires no new parameter to interpret a new category.
- **`AssessmentService`'s two public entry points:** unaffected — no new entry point, no signature change, consistent with every capability shipped since Sprint 6.
- **Knowledge Domain (`modiq-knowledge`):** unaffected by this capability directly. Pairing a future runtime-log Finding with a real Repair Recipe (Sprint 9's own new mechanism) is a natural extension, named explicitly as deferred (Section 9), not built this Sprint.

This capability activates a dormant Evidence category using an already-resolved integration shape; it does not reopen, redesign, or extend any of the above.

---

# 7. In Scope

- Real, objective recognition of **one narrow, well-defined class of runtime log signal**: that a bundled log's content indicates the mod failed to load during an attempted session. Not a general log-reading capability — one class of observable outcome, mirroring the "one hardcoded Version Profile" (Sprint 8) and "one named Repair Recipe" (Sprint 9) discipline exactly.
- Producing real `RuntimeLogs`-category Evidence from that recognized signal, for the first time since the category was defined.
- Producing a Finding when that signal is present, traceable to the Evidence that supports it, following the same Evidence-Based and Explainable execution principles every existing Rule already satisfies.
- Real, checked-in or realistically-constructed test fixtures exercising this recognition, consistent with this project's own Real-I/O Testing Discipline (`PROJECT_HANDOFF_v1.0.md`, Section 5) — no mocked log content standing in for a real log's actual shape.

---

# 8. Out of Scope

- **Any second Assessment Input, cross-Assessment correlation, or standalone Runtime Log Assessment** — all three closed by the already-resolved workflow decision (Section 6); not reopened by this document.
- **A general-purpose log parsing framework, grammar, or multi-format support.** One recognized signal, not an extensible parsing engine.
- **Root-cause diagnosis beyond what the log's own content literally states.** No inference about *why* a failure occurred beyond what is directly observable — the same discipline `VersionCompatibilityRule` already applies (an unrecognized declared version is a `Warning`, not a claim the mod is broken).
- **Any Repair Recipe content for a runtime-log Finding.** A natural pairing with Sprint 9's own new mechanism, but not this Sprint's own charter — named as a future candidate (Section 9), not built.
- **Any change to the Knowledge Domain, Version Profile integration, or Rule Selection filtering.**
- **Any new or changed public entry point on `AssessmentService`, and any change to `RuleEngine::evaluate`'s signature.**
- **Any UI, CLI, or Sandbox presentation change.** Existing consumers already render Findings and Recommendations generically; a new Finding requires no new rendering path, as confirmed at Sprint 8 and Sprint 9 both.

---

# 9. Deferred Capabilities

Named explicitly so they are not later rediscovered as surprises, per this project's own standing practice:

- **Broader log signature coverage** (additional known failure classes, warnings, or performance-adjacent signals) — a future Sprint's own scoping question, once this first slice is proven real.
- **Cross-Assessment correlation** (a mod assessed now, its log supplied and correlated later) — the architecturally heavier Option B this platform's own Sprint 9 evaluation declined to build; remains a real, named future product question, not a current plan.
- **Runtime Log as its own standalone Assessment Subject** (Option C) — evaluated and available in principle, not selected; remains a distinct future possibility, not this Sprint's shape.
- **Pairing a runtime-log Finding with a real Repair Recipe** — a natural, low-risk extension of Sprint 9's own new mechanism, deliberately not bundled into this Sprint to keep each capability's own footprint independently provable.
- **Whether `AssessmentSubject` should ever gain real, non-unit-struct content** — a materially separate question this capability does not require answering, named at Sprint 9's own Architecture Evaluation (Chief Architect Question 3) and still open.

---

# 10. Success Criteria

Per this project's own standing Capability Success Criteria convention (`PROJECT_HANDOFF_v1.0.md`, Section 5, established at Sprint 7):

**After this Sprint, modIQ can now** recognize, as real and traceable Evidence, that a bundled runtime log indicates a mod failed to load during an actual attempt to use it — and produce a Finding stating so.

Concretely, if Sprint 10 completes as scoped: a Player or Creator who submits a mod together with its own runtime log, where that log shows the mod failed to load, receives a Finding describing that occurrence, traceable to the log content itself — the platform's first real, evidence-based answer to `Vision.md`'s own named question, "why does it fail to load?", replacing the trial-and-error, log-reading-by-hand process that question named as today's only recourse.

---

# 11. Chief Engineer Recommendation

**Recommend proceeding to Architectural Resolution, with one material precondition named explicitly rather than assumed away.**

This capability is well-suited to proceed: its workflow question is already resolved (Sprint 9's own Architecture Evaluation and Chief Architect decision), its architectural impact is expected to be low and precedented (a new Evidence-producing source against the existing single Assessment Input, mirroring `XmlCollector`'s own Sprint 7 introduction, plus a new Rule mirroring `VersionCompatibilityRule`'s and `StructuralDuplicationRule`'s own shape), and its scope here is deliberately the smallest slice this platform's own history would recognize as real.

**The one precondition — an engineering requirement, not merely an observation:** this session confirmed directly that **no real Farming Simulator runtime log sample, fixture, or format description exists anywhere in this repository.** This platform's own Evidence-Based Engineering discipline — the same discipline that requires every Finding to trace to real Evidence, never opinion — applies with equal force to how this platform's *own architecture* gets decided, not only to what it concludes about a mod. On that basis: **Architectural Resolution must not assume the structure, wording, stability, or formatting of Farming Simulator runtime logs.** No candidate log-signal shape, no parsing approach, and no claim about what is "typically" present in a Farming Simulator log may be adopted as a working assumption. Every prior content-inspecting capability this platform has built was grounded in exactly this discipline before any design decision was finalized: `XmlCollector`'s manifest parsing proceeded only once `modDesc.xml`'s real shape was in hand; `ArchiveReader`'s ZIP handling proceeded only after Sprint 4 Phase 2's own dedicated Boundary-Proving investigation produced real, empirical evidence against the `zip` crate. Runtime Log Interpretation has neither a real sample nor an equivalent investigation yet, and Architectural Resolution must not proceed as though it does.

**Requirement:** every architectural decision made for this capability — what constitutes a recognizable signal, how a Collector should locate or read a log, what failure modes are possible — must be derived from real runtime log evidence or authoritative, cited Farming Simulator documentation, never from a plausible-sounding assumption about what such a log probably contains. **If representative runtime logs are not available, acquiring and validating them is the first engineering activity Architectural Resolution must undertake — a prerequisite investigation, in the same spirit as Sprint 4 Phase 2's own Boundary-Proving, that must produce real evidence before any implementation-oriented architectural decision is finalized, not proceed in parallel with or after one.** This is not a narrowing of Sprint 10's scope (Section 7–9 are unchanged) — it is a statement of the evidentiary bar Architectural Resolution's own decisions must clear before they may be treated as resolved at all.

---

Awaiting Chief Architect review. No implementation, documentation change, governance item, or ADR has been made this session.

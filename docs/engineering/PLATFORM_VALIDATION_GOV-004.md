# Platform Validation: GOV-004 — Engine Service Granularity

| Property | Value |
|----------|-------|
| **Document** | PLATFORM_VALIDATION_GOV-004.md |
| **Cycle** | Platform Validation (opened by `PROPOSAL_PLATFORM_VALIDATION_REVIEW.md`) |
| **Stage** | Technical evaluation only — no recommendation, no resolution |
| **Prepared by** | Engineering, for Technical Director review |
| **Governance item under evaluation** | GOV-004 — Engine Service Granularity |
| **Status** | Design-only. No Rust code, no governance changes, no implementation proposal. |
| **Repository state reviewed** | `2e3e108` (working tree clean at time of writing) |

---

# 1. Original Architectural Intent

Two Frozen specifications bear directly on this question, and they do not describe the same shape.

**`EngineAPI.md` (Frozen, Documentation Release 1.0; last touched 2026-07-17, Documentation Release 2.0 — untouched since, including through Documentation Release 2.1)** defines "five conceptual services": Assessment Service, Knowledge Service, Rule Evaluation Service, Reporting Service, Version Profile Service — each with its own Responsibility, Capabilities, and Depends On sections. Its Service Relationships diagram shows `Assessment Service → Rule Evaluation Service → Knowledge Service`, `Assessment Service → Reporting Service`, and `Version Profile Service → Reporting Service`, i.e., four distinct downstream services `AssessmentService` is depicted as coordinating.

`EngineAPI.md` also states its own governing principle directly: under "Capability-Oriented," it says the API "exposes capabilities rather than implementation technologies... Implementations remain free to expose those services through REST, SDKs, plugins, CLIs, or other interfaces." It does not itself claim that each conceptual service must correspond to one discrete Rust type.

**`Architecture.md` (Frozen, Documentation Release 1.0, amended under 2.1 — the higher-authority document; `EngineAPI.md` lists `Architecture.md` in its own Specification Authority and is explicit that "if a conflict exists between this document and a higher-level specification, the higher-level specification takes precedence")** describes a different shape in its System Overview: "modIQ is organized as a collection of cooperating platform services centered around the Assessment Service... delegating specialized responsibilities to supporting subsystems," with a diagram showing Assessment Service delegating directly to Evidence Collection, Rule Engine, Version Profiles, Knowledge Base, Reporting, and Storage. These are named as **subsystems** (i.e., crates: `modiq-collection`, `modiq-rules`, `modiq-knowledge`, `modiq-versioning`, `modiq-report`), not as intra-engine service objects distinct from those crates' own real types.

**GOV-004 itself** was raised at Engineering Release v0.1.0-alpha — the platform's very first release — with the question recorded plainly: "Should future orchestration continue through `AssessmentService` \[directly\], or transition to specialized Engine services?" Its Resolution has read "Pending" since that first release. No ADR (0001–0009) addresses this question directly; three ADRs (0003, 0004, 0005) list `EngineAPI.md` only as a cross-reference, without engaging its service-granularity model.

---

# 2. Current Implementation Evidence

Confirmed directly against source at `2e3e108`:

- **`AssessmentService::execute`** (`crates/modiq-engine/src/engine/assessment_service.rs`) constructs `modiq_rules::rules::RuleEngine` directly and calls `modiq_report::report::AssessmentReport::generate` directly. **`execute_from_assessment_input`** constructs `modiq_collection::collection::EvidenceCollector` directly. In both methods, the subsystem's own real, public type is instantiated inline — no intermediate service object appears anywhere in either method.
- **The four `EngineAPI.md`-named services exist as source in `modiq-engine`** — `KnowledgeService`, `ReportingService`, `RuleEvaluationService`, `VersionProfileService` — each a single unit struct with a one-line doc comment, e.g. `pub struct KnowledgeService;` with `/// Provides reusable engineering knowledge.` All four were introduced in the same commit, `81c3fd7` ("Complete Sprint 0 engineering foundation," 2026-07-16), and **no subsequent commit has ever modified any of the four files.** All four are re-exported at `modiq-engine`'s crate root (`pub use ... ;` in `engine/mod.rs`). A workspace-wide search finds no construction, no method call, and no test referencing any of the four outside of that re-export line.
- **The identical pattern exists inside `modiq-rules`** — `RuleSelector`, `EvidenceEvaluator`, `Explainability`, `Traceability` — mirroring `RuleEngine.md`'s six conceptual responsibilities. Same origin commit (`81c3fd7`), same zero modifications since, same zero construction sites anywhere outside their own re-export. `RuleEngine::evaluate` (the one real, tested Rule in the platform) performs selection (trivially — it always applies the same rule), evidence evaluation, Finding generation, Recommendation generation, and traceability (`evidence_ids`, `rule_reference`) entirely inline, without calling any of the four.
- **Test coverage:** zero tests, anywhere in the workspace, reference any of the eight stub types by name.
- **Three real subsystems now exist that could have used the `modiq-engine` services** — Rule Engine (Sprint 1), Reporting (Sprint 1), Evidence Collection (Sprint 3) — and all three were wired into `AssessmentService` by direct instantiation of their own crate's real type, independently, across different Sprints, by the same discipline each time.

---

# 3. Every Observed Deviation

- **Deviation A.** `AssessmentService` composes `RuleEngine`, `AssessmentReport`, and `EvidenceCollector` via direct instantiation of each subsystem's own real type, not via the four `EngineAPI.md`-named intra-engine service objects that exist in source for exactly this purpose.
- **Deviation B.** The four `EngineAPI.md` stub services have never been constructed or called by any code, in any commit, since the moment they were created at Sprint 0.
- **Deviation C.** `modiq-rules` independently exhibits the same pattern: `RuleEngine::evaluate` absorbs the responsibilities `RuleEngine.md` conceptually assigns to Rule Selection, Evidence Evaluation, Traceability, and Explainability, rather than delegating to the four dedicated stub types built for that purpose.
- **Deviation D.** `EngineAPI.md`'s Service Relationships diagram (`Assessment Service → Rule Evaluation Service → Knowledge Service`, `→ Reporting Service`, `← Version Profile Service`) has no implemented counterpart of any kind — none of those edges exist anywhere in running code, at any point in the platform's history.

---

# 4. Classification of Each Deviation

**Deviation A — contested between Architectural Drift and Intentional Evolution; not resolvable from evidence alone.** The case for *intentional evolution*: `Architecture.md`, the higher-authority document, already describes subsystem delegation (not intra-engine service objects) in its own System Overview, and `EngineAPI.md` itself explicitly disclaims mandating a one-service-per-Rust-type mapping. The case for *architectural drift*: `EngineAPI.md` is still Frozen, still names four services with specific Responsibilities and Capabilities, and the stub scaffolding's existence suggests someone at Sprint 0 read it as calling for real objects — three subsequent Sprints of real implementation never revisited that reading, they simply routed around it each time. Which specification was meant to govern `modiq-engine`'s actual internal shape has never been explicitly decided; this deviation's classification depends entirely on that undecided question.

**Deviation B — Implementation Simplification, well-supported.** This reads cleanly as scaffolding built at Sprint 0, ahead of any real need, and correctly left unfilled since nothing has needed it — consistent with this platform's own repeatedly-validated "a capability should justify an abstraction, not the reverse" discipline (twice validated elsewhere: the Rule trait question, the Collector trait question). Calling this "drift" would require evidence that the platform actively moved away from a pattern it was once following; instead, the pattern was never activated in the first place.

**Deviation C — Implementation Simplification, more strongly supported than B.** Same shape as B, but with an important difference: `RuleEngine::evaluate` is real, tested, and demonstrably fulfills every conceptual responsibility `RuleEngine.md` assigns (selection, evaluation, Finding generation, Recommendation generation, traceability) — just not through four separate objects. The conceptual contract is honored; only the internal decomposition differs from what the stub types imply.

**Deviation D — Insufficient Evidence.** `EngineAPI.md` explicitly states it excludes "programming language constructs" from its scope and describes its services as capability-oriented rather than implementation-oriented. Whether the Service Relationships diagram was ever intended to describe an internal Rust call graph, or only a conceptual capability-dependency map with no implementation-fidelity expectation at all, cannot be determined from the specification's own text or from implementation evidence. This is the same underlying question as Deviation A's classification difficulty, viewed at the diagram level.

---

# 5. Consequences of Leaving the Current Architecture Unchanged

- The scaffold/reality divergence persists: a future contributor reading `EngineAPI.md` and finding four matching structs already in source could reasonably conclude they are load-bearing and begin wiring real logic into them — a wrong inference three real subsystems have now independently avoided by accident of individual judgment, not by any documented answer.
- Eight structurally identical, inert files (four in `modiq-engine`, four in `modiq-rules`) continue to compile and ship in every build, unmodified since the platform's first commit, with no record of why they remain.
- The next real collector and any future second Rule will each face the same implicit, undocumented choice a fourth and fifth time, with nothing to point to except "that's what the last three did."
- No cost to correctness, test coverage, or velocity: the stubs are zero-sized types with no runtime cost, and their presence has not blocked, slowed, or complicated any work to date, including Sprint 3's introduction of an entirely new subsystem.
- The `Architecture.md` / `EngineAPI.md` tension identified in Deviation A remains latent and unreconciled, and could resurface identically — unexamined — at the next Documentation Release.

---

# 6. Consequences of Changing It

**If `EngineAPI.md` were amended to formally endorse direct subsystem composition** (which would make the four `modiq-engine` stub services, and by the same logic the four `modiq-rules` stub submodules, deletion candidates):

- Removes the scaffold/reality divergence and the dead code carrying it.
- Amending `EngineAPI.md` — Frozen since Documentation Release 1.0, unmodified since 2.0 — would be the first substantive change to that specification in the platform's history, requiring the same explicit, non-silent amendment discipline already used for `Architecture.md` and `EvidenceCollection.md` under Documentation Release 2.1, and Technical Director approval.
- Forecloses, at least for now, a design option that has never actually been tested as unnecessary: if a future scenario genuinely needs a seam between `modiq-engine` and its subsystems — swapping a Rule Engine implementation, exposing the platform over a remote boundary, a plugin surface — that seam would need to be reintroduced later rather than already existing.

**If the four `modiq-engine` stub services (and the mirrored `modiq-rules` stubs) were instead formally endorsed and real work done to route existing orchestration through them:**

- Requires non-trivial Rust changes to `modiq-engine` and `modiq-rules` — code that has been stable, and heavily tested, since Sprint 1 and Sprint 2 respectively — to introduce indirection that does not currently exist.
- No concrete forcing function currently exists for it: no consumer has ever needed to swap an implementation, no remote boundary exists, no plugin surface exists, no second Knowledge or Version Profile consumer exists.
- Would run directly counter to this platform's own twice-validated "capability before abstraction" principle unless a genuine second concrete need is identified first — the same premature-abstraction pattern this project has explicitly declined twice already, applied here for a third time.

**Either direction:** this would be the first change of any kind to `EngineAPI.md` since Documentation Release 2.0, and the first time this platform has reconciled a Frozen conceptual specification against three Sprints of accumulated implementation evidence rather than against a single implementation attempt. How that reconciliation is handled may itself set a pattern for how future specification-vs-evidence gaps (of which Section 4 of `PROPOSAL_PLATFORM_VALIDATION_REVIEW.md` names several) get resolved.

---

# 7. Additional Evidence That Would Still Be Needed Before a Technical Director Decision

- **Whether `Architecture.md`'s subsystem-delegation model or `EngineAPI.md`'s five-service model was intended to govern `modiq-engine`'s internal shape has never been directly asked or answered.** No ADR addresses it. This is a documentation-history gap that may be answerable by the Technical Director's own recollection or review, rather than by further implementation.
- **No concrete second case yet exists that would exercise an intra-engine service boundary** — a real Knowledge Domain consumer, an alternate Rule Evaluation strategy, a Version Profile that actually varies rule applicability, or a second Reporting output shape. Consistent with this platform's own pattern of resolving abstraction questions concurrently with a real case rather than in the abstract (validated twice, per `PROPOSAL_PLATFORM_VALIDATION_REVIEW.md`, Section 3), the absence of such a case is itself evidence, though this review takes no position on what it implies.
- **`modiq-cli`'s eventual wiring** — already named elsewhere as a near-term, low-risk candidate independent of Collection work — would be the platform's second independent consumer of `modiq-engine`'s public surface, after the Sandbox. Whether it would exercise the stub services any differently than the Sandbox has is currently unknown, because it has not happened yet.
- **No cost estimate exists for either direction.** This review was scoped to evaluate evidence, not to propose implementation, and accordingly did not attempt to estimate the effort of amending `EngineAPI.md`, deleting the stubs, or building real service indirection. That estimate does not yet exist anywhere in the repository.

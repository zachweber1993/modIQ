# Proposal: GOV-004 Resolution — Formalize Direct Subsystem Composition, Retire the Internal Service Model

| Property | Value |
|----------|-------|
| **Document** | PROPOSAL_GOV-004.md |
| **Stage** | Proposal (Architecture Review), formalizing a decision already reached by the Technical Director |
| **Prepared by** | Engineering, for Technical Director review |
| **Governance item addressed** | GOV-004 — Engine Service Granularity |
| **Evidentiary basis** | `PLATFORM_VALIDATION_GOV-004.md` (Technical Director review complete; evidence considered sufficient) |
| **Status** | Proposal only. No governance, documentation, or code change has been made. Awaiting review before any of Sections 4–6, below, are authorized. |
| **Repository state reviewed** | `2e3e108` |

---

# 1. Decision Being Formalized

The Technical Director has reviewed `PLATFORM_VALIDATION_GOV-004.md` and reached a decision: **`AssessmentService`'s direct subsystem composition is the platform's actual orchestration architecture, and the internal service model `EngineAPI.md` describes — four discrete intra-engine service objects (`KnowledgeService`, `ReportingService`, `RuleEvaluationService`, `VersionProfileService`) mediating between `AssessmentService` and each subsystem — is no longer justified as an implementation target.**

This proposal translates that decision into the specific documentation changes, implementation work, and migration considerations it implies, so that scope and sequencing can be reviewed before anything is authorized. It does not itself amend `EngineAPI.md`, does not update `GOVERNANCE.md`, and does not touch any Rust source.

---

# 2. Summary of Implementation Evidence

Condensed from `PLATFORM_VALIDATION_GOV-004.md`; full detail and sourcing there.

- `AssessmentService::execute` and `execute_from_assessment_input` construct `RuleEngine` (`modiq-rules`), `AssessmentReport::generate` (`modiq-report`), and `EvidenceCollector` (`modiq-collection`) directly, inline. No intermediate service object appears in either method.
- The four `EngineAPI.md`-named services in `modiq-engine` — each a one-line unit struct — were introduced in the platform's first foundational commit (`81c3fd7`, Sprint 0) and have not been modified since. No construction site, method call, or test exists for any of the four anywhere in the workspace.
- `modiq-rules` independently exhibits the identical pattern: four stub submodules from the same Sprint 0 commit, never used; the one real, tested Rule performs selection, evaluation, Finding/Recommendation generation, and traceability entirely inline instead.
- Three separate real subsystems — Rule Engine (Sprint 1), Reporting (Sprint 1), Evidence Collection (Sprint 3) — were each independently wired into `AssessmentService` by direct instantiation, across different Sprints, with no prior coordination between those decisions beyond consistent engineering judgment.
- GOV-004 was raised at the platform's first Engineering Release and has remained open since, with no ADR ever directly engaging the question.

---

# 3. Why `AssessmentService` Remains the Orchestration Boundary

`Architecture.md` — the higher-authority specification, which `EngineAPI.md` itself names as taking precedence in the event of conflict — already describes this shape in its own System Overview: "modIQ is organized as a collection of cooperating platform services centered around the Assessment Service... delegating specialized responsibilities to supporting subsystems," with a diagram showing `Assessment Service` delegating directly to Evidence Collection, Rule Engine, Knowledge Base, Version Profiles, Reporting, and Storage as subsystems — not to a further layer of intra-engine service objects distinct from those subsystems' own types. What `AssessmentService` does today is a literal implementation of that diagram: it delegates to each subsystem's real, public type, directly.

This is not merely permitted by `EngineAPI.md`; it is anticipated by it. `EngineAPI.md`'s own "Capability-Oriented" principle states the API "exposes capabilities rather than implementation technologies... Implementations remain free to expose those services through REST, SDKs, plugins, CLIs, or other interfaces." Direct instantiation of each subsystem's own type is one such interface — arguably the simplest one available to a single-process, monolithic-workspace implementation, which is what the platform is today.

Three independent subsystems, introduced across two Sprints by the same engineering discipline, converged on the identical pattern without being told to. That convergence — not a single implementation choice, but three of them, made at different times, under different immediate pressures — is the strongest evidence available that direct composition is not an oversight but the shape this architecture actually produces when implemented honestly against its own higher-authority specification.

---

# 4. Why the Internal Service Model Is No Longer Justified

No concrete forcing function for the four-service model has arrived in three Engineering Releases. Nothing in the platform has ever needed to swap a Rule Engine implementation, expose `modiq-engine` over a remote or process boundary, support a plugin surface, or mediate access to Knowledge or Version Profile data through a dedicated service object — every scenario an intra-engine service layer would exist to support remains hypothetical.

Building or maintaining that layer without such a case would directly contradict this platform's own repeatedly validated engineering discipline: "a capability should justify an abstraction, not the reverse." That principle has already been tested twice under real pressure to build the abstraction anyway — a Rule trait, a Collector trait — and declined both times, with no loss of correctness in either case. Continuing to carry four unused service stubs in `modiq-engine`, and four more of the same shape in `modiq-rules`, is the same premature abstraction this platform has twice declined elsewhere, left standing only because it predates the discipline that would now reject it.

The cost of carrying it is not runtime cost — the stub types are zero-sized and free to compile — but risk: a future contributor reading `EngineAPI.md` and finding matching structs already in source has a real, reasonable path to the wrong conclusion that they are load-bearing, and would be wiring against a pattern three real subsystems have already independently rejected.

---

# 5. Documentation Changes Required

Identified for review; none performed as part of this proposal.

- **`EngineAPI.md`** — requires a Documentation-Release-weight amendment, the first substantive change to this specification since Documentation Release 2.0. The five-service model should be reconciled with `Architecture.md`'s subsystem-delegation shape: `Assessment Service` retained as the platform's one real orchestration service; `Knowledge Service`, `Rule Evaluation Service`, `Reporting Service`, and `Version Profile Service` reframed as descriptions of subsystem-level capabilities (already owned by `KnowledgeModel.md`, `RuleEngine.md`, the Reporting System, and `VersionProfile.md` respectively) rather than as discrete intra-engine objects `modiq-engine` is expected to instantiate. This should follow the same explicit, non-silent amendment discipline already used for `Architecture.md` and `EvidenceCollection.md` under Documentation Release 2.1 — not a silent rewrite.
- **A new ADR** — this decision changes a durable architectural pattern in a Frozen specification, the same category ADR-0007 assigned to the Runtime Entity Design Pattern and ADR-0008/0009 assigned to Evidence Collection's boundary. A dedicated ADR recording "orchestration is by direct subsystem composition; no intra-engine service indirection layer is maintained" would give this decision the same durable, citable record those precedents have, rather than leaving it as governance-register prose alone.
- **`GOVERNANCE.md`** — GOV-004's Resolution field updated to record the decision, its rationale, and the Documentation Release / ADR that closes it, following the same pattern already used for GOV-005/006/007/009/010's Resolution text.
- **`Architecture.md`** — likely requires no content change, since its System Overview already describes the shape being formalized. Worth an explicit cross-reference note pointing to the new ADR, so a future reader has a direct path from "the diagram that was right all along" to "the decision that confirmed it," rather than needing to infer the connection.
- **`CrateRoadmap.md`** — `modiq-engine` and `modiq-rules` maturity entries should note that direct subsystem composition is the platform's formalized pattern, and that the four-plus-four stub types are pending removal (see Section 6).
- **`ENGINEERING_LOG.md`** — an entry recording the deletion, once it happens, following this project's Status/Affected Crates/Affected Documents/Notes structure.

---

# 6. Implementation Work That Would Follow Approval

Identified for planning only; not started, not scoped in detail, no code written.

- Delete `crates/modiq-engine/src/engine/knowledge_service.rs`, `reporting_service.rs`, `rule_evaluation_service.rs`, `version_profile_service.rs`, and their `pub mod` / `pub use` lines in `crates/modiq-engine/src/engine/mod.rs`.
- **Scope includes `modiq-rules`'s mirrored stub submodules** (`RuleSelector`, `EvidenceEvaluator`, `Explainability`, `Traceability`), not only the four services in `modiq-engine` that GOV-004's title names directly. Because the mirrored service scaffolding in `modiq-rules` exists solely to support the same architectural model, its retirement is considered part of the same architectural simplification rather than a separate governance decision. This is recorded explicitly here so a future reviewer does not need to wonder why a second crate changed under a governance item titled "Engine Service Granularity."
- `AssessmentService`'s existing doc comment already states it "owns orchestration only... without owning runtime state, rule logic, or reporting logic itself" — accurate today, and would likely only need a note affirming this is now the platform's formalized, not provisional, pattern.
- No behavioral change is expected anywhere: since none of the eight stub types has ever been constructed or called, removing them changes zero runtime behavior. The only observable effect is a smaller public API surface — eight fewer exported types across the two crates' roots.
- No test rewrites are expected: `PLATFORM_VALIDATION_GOV-004.md` confirmed zero tests reference any of the eight types. A `cargo check --workspace` pass (and the Sandbox's separate `cargo check`) would confirm this rather than assume it.
- `docs/engineering/GOVERNANCE.md` at GOV-004's entry would need Resolution text written (a documentation change, listed above, not code).

---

# 7. Migration Considerations

- **Public API removal.** Both crates currently re-export all eight stub types at their crate roots. Removing them is a breaking change to each crate's public surface. Since neither crate is published outside this workspace, the practical blast radius is limited to in-tree consumers — confirmed, by direct search, to be none — but this cannot be verified against any hypothetical out-of-tree consumer from inside this repository.
- **Sequencing between documentation and code.** `EngineAPI.md`'s amendment should land before or atomically with the code deletion. Deleting the stub types while `EngineAPI.md` still describes them as the platform's service model would leave the specification and the implementation newly, explicitly contradicting each other — a worse state than today's implicit gap, and inconsistent with this project's discipline of never letting a Frozen specification and real behavior silently diverge.
- **Sandbox verification.** `apps/sandbox/src-tauri` is a separate Cargo workspace depending on `modiq-engine` and `modiq-rules` by path. It does not currently reference any of the eight stub types (confirmed by the same workspace-wide search), but its own `cargo check` and `Cargo.lock` should be independently re-verified after deletion, as a mechanical step rather than a design concern — the same discipline already applied when `thiserror` was added to `modiq-engine` in the Sprint 3 Phase 5 commit.
- **No data or runtime migration applies.** This change is compile-time only. The Storage Layer remains unbuilt, so no persisted Assessment, historical record, or serialized state is affected by removing these types.
- **No sequencing dependency on other roadmap work.** This resolution does not block, and is not blocked by, the next real collector or CLI wiring — it touches only orchestration-internal composition inside `modiq-engine` and `modiq-rules`, not any public behavior either of those efforts would depend on. It can be scheduled independently, in either order.

---

# 8. What This Proposal Does Not Do

No file has been modified as part of preparing this proposal, other than its own creation. `EngineAPI.md`, `GOVERNANCE.md`, `CrateRoadmap.md`, and every Rust source file remain exactly as they were at `2e3e108`. Sections 5 and 6 describe work to be scoped and authorized separately, in whatever order and grouping the Technical Director directs — this document does not sequence its own execution beyond what Section 7 notes about documentation-before-code ordering.

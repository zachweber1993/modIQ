# Sprint 8 Implementation Deviation Record

| Property | Value |
|---|---|
| **Document** | SPRINT8_IMPLEMENTATION_DEVIATIONS.md |
| **Project** | modIQ |
| **Purpose** | Permanent record of every meaningful difference between `SPRINT8_IMPLEMENTATION_AUTHORIZATION.md`'s planned approach and Sprint 8's actual implementation |
| **Prepared by** | Lead Engineer (Sonnet 5), on `feature/runtime-implementation` |
| **Status** | All deviations below were reviewed and explicitly accepted by the Chief Architect as part of Sprint 8's Repository Closeout authorization, prior to this document's own creation. |

---

## Purpose and Method

`SPRINT8_IMPLEMENTATION_AUTHORIZATION.md` planned Sprint 8's implementation in seven phases with specific expected mechanisms at each. This document compares that plan against what `git diff` and the current source actually show, precisely — not from memory, and not restated from the Implementation Report's own summary alone. Each entry states the planned approach, the implemented approach, the repository evidence distinguishing them, the engineering rationale for the change, and why the change improved or preserved the architecture rather than degrading it. Where a "deviation" turns out, on inspection, to already have been the planned document's own recommendation, that is stated plainly rather than overclaimed as a new decision.

---

## Deviation 1 — `AssessmentService`'s public entry points remain completely unchanged, rather than gaining a new additive entry point

**Planned approach** (`SPRINT8_IMPLEMENTATION_AUTHORIZATION.md`, Phase 4 and Section 8): *"A new, additive entry point is introduced instead, mirroring exactly how `execute_from_assessment_input` was itself added alongside `execute` without replacing it — preserving both existing entry points as stable contracts and requiring no Public API Policy governance approval this Sprint."* Phase 6 correspondingly planned that `modiq-cli` and `apps/sandbox` would each gain a `modiq-versioning` dependency and updated call sites to supply an explicit `VersionProfile` to that new entry point.

**Implemented approach:** No new entry point exists. `execute` and `execute_from_assessment_input` retain their exact pre-Sprint-8 signatures. `execute` now constructs `VersionProfile::fs25()` internally, at the top of its own body, and uses it for both the `VersionProfileReference` passed into `Assessment::new` and the real `VersionProfile` passed into `RuleEngine::evaluate`. `execute_from_assessment_input` is unaffected — it already delegated to `execute` and required no change at all.

**Repository evidence:** `crates/modiq-engine/src/engine/assessment_service.rs` — `AssessmentService`'s `impl` block contains exactly two public methods, both with pre-existing signatures unchanged. `git diff` for this file shows only additions inside `execute`'s own body (`let version_profile = VersionProfile::fs25();` and the two lines using it), no signature change. `crates/modiq-cli/src/commands/assess.rs` and `apps/sandbox/src-tauri/src/lib.rs` both appear with **zero diff** in `git status` — neither file was touched, confirmed directly, not merely asserted.

**Engineering rationale:** At the point implementation reached Phase 4, no second `VersionProfile` value existed anywhere in the codebase or in any authorized plan for one to exist. A parameterized entry point whose only caller-visible difference from the existing ones would have been accepting a value every actual caller would supply identically (`VersionProfile::fs25()`, the only value that exists) is unforced API surface — a parameter with no real choice behind it. This is the same "capability before abstraction" test this project has applied at least eight times before (`PROJECT_HANDOFF_v1.0.md`, Section 6): does a second concrete case actually exist, right now, that needs this? It did not.

**Why this preserved rather than degraded the architecture:** `AssessmentService`'s Public API Policy protection (`GOVERNANCE.md`) is strongest when no entry point is added without a demonstrated need — this is a stricter, not looser, application of "APIs evolve additively" than the planned new entry point would have been: zero surface added is the most additive outcome available. It also means `modiq-cli` and `apps/sandbox` needed no change at all, the smallest possible blast radius for a capability that touches the Runtime aggregate's own constructor. Should a real second Version Profile arrive, the additive-entry-point option remains fully available then, with genuine evidence behind it, exactly mirroring how `execute_from_assessment_input` itself was only introduced once Evidence Collection created a real need for it — this deviation defers the same kind of decision to the same kind of evidence-driven moment, rather than avoiding it forever.

**A direct consequence, stated precisely:** `SPRINT8_ARCHITECTURAL_RESOLUTION.md`'s own Decision 4 evaluation had recommended the Version Profile value remain *caller-supplied*, reasoning that "deciding which Version Profile governs an Assessment" is a business judgment `AssessmentService` should not make internally (`GOVERNANCE.md`'s Engine boundary: "Must never own runtime state"). Because no new parameter exists through which a caller could supply one, `VersionProfile::fs25()` is in fact now constructed internally by `AssessmentService`, not supplied by any caller — a direct, necessary consequence of this deviation, not an independent choice. This is recorded here as an honest tension with that earlier recommendation, not smoothed over: with only one Version Profile in existence, "selecting" it is a degenerate case with no actual judgment involved (there is nothing to choose between), which is why the Chief Architect accepted this consequence as part of the same implementation simplification rather than treating it as a separate architectural violation. Should a second profile arrive, this specific question — caller-supplied versus internally selected — becomes a real judgment again and should be revisited on its own evidence at that time, not assumed settled by this Sprint's degenerate case.

---

## Deviation 2 — `Assessment` owns an opaque `VersionProfileReference`, not a direct `modiq-versioning` dependency

**Planned approach, precisely stated:** This was **not** a mid-implementation deviation from `SPRINT8_IMPLEMENTATION_AUTHORIZATION.md`'s own text — that document's Section 2 (Decision 1 validation) already recommended exactly this shape: *"`Assessment` holds a new, `modiq-runtime`-local opaque `VersionProfileReference` type, structurally identical to `RuleReference`/`RepairRecipeReference`... Recommendation for implementation: (b)."* Included here, per explicit request, because it represents a genuine refinement relative to the **earlier** `SPRINT8_ARCHITECTURAL_RESOLUTION.md` session, whose own Decision 1 text ("Version Profile shall be a first-class entity owned by Assessment") was compatible with either a direct field or an opaque reference, and because the Chief Architect's explicit confirmation of this specific shape (received at the start of the Implementation session: *"Assessment owns a VersionProfileReference rather than a concrete VersionProfile... Reuse the ADR-0007 opaque reference pattern"*) is what actually authorized implementation to proceed on this point.

**Implemented approach:** Exactly as recommended and confirmed. `VersionProfileReference(String)` (`crates/modiq-runtime/src/assessment/version_profile_reference.rs`) mirrors `RuleReference`/`RepairRecipeReference` precisely — same shape, same crate, same lack of any dependency on the domain it references.

**Repository evidence:** `crates/modiq-runtime/Cargo.toml`'s `[dependencies]` section, diffed against its pre-Sprint-8 state, is **unchanged** — still only `thiserror`. `modiq-runtime` remains the platform's sole workspace crate with zero dependencies on any other workspace crate, confirmed directly, not assumed from the plan.

**Engineering rationale:** Stated in full in `SPRINT8_IMPLEMENTATION_AUTHORIZATION.md`, Section 2, and not repeated here beyond the summary above.

**Why this preserved rather than degraded the architecture:** Preserved `modiq-runtime`'s zero-workspace-dependency leaf status, unbroken since Sprint 0, and extended `ADR-0007`'s Opaque Runtime References pattern to a new category of cross-domain relationship (Version, not Knowledge) for the first time — confirming the pattern generalizes rather than being specific to the Knowledge Domain it was originally written for.

---

## Deviation 3 — Implementation phase order: `modiq-rules` before `modiq-engine`

**Planned approach** (`SPRINT8_IMPLEMENTATION_AUTHORIZATION.md`, Section 3): Phase 4 (`modiq-engine` wiring) listed before Phase 5 (`modiq-rules`, the new Rule and `RuleEngine::evaluate`'s new parameter).

**Implemented approach:** `modiq-rules` (new Rule, `RuleEngine::evaluate`'s new `version_profile` parameter) was implemented and independently validated (`cargo test -p modiq-rules`) before `modiq-engine`'s own wiring.

**Repository evidence:** None beyond the implementation session's own working order — both crates' final states match the authorized architecture regardless of sequencing; this deviation has no effect on any file's final content.

**Engineering rationale:** `AssessmentService::execute`'s own call to `RuleEngine::evaluate` needed that method's final, real signature to compile against. Implementing and validating `modiq-rules`' side of that boundary first, independently, meant `modiq-engine`'s own change was a single, already-known integration rather than a change made against a signature still in flux.

**Why this preserved rather than degraded the architecture:** No architectural consequence — this is a sequencing choice within an already-authorized set of phases, not a change to what any phase produces. Noted here for completeness, per this project's "document every meaningful difference" instruction, and because it is itself a small, direct illustration of the phased-validation practice named in this Sprint's own Engineering Methodology Observation (`ENGINEERING_LOG.md`).

---

## Deviations Considered and Not Found

For completeness: dependency edges (`modiq-engine`/`modiq-rules` → `modiq-versioning`, `modiq-versioning` → nothing), the Rule's severity assignment (`Warning`, never `Error`), the testing discipline (real fixtures, no mocks), and the six-decision Architectural Resolution itself all matched the authorized plan exactly, with no deviation to record.

---

This record is permanent engineering history. It does not itself authorize, retract, or modify any architectural decision — see `SPRINT8_ARCHITECTURAL_RESOLUTION.md` and `SPRINT8_IMPLEMENTATION_AUTHORIZATION.md` for those.

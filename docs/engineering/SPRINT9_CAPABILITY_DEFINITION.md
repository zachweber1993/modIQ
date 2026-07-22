# Sprint 9 Capability Definition

| Property | Value |
|---|---|
| **Document** | SPRINT9_CAPABILITY_DEFINITION.md |
| **Project** | modIQ |
| **Purpose** | Capability Definition for Sprint 9 — Repair Guidance (Minimum Viable `modiq-knowledge` Activation) — for Chief Architect review |
| **Prepared by** | Lead Engineer (Sonnet 5), on `feature/runtime-implementation` |
| **Repository baseline** | `feature/runtime-implementation`, HEAD `aaa08fe` |
| **Status** | **Capability Definition only. Implementation has NOT been authorized.** No code changed, no documentation changed, no commits, no branch, no Governance Register item, no ADR. Awaiting Chief Architect review before Sprint 9 Architectural Resolution can begin. |

---

# 0. Repository Verification

Verified directly against source this session, not carried forward from any prior session's own account.

| Check | Result |
|---|---|
| Working tree | `git status` — clean |
| Current branch | `feature/runtime-implementation`, in sync with `origin` (0 ahead / 0 behind) |
| Latest commit | `aaa08fe` — "docs(planning): finalize Sprint 9 roadmap and planning cycle" |
| All tests pass | `cargo test --workspace` — **205/205**, matching `REPOSITORY_CLOSEOUT_REPORT.md`'s own recorded figure exactly; no drift since Sprint 8 closeout |
| Documentation consistency | `PROJECT_STATUS.md`'s own `Last Updated` field reads 2026-07-22 (today) and its Sprint 8 section, Current Focus paragraph, and Governance Status note are all internally consistent with `REPOSITORY_CLOSEOUT_REPORT.md`, `POST_SPRINT8_CAPABILITY_PRIORITIZATION_STUDY.md`, `SPRINT9_RUNTIME_LOG_ARCHITECTURE_EVALUATION.md`, and `SPRINT_ROADMAP_UPDATE_v1.md` — no contradiction found across any of the five documents |
| `modiq-knowledge` current state | Confirmed by direct source inspection: all seven domain files (`best_practice.rs`, `compatibility_pattern.rs`, `engine_behavior.rs`, `knowledge_reference.rs`, `known_issue.rs`, `repair_recipe.rs`, `rule.rs`) are content-free unit structs, each with a one-line doc comment only, e.g. `pub struct RepairRecipe;`. `Cargo.toml`'s `[dependencies]` is empty. Zero implementation since Sprint 0, exactly as every frozen planning document states. |
| `RepairRecipeReference` current state | Confirmed: `crates/modiq-runtime/src/assessment/repair_recipe_reference.rs` is a real, tested, opaque string wrapper (`RepairRecipeReference(String)`), wired through `Recommendation::new`'s third parameter since Sprint 2. Both real Rules that construct a `Recommendation` (`StructuralDuplicationRule`, `VersionCompatibilityRule`) pass `None` for it today — confirmed directly in both source files. |
| `modiq-rules` → `modiq-knowledge` dependency | Confirmed absent: `crates/modiq-rules/Cargo.toml` depends only on `modiq-runtime` and `modiq-versioning`. No edge to `modiq-knowledge` exists anywhere in the workspace graph today. |
| Frozen planning baseline | `POST_SPRINT8_CAPABILITY_PRIORITIZATION_STUDY.md`, `SPRINT9_RUNTIME_LOG_ARCHITECTURE_EVALUATION.md`, and `SPRINT_ROADMAP_UPDATE_v1.md` re-read in full this session. No inconsistency found between their own recorded repository evidence and the current tree. No repository change has occurred since Sprint 8 Closeout that these documents did not already anticipate. |

**No blocker was found.** Capability Prioritization (Repair Guidance, Sprint 9) and Roadmap Commitment (`SPRINT_ROADMAP_UPDATE_v1.md`, Section 4) are treated as closed inputs, not re-litigated in this document, per the mission's own instruction.

**Sprint 9 is ready to enter Capability Definition.**

---

# 1. Executive Summary

Sprint 9 activates `modiq-knowledge`'s first real content since Sprint 0: a minimum-viable `RepairRecipe`, consumed through the already-existing, already-tested `RepairRecipeReference` seam, by at least one existing Rule. This replaces that Rule's generic, boilerplate `Recommendation` text with real, specific repair guidance — the direct, evidence-grounded realization of `Vision.md`'s own least-answered founding question, "what can I do next?" No new Collector, no new `EvidenceCategory`, and no change to `AssessmentService`'s public entry points are required; this is the smallest architectural footprint of any capability in the Sprint 9 candidate inventory (`POST_SPRINT8_CAPABILITY_PRIORITIZATION_STUDY.md`, Section 6), and the same "give a zero-implementation scaffold crate its first real content" shape Sprint 8 already proved out for `modiq-versioning`.

This document defines the capability and its minimum viable scope only. It does not resolve the open design questions the capability surfaces (Section 8) — those belong to Architectural Resolution, per this project's standing Capability Definition → Architecture Evaluation → Architectural Resolution → Implementation Authorization sequence (`PROJECT_HANDOFF_v1.0.md`, Section 5).

---

# 2. Sprint Objective

Transform existing Findings into actionable Recommendations through the existing `RepairRecipeReference` infrastructure and the `modiq-knowledge` crate — per the mission's own framing, and confirming `SPRINT_ROADMAP_UPDATE_v1.md`, Section 4's already-recorded Sprint 9 commitment. Favor the smallest complete capability; maintain architectural discipline; avoid scope expansion into `KnowledgeModel.md`'s full conceptual model.

---

# 3. Player Question

*"What can I do next?"* — the third of `Vision.md`'s own three founding Assessment questions ("Does this mod work? Why? What can I do next?"), and, by direct inspection of every real Recommendation this platform produces today, the most weakly answered of the three. `StructuralDuplicationRule` and `VersionCompatibilityRule` both currently produce fixed, generic guidance text ("Repackage the archive without duplicate entry names...", "Verify the mod's declared descVersion against a supported Farming Simulator release...") regardless of the specific circumstances of the Finding — real, but not yet the structured, traceable repair guidance `KnowledgeModel.md`'s own Repair Recipe concept describes.

---

# 4. Capability Definition

**Repair Guidance** is the platform's first activation of the Knowledge Domain: a minimum-viable `RepairRecipe` type, real in `modiq-knowledge` for the first time, referenced by at least one existing Rule's `Recommendation` via the already-existing `RepairRecipeReference` seam (`Recommendation::new`'s third parameter, wired since Sprint 2, always `None` in practice until now).

`KnowledgeModel.md` defines a Repair Recipe conceptually: *"A Repair Recipe represents a structured method for resolving a specific engineering issue. Repair Recipes support user education by describing corrective actions rather than automatically modifying user content."* This capability gives that definition its first real, concrete implementation — not a redesign of the concept, an activation of it.

`GOVERNANCE.md`'s Knowledge Domain boundary section already names Repair Recipes explicitly (*"Owns: reusable engineering knowledge, Rule definitions, Repair Recipes, Best Practices, Compatibility Patterns, Known Issues... Must remain independent from any individual Assessment"*) — unlike Sprint 8's discovery of a missing `modiq-versioning` Crate Boundary Rule, no equivalent governance gap was found for `modiq-knowledge` this session. This lowers this capability's governance-preparation footprint relative to Sprint 8's own starting point.

`RepairRecipeReference` already follows ADR-0007's Opaque Runtime References pattern, now twice-proven (`RuleReference`, `VersionProfileReference`) — Runtime stores an opaque identifier only; it does not own or evaluate the Repair Recipe it identifies. This capability activates that pattern's third instance rather than inventing a new one.

---

# 5. Minimum Viable Scope

- `modiq-knowledge` gains real content for `RepairRecipe` — a minimum-viable type carrying enough structured content to inform a Rule's `Recommendation` beyond fixed text (exact field shape is an Architectural Resolution question, Section 8 — not decided here).
- Exactly **one** existing Rule is updated to construct `Some(RepairRecipeReference)` instead of `None`, referencing real `RepairRecipe` content. `VersionCompatibilityRule` is the leading candidate — it is Sprint 8's own newest Rule, already produces a `Warning` Finding with generic text, and both frozen planning documents (`POST_SPRINT8_CAPABILITY_PRIORITIZATION_STUDY.md`, Section 3.6; `SPRINT_ROADMAP_UPDATE_v1.md`, Section 4) illustrate the capability using it as the example — but this is not decided here (Section 8).
- `RepairRecipeReference` itself is reused unchanged — its existing opaque-string shape, constructor, and accessor are not modified by this capability unless Architectural Resolution finds a specific reason to (Section 8).
- One new dependency edge: `modiq-rules` → `modiq-knowledge`, a direct parallel to Sprint 8's own `modiq-rules` → `modiq-versioning` edge.
- No new Collector, no new `EvidenceCategory`, no change to `AssessmentService`'s two public entry points (`execute`, `execute_from_assessment_input`), no change to the Evidence Collection boundary, no change to `RuleEngine::evaluate`'s existing signature.

---

# 6. Explicitly Out of Scope

- Building out `KnowledgeModel.md`'s full conceptual model (`Rule`, `Engine Behavior`, `Compatibility Pattern`, `Best Practice`, `Known Issue`, `Knowledge Reference`) — only `RepairRecipe` gains real content this Sprint. This is the scope-creep risk both frozen planning documents name explicitly.
- Wiring every Rule to a Repair Recipe — exactly one Rule this Sprint, mirroring Sprint 8's own "one hardcoded profile, not exhaustive version knowledge" discipline.
- Any Knowledge Base storage, persistence, lookup registry, or query mechanism. A single, hardcoded, minimum-viable `RepairRecipe` value — the same "one real value, no selection mechanism yet" shape as `VersionProfile::fs25()` — is the expected ceiling, not a general Knowledge Base access layer.
- Standalone Knowledge Base expansion with no Rule consuming it — the Capability Prioritization Study found this repeats a known anti-pattern (the original `EngineAPI` service objects, retired under GOV-004) and is not recommended in isolation; this document does not reopen that finding.
- Rule Selection as a general filtering mechanism — unaffected by this capability, per GOV-012's own standing resolution.
- Referential integrity for Finding/Recommendation references (GOV-005/006 follow-up) — a separate, unranked item, not this Sprint's concern.
- A second Version Profile and Runtime Log Interpretation — both belong to later Sprints per the frozen roadmap (`SPRINT_ROADMAP_UPDATE_v1.md`) and are not reopened, reordered, or reconsidered here.

---

# 7. Repository Impact

*(Expected, contingent on Architectural Resolution and Implementation Authorization — not committed by this document.)*

- `modiq-knowledge` advances from L1 (pure scaffold, unimplemented since Sprint 0) to real content for the first time in the platform's history.
- Exactly one existing Rule's `Recommendation` gains a real `Some(RepairRecipeReference)` in place of `None`.
- One new internal dependency edge (`modiq-rules` → `modiq-knowledge`); no new external dependency.
- No change expected to `modiq-runtime`, `modiq-engine`, `modiq-collection`, `modiq-cli`, or the Sandbox application's own source.
- No breaking change to any public entry point.
- Test suite expected to grow: `modiq-knowledge` (0 tests today) gains its first tests; the updated Rule's existing test suite gains coverage for its new `Some(RepairRecipeReference)` behavior, alongside its unchanged `None`-returning paths for non-matching Evidence.

---

# 8. Architectural Considerations

Identified as open questions for Architectural Resolution — not resolved here, per this session's explicit charter.

1. **Which Rule is the first consumer?** `VersionCompatibilityRule` is the leading candidate (Section 5), but `StructuralDuplicationRule` is an equally real, equally available alternative — both currently produce a `Warning` Finding with fixed text and `None`. Whether one, or eventually both, should be updated this Sprint is a scope decision, not assumed here.
2. **What minimum-viable fields does `RepairRecipe` need?** `KnowledgeModel.md` defines the concept only ("a structured method... describing corrective actions") without specifying a field list. A minimum-viable shape (e.g., an identifier plus guidance text) versus a richer structure (e.g., separate fields for cause, corrective steps, and rationale) is an open design question.
3. **Where is a `RepairRecipe` value constructed and supplied?** Sprint 8 established a precedent — `VersionProfile::fs25()`, a single hardcoded value constructed internally by `AssessmentService` — but that value is consumed by `RuleEngine::evaluate` as an explicit parameter. Whether the analogous `RepairRecipe` value is constructed inside the consuming Rule itself, or supplied to it the way `VersionProfile` is supplied to Rules today, is not decided here.
4. **Does `GOVERNANCE.md`'s existing Knowledge Domain boundary section need any amendment specific to this capability**, or is its current text (already naming Repair Recipes explicitly) sufficient as-is? Unlike Sprint 8's discovered `modiq-versioning` gap, no gap was found this session — but Architectural Resolution should confirm this rather than assume it from this document alone.
5. **Should `RepairRecipeReference`'s existing opaque-string shape change**, or is it sufficient, unchanged, to carry whatever identifier scheme `RepairRecipe`'s own minimum-viable shape ultimately uses? No evidence found this session suggests a change is required, but this is a design judgment, not a foregone conclusion.

---

# 9. Implementation Risks

- **Scope creep** is the principal named risk in both frozen planning documents (`POST_SPRINT8_CAPABILITY_PRIORITIZATION_STUDY.md`, Section 6; `SPRINT_ROADMAP_UPDATE_v1.md`, Section 4): building `KnowledgeModel.md`'s full conceptual model at once, rather than the single minimum-viable `RepairRecipe` type this document scopes. Mitigated by Section 6's explicit exclusions.
- **Reference-shape risk**, low: if `RepairRecipe`'s minimum-viable content turns out richer than `RepairRecipeReference`'s current opaque-string shape can identify, a shape change may be needed — a small, contained risk given ADR-0007's pattern has already absorbed two prior domains without modification.
- **Rule-selection risk**, low: choosing the "wrong" first Rule (Section 8, Item 1) carries little cost — both candidate Rules are structurally similar, and either choice establishes the same activation pattern for the other to follow later.
- **Overall implementation risk: Low–Medium**, consistent with both frozen planning documents' own rating — this is architecturally the least novel candidate in the Sprint 9 inventory, activating dormant, already-designed scaffolding rather than building new architectural surface.

---

# 10. Success Criteria

Per this project's standing Capability Success Criteria convention (`PROJECT_HANDOFF_v1.0.md`, Section 6, established at Sprint 7):

**After this Sprint, modIQ can now** produce at least one Recommendation backed by real, structured Repair Recipe content — traceable through a real `RepairRecipeReference` to `modiq-knowledge`'s first real engineering knowledge — rather than only fixed, generic guidance text, for every Assessment that triggers the updated Rule.

Concretely, if Sprint 9 completes as scoped: a Player or Creator whose Assessment produces the updated Rule's `Warning` Finding receives a Recommendation carrying a real `Some(RepairRecipeReference)`, resolvable to real `RepairRecipe` content in `modiq-knowledge` — the direct, evidence-based mechanism behind `Vision.md`'s own belief that "software should educate its users rather than simply produce results," unreachable by any capability that exists today.

---

# 11. Chief Architect Questions

*(Not answered here.)*

1. Should `VersionCompatibilityRule` be the confirmed first consumer of a real `RepairRecipe`, as both frozen planning documents illustrate, or should `StructuralDuplicationRule` be chosen instead, or should this Sprint scope both?
2. What minimum-viable field shape should `RepairRecipe` take — an identifier and guidance text only, or a richer structure separating cause, corrective steps, and rationale — and should that shape be validated against `KnowledgeModel.md`'s other six knowledge categories now, to avoid a structural mismatch if one of them gains real content later?
3. Should a `RepairRecipe` value be constructed internally by the consuming Rule (mirroring `modiq-versioning`'s `VersionProfile::fs25()` precedent), or supplied to it as an explicit parameter (mirroring how `VersionProfile` itself reaches Rules today)?
4. Is `GOVERNANCE.md`'s existing Knowledge Domain boundary section sufficient as written for this capability, or does it need an explicit amendment naming `modiq-knowledge`'s Crate Boundary Rules the way Sprint 8 eventually named `modiq-versioning`'s?
5. Should `RepairRecipeReference`'s opaque-string shape be confirmed sufficient before implementation begins, or is there reason to evaluate a shape change now rather than discover a mismatch mid-implementation?

---

Awaiting Chief Architect review. No implementation, documentation change, governance item, or ADR has been made this session.

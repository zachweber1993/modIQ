# Post-Sprint 8 Capability Prioritization Study

| Property | Value |
|---|---|
| **Document** | POST_SPRINT8_CAPABILITY_PRIORITIZATION_STUDY.md |
| **Project** | modIQ |
| **Purpose** | Product strategy and capability prioritization for Sprint 9 and beyond — planning only |
| **Prepared by** | Lead Engineer (Sonnet 5), on `feature/runtime-implementation` |
| **Repository baseline** | `feature/runtime-implementation`, HEAD `ad29fd6` (Sprint 8 formally closed) |
| **Status** | Planning and product strategy only. No implementation, no architectural change, no governance work, no documentation modification. Awaiting Chief Architect selection of Sprint 9. |

---

# 1. Repository Capability Summary

Verified directly against source this session (`git status` clean, 205/205 tests passing, HEAD `ad29fd6`), not carried forward from any prior session's own account.

## Current implemented capabilities

- **Evidence Collection**, three real Collectors, composed inline by `AssessmentService`: `EvidenceCollector` (filesystem structure), `ArchiveCollector` (ZIP structure, resource limits, traversal/duplicate policies), `XmlCollector` (`modDesc.xml` well-formedness, declared dependencies, declared `descVersion` — the last added Sprint 8).
- **Rule evaluation**, three real Rules, fixed declaration order, independently composed: `EvidencePresenceRule` (generic, category-agnostic), `StructuralDuplicationRule` (archive duplicate-entry `Warning`), `VersionCompatibilityRule` (declared-`descVersion`-vs-active-profile `Warning`, Sprint 8).
- **Version Profile-aware compatibility**, minimum viable: a single hardcoded `VersionProfile::fs25()`, recognizing exactly `descVersion` 93.
- **Reporting**: `AssessmentReport`, a pure, read-only snapshot — no analysis, no formatting beyond what Runtime carries.
- **CLI and Sandbox**, both real, both driving the identical `AssessmentService::execute_from_assessment_input` entry point.

## Current architecture

Nine crates, strictly downward dependency, zero circular dependencies across eight Sprints. `AssessmentService` is the sole orchestration boundary, composing every subsystem's real type directly (ADR-0010/GOV-004). `modiq-versioning` is real but minimal (L2); `modiq-knowledge` and `modiq-common` remain pure, zero-implementation scaffolding, each with zero forcing function across eight Sprints.

## Current Evidence pipeline

Of the eight `EvidenceCategory` variants (defined since before Sprint 1), **three are produced today**: `FileStructureAnalysis`, `StructuralDuplication`, `XmlInspection`. **Five remain defined but never produced by any Collector**: `LuaAnalysis`, `RuntimeLogs`, `AssetValidation`, `DependencyResolution`, `PerformanceObservations` — each corresponds directly to a named `ProductSpecification.md` Assessment Engine responsibility.

## Current Rule Engine

`RuleEngine::evaluate(evidence: &[Evidence], version_profile: &VersionProfile) -> Vec<RuleOutcome>` — three Rules, fixed order, no suppression, no Rule Selection filtering mechanism (every Rule is always applicable; Sprint 8 deliberately did not build general-purpose Rule Selection, per its own Decision 3). Every `Recommendation` produced by any real Rule today passes `None` for its `repair_recipe_reference` — `RepairRecipeReference` has existed as opaque scaffolding since Sprint 2 and has never once been populated with real content.

## Current Assessment pipeline

Single-mod only. Every Assessment evaluates exactly one Assessment Subject (a file, directory, or archive) via exactly one Assessment Input. `DataModel.md`'s own Assessment Subject examples list "Mod collections" and "Savegames" alongside individual mods — neither has ever been implemented; every real Assessment to date is architecturally a single-mod Assessment.

## Current Version Profile capability

Real but intentionally minimal: one profile, one recognized `descVersion` value, no selection mechanism, no second profile, `AssessmentService`'s two public entry points unchanged (Sprint 8's own explicit implementation simplification).

---

# 2. Capability Inventory

Every significant capability found unimplemented, by direct inspection — not assumed to be limited to the mission's own illustrative examples.

1. Runtime Log Interpretation
2. Dependency Resolution
3. Asset Validation
4. Lua Static Analysis
5. Performance Observations
6. Knowledge Base expansion (standalone)
7. Repair Guidance (real `RepairRecipeReference` content)
8. Report improvements (Display/Serialize, structured output, "Mod Health" synthesis)
9. A second Version Profile (e.g. FS22)
10. Referential integrity for Finding/Recommendation references (GOV-005/006 follow-up)

Items 6 and 7 are evaluated together below — Knowledge Base expansion has no standalone forcing function (Section 4), and Repair Guidance is the concrete capability that would finally give it one.

---

# 3. Capability Comparison Matrix

## 3.1 Runtime Log Interpretation

**Player question:** *"Why did my game fail to load?"* — directly, word for word, one of `Vision.md`'s own three founding questions ("Does this mod work? Why? What can I do next?").

**Architectural Readiness:** `EvidenceCategory::RuntimeLogs` exists, unused. Parsing itself would likely be the platform's *simplest* content-inspection to date — plain text, pattern/regex matching against known FS log error signatures, no exotic dependency. But a genuine, unprecedented architectural question exists: every Collector to date inspects the mod's own package (one Assessment Input). A runtime log is generated by the game *after* attempting to load a mod — a fundamentally different artifact, arguably requiring a second, distinct input alongside the mod itself, or a new Assessment Subject shape entirely. No existing Collector composition precedent (Sprint 7's own multi-Collector work) addresses *multiple Assessment Inputs* — only multiple Collectors against the *same* one.

**Product Value: Very High.** Directly answers the single most common, most painful real-world player moment; unmatched by any other candidate in visceral relevance.

**Engineering Effort: Medium–Large.** Text parsing itself is low-complexity; the "how does a log get supplied" architectural question requires real design work this platform has never done before, likely its own Architecture Evaluation.

**Strategic Leverage: Medium–High.** Would establish the platform's first multi-input Assessment pattern — real leverage for future capabilities (assessing a mod against its own crash log, or a save game against installed mods) but no immediate transfer to purely single-mod-input capabilities.

---

## 3.2 Dependency Resolution

**Player question:** *"What dependency is missing?"* / *"Are all the mods in my collection compatible with each other?"*

**Architectural Readiness:** `XmlCollector` already extracts declared dependency *names* (Sprint 7) — the identification half is done. *Resolution* (checking whether a named dependency is actually present) requires evaluating more than one mod at once — `DataModel.md`'s own "Mod collections" Assessment Subject, specified conceptually since Documentation Release 1.0, never implemented. Every real Assessment today is single-mod; this would be the first departure from that shape.

**Product Value: High.** A named Server Administrator objective ("Validate complete mod collections," "Identify conflicting mods") and the harder half of a named Player objective ("Dependency identification").

**Engineering Effort: Large.** Requires the Mod Collection Assessment Subject to become real — a bigger structural change than any single-Collector or single-Rule capability shipped to date, closer in scope to Sprint 3's original Evidence Collection boundary work than to Sprint 7's or Sprint 8's own additive extensions.

**Strategic Leverage: High.** Realizing "Mod collection" as a genuine Assessment Subject would open several Server-Administrator-facing capabilities at once (conflict detection, batch assessment, collection-wide compatibility summaries) behind the same architectural investment.

---

## 3.3 Asset Validation

**Player question:** *"What assets are missing or broken?"*

**Architectural Readiness:** `EvidenceCategory::AssetValidation` exists, unused. A minimum-viable version — confirming that asset paths referenced in the manifest/scripts actually exist in the package — reuses two already-proven techniques directly: XML-content extraction (`XmlCollector`'s own precedent) and filesystem existence checking (`EvidenceCollector`'s own precedent). Deep validation of individual asset formats (textures, models) would need format-specific parsers and is properly out of scope for a minimum-viable slice.

**Product Value: Medium–High.** Real, concrete, understandable failure mode for both Players and Creators; less foundational than compatibility, less visceral than a crash.

**Engineering Effort: Medium.** The MVP (referenced-path-exists) is architecturally no harder than `XmlCollector` was. Deeper per-format validation would be Large but is avoidable at this scope.

**Strategic Leverage: Medium.** Reuses existing techniques well; does not open a fundamentally new architectural door the way Dependency Resolution or Repair Guidance would.

---

## 3.4 Lua Static Analysis

**Player question:** *"Will this script likely crash?"* / *"Is this mod well-engineered?"*

**Architectural Readiness:** `EvidenceCategory::LuaAnalysis` exists, unused. `EvidenceCollection.md` itself names this **"the platform's highest-risk future collector"** — no Lua parsing dependency has ever been evaluated (unlike `roxmltree`, selected only after Sprint 7's own dependency-authorization step), and "static analysis" is far less bounded a concept than "read one attribute" — syntax validity, undefined-global detection, and common anti-patterns are all plausible scope, each with a different risk/complexity profile. Genuine risk of the observational boundary eroding (deciding a coding pattern is "bad" edges toward interpretation, which belongs to the Rule Engine, not the Collector).

**Product Value: Very High**, in principle — matches a named Vision.md pillar and a named Creator objective ("Detect implementation issues"). But the value is diffuse until a specific, bounded static-analysis scope is chosen.

**Engineering Effort: Large–Very Large.** Almost certainly needs its own dedicated risk-scoping investigation (mirroring Sprint 4 Phase 2's Boundary-Proving against the `zip` crate) before even an Architecture Evaluation can meaningfully begin.

**Strategic Leverage: Medium.** Establishes a second real content-inspecting Collector template, but Lua's own complexity is unlikely to transfer cleanly to other future Collectors (asset and log formats are structurally quite different).

---

## 3.5 Performance Observations

**Player question:** *"Will this impact my game's performance?"*

**Architectural Readiness:** `EvidenceCategory::PerformanceObservations` exists, unused — and a direct architectural conflict was found this session: `Architecture.md`'s own Architectural Constraints section explicitly excludes **"runtime memory analysis"** and **"opaque scoring systems."** True performance measurement is therefore architecturally prohibited outright; only static heuristic proxies (file sizes, texture dimensions, polygon counts where parseable) remain available, and those carry real risk of drifting into an opaque score if not scoped with unusual care.

**Product Value: Medium.** Real player interest, but the only architecturally permissible implementation is inherently weak (heuristic, not measured).

**Engineering Effort: Large.** The ambiguity itself is a cost — this candidate would need its own capability-definition research just to determine what is measurable at all within the existing constraints, before any Collector work could begin.

**Strategic Leverage: Low–Medium.** Narrow, format-specific heuristics; limited transfer to other capabilities.

**This is the weakest candidate in the inventory**, not primarily on effort grounds but because of a direct, named conflict with `Architecture.md`'s own frozen constraints.

---

## 3.6 Knowledge Base Expansion (standalone) / 3.7 Repair Guidance (paired)

**Player question:** *"What can I do next?"* — the third of `Vision.md`'s own three founding questions, and, by direct inspection, the most weakly answered of the three today: every real Recommendation currently produced is generic boilerplate text ("Review the collected evidence...", "Repackage the archive...", "Verify the mod's declared descVersion..."), and `RepairRecipeReference` — real, tested, wired through `Recommendation::new`'s constructor since Sprint 2 — has never once been populated with anything but `None`.

**Architectural Readiness: unusually high for a Knowledge Domain capability.** Unlike every prior Sprint's own finding ("`modiq-knowledge` has zero forcing function"), this pairing *is* the forcing function: a minimum-viable `RepairRecipe` type in `modiq-knowledge`, referenced by exactly one existing Rule (e.g., `VersionCompatibilityRule` recommending a specific, real repair recipe instead of generic text), requires no new Collector, no new Evidence category, and no change to `AssessmentService`'s public entry points — the smallest architectural footprint of any candidate in this inventory. Standalone Knowledge Base expansion (with no Rule consuming it) would repeat the exact speculative-scaffolding pattern this project already paid for once (the original `EngineAPI` service objects, retired under GOV-004) — **not recommended in isolation.**

**Product Value: High.** Directly deepens the platform's own weakest-answered founding question, for every Rule that adopts it — not a narrow, single-category addition.

**Engineering Effort: Medium–Large.** `modiq-knowledge`'s first real implementation (a minimum-viable `RepairRecipe`), plus updating at least one Rule's own `Recommendation` construction to reference it — bounded, well-precedented work (the same "give one scaffold crate its first real content" shape Sprint 8 itself just completed for `modiq-versioning`).

**Strategic Leverage: Very High — the highest in this inventory.** This is the only candidate that improves the *quality of every existing and future Rule's output*, rather than adding one new, narrow Evidence category. It is also the direct, concrete realization of `KnowledgeModel.md`'s and `RuleEngine.md`'s own long-stated intentions, dormant since Documentation Release 1.0.

---

## 3.8 Report Improvements

**Player question:** not a new question — "can the answer I already have be read, exported, or consumed more easily?"

**Architectural Readiness:** `AssessmentReport` is stable and minimal. `Display`/`Serialize` for Runtime identity/enum types has been flagged as a known limitation in **eight consecutive Engineering Release records** — low-risk, well-scoped, no redesign needed. A synthesized "Mod Health" view (`Glossary.md`'s own long-defined-but-never-built concept) would require care: `Architecture.md` states Reporting "performs no analysis," so any real synthesis across Findings belongs in the Rule Engine producing a summarizing Finding, not in Reporting itself — worth naming as a boundary question, not assumed trivial.

**Product Value: Medium.** Real usability value, not a new capability.

**Engineering Effort: Small (Display/Serialize) to Medium (Mod Health, if pursued).**

**Strategic Leverage: Medium.** A clean, low-risk enabler for any future consumer (e.g., a structured UI), but does not open new product capability the way Repair Guidance or Dependency Resolution would.

---

## 3.9 A Second Version Profile (e.g. FS22)

**Player question:** *"Does this work with my specific version of the game?"* — extends Sprint 8's own answer to more than one version.

**Architectural Readiness:** Sprint 8 was explicitly built toward this as the forcing function for its own deferred questions (a real profile-selection mechanism; whether `AssessmentService`'s entry points should finally evolve). Well-precedented, well-understood scope.

**Product Value: Medium–High.** Real extension of an existing, working capability; less novel than any candidate answering a genuinely new question.

**Engineering Effort: Medium.** Requires resolving the profile-selection question Sprint 8 deliberately left open.

**Strategic Leverage: High**, specifically for closing Sprint 8's own named outstanding items — but narrower in scope than a new capability.

---

## 3.10 Referential Integrity (GOV-005/006 follow-up)

Not a user-facing capability — an internal correctness item (whether a Finding's `EvidenceId` references actually resolve within the same Assessment). **Player question: none directly.** Low effort, low leverage beyond closing an old thread. Noted for completeness; not a serious Sprint 9 contender.

---

# 4. Capability Rankings

Balanced across player value, architectural readiness, engineering effort, and strategic leverage — **not** sorted by effort alone.

| Rank | Capability | Player Value | Effort | Leverage | Why this rank |
|---|---|---|---|---|---|
| 1 | **Repair Guidance** (paired with minimum `modiq-knowledge`) | High | Medium–Large | **Very High** | Smallest architectural footprint of any candidate that meaningfully deepens the platform (no new Collector, no new Evidence category, no `AssessmentService` change); the only candidate that improves *every* Rule's output, not just one new category; directly answers the most weakly-served of `Vision.md`'s three founding questions. |
| 2 | **Runtime Log Interpretation** | Very High | Medium–Large | Medium–High | Highest raw player-value candidate, matching `Vision.md` almost verbatim — ranked second, not first, because it carries a genuinely unprecedented architectural question (multi-input Assessment) that Repair Guidance does not. |
| 3 | **Lua Static Analysis** | Very High (diffuse) | Large–Very Large | Medium | Real long-term value, but this project's own specification already flags it as highest-risk; needs a dedicated risk-scoping phase before design, the same discipline Sprint 4 applied to the `zip` crate. |
| 4 | **Dependency Resolution** | High | Large | High | Strong value, but requires the platform's first multi-mod Assessment Subject — a structural investment closer to Sprint 3's own scope than to any Sprint since. |
| 5 | **A second Version Profile** | Medium–High | Medium | High | Real, well-precedented leverage for Sprint 8's own deferred questions, but narrower — widening an existing capability, not answering a new question. |
| 6 | **Asset Validation** | Medium–High | Medium | Medium | Solid, reuses proven techniques well, but does not open a new architectural door. |
| 7 | **Report Improvements** | Medium | Small–Medium | Medium | Worthwhile, low-risk, but a quality-of-life improvement, not a capability. |
| 8 | **Performance Observations** | Medium | Large | Low–Medium | Direct conflict with `Architecture.md`'s own exclusion of runtime memory analysis and opaque scoring; weakest candidate. |
| — | Knowledge Base expansion (standalone) | — | — | — | Superseded by #1 — no standalone forcing function; would repeat a known anti-pattern if pursued alone. |
| — | Referential integrity | None (internal) | Small | Low | Minor technical debt, not ranked as a capability Sprint. |

---

# 5. Recommended Multi-Sprint Roadmap

Illustrative, evidence-grounded, not a commitment — each Sprint's own Capability Definition should still ask its own capability question rather than assume this sequence, per this project's own standing discipline.

- **Sprint 9 — Repair Guidance.** `modiq-knowledge`'s first real content; at least one existing Rule referencing a real `RepairRecipe`. Smallest footprint, highest leverage, closes the platform's oldest-standing dormant scaffolding (`RepairRecipeReference`, since Sprint 2).
- **Sprint 10 — Runtime Log Interpretation.** The clearest remaining "answers the founding question directly" candidate; benefits from Sprint 9's own Knowledge Base groundwork (a log-derived Finding could reference a real Repair Recipe immediately, rather than generic text).
- **Sprint 11 — A second Version Profile, or Dependency Resolution's own dedicated Architecture Evaluation.** By this point, real evidence exists (two content Collectors' worth of Knowledge Base consumption, a second real capability) to responsibly decide whether Dependency Resolution's multi-mod Assessment Subject is worth its own larger investment, or whether widening Version Profile support first is the better-sequenced choice.
- **Sprint 12 — Lua Static Analysis risk-scoping investigation**, mirroring Sprint 4 Phase 2's Boundary-Proving discipline — an investigation phase producing evidence and a recommendation, not itself an implementation authorization, given this candidate's own flagged risk level.

---

# 6. Sprint 9 Recommendation

**Recommend exactly one capability: Repair Guidance — `modiq-knowledge`'s first real implementation, paired with at least one existing Rule referencing real `RepairRecipe` content.**

**User value.** Directly strengthens the single most weakly-answered of `Vision.md`'s own three founding Assessment questions — "what can I do next?" — for every Assessment the platform already produces, not only future ones. A Player or Creator encountering a `Warning` today (declared-version mismatch, structural duplication) receives generic, non-actionable text; this capability replaces that with real, specific guidance, the direct realization of Vision.md's own belief that "software should educate its users rather than simply produce results."

**Architectural fit.** The smallest architectural footprint of any ranked candidate: no new Collector, no new `EvidenceCategory`, no change to `AssessmentService`'s public entry points, no change to the Evidence Collection boundary. `RepairRecipeReference` already exists, already tested, already wired through `Recommendation::new`'s constructor — this capability activates dormant, already-designed scaffolding rather than building new architectural surface, the same shape Sprint 8 itself just proved out for `modiq-versioning`.

**Engineering confidence: High.** This is architecturally the least novel candidate in the inventory — "give a zero-implementation scaffold crate its first minimum-viable real content, consumed by exactly one existing Rule" is precisely Sprint 8's own template, applied to a different crate.

**Implementation risk: Low–Medium.** The main risk is scope creep — building out `KnowledgeModel.md`'s full conceptual model (`Rule`, `Engine Behavior`, `Compatibility Pattern`, `Best Practice`, `Known Issue`, `Knowledge Reference`) all at once, rather than the single minimum-viable `RepairRecipe` type this recommendation scopes. A dedicated Capability Definition session should name this explicitly out of scope, mirroring Sprint 8's own "one hardcoded profile, not exhaustive version knowledge" discipline.

**Expected repository impact.** `modiq-knowledge` advances from L1 (pure scaffold, unimplemented since Sprint 0) to real content for the first time in the platform's history. At least one Rule's `Recommendation` gains a real `Some(RepairRecipeReference)` instead of `None`. No new crate, no new dependency edge beyond `modiq-rules` → `modiq-knowledge` (a direct parallel to Sprint 8's own `modiq-rules` → `modiq-versioning` edge). No breaking change to any public entry point.

---

# 7. Future Platform Leverage

Architectural investments made in Sprint 8 expected to benefit multiple future capabilities — identified, not expanded upon; no new abstraction recommended.

- **The Opaque Runtime Reference pattern (ADR-0007) was confirmed to generalize beyond the Knowledge Domain it was originally written for** (`VersionProfileReference`, extending `RuleReference`/`RepairRecipeReference`'s existing shape to a Version-domain relationship). This directly de-risks Repair Guidance's own design: `RepairRecipeReference` already follows the identical, now twice-proven pattern.
- **The precedent of a single, hardcoded, internally-constructed minimum-viable default value** (`VersionProfile::fs25()`, supplied by `AssessmentService` itself rather than a caller) is directly reusable for Repair Guidance's own first minimum-viable `RepairRecipe` — the same "one real value, no selection mechanism yet" shape.
- **Confirmation that `RuleEngine::evaluate`'s signature can evolve to accept additional external context** (the `VersionProfile` parameter) **without requiring any change to `AssessmentService`'s public entry points** de-risks a future evolution where a Rule might need to accept Knowledge Base content the same way — the exact shape Repair Guidance would need.
- **The "a Rule consumes external context directly, rather than through a general Rule Selection filtering mechanism" pattern** (Decision 3) is a reusable template for how a future Rule could consume Knowledge Base content directly, without first needing to build the general-purpose Rule Selection infrastructure this project has twice now found no forcing function for.
- **The Content Extraction discipline** (`XmlCollector`'s "parse once, extract multiple independent facts") remains directly reusable for a future Log or Asset Collector, unaffected by and unrelated to Sprint 8's own Version Profile work specifically, but confirmed durable across three Sprints of use (7 and 8 both).

---

# 8. Chief Architect Questions

*(Not answered here.)*

1. Should Sprint 9 prioritize player-facing detection breadth (Log Interpretation, Lua, Asset Validation, Dependency Resolution) over platform-wide explainability depth (Repair Guidance)? This study recommends the latter; the former remains a legitimate alternative view.
2. Does Lua Static Analysis's own flagged risk level (`EvidenceCollection.md`: "highest-risk future collector") warrant scheduling a dedicated risk-scoping investigation now, in parallel with whichever capability is chosen for Sprint 9's own build, rather than waiting until Sprint 12?
3. Does Dependency Resolution's requirement for a real "Mod Collection" Assessment Subject warrant being treated as its own larger, possibly multi-Sprint initiative — closer in scope to Sprint 3's original Evidence Collection boundary work — rather than a single Sprint's capability?
4. Should the Version Profile questions Sprint 8 deliberately deferred (profile-selection mechanism, whether `AssessmentService`'s entry points should finally evolve) be resolved opportunistically whenever they next become relevant, or explicitly scheduled as their own Sprint?
5. Should Performance Observations be formally ruled out now, given its direct conflict with `Architecture.md`'s existing exclusion of runtime memory analysis and opaque scoring systems, so it stops recurring as an ambiguous candidate in future capability inventories?

---

Awaiting Chief Architect selection of Sprint 9. No implementation, architectural change, governance work, or documentation modification has been made this session.

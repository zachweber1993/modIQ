# Sprint 9 Architectural Resolution

| Property | Value |
|---|---|
| **Document** | SPRINT9_ARCHITECTURAL_RESOLUTION.md |
| **Project** | modIQ |
| **Purpose** | Architectural Resolution of every Architectural Consideration and Chief Architect Question raised by `SPRINT9_CAPABILITY_DEFINITION.md` — Repair Guidance (Minimum Viable `modiq-knowledge` Activation) |
| **Prepared by** | Lead Engineer (Sonnet 5), on `feature/runtime-implementation` |
| **Repository baseline** | `feature/runtime-implementation`, HEAD `aaa08fe` |
| **Status** | **Architectural Resolution complete. Implementation has NOT been authorized.** Every decision below is resolved, not merely evaluated — per this session's explicit charter not to defer decisions required before implementation. **Amended following Chief Architect clarification** on Question 2 (Section 3): the Rule-internal construction accepted for Sprint 9 is a minimum-viable *retrieval* mechanism, not a long-term authorship model — `modiq-knowledge` authors the actual Repair Recipe content; `VersionCompatibilityRule` only consumes it. No code changed, no documentation changed, no commits, no branch, no Governance Register item, no ADR. Awaiting Chief Architect approval before Implementation Authorization. |

---

# 0. Repository Verification

Verified directly against source this session, not carried forward from any prior session's own account.

| Check | Result |
|---|---|
| Working tree | `git status` — clean except for the untracked `SPRINT9_CAPABILITY_DEFINITION.md` produced by the prior Capability Definition session; no other change present |
| Current branch | `feature/runtime-implementation`, in sync with `origin` (0 ahead / 0 behind) |
| Latest commit | `aaa08fe` — "docs(planning): finalize Sprint 9 roadmap and planning cycle" |
| Repository consistency | `SPRINT9_CAPABILITY_DEFINITION.md` re-read in full this session; its own repository evidence (modiq-knowledge's zero-implementation state, both Warning-producing Rules passing `None`, the absent `modiq-rules` → `modiq-knowledge` edge) re-verified directly against current source, confirmed unchanged |

No repository change has occurred since Capability Definition that its own evidence did not already anticipate. Capability Definition, Sprint selection, roadmap planning, Runtime Log planning, Capability Prioritization, and Sprint 9's own prior Architecture Evaluation (`SPRINT9_RUNTIME_LOG_ARCHITECTURE_EVALUATION.md`, a Sprint 10 concern) are treated as closed inputs and not reopened.

---

# 1. Executive Summary

This document resolves every Architectural Consideration (Section 8) and Chief Architect Question (Section 11) raised by `SPRINT9_CAPABILITY_DEFINITION.md`. The resolved architecture: `VersionCompatibilityRule` becomes the sole Sprint 9 consumer of a minimum-viable `RepairRecipe` type, authored in `modiq-knowledge` as a named, self-contained constructor (`RepairRecipe::version_compatibility_declared_version_mismatch()`, mirroring `VersionProfile::fs25()`) and merely *called* by the Rule, which supplies it to its existing `Recommendation` construction via the already-existing `RepairRecipeReference` seam. This requires exactly one new dependency edge (`modiq-rules` → `modiq-knowledge`), no new public API surface above `modiq-rules`, no change to `RuleEngine::evaluate`'s signature, and no change to `AssessmentService`'s two public entry points — the smallest architecture capable of delivering the approved capability, and a direct extension of ADR-0007's Opaque Runtime References pattern into its third proven instance.

**Chief Architect clarification, incorporated:** the initial draft of this resolution had `VersionCompatibilityRule` construct `RepairRecipe::new(literal, literal)` directly, with the actual guidance text authored inline inside `modiq-rules`. The Chief Architect correctly identified this as acceptable only as a Sprint 9 minimum-viable *mechanism*, not as the Knowledge Domain's long-term ownership model — reusable engineering knowledge belongs to `modiq-knowledge` and is *consumed* by Rules, not *authored* by them. Question 2 (Section 3) is revised below to reflect this: `modiq-knowledge` owns the specific Repair Recipe content as a named constructor; `modiq-rules` only calls it. `RepairRecipe::new` remains available as a general, infallible constructor (mirroring `VersionProfile::new` alongside `VersionProfile::fs25()`), but no production code in `modiq-rules` calls it with inline literal knowledge content.

No ADR is produced. Every decision below is scoped to Sprint 9's own activation of dormant scaffolding, using patterns this repository has already established (Section 12) — none introduces a new cross-platform architectural concept extending beyond Sprint 9's own capability.

---

# 2. Architectural Overview

The resolved shape, in one pass:

```text
modiq-knowledge (authors the knowledge):
    RepairRecipe::version_compatibility_declared_version_mismatch()
        — a named, self-contained constructor, mirroring
          VersionProfile::fs25(); the identifier and guidance
          text live here, not in modiq-rules.

VersionCompatibilityRule::evaluate (consumes the knowledge):
        │
        ├─ (unchanged) detects unsupported declared descVersion Evidence
        │
        ├─ calls RepairRecipe::version_compatibility_declared_version_mismatch()
        │        — the Rule retrieves a pre-authored recipe; it does not
        │          author identifier/guidance content itself
        │
        ├─ Finding::new(...)                                     [unchanged]
        │
        └─ Recommendation::new(
               recipe.guidance(),                                 [was: fixed literal text]
               vec![finding.id()],
               Some(RepairRecipeReference::new(recipe.identifier())) [was: None]
           )
```

Everything below `VersionCompatibilityRule` (`Finding`, `Recommendation`, `Assessment`, `AssessmentReport`, `RuleEngine::evaluate`, `AssessmentService`) is structurally unchanged. The capability's *retrieval* mechanism (how the Rule obtains a `RepairRecipe` value, without a new `RuleEngine::evaluate` parameter) is realized inside the Rule, the same shape `RuleReference::new("version-compatibility-rule")` already demonstrates for a Rule referencing its own hardcoded identity. But the *authorship* of the recipe's content — its identifier and guidance text — lives in `modiq-knowledge`, not `modiq-rules`, preserving the Knowledge Domain boundary `modiq-knowledge`'s own README states: reusable engineering knowledge belongs to the knowledge domain and is consumed by Rules, not authored by them.

This is a materially smaller footprint than Sprint 8's own Version Profile activation, which required a new `Assessment` field, a new `RuleEngine::evaluate` parameter, and internal construction inside `AssessmentService` — because Version Profile is genuinely external context (it varies by which game version an Assessment targets, supplied once per Assessment to every version-aware Rule), while a Repair Recipe is intrinsic knowledge belonging to the specific Rule that already knows which issue it diagnoses. This distinction is the basis for Decision 2 (Section 3) and is not incidental — it is the reason this capability can activate `modiq-knowledge` without touching `modiq-engine` at all, while still keeping `modiq-knowledge`, not `modiq-rules`, as the author of the knowledge content itself.

---

# 3. Resolution of Each Chief Architect Question

Restated from `SPRINT9_CAPABILITY_DEFINITION.md`, Section 11, resolved in order. Each resolution also closes the corresponding Architectural Consideration from that document's Section 8.

## Question 1 / Consideration 1 — Which Rule is the first consumer?

**Decision:** `VersionCompatibilityRule` is the sole Sprint 9 consumer. `StructuralDuplicationRule` is explicitly deferred — not wired this Sprint.

**Rationale:** Both frozen planning documents (`POST_SPRINT8_CAPABILITY_PRIORITIZATION_STUDY.md`, Section 3.6; `SPRINT_ROADMAP_UPDATE_v1.md`, Section 4) illustrate the capability using `VersionCompatibilityRule` as the concrete example. It is also this platform's newest Rule (Sprint 8), giving it the least accumulated test surface to extend, and its declared-version Warning is a natural fit for specific, structured repair guidance ("verify against a supported release" is more amenable to concrete, actionable content than `StructuralDuplicationRule`'s already-fairly-specific "repackage without duplicate entries").

**Repository precedent:** Sprint 8 activated `modiq-versioning` through exactly one Rule (`VersionCompatibilityRule`) even though `EvidencePresenceRule` and `StructuralDuplicationRule` both already existed and could, in principle, have consulted a Version Profile too. Single-Rule activation of a newly-real scaffold crate is this repository's own established pattern, not a novel constraint invented here.

**Alternatives considered:**
- **(a) `StructuralDuplicationRule` only.** No textual precedent in either frozen planning document, and no evidentiary advantage over `VersionCompatibilityRule`.
- **(b) Both Rules simultaneously.** Doubles the implementation and test surface for a Sprint whose stated objective (mission: "activate the minimum viable Repair Guidance capability," not "the ultimate Knowledge System") is activation, not breadth.

**Why rejected:** (a) offers no reason to prefer it over the already-illustrated candidate; (b) is scope expansion beyond what Sprint 9's own Capability Definition committed to ("exactly one existing Rule"), and duplicates hardcoded `RepairRecipe` construction logic across two Rule modules before this platform has proven the pattern once. Wiring a second Rule is explicitly available to a future Sprint once this activation is proven, per the "smallest architecture" principle.

---

## Question 2 / Consideration 3 — Where is a `RepairRecipe` value constructed and supplied?

This question has two separable parts, distinguished explicitly following Chief Architect clarification: **(i)** how does the Rule *obtain* a `RepairRecipe` at evaluation time (a parameter-threading question), and **(ii)** where is the recipe's actual content — its identifier and guidance text — *authored* (an ownership question)? The initial draft of this resolution answered (i) correctly but conflated it with (ii), having the Rule both retrieve and author the content in the same call. This revision separates them.

**Decision:**
- **(i) Retrieval:** `VersionCompatibilityRule::evaluate` obtains its `RepairRecipe` by calling a function directly, at evaluation time — not by receiving it as a new `RuleEngine::evaluate` or `AssessmentService` parameter.
- **(ii) Authorship:** The recipe's actual content is authored in `modiq-knowledge`, as a named, self-contained associated function on `RepairRecipe` — `RepairRecipe::version_compatibility_declared_version_mismatch() -> Self` — analogous to `VersionProfile::fs25()`. `VersionCompatibilityRule` calls this function; it does not construct a `RepairRecipe` from inline string literals of its own. `RepairRecipe::new` (Question 3) remains a general-purpose, infallible constructor in `modiq-knowledge`, used by that named function internally and available for tests and future recipes — but no production code outside `modiq-knowledge` calls `RepairRecipe::new` with hand-authored content.

**Rationale:** Part (i) still holds for the reason originally given: a Repair Recipe does not vary across Assessments the way a Version Profile does, so no threading mechanism through `RuleEngine::evaluate` is warranted — see Alternatives, below. Part (ii) is the corrected element: `modiq-knowledge`'s own README states its Responsibilities include "Repair Recipes" and its Non-Responsibilities exclude "Rule evaluation," and its own closing Engineering Note is explicit — *"The knowledge domain preserves engineering understanding; it does not perform engineering assessment... [Rules] never own or modify the knowledge itself."* A Rule authoring its own hardcoded guidance text inline is authoring knowledge, not merely consuming a reference to it — precisely the inversion that note exists to prevent. Placing the named constructor in `modiq-knowledge` instead means the actual engineering knowledge (what the recipe says, and its identifier) is preserved independently of any Rule, reusable and revisable without touching `modiq-rules`, exactly as `KnowledgeModel.md`'s own "Independent of Runtime" and "Knowledge First" principles require.

**Repository precedent:** `VersionProfile::fs25()` is the direct precedent for part (ii): the *specific*, minimum-viable value (`GameVersion::new("FS25")`, `vec![93]`) is authored inside `modiq-versioning` itself, as a named associated function — `modiq-engine`'s `AssessmentService` merely *calls* `VersionProfile::fs25()` (`assessment_service.rs:47`); it does not construct `VersionProfile::new(GameVersion::new("FS25"), vec![93])` inline with its own literals. The original draft of this decision, by having `VersionCompatibilityRule` call `RepairRecipe::new(literal, literal)` directly, broke with this precedent rather than following it. For part (i), `RuleReference` remains the correct contrasting precedent: a Rule referencing its own identity (`RuleReference::new("version-compatibility-rule")`, hardcoded inline) is a reference to the Rule's own identity, not to reusable cross-cutting knowledge, and continues to differ in kind from Repair Recipe content.

**Alternatives considered:**
- **(a) `RuleEngine::evaluate` gains a new parameter** (a `RepairRecipe`, or a Knowledge Base handle), mirroring `VersionProfile`'s external-supply pattern.
- **(b) `AssessmentService` constructs and threads a `RepairRecipe`** the same way it constructs `VersionProfile::fs25()`.
- **(c) `VersionCompatibilityRule` authors the recipe's content directly** via `RepairRecipe::new(literal, literal)` inside its own module (the original draft of this decision).

**Why rejected:** (a) and (b) require a public signature change to `RuleEngine::evaluate` and/or new construction logic inside `AssessmentService`, for content consumed by exactly one Rule and invariant across every Assessment execution — the opposite of `VersionProfile`'s actual justification for external supply, and speculative extensibility contrary to this session's own Architectural Principles and the Capability Prioritization Study's own warning against standalone Knowledge Base infrastructure with no forcing function (`POST_SPRINT8_CAPABILITY_PRIORITIZATION_STUDY.md`, Section 3.6). (c) is rejected on the Chief Architect's own stated ground: it treats Sprint 9's minimum-viable retrieval mechanism as if it were the long-term ownership model, making `modiq-rules` the author of engineering knowledge rather than its consumer — a boundary violation `modiq-knowledge`'s own README already warns against, and one that would need correcting the moment a second Rule or a second recipe made the duplication visible. The corrected design (named constructor in `modiq-knowledge`) achieves everything (a) and (b) were meant to protect against building prematurely — no new parameter, no new public surface above `modiq-rules`, `modiq-engine` untouched — while keeping authorship where `KnowledgeModel.md` and `modiq-knowledge`'s own README already say it belongs.

---

## Question 3 / Consideration 2 — What minimum-viable fields does `RepairRecipe` need?

**Decision:** `RepairRecipe` gains exactly two fields: `identifier: String` and `guidance: String`, with an infallible constructor `RepairRecipe::new(identifier: impl Into<String>, guidance: impl Into<String>) -> Self` and accessors `identifier(&self) -> &str` / `guidance(&self) -> &str`.

**Rationale:** These are the minimum fields the capability's two actual consumers require: `guidance` supplies `Recommendation::new`'s existing `action` parameter (already a single `String`); `identifier` supplies `RepairRecipeReference::new`'s existing single `String` parameter. Nothing beyond what these two already-existing constructors need is introduced. `KnowledgeModel.md`'s own Repair Recipe definition ("a structured method for resolving a specific engineering issue... describing corrective actions") specifies no field list — this is deliberately the smallest concrete realization of that definition, not an exhaustive one.

**Repository precedent:** `VersionProfile::fs25()` (Sprint 8) is the direct precedent for minimum-viable field selection: `VersionProfile` carries exactly `game_version` and `supported_desc_versions` — the two fields its one real comparison (`supports()`) actually needs — not a richer model of every conceivable version attribute. `RepairRecipe`'s two fields follow the identical discipline, scaled to its own two real consumers. Constructor fallibility also follows existing precedent: Runtime Domain aggregate-owned entities with enforced invariants (`Evidence`, `Finding`, `Recommendation`) use `Result`-returning constructors; independently-existing domain content types with no INV-numbered invariant to enforce (`VersionProfile::new`, `GameVersion::new`) use plain infallible constructors. `RepairRecipe` is Knowledge Domain content, not a Runtime aggregate member, and follows the latter precedent.

**Alternatives considered:**
- **(a) A richer structure** separating cause, corrective steps, and rationale into distinct fields.
- **(b) A bare string wrapper** with no separate identifier field, using the guidance text itself as the reference identifier.

**Why rejected:** (a) is speculative — no current consumer needs separated fields (`Recommendation::action` takes one `String`; a Rule would have to immediately re-join split fields to use them), and prematurely commits a shape for `KnowledgeModel.md`'s other six knowledge categories (`Rule`, `Engine Behavior`, `Compatibility Pattern`, `Best Practice`, `Known Issue`, `Knowledge Reference`), none of which are in scope this Sprint and none of which should be designed by inference from this one type. (b) breaks the Opaque Reference precedent's own shape: every existing reference (`VersionProfileReference::new(profile.game_version().name())`) is constructed from a real, independent accessor on the referenced type, distinct from that type's own descriptive content — collapsing identifier and guidance into a single field would make "opaque identifier, resolved separately from display content" meaningless for this third instance.

---

## Question 4 / Consideration 4 — Does `GOVERNANCE.md`'s Knowledge Domain boundary need amendment?

**Decision:** No amendment needed.

**Rationale:** `GOVERNANCE.md`'s existing Knowledge Domain section already states, verbatim: *"Owns: reusable engineering knowledge, Rule definitions, Repair Recipes, Best Practices, Compatibility Patterns, Known Issues... Must remain independent from any individual Assessment."* This Sprint's design — `RepairRecipe` as fixed, hardcoded knowledge content that never stores or references any Assessment-specific state — is fully consistent with that existing text, confirmed by direct re-reading this session.

**Repository precedent:** Contrasts directly with Sprint 8's own finding: `GOVERNANCE.md` had **no** section at all for Version Profiles / `modiq-versioning`, a genuine, previously-undocumented gap requiring a new Crate Boundary Rule. No equivalent gap exists here — Repair Recipes are already explicitly named in an existing, adequate boundary section.

**Alternatives considered:** Add a `modiq-knowledge`-specific Crate Boundary Rule subsection, mirroring Sprint 8's `modiq-versioning` amendment.

**Why rejected:** Sprint 8's amendment addressed a real absence; here, adequate text already exists and is not stale relative to what this Sprint implements. This session is also explicitly instructed not to modify repository documentation — since no gap was found, none is proposed for a future session to act on either.

---

## Question 5 / Consideration 5 — Should `RepairRecipeReference`'s shape change?

**Decision:** No change. `RepairRecipeReference` remains an unmodified, single-field opaque `String` wrapper.

**Rationale:** Its existing constructor already accepts any `String` identifier; `RepairRecipe::identifier()` (Question 3) supplies exactly that, with no adaptation required. ADR-0007's Opaque Runtime References pattern is satisfied unchanged — this is its third proven instance (`RuleReference`, `VersionProfileReference`, now `RepairRecipeReference`), each requiring zero modification to the pattern itself.

**Repository precedent:** `RepairRecipeReference` (`crates/modiq-runtime/src/assessment/repair_recipe_reference.rs`) has been real, tested, and structurally unchanged since Sprint 2 — its doc comment already anticipates this exact moment: *"Once `modiq-knowledge` is integrated, this reference is the seam through which a real Knowledge-sourced identifier will flow."*

**Alternatives considered:** Extend `RepairRecipeReference` to carry additional content (e.g., a cached copy of guidance text), to avoid a future lookup.

**Why rejected:** Directly contradicts ADR-0007's own principle — Runtime stores a reference only; it does not own or duplicate Knowledge Domain content. Also unnecessary on its own terms: the Rule already places `guidance` text directly into `Recommendation::action` at construction time, so no downstream consumer needs the reference itself to carry a cached copy of anything.

---

# 4. Crate Boundary Decisions

| Crate | Boundary Impact | Decision | Rationale |
|---|---|---|---|
| `modiq-knowledge` | Gains real content for `RepairRecipe` within its existing module (`knowledge::repair_recipe`), including the specific, authored `version_compatibility_declared_version_mismatch()` recipe | No structural change — content added in place, existing file, existing `pub use` chain (`knowledge/mod.rs`, `lib.rs`) unchanged | `RepairRecipe` already exists as a named module; Sprint 9 gives it real fields plus one named, authored recipe, not a new location. `Cargo.toml`'s `[dependencies]` remains empty — `RepairRecipe` needs nothing beyond the standard library. This is also where the actual Repair Recipe *content* is authored (Question 2, part ii) — `modiq-knowledge` remains the sole owner of engineering knowledge, satisfying its own README's boundary. |
| `modiq-rules` | Gains a new dependency (`modiq-knowledge`) and internal logic inside `VersionCompatibilityRule` that *calls* (does not author) a `RepairRecipe` | New `Cargo.toml` edge added; `evaluate()`'s public signature unchanged | Mirrors Sprint 8's own `modiq-rules` → `modiq-versioning` edge exactly — the same crate gaining a second new dependency for the same reason (a Rule needs a domain type it does not itself own). `modiq-rules` consumes knowledge; it does not author it (Question 2). |
| `modiq-engine`, `modiq-runtime`, `modiq-collection`, `modiq-report`, `modiq-cli`, Sandbox | None | Unchanged | Decision 2 (Section 3) keeps `RepairRecipe` construction entirely inside `modiq-rules`; no other crate has any reason to reference `modiq-knowledge` this Sprint. |

`modiq-knowledge`'s own README (`crates/modiq-knowledge/README.md`, Dependencies section) states it "must not depend on... modiq-runtime, modiq-rules, modiq-engine, modiq-report, modiq-cli" — a one-directional constraint. `modiq-rules` depending *on* `modiq-knowledge` does not violate this; it is the same direction Sprint 8 already established for `modiq-versioning`, and `CrateRoadmap.md`'s dependency hierarchy diagram places `modiq-knowledge` and `modiq-rules` as siblings beneath `modiq-common` — a sibling-to-sibling edge, not an upward one. No crate boundary is crossed that this repository's own hierarchy rule ("No crate should depend on a crate above it") prohibits.

---

# 5. Dependency Decisions

**New edge: `modiq-rules` → `modiq-knowledge`.**
- **Decision:** Add `modiq-knowledge = { path = "../modiq-knowledge" }` to `crates/modiq-rules/Cargo.toml`'s `[dependencies]`.
- **Rationale:** `VersionCompatibilityRule` needs the real `RepairRecipe` type to construct one.
- **Repository precedent:** `crates/modiq-rules/Cargo.toml` already gained `modiq-versioning` as a second dependency in exactly this way for Sprint 8's own `VersionCompatibilityRule` introduction. This is the same crate's third dependency addition, same shape.
- **Alternatives considered:** Route through `modiq-engine` instead (Decision 2's rejected alternatives, Section 3).
- **Why rejected:** Already covered under Question 2 — would require `modiq-engine` to also depend on `modiq-knowledge` and would change `RuleEngine::evaluate`'s signature, a strictly larger footprint for no additional capability.

**No new edge: `modiq-engine` → `modiq-knowledge`.** Confirmed by Decision 2 — `RepairRecipe` never leaves `modiq-rules`; only the already-existing, already-`modiq-runtime`-owned `RepairRecipeReference` crosses into `modiq-engine`'s view, via `Recommendation`, exactly as it does today (currently always `None`).

**No new external dependency.** `RepairRecipe` is a plain data type; no crate requires a new external crate for this capability.

**Documentation note (informational only, not actioned here):** `CrateRoadmap.md`'s dependency diagram does not yet show `modiq-rules` → `modiq-versioning` either (per that document's own Section admission, Sprint 8) — updating the diagram to add the new `modiq-rules` → `modiq-knowledge` edge is expected Repository Closeout work for a future session, consistent with how the Sprint 8 edges were recorded retroactively at that Sprint's own closeout, not during Architectural Resolution.

---

# 6. Data Flow

```text
1. VersionCompatibilityRule::evaluate(evidence, version_profile)
       — unchanged: filters XmlInspection Evidence for an unsupported
         declared descVersion, exactly as today.

2. If at least one unsupported declared version is found:
       let recipe = RepairRecipe::version_compatibility_declared_version_mismatch();
       — a call into modiq-knowledge's own named, authored constructor.
         version_compatibility_rule.rs contains no literal identifier
         or guidance text of its own; it only calls this function.

3. Finding::new(...) — unchanged in every respect.

4. Recommendation::new(
       recipe.guidance(),                                   // was: fixed literal
       vec![finding.id()],
       Some(RepairRecipeReference::new(recipe.identifier()))  // was: None
   )

5. RuleOutcome { finding, recommendation } — unchanged shape,
   flows into RuleEngine::evaluate's existing Vec<RuleOutcome>.

6. AssessmentService::execute — unchanged: adds the Finding and
   Recommendation to the Assessment exactly as today.

7. AssessmentReport — unchanged: recommendations() exposes the
   Recommendation; its repair_recipe_reference() now resolves to
   Some(...) for this Rule's outcomes specifically.
```

No Knowledge Base lookup occurs at any later pipeline stage. `RepairRecipeReference` is not re-resolved back into `RepairRecipe` content anywhere downstream this Sprint — mirroring `VersionProfileReference`, which is likewise never resolved back into a `VersionProfile` after Assessment completion (`DataModel.md`'s Opaque Reference discipline: Runtime records identity, not content, and does not require a resolution mechanism to exist yet). Every other Rule's `RuleOutcome` (`EvidencePresenceRule`, `StructuralDuplicationRule`) is entirely unaffected — their `Recommendation`s continue to pass `None`, exactly as today.

---

# 7. Type Design Decisions

**`RepairRecipe`** (`crates/modiq-knowledge/src/knowledge/repair_recipe.rs`):

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepairRecipe {
    identifier: String,
    guidance: String,
}

impl RepairRecipe {
    /// General-purpose, infallible constructor. Used internally by this
    /// module's own named recipes (below) and available for tests and
    /// future recipes — not intended to be called from outside
    /// `modiq-knowledge` with hand-authored content (Question 2).
    pub fn new(identifier: impl Into<String>, guidance: impl Into<String>) -> Self {
        Self { identifier: identifier.into(), guidance: guidance.into() }
    }

    pub fn identifier(&self) -> &str { &self.identifier }
    pub fn guidance(&self) -> &str { &self.guidance }

    /// The platform's minimum viable Repair Recipe for a declared
    /// `descVersion` the active Version Profile does not recognize
    /// (Sprint 9). Authored here, in `modiq-knowledge` — not in
    /// `modiq-rules` — so that `VersionCompatibilityRule` consumes
    /// this content rather than authoring it (mirroring
    /// `VersionProfile::fs25()`'s identical shape in `modiq-versioning`).
    pub fn version_compatibility_declared_version_mismatch() -> Self {
        Self::new(
            "version-compatibility-declared-version-mismatch",
            "Verify the mod's declared descVersion against a supported Farming Simulator \
             release before relying on it, or confirm compatibility manually.",
        )
    }
}
```

- **Decision:** Plain data type, infallible constructor, two `String` fields, `Debug`/`Clone`/`PartialEq`/`Eq` derived — plus exactly one named, authored associated function for Sprint 9's own minimum-viable recipe.
- **Rationale:** Matches `VersionProfile`'s own derive set exactly (`#[derive(Debug, Clone, PartialEq, Eq)]`), enabling the same equality-based test assertions this repository uses throughout (e.g., `assert_eq!` against constructed values). The named function mirrors `VersionProfile::fs25()` precisely: a single, minimum-viable, hardcoded value, owned by the domain crate itself, with no selection mechanism yet (Section 11).
- **Repository precedent:** `VersionProfile`, `GameVersion` (both `modiq-versioning`) — identical derive set, identical "plain data, infallible constructor plus one named minimum-viable value" shape (`VersionProfile::new` alongside `VersionProfile::fs25()`).
- **Alternatives considered:** (a) Deriving `Copy` (both fields are owned `String`, so `Copy` is not available without a structural change to `&str`/lifetime-based fields). (b) Authoring the recipe's content in `modiq-rules` via `RepairRecipe::new(literal, literal)` directly (the original draft; see Question 2).
- **Why rejected:** (a) Introducing borrowed fields to enable `Copy` would tie `RepairRecipe`'s lifetime to its constructing scope for no benefit — every other Knowledge- and Versioning-domain-adjacent type in this repository already accepts owned `String` fields without `Copy` (`VersionProfile`, `GameVersion`). (b) makes `modiq-rules` the author of engineering knowledge rather than its consumer — the exact inversion Question 2's Chief Architect clarification identifies and this revision corrects.

**Module placement:** Real content is added directly into the existing `repair_recipe.rs` file and existing `pub use repair_recipe::RepairRecipe;` chain (`knowledge/mod.rs:14`, re-exported from `lib.rs`'s `pub mod knowledge;`). No new file, no new module path.

**No new type is introduced anywhere else.** `RepairRecipeReference` (Question 5) and `Recommendation` (unchanged since Sprint 2) are reused exactly as they exist today.

---

# 8. Public API Impact

| Crate | Public surface change | Breaking? |
|---|---|---|
| `modiq-knowledge` | `RepairRecipe` gains a real constructor and two accessors, in place of a bare unit struct with no accessible content | No — a unit struct exposes no constructible content today; nothing that compiled against the old shape can exist to break |
| `modiq-rules` | None. `VersionCompatibilityRule::evaluate`'s signature is unchanged; only its internal return *values* change (a `Some(...)` where a `None` was returned before, and different `action` text) | No — callers pattern-match on `Option<RepairRecipeReference>` and `&str` already; no type changes |
| `modiq-runtime` | None | N/A |
| `modiq-engine` | None — `AssessmentService`'s two public entry points (`execute`, `execute_from_assessment_input`) are untouched | N/A |
| `modiq-cli`, Sandbox | None | N/A |

This is the direct consequence of Decision 2 (Section 3): keeping `RepairRecipe` construction inside `modiq-rules` alone means no public API above that crate is touched at all — the smallest possible public-surface impact of any capability shipped since Sprint 3, smaller even than Sprint 8's own (which changed `AssessmentService`'s internal construction logic, though not its signatures).

---

# 9. Testing Strategy

- **`modiq-knowledge`** (currently 0 tests): new unit tests for `RepairRecipe` — construction round-trip (`new` preserves both fields, mirroring `VersionProfileReference`'s own `new_preserves_the_identifier` test shape), and an equality/inequality check mirroring `VersionProfile`'s own test pattern. This crate's first tests since Sprint 0.
- **`modiq-rules`** (`version_compatibility_rule.rs`): the existing test `produces_a_warning_finding_for_an_unrecognized_declared_version` currently asserts `outcome.recommendation.repair_recipe_reference() == None` — this assertion becomes incorrect under the new behavior and must be updated to assert `Some(...)` with the expected identifier, not left in place. The existing `is_deterministic_for_identical_input` test gains an assertion that `repair_recipe_reference()` is equal across repeated calls, mirroring `structural_duplication_rule.rs`'s own determinism test, which already checks this (currently trivially, since it is always `None` there).
- **`modiq-engine`** (`assessment_service.rs`): the existing end-to-end test `execute_from_assessment_input_produces_a_warning_for_an_unrecognized_declared_version` gains an assertion that the resulting `Recommendation`'s `repair_recipe_reference()` is `Some(...)`, exercising the capability through the real, full pipeline — mirroring this project's standing practice (Sprint 8) of validating every new capability both through direct unit tests and through the real engine pipeline, not unit tests alone.
- **Sandbox:** no test change expected — the Sandbox exercises `AssessmentService`'s unchanged public entry points; nothing in its own source references `RepairRecipeReference`'s content directly.
- **Formatting/build/test gate:** `cargo fmt`, `cargo check --workspace`, `cargo test --workspace` (and the Sandbox's own `cargo test`), per this project's standing per-task requirement, run at Implementation, not this session.

---

# 10. Implementation Plan

*(Presented as the resolved shape implementation should take; not itself an authorization to begin.)*

**Phase 1 — `modiq-knowledge`:** Give `RepairRecipe` real content (Section 7's shape) — the general `new` constructor, accessors, and the one named, authored `version_compatibility_declared_version_mismatch()` recipe — plus its own first unit tests. No dependency changes to this crate.

**Phase 2 — `modiq-rules`:** Add the `modiq-knowledge` path dependency to `Cargo.toml`. Update `VersionCompatibilityRule` to *call* `RepairRecipe::version_compatibility_declared_version_mismatch()` (no inline identifier/guidance literals of its own) and wire `recipe.guidance()` / `Some(RepairRecipeReference::new(recipe.identifier()))` into its `Recommendation::new` call. Update the existing test asserting `None` (now incorrect) and extend the determinism test (Section 9).

**Phase 3 — `modiq-engine`:** No production code change expected. Extend the existing real-pipeline integration test in `assessment_service.rs` to assert the new `Some(...)` flows through end to end (Section 9).

**Phase 4 — Repository validation:** `cargo fmt`, `cargo check --workspace`, `cargo test --workspace`, and the Sandbox's own test suite, confirming zero regression across all nine crates before any Repository Closeout activity (a later session's responsibility, not this one).

Each phase is independently small and independently verifiable, mirroring this project's standing practice of phased implementation with validation gates between phases (`ENGINEERING_LOG.md`'s own "Engineering Methodology Observations," recorded at Sprint 8 Closeout).

---

# 11. Risks and Tradeoffs

- **Risk — one named recipe per Rule may need revisiting.** If `VersionCompatibilityRule` ever needs different guidance for different kinds of declared-version mismatch, or if a second Rule needs its own Repair Recipe, `modiq-knowledge` will need more than one named associated function. **Accepted** as appropriate for this Sprint's minimum-viable scope — the same "one real value, no selection mechanism yet" acceptance Sprint 8 made for `VersionProfile::fs25()`, and revisited only once real evidence of a second consumer exists, per this project's own "capability before abstraction" discipline. Because authorship already lives in `modiq-knowledge` (Question 2, as corrected), adding a second named recipe later requires no relocation of existing content — only an additive function alongside the first, the same shape `VersionProfile::fs25()` itself would take if a second profile were ever added.
- **Tradeoff — a named-constructor-per-recipe versus a general Knowledge Base access mechanism.** `modiq-knowledge` authoring one specific, named `RepairRecipe` function is not a lookup/registry/query mechanism — it is a single hardcoded value, exactly as minimal as `VersionProfile::fs25()`. If a future Sprint needs many recipes selected dynamically (by Rule, by severity, by Version Profile), a real Knowledge Base access mechanism becomes a legitimate forcing function then. **Not built now** — explicitly named as a future consideration rather than built speculatively, per this session's own Architectural Principles and the Capability Prioritization Study's own warning against premature Knowledge Base infrastructure.
- **Risk to crate boundary integrity, dependency direction, or determinism:** none identified. The new dependency edge mirrors an already-proven precedent exactly (Sprint 8); no cyclic or upward dependency is introduced; the named recipe function is as deterministic as `VersionProfile::fs25()` itself, and `modiq-rules` no longer authors any knowledge content that could drift from `modiq-knowledge`'s own record of it.
- **Overall implementation risk: Low.** Materially lower than Sprint 8's own "Low–Medium" rating for its comparable activation, because this capability touches no public API above `modiq-rules` and requires no new parameter threading anywhere in the pipeline.

---

# 12. Chief Architect Approval Summary

| # | Question | Resolution |
|---|---|---|
| 1 | Which Rule is the first consumer? | `VersionCompatibilityRule`, sole consumer this Sprint |
| 2 | Where is `RepairRecipe` constructed/supplied? | **Retrieved** by the Rule via a direct call — no new parameter on `RuleEngine::evaluate` or `AssessmentService`. **Authored** in `modiq-knowledge`, as a named associated function (`RepairRecipe::version_compatibility_declared_version_mismatch()`), not in `modiq-rules` — revised following Chief Architect clarification |
| 3 | Minimum-viable `RepairRecipe` field shape? | Two fields: `identifier: String`, `guidance: String`; infallible constructor, mirroring `VersionProfile` |
| 4 | Does `GOVERNANCE.md`'s Knowledge Domain boundary need amendment? | No — existing text already covers this exactly |
| 5 | Should `RepairRecipeReference`'s shape change? | No — reused unchanged, its third proven instance of ADR-0007's pattern |

**Smallest architecture confirmed:** one new dependency edge (`modiq-rules` → `modiq-knowledge`), zero new public API surface above `modiq-rules`, zero change to `RuleEngine::evaluate` or `AssessmentService`'s public entry points, zero new crate, zero new external dependency.

**No ADR produced.** Every decision above activates an already-established pattern (ADR-0007's Opaque Runtime References, now in its third instance; the "Rule owns its own hardcoded identity" shape `RuleReference` already demonstrates) within Sprint 9's own scope. None introduces a new cross-platform architectural concept extending beyond this capability — the bar this session's mission sets for producing one.

**This resolution activates dormant scaffolding; it does not design a general Knowledge System.** Full `KnowledgeModel.md` realization (six remaining knowledge categories, any Knowledge Base access mechanism, multi-Rule Repair Recipe consumption) remains explicitly out of scope, per `SPRINT9_CAPABILITY_DEFINITION.md`, Section 6, unchanged by this document.

**Long-term ownership model, explicitly preserved, not merely deferred:** the Chief Architect's clarification is incorporated as a correction to Question 2, not recorded as an unresolved future concern. Sprint 9's implementation authors zero engineering knowledge inside `modiq-rules` — `modiq-knowledge` is the sole author of Repair Recipe content, today and going forward, with Rules only ever consuming a reference to it. Any future expansion of `modiq-knowledge` (additional recipes, additional knowledge categories) should extend this same shape: named, authored content living in the knowledge domain, called — never constructed from inline literals — by the Rules that consume it.

---

Awaiting Chief Architect approval before Implementation Authorization. No implementation, documentation change, governance item, or ADR has been made this session.

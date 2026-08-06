# C2 Implementation Plan — Declared Dependency Interpretation

| Property | Value |
|---|---|
| **Document** | C2_IMPLEMENTATION_PLAN_DECLARED_DEPENDENCY_INTERPRETATION.md |
| **Project** | modIQ |
| **Origin** | `C2_IMPLEMENTATION_AUTHORIZATION_DECLARED_DEPENDENCY_INTERPRETATION.md`, treated as fixed, unreopened. `C2_ARCHITECTURAL_RESOLUTION_DECLARED_DEPENDENCY_INTERPRETATION.md`, `C2_ARCHITECTURE_EVALUATION_DECLARED_DEPENDENCY_INTERPRETATION.md` (committed `2cf8ba8`), and `CAPABILITY_DEFINITION_C2_DECLARED_DEPENDENCY_INTERPRETATION.md` (committed `7c7faf8`) are fixed inputs, not restated. `C1_IMPLEMENTATION_PLAN.md` supplies this document's own structural precedent, as the most recent approved Implementation Plan in this repository. **As of this Plan's own drafting, confirmed via `git status`: neither the Architectural Resolution nor the Implementation Authorization has been committed.** This Plan inherits the Authorization's own conditioning (its Status: "Approved, conditioned on the Resolution entering repository history as a committed artifact") without restating or resolving it further; the same condition applies here. |
| **Status** | Implementation Planning draft. No source file has been modified in preparing this document. |

---

## 1. Implementation Objective

Implement exactly what `C2_IMPLEMENTATION_AUTHORIZATION_DECLARED_DEPENDENCY_INTERPRETATION.md` §3 authorizes: introduce one new Rule in `modiq-rules` that interprets already-collected `EvidenceCategory::XmlInspection` Evidence bearing the `"Declared Dependency"` label, and dispatch it from `RuleEngine::evaluate` as a fifth, fixed-order Rule. No `modiq-runtime`, `modiq-collection`, `modiq-engine`, `modiq-report`, `modiq-storage`, `modiq-cli`, `apps/console`, or `apps/sandbox` change. No `modiq-knowledge` change (this Plan does not introduce a `RepairRecipe`, per §2 below). Success is defined by the Authorization's own Completion Criteria (§9): a Finding exists whose title, summary, and evidence references are specifically about a mod's declared-dependency content, distinguishable from `EvidencePresenceRule`'s undifferentiated Finding, produced without touching `XmlCollector`, `EvidenceCategory`, or any consumer's transport code.

---

## 2. Implementation Scope

### 2.1 The reserved design question, decided here

`C2_IMPLEMENTATION_AUTHORIZATION_DECLARED_DEPENDENCY_INTERPRETATION.md` §5 left the new Rule's own judgment, matching logic, severity, and Mod Health Dimension unfixed. No document in this capability's own lineage — Capability Definition, Architecture Evaluation, or Architectural Resolution — decided what specific question the new Rule asks; each deliberately reserved it. This Plan decides it now, since Implementation requires a concrete target.

**Decision: the new Rule detects a declared dependency name repeated more than once within one manifest.**

Reasoning, checked against what is actually available:

- **This is the only single-mod-scoped, evidence-groundable judgment currently reachable.** A "missing dependency" judgment requires knowing whether a declared dependency is satisfied elsewhere — cross-mod evidence this capability's own foundation does not have (`CAPABILITY_DEFINITION_C2...md` §Explicit Exclusions; `AssessmentSubject` remains a zero-field unit struct). A "malformed name" judgment has no Evidence to act on — `XmlCollector` already silently drops an empty or whitespace-only `<dependency>` element at Collection time (`xml_collector.rs`: `if name.is_empty() { continue; }`), confirmed unchanged by this Plan (`modiq-collection` is Not Participating). A "self-referential dependency" judgment has no Evidence to compare against — nothing records the assessed mod's own declared name. Duplicate-name detection requires only the Evidence `XmlCollector` already produces today, confirmed directly (this session, and independently during this capability's own Architecture Evaluation) to be undeduplicated: two `<dependency>` elements with an identical name produce two separate, distinct Evidence items.
- **This is the only judgment this capability's own lineage ever named as an available pattern.** `CAPABILITY_DEFINITION_C2...md`'s own "Existing Repository Patterns" section named `StructuralDuplicationRule`'s repetition-within-a-collected-set judgment as the platform's one existing precedent of this shape, without adopting it. This Plan is where that precedent is finally exercised, not invented fresh.
- **Satisfies the Rule Conclusion Non-Contradiction Constraint (Architectural Resolution, Decision 2) by direct textual inspection, not by argument.** The new Rule's own content-shape filter (`"modDesc.xml declares dependency: "`, below) and `VersionCompatibilityRule`'s own (`"modDesc.xml declares descVersion: "`) are two different fixed strings; no Evidence item can match both. This is verified directly in Phase 2 (§5), not merely asserted here.
- **`StructuralDuplicationRule` is the closest implementation analogue for this judgment's own shape — repetition within a declared/collected set as itself the fact worth a Finding — not a directly transferable severity precedent.** `StructuralDuplicationRule` assigns `FindingSeverity::Warning` for a specific, mechanical reason: duplicate archive entries create a genuine functional ambiguity ("which physical entry a reader actually extracts is not guaranteed by the archive format itself"). No repository evidence reviewed establishes an analogous functional consequence for a duplicated declared-dependency name — nothing collected by `XmlCollector`, and nothing elsewhere in this capability's own lineage, states whether Farming Simulator's own dependency handling treats a repeated declaration as functionally significant, merely redundant, or otherwise. `DataModel.md`'s own Finding Severity discipline is acknowledged directly here, not assumed satisfied: "a Rule must never assign a severity stronger than what its Evidence conclusively establishes" — and the Evidence available establishes only that a declared name appears more than once in text, not any functional consequence of that repetition. `FindingSeverity::Warning` and `ModHealthDimension::Structure` are this Plan's own current implementation proposal — made by analogy to `StructuralDuplicationRule`'s judgment shape and consistent with `DataModel.md`'s own discipline, but not a severity or dimension repository evidence uniquely establishes. `ModHealthDimension::Compatibility` (matching `VersionCompatibilityRule`, since declared dependencies concern other mods) was also considered; `Structure` is proposed instead because the fact being judged is a defect in the manifest's own declaration structure (a name listed twice), not a compatibility relationship with another mod's actual content — the same reasoning as before, now stated as this Plan's own proposal, subject to confirmation during implementation, rather than a conclusion the evidence alone compels.
- **A genuinely new mechanism, named as such.** No existing Rule compares multiple Evidence items against each other — every existing Rule's `evaluate` filters or maps independently, item by item; only `ArchiveCollector`, at the Collection layer, performs cross-item comparison (`Architecture Evaluation` §6, Reading B; independently re-confirmed this session). This Rule is the first to do so at the Rule layer. This is squarely Rule-authorship discretion, not a new architectural mechanism — no registry, no coordinator, no dispatch-layer change — but it is new *in kind* for this specific crate, and is named here rather than left for a reviewer to discover.

### 2.2 Rule design

`crates/modiq-rules/src/rules/declared_dependency_duplication_rule.rs` (new file) — described here at the repository-design level, not as compile-ready Rust. The exact data structures used to detect repeated names (for example, whether occurrence-counting borrows `&str` from the Evidence slice directly or normalizes to an owned `String`, mirroring `ArchiveCollector::detect_duplicate_entry_names`'s own choice) are an implementation-time decision, to be made and verified by `cargo check`/`cargo test` during Phase 1 itself — a planning document that touches no source file cannot verify a borrow-checker outcome, and does not attempt to here.

**Constant.** A `DECLARED_DEPENDENCY_PREFIX` constant, value `"modDesc.xml declares dependency: "` — the exact string `XmlCollector` already uses, duplicated here deliberately rather than shared, mirroring `VersionCompatibilityRule`'s own `DECLARED_DESC_VERSION_PREFIX` precedent.

**Type.** A unit struct, `DeclaredDependencyDuplicationRule`, exposing one method: `evaluate(&self, evidence: &[Evidence]) -> Option<RuleOutcome>` — the same signature shape every existing category-specific Rule that does not need `version_profile` already uses.

**Algorithm, stated as required steps, not fixed code:**

1. Filter `evidence` to items whose `category()` is `EvidenceCategory::XmlInspection` and whose `description()` matches `DECLARED_DEPENDENCY_PREFIX`, extracting each matching item's own `EvidenceId` and its declared name (the text following the prefix).
2. Determine which declared names occur more than once among the filtered items, in a way that does not depend on any non-deterministic iteration order. The resulting Finding's own `evidence_ids` and summary text must be stable across repeated evaluations of identical input — this repository's own Determinism-by-Content-and-Order principle (`PROJECT_HANDOFF_v1.1.md` §6), the same requirement every existing Rule's own `is_deterministic_for_identical_input` test already enforces. This is a requirement on the implementation, not a specific data structure fixed by this Plan.
3. If no declared name occurs more than once, return `None` — mirroring every existing Rule's own precedent of returning `None` for non-matching input.
4. If one or more names occur more than once, construct exactly one `Finding` — mirroring `StructuralDuplicationRule`'s own "one Finding, referencing every matching item" shape, not one Finding per duplicated name — whose `evidence_ids` include every Evidence item whose name recurs (not only the second and later occurrences), with:
   - `title`: a short, scannable label naming the duplication (e.g., `"Duplicate declared dependency"`).
   - `summary`: names which declared dependency name(s) recur, authored freely, matching every existing Rule's own free-text summary convention.
   - `severity`: `FindingSeverity::Warning` — this Plan's own current implementation proposal, qualified in §2.1, not treated as evidence-compelled.
   - `mod_health_dimension`: `ModHealthDimension::Structure` — likewise a current proposal, per §2.1.
   - `status`: `FindingStatus::Final`.
   - `rule_reference`: `RuleReference::new("declared-dependency-duplication-rule")`.
5. Pair the Finding with an inline-authored `Recommendation` — no `RepairRecipe` (`repair_recipe_reference: None`), mirroring `StructuralDuplicationRule`'s and `RuntimeLoadFailureRule`'s own precedent, not `VersionCompatibilityRule`'s (which cites a `modiq-knowledge` recipe) — advising that the duplicate `<dependency>` declaration be removed so each required mod is listed once.

**No `RepairRecipe` is introduced.** This keeps `modiq-knowledge` Not Participating, exactly as `C2_IMPLEMENTATION_AUTHORIZATION_DECLARED_DEPENDENCY_INTERPRETATION.md` §4 conditioned it — its participation was authorized only if Planning determined a `RepairRecipe` was warranted; this Plan determines it is not, since no existing recipe fits and authoring a new one is not required by any Completion Criterion.

`crates/modiq-rules/src/rules/mod.rs` (existing file, additive change only):

```rust
pub mod declared_dependency_duplication_rule;
```

added alongside the four existing `pub mod` lines, and

```rust
pub use declared_dependency_duplication_rule::DeclaredDependencyDuplicationRule;
```

added alongside the four existing `pub use` lines — confirmed directly against current source (`mod.rs`, re-read this session) to be the exact, uniform pattern all four existing Rules already follow; no other change to this file.

`crates/modiq-rules/src/rules/engine.rs` (existing file, additive change only): one new `use` entry (`use super::declared_dependency_duplication_rule::DeclaredDependencyDuplicationRule;`), one new dispatch line —

```rust
if let Some(outcome) = DeclaredDependencyDuplicationRule.evaluate(evidence) {
    outcomes.push(outcome);
}
```

— appended after the existing `RuntimeLoadFailureRule` line (preserving every existing Rule's own position, per GOV-012 Question 2: new Rules are appended, never reordering existing declaration order), and the module's own doc comment updated to name the fifth Rule in its own dispatch-order description. No other change.

---

## 3. Repository Boundaries

Restated from the Authorization (§6, §7) as concrete implementation constraints, not re-derived:

- `XmlCollector`, `EvidenceCategory`, `AssessmentService`, and every consumer (`modiq-report`, `modiq-storage`, `modiq-cli`, `apps/console`, `apps/sandbox`) remain untouched by every phase of this Plan.
- `RuleEngine::evaluate`'s own dispatch shape — a fixed-order sequence of `if let` statements — gains exactly one line and no structural change.
- No trait, registry, factory, or dispatch table is introduced anywhere in this Plan.
- `modiq-knowledge` remains untouched (§2.2).
- No field, method, or variant is added to `Evidence`, `Finding`, `Recommendation`, or `EvidenceCategory`.

---

## 4. Crate / File Impact

| File | Required change | Authorization basis |
|---|---|---|
| `crates/modiq-rules/src/rules/declared_dependency_duplication_rule.rs` | **New file.** `DeclaredDependencyDuplicationRule` and its `evaluate` method (§2.2), plus unit tests (§6). | Authorization §3, §4 (Participating: `modiq-rules`). |
| `crates/modiq-rules/src/rules/mod.rs` | Two additive lines: `pub mod` and `pub use` for the new Rule (§2.2). | Authorization §3. |
| `crates/modiq-rules/src/rules/engine.rs` | One new `use`, one new fixed-order dispatch line, doc-comment update, new/extended tests (§2.2, §6). | Authorization §3, §6 (dispatch shape preserved). |
| `modiq-runtime`, `modiq-collection`, `modiq-versioning`, `modiq-report`, `modiq-engine`, `modiq-storage`, `modiq-cli` | **No change.** | Authorization §4: "Not participating." |
| `modiq-knowledge` | **No change.** No `RepairRecipe` is introduced (§2.2). | Authorization §4: participation conditional, not exercised. |
| `apps/console`, `apps/sandbox` | **No change** — separate workspaces, out of scope. | Authorization §4, §7. |

No file outside this table is expected to require modification. If one does, implementation should stop and report it rather than proceed — matching `C1_IMPLEMENTATION_PLAN.md`'s own precedent for this exact table.

---

## 5. Recommended Phase Breakdown

**Phase 1 — Rule Construction, Standalone and Unit-Tested.**
*Objective:* introduce `DeclaredDependencyDuplicationRule` as a complete, independently tested unit, declared in `rules/mod.rs`, but **not yet dispatched from `RuleEngine::evaluate`.** This mirrors direct historical precedent, not an invented convention: `structural_duplication_rule.rs`'s own doc comment records that `StructuralDuplicationRule` "is a complete, independently tested unit today, exactly as `ArchiveReader`/`ArchiveEvidenceBuilder` were real and tested before `AssessmentService` routing existed to reach them" — the same shape of split this Plan uses.
*Files:* `crates/modiq-rules/src/rules/declared_dependency_duplication_rule.rs` (new); `crates/modiq-rules/src/rules/mod.rs` (additive).
*Completion criteria:* `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` all clean; every new unit test (§6) passes; existing test count grows only by net-new assertions, no deletions or modifications to existing tests.

**Phase 2 — Dispatch Wiring and Non-Contradiction Demonstration.**
*Objective:* wire the new Rule into `RuleEngine::evaluate` as the fifth, fixed-order dispatch line; extend `engine.rs`'s own composition tests; **directly demonstrate, not assume, that the new Rule and `VersionCompatibilityRule` never both match the same Evidence item** — the Rule Conclusion Non-Contradiction Constraint (Architectural Resolution, Decision 2; Authorization §6, §9).
*Files:* `crates/modiq-rules/src/rules/engine.rs` (additive).
*Completion criteria:* `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` clean; the new dispatch-order test and the new non-contradiction test (§6) both pass; no existing `engine.rs` test's own assertion is altered, only extended with new cases.

**Phase 3 — Final Reverification.**
*Objective:* full-repository reverification and Repository Impact confirmation only.
*Files:* none — verification-only phase.
*Completion criteria:* root workspace (`cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace`) reconfirmed clean; `apps/sandbox/src-tauri`'s own separate workspace reconfirmed clean and unaffected (9/9 or current count, unchanged); `apps/console`'s own `npm run build` reconfirmed clean and unaffected; a direct `git diff --stat` (or equivalent) confirms the only files touched across all phases are the three named in §4 — no other file shows a diff.

### Why this ordering, and not a single phase or a different split

Nothing in this capability's own scope resembles `C1`'s own reason for a multi-language, breaking-change phase split (a Rust transport retype requiring its one TypeScript consumer's continuity fix to land atomically). This work is single-crate, single-language, and purely additive — no existing type is retyped, no existing function signature changes, no existing consumer is affected. A single, atomic phase would not be unsafe.

`StructuralDuplicationRule`'s and `RuntimeLoadFailureRule`'s own historical precedent of building and unit-testing a new Rule before wiring it into dispatch is re-checked here precisely, not merely cited by label. Both Rules' own doc comments record that dispatch wiring was, at the time, "not yet authorized" (`structural_duplication_rule.rs`) or "a separate, later milestone" (`runtime_load_failure_rule.rs`) — a governance/authorization-sequencing reason specific to those Sprints (GOV-012 itself was resolved as part of that same Sprint 5), not a risk-management strategy freely chosen for its own sake. That specific circumstance does not recur for C2: `C2_IMPLEMENTATION_AUTHORIZATION_DECLARED_DEPENDENCY_INTERPRETATION.md` §3 already authorizes the new Rule and its dispatch line together, in one document; nothing gates Phase 2 on any separate, future authorization.

The two-phase split is chosen here on C2's own implementation characteristics instead. It isolates the new Rule's own correctness (its duplicate-detection logic, exercised entirely through Phase 1's own unit tests, with no dependency on `RuleEngine::evaluate`'s own composition behavior) from `RuleEngine`'s own composition correctness (that wiring the new Rule in does not disturb any existing Rule's behavior, and that the Rule Conclusion Non-Contradiction Constraint actually holds once both Rules run together — a property no unit test internal to the new Rule's own file can demonstrate by itself). Each phase's own verification gate (§8) accordingly answers a narrower, more specific question than one combined gate would: a Phase 1 failure means the new Rule's own logic is wrong; a Phase 2 failure means the wiring, or the interaction between Rules, is wrong. This makes a verification failure, if one occurs, simpler to diagnose than it would be from a single gate covering both concerns at once. Phase 3 mirrors `C1_IMPLEMENTATION_PLAN.md`'s own Phase 4 precedent (a dedicated, zero-diff reverification step) directly.

---

## 6. Testing Strategy

**Phase 1 — new unit tests in `declared_dependency_duplication_rule.rs`, mirroring the exact conventions all four existing Rule test suites already use (no mocking; `Evidence::with_location` constructed directly, matching `structural_duplication_rule.rs`'s and `version_compatibility_rule.rs`'s own test helpers):**

- `returns_none_for_empty_evidence`
- `returns_none_when_no_declared_dependency_evidence_exists` (non-`XmlInspection` Evidence, or `XmlInspection` Evidence not matching the declared-dependency prefix)
- `returns_none_when_every_declared_name_is_distinct`
- `produces_a_warning_finding_for_a_duplicate_declared_dependency`, asserting: `FindingSeverity::Warning`; non-empty title/summary; `ModHealthDimension::Structure`; `FindingStatus::Final`; `evidence_ids` containing every occurrence of the duplicated name; `rule_reference().identifier() == "declared-dependency-duplication-rule"`; a non-empty Recommendation `action`; `repair_recipe_reference() == None`.
- `references_all_matching_items_when_more_than_one_duplicate_name_exists` (two distinct names, each duplicated) — mirroring `structural_duplication_rule.rs`'s own test of the identical name.
- `ignores_non_matching_evidence_alongside_duplicate_evidence` (a `FileStructureAnalysis` or declared-`descVersion` item present alongside the duplicate — confirms the filter is correctly scoped).
- `is_deterministic_for_identical_input` — mirroring every existing Rule's own determinism test shape exactly (content and order, not identity).

**Phase 2 — new/extended tests in `engine.rs`, mirroring its own existing dispatch-order test conventions:**

- `evaluate_dispatches_declared_dependency_duplication_rule_fifth_in_declaration_order` — mirroring the existing `evaluate_dispatches_version_compatibility_rule_third_in_declaration_order` and `..._runtime_load_failure_rule_fourth_...` tests exactly.
- `evaluate_does_not_dispatch_declared_dependency_duplication_rule_when_no_duplicate_exists`.
- **`version_compatibility_rule_and_declared_dependency_duplication_rule_never_match_the_same_evidence_item`** — constructs one `XmlInspection` Evidence item for a declared `descVersion` and one for a declared dependency (duplicated), asserts each Rule's own `evaluate` call matches only its own item, directly demonstrating the Rule Conclusion Non-Contradiction Constraint against real code, not by textual argument alone (Authorization §8, §9).
- `evaluate_dispatches_all_five_rules_independently_when_all_match` — extending the existing `evaluate_dispatches_all_four_rules_independently_when_all_match` test with a fifth matching Evidence item.

**Phase 3 — no new test authored; re-runs every existing and new assertion unmodified, alongside full-repository and Sandbox/Console reverification.**

No existing test is deleted, weakened, or has its own assertion altered at any phase.

---

## 7. Engineering Risks

1. **This is the first Rule-layer cross-item comparison in this crate.** No existing Rule's test suite provides a direct template. Mitigated by grounding the implementation in `ArchiveCollector`'s own Collection-layer precedent (`detect_duplicate_entry_names`) for the underlying comparison shape, adapted to the Rule layer, and by Phase 1's own dedicated, isolated test coverage before dispatch wiring.
2. **Non-contradiction with `VersionCompatibilityRule` must be directly demonstrated, not assumed.** Mitigated by the explicit Phase 2 test naming both Rules together (§6) — a real risk this Plan treats as a completion criterion, not a formality.
3. **Determinism of the duplicate-detection result.** §2.2's own step 2 requires the implementation not to depend on any non-deterministic iteration order (for example, a `HashMap`'s own default iteration order, if a `HashMap` is the data structure Phase 1 ultimately chooses) when constructing the Finding's own `evidence_ids` and summary text. This is stated as a requirement in §2.2, not as a fixed algorithm this Plan itself verifies — named explicitly so it is checked, not assumed, by Phase 1's own `is_deterministic_for_identical_input` test (§6).
4. **Scope creep into cross-mod resolution, `RepairRecipe` authorship, or `apps/console` presentation.** All three are explicitly excluded (Authorization §5). Nothing in this Plan requires any of them, and none should be improvised mid-implementation if the duplicate-detection judgment feels incomplete in isolation — it is deliberately scoped to exactly what single-mod Evidence supports (§2.1).
5. **GOV-013 (`FindingSeverity` Severity/Kind Conflation, Open by design) is exercised again, not newly implicated.** Assigning `Warning` here is a live instance of the same tension GOV-013 already names — not resolved by this Plan, consistent with the Architectural Resolution's own treatment of it as adjacent context, not a blocking question.

---

## 8. Verification Gates

Before progressing past each phase:

- **After Phase 1:** `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` all clean. Every Phase 1 unit test (§6) passes. No existing test is modified.
- **After Phase 2:** `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` all clean. The dispatch-order test and the non-contradiction test (§6) both pass. `engine.rs`'s own dispatch remains a fixed-order sequence of `if let` statements — confirmed by direct inspection, not assumed.
- **After Phase 3:** full root workspace and `apps/sandbox/src-tauri` both reverified clean and unaffected; `apps/console`'s `npm run build` reverified clean and unaffected; `git diff --stat` (or equivalent) confirms only the three files named in §4 were touched across the entire implementation.

No phase begins before the prior phase's gate passes in full.

---

## 9. Repository Integrity Checks

Performed at Phase 3, as part of Final Reverification, not assumed true throughout:

- Every file named "No change" in §4 shows zero diff.
- `EvidenceCategory`'s eight-variant enum is unchanged; no new or dormant category activated.
- `XmlCollector`'s own dependency-collection logic (`xml_collector.rs`) is unchanged, including its own silent-drop-of-empty-declarations behavior (§2.1).
- `RuleEngine::evaluate`'s own signature (`evidence: &[Evidence], version_profile: &VersionProfile) -> Vec<RuleOutcome>`) is unchanged.
- No new Cargo dependency edge is introduced — `declared_dependency_duplication_rule.rs` depends only on the standard library and types already imported by sibling Rule files (§2.2 leaves the exact standard-library collection type to Phase 1's own implementation).
- No Governance Register item, ADR, or `GOVERNANCE.md`/`RuleEngine.md` text is modified by this implementation (both documents' own recommended future amendments, per the Architectural Resolution, remain separate and are not performed here).

---

## 10. Implementation Completion Criteria

- A Finding exists, produced by `DeclaredDependencyDuplicationRule`, whose title, summary, and evidence references are specifically about a duplicated declared-dependency name — distinguishable from `EvidencePresenceRule`'s own undifferentiated Finding.
- A manifest with no declared dependencies, or with only distinct declared names, produces no such Finding.
- The Rule Conclusion Non-Contradiction Constraint is directly demonstrated by a passing test naming both `DeclaredDependencyDuplicationRule` and `VersionCompatibilityRule` together, not merely argued in prose.
- No `modiq-runtime`, `modiq-collection`, `modiq-versioning`, `modiq-report`, `modiq-engine`, `modiq-storage`, `modiq-cli`, `modiq-knowledge`, `apps/console`, or `apps/sandbox` file is touched.
- `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` clean at the root; `apps/sandbox/src-tauri` clean and unaffected; `apps/console`'s `npm run build` clean and unaffected; `modiq-rules`'s own test count grown only by net-new assertions, no deletions.
- No item named in `C2_IMPLEMENTATION_AUTHORIZATION_DECLARED_DEPENDENCY_INTERPRETATION.md` §5 (Explicit Exclusions) is touched, added, or implied.

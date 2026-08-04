# Sprint 24 Implementation Report — Runtime-Owned Representation of RepairRecipe-Derived Structure

| Property | Value |
|---|---|
| **Document** | SPRINT24_IMPLEMENTATION_REPORT.md |
| **Project** | modIQ |
| **Origin** | `RECOMMENDATION_REPAIR_STRUCTURE_IMPLEMENTATION_AUTHORIZATION.md` (`3f2728a`), `SPRINT24_IMPLEMENTATION_PLAN.md` (`5c88547`) — both treated as fixed, unreopened architecture and planning. The Architecture Evaluation and Architectural Resolution behind the Authorization were conducted in a prior session and are cited by title only, not by file path — confirmed directly (repository-wide search): no `RUNTIME_REPAIR_STRUCTURE_ARCHITECTURE_EVALUATION.md` or matching Architectural Resolution file exists in this repository; the Authorization document's own Origin line records them as "this session's fixed inputs, not restated." |
| **Commit** | Implementation complete on `feature/runtime-implementation`; not yet staged, committed, or pushed as of this report's own drafting (per explicit direction at every implementation and audit stage). |
| **Status** | Complete |

---

## Summary

Sprint 24 implemented exactly the four items `RECOMMENDATION_REPAIR_STRUCTURE_IMPLEMENTATION_AUTHORIZATION.md` §3 authorizes: a new Runtime-owned value representation (`RecommendationStep`/`RecommendationStepKind`) projecting `RepairRecipe`'s per-step structure; population of that representation by `VersionCompatibilityRule` at `Recommendation`-construction time; `Recommendation.action` continuing unchanged, supplemented rather than replaced; and a mirrored, one-way addition to `modiq-storage`'s `PersistedRecommendation`. Four phases, each independently gated (`modiq-runtime` → `modiq-rules` → `modiq-storage`/`modiq-report` → final reverification), all findings re-derived independently at a completed Implementation Audit before this report was drafted, not carried forward from phase reports alone.

---

## Capability Summary

No new user-facing capability. `VersionCompatibilityRule`'s Recommendation now carries the same per-step structure `RepairRecipe` already authors — kind and instruction, one entry per step — as a genuine Runtime value alongside the pre-existing flat `action` string, and that structure now survives persistence. No consumer application, transport layer, or CLI command was extended to present it; this Sprint is exclusively a Runtime/Rule/Storage capability, not a presentation one.

---

## Files and Crates Modified

Re-derived directly from `git diff --stat` / `git status`, not from phase reports:

- `modiq-runtime`
  - `crates/modiq-runtime/src/assessment/recommendation_step.rs` (**new**, 45 lines) — `RecommendationStepKind` (5-variant closed set: `XmlChange`, `LuaChange`, `DependencyInstallation`, `AssetReplacement`, `VersionUpdate`), `RecommendationStep` (`kind`, `instruction`), both value types, no identity.
  - `crates/modiq-runtime/src/assessment/mod.rs` (+2/−0) — module declaration and re-exports.
  - `crates/modiq-runtime/src/assessment/recommendation.rs` (+62/−14) — `repair_steps: Vec<RecommendationStep>` field, `repair_steps()` getter, 4th `Recommendation::new` constructor parameter; 11 existing test call sites updated in place; 1 new test.
  - `crates/modiq-runtime/src/assessment/assessment.rs` (+22/−7) — 4 existing test-fixture call sites updated for the new constructor argument. Test-only.
- `modiq-rules`
  - `crates/modiq-rules/src/rules/version_compatibility_rule.rs` (+35/−2) — real projection: exhaustive 5-arm match (no wildcard) from `modiq_knowledge::knowledge::RepairStepKind` to `RecommendationStepKind`; `repair_steps` populated from real `RepairRecipe::version_compatibility_declared_version_mismatch()` content and passed as the constructor's 4th argument; 2 existing tests extended.
  - `crates/modiq-rules/src/rules/evidence_presence_rule.rs`, `structural_duplication_rule.rs`, `runtime_load_failure_rule.rs` (+1/−0 each) — mechanical: `Vec::new()` as the 4th argument. No `RepairRecipe` adopted by any of the three.
- `modiq-storage`
  - `crates/modiq-storage/src/storage/persisted_report.rs` (+88/−2) — `PersistedRecommendationStepKind` (5-variant mirror, exhaustive `From<RecommendationStepKind>`), `PersistedRecommendationStep` (`kind`, `instruction`), both `Serialize + Deserialize`; `PersistedRecommendation` gains `repair_steps` field, `repair_steps()` getter, population inside the existing private `from_recommendation`; one-way only — no reverse `to_recommendation`, matching every other field this type already carries; 1 existing test-fixture call site updated; 1 new test.
- `modiq-report`
  - `crates/modiq-report/src/report/assessment_report.rs` (+7/−2) — one existing test-fixture call site updated for the new constructor argument. No production code path touched — confirmed directly: `AssessmentReport::generate` still only calls `.to_vec()` on the whole `recommendations` collection.

**Confirmed untouched — zero diff:** `modiq-knowledge`, `modiq-engine`, `modiq-cli`, `modiq-collection`, `modiq-versioning`, `apps/console`, `apps/sandbox`, every `Cargo.toml` and `Cargo.lock` in the workspace.

Total: 9 modified files + 1 new file, 264 insertions / 27 deletions (219/27 across the 9 tracked files per `git diff --stat`, plus the 45-line new file).

---

## Public API Changes

- `Recommendation::new` — signature changed: gains a 4th, trailing parameter `repair_steps: Vec<RecommendationStep>`. `repair_steps()` accessor added. `action()`, `finding_ids()`, `repair_recipe_reference()` — all three **unchanged** in signature and behavior, confirmed by diff (only additive lines touch this file's production code).
- New public types in `modiq-runtime`: `RecommendationStep`, `RecommendationStepKind`.
- `PersistedRecommendation` — gains `repair_steps()`. `action()`, `finding_indices()`, `repair_recipe_reference()` — unchanged.
- New public types in `modiq-storage`: `PersistedRecommendationStep`, `PersistedRecommendationStepKind`.
- `RecommendationError` — **unchanged**; no new variant added (confirmed: `recommendation_error.rs` has zero diff).
- No change to any `modiq-knowledge` public type, `AssessmentService`'s entry points, `RuleEngine::evaluate`'s signature, or any transport/IPC boundary.

---

## Repository Impact

Crates touched: `modiq-runtime`, `modiq-rules`, `modiq-storage`, `modiq-report` (test fixture only). No new crate. No new external dependency — confirmed by empty diff on every `Cargo.toml`/`Cargo.lock` in the workspace. No Crate Boundary Rule modified.

Tests, re-verified fresh this session: root workspace (default-members) 269 → **271** (`modiq-runtime` +1, `modiq-storage` +1; every other touched crate's count unchanged, updated in place). `console` unchanged at 5/5. Full workspace (`--workspace`, 10 members) 274 → **276**. Sandbox (`apps/sandbox/src-tauri`, separate workspace) unchanged at 9/9. Zero tests deleted anywhere — confirmed by a diff-wide grep for removed `#[test]`/`fn` lines, returning none.

---

## Specification References

- `docs/adrs/0007-runtime-entity-design-pattern.md` (Opaque Runtime References, Constructor Validation, Governance-Controlled Invariants)
- `docs/engineering/GOVERNANCE.md` (Runtime Domain Crate Boundary Rule — "must never own Knowledge")
- `docs/engineering/RECOMMENDATION_REPAIR_STRUCTURE_IMPLEMENTATION_AUTHORIZATION.md`
- `docs/engineering/SPRINT24_IMPLEMENTATION_PLAN.md`

---

## Invariants Implemented

No new invariant added; existing invariants extended to the new types:

- ADR-0007's Runtime Entity Design Pattern — `RecommendationStep` follows the Opaque-value shape (no identity, infallible constructor), the same category as `RepairRecipeReference` and `modiq_knowledge::knowledge::RepairStep`, not one of the four identity-bearing entities.
- ADR-0007's Governance-Controlled Invariants — no new relationship between `repair_steps` and `repair_recipe_reference` was enforced (e.g., "must be non-empty when a reference exists"); doing so would be a Level 3 (Behavioral) change requiring its own Governance Register item, not authorized here. `repair_steps` is exactly as permissive as `repair_recipe_reference` already was.
- Runtime Domain Crate Boundary Rule ("must never own Knowledge") — confirmed by direct evidence: `modiq-runtime`'s `Cargo.toml` lists no dependency on `modiq-knowledge`; a code-only grep (`use modiq_knowledge`) inside `crates/modiq-runtime/src` returns zero hits.

---

## Tests Added

- `modiq-runtime` (`recommendation.rs`, net +1): `new_preserves_provided_repair_steps`, mirroring `new_preserves_provided_repair_recipe_reference` exactly.
- `modiq-rules` (`version_compatibility_rule.rs`, net +0, 2 existing tests extended): `produces_a_warning_finding_for_an_unrecognized_declared_version` gained assertions that `repair_steps()` is non-empty and its first entry's `kind()`/`instruction()` match the real `RepairRecipe::version_compatibility_declared_version_mismatch()` step content; `is_deterministic_for_identical_input` gained a `repair_steps()` comparison between two evaluations.
- `modiq-storage` (`persisted_report.rs`, net +1): `from_recommendation_preserves_repair_steps_content_and_order`, asserting `PersistedRecommendation::from_recommendation` preserves two ordered, differently-kinded `RecommendationStep`s faithfully.
- No test deleted anywhere. All new assertions exercise real projected content — no mocking, matching this repository's standing discipline since Sprint 3 Phase 5.

**Kind-mapping table (`RepairStepKind` → `RecommendationStepKind`, `modiq-rules`; and `RecommendationStepKind` → `PersistedRecommendationStepKind`, `modiq-storage`) — both exhaustive, no wildcard arm:**

| `RepairStepKind` (Knowledge) | `RecommendationStepKind` (Runtime) | `PersistedRecommendationStepKind` (Storage) |
|---|---|---|
| `XmlChange` | `XmlChange` | `XmlChange` |
| `LuaChange` | `LuaChange` | `LuaChange` |
| `DependencyInstallation` | `DependencyInstallation` | `DependencyInstallation` |
| `AssetReplacement` | `AssetReplacement` | `AssetReplacement` |
| `VersionUpdate` | `VersionUpdate` | `VersionUpdate` |

---

## Design Decisions

- **The new Runtime type is named `RecommendationStep`/`RecommendationStepKind`, deliberately not `RepairStep`/`RepairStepKind`.** `modiq_knowledge::knowledge::RepairRecipe` and the new Runtime type are both visible, by name, in the one file where both domains legitimately meet (`version_compatibility_rule.rs`). Reusing Knowledge's own names would force qualification discipline at every reference in that file for no architectural benefit, since two distinct types are required regardless of naming (Runtime must never hold Knowledge's own type).
- **`repair_steps` is a plain `Vec`, never `Option<Vec<_>>`.** An empty `Vec` is the "no Repair Recipe informed this Recommendation" state — mirroring how `Vec<FindingId>` and `Vec<EvidenceId>` are already plain, never-`Option`-wrapped collections elsewhere on the same and adjacent types, rather than introducing a new optionality shape with no precedent on this platform.
- **`RecommendationStep::new` is infallible.** It is not one of ADR-0007's four identity-bearing entities requiring a `Result`-returning constructor; it mirrors `RepairRecipeReference::new`'s and `modiq_knowledge::knowledge::RepairStep::new`'s own infallible shape.
- **`PersistedRecommendation`'s mirror is one-way only** — `Serialize + Deserialize` on the new types enables genuine restoration of the persisted data as Storage's own read-only value (retrievable via `repair_steps()`), but no `to_recommendation` reconstructing a live `Recommendation` was added, matching the fact that no other field on this type (`action`, `finding_indices`, `repair_recipe_reference`) round-trips back into a live Runtime value either.

---

## Assumptions Made

- That the word "restoration," used in the Phase 3 authorization prompt, was consistent with — not a request to extend beyond — the Plan's own explicitly one-way shape. Resolved by implementing `Deserialize` on the new Storage types (genuine restoration of persisted data as Storage's own value) without adding any reverse constructor into a live `Recommendation`, and by flagging the reading explicitly in the Phase 3 report rather than resolving it silently.

---

## Known Limitations

- `apps/console`, `apps/sandbox`, and `modiq-cli` do not present `repair_steps` in any form — out of scope by Authorization §4, unaffected and unattempted.
- `PersistedRecommendation`'s new `repair_steps` field has no `#[serde(default)]` and no schema-versioning mechanism; a report persisted before this Sprint that is later retrieved will fail to deserialize. This is not a new category of exposure this Sprint introduces — Sprint 22 already created the identical exposure when `PersistedFinding`'s shape changed with no migration mechanism, and no such mechanism exists anywhere in `modiq-storage` today. Recorded as an accepted, pre-existing condition, not resolved here (Authorization §3 does not name a persistence-migration mechanism as in scope).
- `RepairRecipeReference` remains permanently unresolved — the new `repair_steps` structure is a parallel, independently populated representation, not a resolution mechanism for the reference.

---

## Architectural Validation

Every Authorization §3 condition was checked directly against source, not assumed:

- No new Cargo dependency edge anywhere — confirmed, zero diff on every `Cargo.toml`/`Cargo.lock`.
- No `modiq-knowledge` type referenced, aliased, or re-exported outside `modiq-knowledge`'s own boundary — confirmed by a code-only grep (`use modiq_knowledge`) inside every other crate's `src/`, returning zero hits; the one legitimate code reference (`version_compatibility_rule.rs`) is unchanged in kind from before this Sprint, only extended to also match on `RepairStepKind`.
- `RepairRecipeReference` remains unresolved everywhere downstream — confirmed, `repair_recipe_reference.rs` has zero diff.
- No mutation method added to `Recommendation`, `Finding`, or `Evidence` — confirmed by a diff-wide grep for `fn (set_|push_|add_)`, returning none.

---

## Architectural Concerns

None substantive. One verification-gate limitation was surfaced and is recorded below as an Engineering Observation, not an architectural concern — the underlying invariant it checks was independently confirmed true by direct evidence (Cargo dependency graph, code-only grep), and no conflict was found between this implementation and the Authorization, the Resolution, or ADR-0007.

---

## Governance Observations

No Governance Register item, ADR, or Crate Boundary Rule change is indicated by this work's evidence. Confirmed directly: `docs/engineering/GOVERNANCE.md` and `docs/adrs/` are untouched by this Sprint. As instructed at closeout, the pre-existing GOV-017 tracking-document discrepancy (`PROJECT_STATUS.md`, `CHANGELOG.md`, and `ENGINEERING_LOG.md` have never recorded GOV-017, understating the Governance Register by one item in all three, first surfaced at Sprint 23's own closeout) is **not corrected by this Sprint's own tracking updates** — named here again for continuity, not resolved.

**Engineering Observation — Phase 4 verification-gate limitation, not an implementation or architectural defect.** The Sprint Plan's Knowledge-boundary verification gate (a literal `grep` for `RepairRecipe`/`RepairStep`/`RepairStepKind` across every participating and mechanically-touched crate) returned matches in two files instead of the Plan's stated one: `crates/modiq-rules/src/rules/version_compatibility_rule.rs` (expected, the one authorized call site) and `crates/modiq-runtime/src/assessment/recommendation_step.rs` (unexpected). The completed Implementation Audit re-derived this independently and classified it precisely: the two extra matches are inside `///` doc comments explaining the Phase 1 naming decision, not `use` statements or type references — confirmed by a code-only grep returning zero hits, and by `modiq-runtime`'s `Cargo.toml` carrying no dependency on `modiq-knowledge`, both unchanged by this Sprint. More decisively, this exact convention — a `modiq-runtime` doc comment naming "modiq-knowledge" or a Knowledge-domain type by name to explain a Runtime/Knowledge seam — already exists in two files this Sprint never touched (`rule_reference.rs`, `repair_recipe_reference.rs`, both confirmed zero-diff). **Classification: a verification-gate limitation (the gate's regex cannot distinguish prose from code), not an implementation defect, not a documentation issue (the wording matches settled, pre-existing repository convention), and not an architectural boundary crossing (independently confirmed by direct evidence).**

---

## Implementation Constraints

All constraints stated in `RECOMMENDATION_REPAIR_STRUCTURE_IMPLEMENTATION_AUTHORIZATION.md` §3 and §7 were honored, confirmed by direct re-derivation at the completed Implementation Audit, not carried forward from phase reports alone:

- Projection occurs exactly once, inside `modiq-rules`, at the single existing crate boundary where both domains are already visible — confirmed, no other file performs it.
- Population at construction only; no post-construction setter or mutation path — confirmed.
- `action` remains architecturally valid and unchanged — confirmed, `action()`'s field, getter, and every one of its 7 real call sites (`apps/sandbox` ×2, `apps/console` ×1, `modiq-cli` ×2, `modiq-storage` ×2) are unaffected.
- No item named in Authorization §4 (Explicit Exclusions) was touched, added, or implied.

---

## Recommendations

- `repair_steps` presentation in any consumer application remains a real, evidence-backed future opportunity, requiring its own, separate Implementation Authorization against `apps/console`'s transport DTOs — not a continuation of this Sprint's own pattern, mirroring how Sprint 22 (Runtime) and Sprint 23 (presentation) were deliberately kept separate.
- The pre-existing persistence-migration gap (no `#[serde(default)]`, no schema versioning, anywhere in `modiq-storage`) now has two independent instances (`PersistedFinding` since Sprint 22, `PersistedRecommendation` since this Sprint) — worth a dedicated evaluation before a third field-shape change compounds it further.
- The GOV-017 tracking-document staleness (Governance Observations, above) remains outstanding across three documents and two Sprints' worth of closeout notes; a dedicated, small reconciliation pass remains the appropriate mechanism, explicitly not this Sprint's own closeout.

---

## Validation Summary

```
Root workspace (default-members):
cargo fmt --check       ✅ clean
cargo check --workspace ✅ clean
cargo test              ✅ 269 → 271 passed

Full workspace (--workspace, 10 members):
cargo test --workspace  ✅ 274 → 276 passed (console unchanged at 5/5)

modiq-runtime (explicit -p):
cargo test -p modiq-runtime  ✅ 89 → 90 passed

modiq-rules (explicit -p):
cargo test -p modiq-rules  ✅ 36/36 passed (unchanged count, assertions extended)

modiq-storage (explicit -p):
cargo test -p modiq-storage  ✅ 18 → 19 passed

modiq-report (explicit -p):
cargo test -p modiq-report  ✅ 3/3 passed (unchanged)

apps/sandbox/src-tauri (separate workspace):
cargo fmt --check ✅ clean
cargo check       ✅ clean
cargo test        ✅ 9/9 passed, unchanged

Dependency-edge check:
git diff on all Cargo.toml, filtered for added `path =` lines  ✅ zero matches

Knowledge-boundary check:
grep for RepairRecipe/RepairStep/RepairStepKind, code + doc comments  ⚠ 2 files (1 expected code
reference + 1 doc-comment-only false positive — see Engineering Observation, above)

action() consumer check (apps/sandbox, apps/console, modiq-cli, modiq-storage):
same 7 call sites as pre-Sprint-24 baseline  ✅ unaffected
```

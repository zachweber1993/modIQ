# Sprint 22 Implementation Plan — Initiative 3

| Property | Value |
|---|---|
| **Document** | SPRINT22_IMPLEMENTATION_PLAN.md |
| **Project** | modIQ |
| **Origin** | `INITIATIVE_3_ARCHITECTURE_EVALUATION.md`, `INITIATIVE_3_ARCHITECTURAL_RESOLUTION.md`, `INITIATIVE_3_IMPLEMENTATION_AUTHORIZATION.md` — all treated as fixed, unreopened architecture. |
| **Status** | Sprint Planning draft. No code, crate, or test has been produced in preparing this document. |

---

## 1. Sprint Objective

Implement exactly the six items `INITIATIVE_3_IMPLEMENTATION_AUTHORIZATION.md` authorizes — Items 1, 2, 3, 4 (Label/Source/Content), 5, and 10 — across `Finding`, `Evidence`, `RuleOutcome`, and `RepairRecipe`, and propagate the resulting signature changes through every real and test consumer so the full workspace, both consumer applications, and the separate Sandbox workspace all build and pass cleanly. No new capability, no presentation-layer exposure beyond what compilation correctness requires, and no item outside the authorized six.

## 2. Implementation Scope

- **Item 1.** `Finding` gains `title: String`, `summary: String`; `description: String` and its getter are removed. `Finding::new`'s signature changes; every construction site and every reader of `description()` updates.
- **Item 2.** New closed-set type `ModHealthDimension` (`Compatibility`, `Stability`, `Maintainability`, `Performance`, `Structure`, `EngineeringQuality`, per Authorization §3). `Finding` gains `mod_health_dimension: ModHealthDimension`. Every Rule requires an explicit `Rule → ModHealthDimension` mapping, reviewed and finalized as a discrete artifact before Phase 2 begins — not decided incrementally while each Rule's own call site is edited.
- **Item 3.** `RuleOutcome.recommendation` changes from `Recommendation` to `Option<Recommendation>`. Every Rule's `RuleOutcome` literal wraps its existing value in `Some(...)`.
- **Item 4 (Label/Source/Content only).** `Evidence` gains `label: String`, `source: String`, `content: String`. Both existing constructors (`Evidence::new`, `Evidence::with_location`) update. Each of the four Collectors decides what to populate for its own Evidence.
- **Item 5.** `RepairRecipe.guidance: String` is replaced by a structured step type referencing the five change-kinds `Glossary.md` names (XML changes, Lua changes, Dependency installation, Asset replacement, Version updates). The one real authored recipe (`version_compatibility_declared_version_mismatch`) migrates to the new shape without changing its own meaning.
- **Item 10.** `Finding` gains `status: FindingStatus` (`Provisional`, `Final`), populated at construction only — no naming collision applies here (unlike Item 2), so no separate naming step is required.

## 3. Crate Impact

This table constitutes the complete pre-implementation call-site inventory; Rust's own compiler enforces its completeness the moment `Finding::new`'s and `Evidence`'s signatures change (Phase 2 gate, §7) — no separate audit phase is required.

| Crate | Required changes | Downstream impact | Required or verification-only |
|---|---|---|---|
| `modiq-runtime` | `Finding`: new fields/types (Items 1, 2, 10), `Finding::new` signature change, `description()` removed. `Evidence`: new fields (Item 4), both constructors updated. | Every crate below. | **Required.** |
| `modiq-rules` | All 4 Rules' `Finding::new` calls (title/summary split, dimension assignment, status); all 4 `RuleOutcome` literals wrap `recommendation` in `Some(...)`. 5 test files assert `.description()`/`.severity()` directly and need updating. | `modiq-engine`, `modiq-report`, consumer apps. | **Required.** |
| `modiq-knowledge` | `RepairRecipe`'s structured-step type; migrate the one real authored recipe; update `repair_recipe.rs`'s own test constructions (9 sites, mostly test-only). | `modiq-rules` (`version_compatibility_rule.rs` reads `RepairRecipe::version_compatibility_declared_version_mismatch()`). | **Required.** |
| `modiq-collection` | Each of 4 Collector modules (`evidence_collector.rs`, `archive_collector.rs`, `xml_collector.rs`, `runtime_log_collector.rs`) updates its `Evidence::new`/`with_location` calls for Item 4's new fields — a per-Collector content decision, not mechanical. | `modiq-engine` (via `execute_from_assessment_input`). | **Required.** |
| `modiq-report` | No production code touches individual fields (`AssessmentReport::generate` only clones/`.to_vec()`s whole collections — confirmed directly). Its own test module constructs `Finding::new`/`Evidence::new` directly as fixtures (`assessment_report.rs:62,67`) and needs updating. | None beyond its own tests. | **Required, confined to test fixtures only** — no production path changes. |
| `modiq-engine` | Production orchestration loop (`assessment_service.rs`) unconditionally calls `add_recommendation(outcome.recommendation)` — must become conditional on `Some` once `RuleOutcome.recommendation` is `Option<Recommendation>` (Item 3). Its own tests also construct `Evidence::new` directly (`assessment_service.rs`, `end_to_end.rs`) and assert `.description()`/`.severity()` on Findings the real pipeline produces (multiple sites, both files). | `AssessmentReport`'s own `recommendations` list — a skipped `None` means one fewer entry, not an error. | **Required — production code, not test-only.** |
| `modiq-storage` | `PersistedFinding::from_finding` (`persisted_report.rs`) calls `finding.description()` directly in real production code. Requires a design decision: add `title`/`summary` fields to `PersistedFinding` (recommended — preserves symmetry with Runtime's own shape) versus synthesizing a `description` string to preserve the existing persisted format. | `modiq-cli` (`retrieve.rs` reads `PersistedFinding`). | **Required.** |
| `modiq-cli` | Two commands, not one: `assess.rs` calls `finding.description()` on a live `Finding` (Item 1, Phase 1 dependency); `retrieve.rs` calls the same method name on a `PersistedFinding` (`modiq-storage`, Phase 3 dependency) — both must be updated, but on different underlying types. | None beyond its own output formatting. | **Required.** |
| `apps/sandbox` | `lib.rs`, two distinct call sites on two different types: `create_assessment` calls `finding.description()` on a live `Finding` (Item 1). `retrieve_report`'s own `PersistedFindingEntry::from(&PersistedFinding)` (`lib.rs:135–141`) separately calls `finding.description()` on a `PersistedFinding` — depends on `modiq-storage`'s Phase 3 design decision, not on `Finding`'s shape directly. | None — a developer harness, no further downstream. | **Required**, confined to compilation correctness — no new UI. |
| `apps/console` | `assessment.rs`, one call site, calls `finding.description()` directly to build `FindingSummary`. | `Reviewing.tsx`/`Overview.tsx` currently render `finding.description` — this Sprint updates the Rust-side DTO only; whether the TypeScript presentation layer is updated to show title/summary/dimension separately is explicitly not part of this Sprint's scope (§6). | **Required**, confined to compilation correctness at the transport boundary. |

## 4. Recommended Phase Breakdown

**Phase 1 — Foundational domain types.**
*Objective:* establish every new type and field, in the crates with no dependency on each other, in one pass.
*Crates:* `modiq-runtime` (Items 1, 2, 4, 10), `modiq-knowledge` (Item 5).
*Why combined:* `Finding`'s constructor changes exactly once here (Items 1, 2, 10 together), not three separate times. `modiq-knowledge` has zero dependency on `modiq-runtime`'s Finding/Evidence changes and can proceed in the same phase without coordination.
*Outputs:* `Finding`, `Evidence`, `ModHealthDimension`, `FindingStatus`, `RepairRecipe`'s new shape, all compiling within their own crate's own test suite, plus a reviewed `Rule → ModHealthDimension` mapping table (four rows: `EvidencePresenceRule`, `StructuralDuplicationRule`, `VersionCompatibilityRule`, `RuntimeLoadFailureRule`), finalized before Phase 2 begins.
*Completion criteria:* `cargo test -p modiq-runtime` and `cargo test -p modiq-knowledge` both pass; no other crate touched yet (expected to be broken workspace-wide at this point — acceptable within a phase, not across a phase boundary).

**Phase 2 — Direct dependents.**
*Objective:* propagate Phase 1's new constructors into every crate that directly produces a `Finding` or `Evidence`.
*Crates:* `modiq-rules` (Items 1, 2, 3, 10 propagation), `modiq-collection` (Item 4 propagation).
*Why combined:* `modiq-rules` depends on `modiq-runtime` and `modiq-knowledge`; `modiq-collection` depends on `modiq-runtime` only — both dependencies are fully satisfied by Phase 1, and `modiq-rules` / `modiq-collection` do not depend on each other, making them independent and parallelizable within the phase.
*Outputs:* all 4 Rules constructing valid `Finding`/`RuleOutcome` values, each using its own reviewed mapping-table dimension; all 4 Collectors constructing valid `Evidence` values.
*Completion criteria:* `cargo test -p modiq-rules` and `cargo test -p modiq-collection` both pass.

**Phase 3 — Indirect and storage consumers.**
*Objective:* fix every remaining workspace crate that references the changed types without producing them directly.
*Crates:* `modiq-report` (test fixtures only), `modiq-engine` (production orchestration loop plus test fixtures and assertions), `modiq-storage` (`PersistedFinding`'s real design decision), `modiq-cli` (`assess.rs`, `retrieve.rs`).
*Why this grouping:* all four depend on Phase 1 and Phase 2 being complete. Within this phase, `modiq-storage` must complete before `modiq-cli`, since `modiq-cli`'s `retrieve.rs` depends directly on `modiq-storage`'s `PersistedFinding`; `modiq-report` and `modiq-engine` remain independent of this pair and of each other.
*Outputs:* root workspace (`cargo test`, default-members) fully green.
*Completion criteria:* `cargo fmt --check`, `cargo check`, `cargo test` all clean at the root workspace.

**Phase 4 — Application consumers.**
*Objective:* restore compilation and correctness in both consumer applications, without expanding scope into presentation changes.
*Crates:* `apps/sandbox` (separate workspace), `apps/console` (`console` crate, `-p console`/`--workspace`).
*Outputs:* both applications compile and their own tests pass, reading `title`/`summary` in place of `description` at minimum.
*Completion criteria:* `apps/sandbox/src-tauri` — `cargo fmt --check`, `cargo check`, `cargo test` all clean, 9/9 unaffected in count. `console` — `cargo check -p console`/`--workspace`, `cargo test -p console` clean, 4/4 unaffected in count. `npm run build` in `apps/console` clean.

This ordering minimizes broken builds (each phase leaves its own crate boundary green before the next begins) and touches `Finding`'s constructor exactly once (Phase 1), not once per item.

## 5. Testing Strategy

Per phase, following existing repository precedent only — no new methodology:

- **Phase 1:** new/updated unit tests in `modiq-runtime` for `Finding::new`'s new signature (construction succeeds with valid inputs; any new validation, e.g., non-empty `title`, rejects the same way `FindingError::EmptyDescription` already does) and in `modiq-knowledge` for the new `RepairRecipe` shape. No mocking, per standing discipline since Sprint 3 Phase 5.
- **Phase 2:** the 5 existing `modiq-rules` test files updated in place (assertions move from `.description()` to `.title()`/`.summary()`); each Rule's own test gains an assertion confirming its assigned `ModHealthDimension` and `FindingStatus`, matching its own row in the mapping table. `modiq-collection`'s existing per-Collector tests updated for the new Evidence fields; no new determinism test required beyond what already exists, since field addition doesn't change ordering.
- **Phase 3:** `modiq-report`'s and `modiq-engine`'s test fixtures updated in place. `modiq-storage` requires a genuinely new test: `PersistedFinding::from_finding` round-tripping title/summary correctly — this is real, new behavior, not a mechanical update. `modiq-cli`'s existing `assess`/`retrieve` integration tests (real fixture directories, real temp storage — already established pattern) updated for the new output format.
- **Phase 4:** `apps/console`'s existing 4 real-fixture tests (`assessment.rs`) updated for the new `FindingSummary` shape; `apps/sandbox`'s existing tests likewise. No new test framework introduced in either.
- **Regression expectation, every phase:** root workspace test count changes only by net-new tests added (Phase 1 additions, `modiq-storage`'s new round-trip test) — no existing test should be deleted, only updated in place.

## 6. Engineering Risks

Ranked by implementation risk:

1. **The production orchestration loop's `None`-handling decision (`assessment_service.rs`) is a new highest-tier risk, not merely a compile fix.** Once `RuleOutcome.recommendation` is `Option<Recommendation>`, `assessment_service.rs`'s own evaluation loop must decide explicit runtime behavior when it is `None`: skip adding a Recommendation, never panic, never synthesize a placeholder. The compiler forces a decision here; it does not enforce the correct one.
2. **`modiq-storage`'s `PersistedFinding` design decision (Item 1 cascade).** This Sprint should add `title`/`summary` fields directly, mirroring Runtime's own shape — deferring or improvising this decision mid-implementation is the most likely source of rework.
3. **`Finding::new`'s constructor signature change is the widest single breaking change in the Sprint** — 4 Rules, 5 test files in `modiq-rules`, 2 fixtures in `modiq-report`, fixtures and assertions in `modiq-engine`, 2 commands in `modiq-cli`, `modiq-storage`, both consumer apps. Compiler-caught, not a silent-correctness risk, but the largest surface area of the six items.
4. **Item 2's per-Rule dimension assignment is a substantive judgment call** — mitigated by the reviewed mapping table now required at Phase 1 (§2, §4), rather than left to occur incrementally.
5. **Item 4's per-Collector content decision** carries a real risk of inconsistent population across the four Collectors if not deliberately reviewed together in Phase 2, rather than done Collector-by-Collector in isolation.
6. **`apps/console`'s `ReportSummary` DTO is explicitly provisional** (`FRONTEND_IMPLEMENTATION_AUTHORIZATION.md` §4, GOV-008) — extending it here is expected, not a new risk, but should not be mistaken for finalizing its shape.
7. **Item 5's migration of the one real authored recipe** — lowest risk, single call site, single crate, no downstream consumer beyond `version_compatibility_rule.rs`.

## 7. Verification Gates

Before progressing past each phase:

- **After Phase 1:** `cargo fmt --check`, `cargo check -p modiq-runtime -p modiq-knowledge`, `cargo test -p modiq-runtime -p modiq-knowledge` — all clean. Mapping table reviewed and finalized.
- **After Phase 2:** `cargo check -p modiq-rules -p modiq-collection`, `cargo test -p modiq-rules -p modiq-collection` — all clean. Constructor migration for `Finding` and `Evidence` confirmed complete (no remaining call site anywhere in these two crates using the old shape).
- **After Phase 3:** full root-workspace `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` — all clean. `modiq-storage`'s new round-trip test passing specifically, not just the crate's suite as a whole.
- **After Phase 4:** `apps/sandbox/src-tauri` — `cargo fmt --check`, `cargo check`, `cargo test`, all clean, 9/9 unaffected. `console` — `cargo check -p console`/`--workspace`, `cargo test -p console`, `npm run build` — all clean. Downstream consumer compilation confirmed for every one of the four consumers named in §3, not assumed from crate-level success alone.

No phase begins before the prior phase's gate passes in full.

## 8. Sprint Completion Criteria

- All six authorized items (1, 2, 3, 4 Label/Source/Content, 5, 10) implemented exactly as `INITIATIVE_3_IMPLEMENTATION_AUTHORIZATION.md` specifies — no more, no less.
- `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` clean at the root.
- `apps/sandbox/src-tauri`'s own workspace clean, test count unchanged in kind (only net-new/updated tests, no deletions).
- `console` clean under both bare and `--workspace`/`-p console` invocations; `npm run build` clean.
- Every call site identified in §3 updated — verified by a workspace-wide search for `.description()` on `Finding` returning zero remaining matches, and for `RuleOutcome { finding, recommendation }` (unwrapped) returning zero remaining matches.
- No item outside the authorized six touched, added, or implied.

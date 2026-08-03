# Sprint 23 Implementation Plan — Frontend Presentation of Sprint 22's Domain Model

| Property | Value |
|---|---|
| **Document** | SPRINT23_IMPLEMENTATION_PLAN.md |
| **Project** | modIQ |
| **Origin** | `FRONTEND_PRESENTATION_IMPLEMENTATION_AUTHORIZATION.md` (`f1d4882`), treated as fixed, unreopened architecture. `FrontendArchitecture.md`, `FRONTEND_IMPLEMENTATION_AUTHORIZATION.md`/Sprint 21, and Sprint 22 (`969e595`/`80c79f4`) are all fixed inputs, not restated. |
| **Status** | Sprint Planning draft. No code has been produced in preparing this document. |

---

## 1. Sprint Objective

Implement exactly what `FRONTEND_PRESENTATION_IMPLEMENTATION_AUTHORIZATION.md` §5 authorizes: extend `apps/console`'s transport DTOs and TypeScript mirrors to carry `mod_health_dimension`, `status`, and Evidence's `label`/`source`/`content`, then render Title, `ModHealthDimension`, `FindingStatus`, and Evidence provenance in `Reviewing.tsx` (and, for aggregate-only content, `Overview.tsx`), correcting `Overview.tsx`'s stale doc comment in the same pass. No Runtime, Rule, Collector, Engine, Reporting, Storage, or CLI change; no new Tauri command; no new navigation or interaction mechanism. Success is defined by the Authorization's own Success Criterion (§3): the four fields become visible, using only what the Runtime already exposes — nothing more.

---

## 2. Implementation Scope

Direct translation of Authorization §5's five bullets:

- **Item A.** `FindingSummary` (Rust) gains `mod_health_dimension: String` and `status: String`, populated via `format!("{:?}", finding.mod_health_dimension())` / `format!("{:?}", finding.status())` — mirroring the existing `severity` field's own `Debug`-formatted-string convention (`assessment.rs:108`), not a new representation choice.
- **Item B.** `EvidenceSummary` (Rust) gains `label: Option<String>`, `source: Option<String>`, `content: Option<String>`. Each field is populated via `.map(str::to_string)` over the corresponding `Evidence::label()`, `Evidence::source()`, and `Evidence::content()` accessors, identical to the existing `location` conversion already used in `assessment.rs` (`evidence.location().map(str::to_string)`, `assessment.rs:49`), rather than introducing any new conversion pattern.
- **Item C.** `types.ts`'s `FindingSummary`/`EvidenceSummary` interfaces mirror Items A/B exactly, in the existing camelCase convention `#[serde(rename_all = "camelCase")]` already produces (`modHealthDimension`, `status`, `label`, `source`, `content`).
- **Item D.** `Reviewing.tsx` renders `finding.title` (already present in the DTO, currently unused) alongside `finding.summary`; renders `finding.modHealthDimension` and `finding.status` in the Finding's own row/expansion; renders each Evidence item's `label`/`source`/`content` where present, in the existing expansion layer — no new expansion level.
- **Item E.** `Overview.tsx`'s doc comment is corrected to remove the now-false claim that `ModHealthDimension` "does not exist on the Runtime yet." Aggregate presentation of `ModHealthDimension` (e.g., a per-dimension count, mirroring the existing per-severity count) is authorized only if it stays within Overview's existing derived-view discipline — computed from `findings`, never transported separately.

## 3. Crate / File Impact

| File | Required change | Authorization basis |
|---|---|---|
| `apps/console/src-tauri/src/assessment.rs` | `FindingSummary` gains 2 fields (Item A); `EvidenceSummary` gains 3 fields (Item B); both `From` impls updated. | §5, conditioned on §5's "existing, already-public Runtime getter" constraint. |
| `apps/console/src/engine/types.ts` | `FindingSummary`/`EvidenceSummary` interfaces mirrored (Item C); header doc comment's third "Notably absent" bullet updated to state that Mod Health Dimension, Evidence Label, Evidence Source, and Evidence Content are now consumed — exactly four fields leave the list. **Confidence remains listed as absent**, unaffected by this Sprint, since it does not yet exist on the Runtime at all (a distinct reason from the other four, which were merely unconsumed until now). | §5. |
| `apps/console/src/workspace/Reviewing.tsx` | Render `title`, `modHealthDimension`, `status`, Evidence `label`/`source`/`content` (Item D). | §5; §6 Implementation Constraints ("no new expansion level"). |
| `apps/console/src/workspace/Overview.tsx` | Doc-comment correction (Item E); optional per-dimension aggregate count, if pursued. | §5, §10 (Risk: doc comment "asserts something false"). |
| `modiq-runtime`, `modiq-rules`, `modiq-knowledge`, `modiq-collection`, `modiq-engine`, `modiq-report`, `modiq-storage`, `modiq-cli` | **No change.** | §9 Crate/Application Impact: "Unaffected." |
| `apps/sandbox` | **No change** — separate workspace, out of scope. | §6, out-of-scope list. |

No file outside this table is expected to require modification. If one does, implementation should stop and report it rather than proceed — per the Authorization's own Success Criterion, anything requiring a change outside `apps/console` has exceeded this Sprint's scope.

## 4. Recommended Phase Breakdown

**Phase 1 — Transport (Rust).**
*Objective:* extend `FindingSummary`/`EvidenceSummary` and their `From` impls.
*Files:* `apps/console/src-tauri/src/assessment.rs`.
*Completion criteria:* `cargo check -p console` clean; existing 4 tests still pass unmodified (field additions are additive, not breaking, to already-passing assertions); new assertions added confirming `mod_health_dimension` and `status` are populated on real fixture output, and confirming at least one Evidence item in the fixture's output carries a non-`None` `label` — exercising this phase's own new fields immediately, rather than deferring their verification past phases with no dependency on them (matching Sprint 22's own per-phase test-gate discipline).

**Phase 2 — Transport (TypeScript).**
*Objective:* mirror Phase 1's fields in `types.ts`; correct its own header comment's "Notably absent" list.
*Files:* `apps/console/src/engine/types.ts`.
*Completion criteria:* `tsc` (part of `npm run build`) clean; no runtime behavior change yet, since nothing consumes the new fields until Phase 3.

**Phase 3 — Presentation.**
*Objective:* render the five newly-transported fields in `Reviewing.tsx`; correct `Overview.tsx`'s stale comment; add the optional per-dimension aggregate in `Overview.tsx` only if it stays within existing derived-view discipline.
*Files:* `apps/console/src/workspace/Reviewing.tsx`, `apps/console/src/workspace/Overview.tsx`.
*Completion criteria:* `npm run build` clean; manual verification against the existing fixture (`FIXTURE_SAMPLE_MOD`) that Title, Mod Health dimension, Status, and Evidence provenance are all visible in the rendered Reviewing experience. See §7 for why this gate is intentionally interpretive rather than automated.

**Phase 4 — Final Reverification.**
*Objective:* final repository reverification, regression confirmation, and end-to-end validation only — Phase 1 already added the Rust-side assertions for this Sprint's new fields; this phase confirms nothing regressed once TypeScript and presentation are also complete.
*Files:* None — verification-only phase.
*Completion criteria:* `cargo test -p console` — 4/4 or more (including Phase 1's new assertions), no deletions; root workspace `cargo fmt --check`/`check --workspace`/`test --workspace` unaffected (269/269, 273/273 full workspace); `apps/sandbox/src-tauri` unaffected (9/9); `npm run build` clean.

This ordering matches Sprint 22's own phase discipline (foundational layer before dependents) and Sprint 21's own precedent (Rust transport before TypeScript before presentation) — no phase renders data the prior phase has not yet made available.

## 5. Testing Strategy

Per repository precedent, restated, not newly invented:

- Phase 1: extend `every_finding_carries_a_non_empty_severity_title_and_summary` (or add an adjacent test) to assert `mod_health_dimension` and `status` are non-empty strings, matching the existing test's own real-fixture, no-mocking pattern (`FIXTURE_SAMPLE_MOD`). Add an assertion that at least one Evidence item in the fixture's output carries a non-`None` `label` — confirming the transport, not merely compiling. These assertions exercise Phase 1's own new fields and require nothing from Phase 2 or Phase 3.
- Phase 2: no new test required — `tsc`'s own type-checking is this phase's own verification; TypeScript interfaces have no independent runtime behavior to assert against.
- Phase 4: no new test authored here — this phase re-runs Phase 1's assertions (now unmodified) alongside a full-repository and Sandbox reverification, confirming no regression once Phases 2–3 are also complete.
- No test is deleted at any phase, matching Sprint 22's own regression discipline for this exact crate.

## 6. Engineering Risks

Restated and ranked from `FRONTEND_PRESENTATION_IMPLEMENTATION_AUTHORIZATION.md` §10, not re-derived:

1. **Presentation inconsistency between Evidence's now-structured provenance and `Recommendation`'s still-flat `action` string.** Not a defect Sprint 23 creates — `RepairRecipe` steps remain excluded (Authorization §6) — but Phase 3 should present `Recommendation` unchanged, not attempt to enrich it to match.
2. **`ModHealthDimension`'s six values have no existing visual precedent.** Phase 3 is authorized for plain-text presentation only; any grouping, iconography, or color treatment is out of scope and must not be improvised mid-phase.
3. **`Overview.tsx`'s comment correction must be precise** — restate only what is now true (`ModHealthDimension` exists on the Runtime), not expand the comment's own scope into unrelated claims.
4. **Scope creep back into the transport's own eventual GOV-008 shape.** Nothing in this Sprint should be implemented as if `ReportSummary`'s current shape were final — it remains provisional exactly as before.

## 7. Verification Gates

Before progressing past each phase:

- **After Phase 1:** `cargo fmt --check`, `cargo check -p console`, and `cargo test -p console` all clean. Existing real-fixture coverage remains intact, with the required new assertions for `mod_health_dimension`, `status`, and Evidence `label` present. Test count may remain unchanged if assertions extend existing tests, or increase if adjacent tests are added; no existing test is removed.
- **After Phase 2:** `tsc` clean (via `npm run build` or standalone).
- **After Phase 3:** `npm run build` clean; manual fixture-driven verification of all five fields' visibility. **This gate is intentionally interpretive, not automated:** `apps/console` currently contains no React/component testing framework (confirmed — no Vitest, React Testing Library, Jest, or Playwright in `package.json`'s `devDependencies`), and introducing one is outside this Authorization's scope. Manual verification against the running application, using the existing `sample-mod` fixture data, is therefore the correct verification method available today, not a shortcut taken in place of an automated one.
- **After Phase 4:** full root workspace (`cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace`) and `apps/sandbox/src-tauri` (`cargo fmt --check`, `cargo check`, `cargo test`) both reverified clean and unaffected — confirming the Authorization's own "Unaffected" claim (§9) empirically, not by assertion alone.

No phase begins before the prior phase's gate passes in full.

## 8. Sprint Completion Criteria

- Title, `ModHealthDimension`, `FindingStatus`, and Evidence `label`/`source`/`content` are all visible in `apps/console`'s Reviewing experience against the existing fixture.
- `Overview.tsx`'s stale doc comment is corrected.
- No field was added to any transport DTO without an existing, already-public Runtime getter backing it (Authorization §3's Success Criterion, verified by inspection at Phase 1/2 gates).
- No `modiq-runtime`, `modiq-rules`, `modiq-knowledge`, `modiq-collection`, `modiq-engine`, `modiq-report`, `modiq-storage`, or `modiq-cli` file is touched.
- `apps/sandbox` is untouched.
- `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` clean at the root; `apps/sandbox/src-tauri` clean and unaffected; `console` clean with test count grown only by net-new assertions, no deletions; `npm run build` clean.
- No item named in `FRONTEND_PRESENTATION_IMPLEMENTATION_AUTHORIZATION.md` §6 (Explicit Exclusions) is touched, added, or implied.

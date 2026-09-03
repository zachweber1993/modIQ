# IMPLEMENTATION_PLAN_REPORT_ANATOMY_ALIGNMENT.md

## 1. Plan Identity

| Property | Value |
|---|---|
| **Document** | IMPLEMENTATION_PLAN_REPORT_ANATOMY_ALIGNMENT.md |
| **Origin** | `IMPLEMENTATION_AUTHORIZATION_REPORT_ANATOMY_ALIGNMENT.md` (`ffc5282`), treated as fixed, unreopened. `PROCESS_DETERMINATION_REPORT_ANATOMY_ALIGNMENT.md` and `CAPABILITY_DEFINITION_REPORT_ANATOMY_ALIGNMENT.md` are fixed inputs, not restated. |
| **Status** | Implementation Planning draft. No code has been produced in preparing this document. |

## 2. Governing Authorization

Authorized files: `apps/console/src/workspace/Overview.tsx`, `apps/console/src/workspace/Reviewing.tsx` only. Eight authorized behaviors per Authorization §5. No transport, Rust bridge, Runtime, Storage, or other crate change authorized. This Plan does not reopen or restate the Authorization's reasoning — it schedules work strictly within it.

## 3. Current Implementation Baseline (re-verified, direct source read)

`Overview.tsx` (53 lines): severity counts + recommendation-exists boolean only; own comment states dimension aggregation was out of prior scope. `Reviewing.tsx` (145 lines): severity-only sort (`severityRank`/`SEVERITY_ORDER`, lines 42–44, 139–144, duplicated verbatim in `Overview.tsx`); collapsed row shows Title/Severity/Summary only (lines 67–75); expansion (lines 77–129) shows `modHealthDimension`/`status` (line 80), `recommendation.action` + `repairSteps` (82–104), `evidence` list including full `content` inline (105–127) — all in one undifferentiated block.

**Empirical fixture check** (`modiq-cli assess apps/console/src-tauri/fixtures/sample-mod`, read-only): produces exactly 2 Findings — `[Informational, EngineeringQuality]` and `[Warning, Compatibility]` — 4 Evidence items, and **both** Findings carry a Recommendation (one plain, one with a single `VersionUpdate` repair step). Confirmed against source (`crates/modiq-rules/src/rules/*.rs`): all 5 current Rules unconditionally construct `RuleOutcome { recommendation: Some(...), .. }` — none ever produces `None`. Confirmed against `RuntimeInvariants.md`/`Finding::new`: a Finding always references ≥1 Evidence (INV-013) — an evidence-less Finding cannot exist today, in this fixture or any other.

## 4. Implementation Strategy

Two independent, additive changes to two independent files, each reorganizing already-rendered or already-available data — no new state, no new props beyond what's already passed, no new imports beyond a possible small local constant. `Overview.tsx` gains dimension presentation (Item 1). `Reviewing.tsx` gains grouping (Items 2–3) and collapsed-layer field promotion (Items 4–8). The two files share no logic today (each defines its own local `SEVERITY_ORDER`) and this Plan does not introduce a shared module — consistent with Authorization §10's "no opportunistic refactor" boundary, since nothing here strictly requires one.

## 5. Behavior-by-Behavior Plan

**Item 1 — Mod Health Dimensional Presentation (Overview.tsx).** *Current state:* not present. *Required behavior:* each dimension represented among current `findings` shown independently (e.g., a per-dimension count), no combined value. *Data available:* `finding.modHealthDimension` on every element of the already-passed `findings` prop. *Location:* `Overview.tsx`, alongside the existing severity-counts block. *Approach:* compute a per-dimension count the same way `counts` is already computed for severity (`filter` + `length`), iterated over the canonical six-dimension order already named in `WORKSPACE_EVOLUTION.md` §6 and already referenced in this console's own prior comment (`Compatibility, Stability, Maintainability, Performance, Structure, EngineeringQuality`) — a fixed local array, mirroring `SEVERITY_ORDER`'s existing pattern exactly. Only dimensions with `count > 0` render (same filter pattern already used for severity). *Verification:* traced against the fixture's real output (both `EngineeringQuality` and `Compatibility` present, both other four absent and correctly not rendered). *Dependency:* none on other items.

**Item 2 — Category Grouping (Reviewing.tsx).** *Current state:* `ordered` is a flat severity-only sort. *Required behavior:* Findings partitioned by `modHealthDimension`. *Data available:* same field as Item 1. *Location:* `Reviewing.tsx`, replacing/extending the existing `ordered` computation. *Approach:* group `findings` by `modHealthDimension` using the same canonical six-dimension order as Item 1 for group sequence (a Planning-level consistency choice, not a new architectural concept — reusing the one order the frozen spec already names); within each group, apply the existing `severityRank` sort unchanged (Item 3). *Verification:* traced against fixture output (one group per dimension present; each internally correctly ordered — trivially true here since each present group holds exactly one Finding, see §8's limitation note). *Dependency:* Item 3 nests inside this structure.

**Item 3 — Severity Ordering Within Category.** *Current state:* correct logic, wrong nesting level (global, not per-group). *Required behavior:* preserve `severityRank`/`SEVERITY_ORDER` exactly, applied inside each dimension group from Item 2. *Approach:* no change to the ordering function itself — only where it's applied (inside each group's own `.sort`, not once over the flat list). *Verification:* see §8 limitation — the fixture cannot currently exercise two same-dimension Findings of different severity; verified instead by direct code inspection that the identical, unmodified `severityRank` function governs intra-group order. *Dependency:* Item 2.

**Item 4 — Finding Collapsed Anatomy.** *Current state:* Title/Severity/Summary visible collapsed; dimension/status inside expansion. *Required behavior:* dimension and status also visible collapsed. *Data available:* `finding.modHealthDimension`, `finding.status` — already present in the same collapsed row's own `finding` object, currently just not rendered there. *Location:* the existing collapsed `<button>` block (lines 60–76), not the expansion block. *Approach:* add these two values to the always-visible row, in whatever compact form is visually consistent with the existing Title/Severity presentation (a Planning-level, not implementation-level, judgment — exact markup left open). *Verification:* traced against fixture — both Findings' dimension and status readable without expansion. *Dependency:* shares the collapsed-row location with Items 5 and 6 — implemented together as one row-anatomy change, not three separate edits.

**Item 5 — Provisional/Final Status Visibility.** *Current state:* buried in expansion. *Required behavior:* visible collapsed (folded into Item 4). *Explicitly noted, per Authorization §5 Item 5:* every Rule today always produces `Final`; this Plan schedules no change that could produce `Provisional`, and none is expected to ever render under this work. *Verification:* traced against fixture — both Findings correctly show "Final". *Dependency:* Item 4 (same row).

**Item 6 — Recommendation Headline.** *Current state:* full Recommendation, including `action`, shown only on expansion. *Required behavior:* `recommendation.action` visible collapsed when `recommendation !== null`; full detail (including `repairSteps`) remains behind expansion, unchanged. *Data available:* `finding.recommendation.action`, already a string. *Location:* collapsed row, conditional on `recommendation !== null` (existing null-check pattern, e.g. `Overview.tsx:29-31`, reused). *Verification:* traced against fixture — both Findings currently have a Recommendation, so the "present" case is directly verifiable; the "absent" case is not reachable by any current Rule (§8 limitation) and is verified instead by direct inspection that the `recommendation !== null` guard is unchanged from its already-correct existing form. *Dependency:* Item 4 (same row); does not touch expansion's own existing Recommendation-detail rendering.

**Item 7 — Evidence Availability.** *Current state:* not indicated until expansion; even then, nothing distinguishes zero evidence from unopened. *Required behavior:* an availability indicator visible collapsed. *Data available:* `finding.evidence.length`, computable from the array already fully delivered. *Location:* collapsed row. *Approach:* use `evidence.length` to drive the availability presentation. Under the current Finding invariant, every Finding has at least one Evidence item; therefore the current real-data verification will exercise the available case only. Do not alter the Finding/Evidence invariant or introduce behavior intended to compensate for its absence. *Verification:* traced against fixture (both Findings have 4/4 evidence items visible in the underlying data — both show "available"); the empty case remains a stated verification limitation (§8), not compensated for by any invariant change. *Dependency:* Item 4 (same row).

**Item 8 — Evidence Progressive Disclosure.** *Current state:* `label`/`source`/`description`/`content` (when present) all rendered together in one block on expansion. *Required behavior:* `label`/`source`/`description` remain visible on expansion (unchanged); `content` requires one further, explicit user action when substantial, using only the already-delivered string. *Location:* the existing per-Evidence `<li>` block (lines 107–126), expansion-scoped, unchanged in trigger. *Approach:* implement a presentation-only further reveal for substantial `content`, using only the already-delivered value. The concrete determination of "substantial" and the corresponding UI mechanism are implementation details to be selected during implementation, provided they remain entirely within `Reviewing.tsx` and do not introduce new data, state outside the component, transport behavior, or architectural concepts. `label`/`source`/`description` remain as currently named. **Explicit boundary, restated:** `description` is not renamed, relabeled, or presented under the word "Explanation" anywhere in this change; Evidence Explanation remains unresolved. *Verification:* fixture's longest `content` value is `"99"` — trivially brief (§8 limitation: the fixture cannot exercise the "substantial content requiring a further reveal" case with real data). Verified instead by direct code inspection that brief content (the only case the fixture produces) continues to display inline exactly as today, and that the further-reveal code path exists and is reachable in principle (traced by construction, not by observed fixture output). *Dependency:* none on other items.

## 6. File-Level Change Plan

| File | Required change | Authorization basis |
|---|---|---|
| `apps/console/src/workspace/Overview.tsx` | Add per-dimension presentation alongside existing severity counts (Item 1). No other change. | Authorization §5 Item 1, §10 |
| `apps/console/src/workspace/Reviewing.tsx` | Replace flat severity sort with dimension-grouped, severity-nested ordering (Items 2–3); add dimension/status/Recommendation-headline/Evidence-availability to the collapsed row (Items 4–7); add a further-reveal step for substantial Evidence `content` within the existing expansion (Item 8). | Authorization §5 Items 2–8, §10 |

No other file. If implementation finds either change cannot be completed without touching a third file, this is **ESCALATION — NOT AUTHORIZED** (§10), not a Plan decision to make silently.

## 7. Data/Transport Confirmation

Every field consumed (`modHealthDimension`, `status`, `recommendation.action`, `evidence`, `evidence.length`, `evidence.content`, `evidence.label`, `evidence.source`, `evidence.description`) is already present in `FindingSummary`/`EvidenceSummary` today, confirmed by direct re-read of `apps/console/src/engine/types.ts` in the governing Capability Definition and unchanged since (`git status` confirms no drift). No field is added, renamed, or restructured by this Plan. `engine/types.ts`, `assessment.rs`, and `engine/index.ts` are not touched.

## 8. Testing and Verification Plan

**Mechanical gates** (per Authorization §11): `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` — expected to pass with zero diff impact, since no Rust file changes; run anyway as a boundary-confirmation gate. `apps/console`'s `npm run build` (`tsc && vite build`) clean. Direct `git diff --stat` confirmation that `apps/sandbox` and every file outside the two authorized files show zero change.

**Real-I/O behavioral verification**, against `apps/console/src-tauri/fixtures/sample-mod`, per case requested:

| Case | Verifiable against current fixture? | Method |
|---|---|---|
| Multiple Mod Health dimensions | **Yes** — fixture produces `EngineeringQuality` and `Compatibility` | Direct trace against real CLI/console output |
| Multiple severity levels | **Yes** — `Informational` and `Warning` present | Direct trace |
| Severity ordering *within* a single dimension (≥2 Findings sharing one dimension) | **No** — fixture's two Findings occupy two different dimensions | **Verification limitation**, stated explicitly: verified by code inspection of the unmodified `severityRank` function's placement, not by observed output |
| Finding *with* a Recommendation | **Yes** — both current Findings have one | Direct trace |
| Finding *without* a Recommendation | **No** — structurally unreachable today; every current Rule always attaches one (confirmed by source, §3) | **Verification limitation**, stated explicitly: verified by code inspection of the unchanged `recommendation !== null` guard, not by observed output |
| Finding *with* Evidence | **Yes** — all Findings carry Evidence | Direct trace |
| Finding *without* Evidence | **No** — structurally unreachable today; INV-013 guarantees ≥1 Evidence per Finding | **Verification limitation**, stated explicitly: verified by code inspection only; no Finding/Evidence invariant change is proposed to close this gap |
| Evidence with substantial `content` | **No** — fixture's longest `content` value is `"99"` | **Verification limitation**, stated explicitly: verified by code inspection that the further-reveal path exists and that brief content still renders inline as today |

No mock data is proposed and no fixture change is proposed to close these gaps — both are outside this Plan's authorized files. These five limitations are reported as-is, per the Authorization's own instruction, rather than worked around.

## 9. Scope / Non-Goals

Not touched, under any circumstance, by this Plan: `assessment.rs`, `engine/types.ts`, `engine/index.ts`, `Workspace.tsx`, `regions/*`, `session/*`, any Rust crate, `apps/sandbox`, Report Identity, `AssessmentSubject`, Report currency, Evidence Explanation, Confidence, C12 historical presentation, Error Experience Alignment, Initiative 1, Initiative 2, GOV-008, or the transport structure. No composite Mod Health score, formula, weighting, or ranking of any kind (Item 1). No renaming of `description` to "Explanation" (Item 8). No Finding/Evidence invariant change (Item 7).

## 10. Escalation Conditions

**ESCALATION — NOT AUTHORIZED** applies, and this Plan does not resolve it internally, if implementation discovers: a need to modify `apps/console/src-tauri/fixtures/sample-mod` to achieve fuller verification coverage (§8's five limitations stand as reported, not closed by fixture changes); a need to touch any file outside §6; a need for a field not already in the current transport; a need to alter the Finding/Evidence invariant (INV-013) or any Rule's recommendation-construction behavior to manufacture a test case; any point where implementing Item 8's disclosure mechanism would require naming, storing, or exposing an "Explanation" concept distinct from `description`.

## 11. Implementation Sequence

**Phase 1 — Overview.tsx (Item 1).** Independent of Reviewing.tsx; no shared state. *Gate:* `tsc`/`npm run build` clean; traced against fixture.

**Phase 2 — Reviewing.tsx grouping (Items 2–3).** Restructures `ordered` into a grouped structure; no rendering change yet beyond what grouping requires structurally. *Gate:* build clean; traced against fixture (groups present, severity order preserved within each).

**Phase 3 — Reviewing.tsx collapsed-row anatomy (Items 4–7).** Extends the collapsed row within the now-grouped structure from Phase 2. *Gate:* build clean; traced against fixture per item.

**Phase 4 — Reviewing.tsx Evidence disclosure (Item 8).** Independent of Phases 2–3's grouping logic; touches only the expansion's Evidence block. *Gate:* build clean; traced against fixture, limitation noted (§8).

**Phase 5 — Final Reverification.** No file change. Full gate re-run (§8 mechanical gates); `git diff --stat` confirms exactly the two authorized files changed; five verification limitations re-confirmed as still limitations, not silently worked around.

Phases 1 and 4 do not depend on Phases 2–3 and could run in either order; sequenced this way only for a clean, single-file-at-a-time review trail.

## 12. Completion Criteria

All eight items' individual completion criteria (Authorization §5) hold against real fixture output where the fixture permits, and against direct code inspection where §8 identifies a limitation; all mechanical gates (§8) pass; `git diff --stat` shows exactly `Overview.tsx` and `Reviewing.tsx` changed; no item in Authorization §9 (Explicit Exclusions) touched. Partial completion (fewer than eight items) is not Engineering Release, per Authorization §13.

## 13. Implementation Report Expectations

Must document: files changed (expected: exactly the two above); each of the eight behaviors' verification outcome, explicitly distinguishing fixture-traced confirmation from code-inspection-only confirmation (§8's five limitations); gate results; any deviation from this Plan or the Authorization, classified as Planning defect vs. implementation defect per this project's standing discipline; explicit confirmation that none of §9's exclusions were touched; explicit restatement that the five verification limitations in §8 remain limitations, not silently resolved by an unauthorized fixture or invariant change.

## 14. Final Planning Determination

**Yes — there is now a sufficiently precise, bounded, implementation-ready plan for the authorized capability.**

All eight behaviors have a stated current state, required behavior, data source, file location, approach, and verification method. The two-file boundary holds with no discovered need to expand it. Five real verification limitations were found and reported rather than worked around, consistent with the Authorization's own instruction — this Plan does not consider that gap a blocker to readiness, since it correctly distinguishes what real-I/O evidence can and cannot show today, and specifies code-inspection as the honest fallback where fixture data cannot reach.

---

**Repository confirmation:** no source file, fixture, or test file created or modified in preparing this Plan. No commit, no push.

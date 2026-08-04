# C1 Implementation Plan — RecommendationStep Presentation

| Property | Value |
|---|---|
| **Document** | C1_IMPLEMENTATION_PLAN.md |
| **Project** | modIQ |
| **Origin** | `C1_IMPLEMENTATION_AUTHORIZATION.md` (`a513102`), treated as fixed, unreopened architecture. `PROCESS_DETERMINATION_C1_RECOMMENDATION_STEP_PRESENTATION.md` (`12a8b89`) and `CAPABILITY_DEFINITION_C1_RECOMMENDATION_STEP_PRESENTATION.md` (`d5b8735`) are fixed inputs, not restated. `FRONTEND_PRESENTATION_IMPLEMENTATION_AUTHORIZATION.md`/`SPRINT23_IMPLEMENTATION_PLAN.md` (Sprint 23) supply this document's own structural precedent. |
| **Status** | Implementation Planning draft. No code has been produced in preparing this document. |

---

## 1. Implementation Objective

Implement exactly what `C1_IMPLEMENTATION_AUTHORIZATION.md` §3 authorizes: extend `apps/console`'s transport DTOs and TypeScript mirrors so a Finding's Recommendation carries its structured `repair_steps` alongside the existing flat `action` text, then render that structure in `Reviewing.tsx`, distinguishable from `action`, within the existing single expansion layer. No `modiq-runtime`, `modiq-rules`, `modiq-storage`, or other `modiq-*` crate change; no new Tauri command; no new navigation or interaction mechanism. Success is defined by the Authorization's own Completion Criteria (§8): a Finding with a non-empty `repair_steps` shows each step's `kind` and `instruction`; a Finding with an empty `repair_steps` presents exactly as today.

---

## 2. Implementation Scope

### 2.1 The reserved design question, decided here

`C1_IMPLEMENTATION_AUTHORIZATION.md` §5 left one question open: what shape does the transport take, given `FindingSummary.recommendation` is currently a flat `Option<String>` (`apps/console/src-tauri/src/assessment.rs:73`), populated solely from `Recommendation::action()` (`assessment.rs:109`)?

**Decision: `recommendation` becomes a nested object, not a sibling array.**

`action` and `repair_steps` are two fields of the *same* `Recommendation` value, already resolved together by the existing per-Finding lookup (`recommendation_for`, `assessment.rs:104–110`, which finds the one applicable `Recommendation` for a `finding_id`). Exposing them as two independent fields — a flat `recommendation: string | null` alongside a separate `recommendationSteps: [...]` array — would present two facts as unrelated when the domain has one Recommendation with two properties, and would require resolving the same `Recommendation` twice. A nested `RecommendationSummary` object, carrying both `action` and `repairSteps`, keeps the one-Recommendation-per-Finding relationship intact and mirrors `EvidenceSummary`'s own established shape: a nested struct as the value of a per-Finding field, populated by a `From` conversion identical in form to the one already governing `EvidenceSummary` (`assessment.rs:48–59`).

This changes `recommendation`'s own field type — from `string | null` to an object — which is a breaking change to this internal transport's existing shape, not merely an addition. This is not new to C1: `types.ts`'s own header comment (`apps/console/src/engine/types.ts:23–24`) records that Sprint 22 already replaced `FindingSummary`'s prior flat `description` field with `title`/`summary`, a shape change of the same kind, made without a separate governance event — `ReportSummary` remains, by `assessment.rs`'s own header comment, "provisional, not final or stable... held open pending GOV-008," and this transport has exactly one consumer (`apps/console` itself), not an external or public API under `GOVERNANCE.md`'s Public API Policy.

### 2.2 Type design

`apps/console/src-tauri/src/assessment.rs`:

```rust
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationStepSummary {
    kind: String,
    instruction: String,
}

impl From<&RecommendationStep> for RecommendationStepSummary {
    fn from(step: &RecommendationStep) -> Self {
        Self {
            kind: format!("{:?}", step.kind()),
            instruction: step.instruction().to_string(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationSummary {
    action: String,
    repair_steps: Vec<RecommendationStepSummary>,
}

impl From<&Recommendation> for RecommendationSummary {
    fn from(recommendation: &Recommendation) -> Self {
        Self {
            action: recommendation.action().to_string(),
            repair_steps: recommendation
                .repair_steps()
                .iter()
                .map(RecommendationStepSummary::from)
                .collect(),
        }
    }
}
```

`kind` uses `format!("{:?}", ...)`, identical to `severity`/`mod_health_dimension`/`status`'s own existing convention (`assessment.rs:117,120–121`) — not a new representation choice. `FindingSummary.recommendation` becomes `Option<RecommendationSummary>` (was `Option<String>`); `recommendation_for`'s own closure (`assessment.rs:104–110`) changes its final `.map(...)` from `|recommendation| recommendation.action().to_string()` to `RecommendationSummary::from`, otherwise unchanged. The `use modiq_runtime::assessment::{...}` import gains `Recommendation, RecommendationStep` — both already re-exported at `modiq_runtime::assessment`'s own module root (`crates/modiq-runtime/src/assessment/mod.rs:40,43`), identical in form to the existing `Evidence, FindingId` imports.

`apps/console/src/engine/types.ts`:

```ts
export interface RecommendationStepSummary {
  kind: string;
  instruction: string;
}

export interface RecommendationSummary {
  action: string;
  repairSteps: RecommendationStepSummary[];
}
```

`FindingSummary.recommendation` becomes `RecommendationSummary | null` (was `string | null`). `types.ts`'s own module doc comment does not reference `RepairRecipe` or structured steps anywhere (confirmed directly — no match) and requires no correction, unlike Sprint 23's `Overview.tsx` comment.

`apps/console/src/workspace/Reviewing.tsx` (`Reviewing.tsx:82–86`) changes from:

```tsx
{finding.recommendation && (
  <p className="text-sm text-foreground">
    Recommendation: {finding.recommendation}
  </p>
)}
```

to rendering `finding.recommendation.action` where the string previously appeared, followed by `finding.recommendation.repairSteps`, each rendered as `kind`/`instruction`, only when the array is non-empty — within the same `<div>` this Finding's expansion already renders into, no new expansion level, mirroring the existing conditional-rendering pattern already used for `evidence.content` (`Reviewing.tsx:102–106`).

---

## 3. Crate / File Impact

| File | Required change | Authorization basis |
|---|---|---|
| `apps/console/src-tauri/src/assessment.rs` | New `RecommendationStepSummary`/`RecommendationSummary` structs and `From` impls (§2.2); `FindingSummary.recommendation` retyped to `Option<RecommendationSummary>`; `recommendation_for` closure updated; import gains `Recommendation`, `RecommendationStep`. | Authorization §3, conditioned on §3's "existing, already-public Runtime getter" constraint. |
| `apps/console/src-tauri/fixtures/sample-mod` | Extended with real content sufficient to trigger `VersionCompatibilityRule`'s existing `repair_steps` population — exact file content is Phase 1's own task, not designed here (§4, §5). | Authorization §7 Verification Gates ("console's existing real-fixture tests extended... consistent with... Real-I/O Testing Discipline"). |
| `apps/console/src/engine/types.ts` | `RecommendationStepSummary`/`RecommendationSummary` interfaces mirrored (§2.2); `FindingSummary.recommendation` retyped. | Authorization §3. |
| `apps/console/src/workspace/Reviewing.tsx` | Render `finding.recommendation.action` and, when non-empty, `finding.recommendation.repairSteps` (§2.2). | Authorization §3 ("introducing no new navigation level"). |
| `apps/console/src/workspace/Overview.tsx` | **No change required — verified compatible.** `Overview.tsx:30` reads `finding.recommendation !== null` (a bare null-check, no string operation); this remains valid and semantically correct once `recommendation` is `RecommendationSummary \| null`. Confirmed directly against current source, not assumed. | N/A — verified, not authorized work. |
| `modiq-runtime`, `modiq-rules`, `modiq-knowledge`, `modiq-collection`, `modiq-engine`, `modiq-report`, `modiq-storage`, `modiq-cli` | **No change.** | Authorization §4: "Not participating." |
| `apps/sandbox` | **No change** — separate workspace, out of scope. | Authorization §4. |

No file outside this table is expected to require modification. If one does, implementation should stop and report it rather than proceed — matching Sprint 23's own precedent for this exact table.

---

## 4. Recommended Phase Breakdown

**Phase 1 — Transport (Rust) and Fixture Extension.**
*Objective:* introduce `RecommendationStepSummary`/`RecommendationSummary`, retype `FindingSummary.recommendation`, update `recommendation_for`; extend `fixtures/sample-mod` with content that causes a real assessment run to produce a `Recommendation` with non-empty `repair_steps` (confirmed necessary this session: the current fixture, run directly, produces only a generic Evidence-collection Finding whose Recommendation carries no `repair_steps` — `VersionCompatibilityRule` never fires against it today).
*Files:* `apps/console/src-tauri/src/assessment.rs`, `apps/console/src-tauri/fixtures/sample-mod`.
*Completion criteria:* `cargo check -p console` clean; existing 5 tests still pass, updated only where the `recommendation` field's own type change requires it (`submit_assessment_from_path`'s callers are unaffected — no test currently asserts `recommendation`'s exact value); a new assertion confirms at least one Finding's `recommendation.repairSteps` is non-empty against the extended fixture, and a new assertion confirms `recommendation.action` remains a non-empty string wherever `recommendation` is present — exercising both halves of the new structure immediately, not deferred to a later phase.

**Phase 2 — Transport (TypeScript).**
*Objective:* mirror Phase 1's types in `types.ts`.
*Files:* `apps/console/src/engine/types.ts`.
*Completion criteria:* `tsc` (part of `npm run build`) clean; no runtime behavior change yet, since nothing consumes the new shape until Phase 3.

**Phase 3 — Presentation.**
*Objective:* render `action` and, when present, each `repairSteps` entry's `kind`/`instruction` in `Reviewing.tsx`, distinguishable from `action`, within the existing expansion.
*Files:* `apps/console/src/workspace/Reviewing.tsx`.
*Completion criteria:* `npm run build` clean; manual verification against the extended fixture that both the empty-`repairSteps` case (existing fixture behavior, unchanged) and the non-empty case (Phase 1's new fixture content) render correctly and distinguishably. This gate is intentionally interpretive, not automated — restated from Sprint 23's own precedent: `apps/console` still contains no component-testing framework (`package.json`'s `devDependencies` confirmed this session: no Vitest, React Testing Library, Jest, or Playwright), and introducing one is outside this Authorization's scope.

**Phase 4 — Final Reverification.**
*Objective:* full repository reverification and regression confirmation only.
*Files:* None — verification-only phase.
*Completion criteria:* `cargo test -p console` — all tests pass, including Phase 1's new assertions, no deletions; root workspace `cargo fmt --check`/`check --workspace`/`test --workspace` unaffected; `apps/sandbox/src-tauri` unaffected (9/9); `npm run build` clean.

This ordering matches Sprint 21's and Sprint 23's own precedent: Rust transport before TypeScript before presentation, no phase rendering data the prior phase has not yet made available.

---

## 5. Testing Strategy

- Phase 1 requires the fixture itself to change, not only the assertions against it — a departure from Sprint 22/23's own Testing Strategy, where the existing fixture already happened to exercise every new field. This is named explicitly rather than assumed: the exact fixture content (which file, what version mismatch) is Phase 1's own implementation decision, not specified by this Plan, but the requirement that it exists — a real, checked-in scenario that causes `VersionCompatibilityRule` to populate `repair_steps` — is a Phase 1 completion criterion (§4).
- New assertions, added to or alongside the existing `every_finding_carries_non_empty_severity_title_summary_dimension_and_status`-style tests, confirm: at least one Finding's `recommendation.repairSteps` is non-empty against the extended fixture; every present `recommendation.action` remains non-empty. Both exercise Phase 1's own new fields directly against real, no-mocking assessment output — matching this crate's unbroken real-fixture-only testing convention.
- Phase 2: no new test required — `tsc`'s own type-checking is this phase's own verification.
- Phase 4: no new test authored here — re-runs Phase 1's assertions unmodified alongside full-repository and Sandbox reverification.
- No existing test is deleted at any phase.

---

## 6. Engineering Risks

1. **The existing fixture does not exercise the non-empty case.** Confirmed directly this session by running the CLI against `fixtures/sample-mod`: it produces one generic Evidence-collection Finding, no `VersionCompatibilityRule` Finding. Phase 1 must add real content, not assume the existing fixture already covers this — the single largest risk this Plan identifies that Sprint 23's own planning did not have to address, since Sprint 22's fields were already present in that fixture's output.
2. **`recommendation`'s own type change is a breaking change to this internal transport**, not an addition. Mitigated by precedent (Sprint 22's `description` → `title`/`summary` replacement) and by the transport's own explicitly provisional status. A repository-wide grep found two call sites, not one: `Reviewing.tsx` (requires the Phase 3 change described above) and `Overview.tsx:30` (`finding.recommendation !== null`, a bare null-check verified compatible with the new type as-is — see §3). Both are named here so neither is a silent surprise during implementation.
3. **`RecommendationStepKind`'s five values have no existing visual precedent**, the same category of risk Sprint 23 recorded for `ModHealthDimension`. Phase 3 is authorized for plain-text presentation only; any grouping, iconography, or color treatment is out of scope and must not be improvised mid-phase.
4. **Scope creep into `RepairRecipeReference`'s own unresolved status.** Nothing in this work should attempt to resolve or interpret `RepairRecipeReference`; only `repair_steps` is presented.

---

## 7. Verification Gates

Before progressing past each phase:

- **After Phase 1:** `cargo fmt --check`, `cargo check -p console`, `cargo test -p console` all clean. The extended fixture is checked in; the new `repairSteps`/`action` assertions pass against it; no existing test is removed.
- **After Phase 2:** `tsc` clean (via `npm run build` or standalone).
- **After Phase 3:** `npm run build` clean; manual fixture-driven verification of both the empty- and non-empty-`repairSteps` cases.
- **After Phase 4:** full root workspace (`cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace`) and `apps/sandbox/src-tauri` both reverified clean and unaffected.

No phase begins before the prior phase's gate passes in full.

---

## 8. Implementation Completion Criteria

- A Finding whose Recommendation carries a non-empty `repair_steps` shows each step's `kind` and `instruction` in `apps/console`'s Reviewing experience, distinguishable from `action`.
- A Finding whose Recommendation carries an empty `repair_steps` presents exactly as today.
- No field was added to any transport DTO without an existing, already-public Runtime getter backing it.
- No `modiq-runtime`, `modiq-rules`, `modiq-knowledge`, `modiq-collection`, `modiq-engine`, `modiq-report`, `modiq-storage`, or `modiq-cli` file is touched.
- `apps/sandbox` is untouched.
- `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` clean at the root; `apps/sandbox/src-tauri` clean and unaffected; `console` clean with test count grown only by net-new assertions, no deletions; `npm run build` clean.
- No item named in `C1_IMPLEMENTATION_AUTHORIZATION.md` §5 (Explicit Exclusions) is touched, added, or implied.

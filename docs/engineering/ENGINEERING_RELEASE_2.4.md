# Engineering Release 2.4

| Property | Value |
|---|---|
| **Release** | 2.4 |
| **Documentation Release** | 2.1 (Frozen — unchanged this capability; no specification amendment) |
| **Milestone** | C14 complete (Operational Assessment Console) — `apps/console`'s Workspace now exposes `.zip` mod archive intake alongside its existing directory intake, and a return-to-Intake affordance ("Assess another mod") that starts a new, independent Assessment, both using the existing, unmodified `submitAssessment`/`AssessmentService::execute_from_assessment_input` path |
| **Scope** | The work `IMPLEMENTATION_AUTHORIZATION_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md` §3 authorizes: `.zip` intake via the existing dialog API, preserved directory intake, a local-state-only return-to-Intake affordance, independent second-Assessment execution via the already-existing path, and a documentation correction in `Reviewing.tsx` — no more |
| **Predecessor** | `docs/engineering/ENGINEERING_RELEASE_2.3.md` (C4) |
| **Governing ADRs** | None new |
| **Governing Documents** | `docs/engineering/CAPABILITY_DEFINITION_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md`, `docs/engineering/PROCESS_DETERMINATION_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md`, `docs/engineering/IMPLEMENTATION_AUTHORIZATION_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md`, `docs/engineering/IMPLEMENTATION_PLAN_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md`, `docs/engineering/IMPLEMENTATION_REPORT_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md` |

---

## 1. Executive Summary

C14 closes the gap named directly by its own Capability Definition: `AssessmentService::execute_from_assessment_input` has routed `.zip` archives to `ArchiveCollector` since Sprint 4, fully tested end-to-end, but `apps/console`'s Workspace could only ever hand it a directory path — its intake dialog was hardcoded to directory mode. Separately, once Reviewing was reached, the console had no way back to Intake at all, by explicit prior design (Sprint 21). Neither gap required any new backend capability; both were exposed, not created, by this release.

`Workspace.tsx` now offers a second intake affordance calling the same `@tauri-apps/plugin-dialog` `open()` import in file mode with a `.zip` filter, submitting through the same, unmodified `submitAssessment(inputPath)` call the existing directory affordance already uses. An "Assess another mod" control, reachable once Reviewing is active, resets exactly the four local `useState` values this component owns and returns to a fresh Intake presentation — performing zero Tauri calls and touching no prior Assessment. `Reviewing.tsx`'s own doc comment, which previously and inaccurately attributed the absence of any return path to Initiative 2, was corrected to describe the real affordance and restate the distinction.

This is the fifth post-foundation capability structured through this repository's own lightweight lifecycle (Capability Definition → Process Determination → Implementation Authorization → Implementation Plan → Implementation → Implementation Report), and — like C1, C3, and C4, and unlike C2 and C12 — required no Architecture Evaluation or Architectural Resolution. The Process Determination's own two-prong test, independently re-derived rather than assumed from precedent, found every one of six named questions resolved to already-implemented, already-tested mechanism or an already-shipped, directly analogous local-state-reset precedent already in this exact codebase (`Console.tsx`'s own `signOut`).

C14's own repository history diverges from C1–C4 precedent in one respect, named plainly here rather than smoothed over: **this release combines the implementation and all five lifecycle documents, plus this release document itself, into a single commit**, rather than the one-commit-per-lifecycle-stage granularity C1 through C4 each established. This is recorded in §6 below as an explicit, directed deviation for this capability, not a new default convention.

---

## 2. Authorization and Planning Summary

C14 followed the same five-document lineage C1/C3/C4 established, with one addition: a Process Determination re-verifying, rather than assuming, the Capability Definition's own preliminary no-Architecture-Evaluation conclusion, sharpening one of its findings in the process (the Tauri dialog `allow-open` permission is scoped at the command level, not per-parameter — confirmed directly against `tauri-plugin-dialog-2.7.2`'s own permission definitions — so no capability-file change was ever required, not merely unlikely).

The Process Determination independently investigated six named questions: whether ZIP selection requires a new boundary/transport mechanism (no); whether repeated independent execution requires a Runtime lifecycle change (no — re-confirmed directly against `AssessmentService`'s own `separate_executions_are_independent_and_deterministic` test and `INV-012`'s exact text); whether resetting `Workspace.tsx`'s local state carries architectural consequence (no — directly analogous to `Console.tsx`'s own already-shipped `signOut`); whether "assess another mod" could implicate Initiative 2 (no — Initiative 2's own Central Architectural Question, re-read directly, concerns same-Assessment supplementation only); whether the existing dialog capability is sufficient (yes, confirmed more conclusively than originally claimed); and whether any DTO or command change is necessary (no).

The Implementation Authorization fixed the exact scope: `.zip` intake and preserved directory intake in `Workspace.tsx`; a local-state-only "Assess another mod" reset; independent second-Assessment execution via the unmodified existing path; and a `Reviewing.tsx` documentation correction with **no behavioral change** — explicitly not authorizing any Tauri capability, DTO, transport, or Rust change. The Implementation Plan translated this into five concrete steps and, during its own preparation, discovered and formally flagged a real documentation defect: all three prior lifecycle documents had cited `apps/console/src/workspace/Workspace.tsx`, a path that does not exist — the real file is `apps/console/src/regions/Workspace.tsx`. The Plan proceeded against the corrected, verified path and named the correction explicitly rather than silently substituting it; the historical documents were left unmodified, and the Implementation Report (§3 there) is the formal record of the correction.

---

## 3. Implementation Summary

One implementation pass across the two authorized files, matching the Plan's own five-step scope, with no phase boundary required (unlike C4's two-phase DTO extension) since both files' changes are independent of each other and each is small:

1. **`apps/console/src/regions/Workspace.tsx`.** A shared `submitPath(path: string)` helper was factored out of the pre-existing `chooseFolder` body — an implementation choice the Plan itself explicitly pre-authorized, not an unplanned refactor. `chooseFolder` now delegates to it, behaviorally unchanged. A new `chooseArchive` handler calls the same `open` import in file mode (`{ multiple: false, filters: [{ name: "Mod Archive", extensions: ["zip"] }] }`) and delegates to the same `submitPath`. A new `resetToIntake` function sets `status`, `selectedPath`, `error`, and `report` back to their initial values and is wired to a new "Assess another mod" button, rendered alongside (not inside) the existing `<Reviewing>` element via a fragment — requiring no prop or signature change to `Reviewing.tsx`. The doc comment was extended to describe both additions and to state the Initiative 2 distinction directly at the point the reset logic is defined.
2. **`apps/console/src/workspace/Reviewing.tsx`.** The doc comment's final paragraph — previously claiming no affordance returns to Intake, attributed to Initiative 2 — was replaced with an accurate description of the real affordance `Workspace.tsx` now renders, and a restatement of the distinction. Confirmed by direct diff inspection: no line outside this comment block changed.

No third file participated. No Rust, DTO, transport, Tauri command, or Tauri capability file was touched at any point.

---

## 4. Repository Impact

| Area | Change |
|---|---|
| `apps/console/src/regions/Workspace.tsx` | +66/−17 |
| `apps/console/src/workspace/Reviewing.tsx` | +8/−3 (comment-only, confirmed by diff — no JSX/prop/state line changed) |
| `crates/modiq-runtime`, `modiq-rules`, `modiq-collection`, `modiq-versioning`, `modiq-engine`, `modiq-report`, `modiq-storage`, `modiq-knowledge` | **Unmodified** — confirmed via empty diff |
| `apps/console/src-tauri/src/assessment.rs`, `apps/console/src-tauri/src/lib.rs` | **Unmodified** — Tauri command signature, `ReportSummary`/`FindingSummary`/`EvidenceSummary`/`RecommendationSummary`/`RecommendationStepSummary` DTOs untouched |
| `apps/console/src/engine/index.ts`, `apps/console/src/engine/types.ts` | **Unmodified** — `submitAssessment(inputPath: string): Promise<ReportSummary>` unchanged in signature and every field |
| `apps/console/src-tauri/capabilities/default.json` | **Unmodified** — confirmed not required (§2; `allow-open` scoped at the command level, already covers file-mode selection) |
| `apps/console/src/workspace/Overview.tsx` | **Unmodified** — C13's presentation unaffected |
| `apps/sandbox`, `modiq-cli` | **Unmodified** — separate consumers, untouched |
| Every `Cargo.toml` / `Cargo.lock` | **Unmodified** — zero new dependency edge; no new package added to `apps/console/package.json` either — `@tauri-apps/plugin-dialog` was already a dependency |
| Governance Register | Unaffected — GOV-002, GOV-008 remain exactly as they were; Initiative 2's Central Architectural Question remains "Requires Governance Reconciliation," untouched |
| ADRs | None new |
| New documents | `CAPABILITY_DEFINITION_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md`, `PROCESS_DETERMINATION_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md`, `IMPLEMENTATION_AUTHORIZATION_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md`, `IMPLEMENTATION_PLAN_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md`, `IMPLEMENTATION_REPORT_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md`, this release document |

Total source impact: 2 files changed, 74 insertions, 20 deletions (`git diff --numstat`, re-confirmed fresh at this release's own drafting).

---

## 5. Validation Status

```
Root workspace:
cargo fmt --check       → clean
cargo check --workspace → clean
cargo test --workspace  → clean, 0 failures, full suite unaffected (no Rust file touched)
  console_lib 7 · modiq_cli 27 · modiq_collection 70 · modiq_engine 23(+3 e2e) · modiq_knowledge 5
  modiq_report 3 · modiq_rules 46 · modiq_runtime 90 · modiq_storage 29 · modiq_versioning 4

apps/console:
tsc && vite build       → clean, 147 modules transformed, built in 691ms

git diff --check        → clean, no whitespace errors

Real-I/O Assessment Verification (real material, no mock data, no fixture):
  Real extracted mod directory  → real report: 45 Evidence, 1 Finding (Informational/EngineeringQuality)
  Real .zip mod archive         → real report: 128 Evidence, 2 Findings (Informational/EngineeringQuality,
                                   Warning/Compatibility — real "descVersion 94" unrecognized-version content)
  Independent sequence, both directions:
    Hemp .zip → Assess another mod → fresh Intake → Hashy → Hashy Reviewing (Hashy-only content confirmed)
    Hashy → Assess another mod → fresh Intake → Hemp .zip (also exercised)
  C13 presentation (dimension grouping, expansion, Evidence access, Recommendation visibility) observed
    functioning, unregressed, for both reports

Dependency-edge check (all Cargo.toml/Cargo.lock, package.json): zero matches
Governance check: GOVERNANCE.md — no entry touched; Initiative 2 Central Architectural Question — textually
  unchanged, "Requires Governance Reconciliation"
```

All figures re-verified fresh at this release's own drafting, against the same working-tree diff the Implementation Report describes.

---

## 6. Engineering Observations

Recorded as evidence only — no process change, new governance mechanism, or Governance Register item follows from any observation below.

**Observation 1 — a documentation-location defect was carried through three prior lifecycle documents and is formally corrected only here and in the Implementation Report, not by rewriting history.** `CAPABILITY_DEFINITION_C14...md`, `PROCESS_DETERMINATION_C14...md`, and `IMPLEMENTATION_AUTHORIZATION_C14...md` each cite `apps/console/src/workspace/Workspace.tsx` — a path that has never existed. Direct repository inspection (`find apps/console/src -iname "Workspace.tsx"`), performed during Implementation Planning, established the real file as `apps/console/src/regions/Workspace.tsx`. This was a location error, not a scope error — every description of the file's contents and role in all three documents unambiguously identified the one real Workspace file in this codebase — and no incorrect `workspace/Workspace.tsx` file was ever created at any point. The three historical documents remain textually unmodified, preserving the record of the error, per this project's own standing practice of naming a discovered documentation defect rather than silently absorbing it.

**Observation 2 — unlike C1 through C4, this release combines every lifecycle document, the implementation, and this release document itself into a single commit, per explicit Product Owner direction, not a new default convention.** C1 through C4 each landed their five (or more) lifecycle documents as separate commits, followed by a separate implementation commit (one-commit-per-phase for C2/C3, phases combined for C1/C4), followed by a separate Implementation Report commit, followed by a final, separate, docs-only Engineering Release commit. C14's own release instead stages exactly eight files — the two authorized source files, the five C14 lifecycle documents, and this document — as one Engineering Release commit. This is recorded here as a fact about this release's own history, not normalized into the prior shape; it reflects an explicit, deliberate instruction at this release stage, not an oversight, and carries no correctness consequence — every one of the eight files was independently authored, reviewed, and verified in its own right before being staged together.

**Observation 3 — the shared `submitPath` helper is a Plan-authorized implementation choice, not an unplanned refactor.** `IMPLEMENTATION_PLAN_C14...md` §4 explicitly named this as an available option, conditioned on touching no file beyond `Workspace.tsx`, adding no new file, and introducing no new state — all three conditions are satisfied, confirmed directly against the diff. This is noted here only because a smaller, more literal reading of "smallest changes necessary" might mistake it for scope drift; it is not — it was named as acceptable before implementation began.

---

## 7. Outstanding Limitations / Reserved Responsibilities

Named, not resolved, exactly as `CAPABILITY_DEFINITION_C14...md`'s own Explicit Exclusions already name them, plus three UX observations the Implementation Report recorded during manual acceptance — none is created or newly recommended by this release:

- **Initiative 2 (Reentrant Assessment Lifecycle)** — same-Assessment supplementation remains "Requires Governance Reconciliation," untouched, unadvanced. "Assess another mod" creates a new, independent Assessment; it is not, and was never implemented as, a mechanism for a completed Assessment to accept new material.
- **GOV-002, GOV-008** — untouched.
- **Finding disclosure discoverability** — Finding cards are fully clickable and expand correctly (single-open behavior), but carry no visual affordance (e.g., a chevron) indicating expandability. This is a pre-existing C13 presentation characteristic, not introduced or touched by C14 — `Reviewing.tsx`'s expansion markup is unmodified. Recorded as a UX follow-up candidate only.
- **Assessment history/retrieval** — the console provides no way to return to a previously completed assessment once "Assess another mod" has been chosen. An observed limitation of the current console UI, not a claim about Runtime or Storage deletion; explicitly outside C14's own scope (Capability Definition's exclusion of "history UI").
- **Progress-state visibility** — the existing `"Submitting…"` state was not meaningfully observable during acceptance due to fast completion. C14 explicitly did not expand the assessment-progress experience; this is an observation, not a deviation.
- **Cancellation** — no formal, dedicated cancellation test (dismissing the native dialog without selecting anything) was performed during acceptance. Recorded honestly as untested, not as a defect; no repository evidence indicates a problem.
- `Report Identity`, `Confidence`, cross-mod dependency resolution (C5), C12 presentation, Knowledge architecture, Lua analysis, and every other named-out-of-scope item — untouched.

---

## 8. Technical Director Assessment

**C14 is a clean instance of this repository's own "expose, don't extend" pattern, verified by real execution against real, non-synthetic material rather than fixtures alone.** Every mechanism this capability activates — `ArchiveCollector` routing, independent repeated Assessment execution, the Tauri dialog's unscoped `allow-open` permission — already existed, already public, already tested, before this capability began; nothing was invented at the engine or transport layer to support it. The Process Determination's own re-verification sharpened, rather than merely repeated, the Capability Definition's preliminary finding on the one point that mattered most for implementation risk (the dialog capability question), which is exactly the kind of independent-verification value this repository's Process Determination stage exists to provide.

**Verification included two real FS25 mods, exercised through both intake shapes and the full independent-sequence return path, in both directions.** This is a stronger real-I/O standard than a fixture-only pass would provide, and it directly demonstrated the property the capability's own success criteria required — the second report contained no residue of the first — rather than merely arguing it from code inspection alone. One honest verification gap was named, not worked around: full interactive GUI click-through (the native file dialog and button clicks in an actually-running window) could not be performed by the implementing session, which had no GUI-automation tool available; this was substituted with real backend execution against the identical unmodified path both affordances call into, plus direct inspection of the deterministic React state transitions involved, and stated as a limitation rather than a false claim of completeness.

**Risk: low.** No architectural document was reopened. No Runtime, Storage, Engine, transport, or Tauri-command boundary was crossed. The one genuine deviation from prior release practice — combining the entire lifecycle into a single commit (Observation 2) — was directed explicitly and carries no correctness risk of its own; every constituent file was independently produced and reviewed before being combined. The documentation-location defect (Observation 1) was found, named, and corrected without ever producing an incorrect file or reopening capability scope.

---

## 9. Final Release Recommendation

**C14 is complete.** All five authorized behaviors are implemented and confirmed by direct diff inspection; all required automated gates pass clean; real, non-mock manual acceptance was performed against two real FS25 mods in both intake shapes and both directions of the independent-assessment sequence; C13's existing presentation was directly observed unregressed; the scope audit confirms exactly the two authorized source files changed; the Initiative 2 distinction holds by construction, not merely by description; and the one documentation defect discovered during this lifecycle was named and formally corrected rather than silently fixed.

**Recommend:** Product Owner final approval of this release.

---

## Repository Status

C14 is complete. The root workspace is clean at full suite green (§5); `apps/console`'s build is clean at 147 modules. No Governance Register item or ADR resulted from this capability; GOV-002 and GOV-008 remain exactly as they were; Initiative 2's Central Architectural Question remains "Requires Governance Reconciliation," untouched. As of this document's own drafting, HEAD remains `06e48b3` on `feature/runtime-implementation`, and the entire C14 lifecycle — Capability Definition, Process Determination, Implementation Authorization, Implementation Plan, Implementation, Implementation Report, and this release document — exists only as uncommitted working-tree state, staged together into a single Engineering Release commit per explicit Product Owner direction (§6, Observation 2), not yet made as of this document's own drafting. `PROJECT_STATUS.md`, `CHANGELOG.md`, and `ENGINEERING_LOG.md` are explicitly deferred to a separate Repository Closeout step and are not touched by this release. The repository is ready for its next engineering objective, not yet scoped by this document.

---

## Repository Timeline

```
Engineering Release 2.3 — C4 complete
        ↓
Capability Definition C14 — Operational Assessment Console
        ↓
Process Determination C14 — matches C1's/C3's/C4's path; no Architecture
Evaluation or Architectural Resolution required; sharpens the dialog-
permission finding beyond the Capability Definition's own claim
        ↓
C14 Implementation Authorization
        ↓
C14 Implementation Plan — discovers and formally flags the
regions/Workspace.tsx vs. workspace/Workspace.tsx documentation defect
        ↓
C14 Implementation — Workspace.tsx (.zip intake, submitPath, resetToIntake,
"Assess another mod") and Reviewing.tsx (comment correction only)
        ↓
C14 Implementation Report — real-I/O acceptance against two real FS25 mods,
both intake shapes, both directions of the independent-assessment sequence;
formal correction record for the documentation defect
        ↓
Engineering Release 2.4 — C14 complete — this document
        ↓
[pending] Single combined Engineering Release commit — implementation +
five lifecycle documents + this release document (Observation 2)
```

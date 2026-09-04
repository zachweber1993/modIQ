# Capability Definition — C14: Operational Assessment Console

| Property | Value |
|---|---|
| **Document** | CAPABILITY_DEFINITION_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md |
| **Project** | modIQ |
| **Type** | Capability Definition — defines the capability only. Not a Process Determination, Architecture Evaluation, Architectural Resolution, Implementation Authorization, or Implementation Plan. |
| **Origin** | Direct mission authorization from the Product Owner: make the existing, already-built assessment pipeline operationally usable through `apps/console` without touching the terminal. Not drawn from `CAPABILITY_PORTFOLIO_ASSESSMENT.md`'s numbered candidate sequence — this is a console-usability capability, not a backend-capability-portfolio item. |
| **Prepared by** | Lead Engineer (Sonnet 5), on `feature/runtime-implementation` |
| **Repository baseline** | `feature/runtime-implementation`, HEAD `06e48b3` ("feat: align report anatomy presentation" — C13 complete), working tree clean |
| **Status** | **Approved by the Product Owner. Authorized to advance to Process Determination only. No implementation, Architecture Evaluation, Architectural Resolution, or Implementation Authorization is made by this document.** |

---

## 1. Repository Verification

Verified directly against source this session, not carried forward from memory:

| Check | Result |
|---|---|
| Branch / HEAD | `feature/runtime-implementation` @ `06e48b3`, clean |
| `AssessmentService::execute_from_assessment_input` | Confirmed in `crates/modiq-engine/src/engine/assessment_service.rs`: routes any path ending `.zip` (case-insensitive) to `ArchiveCollector`, everything else to `EvidenceCollector`; both paths fully covered by passing tests, including real, well-formed ZIP archives constructed and read end-to-end in-test. No backend gap exists. |
| Tauri command surface | `apps/console/src-tauri/src/assessment.rs` exposes exactly one command, `submit_assessment(input_path: String)`, calling `execute_from_assessment_input` directly and mapping the result to `ReportSummary`. Accepts any path string already — directory or archive — with no branching on input shape at the Tauri layer. |
| Console intake | `apps/console/src/workspace/Workspace.tsx` calls `open({ directory: true, multiple: false })` — folder-only. No code path offers `directory: false` or a file-extension filter. This is the only place ZIP intake is actually blocked. |
| Console return-to-Intake | `Reviewing.tsx`'s own doc comment: *"No affordance returns to Intake: Sprint 21 does not implement reentrancy or supplementation (Initiative 2)."* Once `report`/`selectedPath` are set in `Workspace.tsx`, no code path resets them. |
| C13 scope | `IMPLEMENTATION_REPORT_REPORT_ANATOMY_ALIGNMENT.md` confirms exactly two files touched (`Overview.tsx`, `Reviewing.tsx`), zero transport/Rust/Runtime change. Intake and navigation were untouched by C13 and remain exactly as Sprint 21 left them. |
| Initiative 2 status | `INITIATIVE_2_ARCHITECTURAL_RESOLUTION.md`: Question 1 (terminal completion vs. reentrancy) is dispositioned **Requires Governance Reconciliation** — an unresolved conflict between `DataModel.md` and `THE_ASSESSMENT_REPORT.md` over whether a *single, completed* Assessment may later accept more evidence. This governs same-Assessment supplementation only. |
| Tauri dialog capability | `apps/console/src-tauri/capabilities/default.json` grants `dialog:default`, already exercised today for directory selection. |
| Real test material | Two real FS25 mod `.zip` files exist on this machine's actual FS25 mods folder (`FS25_HH_HempProdSystem.zip`, `Hashy_01Silv.zip`), usable directly for real-I/O verification — not fixtures manufactured for this capability. |

---

## 2. Capability Statement

**C14 exposes assessment capability that already exists end-to-end in the backend, through the console, for both supported intake shapes.** `AssessmentService::execute_from_assessment_input` already assesses a directory or a `.zip` archive identically and deterministically; `apps/console`'s Tauri bridge already calls it and already renders a full, C13-aligned Reviewing experience from its result. The only things missing are: (a) a way for the console's file picker to hand it a `.zip` path, and (b) a way to return the console to a fresh Intake state afterward so a second, independent mod can be assessed. Nothing about assessment *semantics* is introduced — this capability exposes existing behavior, it does not extend it.

---

## 3. Governance Distinction — "Assess Another Mod" Is Not Initiative 2

Stated explicitly because `Reviewing.tsx`'s own current comment invites this confusion, and because the mission names Initiative 2 as out of scope:

- **Initiative 2** ("Reentrant Assessment Lifecycle") concerns whether *one, already-completed* Assessment can accept *additional evidence* and have its *same* Report continue to reflect it — Workspace Evolution's "Assessing, recurring on top of Reviewing," §3's "Updated" marker, and `INV-012`'s own terminality. This remains unresolved (**Requires Governance Reconciliation**) and is not touched, implemented, or reopened by C14 in any way.
- **"Assess another mod"** is the ordinary case of a user choosing a *second, unrelated* Assessment Subject after finishing the first. Mechanically, this is nothing more than a second, independent call to `execute_from_assessment_input` — the same call the console already makes for the first mod, already proven independent and deterministic by `AssessmentService`'s own test suite (`separate_executions_are_independent_and_deterministic`). No Runtime state is reopened; no completed `Assessment` is mutated; no persisted report is touched. The only state involved is `Workspace.tsx`'s own local, ephemeral React state (`report`, `selectedPath`, `status`, `error`), reset to its initial values.
- **Consequence for this capability:** C14 requires no position on Initiative 2's own open governance question, and creates no new fact bearing on it either way.

---

## 4. User / Product Need

Restated from mission framing, not reinterpreted: a user should be able to open modIQ, hand it a real FS25 mod — directory or `.zip` — run the existing assessment pipeline, and understand the result entirely through the console, then repeat for a different mod, without touching a terminal. Today this works only for the directory case, and only once per console session (Reviewing has no way back to Intake). C14 closes exactly those two gaps.

---

## 5. Existing Repository Foundation

- **ZIP routing already exists, is already tested, and requires no backend change.** `AssessmentService::is_archive_location` and its dispatch in `execute_from_assessment_input` are unconditional, already-shipped behavior — this capability activates an existing path, it does not create one.
- **The Tauri command already accepts either input shape.** `submit_assessment(input_path: String)` performs no validation or branching on its argument's shape before handing it to the engine — a `.zip` path and a directory path are equally valid arguments today.
- **The transport (`ReportSummary`/`FindingSummary`/`EvidenceSummary`/`RecommendationSummary`) already carries everything C13 needs to present a report from either intake shape** — nothing about a report's *content* differs based on how its input was routed internally.
- **C13 already aligned Reviewing's presentation to the frozen Report Identity/Overview/Findings/Evidence specification.** C14 reuses that presentation unmodified — this capability is intake and navigation only.
- **The console's own dialog plugin (`tauri-plugin-dialog`) already supports file-mode selection with extension filters** as part of the same API family (`open()`) already used for directory-mode selection, under the same already-granted `dialog:default` capability.

---

## 6. Architectural Boundaries

The following are unchanged by this capability, as a direct consequence of already-adopted architecture, and were checked directly against source rather than assumed:

- No `modiq-runtime`, `modiq-rules`, `modiq-collection`, `modiq-versioning`, `modiq-engine`, `modiq-report`, `modiq-storage`, or `modiq-knowledge` file.
- No change to `assessment.rs`'s Tauri command signature, its `ReportSummary`/`FindingSummary`/`EvidenceSummary`/`RecommendationSummary` DTOs, or `engine/types.ts`/`engine/index.ts` — the existing Request/Response Mechanism transports exactly what it transports today.
- No change to `Overview.tsx` — C13's presentation stands unmodified.
- **No behavioral change to `Reviewing.tsx`; a documentation/comment correction is permitted solely because C14 makes the existing comment factually false.**
- No new Tauri command — `submit_assessment` already accepts everything this capability needs to submit.
- `FrontendArchitecture.md`'s Boundary Enforcement (no inference across the Engine/Consumer boundary, every crossing a deliberate request/response) is satisfied by construction: selecting a `.zip` path and resetting local UI state both involve zero evaluation of engine output.

---

## 7. Architecture Evaluation Determination

**No dedicated Architecture Evaluation or Architectural Resolution is required.** Unlike C2 or C12 (where this repository's own precedent found a genuinely open design question hiding beneath an apparently mechanical change), direct inspection here found no analogous open question:

- The capability introduces no new subsystem, no new dependency edge, no new crate-boundary crossing, no new Tauri command, and no `AssessmentService` signature change.
- Both behaviors this capability exposes (ZIP routing, independent repeated Assessment execution) are not merely *supported* by existing architecture — they are already *implemented and tested* by it. This capability's own work is confined to the console's own local input-selection and presentation-state code, which `FrontendArchitecture.md` already places entirely on the presentation side of the Boundary.
- The one governance-adjacent question this capability touches (Initiative 2) is resolved by *not engaging it at all* (§3), not by any architectural decision this document makes.

This capability proceeds directly to a Process Determination confirming this conclusion, rather than to a dedicated Architecture Evaluation.

---

## 8. In-Scope Behavior

- `apps/console/src/workspace/Workspace.tsx`: add a way to select a `.zip` file, alongside the existing directory picker, submitted through the same `submitAssessment(path)` call already in place.
- `apps/console/src/workspace/Workspace.tsx`: add an "Assess another mod" affordance, reachable from Reviewing, that resets Workspace's own local state (`report`, `selectedPath`, `status`, `error`) back to a fresh Intake presentation — no Runtime, Storage, or transport interaction of any kind.
- `apps/console/src-tauri/capabilities/default.json`: touched **only if** real-machine testing shows the existing `dialog:default` grant does not already cover a filtered, file-mode `open()` call (directory-mode already works under it; this is treated as a low-probability, purely additive permission fix if needed, not a design decision).
- Correcting `Reviewing.tsx`'s doc comment about Initiative 2, once the return-to-Intake affordance exists and makes the current comment's claim false.

---

## 9. Explicit Exclusions

Intentionally outside this capability's own scope, matching the mission's own stated boundary:

- Any new backend assessment behavior, Rule, Evidence Collector, or DTO field.
- Any transport, Runtime, Storage, or persistence change.
- Initiative 2 (same-Assessment supplementation/reentrancy) — not implemented, not advanced, not reopened (§3).
- `AssessmentSubject`/GOV-002 content, Report Identity architecture, Report currency, GOV-008 transport, Evidence Explanation, Confidence/Initiative 4.
- C5 (cross-mod comparison), C12 (historical analysis presentation), Knowledge architecture, Lua analysis, malware/security analysis, production-chain analysis, any new Rule category.
- New authentication, history UI, and any other capability not named in this document's own scope.
- Any frontend framework, routing library, state-management library, or other new architectural infrastructure — the existing local `useState` pattern already used throughout `Workspace.tsx` is sufficient for both new behaviors.
- Any change to `Overview.tsx` or `Reviewing.tsx`'s presentation logic itself (only the doc comment correction named in §8 touches `Reviewing.tsx`).
- Any change to `apps/sandbox` or `modiq-cli`.

---

## 10. Data / Transport Boundary

Unchanged in every respect. `submitAssessment(inputPath: string): Promise<ReportSummary>` (`engine/index.ts`) is called identically regardless of which intake affordance produced `inputPath`, and identically on the second call as on the first. No new field, no new command, no new serialization shape.

---

## 11. Capability Success Criteria

After this capability ships, a user can, entirely through the console:

1. Select a real, extracted FS25 mod directory and receive a real, non-mock assessment report through the existing Reviewing presentation.
2. Select a real FS25 mod `.zip` archive and receive a real, non-mock assessment report through the same Reviewing presentation, produced through the same `submitAssessment` call and the same backend `ArchiveCollector` routing already implemented.
3. From Reviewing, choose "Assess another mod" and land on a fresh Intake state — no residual report content, selected path, or error state from the prior Assessment visible anywhere.
4. Assess a second, different mod (directory or `.zip`, either combination) and see a report that reflects only the second mod's own real Evidence/Findings — with no trace of the first mod's Findings, Evidence, or path label.
5. Confirm, by direct inspection, that no `modiq-runtime`, `modiq-storage`, `modiq-report`, `modiq-engine`, transport, or Tauri-command-signature file changed.
6. Confirm that no code path added by this capability reopens, mutates, or reasons about a *completed* Assessment's own state — the only state reset is Workspace's own local presentation state.
7. Confirm C13's existing report presentation (dimension grouping, collapsed anatomy, evidence progressive disclosure) is pixel-for-pixel/behavior-for-behavior unchanged for both intake shapes.

---

## 12. What Success Does Not Require

- It does not require a combined file-or-directory picker in a single dialog — two distinct, clearly labeled affordances (folder vs. archive) is a sufficient and OS-realistic interaction shape.
- **C14 does not expand the assessment-progress experience. The existing `Submitting…` state remains unchanged.**
- It does not require solving Initiative 2, GOV-002, or any other named-out-of-scope item.
- It does not require a frontend test framework — `tsc`/`npm run build` plus manual real-I/O verification remains sufficient, per established C13 precedent.
- It does not require any new Tauri command.

---

## 13. Testing / Verification Implications

Following the established C13 precedent exactly:

- `cargo fmt --check`
- `cargo check --workspace`
- `cargo test --workspace`
- `apps/console` TypeScript/build verification (`tsc && vite build`)
- Manual real-I/O verification, minimum coverage:
  - One real extracted mod directory, assessed successfully.
  - One real `.zip` mod (from this machine's actual FS25 mods folder), assessed successfully through the same path.
  - Assess → Review → "Assess another mod" → Intake, exercised by hand.
  - A second, different mod assessed immediately after, with its report visually confirmed to contain no stale content or path label from the first.
- No frontend testing framework introduced.
- Optional, only if a gap is found: one or two new Rust-side tests in `assessment.rs` mirroring its existing fixture-directory tests, against a bundled `.zip` fixture — likely unnecessary, since `AssessmentService`'s own suite already covers this at the engine layer.

---

## 14. Repository Impact

**Required participation:** `apps/console/src/workspace/Workspace.tsx` only.

**Conditional participation:** `apps/console/src-tauri/capabilities/default.json` (only if real testing requires it). **`apps/console/src/workspace/Reviewing.tsx` participates only through a documentation/comment correction — no behavioral change of any kind.**

**Explicitly not participating:** every `modiq-*` crate, `apps/console/src-tauri/src/assessment.rs`, `apps/console/src-tauri/src/lib.rs`, `apps/console/src/engine/index.ts`, `apps/console/src/engine/types.ts`, `apps/console/src/workspace/Overview.tsx`, `apps/sandbox`, `modiq-cli`.

---

## 15. Risks

- **Dialog-permission risk (low):** if `dialog:default` does not already cover filtered file-mode `open()`, a one-line capability grant resolves it — not a design question.
- **Scope-creep risk:** temptation to also improve the in-progress/assessing visual state, or to build a fuller "recent assessments" affordance while touching `Workspace.tsx`. Mitigated by §9's explicit exclusions.
- **Terminology risk:** conflating "assess another mod" with Initiative 2 during implementation (as the current codebase's own comment already does). Mitigated by §3's explicit distinction, to be carried into the Implementation Plan and the corrected doc comment itself.
- **Overall capability-definition-stage risk: low** — both required behaviors are thin UI-layer exposures of already-implemented, already-tested backend capability.

---

## 16. Next Required Repository Artifact

A **Process Determination for C14**, confirming §7's conclusion (no Architecture Evaluation required) and proceeding toward Implementation Authorization — not performed by this document.

---

## Status

**Approved by the Product Owner. Authorized to advance to Process Determination only.** No Process Determination, Architecture Evaluation, Architectural Resolution, Implementation Authorization, or implementation has been performed by this document. No Initiative 2, GOV-002, or GOV-008 item is resolved, advanced, or touched by this document.

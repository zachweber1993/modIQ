# IMPLEMENTATION_PLAN_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md

## 1. Plan Identity

| Property | Value |
|---|---|
| **Document** | IMPLEMENTATION_PLAN_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md |
| **Origin** | `IMPLEMENTATION_AUTHORIZATION_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md` (working tree, uncommitted), treated as fixed, unreopened. `PROCESS_DETERMINATION_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md` and `CAPABILITY_DEFINITION_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md` are fixed inputs, not restated except where a factual correction is required (§2). |
| **Status** | Implementation Planning draft. No code has been produced in preparing this document. |

---

## 2. Repository Verification — Including One Correction to the Governing Documents

Re-verified directly against current source this session, per this stage's own instruction not to rely blindly on prior text where inspection can check it.

**Branch/HEAD/tree:** `feature/runtime-implementation` @ `06e48b3`, clean apart from the three prior C14 documents (untracked). No source file has changed since those three documents were written.

**A path correction, found by direct inspection, not by assumption:** all three prior governing documents (`CAPABILITY_DEFINITION_C14...md`, `PROCESS_DETERMINATION_C14...md`, `IMPLEMENTATION_AUTHORIZATION_C14...md`) name the Workspace file as `apps/console/src/workspace/Workspace.tsx`. **This path does not exist.** A direct `find` against `apps/console/src` confirms the file actually lives at `apps/console/src/regions/Workspace.tsx` — alongside `Console.tsx` and `Dashboard.tsx`, not alongside `Overview.tsx`/`Reviewing.tsx` (which genuinely are under `apps/console/src/workspace/`). This appears to be a directory-name assumption made once, early in the lifecycle, and carried forward unchecked through all three subsequent documents rather than three independent errors.

This is a documentation defect in the prior lifecycle artifacts, not a scope, behavior, or architecture question — the file those documents mean, identify by name, and describe the contents of (confirmed by direct re-read, §3 below, matching every prior description exactly: `IntakeStatus`, `chooseFolder`, `status`/`selectedPath`/`error`/`report` state, the `open({ directory: true, multiple: false })` call, the `Reviewing`/intake branch) is unambiguous. This Plan proceeds using the **correct, verified path** (`apps/console/src/regions/Workspace.tsx`) throughout, and flags this correction explicitly rather than silently substituting it — consistent with this project's own established practice of naming a discovered documentation defect rather than absorbing it without comment (`IMPLEMENTATION_REPORT_REPORT_ANATOMY_ALIGNMENT.md` §8, item 6, is the direct precedent for correcting a prior document's own factual claim once implementation-stage inspection finds it false). This correction does not reopen, revise, or expand the Authorization's own scope in any way — it identifies the same one file the Authorization already bounded work to, by its real path. **The Product Owner should treat this as a housekeeping correction to record in the eventual Implementation Report, not as new scope requiring separate re-authorization** — the authorized file, by content and identity, has not changed.

`Reviewing.tsx`'s path (`apps/console/src/workspace/Reviewing.tsx`) is confirmed correct as stated in all three prior documents.

---

## 3. Current Implementation Baseline (re-verified, direct source read)

**`apps/console/src/regions/Workspace.tsx`** (64 lines): a single `Workspace()` function component holding four `useState` values — `status: IntakeStatus` (`"idle" | "submitting" | "failed"`), `selectedPath: string | null`, `error: string | null`, `report: ReportSummary | null`. One handler, `chooseFolder`, calls `open({ directory: true, multiple: false })` from `@tauri-apps/plugin-dialog`; on a non-cancelled result it sets `selectedPath`, clears `error`, sets `status` to `"submitting"`, calls `submitAssessment(path)`, and on success sets `report`/`status: "idle"` or on failure sets `error`/`status: "failed"`. Rendering is a two-way branch: `if (report && selectedPath)` renders `<Reviewing findings={report.findings} subjectLabel={selectedPath} />`; otherwise renders the Intake view (a status message, an error message when `status === "failed"`, and one `<Button onClick={chooseFolder}>`). There is no code path anywhere in this file that ever sets `report`, `selectedPath`, `status`, or `error` back toward their initial values once populated.

**`apps/console/src/workspace/Reviewing.tsx`** (top of file, re-read directly): a doc comment block (lines 5–35) describing the component's own disclosure model, ending with the passage this Plan corrects: *"No affordance returns to Intake: Sprint 21 does not implement reentrancy or supplementation (Initiative 2), so once Reviewing is reached there is deliberately nowhere else for this phase to go."* The component itself (`Reviewing({ findings, subjectLabel })`) receives both props from `Workspace.tsx` and renders `Overview` plus the grouped/expandable Findings list; it holds no state of its own relevant to returning to Intake, and none is added by this Plan — the return path is authorized to live entirely in `Workspace.tsx` (Authorization §4), reached by a control `Reviewing.tsx` is handed as a prop, not by any state `Reviewing.tsx` itself owns.

**`@tauri-apps/plugin-dialog`'s actual TypeScript API** (`apps/console/node_modules/@tauri-apps/plugin-dialog/dist-js/index.d.ts`, re-read directly this session): `open(options?: OpenDialogOptions)`. Relevant fields on `OpenDialogOptions`: `filters?: DialogFilter[]` (each `{ name: string; extensions: string[] }`, extensions given without a leading `.`); `multiple?: boolean`; `directory?: boolean`. The same single `open()` function already imported in `Workspace.tsx` serves both directory mode (`directory: true`) and file mode (`directory` omitted or `false`, optionally with `filters`) — there is no separate function, import, or dependency for file-mode selection. `OpenDialogReturn<T>`'s own conditional type confirms that with `multiple: false` and `directory` omitted, the resolved value is `string | null` — the exact same shape `chooseFolder`'s existing `if (!path || Array.isArray(path)) return;` guard already handles defensively.

**Dialog permission, re-confirmed unaffected:** `apps/console/src-tauri/capabilities/default.json` grants `dialog:default` (`allow-message`, `allow-save`, `allow-open`); `allow-open` is scoped at the `open` command level, not per-parameter (`PROCESS_DETERMINATION_C14...md` §2, itself re-verified against `tauri-plugin-dialog-2.7.2`'s own `permissions/default.toml`/`autogenerated/commands/open.toml`). No capability file change is planned, matching the Authorization's explicit non-authorization of that file.

---

## 4. Implementation Strategy

Two changes, both additive, confined to the two authorized files:

- `apps/console/src/regions/Workspace.tsx` gains: (a) a second intake affordance calling the same `open()` import in file mode with a `.zip` filter, submitted through the unchanged `submitAssessment` call already in place; (b) a reset function, invoked by a new "Assess another mod" control rendered from within the existing Reviewing branch, that sets all four existing `useState` values back to their initial values.
- `apps/console/src/workspace/Reviewing.tsx` gains: a corrected doc comment only. No prop, no state, no JSX, no rendering behavior changes. The "Assess another mod" control itself is authorized to live in `Workspace.tsx`, not inside `Reviewing.tsx` — see §5, Step 3, for the exact mechanism, which requires no new prop on `Reviewing` at all.

No new component, no new file, no new dependency, no new top-level state container. Both `chooseFolder`-shaped handlers (existing directory handler, new archive handler) share the existing `submitAssessment`/state-update logic pattern; this Plan does not mandate extracting a shared helper function, since the existing file is 64 lines and duplicating roughly six lines of already-simple logic does not create a maintainability problem the Authorization asks this Plan to solve — but Implementation may factor the shared submit-and-update logic into one small local function if doing so does not touch `Reviewing.tsx`, add a new file, or introduce new state; this is a Planning-level, not architectural, choice left open per Authorization §5's own reservation of interaction-design detail.

---

## 5. Step-by-Step Implementation Plan

### Step 1 — `.zip` archive intake affordance

- **Exact file:** `apps/console/src/regions/Workspace.tsx`.
- **Relevant existing code/state:** the existing `chooseFolder` handler (lines 25–43) and its `<Button onClick={chooseFolder}>` (line 59); the existing `open` import from `@tauri-apps/plugin-dialog` (line 2); the existing `submitAssessment` import (line 4); `status`/`selectedPath`/`error`/`report` state (lines 20–23).
- **Intended change:** add a second handler, e.g. `chooseArchive`, calling `open({ multiple: false, filters: [{ name: "Mod Archive", extensions: ["zip"] }] })` — the same `open` import already in scope, no new import. On a non-cancelled result (`string`, per `OpenDialogReturn`'s own typing for `multiple: false` with `directory` omitted — confirmed §3), it performs the identical sequence `chooseFolder` already performs: set `selectedPath`, clear `error`, set `status` to `"submitting"`, call `submitAssessment(path)`, then update `report`/`status`/`error` on resolve/reject exactly as today. Add a second `<Button onClick={chooseArchive}>` (e.g. "Choose a mod .zip") alongside the existing "Choose a mod folder" button, both disabled under the same `status === "submitting"` condition already governing the existing button.
- **Why this satisfies the authorized behavior:** Authorization §3 item 1 requires exactly this — a `.zip` affordance using the existing dialog API and the existing `submitAssessment(inputPath)` path, with the directory path preserved unmodified. `AssessmentService::execute_from_assessment_input`'s own archive routing (re-confirmed unchanged, `PROCESS_DETERMINATION_C14...md` §2) requires nothing from the caller beyond a path string ending in `.zip` — this affordance's only job is producing that string and handing it to the same already-existing call.
- **Invariant/constraint:** no new Tauri command, no new import beyond what's already present, no change to `submitAssessment`'s signature or call shape, no change to the existing `chooseFolder` handler's own behavior. The two affordances must remain two distinct, clearly labeled actions (Authorization §3's own conditioning clause) — not merged into one dialog invocation, since the underlying OS/plugin API does not provide a single dialog that is simultaneously directory-mode and filtered-file-mode.
- **Verification associated with this step:** `apps/console`'s `tsc && vite build` clean; manual real-I/O — selecting a real `.zip` from this machine's own FS25 mods folder and confirming a real `ReportSummary` renders via the unchanged `Reviewing` branch.

### Step 2 — Preserve existing directory-selection path

- **Exact file:** `apps/console/src/regions/Workspace.tsx`.
- **Relevant existing code/state:** `chooseFolder` (lines 25–43), its button (line 59).
- **Intended change:** none to this handler's logic. Step 1 adds a sibling handler and a sibling button; `chooseFolder` and its existing button are not edited, renamed, or restructured (beyond whatever minimal, purely additive JSX restructuring is needed to lay out two buttons instead of one — e.g., wrapping both in an existing or new flex container — which does not alter either button's own behavior).
- **Why this satisfies the authorized behavior:** Authorization §3 item 2 requires the current directory-selection behavior to function exactly as it does today. Not touching the handler is the direct way to guarantee this, rather than refactoring it and re-deriving equivalence.
- **Invariant/constraint:** `chooseFolder`'s own five-line body (lines 26–42) remains byte-identical, or is only touched if Implementation elects the optional shared-helper factoring named in §4 — in which case both handlers' observable behavior, not their literal source text, is what must remain identical.
- **Verification associated with this step:** manual real-I/O — selecting a real, extracted FS25 mod directory and confirming behavior is indistinguishable from pre-C14 behavior.

### Step 3 — "Assess another mod" affordance and local state reset

- **Exact file:** `apps/console/src/regions/Workspace.tsx`.
- **Relevant existing code/state:** the existing Reviewing branch, `if (report && selectedPath) { return <Reviewing findings={report.findings} subjectLabel={selectedPath} />; }` (lines 45–47); the four `useState` values (lines 20–23).
- **Intended change:** add a reset function, e.g. `resetToIntake`, that calls each of the four existing setters with each state's own original initial value (`setStatus("idle")`, `setSelectedPath(null)`, `setError(null)`, `setReport(null)`). Render one control (e.g. a `<Button>`, "Assess another mod") alongside the `<Reviewing>` element in the Reviewing branch, wired to call `resetToIntake`. This requires **no new prop on `Reviewing`** — the control is rendered by `Workspace.tsx` itself, positioned adjacent to (not inside) the `<Reviewing>` element it currently returns alone, so `Reviewing.tsx` requires no signature change to support it (consistent with Authorization §4's prohibition on any behavioral change to that file).
- **Why this satisfies the authorized behavior:** Authorization §3 item 3 requires exactly this affordance, resetting only Workspace's own local presentation state. Setting each of the four `useState` values back to its own initial value is the complete and only state this component owns (§3) — there is no fifth piece of state, no ref, no external store, and nothing in `Reviewing.tsx` that would need a corresponding reset, since `Reviewing.tsx` owns no state relevant to which Assessment is being viewed (its own `expandedFindingId`/`revealedEvidenceIds` are disclosure state scoped to whichever `findings` prop it currently receives, and are naturally discarded when `Reviewing` unmounts on the next render after `report`/`selectedPath` go back to `null` — no explicit action on this state is needed or authorized).
- **How this differs from reopening or mutating a completed Assessment (required explicit explanation):** `resetToIntake` performs zero Tauri calls, zero Runtime interaction, and zero engine invocation of any kind — it is four synchronous `useState` setter calls, executing entirely within the React component tree, with no `invoke()` crossing the Engine/Consumer boundary at all. The `Assessment` value the prior `report` was derived from was already completed and returned by the time `report` was set (`AssessmentService::execute` returns the report *before* calling `assessment.complete()`'s corresponding Runtime object is discarded on the Rust side once the Tauri command returns — no reference to it is held anywhere in the console process). Clearing `report` therefore does not reach back into, reopen, or mutate any Runtime `Assessment` — there is no live `Assessment` value on the TypeScript side to reopen in the first place; what is being cleared is this component's own *display* of an already-finished result, exactly as `Console.tsx`'s existing `signOut` clears its own display of an already-established session (`Process Determination §2/§4 Question 3`, re-confirmed here). The subsequent Assessment (Step 4) is triggered only by a **new** call to `chooseFolder`/`chooseArchive`, which is only reachable once `resetToIntake` has returned the component to its Intake-branch rendering — there is no code path by which choosing "Assess another mod" causes any existing `AssessmentReport`, `Assessment`, or persisted record to be read, referenced, supplemented, or altered. This is the mechanical basis for Authorization §7's invariant that `INV-012` remains unimplicated.
- **Invariant/constraint:** the reset must set all four state values, not a subset (a partial reset risking, e.g., a stale `selectedPath` label surviving into the next Intake render) — this is the direct basis for the "no stale report/path/error state" verification requirement (§7, below). No Tauri call, no `submitAssessment` invocation, and no new state variable may be introduced by this step.
- **Verification associated with this step:** manual real-I/O — from a completed Reviewing state, clicking "Assess another mod" and confirming the component renders the Intake branch with no residual "Included: …" text, no residual error text, and the button back in its non-submitting label state.

### Step 4 — Independent second Assessment execution

- **Exact file:** `apps/console/src/regions/Workspace.tsx` (no additional code beyond Steps 1–3 — this step is a verification-and-confirmation step over the composition of the prior three, not a fourth code change).
- **Relevant existing code/state:** `chooseFolder`/`chooseArchive` (Steps 1–2), reachable again only after `resetToIntake` (Step 3) has returned `report`/`selectedPath` to `null`.
- **Intended change:** none. Because `resetToIntake` (Step 3) sets `report`/`selectedPath` back to `null`, the component's own existing render branch (`if (report && selectedPath)`) naturally returns to the Intake JSX without any new conditional logic — the "second assessment" is simply the same `chooseFolder`/`chooseArchive` → `submitAssessment` sequence invoked a second time, against whatever second path the user selects.
- **Why this satisfies the authorized behavior:** Authorization §3 item 4 requires the second assessment to use the existing independent execution path with no new invocation shape. `submitAssessment(path)` is called identically on the second invocation as the first — same function, same signature, same absence of any correlating argument to the prior call. On the Rust side, `AssessmentService::execute_from_assessment_input` constructs a wholly new `Assessment` each call (re-confirmed, `PROCESS_DETERMINATION_C14...md` §2, `separate_executions_are_independent_and_deterministic`) — there is no mechanism, on either side of the Tauri boundary, by which a second call could reference, depend on, or be influenced by the first.
- **Invariant/constraint:** no new argument, flag, or correlating identifier may be added to the `submitAssessment` call to "link" the two assessments — doing so would be a transport change (explicitly unauthorized) and would misrepresent independent Assessments as related, contrary to the Initiative 2 distinction (§8, below).
- **Verification associated with this step:** manual real-I/O — after Step 3's reset, selecting a **second, different** real mod (recommended: the opposite intake shape from the first, e.g. directory-then-`.zip` or `.zip`-then-directory, to exercise both paths across the two assessments) and confirming a fresh, independently-produced `ReportSummary` renders.

### Step 5 — `Reviewing.tsx` documentation correction

- **Exact file:** `apps/console/src/workspace/Reviewing.tsx`.
- **Relevant existing code/state:** the doc comment's final sentence (lines 32–35): *"No affordance returns to Intake: Sprint 21 does not implement reentrancy or supplementation (Initiative 2), so once Reviewing is reached there is deliberately nowhere else for this phase to go."*
- **Intended change:** replace this sentence with an accurate statement that (a) a return-to-Intake affordance now exists (rendered by `Workspace.tsx`, adjacent to `Reviewing`, per Step 3 — not inside this file), (b) it starts a new, independent Assessment, and (c) this is distinct from, and does not implement, Initiative 2's reentrant/supplementation lifecycle question, which remains unresolved. No other line of the doc comment (describing disclosure state, expansion behavior, or Evidence progressive disclosure) is touched.
- **Why this satisfies the authorized behavior:** Authorization §3 item 5 requires exactly this correction, and only this correction — the comment's current claim becomes factually false once Step 3 ships, and this project's own standing discipline (§2's cited precedent) requires correcting a now-false claim in a governing comment rather than leaving it to silently drift.
- **Invariant/constraint:** **zero behavioral change** — no JSX, prop, state, or import in `Reviewing.tsx` may change as part of this step (Authorization §4's explicit condition). This step is textual only, inside an existing comment block.
- **Verification associated with this step:** `git diff` on this file shows only comment-text lines changed; `tsc && vite build` clean (a comment-only change cannot itself cause a build failure, but the gate is re-run per §7's own repository-diff requirement).

---

## 6. File-Level Change Plan

| File | Required change | Authorization basis |
|---|---|---|
| `apps/console/src/regions/Workspace.tsx` *(path corrected — see §2)* | Add `.zip` intake affordance (Step 1); preserve directory affordance unmodified (Step 2); add `resetToIntake` and its "Assess another mod" control (Step 3); no additional code for independent second-execution behavior, which follows from Steps 1–3 (Step 4). | Authorization §3 items 1–4, §4 |
| `apps/console/src/workspace/Reviewing.tsx` | Doc-comment correction only (Step 5). No behavioral change. | Authorization §3 item 5, §4 |

No other file. If implementation finds either change cannot be completed without touching a third file — including, but not limited to, `apps/console/src-tauri/capabilities/default.json`, `assessment.rs`, `engine/index.ts`, `engine/types.ts`, or `Overview.tsx` — this is **ESCALATION — NOT AUTHORIZED** (§10), not a Plan decision to make silently.

---

## 7. Data/Transport Confirmation

`submitAssessment(inputPath: string): Promise<ReportSummary>` (`apps/console/src/engine/index.ts`, re-confirmed unchanged this session by `git status` showing zero diff against HEAD) is called with the identical signature by both `chooseFolder` and the new `chooseArchive`, and identically on a second invocation as the first. No field is added, renamed, or restructured in `ReportSummary`, `FindingSummary`, `EvidenceSummary`, `RecommendationSummary`, or `RecommendationStepSummary` (`apps/console/src/engine/types.ts`). `assessment.rs`, `engine/index.ts`, and `engine/types.ts` are not touched by this Plan.

---

## 8. Initiative 2 Distinction (Restated for Implementation)

Preserved explicitly, per this stage's own instruction, and directly grounded in Step 3's own mechanical explanation above:

- **"Assess another mod"** = `resetToIntake`'s four local `useState` resets, followed by a **new**, independent call to `submitAssessment`/`execute_from_assessment_input`, producing a **new** `Assessment` with no reference to any prior one. This is what C14 implements.
- **Initiative 2** = an already-**completed** `Assessment` accepting **additional evidence** so that its **same** Report continues to reflect it — governed by `INV-012` and currently dispositioned "Requires Governance Reconciliation." **Nothing in this Plan's five steps touches, approaches, or requires resolving this.** No step reads, references, or holds a reference to a prior `Assessment`, `AssessmentReport`, or persisted record once `resetToIntake` has run.

---

## 9. Testing and Verification Plan

Exact sequence, per this stage's own specification, run in this order:

1. `cargo fmt --check` — expected clean; no Rust file is touched by this Plan, so this gate is a repository-integrity confirmation, not an anticipated site of change.
2. `cargo check --workspace` — expected clean, same basis.
3. `cargo test --workspace` — expected clean, full suite, same basis.
4. `apps/console`'s `tsc && vite build` — must pass with the two authorized files' changes present; this is the one gate this Plan's own changes can actually affect.
5. **Real extracted FS25 mod → assess → review**: select a real, extracted mod directory via the preserved (Step 2) affordance; confirm a real, non-mock `ReportSummary` renders through the unchanged Reviewing/Overview presentation.
6. **Real FS25 `.zip` → assess → review**: select a real `.zip` mod (this machine's own FS25 mods folder already contains suitable material, confirmed present during Capability Definition) via the new (Step 1) affordance; confirm an equivalent real, non-mock report renders through the same, unchanged presentation.
7. **Review → Assess another mod → fresh Intake**: from either of the two reports above, invoke the Step 3 affordance; confirm the workspace returns to the Intake branch with no residual `selectedPath`/`error` text and both buttons back in their non-submitting state.
8. **Assess a second, different real mod**: immediately after Step 7, select a mod of the *other* intake shape from the first (directory after `.zip`, or `.zip` after directory) and confirm a fresh report renders.
9. **Verify no stale report, path, error, or presentation state**: direct visual comparison confirming the report, subject label, and any prior error text visible after Step 8 belong exclusively to the second mod — no Finding, Evidence, path label, or error message from the first mod is present anywhere in the view.
10. **Verify C13 report presentation remains unchanged**: direct comparison of `Overview`/`Reviewing`'s rendering (dimension grouping, collapsed anatomy, evidence progressive disclosure) for both reports above against pre-C14 behavior — no regression.
11. **Inspect final diff and confirm only authorized files changed**: `git diff --stat` shows exactly `apps/console/src/regions/Workspace.tsx` and `apps/console/src/workspace/Reviewing.tsx`, with the latter limited to comment-text lines; no other file — Rust, TypeScript, DTO, capability, or otherwise — appears.

No frontend test framework is introduced for any of the above, matching this stage's own instruction and the established C13 precedent.

---

## 10. Escalation Conditions

**ESCALATION — NOT AUTHORIZED** applies, and this Plan does not resolve it internally, if implementation discovers: a need to touch any file outside §6 (including `apps/console/src-tauri/capabilities/default.json`, on the theory that the existing `dialog:default` grant proves insufficient in practice — contrary to the Process Determination's own verified finding, but not to be silently worked around if real testing somehow contradicts it); a need for any new Tauri command, DTO field, or transport shape; a need to add a prop to `Reviewing.tsx` or otherwise give it behavior beyond its existing disclosure model; a need to hold a reference to a prior `Assessment`/`AssessmentReport`/persisted record across a reset (which would constitute Initiative 2 territory, not this capability); or any point where implementing the "Assess another mod" affordance seems to require anything resembling supplementing, reopening, or versioning a completed Assessment rather than simply starting a new one.

---

## 11. Implementation Sequence

**Phase 1 — `Workspace.tsx` intake affordances (Steps 1–2).** Add `chooseArchive` and its button alongside the untouched `chooseFolder`. *Gate:* `tsc`/`vite build` clean; manual verification items 5–6.

**Phase 2 — `Workspace.tsx` reset affordance (Step 3).** Add `resetToIntake` and its control in the Reviewing branch. *Gate:* build clean; manual verification item 7.

**Phase 3 — Second-execution confirmation (Step 4).** No new code; direct manual exercise of Phases 1–2 in sequence against a second, different real mod. *Gate:* manual verification items 8–9.

**Phase 4 — `Reviewing.tsx` comment correction (Step 5).** Independent of Phases 1–3; may be done in any order relative to them, sequenced last here only so the comment's new text can accurately describe the affordance once it actually exists. *Gate:* `git diff` on this file shows comment-only change; build clean.

**Phase 5 — Final Reverification.** No file change. Full gate re-run (§9, items 1–4); manual verification item 10 (C13 presentation unchanged) and item 11 (`git diff --stat` confirms exactly the two authorized files).

---

## 12. Completion Criteria

All five steps' individual completion criteria (§5) hold against real-I/O verification (§9, items 5–10); both mechanical gates and the manual sequence (§9) pass in full; `git diff --stat` shows exactly the two files named in §6 changed, with `Reviewing.tsx` limited to its comment; the §2 path correction is carried into the eventual Implementation Report; no item in Authorization §5 (Explicit Exclusions) is touched. Partial completion (fewer than five steps) is not Engineering Release, per this project's standing discipline.

---

## 13. Implementation Report Expectations

Must document: files changed (expected: exactly the two named in §6, with the corrected `regions/Workspace.tsx` path carried through and the §2 discrepancy formally noted as a documentation correction to the three prior C14 artifacts); each of the five steps' verification outcome against §9's real-I/O sequence; gate results (§9, items 1–4); explicit confirmation that Initiative 2 was not touched, approached, or implicated anywhere in the implementation (§8); explicit confirmation that C13 presentation is unchanged (§9, item 10); any deviation from this Plan or the Authorization, classified as Planning defect vs. implementation defect per this project's standing discipline.

---

## 14. Final Planning Determination

**Yes — there is now a sufficiently precise, bounded, implementation-ready plan for the authorized capability.**

All five authorized behaviors have a stated exact file, current-state baseline, intended change, authorization basis, invariant, and verification method. The two-file boundary holds, using the corrected real path for `Workspace.tsx` found by direct inspection during this Plan's own preparation (§2) — a documentation correction, not a scope change. The local-state-reset mechanism is explained mechanically, not merely asserted, as categorically distinct from reopening or mutating a completed Assessment (§5, Step 3). The exact `tauri-plugin-dialog` configuration for `.zip` selection is specified from the plugin's own real type definitions, introducing no new dependency or capability grant. The eleven-item verification sequence is reproduced exactly as specified, in order, with no frontend test framework introduced.

---

**Repository confirmation:** no source file, fixture, or test file created or modified in preparing this Plan. No commit, no push.

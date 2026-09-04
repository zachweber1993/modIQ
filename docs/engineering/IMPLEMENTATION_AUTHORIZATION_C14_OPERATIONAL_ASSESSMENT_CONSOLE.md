# Implementation Authorization — C14: Operational Assessment Console

| Property | Value |
|---|---|
| **Document** | IMPLEMENTATION_AUTHORIZATION_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md |
| **Project** | modIQ |
| **Purpose** | Convert `PROCESS_DETERMINATION_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md`'s own conclusion into a bounded engineering envelope. Authorizes implementation in principle — participating files, exclusions, invariants, verification gates, and completion criteria — and nothing beyond that. Does not perform implementation and does not create the Implementation Plan. |
| **Origin** | `PROCESS_DETERMINATION_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md` (working tree, uncommitted), whose own conclusion (§6) is that C14 requires no Architecture Evaluation or Architectural Resolution and that an Implementation Authorization is the next required artifact. `CAPABILITY_DEFINITION_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md` (working tree, uncommitted) is the governing capability, treated as fixed and unreopened. `IMPLEMENTATION_AUTHORIZATION_C4_APPS_SANDBOX_FIELD_PARITY.md` is the directly on-point structural precedent this document follows, adapted for a console-presentation capability with no DTO or transport participation at all, rather than C4's own DTO-extension shape. |
| **Status** | **Approved. Authorizes C14 Implementation Planning within the scope defined below. No TypeScript code, Rust code, or UI has been produced in preparing this document.** |

---

## 1. Authorization Decision

**Authorized with Scope Constraint.**

This determination is not newly reasoned here — it is `PROCESS_DETERMINATION_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md`'s own conclusion (§6: both prongs of the two-prong test resolve against C14; no Architecture Evaluation required), converted into binding form. Nothing in this document revisits how that conclusion was reached, reweighs C2's/C12's own precedent against C1's/C3's/C4's, or reopens any of the six questions the Process Determination already investigated and resolved.

This document's own organization follows `IMPLEMENTATION_AUTHORIZATION_C4_APPS_SANDBOX_FIELD_PARITY.md`'s condensed shape, adapted where C14's own facts differ materially: C4 extended a Tauri IPC DTO across a serialization boundary; C14 introduces **no** DTO, transport, or Tauri command change of any kind — its entire authorized surface is console-local input selection and console-local presentation-state lifecycle, calling an already-complete backend path exactly as it is called today.

---

## 2. Architectural Basis

Derived exclusively from already-Adopted determinations and already-verified repository facts; nothing here is newly reasoned:

- **`AssessmentService::execute_from_assessment_input`'s existing archive-vs-filesystem routing** (`crates/modiq-engine/src/engine/assessment_service.rs`) is the entire basis for authorizing ZIP intake — routing is unconditional, already implemented, and already tested against real ZIP archives. This authorization activates an existing path; it does not create one.
- **`separate_executions_are_independent_and_deterministic`** (same file, re-verified directly by `PROCESS_DETERMINATION_C14...md` §2) is the entire basis for authorizing "assess another mod" as a new, independent Assessment — each call to `execute_from_assessment_input` already constructs its own fresh `Assessment`, with no dependency on, or interaction with, any prior call's result.
- **`INV-012`** (`docs/implementation/RuntimeInvariants.md:37-39`, "A Completed Assessment SHALL reject further lifecycle transitions") governs one `Assessment` value continuing to mutate after completion. Re-confirmed by the Process Determination (§2, §4 Question 2) to not reach the case this authorization scopes — no `Assessment` value is reopened by anything authorized here.
- **`Console.tsx`'s own `signOut` mechanism** is the direct, already-shipped precedent for resetting local, ephemeral presentation state to return a user to an earlier screen — confirmed by the Process Determination (§2, §4 Question 3) to be architecturally unremarkable in this exact codebase.
- **`tauri-plugin-dialog`'s `allow-open` permission**, confirmed by the Process Determination (§2, §4 Question 5) to be scoped at the command level, not the parameter level — the existing `dialog:default` grant in `apps/console/src-tauri/capabilities/default.json` already covers file-mode, filtered selection exactly as it covers today's directory-mode selection. **No capability-file change is authorized, because none is required.**
- **`submit_assessment(input_path: String) -> Result<ReportSummary, String>`** (`apps/console/src-tauri/src/assessment.rs`) already accepts either a directory or a `.zip` path with no branching, validation, or shape distinction of any kind. This authorization builds entirely within that existing, unmodified command surface.
- **`INITIATIVE_2_ARCHITECTURAL_RESOLUTION.md` §2's own Central Architectural Question**, re-read directly by the Process Determination (§4 Question 4), is framed entirely in terms of one completed Assessment's Report continuing to reflect new material. C14's "assess another mod" produces a second, unrelated Assessment's own Report through a second, independent engine call — outside that question's scope, confirmed by direct textual comparison, not by analogy.

---

## 3. Authorized Scope

Implementation Planning is authorized to scope work that:

1. **Adds a console intake affordance for selecting a `.zip` archive**, in `apps/console/src/workspace/Workspace.tsx`, using `@tauri-apps/plugin-dialog`'s existing `open()` API in file mode (`directory: false`, with an extension filter for `.zip`), submitted through the existing, unmodified `submitAssessment(inputPath)` call already used for directory selection.
2. **Preserves the existing directory-selection path** exactly as it functions today — the existing `open({ directory: true, multiple: false })` call, and its existing submission behavior, are not altered by adding the archive affordance alongside it.
3. **Adds an "Assess another mod" affordance**, reachable from the Reviewing view, that resets `Workspace.tsx`'s own local state (`report`, `selectedPath`, `status`, `error`) to their initial values and returns the workspace to a fresh Intake presentation. This affordance performs no Tauri call, no Runtime interaction, and no persistence interaction of any kind — it is a pure local state reset.
4. **Ensures the resulting second assessment is a new, independent Assessment**, using the existing `submitAssessment`/`execute_from_assessment_input` path exactly as invoked for the first — no new invocation shape, no correlation, reference, or relationship recorded between the first and second Assessment anywhere in the console.
5. **Corrects `Reviewing.tsx`'s stale documentation comment** — the passage currently reading *"No affordance returns to Intake: Sprint 21 does not implement reentrancy or supplementation (Initiative 2)"* — to accurately state that a return-to-Intake affordance now exists, that it starts a new, independent Assessment, and that this is distinct from, and does not implement, Initiative 2's own reentrant/supplementation lifecycle question. This is a documentation/comment correction only; it carries no behavioral change.

This authorization is conditioned on all of the following:

- No file outside §4's Participating list is touched.
- No new Tauri command, argument, or interaction mechanism beyond the existing `submit_assessment(input_path)` invocation shape.
- `ReportSummary`, `FindingSummary`, `EvidenceSummary`, `RecommendationSummary`, and `RecommendationStepSummary` remain unchanged in every field, meaning, and shape.
- The two intake affordances (directory, archive) are presented as two distinct, clearly labeled actions — not merged into a single dialog invocation that would require a mechanism the underlying OS dialog API does not uniformly provide.
- The "Assess another mod" reset touches only the four named local state variables in `Workspace.tsx` — no new state variable, no new persisted value, no new external call.

No responsibility outside this list is authorized by this document.

---

## 4. Participating Files

**Participating:**

- `apps/console/src/workspace/Workspace.tsx` — the two new affordances (§3 items 1–4): archive selection, preserved directory selection, and the "Assess another mod" state-reset affordance.
- `apps/console/src/workspace/Reviewing.tsx` — **documentation/comment correction only** (§3 item 5). No behavioral change of any kind is authorized in this file. No JSX, prop, state, or rendering logic may change.

**Not participating — confirmed unchanged by this Authorization:**

- Every `modiq-*` crate (`modiq-runtime`, `modiq-rules`, `modiq-collection`, `modiq-versioning`, `modiq-engine`, `modiq-report`, `modiq-storage`, `modiq-knowledge`) — no field, function, or type this authorization relies on requires any change; every mechanism it activates already exists and is already tested.
- `apps/console/src-tauri/src/assessment.rs` and `apps/console/src-tauri/src/lib.rs` — the Tauri command surface, its argument shape, and its `ReportSummary`/`FindingSummary`/`EvidenceSummary`/`RecommendationSummary`/`RecommendationStepSummary` DTOs are unchanged.
- `apps/console/src/engine/index.ts` and `apps/console/src/engine/types.ts` — the Request/Response Mechanism's transport is unchanged; `submitAssessment(inputPath: string): Promise<ReportSummary>` is called with the identical signature regardless of which affordance produced `inputPath`, and identically on a second call as on the first.
- `apps/console/src-tauri/capabilities/default.json` — **not authorized**, because `PROCESS_DETERMINATION_C14...md` §2/§4 (Question 5) confirmed the existing `dialog:default` grant's `allow-open` permission is scoped at the command level and already covers file-mode, filtered selection with no change.
- `apps/console/src/workspace/Overview.tsx` — C13's presentation is unaffected and unauthorized for change.
- `apps/sandbox`, `modiq-cli` — separate consumers, sharing no transport or presentation code with `apps/console`; unaffected.

---

## 5. Explicit Exclusions

**Architecturally out of scope — unaffected by this document:**

- Any Rust or backend change, including `modiq-runtime`, `modiq-rules`, `modiq-collection`, `modiq-versioning`, `modiq-engine`, `modiq-report`, `modiq-storage`, `modiq-knowledge`, or `apps/console/src-tauri`.
- Any DTO field, transport shape, or Tauri command signature change.
- Any Tauri capability/permission-file change (`apps/console/src-tauri/capabilities/default.json`) — confirmed unnecessary, not merely deferred.
- **Initiative 2 (Reentrant Assessment Lifecycle)** — same-Assessment supplementation, `INV-012`'s own terminality, or the "Updated" marker mechanism. Not implemented, not advanced, not reopened. "Assess another mod" is a new, independent Assessment; it is not, and must not be implemented as, a mechanism for an existing completed Assessment to accept new material.
- GOV-002 (`AssessmentSubject` content), GOV-008 (transport contract), Report Identity architecture, Report currency, Evidence Explanation, Confidence/Initiative 4.
- C5 (cross-mod comparison), C12 (historical analysis presentation), Knowledge architecture, Lua analysis, malware/security analysis, production-chain analysis, any new Rule category.
- `Overview.tsx`, and any behavioral change to `Reviewing.tsx`'s presentation of Overview/Findings/Evidence/Recommendations — C13's presentation stands exactly as it is.
- Any new frontend framework, routing library, or state-management library — the existing local `useState` pattern already used throughout `Workspace.tsx` is the only mechanism this authorization permits.
- `apps/sandbox`, `modiq-cli` — untouched.
- Any unrelated cleanup, refactor, or styling change not directly required by §3's five authorized behaviors.

**Outside the scope of this authorization by implementation scope — no architectural decision is required for any of these; they simply are not part of this work:**

- The exact visual/interaction design of the two intake affordances (e.g., two buttons vs. a single control with a mode toggle) — reserved for Implementation Planning, provided both remain distinct, discoverable, single-step actions consistent with `ASSESSMENT_INTAKE_AND_UPLOAD.md`.
- The exact label and placement of the "Assess another mod" affordance — reserved for Implementation Planning.
- Any Sprint Plan, phase breakdown, or testing sequencing beyond §7's own boundary-level requirement.

---

## 6. Data / Transport Boundary

**Explicitly unchanged.** `submitAssessment(inputPath: string): Promise<ReportSummary>` (`apps/console/src/engine/index.ts`) remains exactly as it is today — same name, same signature, same return type. No new request field, no new response field, and no new command are authorized. Both authorized affordances (archive selection, "assess another mod") produce nothing more than a path string handed to this existing function, exactly as directory selection does today.

---

## 7. Architectural Invariants That Must Not Be Violated

- `apps/console` remains presentation-only — no code introduced by this work may evaluate Evidence, produce a Finding, compute a Recommendation, or mutate Assessment state; every fact shown continues to originate from the engine's own `ReportSummary` response.
- No `Assessment` value is reopened, mutated, or reasoned about post-completion by anything this authorization permits — "assess another mod" creates a new value via the existing `execute_from_assessment_input` call, never reaches into a prior one.
- `INV-012` remains untouched and unimplicated — no code path introduced by this work interacts with Runtime lifecycle transitions at all.
- The two intake affordances remain two distinct, equally valid, non-competing actions — neither is presented as more "correct" than the other, consistent with `ASSESSMENT_INTAKE_AND_UPLOAD.md` §3.
- The "Assess another mod" reset touches only `Workspace.tsx`'s own local state — no Tauri call, no persistence write, no Runtime interaction of any kind occurs as part of the reset itself.
- `ReportSummary` and every nested DTO remain unchanged in field, meaning, and shape.
- `GOVERNANCE.md`, `FrontendArchitecture.md`, `INITIATIVE_2_ARCHITECTURAL_RESOLUTION.md`, and every ADR remain textually unmodified by this implementation.
- The `Reviewing.tsx` correction is textual only — the file's JSX, props, state, and rendering behavior remain byte-for-byte unchanged apart from the doc comment.

---

## 8. Required Verification Gates

- `cargo fmt --check` (root workspace) — clean, reconfirming no Rust file was touched (this work is not expected to require Rust changes at all; this gate is a repository-integrity check, not an anticipated site of change).
- `cargo check --workspace` — clean.
- `cargo test --workspace` — clean, full suite, 0 failures.
- `apps/console`'s TypeScript/build verification (`tsc && vite build`) — clean, following C13's own established precedent; no frontend test framework introduced.
- **Real extracted-mod assessment**, exercised by hand: a real, extracted FS25 mod directory selected and assessed successfully through the existing directory path, producing a real, non-mock report.
- **Real `.zip` assessment**, exercised by hand: a real FS25 mod `.zip` archive (this machine's own FS25 mods folder already contains suitable material) selected and assessed successfully through the newly authorized archive path, producing a real, non-mock report.
- **Assess → Review → "Assess another mod" → Intake**, exercised by hand: confirming the workspace returns to a fresh Intake presentation with no residual report, path, or error content visible.
- **Second independent assessment**, exercised by hand: a second, different real mod (directory or `.zip`, either combination) assessed immediately after the reset, confirming a real, independently produced report.
- **No stale report/path/error state**: direct visual confirmation that the second report's Findings, Evidence, and displayed subject label belong only to the second mod, with no trace of the first mod's content anywhere in the view.
- **C13 report presentation unchanged**: direct confirmation that `Overview.tsx` and `Reviewing.tsx`'s rendering behavior (dimension grouping, collapsed anatomy, evidence progressive disclosure) is identical to pre-C14 behavior for both intake shapes — no regression.
- **Repository diff limited to authorized files**: `git diff --stat` confirming only `apps/console/src/workspace/Workspace.tsx` (behavioral) and `apps/console/src/workspace/Reviewing.tsx` (comment-only) changed, with no other file — Rust, TypeScript, DTO, or capability — appearing in the diff.

---

## 9. Completion Criteria

- Both intake affordances (directory, `.zip`) are present, discoverable, and each independently produces a real, non-mock `ReportSummary` via the unchanged `submitAssessment` call.
- "Assess another mod" is reachable from Reviewing and reliably returns the workspace to a fresh Intake presentation with no residual state.
- A second, independent Assessment can be run immediately after the first, for a mod of either intake shape, producing a report that reflects only that second mod.
- `Reviewing.tsx`'s documentation comment accurately reflects the implemented behavior and correctly distinguishes it from Initiative 2.
- No `modiq-*` crate, Tauri command, DTO, transport, or capability file changed.
- No Initiative 2, GOV-002, or GOV-008 item was resolved, advanced, reopened, or implicated.
- C13's report presentation is unchanged, confirmed by direct comparison.
- Full verification gates (§8) pass clean.
- No item named in §5 (Explicit Exclusions) is touched, added, or implied.

**If implementation demonstrates that an additional file, DTO change, capability grant, or architectural mechanism beyond §3/§4 is genuinely required, implementation must stop and escalate rather than expanding scope autonomously** — this authorization does not delegate scope-expansion judgment to implementation time.

---

## 10. Expected Next Artifact

**C14 Implementation Plan** — translating this Authorization's scope into concrete phases (if more than one is warranted), the exact interaction design for the two intake affordances and the "Assess another mod" affordance within §5's reserved bounds, and testing sequencing. Not produced by this document.

---

## Status

This document defines the engineering envelope for C14's implementation and authorizes Implementation Planning within it. It does not authorize implementation beyond Section 3, nor any work named in Section 5. No TypeScript code, Rust code, or UI has been produced in preparing this document. No source file has been modified. Initiative 2, GOV-002, and GOV-008 remain exactly as unresolved and untouched as before this document.

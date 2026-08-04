# C1 Implementation Authorization — RecommendationStep Presentation

| Property | Value |
|---|---|
| **Document** | C1_IMPLEMENTATION_AUTHORIZATION.md |
| **Project** | modIQ |
| **Purpose** | Convert `PROCESS_DETERMINATION_C1_RECOMMENDATION_STEP_PRESENTATION.md`'s own conclusion into a bounded engineering envelope. Authorizes implementation in principle — participating crates, exclusions, invariants, verification gates, and completion criteria — and nothing beyond that. |
| **Origin** | `PROCESS_DETERMINATION_C1_RECOMMENDATION_STEP_PRESENTATION.md` (`12a8b89`), whose own conclusion is that C1 requires no Architecture Evaluation or Architectural Resolution and that an Implementation Authorization is the next required artifact. `CAPABILITY_DEFINITION_C1_RECOMMENDATION_STEP_PRESENTATION.md` (`d5b8735`) and `CAPABILITY_PORTFOLIO_ASSESSMENT.md` (`ca11328`, classifying C1 Category A) — both treated as fixed, unreopened. `FrontendArchitecture.md` (Approved) is the governing architecture. `FRONTEND_PRESENTATION_IMPLEMENTATION_AUTHORIZATION.md` (Sprint 23) is the directly on-point procedural precedent this document follows. |
| **Status** | **Approved. Authorizes C1 Implementation Planning within the scope defined below. No Rust type, TypeScript interface, field name, or code has been produced in preparing this document.** |

---

## 1. Authorization Decision

**Authorized with Scope Constraint.**

This determination is not newly reasoned here — it is `PROCESS_DETERMINATION_C1_RECOMMENDATION_STEP_PRESENTATION.md`'s own conclusion, converted into binding form. Nothing in this document revisits how that conclusion was reached, reweighs Sprint 21's precedent against Sprint 23's, or reopens the lighter Sprint 9/Sprint 20 route that determination examined and ruled out.

---

## 2. Architectural Basis

Derived exclusively from already-Adopted determinations; nothing here is newly reasoned:

- **`FrontendArchitecture.md`, Consumer-Owned State:** presentation state is the consumer's own decision, in full, as a direct and non-discretionary consequence of Initiative 5's Architectural Resolution. This is the entire basis for authorizing this crossing.
- **`FrontendArchitecture.md`, Explainable Continuity:** `Recommendation::repair_steps()` (`crates/modiq-runtime/src/assessment/recommendation.rs`) is already engine-produced content, populated by `VersionCompatibilityRule` at construction time — nothing this authorization permits is inferred, evaluated, or concluded at the presentation layer.
- **`FrontendArchitecture.md`, Reserved Responsibilities:** checked directly — neither Initiative 1 (Assessing/Reviewing derivation) nor Initiative 2 (post-completion/reentrant presentation) applies. This work concerns only what an already-completed Report displays, not when or whether it is live.
- **`apps/console/src-tauri/src/assessment.rs`'s own header comment**, unchanged since Sprint 21: `ReportSummary` is "provisional, not final or stable... held open pending GOV-008." Extending its field set is the kind of change this framing already anticipates.
- **`GOVERNANCE.md`'s Console Crate Boundary Rule:** Console owns "presentation of Assessment results" and "must never contain business logic" — squarely covers this work; not extended or reinterpreted by it.

---

## 3. Authorized Scope

Implementation Planning is authorized to scope work that:

- Extends the console's transport so that a Finding's Recommendation carries, in addition to its existing flat guidance text, the structured content `Recommendation::repair_steps()` already exposes — each step's `kind` and `instruction`, read through `RecommendationStep::kind()`/`RecommendationStep::instruction()`.
- Mirrors that extension in the TypeScript transport.
- Renders the structured content in `Reviewing.tsx`, distinguishable from the existing flat `action` text, within the existing single expansion layer — introducing no new navigation level.

This authorization is conditioned on all of the following:

- No field is added to any transport DTO that the Runtime does not already expose through an existing, public getter (`repair_steps()`, `kind()`, `instruction()`).
- No new Tauri command, IPC channel, or request/response cycle — the existing synchronous `submit_assessment` round trip is the only mechanism this work may use.
- `Recommendation::action`'s own meaning and content remain unchanged — supplemented, never replaced, exactly as Sprint 24's own Authorization already established.
- No change to `Workspace.tsx`'s Intake/Reviewing derivation, or to any navigation or interaction state shape Sprint 21 established.

No responsibility outside this list is authorized by this document.

---

## 4. Participating Crates

**Participating:**

- `console` (`apps/console/src-tauri`) — `assessment.rs`'s DTO extension.
- `apps/console` (TypeScript) — `src/engine/types.ts` mirror; `src/workspace/Reviewing.tsx` presentation.

**Not participating — confirmed unchanged by this Authorization:**

- `modiq-runtime`, `modiq-rules`, `modiq-storage`, `modiq-report`, `modiq-engine`, `modiq-cli`, `modiq-collection`, `modiq-knowledge`, `modiq-versioning`.
- `apps/sandbox` (separate workspace, out of scope).

---

## 5. Explicit Exclusions

**Architecturally blocked or reserved — unaffected by this document:**

- `RepairRecipeReference` — remains permanently unresolved; this authorization presents `repair_steps`, not a resolution mechanism for the reference.
- Initiative 1 (execution-phase signal granularity, progressive/in-place Report updates) and Initiative 2 (reentrancy, post-completion presentation) — both remain undecided design choices; nothing in this authorization touches Workspace-state derivation or in-progress presentation.
- GOV-008 (`AssessmentService` public API evolution) — untouched; the transport extended here remains provisional exactly as before, pending GOV-008's own resolution.
- Confidence — does not exist on the Runtime; nothing to expose.
- `DataModel.md`'s own Recommendation section, which does not yet describe `RepairRecipe`-derived structure — not amended by this authorization or the work it authorizes.

**Outside the scope of this authorization by implementation scope — no architectural decision is required for any of these; they simply are not part of this work:**

- The exact shape of the transport change — whether `FindingSummary.recommendation` (currently a flat `Option<String>`) becomes a nested structure carrying both `action` and `repair_steps`, or takes some other form. Sprint 23's own Authorization already named this a genuine design question it did not decide; it remains undecided here and is reserved for Implementation Planning.
- Any TypeScript interface or Rust struct definition for the new content.
- Any visual, interaction, or grouping treatment for `RecommendationStepKind`'s five values beyond plain-text presentation.
- Any change to `apps/sandbox` or `modiq-cli` presentation parity (the Capability Portfolio Assessment's own C4 and C3, separately deferred candidates).
- Any Sprint Plan, phase breakdown, or testing strategy.

---

## 6. Architectural Invariants That Must Not Be Violated

- `assessment.rs` remains the only module in the `console` crate permitted to depend on a `modiq-*` crate; `engine/index.ts` remains the only file calling `invoke` — confirmed as the crate's current, unique boundary crossings.
- No business logic, Evidence evaluation, Finding/Recommendation/Report generation, or Assessment mutation may exist anywhere in the implementation (Presentation Without Inference).
- No fact may be introduced by the consumer that the engine did not already produce — every presented value must trace to an existing, already-public Runtime getter (Explainable Continuity).
- No route-per-view navigation model; movement changes scope, never identity — unaffected and unchanged by this work.
- `Recommendation::action`'s own meaning and content remain unchanged — supplemented, never replaced.
- The Runtime/Knowledge boundary is not implicated — this work is presentation-only and touches neither `modiq-runtime`'s nor `modiq-knowledge`'s own boundary with the other.

---

## 7. Required Verification Gates

- `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` — both the root workspace and `apps/sandbox/src-tauri`'s own separate workspace.
- `apps/console`'s `npm run build` (`tsc && vite build`) clean.
- `console`'s existing real-fixture tests extended, not replaced or mocked — consistent with this repository's standing Real-I/O Testing Discipline.
- Direct confirmation (diff/grep, not assertion) that no field was added to any transport DTO without an existing, already-public Runtime getter backing it.
- Direct confirmation that `assessment.rs` remains the crate's only `modiq-*`-dependent file, and `engine/index.ts` remains the only file calling `invoke`.

---

## 8. Completion Criteria

- A Finding whose Recommendation carries a non-empty `repair_steps` is presented in `apps/console` with each step's `kind` and `instruction` visible, distinguishable from the existing flat `action` text.
- A Finding whose Recommendation carries an empty `repair_steps` presents exactly as today, with no error, placeholder, or synthesized content — an empty `Vec` is a valid, expected outcome, not a gap to fill.
- `Recommendation::action`'s existing behavior and every existing consumer remain unaffected.
- Full verification gates (§7) pass clean.
- No item named in §5 (Explicit Exclusions) is touched, added, or implied.

---

## 9. Expected Next Artifact

**C1 Implementation Plan** — translating this Authorization's scope into concrete phases, the exact transport shape reserved by §5, TypeScript interface and Rust struct design, and testing sequencing. Not produced by this document.

---

## Status

This document defines the engineering envelope for C1's implementation and authorizes Implementation Planning within it. It does not authorize implementation beyond Section 3, nor any work named in Section 5. No Rust type, TypeScript interface, field name, or code has been produced in preparing this document.

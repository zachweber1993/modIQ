# Frontend Presentation Implementation Authorization

| Property | Value |
|---|---|
| **Document** | FRONTEND_PRESENTATION_IMPLEMENTATION_AUTHORIZATION.md |
| **Project** | modIQ |
| **Purpose** | Authorize the next `apps/console` implementation round: presentation-layer propagation of Runtime fields Sprint 22 (Initiative 3) already added, verified by direct source review to be built but not yet exposed. Not a new capability. Not a Runtime, Rule, Collector, or Assessment lifecycle change. |
| **Origin** | `FrontendArchitecture.md` (Approved), `FRONTEND_IMPLEMENTATION_AUTHORIZATION.md` and Sprint 21 (implemented and closed), `INITIATIVE_3_ARCHITECTURAL_RESOLUTION.md` and `INITIATIVE_3_IMPLEMENTATION_AUTHORIZATION.md`, Sprint 22 implementation (`969e595`) and closeout (`80c79f4`) — all treated as fixed, unreopened repository history. |
| **Status** | **Approved. Authorizes Sprint 23 Planning within the scope defined below. No application, crate, or code has been produced in preparing this document; implementation has not begun.** |

---

## 1. Executive Summary

Sprint 22 gave `Finding`, `Evidence`, and `RepairRecipe` real field anatomy — Title/Summary, `ModHealthDimension`, `FindingStatus`, Evidence `label`/`source`/`content`, and structured `RepairRecipe` steps — entirely within `modiq-runtime`, `modiq-rules`, `modiq-knowledge`, `modiq-collection`, `modiq-engine`, `modiq-report`, `modiq-storage`, and `modiq-cli`. A direct re-verification of current source (Section 4) confirms most of this content never reaches `apps/console`'s own transport, and one field that does reach it is never rendered. This is real, already-adopted, already-implemented capability sitting inert behind a boundary that `FrontendArchitecture.md` already grants full authority to cross.

This document authorizes exactly that crossing — extending `apps/console`'s existing presentation-layer DTOs and components to carry and display what the Runtime already produces — and nothing else. It follows the same procedural shape as `FRONTEND_IMPLEMENTATION_AUTHORIZATION.md` (Sprint 21), at a proportionally smaller scope: no new boundary is being established, no new mechanism is being designed, only an existing, already-adopted one is being extended to carry more of what it was always entitled to carry.

---

## 2. Authorization Decision

**Authorized with Scope Constraint.**

No Architecture Evaluation or Architectural Resolution precedes this document because none is required — Section 7 states precisely why. This determination is not newly reasoned here: `FrontendArchitecture.md`'s Consumer-Owned State section already, and non-discretionarily, settles the only architectural question this work could raise. What this document performs is the same function `FRONTEND_IMPLEMENTATION_AUTHORIZATION.md` performed for Sprint 21 — converting settled architecture into a bounded, permitted engineering envelope — not re-deriving or revisiting the architecture itself.

---

## 3. Success Criterion

This authorization is satisfied when `apps/console` presents Title, `ModHealthDimension`, `FindingStatus`, and Evidence `label`/`source`/`content` — fields Sprint 22 already added to the Runtime — using only what the Runtime already exposes.

**It authorizes exposing existing Runtime capability through the frontend presentation layer. It does not authorize introducing new Runtime capability, new transport semantics, or new architectural concepts. Any additional data required beyond what Sprint 22 already implemented falls outside the scope of this Authorization** and must return to Architecture Evaluation — it is not Sprint 23 implementation's to absorb, however small it appears.

This is the bright-line test for this document, in the same spirit as Item 9's naming constraint or GOV-004's convergence threshold: implementation that renders fewer than the fields above is incomplete; implementation that reaches back into `modiq-runtime`, `modiq-rules`, `modiq-collection`, `modiq-knowledge`, or any other engine crate to add something new — a field, a variant, a new getter exposing previously-internal state — has exceeded this Authorization regardless of intent or apparent size.

---

## 4. Current State — Verified Directly Against Source

Re-verified this session, not taken from `SPRINT22_IMPLEMENTATION_REPORT.md` or any other narrative document:

| Runtime field (`modiq-runtime`) | In `apps/console/src-tauri/src/assessment.rs` DTOs | In `apps/console/src/engine/types.ts` | Rendered in `Reviewing.tsx` / `Overview.tsx` |
|---|---|---|---|
| `Finding::title()` | ✅ `FindingSummary.title` | ✅ | ❌ — never referenced in either component |
| `Finding::summary()` | ✅ `FindingSummary.summary` | ✅ | ✅ (`Reviewing.tsx`) |
| `Finding::mod_health_dimension()` | ❌ absent from `FindingSummary` | ❌ absent | ❌ |
| `Finding::status()` | ❌ absent from `FindingSummary` | ❌ absent | ❌ |
| `Evidence::label()` | ❌ — `EvidenceSummary` carries only `description`/`location` | ❌ absent | ❌ |
| `Evidence::source()` | ❌ | ❌ absent | ❌ |
| `Evidence::content()` | ❌ | ❌ absent | ❌ |
| `RepairRecipe`'s `Vec<RepairStep>` | ❌ — `Recommendation` (and therefore `FindingSummary.recommendation`) exposes only a flat `action: String`; individual steps are not reachable from `Recommendation` at all | ❌ absent | ❌ |

Confirmed directly:
- `apps/console/src/engine/types.ts`'s own doc comment already states this gap in its own words: *"No Mod Health dimension, no Evidence `Content`/`Label`/`Source` field, no Confidence — Sprint 22 (Initiative 3) added these to the Runtime types, but nothing on this side of the boundary consumes them yet."*
- `apps/console/src/workspace/Overview.tsx`'s own doc comment is now stale in one specific respect: it states Mod Health "depends on a Finding-level Category field that does not exist on the Runtime yet (Initiative 3, unimplemented)" — `ModHealthDimension` exists on the Runtime as of Sprint 22; the comment predates that Sprint and was not updated by it, since Sprint 22's own authorized scope never included `apps/console`'s presentation layer. Correcting this comment is in scope as part of implementing this authorization (Section 5); it is not, on its own, evidence of any conflict.
- Transport additions **are** required: `FindingSummary`/`EvidenceSummary` (Rust) and their TypeScript mirrors do not currently carry `mod_health_dimension`, `status`, `label`, `source`, or `content` at all. This is ordinary DTO extension, not a boundary change — `assessment.rs`'s own header comment already frames `ReportSummary` as provisional, held open pending GOV-008, so extending its field set is anticipated evolution, not a new decision.
- No `FrontendArchitecture.md` constraint limits this work. Checked directly against its Consumer-Owned State and Reserved Responsibilities sections (Section 7, below) — neither reservation (Initiative 1's Assessing/Reviewing derivation; Initiative 2's reentrancy/post-completion presentation) applies to displaying additional fields on an already-completed, already-received Report.

---

## 5. Authorized Scope

Sprint 23 is authorized to implement only the following:

- Extend `apps/console/src-tauri/src/assessment.rs`'s `FindingSummary` to carry `mod_health_dimension` and `status`, read from `Finding::mod_health_dimension()`/`Finding::status()`.
- Extend `EvidenceSummary` to carry `label`, `source`, and `content`, read from `Evidence::label()`/`Evidence::source()`/`Evidence::content()`.
- Mirror both extensions in `apps/console/src/engine/types.ts`.
- Render `title` (already in the DTO, currently unused) alongside `summary` in `Reviewing.tsx`.
- Render `mod_health_dimension`, `status`, and Evidence's `label`/`source`/`content` in `Reviewing.tsx` and, where the existing derived-view discipline supports it (severity-count-style aggregation only), `Overview.tsx`.
- Correct `Overview.tsx`'s own doc comment, which currently misstates `ModHealthDimension` as not existing on the Runtime.

This authorization is conditioned on all of the following:
- No field is added to any transport DTO that the Runtime does not already expose through an existing, public getter.
- No new Tauri command, IPC channel, or request/response cycle is introduced — the existing synchronous `submit_assessment` round trip is the only mechanism this work may use.
- `RepairRecipe`'s structured steps remain excluded (Section 6) — only `Recommendation::action()`'s existing flat string may be presented, exactly as today.
- No change to `Workspace.tsx`'s Intake/Reviewing derivation, or to any navigation or interaction state shape Sprint 21 established.

No responsibility outside this list is authorized by this document.

---

## 6. Explicit Exclusions

**Architecturally blocked — no separate authorization can substitute for the missing repository decision, unaffected by this document:**

- Initiative 3 Item 6a (Report Identity / `AssessmentSubject`) — remains blocked on its own Adopted Implementation Prerequisite; `AssessmentSubject` is confirmed, by direct source review, still a zero-field marker (`pub struct AssessmentSubject;`), untouched by Sprint 22 and untouched by this authorization.
- Initiative 3 Item 4 (Evidence's Explanation field) — remains "Requires Additional Investigation."
- Initiative 3 Item 3C (Report currency, the Updated marker) — remains deferred pending Initiative 2.
- Initiative 1 D-1 (execution-phase signal granularity) and D-2 (Report-evolution resolution model) — both remain undecided design choices; nothing in this authorization touches Workspace state derivation or in-progress presentation.
- GOV-001 (Assessment Report Generation Timing / `Completed` status semantics) — untouched; this work concerns `FindingStatus` (Provisional/Final), a distinct field from Assessment-level status, and does not bear on GOV-001's question.
- GOV-002 (`AssessmentSubject` minimal content) — untouched; not reopened, not reconsidered.
- GOV-008 (`AssessmentService` public API evolution) — untouched; the transport extended here remains provisional exactly as before, pending GOV-008's own resolution.
- GOV-013 (`FindingSeverity` Severity/Kind conflation) — untouched; this work does not touch `FindingSeverity` in any respect.
- GOV-014 (Lua Fixture Acquisition Governance) — unrelated, untouched.
- GOV-016 (Evidentiary Standard for Governance Decisions) — unrelated, untouched.
- Confidence — does not exist on the Runtime; nothing to expose.
- Report evolution, progressive execution, any live/in-progress presentation — reserved pending Initiative 1, per `FrontendArchitecture.md`'s own Reserved Responsibilities.
- Assessment lifecycle — no mutation, reentrancy, or supplementation behavior is authorized or implied.
- `AssessmentService` API redesign — the current public contract (`execute`, `execute_from_assessment_input`) is the only mechanism this work may call, unchanged.
- New frontend interaction mechanisms — `expandedFindingId`'s existing navigation model (Sprint 21) is not modified; new fields are presented within the existing expansion layer, not a new one.

**Outside the scope of this authorization (by implementation scope) — no architectural decision is required for any of these; they simply are not part of Sprint 23:**

- `RepairRecipe`'s structured steps reaching the transport (would require deciding how `Recommendation` — currently a flat `action: String` plus an opaque `RepairRecipeReference` — should expose step-level content; a genuine design question this authorization does not decide).
- Any visual design system, icon set, or grouping treatment for `ModHealthDimension`'s six values beyond plain text presentation.
- Any change to `apps/sandbox` (a separate workspace, not in scope).

---

## 7. Architectural Basis

Derived exclusively from already-adopted determinations; nothing here is newly reasoned:

- **`FrontendArchitecture.md`, Consumer-Owned State:** *"Presentation state, interaction state, and navigation state are owned by the consumer, in full, as a direct and non-discretionary consequence of Initiative 5's Architectural Resolution. None of the three has any engine-side representation, and none requires one."* This is the entire architectural basis for this authorization: what a consumer displays about an already-completed Assessment is already, fully, its own decision.
- **`FrontendArchitecture.md`, Explainable Continuity:** *"Whatever the consumer presents about an Assessment's state — current, still forming, or historical — must be traceable to something the engine actually produced. The consumer introduces no fact of its own about whether, or how well, an Assessment is proceeding."* Every field this authorization exposes is already engine-produced (Sprint 22); none is invented at the presentation layer. This principle is satisfied, not extended.
- **`FrontendArchitecture.md`, Reserved Responsibilities:** checked directly — the only two reservations (Initiative 1: Assessing/Reviewing derivation and progressive updates; Initiative 2: post-completion/reentrant presentation) both concern *when* or *whether* a Report is live or evolving. This work concerns only *what* an already-completed, already-received Report displays. Neither reservation applies.
- **`assessment.rs`'s own header comment**, unchanged since Sprint 21: `ReportSummary` is "provisional, not final or stable... held open pending GOV-008." Extending its field set is the kind of change this framing already anticipates, not a new architectural event.

**Why no Architecture Evaluation is required:** an Architecture Evaluation exists in this repository's vocabulary to gather and classify evidence toward a genuinely open design question. There is none here — Consumer-Owned State is Adopted, not deferred, and Explainable Continuity is satisfied by construction (the fields are already engine-produced facts).

**Why no Architectural Resolution is required:** a Resolution exists to disposition alternatives among a real architectural question. No alternative is being weighed — this document authorizes exposing data, not deciding how the engine/consumer boundary itself should work.

**Why an Implementation Authorization is still appropriate:** per this repository's own canonical workflow (`PROJECT_HANDOFF_v1.1.md` §5), Implementation Authorization is a distinct stage from Architectural Resolution — it "confirms every remaining precondition is satisfied and gives explicit, recorded permission to begin," independent of whether new architecture was produced. The directly on-point precedent is Sprint 21 itself: `FrontendArchitecture.md` (Approved) → `FRONTEND_IMPLEMENTATION_AUTHORIZATION.md` → `SPRINT21_PLAN.md`. This document follows that same sequence, for the same subsystem, under the same governing architecture, at a smaller scope — consistent with the Procedural Validation performed immediately prior to this document (see that review for the full precedent comparison, including why the lighter Sprint 9/Sprint 20 path was not selected here).

---

## 8. Implementation Constraints

Restated directly from `FrontendArchitecture.md` and Sprint 21's own precedent; none originates here:

- No business logic, Evidence evaluation, Finding/Recommendation/Report generation, or Assessment mutation anywhere in the implementation.
- No redefinition of any Runtime Architecture, Platform Architecture, Product Design, or Interaction Design concept.
- No route-per-view navigation model; movement changes scope, never identity (unchanged from Sprint 21).
- Completion is recognized by absence, never by an explicit signal (`EngineAPI.md` AC-5) — unaffected by this work.
- No design decision that presupposes an answer to Initiative 1's granularity question, Initiative 2's reentrancy question, or GOV-008's payload question.
- No implementation technology, framework, or library selected under this authorization's own authority — this work extends existing files in an existing stack.
- Every new DTO field must trace to an existing, already-public Runtime getter — no field may be synthesized, inferred, or defaulted at the presentation layer.

---

## 9. Crate / Application Impact

| Area | Expected change |
|---|---|
| `apps/console/src-tauri` (`console` crate) | `assessment.rs` — `FindingSummary`, `EvidenceSummary` gain fields; `From<&Finding>`/`From<&Evidence>` construction sites updated. No other module changes — `assessment.rs` remains the crate's only `modiq-*`-dependent file. |
| `apps/console` (TypeScript) | `src/engine/types.ts` — mirrored field additions. `src/workspace/Reviewing.tsx` — render `title`, `mod_health_dimension`, `status`, Evidence provenance. `src/workspace/Overview.tsx` — doc-comment correction; aggregate presentation only if it stays within the existing derived-view discipline. |
| `modiq-runtime`, `modiq-rules`, `modiq-knowledge`, `modiq-collection`, `modiq-engine`, `modiq-report`, `modiq-storage`, `modiq-cli` | **Unaffected.** No Runtime, Rule, Collector, Engine, Reporting, Storage, or CLI change is authorized or anticipated. |
| `apps/sandbox` | Unaffected — separate workspace, out of scope. |
| `AssessmentService` public entry points | Unaffected. |
| Governance Register | Unaffected — no item opened, modified, or implicitly resolved by this authorization. |
| ADRs | None new. |

---

## 10. Risks

- **`Recommendation`'s flat `action: String` may read as inconsistent once Evidence and Finding both present richer, structured content.** Not a defect this authorization creates — `RepairRecipe`'s structured steps are explicitly excluded (Section 6) — but Sprint 23 Planning should be aware the contrast may be visible once implemented.
- **`ModHealthDimension`'s six values have no existing visual or grouping precedent in `apps/console`.** Plain-text presentation is authorized; any richer treatment is out of scope (Section 6) and should not be improvised mid-implementation.
- **`Overview.tsx`'s doc-comment correction touches a file whose own text currently asserts something false** (`ModHealthDimension` "does not exist on the Runtime yet"). The correction is authorized and should be precise — restating only what is now true, not expanding the comment's own scope.
- **Transport instability remains a standing, pre-existing condition, not a new risk this work introduces.** `ReportSummary` was provisional before this authorization and remains provisional after it; nothing here is a commitment toward its eventual, GOV-008-resolved final shape.

---

## 11. Testing Expectations

Per repository precedent, restated, not newly invented:

- `console`'s existing real-fixture tests (`assessment.rs`, 4 tests as of Sprint 22) must be extended to assert the new `FindingSummary`/`EvidenceSummary` fields are populated from real Assessment output — no mocking, matching this repository's standing discipline since Sprint 3 Phase 5.
- No existing test may be deleted; only extended, matching Sprint 22's own precedent for this exact crate.
- `npm run build` (`tsc && vite build`) must remain clean.
- Full workspace (`cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace`) and `apps/sandbox/src-tauri`'s own suite must both be reverified unaffected, even though neither is expected to change — confirming this work's boundary claim empirically, not just by assertion.

---

## 12. Repository Impact

Crates touched: `console` only (Rust) plus its own TypeScript layer. No new crate. No new external dependency anticipated. No change to any workspace member list. Test count expected to grow only within `console`'s own suite (currently 4/4); root workspace (269/269) and Sandbox (9/9) expected unchanged.

---

## 13. Non-Actions

No Architecture Evaluation performed. No Architectural Resolution performed. No ADR created. No Governance Register entry opened, modified, or implicitly resolved. No Runtime, Rule, Collector, Engine, Reporting, Storage, or CLI crate touched. No Sprint Plan produced — phasing and task breakdown remain Sprint 23 Planning's own work, not this document's. No implementation begun in preparing this document.

---

## Status

This document defines the engineering envelope for exposing Sprint 22's already-implemented `Finding`/`Evidence` field anatomy through `apps/console`'s existing presentation layer, and authorizes Sprint 23 Planning within it. It does not authorize any work outside Sections 3 and 5, nor any item named in Section 6. No application, crate, technology selection, or code has been produced in preparing this document.

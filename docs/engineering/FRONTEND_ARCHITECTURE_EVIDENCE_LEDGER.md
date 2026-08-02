# Frontend Architecture Evidence Ledger

| Property | Value |
|---|---|
| **Document** | FRONTEND_ARCHITECTURE_EVIDENCE_LEDGER.md |
| **Project** | modIQ |
| **Purpose** | An itemized ledger of implementation evidence against each `FrontendArchitecture.md` responsibility, classified by a fixed taxonomy. Extends `IMPLEMENTATION_REPORT_FRONTEND_CONSOLE.md`'s own six-item Architectural Validation section to cover every named responsibility that section did not address. |
| **Repository state reviewed** | `76ccd8e`, `b63464d`, `c4efc22` (Sprint 21 implementation), `3a64d54` (Sprint 21 closeout) |
| **Documentation Authority** | None. Subordinate to `FrontendArchitecture.md` if any conflict arises. |
| **Amendment policy** | Permanent point-in-time record. Never amended. New evidence belongs in a future document, not an edit here. |
| **Status** | Final. |

---

## Method

Each entry names one responsibility, states the implementation evidence inspected, and assigns exactly one classification:

- **Empirically Validated** — exercised by real implementation; held without exception.
- **Partially Validated** — exercised in part; the unexercised remainder is named.
- **Not Exercised** — no scenario requiring this responsibility occurred.
- **Reserved by Architectural Scope** — not attempted, matching `FrontendArchitecture.md`'s own Reserved Responsibilities.

---

## Constitutional Foundation

**Boundary Enforcement.** Evidence: `apps/console/src-tauri/src/assessment.rs` is the crate's only module importing `modiq_engine`/`modiq_report`/`modiq_runtime`. `apps/console/src/engine/index.ts` is the only file importing `invoke`. `ReportSummary`'s construction (`assessment.rs`) performs only id-based lookup and formatting over engine-produced values. `Cargo.toml` declares the three `modiq-*` crates at crate level, not scoped to `assessment.rs` specifically.
**Classification: Empirically Validated.**

---

## Core Responsibilities

**Application Shell and Region Composition.** Evidence: `App.tsx` composes `SessionProvider` with a session-gated `Shell`; `Console.tsx` hosts `Dashboard`/`Workspace` with no state beyond which region is active.
**Classification: Empirically Validated.**

**Consumer-Owned State.** Evidence: no engine-side representation of presentation, interaction, or navigation state exists in `Console.tsx`, `Dashboard.tsx`, `Workspace.tsx`, `Reviewing.tsx`, or `SessionContext.tsx`.
**Classification: Empirically Validated.**

**Workspace Ownership.** Evidence: `Workspace.tsx` derives workspace state from `report`'s presence, with no tracked variable and no engine-side representation. Of the three named workspace values, two (Intake, Reviewing) were reachable; no code path represents Assessing.
**Classification: Partially Validated.**

**Navigation Realization.** Evidence: `Reviewing.tsx` uses a single `expandedFindingId` value; the same setter both expands and collapses it; `Overview` renders regardless of expansion state; no routing library exists in `apps/console/package.json` or the source tree.
**Classification: Empirically Validated.**

**Workspace Realization: Structure and Mandatory Entry Point.** Evidence: `Reviewing.tsx` renders `Overview` above the Finding list in every case; no code path reaches Finding detail without `Overview` present. Exercised for one Assessment Subject per session; no multi-Subject Workspace occurred.
**Classification: Partially Validated.**

**Workspace Realization: Progressive Update During Assessing.** Evidence: no progressive or in-place update of the Overview during Assessing occurs anywhere in the inspected source.
**Classification: Reserved by Architectural Scope.**

**Request/Response Mechanism: Exclusive Boundary Crossing.** Evidence: `submit_assessment` is a single async command; `submitAssessment()` (`engine/index.ts`) awaits and resolves or rejects; no polling or second call type exists in the codebase.
**Classification: Empirically Validated.**

**Request/Response Mechanism: Completion Recognized by Absence.** Evidence: the request is a single, synchronous round trip with no intermediate state; no progressive-uncertainty scenario occurred for this principle to apply to.
**Classification: Not Exercised.**

**Identity and Session Mechanism.** Evidence: `SessionContext.tsx` and `session/types.ts` construct a typed, in-memory `Session` (`Role: "Owner" | "Member"`, auto-provisioned `Project`); no credential-verification call exists; nothing persists beyond the running session.
**Classification: Empirically Validated.**

**Error and Latency Handling: Content-Level Conditions.** Evidence: `Workspace.tsx` handles two conditions — folder-picker cancellation (no state change, no error) and submission failure (`catch`/`setError`/visible message). The complete named condition set was not independently re-verified against this evidence; Console's directory-only intake path may not reach every named condition.
**Classification: Partially Validated.**

**Error and Latency Handling: Attention and Absence During Execution.** Evidence: no Assessing-state view, progress indicator, or attention/absence handling exists in `Workspace.tsx`; the `"submitting"` status is documented in-code as transient Intake-phase state.
**Classification: Reserved by Architectural Scope.**

**Engine Transport Failure Handling.** Evidence: `Workspace.tsx`'s `catch` block sets a distinct failure state, rendered as an explicit message; `Reviewing.tsx` and `Overview.tsx` do not render on this path since `report` remains `null`.
**Classification: Empirically Validated.**

---

## Consolidated Ledger

| Responsibility | Classification |
|---|---|
| Boundary Enforcement | Empirically Validated |
| Application Shell and Region Composition | Empirically Validated |
| Consumer-Owned State | Empirically Validated |
| Workspace Ownership | Partially Validated |
| Navigation Realization | Empirically Validated |
| Workspace Realization: Structure and Mandatory Entry Point | Partially Validated |
| Workspace Realization: Progressive Update During Assessing | Reserved by Architectural Scope |
| Request/Response Mechanism: Exclusive Boundary Crossing | Empirically Validated |
| Request/Response Mechanism: Completion Recognized by Absence | Not Exercised |
| Identity and Session Mechanism | Empirically Validated |
| Error and Latency Handling: Content-Level Conditions | Partially Validated |
| Error and Latency Handling: Attention and Absence During Execution | Reserved by Architectural Scope |
| Engine Transport Failure Handling | Empirically Validated |

**Composition: 7 Empirically Validated, 3 Partially Validated, 2 Reserved by Architectural Scope, 1 Not Exercised, evaluated against the repository state named above.**

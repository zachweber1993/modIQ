# Frontend Implementation Authorization

| Property | Value |
|---|---|
| **Document** | FRONTEND_IMPLEMENTATION_AUTHORIZATION.md |
| **Project** | modIQ |
| **Purpose** | Convert the approved Frontend Implementation Readiness Assessment into a formal engineering envelope for Sprint 21 — authorized scope and binding constraints — without specifying how implementation proceeds. |
| **Origin** | Frontend Implementation Readiness Assessment (this session), grounded in `FrontendArchitecture.md`, the Frontend Architecture Architectural Resolution, and `ENGINEERING_ALIGNMENT_PROGRAM.md` Initiatives 1, 2, and 5. |
| **Status** | **Approved. This document authorizes Sprint 21 Planning within the scope defined below. No application, crate, technology, or code has been produced in preparing it; implementation itself has not begun.** |

---

## 1. Authorization Decision

**Authorized with Scope Constraint.**

This determination is not newly reasoned here — it is the Frontend Implementation Readiness Assessment's own conclusion, converted into binding form. Nothing in this document revisits how that conclusion was reached.

---

## 2. Constitutional Basis

The Frontend Architecture Architectural Resolution adopted the constitutional responsibilities implemented by `FrontendArchitecture.md`. This authorization derives its scope exclusively from those adopted determinations, and from nothing else:

- Initiative 5's Architectural Resolution (Decisions 1, 2, 3, 4a, 5a, 6 — all Adopted, none reopened) supplies the engine/consumer boundary and the request/response mechanism baseline this authorization implements.
- `Architecture.md`'s Dependency Rules and Platform Boundaries supply the constraint this authorization enforces, not a new one.
- The current `AssessmentService` public contract remains, per GOV-008, "the approved platform boundary until future implementation provides additional evidence" — the one mechanism this authorization permits implementation to call.
- `PROJECT_STATUS.md` confirms no prior Sprint has touched this scope; this authorization revises no existing implementation.

---

## 3. Authorized Scope

Sprint 21 is authorized to implement only the following architectural responsibilities:

- Boundary Enforcement
- Application Shell and Region Composition
- Consumer-Owned State
  - presentation
  - interaction
  - navigation
  - workspace, limited to Intake and Reviewing
- Navigation Realization
- Workspace Realization (Intake and Reviewing only)
- Request/Response Mechanism
  - synchronous submit-and-receive-completed-Report only
- Identity and Session Mechanism
- Error and Latency Handling
- Engine Transport Failure Handling

This authorization is conditioned on all of the following:

- Implementation operates exclusively against the currently approved public `AssessmentService` contract.
- Implementation produces a synchronous Intake → Reviewing experience.
- Implementation introduces no dependency on Initiative 1, Initiative 2, or GOV-008's resolution.

No responsibility outside this list is authorized by this document.

---

## 4. Explicitly Unauthorized Work

**Architecturally blocked — no separate authorization can substitute for the missing repository decision:**

- Assessing-state derivation, its granularity, and any live or progressive presentation of it (Initiative 1: granularity "intentionally unresolved"; the one-shot-generation/progressive-Overview tension "requires future resolution").
- Reentrancy, post-completion supplementation, or "living Report" behavior (Initiative 2: "cannot presently be reconciled under the repository's current governance model").
- Treating the request/response payload shape as final or stable (GOV-008: "Status remains Open").
- Any supplementary notification or poll-trigger mechanism (Initiative 5, Decision 5b: Deferred, pending Initiative 1).

**Outside the scope of this authorization (by implementation scope) — no architectural decision is required for any of these; they simply are not part of Sprint 21:**

- Region richness beyond the minimum composition this scope requires — full Dashboard presentation across multiple Projects, and any Settings region implementation.
- Identity and session capability beyond what is required to gate this scope — full Platform Architecture Membership/Role/Organization implementation.
- Assessment Input acquisition beyond a single Assessment Subject type sufficient to exercise this scope.

---

## 5. Implementation Constraints

Restated directly from `FrontendArchitecture.md`'s Constitutional Exclusions and the Readiness Assessment; none originates here:

- No business logic, Evidence evaluation, Finding/Recommendation/Report generation, or Assessment mutation anywhere in the implementation.
- No redefinition of any Runtime Architecture, Platform Architecture, Product Design, or Interaction Design concept.
- No route-per-view navigation model; movement changes scope, never identity.
- No presentation of an engine transport failure as an assessment outcome.
- Completion is recognized by absence, never by an explicit signal (`EngineAPI.md` AC-5).
- No design decision that presupposes an answer to Initiative 1's granularity question, Initiative 2's reentrancy question, or GOV-008's payload question.
- No implementation technology, framework, or library selected under this authorization's own authority.

---

## 6. Required Implementation Work

The following are included within, and required by, the authorized scope — they are implementation work this authorization directs, not prerequisites that must exist before it can take effect:

- **Real Assessment Input acquisition**, in place of any synthetic or hardcoded stand-in. `EvidenceCollection.md` already assigns this to the application layer as a settled boundary.
- **A real identity/session mechanism**, in place of no mechanism at all. `IdentityAndAccess.md` already defines the concepts it must respect and explicitly disclaims owning the mechanism itself.
- Platform Architecture representation is included within the required implementation work of this Sprint only to the extent necessary for Application Shell and Region Composition and Identity and Session Mechanism to function as already authorized in Section 3. Any Platform Architecture representation beyond that extent is not authorized by this section and remains excluded under Section 4.

---

## 7. Recommendation

**Proceed to Sprint 21 Planning.**

This document authorizes Sprint 21 Planning to begin within the scope defined in Section 3. It does not authorize implementation beyond that scope, nor work reserved to the architectural or governance decisions named in Section 4. It does not itself perform Sprint Planning — sequencing, phasing, and task breakdown remain Sprint Planning's own work, not this document's.

---

## Status

This document defines the engineering envelope and authorizes Sprint 21 Planning within it. It does not authorize implementation beyond the scope defined in Sections 3 and 6, nor work reserved by Section 4. No application, crate, API, technology selection, or code has been produced in preparing this document. Approved and synchronized into the repository 2026-08-02.

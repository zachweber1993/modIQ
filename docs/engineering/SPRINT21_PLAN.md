# Sprint 21 Planning — Frontend Implementation

| Property | Value |
|---|---|
| **Document** | SPRINT21_PLAN.md |
| **Project** | modIQ |
| **Purpose** | Translate the approved Frontend Implementation Authorization into a concrete engineering execution plan, entirely within its authorized envelope. Sprint Planning — not implementation. |
| **Origin** | Frontend Architecture Necessity Justification Evaluation (`docs/engineering/FRONTEND_ARCHITECTURE_NECESSITY_JUSTIFICATION_EVALUATION.md`, committed) → Documentation Authority Decision → Frontend Architecture Evaluation → Frontend Architecture Architectural Resolution → `FrontendArchitecture.md` (committed) → Frontend Implementation Readiness Assessment → Frontend Implementation Authorization (`docs/engineering/FRONTEND_IMPLEMENTATION_AUTHORIZATION.md`, committed) — all this session; the Documentation Authority Decision, the Frontend Architecture Evaluation, the Frontend Architecture Architectural Resolution, and the Frontend Implementation Readiness Assessment remain approved session determinations and have not been committed as standalone repository documents. |
| **Status** | **Sprint Planning draft only. Not yet approved. No application, technology, or code has been produced in preparing it. Implementation does not begin on the basis of this document alone.** |

**Fixed, per Implementation Authorization §3–§6 — not revisited by this plan:** the authorized responsibility set, its conditioning (current public `AssessmentService` contract only, synchronous experience only, no dependency on Initiative 1/2/GOV-008), the two explicitly required implementation items (real Assessment Input acquisition, a real identity/session mechanism), and every item named Explicitly Unauthorized. No conflict with any of these was found while producing this plan.

---

## 1. Executive Summary

Sprint 21 builds the first real increment of modIQ's production frontend: an application, independent of `apps/sandbox`, that takes a real user from an empty Workspace through a real Assessment submission to a fully realized Reviewing experience of one completed Assessment Report — synchronously, session-gated, and structurally incapable of crossing into the engine's own responsibilities. Work is organized into three sequential phases, with Boundary Enforcement validated as a standing constraint at every phase gate.

---

## 2. Sprint Objective

Produce a running, session-gated production consumer application capable of taking one real Assessment Subject from submission through a fully navigable Reviewing experience of its completed Assessment Report, entirely within the Implementation Authorization's envelope.

---

## 3. Sprint Scope

Organized by phase (see §9 for the work breakdown):

- **Foundation:** the structural separation Boundary Enforcement requires; the Application Shell (Console containing, at minimum, Dashboard and Workspace); a real identity/session mechanism gating the shell.
- **Intake and submission:** real Assessment Input acquisition for one Assessment Subject type; the Request/Response Mechanism's submit-and-await path against the current public `AssessmentService` contract; content-level error handling; Engine Transport Failure Handling for this call.
- **Reviewing and navigation:** presentation of a received, completed Assessment Report in the Workspace's Reviewing state, Assessment Overview as mandatory entry point; Navigation Realization (single continuous object, no route-per-view, single-step locality, symmetric reversal) across Overview, Finding, Recommendation, and Evidence; presentation, interaction, and navigation state fully exercised; workspace state holding only its Intake and Reviewing values.

---

## 4. Explicit Out-of-Scope Work

Restated in full from Implementation Authorization §4, not narrowed or reinterpreted:

**Architecturally blocked:**
- Assessing-state derivation, its granularity, and any live or progressive presentation of it (Initiative 1).
- Reentrancy, post-completion supplementation, or "living Report" behavior (Initiative 2).
- Treating the request/response payload shape as final or stable (GOV-008).
- Any supplementary notification or poll-trigger mechanism (Initiative 5, Decision 5b).

**Outside this authorization's scope, by implementation-scope choice, not architectural blocker:**
- Dashboard presentation across multiple Projects; any Settings region implementation.
- Identity/session capability beyond what gates this scope.
- Assessment Input acquisition for more than one Assessment Subject type.

---

## 5. Implementation Deliverables

- A running consumer application, independent of `apps/sandbox`, structurally separating presentation code from any code capable of engine-side evaluation.
- A session-gated entry point recognizing a real User and their standing.
- A working submission path accepting one real Assessment Subject and producing either a completed Assessment Report or a well-handled transport failure — never a silent or ambiguous outcome.
- A Reviewing experience presenting that Report's Overview, Findings, Recommendations, and Evidence as one continuous, navigable object, exactly per `NAVIGATION_AND_WORKSPACE_BEHAVIOR.md`.

---

## 6. Acceptance Criteria

- [ ] No business logic, Evidence evaluation, Finding/Recommendation/Report generation, or Assessment mutation exists anywhere in the application.
- [ ] A single, auditable structural separation exists between presentation code and any code that calls or interprets engine output.
- [ ] All engine calls are made against the current public `AssessmentService` contract only, synchronously — no polling, no intermediate observation.
- [ ] Completion is recognized by the presence of a returned Report or failure, never by an explicit "in progress" or "done" signal invented by the consumer.
- [ ] No route, URL, or navigational mechanism treats Overview/Finding/Recommendation/Evidence as separate destinations; every movement narrows or widens focus on one object.
- [ ] Every content-level condition named in `ASSESSMENT_INTAKE_AND_UPLOAD.md` §6 is handled exactly as specified.
- [ ] An engine transport failure is never presented in a form that could be mistaken for a Finding, an empty result, or a completed Assessment.
- [ ] No Assessing-state view, live indicator, or progress signal exists anywhere in the application.
- [ ] No mechanism exists for submitting material against an already-completed Assessment.
- [ ] Session gating reflects only the Role/Membership concepts `IdentityAndAccess.md` already defines, without redefining any of them.

---

## 7. Engineering Risks

- **Contract instability (GOV-008 Open).** The current `AssessmentService` contract is usable but not guaranteed final; implementation should expect the concrete payload shape to require revision once GOV-008 and Initiatives 3–4 resolve.
- **No prior real Assessment Input acquisition exists anywhere in this codebase.** `apps/sandbox` constructs only a hardcoded, synthetic Evidence item; real acquisition against the live engine is genuinely new engineering, not a proven pattern being repeated.
- **No real identity backend exists to integrate against.** `IdentityAndAccess.md` defines the concepts a session must respect but no persistence or credential mechanism exists anywhere in the repository; building "just enough" without drifting into full Platform Architecture implementation (explicitly out of scope, §4) requires active discipline.
- **Scope drift toward reserved responsibilities.** Assessing-state observation and reentrancy are the two most product-obviously-desirable capabilities this Sprint excludes; §4 is the standing guard against incorporating either "while already in the code."

---

## 8. Dependencies

- The current public `AssessmentService` contract — real, existing, available today.
- `EvidenceCollection.md`'s Assessment Input boundary — already defined, assigns acquisition to the application layer.
- `IdentityAndAccess.md`'s Role/Membership concepts — already defined; no mechanism, so nothing to depend on beyond the concepts themselves.
- No dependency on Initiative 1, Initiative 2, or GOV-008's resolution — by the Authorization's own conditioning, none may be introduced.

---

## 9. Proposed Work Breakdown

Work is organized into three sequential phases:

### Phase 1 — Foundation
Application Shell composition (Console containing Dashboard and Workspace); a real identity/session mechanism gating entry. Produces a running, session-gated, empty shell with no Assessment capability yet. **Gate:** the structural separation required by Boundary Enforcement is present and auditable before any Assessment-facing work begins.

### Phase 2 — Intake and Submission
Real Assessment Input acquisition for one Assessment Subject type; the Request/Response Mechanism's submit-and-await path; content-level error handling (`ASSESSMENT_INTAKE_AND_UPLOAD.md` §6); whatever attention/absence discipline is exercisable without a live Assessing view (see Note — Traceability Observation, below); Engine Transport Failure Handling for this call. Produces the ability to submit a real Assessment Subject and receive either a completed Report or a well-handled failure. **Gate:** boundary separation still holds; no code introduced in this phase evaluates Evidence or interprets engine output as anything other than pass-through data.

### Phase 3 — Reviewing and Navigation
Presentation of the received, completed Assessment Report; Navigation Realization across Overview, Finding, Recommendation, Evidence; presentation, interaction, and navigation state fully exercised. Produces the complete Intake → Reviewing experience this Sprint exists to deliver. **Gate:** every acceptance criterion in §6 verified, not assumed.

Phase 1 precedes Phase 2; Phase 2 precedes Phase 3. No phase depends on a capability a later phase produces.

---

## 10. Completion Criteria

- A real user, via the production application, can submit one real Assessment Subject and review its completed Assessment Report following the frozen Interaction Design navigation model — not demonstrated in `apps/sandbox`, not simulated.
- Every acceptance criterion in §6 verified.
- No item listed in §4 is present in the delivered application, in any form.
- Documentation reconciliation completed, including updates to `PROJECT_STATUS.md`, `CHANGELOG.md`, and any Sprint-specific closeout documentation required by the repository's engineering process.
- No Governance Register item and no ADR required unless implementation surfaces a genuine, previously-unidentified conflict with a fixed decision.

---

## 11. Explicit Non-Actions

Initiative 1, Initiative 2, GOV-008, Frontend Architecture, the Frontend Architecture Architectural Resolution, and the Documentation Authority Decision are not reopened. No implementation technology, framework, routing approach, state-management approach, UI library, or folder structure is chosen. No implementation code is produced. No responsibility outside Implementation Authorization §3 and §6 is incorporated. No repository file is modified.

---

## Status

Sprint Planning only. No application, technology, folder structure, or code has been produced in preparing this plan. Implementation does not begin on the basis of this document alone — approval remains a separate, explicit act.

---

## Note — Basis for Phase Sequencing (traceability only, not part of the operative plan)

Boundary Enforcement is validated at each phase gate rather than assigned its own phase because it is a standing constraint, not a discrete deliverable. Error and Latency Handling's two behavioral sources are split across Phases 1–2 because each governs a different point in the user's lifecycle: `ASSESSMENT_INTAKE_AND_UPLOAD.md` §6 applies before submission; `ASSESSING_AND_PROGRESSIVE_DISCOVERY.md` §7 applies during the wait.

## Note — Traceability Observation

`FrontendArchitecture.md`'s Error and Latency Handling responsibility references `ASSESSING_AND_PROGRESSIVE_DISCOVERY.md` §7 in full. Because Sprint 21 intentionally does not implement an observable Assessing state (reserved pending Initiative 1), only the subset of that behavior exercisable during a synchronous wait is implemented within this Sprint. This is a consequence of the approved Implementation Authorization rather than a conflict between repository documents.

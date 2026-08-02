# modIQ Console

The production interaction layer `FrontendArchitecture.md` authorizes — the consumer application through which a user experiences an Assessment.

This is not `apps/sandbox`. `apps/sandbox` remains an independent developer validation harness, exactly as Initiative 5 specifies; nothing here evolves from it or depends on it.

## Status

Sprint 21 complete (`docs/engineering/SPRINT21_PLAN.md`): Application Shell and Region Composition, Identity and Session Mechanism, the Request/Response Mechanism, and the full Intake → Reviewing experience with Navigation Realization. A user can sign in, submit a real mod folder, and review its resulting Findings, Evidence, and Recommendations as one continuous, navigable object.

Not implemented, by design: any Assessing-state view or progress indicator (Initiative 1, reserved), any path back to Intake from Reviewing or supplementation of a completed Assessment (Initiative 2, reserved), more than one Assessment Subject type, or Dashboard presentation beyond a single session-derived Organization and Project.

## Boundary Enforcement

`src-tauri/src/assessment.rs` is the only module in this crate permitted to depend on a `modiq-*` crate; `src/engine/index.ts` is the only file permitted to call Tauri's `invoke`. Everywhere else, in both languages, the dependency graph contains nothing capable of evaluating Evidence or producing a Finding. The Request/Response Mechanism's transport (`ReportSummary`) is provisional, not final or stable — `FRONTEND_IMPLEMENTATION_AUTHORIZATION.md` §4 holds the concrete payload shape open pending GOV-008.

## Platform Architecture Representation

Session sign-in constructs a real (not fabricated) Organization, Membership, and Project, scoped exactly to what `FRONTEND_IMPLEMENTATION_AUTHORIZATION.md` §6 authorizes — no more. See `src/session/types.ts` and `src/session/SessionContext.tsx`. Nothing persists beyond the running session; no credential is verified against anything, since no Platform Architecture identity backend exists anywhere in this repository to verify against.

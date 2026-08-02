# modIQ Console

The production interaction layer `FrontendArchitecture.md` authorizes — the consumer application through which a user experiences an Assessment.

This is not `apps/sandbox`. `apps/sandbox` remains an independent developer validation harness, exactly as Initiative 5 specifies; nothing here evolves from it or depends on it.

## Status

Sprint 21, Phase 2 (`docs/engineering/SPRINT21_PLAN.md`): real Assessment Input acquisition (one Assessment Subject type — a mod folder), the Request/Response Mechanism, and Engine Transport Failure Handling. No Reviewing UI, no Navigation Realization, no presentation of Evidence, Finding, or Recommendation content — that is Phase 3.

## Boundary Enforcement

`src-tauri/src/assessment.rs` is the only module in this crate permitted to depend on a `modiq-*` crate; `src/engine/index.ts` is the only file permitted to call Tauri's `invoke`. Everywhere else, in both languages, the dependency graph contains nothing capable of evaluating Evidence or producing a Finding. The Request/Response Mechanism reports only whether a submission succeeded — no Evidence, Finding, or Recommendation content crosses the boundary in either direction, since `FRONTEND_IMPLEMENTATION_AUTHORIZATION.md` §4 holds the concrete payload shape open pending GOV-008, and a bare success/failure signal commits to nothing that decision has not yet settled.

## Platform Architecture Representation

Session sign-in constructs a real (not fabricated) Organization, Membership, and Project, scoped exactly to what `FRONTEND_IMPLEMENTATION_AUTHORIZATION.md` §6 authorizes — no more. See `src/session/types.ts` and `src/session/SessionContext.tsx`. Nothing persists beyond the running session; no credential is verified against anything, since no Platform Architecture identity backend exists anywhere in this repository to verify against.

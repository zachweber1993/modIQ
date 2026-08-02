# modIQ Console

The production interaction layer `FrontendArchitecture.md` authorizes — the consumer application through which a user experiences an Assessment.

This is not `apps/sandbox`. `apps/sandbox` remains an independent developer validation harness, exactly as Initiative 5 specifies; nothing here evolves from it or depends on it.

## Status

Sprint 21, Phase 1 (`docs/engineering/SPRINT21_PLAN.md`): Application Shell, Console/Dashboard/Workspace region composition, and the minimum Identity/Session mechanism required to gate the shell. No Assessment submission, no `AssessmentService` integration, no Reviewing UI — that is Phase 2 and Phase 3.

## Boundary Enforcement

`src-tauri/Cargo.toml` depends on no `modiq-*` crate. This is deliberate, not an oversight: `FrontendArchitecture.md`'s Boundary Enforcement requires a structural separation between presentation code and anything capable of reaching into assessment logic, and the strongest form of that separation available at this phase is a compile-time one — there is nothing in this crate's dependency graph capable of evaluating Evidence or producing a Finding. Phase 2 introduces the Request/Response Mechanism and, with it, the first real dependency on the engine.

## Platform Architecture Representation

Session sign-in constructs a real (not fabricated) Organization, Membership, and Project, scoped exactly to what `FRONTEND_IMPLEMENTATION_AUTHORIZATION.md` §6 authorizes — no more. See `src/session/types.ts` and `src/session/SessionContext.tsx`. Nothing persists beyond the running session; no credential is verified against anything, since no Platform Architecture identity backend exists anywhere in this repository to verify against.

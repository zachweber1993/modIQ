# Storage Implementation Authorization

| Property | Value |
|---|---|
| **Document** | STORAGE_IMPLEMENTATION_AUTHORIZATION.md |
| **Project** | modIQ |
| **Purpose** | Translate the four Accepted Architectural Resolution decisions (`STORAGE_ARCHITECTURE_EVALUATION.md` §10) into an engineering envelope — constraints and acceptance criteria implementation must satisfy — without specifying how implementation proceeds. |
| **Origin** | Chief Architect authorization, following the committed Storage Architectural Resolution (commit `5bea10d`). |
| **Status** | **Implementation Authorization draft only. Not yet approved. No Sprint has been scoped, no crate created, no API designed, no Cargo workspace membership changed, no code written. This document authorizes an envelope for future engineering; it does not itself authorize Sprint Planning or Implementation to begin.** |

---

**A note on scope, relative to prior precedent:** `SPRINT8_IMPLEMENTATION_AUTHORIZATION.md` combined implementation authorization with engineering sequencing — a phased roadmap, crate implementation order, and per-phase testing strategy, in one document. This document deliberately does not. It limits itself to the engineering envelope the accepted architecture authorizes — constraints and acceptance criteria only. Sprint Planning, where sequencing and phasing belong, remains its own distinct, subsequent stage. This reflects how the repository's engineering workflow has evolved to separate its own stages more strictly, not any difference in what Storage's architecture itself requires.

---

## 1. Minimum Viable Implementation

The smallest complete realization of this Architectural Activation: an already-generated `AssessmentReport`, produced by an existing consumer in one process invocation, can be made durable and retrieved — by its own Assessment identifier, in a separate, later process invocation — after the process that generated it has exited. Nothing beyond this constitutes the minimum; nothing less would constitute it at all, since an unretrievable write is not meaningfully different from no persistence.

---

## 2. Explicitly Authorized

- The existence of a new workspace crate, `modiq-storage`, as the sole owner of this responsibility (Decision 2).
- A write path accepting one already-generated `AssessmentReport` and the identifier of the Assessment it describes.
- A read path returning a previously-written `AssessmentReport`, given only that identifier.
- Whatever internal mechanism is necessary to make that write and read durable across process boundaries — the *existence* of such a mechanism, not its technology, format, or design (Section 3).

---

## 3. Explicitly Out of Scope

- **Storage technology selection** — a separate decision, for a later stage, not made or implied here.
- **API design** — no method, trait, or function signature is authorized by naming it here; Section 2 authorizes capabilities to exist, not their shape.
- **Sprint scope or task breakdown** — sequencing, phasing, and crate-implementation order belong to Sprint Planning, a distinct, later, separately-authorized step.
- **Cargo workspace membership changes** — this document authorizes that a crate will eventually exist; it does not add one.
- **Any general, pluggable, or backend-abstracted persistence framework.**
- **Schema versioning or migration mechanisms.**
- **Querying, filtering, comparison, ranking, or aggregation across multiple Assessments.**
- **Any feed into `modiq-knowledge` / the Knowledge Base.**
- **Persistence of `Assessment`, `Evidence`, `Finding`, or `Recommendation` as individually addressable entities** — only the already-bundled `AssessmentReport` snapshot (Decision 1).
- **Any resolution of GOV-001** (Assessment Report Generation Timing, still Open).
- **Any change to `AssessmentService`'s two public entry points** (Decision 4).

---

## 4. Evidence That Would Demonstrate Successful Activation

- A `modiq-storage` crate exists in the workspace, builds, and carries real (non-scaffold) content — the same L1→L2 crate-maturity transition `modiq-versioning` and `modiq-knowledge` each underwent at their own activation.
- A genuine cross-process round trip is demonstrated: an `AssessmentReport` written by one process invocation is retrieved, unchanged, by a separate, later process invocation — not merely an in-memory or same-process test.
- `AssessmentService::execute` and `execute_from_assessment_input` are shown, by a regression test, to be unaffected in signature or behavior.
- Full-workspace validation (`cargo fmt`/`check`/`test`, both workspaces) remains green throughout, per this project's continuous-validation discipline.

---

## 5. Acceptance Criteria

- [ ] `modiq-storage` exists as a workspace crate with real content, not a scaffold.
- [ ] A write capability exists, accepting an `AssessmentReport` and an Assessment identifier.
- [ ] A read capability exists, returning a previously-written `AssessmentReport` given only that identifier.
- [ ] Cross-process persistence is proven by a real test, not asserted by same-process behavior alone.
- [ ] Neither of `AssessmentService`'s two public entry points changed signature.
- [ ] No Runtime or Report-adjacent type other than `AssessmentReport` is persisted.
- [ ] No cross-Assessment operation (listing, filtering, comparison, aggregation) exists anywhere in the implementation.
- [ ] Both workspaces remain green, zero warnings, throughout.
- [ ] Real-I/O testing discipline is followed — no mocked I/O, per this project's own standing practice since Sprint 3 Phase 5.

---

## 6. Repository Constraints Implementation Must Preserve

- **The platform's one-directional information flow** (Evidence → Rule Evaluation → Findings → Recommendations → Report) — Storage sits downstream of Reporting and must never feed back upstream into any of it (Decision 3).
- **ADR-0007's Opaque Runtime References pattern** — if any relationship between a stored Report and a Runtime/Report type is ever needed beyond the write/read path itself, it follows this platform's existing pattern rather than a new one invented for this purpose.
- **`GOVERNANCE.md`'s Crate Boundary Rules convention** — `modiq-storage` will need its own "Owns / Must never" pair recorded, mirroring every existing crate, as part of whatever stage actually implements it.
- **GOV-008's stability discipline** — `AssessmentService`'s two public entry points have held stable for nine Sprints; this activation does not become the reason that streak ends (Decision 4).
- **Real-I/O Testing Discipline** (established Sprint 3 Phase 5) — no mocking of real I/O; real, checked-in or test-constructed fixtures throughout.
- **Zero-warnings, both-workspaces validation** — `cargo fmt`/`check`/`test` clean across the root workspace and `apps/sandbox/src-tauri`'s own separate workspace, continuously, not only at closeout.

---

## 7. Architectural Decisions Now Fixed — Not to Be Revisited During Implementation

Per `STORAGE_ARCHITECTURE_EVALUATION.md` §10, the following are Architectural Resolution outputs, not implementation-time choices:

1. **Persisted domain object is `AssessmentReport`** — not `Assessment` itself, not individually addressable Evidence/Finding/Recommendation records.
2. **Subsystem ownership is a new `modiq-storage` crate** — not folded into `modiq-report` or `modiq-engine`.
3. **Lifecycle boundary is strictly downstream of Reporting** — no participation in Evidence Collection, Rule Evaluation, or Report generation; no presupposition of GOV-001's eventual answer.
4. **`AssessmentService`'s two public entry points are unchanged.**

Revisiting any of these during implementation is not an implementation decision — it would require a new Architecture Evaluation, per this project's own standing discipline that Architectural Resolution outputs do not carry forward silently altered.

---

## Status

This document defines the engineering envelope only. It does not authorize Sprint Planning or Implementation to begin — that remains a separate, explicit Chief Architect act, per this project's own Permanent Engineering Workflow (`PROJECT_HANDOFF_v1.1.md` §5). No crate, API, Cargo workspace change, or code has been produced in preparing it.

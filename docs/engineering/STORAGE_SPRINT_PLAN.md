# Storage Sprint Plan — Architectural Activation

| Property | Value |
|---|---|
| **Document** | STORAGE_SPRINT_PLAN.md |
| **Project** | modIQ |
| **Purpose** | Translate the approved `STORAGE_IMPLEMENTATION_AUTHORIZATION.md` into a concrete engineering execution plan, entirely within the accepted architectural boundaries. Sprint Planning — not implementation. |
| **Origin** | Chief Architect authorization, following the committed Storage Investigation (`INV-002`), Architecture Evaluation, Architectural Resolution (commit `5bea10d`), and Implementation Authorization (commit `7640b77`). |
| **Status** | **Sprint Planning draft only. Not yet approved. No Rust source, crate, or Cargo workspace change has been made. Awaiting Chief Architect review before Implementation may begin.** |

**A note on Sprint numbering:** whether this work is formally designated "Sprint 13" in the repository's own sequence is an administrative/roadmap confirmation `PROJECT_HANDOFF_v1.1.md` §10 explicitly reserves for the Chief Architect — it is not asserted or decided here. This plan is written to be adopted under whatever Sprint designation is assigned.

**Fixed, per Implementation Authorization §7 — not revisited by this plan:** `AssessmentReport` is the persisted domain object; `modiq-storage` owns persistence; Storage remains strictly downstream of Reporting; `AssessmentService`'s two public entry points are unchanged. **No architectural conflict with any of these four was found while producing this plan** — each proved directly satisfiable, as shown below.

---

## 1. Sprint Objective

Activate the Storage subsystem's first real content — durable, cross-process persistence and retrieval of a single `AssessmentReport` — closing the gap between "modIQ produces an assessment" and "modIQ preserves one." One sentence, per this project's own standing discipline that every Sprint should increase product capability statably in one.

---

## 2. Definition of Done

An `AssessmentReport` produced by an existing consumer (`modiq-cli` or `apps/sandbox`) in one process invocation can be retrieved, unchanged, by its Assessment identifier, from a **separate, later process invocation** of the same consumer — with both workspaces green, zero warnings, and none of the four fixed decisions altered.

---

## 3. Ordered Implementation Phases

### Phase 1 — `modiq-storage`: crate creation and minimum domain content

Create the crate; give it its first real content — a write operation accepting an `AssessmentReport` and the identifier of the Assessment it describes, and a read operation returning a previously-written `AssessmentReport` given only that identifier. Takes a dependency on `modiq-report` only, for the concrete `AssessmentReport` type — consistent with existing precedent, not a new pattern: `modiq-cli` and `apps/sandbox` already both depend on `modiq-report` directly today, since `modiq-engine` does not re-export it. No dependency on `modiq-runtime`, `modiq-engine`, `modiq-rules`, `modiq-versioning`, or `modiq-knowledge`. Independently testable — write then read, same process — before any caller integration begins.

### Phase 2 — `modiq-cli` wiring

Add the `modiq-storage` dependency. After `AssessmentReport::generate` (already called today), hand the Report to `modiq-storage`'s write path. Add a retrieval path invokable independently of running a new Assessment — the CLI-side half of the cross-process round trip this Sprint exists to prove.

### Phase 3 — `apps/sandbox` wiring

The same shape as Phase 2, added at the Tauri app's own existing Report-consumption point. Validated in the Sandbox's own separate workspace, independently, per this project's standing two-workspace discipline.

### Phase 4 — Closeout

Full-workspace and Sandbox-workspace validation. Documentation reconciliation (`PROJECT_STATUS.md`, `CHANGELOG.md`, `ENGINEERING_LOG.md`, `CrateRoadmap.md` — `modiq-storage`'s own maturity entry). `GOVERNANCE.md` gains `modiq-storage`'s own Crate Boundary Rule ("Owns / Must never") pair, mirroring every existing crate.

---

## 4. Crate Activation Sequence

1. **`modiq-storage`** (new; depends on `modiq-report` only) — created and given real content first; fully testable in isolation.
2. **`modiq-cli`** (existing; gains a `modiq-storage` dependency) — proceeds once Phase 1 is complete.
3. **`apps/sandbox`** (existing; gains a `modiq-storage` dependency) — can proceed in parallel with `modiq-cli`'s own wiring; neither depends on the other.
4. **Closeout** — requires all preceding phases green in both workspaces.

**`modiq-runtime`, `modiq-engine`, `modiq-rules`, `modiq-report`, `modiq-versioning`, `modiq-knowledge`, and `modiq-common` require zero changes.** No boundary in the existing assessment pipeline is touched — direct confirmation that Decision 3 (strictly downstream) and Decision 4 (`AssessmentService` unchanged) hold throughout, not just in principle.

---

## 5. Expected Repository Changes

- **New:** `crates/modiq-storage/` (crate, added to workspace `members`).
- **Modified:** `modiq-cli`'s `Cargo.toml` (new dependency) and its own command-handling source (write-after-generate call, new retrieval path).
- **Modified:** `apps/sandbox/src-tauri`'s `Cargo.toml` (new dependency) and its own Report-consumption call site.
- **Unmodified:** `modiq-runtime`, `modiq-engine`, `modiq-rules`, `modiq-report`, `modiq-versioning`, `modiq-knowledge`, `modiq-common` — zero source changes across all seven.
- **Documentation:** `GOVERNANCE.md`, `CrateRoadmap.md`, `PROJECT_STATUS.md`, `CHANGELOG.md`, `ENGINEERING_LOG.md`, and `docs/README.md` if it enumerates crates — standard closeout reconciliation, per this project's own unbroken five-Sprint discipline since Sprint 8.

---

## 6. Testing Strategy — Real-I/O Testing Discipline

No mocked I/O anywhere; real fixtures and a genuine round trip, per the standing discipline established at Sprint 3 Phase 5 and reused at every I/O-touching phase since.

- **Phase 1:** `modiq-storage`'s first tests ever — write a real `AssessmentReport`, read it back within the same process, assert full equality; a not-found case for an unrecognized identifier. Whatever durability mechanism is chosen is exercised for real, never mocked.
- **Phase 2/3 — the load-bearing test for this Sprint:** a genuine **two-process** test — invoke the consumer once (producing and storing a Report), then invoke it again as a **separate process** to retrieve it, asserting the retrieved content matches. A same-process, in-memory-only test does not satisfy Decision 3's "durable beyond the lifetime of the process" requirement and must not be treated as sufficient.
- **Explicit regression test:** `AssessmentService::execute` and `execute_from_assessment_input` produce identical output before and after this Sprint — guarding Decision 4 directly, named as its own test case, not left to incidental coverage.
- **Phase 4:** full-workspace and Sandbox-workspace validation, zero warnings, as the closing gate.

---

## 7. Validation Checkpoints

- After Phase 1: `modiq-storage`'s own tests green, in isolation, before any caller integration begins.
- After Phase 2: `modiq-cli`'s full suite green, including the new two-process round-trip test.
- After Phase 3: `apps/sandbox`'s own separate workspace green, including its own round-trip test.
- After Phase 4: `cargo fmt`/`check`/`test` clean across both workspaces, zero warnings.
- Each checkpoint independently validated before the next phase begins — this project's own standing practice, not a new requirement for this Sprint.

---

## 8. Risks and Rollback Considerations

- **First crate depending on `modiq-report` other than the two existing application-layer consumers.** Low risk — direct precedent already exists; `modiq-storage` is a third consumer of an already-stable public type, not a new pattern.
- **The two-process test is a genuinely new testing shape for this codebase** — every existing test runs in-process. This needs explicit design attention during Phase 1/2; it should not be assumed to fall out of existing test infrastructure for free.
- **Scope creep is the primary risk to this Sprint's own minimality** — e.g., adding a list or query operation "while already in the crate." Mitigation: `STORAGE_IMPLEMENTATION_AUTHORIZATION.md` §3 is the standing guard; any such addition is an Architecture Evaluation question, not an implementation-time one.
- **Rollback:** every phase is additive — one new crate, two new dependency edges, zero modification to any existing crate's own logic. Rolling back is removing the new crate and its two dependency edges; no existing behavior needs restoring, since none was changed.
- **No architectural conflict was identified during this planning pass.** All four fixed decisions remain satisfiable exactly as approved.

---

## 9. Completion Criteria for Closing the Architectural Activation

- Every acceptance criterion in `STORAGE_IMPLEMENTATION_AUTHORIZATION.md` §5 verified, not assumed.
- `modiq-storage` advances from not-existing to a real, tested crate — its own maturity progression recorded in `CrateRoadmap.md`, mirroring `modiq-versioning`'s and `modiq-knowledge`'s own Sprint 8/9 entries.
- `GOVERNANCE.md` carries `modiq-storage`'s own Crate Boundary Rule pair.
- Full documentation reconciliation performed, per every Sprint since Sprint 8's own unbroken closeout discipline.
- An Engineering Release record produced at or near closeout.
- No Governance Register item and no ADR required, unless implementation surfaces a genuine conflict with one of the four fixed decisions — consistent with Sprint 8's own precedent of not creating governance machinery ahead of real forcing evidence.

---

## Status

Sprint Planning only. No Rust source, crate, Cargo workspace change, or API has been produced in preparing this plan. Implementation does not begin on the basis of this document alone — that remains a separate, explicit Chief Architect act.

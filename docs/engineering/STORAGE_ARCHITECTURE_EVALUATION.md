# Storage Architecture Evaluation

| Property | Value |
|---|---|
| **Document** | STORAGE_ARCHITECTURE_EVALUATION.md |
| **Project** | modIQ |
| **Purpose** | Architecture Evaluation for Storage's minimum-viable Architectural Activation — determining the smallest coherent architectural surface that makes Storage real, following the Sprint 8 (Version Profiles) / Sprint 9 (Knowledge Base) precedent, per `PROJECT_HANDOFF_v1.1.md` §5's now-clarified scope. |
| **Origin** | Chief Architect authorization, following `INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md`'s finding that Storage is a valid, well-evidenced capability candidate, and the subsequent governance reconciliation (`GOVERNANCE_OBSERVATION_SUBSYSTEM_ACTIVATION.md`; `PROJECT_HANDOFF_v1.1.md` §5 amendment, commit `fd2db36`) confirming subsystem-level activation proceeds via Architectural Activation, not the Capability Identity gate. |
| **Status** | **Architecture Evaluation complete; Architectural Resolution complete — all four decisions Accepted by the Chief Architect (Section 10). Implementation Authorization has NOT been granted (Section 11). No ADR, Governance Register item, crate, API, or storage technology has been created, designed, or chosen.** |

---

## 1. Evaluation Scope

Per this project's own Decision Framework (`CHIEF_ARCHITECT_HANDOFF_v1.1.md` §6), Architecture Evaluation is where "concrete design alternatives are evaluated against repository evidence and established principles, and one is recommended — not yet approved." This document does exactly that, and only that, for six questions the Chief Architect specified: the smallest persistable domain object, subsystem ownership, lifecycle boundaries, architectural responsibilities, explicit non-goals, and the rationale for the proposed activation.

**Deliberately excluded**, per explicit instruction and consistent with how Sprint 8's own Architectural Resolution deferred implementation specifics past its own stage:

- No implementation plan, no phasing, no Sprint scope.
- No API — no method signature, trait, or function name is proposed anywhere below.
- No storage technology — no database, file format, or serialization mechanism is named or implied as a choice.
- No Sprint Planning.
- No resolution of GOV-001 (Assessment Report Generation Timing, still Open) — this evaluation is designed to remain compatible with either answer, not to presuppose one.

---

## 2. Decision 1 — The Smallest Persistable Domain Object

**Question:** of everything a completed Assessment produces, what is the smallest single thing worth making durable first?

**Alternatives considered:**

- **`Assessment` itself** (the full aggregate — Evidence, Findings, Recommendations, lifecycle state). Rejected as the starting slice: `Assessment` is lifecycle-governed (`INV-010`–`INV-012`), and persisting it raises questions this evaluation would otherwise have to answer to stay coherent — does a reloaded `Assessment` re-enter its own lifecycle machinery, can a persisted `Assessment` resume mid-lifecycle, is a deserialized instance permitted to call its own mutating methods. Each is a real design question with no existing precedent to draw on, and answering any of them here would be designing behavior, not activating a dormant subsystem.
- **Individual `Evidence` / `Finding` / `Recommendation` records**, persisted and queryable independently. Rejected: this presupposes a querying and cross-referencing capability (which Evidence supports which Finding, across which Assessments) that does not exist today even in memory, and would require resolving the GOV-005/GOV-006 referential-integrity follow-up (`PROJECT_HANDOFF_v1.1.md` §9) as a precondition — a second open governance question this evaluation should not need to touch to succeed.
- **`AssessmentReport`** (`modiq-report`'s existing, already-generated snapshot). **Recommended.** `AssessmentReport::generate` "produces a pure, read-only snapshot of an `Assessment` (evidence, findings, recommendations, status, id) — no analysis, no formatting beyond what Runtime already carries" (`PROJECT_HANDOFF_v1.1.md` §3). It is already inert: no lifecycle, no mutation methods, no further transitions once generated. It is also already the exact artifact both real consumers (`modiq-cli`, `apps/sandbox`) already produce and immediately discard today (`INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md` §1.1). Persisting it durably requires deciding nothing about lifecycle re-entry, mutation, or cross-entity querying — it is the one candidate that is already, by design, a closed, self-contained value.

**Recommendation:** `AssessmentReport` is the smallest persistable domain object. This mirrors the platform's own repeated "smallest real slice" pattern exactly — `VersionProfile::fs25()` (one hardcoded value), `RepairRecipe::version_compatibility_declared_version_mismatch()` (one authored recipe) — applied here to the smallest already-existing, already-inert Runtime-adjacent type.

---

## 3. Decision 2 — Subsystem Ownership

**Question:** what crate owns this responsibility?

**Alternatives considered:**

- **Fold into `modiq-report`.** Rejected: `Architecture.md`'s own "Platform Boundaries" section lists Reporting and Persistence as two of six things the architecture "intentionally separates." `modiq-report`'s own stated responsibility is presentation, not durability; folding storage in would cross a boundary the architecture already draws explicitly, not one this evaluation would be drawing for the first time.
- **Fold into `modiq-engine`.** Rejected: `AssessmentService` orchestrates subsystems "by direct composition... no intra-engine service indirection layer" (ADR-0010) — it composes subsystems, it does not contain one. Adding storage logic to the engine crate would blur the one crate this project has protected most consistently (GOV-008, nine Sprints stable) into also being a storage implementation.
- **A new `modiq-storage` crate.** **Recommended.** This mirrors the shape every other named System Overview subsystem already takes — one crate per subsystem, `modiq-versioning` and `modiq-knowledge` foremost among them.

**A real asymmetry from the Sprint 8/9 precedent, disclosed honestly:** `modiq-versioning` and `modiq-knowledge` both existed as empty, scaffolded crates in the workspace *before* their own Sprint 8/9 activation gave them real content. **No `modiq-storage` crate exists today, in any form** — confirmed directly against `Cargo.toml`'s workspace member list and `crates/`. Storage's own activation would therefore include the crate's own creation, one step earlier than Sprint 8 or Sprint 9 needed to take. This is not unprecedented in the repository generally — `modiq-collection` itself was created from nothing at Sprint 3 ("Added `modiq-collection` as a planned crate (not yet created)" at Phase 3, created at Phase 4, per `CrateRoadmap.md`) — but it is a genuine difference from the specific Sprint 8/9 comparison this evaluation is otherwise following closely, and is worth the Chief Architect weighing explicitly rather than this evaluation quietly treating the two cases as identical.

---

## 4. Decision 3 — Lifecycle Boundary

**Question:** where, in the existing one-directional pipeline (Evidence → Rule Evaluation → Findings → Recommendations → Report), does Storage sit?

**Recommendation:** strictly downstream of Reporting, and nowhere else. Storage receives an already-generated `AssessmentReport` after `AssessmentReport::generate` has run; it is never consulted during Evidence Collection, Rule Evaluation, or Report generation itself. This is not a new principle — it is the direct, unmodified application of the platform's own standing one-directional information flow ("nothing downstream feeds back upstream," `CHIEF_ARCHITECT_HANDOFF_v1.1.md` §3) to Storage's own placement in the System Overview diagram, where it already sits last.

**Interaction with GOV-001, not a resolution of it:** GOV-001 asks whether reports should represent the Assessment immediately before or after completion — still Open. Storage's own lifecycle boundary ("accepts whatever `AssessmentReport::generate` already produces, whenever it is called") is deliberately agnostic to that question's eventual answer. Whichever way GOV-001 resolves, Storage's own boundary does not need to change, because it never inspects the timing decision — it only ever receives the Report already produced.

**Retrieval, kept to the minimum that makes this a real round trip:** a stored Report must be retrievable, or nothing has actually been made durable — an unretrievable write is not meaningfully different from no persistence at all. The minimum retrieval shape this evaluation recommends: lookup by the Report's own Assessment identifier, and nothing further. No filtering, no cross-Assessment comparison, no listing, no querying by mod name or date. Each of those is a real, later capability — not part of the smallest coherent slice.

---

## 5. Decision 4 — Impact on `AssessmentService`'s Public Entry Points

**Question:** does activating Storage require changing `AssessmentService::execute` or `execute_from_assessment_input` — the two public entry points GOV-008 has held stable for nine Sprints?

**Recommendation: no.** Precedent already exists for a subsystem being consumed without passing through the engine's own two entry points: `PROJECT_HANDOFF_v1.1.md` §2 notes both `apps/sandbox` and `modiq-cli` "also depend on `modiq-report` directly, since `modiq-engine` does not re-export `AssessmentReport`." Storage, sitting downstream of Reporting in exactly the same position, can be consumed the identical way — a caller receives a `Report` from `modiq-report`, and separately, optionally, hands it to `modiq-storage`. `AssessmentService` itself needs no awareness that Storage exists. This preserves the "zero signature change" outcome Sprint 8 and Sprint 9 each independently achieved, for the third time.

---

## 6. Architectural Responsibilities

Stated in this project's own "Owns / Must never" convention (`GOVERNANCE.md`'s Crate Boundary Rules format):

**Owns:**
- Accepting an already-generated `AssessmentReport` and making it durable beyond the lifetime of the process that produced it.
- Returning a previously-stored `AssessmentReport`, given the identifier it was stored under.

**Must never:**
- Participate in Evidence Collection, Rule Evaluation, or Report generation, or be consulted during any of them.
- Mutate a stored Report once written — write-once, matching the platform's own existing immutability discipline for Evidence and Finding post-phase.
- Make any Reporting-level or Rule-level decision, or apply any judgment to what it stores.
- Require any change to `AssessmentService`'s two public entry points.
- Own or depend on Knowledge Base, Version Profile, or Rule Engine types directly — if a later capability needs Storage to relate to those, it follows ADR-0007's own opaque-reference pattern, the same way every other cross-domain relationship in this platform already does, rather than a direct dependency edge.

---

## 7. Explicit Non-Goals

- No general, pluggable, or backend-abstracted persistence framework.
- No schema versioning or migration mechanism.
- No querying, filtering, comparison, ranking, or aggregation across multiple Assessments — this defers `INV-002`'s own flagged "cross-mod collection validation" product question entirely.
- No feed into `modiq-knowledge` / the Knowledge Base — this defers `INV-002`'s own flagged "MKB accumulation from real Assessments" product question entirely; that remains its own, later, separately-evaluated capability, the same way Repair Guidance (Sprint 9) was its own capability built on top of Version Profiles (Sprint 8) rather than bundled into it.
- No persistence of `Assessment`, `Evidence`, `Finding`, or `Recommendation` as individually addressable entities — only the already-bundled `AssessmentReport` snapshot.
- No resolution of GOV-001.
- No storage technology selection, no API, no crate implementation, no test, no fixture — all explicitly out of this evaluation's scope.

---

## 8. Rationale for the Proposed Activation

**Product grounding**, per `INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md` §2: `Vision.md`'s own "Historical knowledge preservation" (listed as a core platform capability, not an aspiration), `Principles.md`'s "Knowledge Preservation" ("Validated knowledge should accumulate over time... rather than remaining isolated within individual assessments"), and `ProductSpecification.md`'s own MKB content list ("Historical Assessments") and Assessment Workflow step 7 all describe a real, specified, currently-unreachable capability.

**Architectural grounding:** Storage already occupies a named position in `Architecture.md`'s System Overview, with its own dedicated section and stated responsibilities — this is activation of something already specified, not invention of something new, the same shape `SPRINT8_ARCHITECTURAL_RESOLUTION.md` §8 named Architectural Activation for Version Profiles and Sprint 9 repeated for Knowledge Base.

**Why `AssessmentReport` specifically, restated:** it is the one candidate object requiring no new decision about lifecycle, mutation, or cross-referencing to persist safely — every other candidate (`Assessment` itself, individual Evidence/Finding/Recommendation records) would require this evaluation to resolve a real design question it has no historical precedent to draw on, which is exactly the "infrastructure expansion for its own sake" and "speculative extensibility" this project's own discipline exists to prevent (`CHIEF_ARCHITECT_HANDOFF_v1.1.md` §8).

---

## 9. Cross-Cutting Classification

Following `SPRINT8_ARCHITECTURAL_RESOLUTION.md` §8's own convention, applied here directly:

**Recommended classification: Architectural Activation.** Not Capability implementation (this activates a subsystem, not a Collector/Rule within already-real surface — and per `INV-002` §3, Capability Identity's own axes do not classify it). Not Infrastructure expansion (no generic machinery is proposed for its own sake — Section 7 explicitly excludes a general persistence framework). Not Platform evolution (the surface proposed is deliberately narrow — one object, one write path, one lookup — not broad, open-ended change). This Architecture Evaluation is the first time this classification has been applied *prospectively*, during Architecture Evaluation itself, rather than named only in retrospect at a Sprint's own close (Sprint 8) or reused informally (Sprint 9) — worth noting as a new use of the term, not a change to what it means.

---

## 10. Chief Architect Decision Summary — Architectural Resolution

Per `PROJECT_HANDOFF_v1.1.md` §5, Architectural Resolution "marks every open architectural question Accepted, Rejected, or Deferred; nothing may carry forward silently into implementation." The Chief Architect has reviewed the four decisions above and resolved each:

| Decision | Recommendation | Resolution |
|---|---|---|
| 1 — Persisted domain object | `AssessmentReport` | **Accepted** |
| 2 — Subsystem ownership | New `modiq-storage` crate | **Accepted** |
| 3 — Lifecycle boundary | Strictly downstream of Reporting | **Accepted** |
| 4 — Public entry points | `AssessmentService`'s two entry points unchanged | **Accepted** |

All four decisions carried into this Architectural Resolution exactly as recommended, with no modification. The crate-creation asymmetry disclosed under Decision 2 — no `modiq-storage` crate exists in any form today, unlike `modiq-versioning`/`modiq-knowledge` at their own Sprint 8/9 activation — is accepted as a known, disclosed difference from the precedent, not a blocking concern.

## 11. Implementation Authorization Status

**Architectural Resolution is complete. Implementation Authorization has NOT been granted.** Per this project's own Permanent Engineering Workflow, these are two separate, separately-recorded steps: Architectural Resolution decides *what* is approved; Implementation Authorization "confirms every remaining precondition is satisfied and gives explicit, recorded permission to begin" (`PROJECT_HANDOFF_v1.1.md` §5). No Rust source, crate, test, or fixture should be created on the basis of this Resolution alone. No Sprint has been scoped. No implementation plan exists.

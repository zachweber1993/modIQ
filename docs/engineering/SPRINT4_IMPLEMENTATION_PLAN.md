# Sprint 4 Implementation Plan

| Property | Value |
|----------|-------|
| **Document** | SPRINT4_IMPLEMENTATION_PLAN.md |
| **Project** | modIQ |
| **Sprint** | Sprint 4 — ZIP Evidence Collection |
| **Status** | Approved — Pending Governance Prerequisites |
| **Predecessor Documents** | `PROPOSAL_ZIP_EVIDENCE_COLLECTION.md` (approved), `PLATFORM_VALIDATION_GOV-004.md`, `PLATFORM_VALIDATION_GOV-008.md`, `PLATFORM_VALIDATION_EXECUTION_CONTRACT.md` |
| **Last Updated** | 2026-07-20 |

---

This document is the authoritative implementation contract for Sprint 4. It is not a proposal, a backlog, or a summary of design discussion. Any engineer implementing Sprint 4 should be able to do so from this document, `PROPOSAL_ZIP_EVIDENCE_COLLECTION.md`, and the Engineering Specification alone, without reference to prior design conversation. No implementation has been performed under this document; it records what implementation would do once its Governance Prerequisites are met.

---

# Sprint Objective

`PROPOSAL_ZIP_EVIDENCE_COLLECTION.md` established that ZIP archive traversal is Evidence Collection's correct next capability and that it fits the existing subsystem boundary (ADR-0008) without modification. The Technical Director has since approved that proposal and made one further architectural decision, recorded in full under Technical Director Decisions, below: no collector-dispatch abstraction is authorized; `AssessmentService` will select the concrete collector explicitly, based on `AssessmentInput`.

Sprint 4's objective is to deliver that capability: a second, additive, concrete collector inside `modiq-collection` that deterministically discovers the structural contents of a ZIP archive (entry names, sizes, and kind — file or directory) without reading or interpreting what any entry contains, wired into `AssessmentService` by explicit, simple routing rather than any dispatch mechanism.

This sprint does not attempt content inspection of anything inside an archive, a second archive format, or a general-purpose collector abstraction. It delivers exactly the capability `PROPOSAL_ZIP_EVIDENCE_COLLECTION.md` scoped, using exactly the routing mechanism the Technical Director approved.

---

# Scope

## In Scope

- A second, additive concrete collector in `modiq-collection`, structurally analogous to the existing `EvidenceCollector` (receives a location and Assessment Context, returns Evidence or a categorized error) but internally distinct — referred to in this document as **the Archive Collector**; its exact type name is an implementation detail to be finalized during Phase 2, not fixed by this document.
- Deterministic entry enumeration: name, uncompressed size, and file-vs-directory kind for each archive entry, in an explicitly imposed, stable order.
- Extension of the Collection Error Model for the archive case, per Governance Prerequisite GOV-011, below.
- An archive traversal boundary policy (entry names that would resolve outside the archive's own extraction boundary), per GOV-011.
- Resource-limit handling for archive inspection (entry count, claimed decompression size), per GOV-011.
- One new external dependency in `modiq-collection` — an archive-parsing crate, selected during Phase 2 against criteria recorded there, not named by this document.
- Explicit routing logic added to `AssessmentService::execute_from_assessment_input`: an inline, direct check against the `AssessmentInput` that selects the filesystem `EvidenceCollector` or the Archive Collector and calls it directly. No trait, no registry, no factory, no plugin mechanism — per the Technical Director's decision.
- Sandbox exercise of the new capability, mirroring Sprint 3 Phase 5's precedent: a checked-in archive fixture, exercised through the real pipeline.
- Full test coverage: well-formed, malformed, and adversarial archive fixtures; determinism verification; workspace-wide regression.

## Out of Scope

- Reading or interpreting the content of any entry inside an archive (this is XML inspection's or a future capability's scope, not this one's — the same content/structure line `PROPOSAL_FILESYSTEM_COLLECTION.md` drew for the filesystem case, restated here for the archive case).
- Nested archive traversal (an archive within an archive). Recognizing that a nested archive exists as an entry is in scope; opening it is not.
- Any archive or container format other than ZIP.
- Any collector-dispatch abstraction, trait, registry, factory, or plugin mechanism — explicitly rejected by the Technical Director for this sprint.
- Any change to the existing filesystem `EvidenceCollector` beyond what routing requires (none — it is called exactly as it is today).
- Any change to `EngineAPI.md`.
- CLI wiring (`modiq-cli`).
- Resolution of GOV-008 (remains deferred, per Platform Validation Phase 1).
- Resolution of GOV-001, GOV-002, or GOV-003.
- Persistent storage, Version Profile integration, Knowledge Domain integration.

---

# Technical Director Decisions Incorporated

Recorded here verbatim in substance, so this plan is traceable to the decision that authorized it, matching this document's own "authoritative implementation contract" standard:

1. **No collector-dispatch abstraction.** No dispatcher, registry, provider abstraction, factory, trait hierarchy, or plugin mechanism will be introduced. One existing collector and one newly approved collector is insufficient evidence to justify a routing abstraction.
2. **`AssessmentService` remains the orchestration boundary**, unchanged from GOV-004's resolution.
3. **Explicit routing.** `AssessmentService` will explicitly select the appropriate concrete collector based on `AssessmentInput`. Routing is intentionally simple and direct — an inline check, not an indirection layer.
4. **Rationale on record.** This follows the platform's established principle, capability before abstraction, applied here for a third time (after the Rule trait and Collector trait questions) to the collector-selection question specifically. Future collector abstractions may be reconsidered only after implementation evidence demonstrates a real need — not decided preemptively for a two-collector platform.

---

# Approved Routing & Collector Shape

Full archive-format-specific design detail (entry ordering mechanics, the exact resource-limit values, the exact error variant shape) is resolved through Governance Prerequisite GOV-011, below, and is not duplicated here. This section states the shape this plan is written against.

## Explicit Routing

`AssessmentService::execute_from_assessment_input` gains one additional decision point, inline, before invoking a collector: it inspects the `AssessmentInput` it has just constructed and determines whether the location it names should be treated as a plain filesystem location (routed to the existing `EvidenceCollector`, exactly as today) or as a ZIP archive (routed to the Archive Collector). This is a direct conditional, not a lookup table, not a trait dispatch, and not a configuration-driven mechanism. The exact determination method — a file-extension check, a content-signature check (reading the archive's leading bytes), or some combination — is an implementation decision for Phase 2, not fixed here; both are deterministic and consistent with `EvidenceCollection.md`'s Determinism Expectations, and neither requires an abstraction to implement.

## The Archive Collector

Analogous in shape to the existing `EvidenceCollector`: receives a location reference and relevant Assessment Context, performs deterministic inspection, and returns `Vec<Evidence>` or a categorized error — the same conceptual Collector Contract `EvidenceCollection.md` already defines, applied to a second concrete case. It shares no trait, no common supertype, and no code path with the existing `EvidenceCollector` beyond both being invoked, independently, from the same explicit routing point in `AssessmentService`.

## Evidence Category

Evidence produced by the Archive Collector is expected to reuse the existing `EvidenceCategory::FileStructureAnalysis` variant — an archive entry's existence, name, and kind is the same conceptual category of structural observation `PROPOSAL_FILESYSTEM_COLLECTION.md` already established for the filesystem case, whether the structure being observed sits on disk directly or inside an archive. No new `EvidenceCategory` variant, and no change to `modiq-runtime`, is anticipated by this plan. This expectation is confirmed, not assumed, during Phase 1.

---

# Governance Prerequisites

One item must be resolved through the governance process defined in `GOVERNANCE.md` before its corresponding implementation may be written. Implementation of the routing decision itself (already authorized by the Technical Director, above) is not blocked by this item.

## GOV-011 (candidate) — Archive Collection Model

**Requirement:** resolve, for the archive case, the same class of questions GOV-009 and GOV-010 resolved for the filesystem case:

- Whether a malformed or corrupt archive is categorized as Unsupported Input (the same category the filesystem case uses for "reachable but not a supported kind"), or warrants its own distinct outcome.
- How duplicate entry names within a single archive are categorized, if the chosen archive-parsing approach permits them.
- What resource limits (maximum entry count, maximum claimed decompressed size) bound archive inspection, and whether exceeding them is a new Collection Outcome or an instance of an existing one.
- The archive traversal boundary policy: how an entry name that would resolve outside the archive's own extraction boundary (a path-traversal-shaped name) is handled — rejected, sanitized, or categorized as Unsupported — the archive-format analog of Phase 5's Symbolic Link Policy.

**Why this requires governance approval:** each of these is a Level 3 (Behavioral) change under `GOVERNANCE.md`'s Change Categories — it defines what counts as a valid or invalid input and what failure categories the platform recognizes, the same category of decision GOV-009 and GOV-010 already required approval for.

**Resolution method, per this platform's own established, twice-validated pattern** (`PROPOSAL_PLATFORM_VALIDATION_REVIEW.md`, Section 3): resolved concurrently with a real implementation attempt (Phase 2, below), not decided fully in the abstract beforehand — the same sequencing GOV-009/GOV-010 followed relative to Sprint 3 Phase 5.

**Implementation may not proceed on Phase 3 (Real Collector Implementation) until GOV-011 is marked Resolved and `EvidenceCollection.md` is amended accordingly.** Phase 2 (Boundary-Proving) is not blocked, since it produces no Evidence and enforces no error categorization.

---

# Implementation Phases

## Phase 1 — Governance Preparation

- **Objective** — prepare GOV-011 for resolution: draft the candidate archive-specific Collection Outcomes, resource-limit values, and traversal boundary policy in concrete terms, informed by whatever Phase 2 learns.
- **Affected documents** — none yet; this phase produces the input to a governance decision, not the decision itself.
- **Completion criteria** — a clear, concrete set of candidate answers to every question GOV-011 names, ready for governance review.

## Phase 2 — Boundary-Proving Implementation

- **Objective** — confirm a chosen archive-parsing dependency can deterministically read a well-formed ZIP archive's structure at all, before any real Evidence production or error-model implementation. Mirrors Sprint 3 Phase 4's minimal-slice discipline.
- **Affected crates** — `modiq-collection` (new dependency added to `Cargo.toml`; a minimal read-only proof, not the final Archive Collector).
- **Dependency selection criteria** — actively maintained, widely used within the Rust ecosystem, license-compatible with this platform, and (preferred, not required) free of its own transitive dependency on unsafe or unmaintained code where a suitable alternative exists. The specific crate is chosen during this phase, not by this document.
- **Completion criteria** — a well-formed archive's entries can be read deterministically in an explicitly imposed order; this finding informs Phase 1's candidate answers.

## Phase 3 — Real Collector Implementation

- **Objective** — implement the Archive Collector's real capability: entry enumeration as Evidence, full error handling per GOV-011's resolved text, deterministic ordering, and the resolved traversal boundary policy.
- **Affected crates** — `modiq-collection`.
- **Expected deliverables** — the Archive Collector type; its error handling, shaped per GOV-011 (a new error type, or new variants on the existing `CollectionError` — the exact shape is GOV-011's decision, not this document's); unit tests for every reachable outcome.
- **Completion criteria** — GOV-011 is marked Resolved before this phase begins; every outcome GOV-011 defines has a corresponding, tested code path; no content of any archive entry is read.

## Phase 4 — Explicit Routing in `AssessmentService`

- **Objective** — add the Technical-Director-approved routing decision point to `execute_from_assessment_input`.
- **Affected crates** — `modiq-engine`.
- **Expected deliverables** — one inline conditional selecting between the existing `EvidenceCollector` and the new Archive Collector; no other change to `AssessmentService`'s signatures, behavior, or documented responsibilities.
- **Completion criteria** — both collectors remain independently reachable and independently testable; `execute` (the `Vec<Evidence>`-accepting entry point) is untouched; no trait or common supertype is introduced to unify the two collectors at this call site.

## Phase 5 — Sandbox Exercise

- **Objective** — exercise the new capability through the real pipeline, mirroring Sprint 3 Phase 5's precedent.
- **Affected crates** — `apps/sandbox` (its own, separate workspace).
- **Expected deliverables** — a small, checked-in archive fixture (mirroring the existing `fixtures/sample-assessment-input/` directory's role); confirmation that the Sandbox's existing `create_assessment` command can, unmodified in its own orchestration logic, exercise archive collection once `AssessmentService`'s routing (Phase 4) recognizes the fixture. If the Sandbox's fixed-fixture-path design requires a decision about *which* fixture (directory or archive) it points to, that decision is scoped narrowly to this phase and does not reopen the Sandbox's own no-file-picker constraint.
- **Completion criteria** — the Sandbox's own test suite passes against the updated fixture; no new UI, file picker, or input mechanism is added, consistent with the Sandbox's standing design constraint.

## Phase 6 — Testing & Verification

- **Objective** — bring full workspace and Sandbox test coverage in line with the new capability.
- **Affected crates** — `modiq-collection`, `modiq-engine`, `apps/sandbox` (test code).
- **Completion criteria** — `cargo fmt`, `cargo check --workspace`, `cargo test --workspace` all pass cleanly; the Sandbox's own `cargo fmt`, `cargo check`, `cargo test` (its separate workspace) all pass cleanly; see Testing Strategy, below, for coverage detail.

---

# Expected Crate Changes

| Crate | Change | Notes |
|---|---|---|
| `modiq-collection` | New source file(s) for the Archive Collector; new error handling per GOV-011; `Cargo.toml` gains one new external dependency | First external, non-workspace dependency any domain crate in this platform has taken |
| `modiq-engine` | `assessment_service.rs` gains one inline routing conditional inside `execute_from_assessment_input` | No signature change to `execute` or `execute_from_assessment_input`; no new public type |
| `modiq-runtime` | None anticipated | `EvidenceCategory::FileStructureAnalysis` is expected to be reused, not extended — see Approved Routing & Collector Shape |
| `modiq-rules` | None anticipated | `RuleEngine::evaluate` is Evidence-count-driven, not Evidence-source-aware; no change expected |
| `modiq-report` | None anticipated | `AssessmentReport` reflects whatever Evidence exists; no new field is needed for a new Evidence source |
| `apps/sandbox` | New fixture; existing `create_assessment` command exercises it via Phase 4's routing, without its own orchestration logic changing | Separate workspace; own `Cargo.lock` regenerates mechanically once `modiq-collection`'s new dependency is added |

---

# Testing Strategy

**Unit testing expectations.** The Archive Collector receives the same construction- and outcome-level testing discipline Phase 5 established for the filesystem `EvidenceCollector`: one test per reachable outcome (successful enumeration, each GOV-011-defined error category, Empty Collection for a zero-entry archive).

**Fixture-based testing.** Real, checked-in or test-constructed archive fixtures, not mocked archive-reading behavior — consistent with this platform's real-I/O testing discipline established in Sprint 3 Phase 5. Categories: well-formed (single entry, multiple entries, nested directory structure within the archive, empty archive); malformed (truncated file, corrupted structure, a non-archive file bearing an archive-shaped name or extension); adversarial (path-traversal-shaped entry names, an extreme claimed decompression ratio, an extreme entry count) — see Security Validation, below.

**Routing testing.** `AssessmentService`'s new conditional is tested directly: a filesystem `AssessmentInput` reaches the existing `EvidenceCollector`; an archive `AssessmentInput` reaches the Archive Collector; both remain independently correct, matching Phase 4's completion criteria.

**Regression testing.** The full workspace suite — `modiq-runtime`, `modiq-collection`, `modiq-rules`, `modiq-report`, `modiq-engine` — and the Sandbox's own separate suite must pass after every phase. No phase is complete if it regresses a previously passing test. The existing filesystem `EvidenceCollector`'s own test suite is expected to require zero changes, since it is not modified by this plan.

---

# Determinism Validation

- **Ordering.** Collection of the same, unchanged archive must produce identical Evidence content, in identical order, across repeated runs — verified by a test structurally identical to Phase 5's `collect_is_deterministic_across_repeated_calls`, applied to the Archive Collector.
- **Stable fact selection.** Only structural facts stable across otherwise-identical archives (entry name, uncompressed size, file-vs-directory kind) are recorded as Evidence. Fields that could vary without the archive's meaningful structure changing (compressed size, compression method, stored modification timestamps) are excluded, mirroring the same principle Phase 5 applied to filesystem timestamps.
- **Order-imposition point.** Whichever explicit ordering is chosen (mirroring Phase 5's lexicographic sort) must be imposed by the Archive Collector itself, not inherited from the archive-parsing dependency's own iteration order, which this plan does not assume is stable — verified directly, not assumed, during Phase 2.
- **Identity vs. content.** As with every other Runtime entity, each Evidence item's identity is freshly assigned per collection run; determinism is judged by content and order, not by incidental identity — the same convention applied without exception elsewhere in this platform.

---

# Security Validation

- **Decompression-ratio ("zip bomb") handling.** Since this plan's scope is structural observation (names, sizes, kind) rather than full content decompression, the actual decompression risk surface is expected to be smaller than a content-reading collector would face — but this expectation is verified, not assumed, during Phase 2, and any resource limit GOV-011 sets is tested directly against a fixture engineered to approach it.
- **Path-traversal ("Zip Slip") handling.** The traversal boundary policy GOV-011 resolves is tested directly against entry names containing `../` sequences, absolute paths, and any other shape the resolved policy names.
- **Malformed-input resilience.** Every malformed-archive fixture (Testing Strategy, above) must resolve to a categorized `Result`, never a panic — the same discipline Phase 5 applied to permission-denied and non-existent-path cases.
- **Dependency provenance.** The archive-parsing dependency chosen in Phase 2 is selected against the criteria stated there; this plan does not authorize adding it silently — its addition is itself one of this plan's tracked deliverables (Expected Crate Changes, above), not an incidental side effect.
- **No content execution.** Nothing in this plan opens, decompresses fully, or executes any entry's content. This boundary is structural (Scope, above) and is itself a security property, not only a scope decision.

---

# Documentation Updates Expected

- `EvidenceCollection.md` — amendment recording GOV-011's resolved Archive Collection Outcomes and Traversal Boundary Policy, mirroring how Phase 5's Symbolic Link Policy and Collection Atomicity were recorded following `PROPOSAL_FILESYSTEM_COLLECTION.md`'s approval.
- `GOVERNANCE.md` — new Governance Register entry (candidate GOV-011), resolved during Phase 1/2.
- `CrateRoadmap.md` — `modiq-collection`'s maturity entry updated; the new external dependency recorded; a new revision history entry.
- `ENGINEERING_LOG.md` — a Sprint 4 entry, following the established Status/Affected Crates/Affected Documents/Notes structure, per phase or as one consolidated entry at Sprint close.
- A new ADR is plausible, recording the explicit-routing decision (Technical Director Decisions, above) as a durable architectural artifact — mirroring ADR-0010's treatment of GOV-004. This document does not draft one; whether one is warranted is for the Technical Director to confirm at Sprint close, consistent with `docs/adrs/README.md`'s own criteria (a decision "establishes a new architectural principle").

---

# Risks

- **First adversarial input surface and first external dependency**, both named already in `PROPOSAL_ZIP_EVIDENCE_COLLECTION.md` and inherited unchanged by this plan — real, not hypothetical, engineering territory this platform has not previously had to cross.
- **GOV-011 resolution timing.** Phase 3 cannot begin until GOV-011 is Resolved; if Phase 2's findings do not cleanly answer GOV-011's questions, this could extend the sprint beyond a single governance-resolution cycle, the same risk profile GOV-009/GOV-010 carried in Sprint 3.
- **Scope-creep risk at the routing point.** The explicit conditional in `AssessmentService` (Phase 4) is deliberately minimal; a natural temptation once two collectors exist is to generalize the conditional prematurely into something abstraction-shaped. This plan and the Technical Director's decision both foreclose that for this sprint.
- **Determinism risk specific to the format.** ZIP's central-directory-vs-physical-archive ordering ambiguity (named in `PROPOSAL_ZIP_EVIDENCE_COLLECTION.md`, Section 3) has no test coverage anywhere in this platform today; Phase 2 is the first point this plan verifies it directly rather than assumes it.
- **Sandbox fixture design risk.** Phase 5's scope note (above) flags that accommodating a second fixture kind could pressure the Sandbox's fixed-fixture-path design in a way not yet fully scoped; this plan constrains that risk narrowly rather than resolving it in advance.

---

# Rollback Considerations

This sprint is additive throughout — no existing type, method signature, or file is modified except `AssessmentService`'s one new inline conditional (Phase 4) and `modiq-collection`'s `Cargo.toml`/`Cargo.lock` (Phase 2). Rollback, at any phase boundary, is a direct revert:

- Reverting Phase 4 alone restores `AssessmentService` to routing every `AssessmentInput` to the existing `EvidenceCollector`, exactly as today — the Archive Collector, if already implemented, simply becomes unreachable dead code, not a breaking change to any existing behavior.
- Reverting Phase 2/3 removes the new dependency and the Archive Collector entirely; no other crate references either, so no downstream cleanup is required beyond the two files/directories added.
- No data migration, persisted state, or backward-compatibility concern applies at any phase — the Storage Layer remains unbuilt, and every artifact this sprint produces is either source code or a checked-in test/Sandbox fixture, both trivially revertible via git.
- `apps/sandbox`'s separate `Cargo.lock` regenerates mechanically on the next build after any revert; no manual reconciliation is anticipated.

---

# Success Criteria

Sprint 4 is complete only when all of the following are true:

- GOV-011 is marked Resolved in the Governance Register, and `EvidenceCollection.md` reflects its approved text.
- The Archive Collector exists, is wired into `AssessmentService` by explicit routing only, and produces deterministic, correctly-categorized Evidence or errors for every case GOV-011 defines.
- No collector-dispatch trait, registry, factory, or plugin mechanism exists anywhere in the workspace.
- The existing filesystem `EvidenceCollector` and `execute` entry point are unchanged in behavior.
- `cargo fmt`, `cargo check --workspace`, and `cargo test --workspace` all pass cleanly across the full workspace; the Sandbox's separate workspace passes its own equivalent three commands.
- Every architectural boundary defined in `GOVERNANCE.md` remains exactly as it was at the close of Platform Validation Phase 1, except for the one explicit, Technical-Director-approved routing addition to `AssessmentService`.

---

# Completion Checklist

☐ Governance prerequisite completed — GOV-011 Resolved in the Governance Register

☐ `EvidenceCollection.md` updated to reflect GOV-011's approved archive-specific outcomes and traversal boundary policy

☐ Archive Collector implemented — deterministic, tested against every GOV-011-defined outcome

☐ No dispatch abstraction introduced — no trait, registry, factory, or plugin mechanism added anywhere

☐ Explicit routing added to `AssessmentService` only — no other signature or behavior change

☐ Existing filesystem `EvidenceCollector` and `execute` entry point unchanged

☐ New external dependency recorded in `CrateRoadmap.md` and evaluated against Phase 2's stated criteria

☐ Sandbox exercises the new capability against a checked-in archive fixture, with no new input UI

☐ Security validation complete — zip-bomb, path-traversal, and malformed-archive cases tested directly

☐ Determinism validation complete — repeated-collection test passes; ordering imposed explicitly, not inherited

☐ Tests passing — `cargo fmt`, `cargo check --workspace`, `cargo test --workspace` clean; Sandbox's own three commands clean

☐ Engineering Log updated — Sprint 4 entry added following the established entry format

☐ New ADR drafted, or explicitly declined, for the explicit-routing decision — Technical Director to confirm

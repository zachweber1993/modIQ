# modIQ Governance

Version: 1.0

---

# Purpose

This document defines the governance process for evolving the modIQ architecture after implementation has begun.

Architecture is considered a managed asset of the project. Once documented and implemented, it may only evolve through an intentional governance process.

Implementation must never silently redefine the architecture.

Engineering discoveries are expected, but architectural changes must always be documented, reviewed, and approved before becoming part of the platform.

---

# Governance Principles

1. Documentation is authoritative.
2. Architecture precedes implementation.
3. Engineering may expose documentation deficiencies.
4. Implementation does not redefine architecture.
5. Every architectural change must be documented.
6. Simplicity takes precedence over speculation.
7. Crate boundaries are protected.
8. Public APIs are treated as contracts.
9. Deterministic behavior is mandatory.
10. Governance decisions are repository artifacts.

---

# Governance Scope

This document governs:

- Architectural evolution
- Runtime domain boundaries
- Crate responsibilities
- Dependency hierarchy
- Public API evolution
- Runtime invariant evolution
- Documentation authority
- Engineering release governance
- Documentation release governance

This document does **not** govern:

- Product roadmap
- Feature prioritization
- Sprint planning
- Coding style
- Formatting conventions
- Development tooling
- Individual implementation tasks

---

# Documentation Authority

When two documents conflict, precedence shall be determined in the following order.

1. Vision
2. Product Specification
3. Architecture
4. Data Model
5. Principles
6. Architecture Decision Records (ADRs)
7. Governance
8. Engineering Release Documents
9. Engineering Log
10. Source Code

Implementation shall never be considered authoritative when it conflicts with higher-level documentation.

Instead, the discrepancy shall be recorded as a Governance Item and resolved through the governance process.

---

# Change Categories

All repository changes fall into one of four categories.

## Level 1 — Editorial

Examples:

- grammar
- spelling
- formatting
- wording improvements with no semantic change

Requires:

- no governance review

---

## Level 2 — Clarification

Examples:

- improved documentation wording
- clearer invariant descriptions
- terminology reconciliation

Requires:

- governance review
- documentation update

Implementation should remain unaffected.

---

## Level 3 — Behavioral

Examples:

- lifecycle modifications
- invariant changes
- API behavior changes
- assessment workflow revisions

Requires:

- governance approval
- documentation update
- implementation update
- engineering review

---

## Level 4 — Architectural

Examples:

- new crates
- aggregate redesign
- dependency changes
- ownership changes
- major public API redesign

Requires:

- ADR
- governance approval
- documentation release
- engineering implementation
- engineering release

---

# Architectural Review Process

Every architectural change follows the same lifecycle.

```text
Proposal

↓

Architecture Review

↓

Governance Decision

↓

Documentation Update

↓

Implementation

↓

Engineering Review

↓

Engineering Release
```

Implementation must never skip documentation review.

---

# Public API Policy

Every public API shall exist in one of the following states.

- Experimental
- Internal
- Stable
- Deprecated
- Removed

Public APIs are considered contracts.

Breaking changes require governance approval.

---

# Crate Boundary Rules

## Runtime Domain

Owns:

- Assessment
- Lifecycle
- Runtime state
- Evidence
- Findings
- Recommendations

Must never:

- evaluate Rules
- generate Reports
- own Knowledge
- perform orchestration

---

## Rule Engine

Owns:

- deterministic evaluation
- rule execution
- rule outcomes

May consume Runtime state.

Must never mutate Assessment directly.

`RuleEngine::evaluate` fulfills selection, evaluation, Finding/Recommendation generation, and traceability inline, without delegating to separate internal types for each. See ADR-0010 and GOV-004 for why this is the approved pattern.

---

## Evidence Collection

Owns:

- inspection of an Assessment Subject's actual content
- production of Evidence

May consume:

- an application-supplied Assessment Input
- relevant Assessment Context

Must never:

- evaluate Evidence, or produce Findings or Recommendations
- mutate Assessment directly
- own orchestration (invoked by Engine; does not invoke itself)
- acquire its own Assessment Input (an application-layer responsibility)
- persist anything

See ADR-0008 and `EvidenceCollection.md` for the full boundary and Collector Contract. Architecture approved; a minimal implementation proving the boundary is complete (GOV-007, Sprint 3 Phase 4). Assessment Input ownership (GOV-009) and the Collection Error Model (GOV-010) are resolved for the filesystem case and implemented (Sprint 3 Phase 5): the first real collector discovers files and directories on the local filesystem. ZIP, XML, and Lua collectors remain future capabilities.

---

## Reporting

Owns:

- AssessmentReport
- report formatting
- report summarization
- traceability output

Must never:

- evaluate rules
- mutate runtime state

Reports are reflections of Assessment state.

They never create new state.

---

## Knowledge Domain

Owns:

- reusable engineering knowledge
- Rule definitions
- Repair Recipes
- Best Practices
- Compatibility Patterns
- Known Issues

Must remain independent from any individual Assessment.

---

## Engine

Owns:

- orchestration
- execution flow
- composition of platform services

Must never:

- own runtime state
- implement business rules
- generate reports

Composition of platform services means direct composition of each subsystem's own real type — not an intermediate layer of engine-local service objects. See ADR-0010 and GOV-004 (Resolved) for the full evaluation and decision record.

---

## CLI

Owns:

- user interaction
- command execution
- platform entry point

Must never contain business logic.

---

# Governance Register

Governance Items track architectural questions discovered during engineering.

Each item receives a permanent identifier.

Each item shall remain open until resolved by a Documentation Release.

---

## GOV-001

Title

Assessment Report Generation Timing

Status

Open

Raised

Engineering Release v0.1.0-alpha

Description

Assessment reports are currently generated before Assessment completion.

Question

Should reports represent the Assessment immediately before completion or after completion?

Resolution

Pending Documentation Release 1.1

---

## GOV-002

Title

Runtime Invariant Reconciliation

Status

Open

Raised

Engineering Release v0.1.0-alpha

Description

Implementation enforces lifecycle behaviors that are not yet fully represented within RuntimeInvariants.md.

Resolution

Pending Documentation Release 1.1

---

## GOV-003

Title

Role of modiq-common

Status

Open

Raised

Engineering Release v0.1.0-alpha

Description

The shared crate currently contains no implemented shared types.

Question

Should modiq-common become the repository for shared platform types, or should its architectural purpose be reconsidered?

Resolution

Pending

---

## GOV-004

Title

Engine Service Granularity

Status

Resolved

Raised

Engineering Release v0.1.0-alpha

Description

AssessmentService currently orchestrates the entire pipeline directly.

Question

Should future orchestration continue through AssessmentService, or transition to specialized Engine services?

Resolution

Approved by Technical Director, following `PROPOSAL_GOV-004.md`'s architectural direction, itself based on the implementation evidence gathered in `PLATFORM_VALIDATION_GOV-004.md`.

**Original concern.** `EngineAPI.md` (Documentation Release 1.0) described four intra-engine service objects — `KnowledgeService`, `ReportingService`, `RuleEvaluationService`, `VersionProfileService` — intended to mediate between `AssessmentService` and each subsystem. Whether `AssessmentService` should continue orchestrating directly, or transition to these specialized services, was left open at the platform's first Engineering Release.

**Implementation evidence.** All four services (and the mirrored `RuleSelector`/`EvidenceEvaluator`/`Explainability`/`Traceability` scaffolding in `modiq-rules`) originate from the platform's first foundational commit and have never been modified since. Across three Engineering Releases, three independently introduced real subsystems — the Rule Engine (Sprint 1), Reporting (Sprint 1), and Evidence Collection (Sprint 3) — were each wired into `AssessmentService` by direct instantiation of that subsystem's own real type. No construction site, method call, or test exists for any of the eight stub types anywhere in the workspace.

**Architectural decision.** `AssessmentService` remains the engine orchestration boundary. Direct composition of collaborating subsystems is the approved engine architecture. The internal `EngineAPI` service model is retired. The mirrored service scaffolding within `modiq-rules` is retired as part of the same architectural simplification, since it exists solely to support the same speculative model.

**Reasoning.** `Architecture.md` — the higher-authority specification `EngineAPI.md` itself subordinates to — already described subsystem delegation, not intra-engine service objects, in its own System Overview. Three subsystems converging independently on direct composition, without coordination, is strong evidence that this is the architecture the platform's own higher-authority specification actually produces when implemented honestly. No forcing function for the four-service model arrived in three Engineering Releases, and carrying it further would contradict the platform's own repeatedly validated principle: capability before abstraction.

**Implementation implications.** `EngineAPI.md` is amended to describe the approved architecture (see below). ADR-0010 records the decision. Deletion of the eight unused stub types (`modiq-engine`: `knowledge_service.rs`, `reporting_service.rs`, `rule_evaluation_service.rs`, `version_profile_service.rs`; `modiq-rules`: `selector.rs`, `evaluator.rs`, `explainability.rs`, `traceability.rs`) is authorized as future implementation work, not performed by this resolution. No behavioral change is expected: none of the eight types has ever been constructed or called.

---

## GOV-005

Title

New Finding Invariant — Evidence Reference Requirement

Status

Resolved (cardinality only)

Raised

Sprint 2 (Runtime Domain Content Implementation)

Description

Finding now carries an `evidence_ids` field, but no invariant requires it to be non-empty or to reference Evidence that exists within the same Assessment. RuntimeInvariants.md defines no invariant of this kind.

Question

Should a new invariant require every Finding to reference at least one Evidence item, validated against the Assessment's own Evidence collection?

Resolution

Approved by Technical Director, Sprint 3 Phase 2: a Finding SHALL reference at least one Evidence item (RuntimeInvariants.md, INV-013). Scope explicitly limited to cardinality; `Finding::new` rejects empty `evidence_ids` (`FindingError::EmptyEvidenceIds`). Referential-integrity validation (whether each referenced `EvidenceId` resolves within the same Assessment) was explicitly excluded from this decision and remains open, to be raised as a separate future governance item.

---

## GOV-006

Title

INV-005 Refinement — Recommendation Finding Reference Requirement

Status

Resolved (cardinality only)

Raised

Sprint 2 (Runtime Domain Content Implementation)

Description

Recommendation now carries a `finding_ids` field. INV-005 ("Recommendations SHALL only be produced from one or more Findings") is currently enforced only as a temporal precondition — that some Finding exists in the Assessment — not as a content-level requirement that the Recommendation reference specific, existing Finding(s).

Question

Should INV-005 be refined to require content-level reference validation, and should this be a wording change to INV-005 itself or a new, separate invariant?

Resolution

Approved by Technical Director, Sprint 3 Phase 2: a Recommendation SHALL reference at least one Finding item, recorded as a new invariant (RuntimeInvariants.md, INV-014) rather than a rewording of INV-005, since INV-005's existing text is not incorrect, only under-specified for this case. Scope explicitly limited to cardinality; `Recommendation::new` rejects empty `finding_ids` (`RecommendationError::EmptyFindingIds`). Referential-integrity validation (whether each referenced `FindingId` resolves within the same Assessment) was explicitly excluded from this decision and remains open, to be raised as a separate future governance item.

---

## GOV-007

Title

Evidence Collection Subsystem Implementation Approval

Status

Resolved

Raised

Sprint 3 Phase 3 (Evidence Collection Architecture & Governance Foundation)

Description

ADR-0008 establishes the architectural boundary for an Evidence Collection subsystem (new crate, orchestrated by `modiq-engine`) but authorizes architecture only, not implementation. No concrete collector, no new crate, and no change to `modiq-engine`'s orchestration code has been implemented.

Question

Once Documentation Release 2.1 is frozen, what specific implementation scope (a first concrete collector, the new crate's initial shape, and any accompanying test strategy) should be authorized, and in what order?

Resolution

Approved by Technical Director, Sprint 3 Phase 4: implementation scoped to the smallest slice that proves the ADR-0008 boundary, explicitly excluding ZIP/XML/Lua parsing and any collector trait or plugin mechanism. Delivered: a new `modiq-collection` crate (`InputDescriptor`, `InputDescriptorError`, `EvidenceCollector`), added to the Cargo workspace, depending only on `modiq-runtime`, exactly as ADR-0008 specified. Wired into `modiq-engine` via a new, additive `AssessmentService::execute_from_descriptor` method rather than a change to `execute` itself, since GOV-008 (whether `execute`'s own signature should change) remained separately open and unresolved. `apps/sandbox` updated to exercise the new capability. The complete pipeline (`AssessmentService` → Evidence Collection → Evidence → `Assessment` → Rule Engine → `AssessmentReport`) is demonstrated end-to-end via the real engine, verified by 106 passing workspace tests (up from 95) plus the sandbox's own 3. GOV-008, GOV-009, and GOV-010 remain open, untouched and un-prejudiced by this implementation.

---

## GOV-008

Title

AssessmentService Public API Evolution

Status

Open

Raised

Sprint 3 Phase 3 (Evidence Collection Architecture & Governance Foundation)

Description

ADR-0009 records that `modiq-engine`'s public Assessment-execution entry point will require a breaking change to accept an Input Descriptor once Evidence Collection is implemented, without deciding that change's shape.

Question

Should the existing entry point's signature change directly, should a new parallel entry point be introduced with the old one deprecated, or should some other approach be taken? What is the exact new input shape?

Resolution

Pending. Must be resolved, and separately approved under the Public API Policy's breaking-change requirement, before Evidence Collection implementation can integrate with `modiq-engine`.

Reviewed during Platform Validation Phase 1, following GOV-004's resolution (`PLATFORM_VALIDATION_GOV-008.md`, `PLATFORM_VALIDATION_EXECUTION_CONTRACT.md`): implementation evidence gathered was found insufficient to resolve this item. No architectural change is authorized by that review. The current `AssessmentService` execution contract — both public entry points, `AssessmentInput`, `AssessmentReport`, and the public error model — remains the approved platform boundary until future implementation provides additional evidence. Status remains Open.

---

## GOV-009

Title

Assessment Input Ownership

Status

Resolved

Raised

Sprint 3 Phase 3 (Evidence Collection Architecture & Governance Foundation)

Description

`EvidenceCollection.md` defines the Input Descriptor conceptually (what an application supplies to identify content for Evidence Collection to inspect) but does not authoritatively assign which specification owns its definition or what content it eventually carries.

Question

Which specification should own the Input Descriptor's authoritative definition — EvidenceCollection.md, DataModel.md (as content on Assessment Subject or Assessment Context), or a new specification — and what should it contain?

Resolution

Approved by Technical Director, following `PROPOSAL_FILESYSTEM_COLLECTION.md`'s architecture: `EvidenceCollection.md` owns the authoritative definition, formalized as **Assessment Input** (the term "Input Descriptor" was the Sprint 3 Phase 3/4 placeholder; Sprint 3 Phase 5 renamed the corresponding Rust types — `InputDescriptor` to `AssessmentInput`, `InputDescriptorError` to `AssessmentInputError` — reconciling implementation with this vocabulary). Resolved for the filesystem case only, matching the proposal's own explicitly limited scope:

- Assessment Input represents a stable reference to a filesystem object at the moment collection begins.
- Files are valid Assessment Inputs.
- Directories are valid Assessment Inputs.
- Non-existent paths are invalid input.
- Symbolic links are intentionally not traversed for the first real collector (see `EvidenceCollection.md`, Symbolic Link Policy) — an explicit Phase 5 boundary, not a permanent one.
- The Collection subsystem consumes an Assessment Input; it never creates or reinterprets one — acquiring it remains an application-layer concern, unchanged from the existing Non-Responsibilities boundary.
- Future input types (archives, remote sources, virtual sources, and any non-filesystem origin) are intentionally out of scope and explicitly deferred; this resolution does not claim to be Assessment Input's final shape for every future collector.

Full definition recorded in `EvidenceCollection.md`, Assessment Input section.

---

## GOV-010

Title

Collection Error Model

Status

Resolved

Raised

Sprint 3 Phase 3 (Evidence Collection Architecture & Governance Foundation)

Description

`EvidenceCollection.md`'s Collector Contract requires that collection failure be represented distinctly from legitimate absence (successfully inspecting content and finding nothing relevant), but does not define the mechanism.

Question

How should a Collector's failure to complete inspection be represented and surfaced — as part of Evidence itself, as a distinct error/result type, as a new kind of Finding, or some other mechanism — and how should it interact with Assessment lifecycle state?

Resolution

Approved by Technical Director, following `PROPOSAL_FILESYSTEM_COLLECTION.md`'s architecture. Every collection attempt resolves to exactly one of four architectural outcomes:

1. **Invalid Input** — the Assessment Input itself is malformed or empty. Collection never begins.
2. **Inaccessible Input** — the input is well-formed but its location cannot be reached (does not exist, access denied, unavailable storage). Collection aborts.
3. **Unsupported Input** — the location is reachable but is not a supported kind (e.g., not a regular file or directory). Collection aborts.
4. **Empty Collection** — the input is valid, reachable, and supported, but structurally contains nothing. Collection **succeeds**, producing zero Evidence. This is explicitly **not** an error.

Successful collection (including Empty Collection) is distinct from a successful, or even meaningful, Assessment: Collection succeeding means only that Evidence Collection completed its own responsibility, never a claim about what the Rule Engine will later conclude from the (possibly empty) result.

For the first real collector (Sprint 3 Phase 5, filesystem collection), collection is atomic: it either completes (including as Empty Collection) or the Assessment terminates entirely — no partial Assessment, no partial Evidence, no partial Report. This is an explicit, intentional Phase 5 scope decision, not a permanent platform limitation; a future collector or phase may revisit it (e.g., incremental collection across multiple sources). No `RuntimeInvariants.md` change was required: no Runtime aggregate invariant governs this — a collection failure means the Assessment's lifecycle simply never progresses far enough to produce a Report, which existing invariants (INV-002, INV-003) already accommodate without modification. This is an Engine-orchestration policy, not a Runtime state concern.

Full definition recorded in `EvidenceCollection.md`, Collection Outcomes and Collection Atomicity (Phase 5) sections.

---

## GOV-011

Title

Archive Collection Model

Status

Resolved

Raised

Sprint 4 Phase 1 (ZIP Evidence Collection — Governance Preparation)

Description

`PROPOSAL_ZIP_EVIDENCE_COLLECTION.md` (approved) and `SPRINT4_IMPLEMENTATION_PLAN.md` require the same class of architectural decisions GOV-009 and GOV-010 resolved for the filesystem case, applied to the archive case: how a malformed or corrupt archive is categorized within the Collection Error Model, how duplicate entry names within a single archive are handled, what resource limits bound archive inspection, and how an entry name that would resolve outside the archive's own extraction boundary is handled.

Question

1. Is a malformed or corrupt archive an instance of the existing Unsupported Input outcome, or does it warrant its own distinct outcome?
2. How are duplicate entry names within a single archive categorized?
3. What resource limits (entry count, claimed decompression ratio) bound archive inspection, and is exceeding them a new outcome or an instance of an existing one?
4. What is the archive-format analog of the Symbolic Link Policy — how is an entry name that would resolve outside the archive's own conceptual root handled?

Resolution

Approved by Technical Director in its entirety, following `PROPOSAL_GOV-011.md` (informed by `SPRINT4_IMPLEMENTATION_PLAN.md` Phase 2 Boundary-Proving evidence). All four questions resolved:

**Question 1 (malformed or corrupt archives):** an instance of the existing Unsupported Input outcome. No new outcome. A location reachable but whose content is not a well-formed archive is architecturally the same case as a device file or symbolic-link root in the filesystem case — reachable, but not a usable kind of thing for this Collector.

**Question 2 (duplicate entry names):** the platform does not silently resolve duplicates to a last-write-wins result, and does not fabricate a discrete Evidence item for an entry a collection mechanism cannot actually observe. Where duplicate entries are detected, that detection is itself recorded as an observable fact of the collection. This supersedes the originally drafted candidate (each duplicately-named entry produces its own discrete Evidence item), which Phase 2 Boundary-Proving found technically unachievable against the investigated dependency — confirmed independently via three separate tools that an archive containing two identically-named entries was only partially observable through that dependency's ordinary enumeration API. The precise representation mechanism is an implementation detail, not fixed by this resolution — see `EvidenceCollection.md`, Duplicate Archive Entry Policy.

**Question 3 (resource limits):** bound both entry count and the ratio between an entry's claimed uncompressed size and its compressed size, checked from archive metadata alone, before any content is decompressed. Exceeding either bound is Unsupported Input (Question 1's category), not a new outcome. Phase 2 confirmed both quantities are recoverable from metadata alone, verified empirically by timing (metadata-only enumeration measured at roughly 1,270 times faster than full decompression of the same content). Exact numeric thresholds remain an implementation detail, to be calibrated during Sprint 4 Phase 3.

**Question 4 (archive traversal boundary, including absolute paths):** the collector SHALL normalize archive entry paths. An entry containing a path-traversal sequence SHALL be treated as an invalid archive entry and skipped; it is not recorded as Evidence. Collection SHALL continue for all remaining valid entries. An archive SHALL NOT be rejected solely because one or more entries are invalid under this policy — only an archive that cannot be read or parsed at all terminates collection. An archive entry that was originally an absolute path is treated as a path-traversal violation under this same policy, **independent of any dependency's internal sanitization** — the collector shall not rely on a dependency's sanitized path representation as evidence that such an entry is acceptable, since Phase 2 confirmed the investigated dependency sanitizes an absolute-path entry into an accepted, safe-looking relative form without preserving the fact that it was originally absolute. This aligns with the existing Symbolic Link Policy (`EvidenceCollection.md`) and the platform's evidence-first philosophy.

**Archive Metadata Policy — Approved:** unless a documented product requirement states otherwise, archive metadata SHALL NOT participate in Assessment Evidence. Excluded: timestamps, permissions, ownership, archive comments, host operating system metadata, and compression metadata that does not affect deterministic evidence. Evidence produced by archive collection is based only on deterministic inputs — normalized paths, archive structure, and file identity/content where applicable.

**Implementation-mechanism questions remaining** (the exact Rust representation for Question 2's observable fact, the exact detection mechanism, the exact numeric thresholds for Question 3, and the exact absolute-path check for Question 4) do not block implementation, provided implementation faithfully realizes the policy resolved above. These do not require further governance approval unless implementation surfaces a genuine architectural question beyond what this resolution already answers.

Full definition recorded in `EvidenceCollection.md`, Archive-Specific Outcomes, Duplicate Archive Entry Policy, and Archive Traversal Boundary Policy sections.

---

# Documentation Release Process

Architecture evolves through Documentation Releases.

A Documentation Release represents the authoritative state of the platform architecture.

Engineering Releases represent the implementation state of the repository.

The two are intentionally independent.

---

# Repository Development Cycle

The expected development cadence is:

```text
Documentation Release

↓

Sprint Planning

↓

Implementation

↓

Engineering Review

↓

Engineering Release

↓

Governance Review

↓

Documentation Release
```

This process ensures implementation continually validates the architecture while governance continuously reconciles engineering discoveries.

---

# Governance Objectives

Governance exists to preserve long-term architectural integrity.

Its objectives are to:

- Maintain deterministic behavior.
- Preserve architectural boundaries.
- Prevent implementation drift.
- Protect public API stability.
- Ensure documentation remains authoritative.
- Capture architectural decisions as permanent repository artifacts.
- Enable sustainable evolution of the platform without sacrificing clarity or consistency.
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

## Storage

Owns:

- durability of an already-generated `AssessmentReport`, beyond the lifetime of the process that produced it
- Storage's own persisted representation of a report's content, populated only from `AssessmentReport`'s already-public API

Must never:

- participate in Evidence Collection, Rule Evaluation, or Report generation, or be consulted during any of them
- mutate a stored report once written
- persist `Assessment`, `Evidence`, `Finding`, or `Recommendation` as individually addressable entities — only the already-bundled `AssessmentReport` snapshot
- require any change to `AssessmentService`'s public entry points

Storage sits strictly downstream of Reporting in the platform's one-directional information flow (Evidence → Rule Evaluation → Findings → Recommendations → Report → Storage) and never feeds back upstream. Activated Sprint 13 (`modiq-storage`, its first real content since the crate's own creation) as an instance of Architectural Activation (`SPRINT8_ARCHITECTURAL_RESOLUTION.md` §8), per the governance reconciliation recorded in `PROJECT_HANDOFF_v1.1.md` §5 and `GOVERNANCE_OBSERVATION_SUBSYSTEM_ACTIVATION.md` — not through the Capability Identity procedure, which does not classify a subsystem-level candidate (`INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md` §3). See `STORAGE_ARCHITECTURE_EVALUATION.md`, `STORAGE_IMPLEMENTATION_AUTHORIZATION.md`, and `STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md` for the full evaluation, authorization, and representation rationale.

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

Assessment reports are currently generated before Assessment completion — confirmed, post-Sprint 13, to match `DataModel.md`'s own canonical Runtime Lifecycle diagram exactly, which documents "Assessment Report Produced" as the step immediately preceding "Assessment Completed." `AssessmentService::execute`/`execute_from_assessment_input` both call `AssessmentReport::generate` once, before `Assessment::complete()` — enforced directly by an existing test, `execute_reflects_state_at_report_generation_prior_to_completion`. `Assessment::complete()` itself changes only the `status` field, confirmed directly against its own implementation and against `modiq-report`'s own `generate_after_completion_matches_generate_before_completion` test — Evidence, Findings, and Recommendations are identical before and after completion.

Question

The item's original framing — "before or after completion?" — is already answered by the documented specification, and the implementation conforms to it exactly; no inconsistency exists. The real, narrower question: because both public entry points generate exactly once, always before completion, and never again afterward, `AssessmentStatus::Completed` is a value no report produced by either entry point can ever hold in practice. Is that an intended consequence of the documented lifecycle ordering, or an unexamined gap? Ephemeral reports gave this low practical stakes; `modiq-storage`'s own Sprint 13 activation durably persists whatever status a report holds, for the first time giving the question real, if still unforced, weight.

Resolution

Not resolved. Architecture Evaluation performed post-Sprint 13 (no Architectural Resolution) found no inconsistency between specification and implementation — this item's original framing is satisfied as documented. Returned to Open under the narrower question above, deliberately, per this project's own evidence-based resolution discipline (the same restraint already applied to GOV-008 and GOV-013): no implementation currently depends on a report ever reflecting `Completed` status, so no concrete forcing function yet requires deciding the status field's own semantics. Full investigation record: `docs/engineering/ENGINEERING_LOG.md` (post-Sprint-13 entry). Revisit once a real forcing function exists — most plausibly a future Storage capability or consumer that reasons about a persisted report's own completion state directly.

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

## GOV-012

Title

Rule Evaluation Model

Status

Resolved

Raised

Sprint 5 Phase 1 (Assessment Intelligence Layer — Design Preparation)

Description

Since Sprint 1, `modiq-rules::RuleEngine` has evaluated exactly one Rule, applied unconditionally to all Evidence regardless of category. `SPRINT5_IMPLEMENTATION_PLAN.md` requires a second, category-specific Rule, which raises the same class of architectural question GOV-009/010/011 resolved for Evidence Collection, applied here to Rule Evaluation for the first time: how `RuleEngine::evaluate` represents more than one Rule's outcome, in what order, and whether Rules interact with or suppress one another.

Question

1. Does `RuleEngine::evaluate` return a single aggregated outcome, or one outcome per matching Rule?
2. When more than one Rule matches, in what order are the resulting Findings and Recommendations produced?
3. When more than one Rule matches the same Evidence, does a more specific Rule suppress a more general one, or do both fire independently?

Resolution

Approved by Technical Director in its entirety, following `SPRINT5_IMPLEMENTATION_PLAN.md`'s Design Questions 1–3. All three questions resolved:

**Question 1 (return shape):** `RuleEngine::evaluate` returns `Vec<RuleOutcome>` — zero, one, or several `(Finding, Recommendation)` pairs, one per matching Rule. This requires no change to `Finding`'s or `Recommendation`'s existing one-Rule-per-Finding shape (`Finding.rule_reference` remains a single `RuleReference`), and keeps each Rule's Finding independently traceable to exactly the Rule and Evidence that produced it, per `RuleEngine.md`'s Traceability Management responsibility.

**Question 2 (ordering):** Rules are evaluated, and their outcomes produced, in a fixed, explicit declaration order internal to `RuleEngine` — the order Rules are listed in `evaluate`'s own dispatch logic — never an order derived from Evidence's own arrival sequence. This mirrors the same discipline `ArchiveReader::entries()` and `EvidenceCollector`'s directory traversal already apply: explicit order imposed by the producer, never inherited from an unordered or incidentally-ordered source.

**Question 3 (composition):** Rules compose independently; no suppression model exists. Every Rule is evaluated against whichever Evidence it applies to, regardless of whether another Rule also matches related or overlapping Evidence. An Assessment whose Evidence matches both the existing generic Rule and a new category-specific Rule produces both Rules' Findings and Recommendations, not one at the expense of the other. This avoids inventing a precedence/suppression mechanism this Sprint's own evidence does not yet justify — only two concrete Rules exist even after Sprint 5 — consistent with `RuleEngine.md`'s description of Rule Selection as determining "which Rules are applicable," plural and independent, not a single winner-takes-all choice.

**Implementation-mechanism questions remaining:** the exact internal dispatch structure of `RuleEngine::evaluate` (a `match` over category, a sequence of `if let` checks, or another shape) is an implementation detail, not fixed by this resolution, provided no trait, registry, factory, or plugin mechanism is introduced — this document's Crate Boundary Rules already states inline fulfillment as `RuleEngine`'s approved pattern (ADR-0010, GOV-004), and this resolution does not revisit that.

Full definition recorded in `SPRINT5_IMPLEMENTATION_PLAN.md`, Design Questions 1–3.

---

## GOV-013

Title

FindingSeverity Severity/Kind Conflation

Status

Open

Raised

Sprint 5 Phase 1 (Assessment Intelligence Layer — Design Preparation), during Technical Director review of the `FindingSeverity` semantic definitions

Description

`FindingSeverity` (`Error`, `Warning`, `Informational`, `BestPractice`) has existed in `modiq-runtime` since Sprint 2 but was exercised by only one variant, `Informational`, until Sprint 5's second Rule required a real choice among all four. Drafting semantic definitions for each variant surfaced that `BestPractice` does not sit on the same axis the other three do. `Error`/`Warning`/`Informational` answer one question — how urgent is this? — on a single ordered scale. `BestPractice` answers a different question — what kind of observation is this? — independent of urgency. A real best-practice deviation could independently be more or less urgent than another, and nothing in the current model can express that, since `BestPractice` and `Warning` are mutually exclusive values of the same field.

Question

Does this conflation warrant splitting `FindingSeverity` into two independent concepts — an ordered severity scale, and a separate, orthogonal classification of Finding kind — or is the existing four-variant model adequate as actually used in practice?

Resolution

Not resolved. Technical Director decision, recorded here rather than acted on: `FindingSeverity` remains unchanged for Sprint 5. The current model is provisionally accepted, not confirmed permanently correct. This item stays Open, to be revisited once the Rule Engine has multiple concrete Rules operating in practice and this question can be evaluated against real implementation evidence — not decided from two Rules alone. This is the same evidence-based resolution discipline GOV-004 and GOV-011 both already applied: a concrete forcing function should justify a model change, not the reverse.

Full definition recorded in `DataModel.md`, Finding Severity section, and `SPRINT5_IMPLEMENTATION_PLAN.md`, Design Question 5.

---

## GOV-014

Title

Lua Fixture Acquisition Governance

Status

Open

Raised

Following `INV-001_LUA_ANALYSIS_CAPABILITY.md` (Lua Analysis Capability Investigation)

Description

INV-001 found that no real Farming Simulator mod Lua script has ever been examined by this project, and that acquiring one requires the same real, human-performed, provenance-tracked acquisition discipline Sprint 10 established for runtime log fixtures (`fixtures/runtime-logs/README.md`: Runtime Log Normalization, Warning Categorization, Installation State versus Savegame State). That policy set was written specifically for captured runtime session logs. A Lua script is a structurally different kind of artifact — real, potentially copyrighted third-party source code, not a captured session transcript — and Sprint 10's own policies do not automatically extend to it without their own review.

Question

Before any future Lua fixture may be acquired, what provenance requirements, licensing/consent expectations, storage policy (most concretely: whether real third-party Lua source code is ever committed to this repository at all, given that mod archives themselves are never stored, per Sprint 10's own explicit policy), and acquisition-governance discipline must be established — mirroring, but not assumed identical to, `fixtures/runtime-logs/README.md`'s own standing policies?

Resolution

Pending. Must be resolved before any Lua fixture acquisition begins, and before Lua Analysis's own Architecture Evaluation may be authorized, per `INV-001_LUA_ANALYSIS_CAPABILITY.md`'s own Recommendation A. This is architectural governance work, not implementation, and not Sprint work — consistent with how Sprint 10's own fixture-corpus policies (Normalization, Warning Categorization, Installation State vs. Savegame State) were each resolved as governance/documentation decisions before the fixture that needed them was accepted, never worked around informally.

---

# Documentation Release Process

Architecture evolves through Documentation Releases.

A Documentation Release represents the authoritative state of the platform architecture.

Engineering Releases represent the implementation state of the repository.

The two are intentionally independent.

---

# Repository Development Cycle

Operates at Documentation Release granularity, spanning multiple Sprints — distinct from `PROJECT_HANDOFF_v1.0.md` Section 5's canonical per-Sprint workflow, which this cycle contains one instance of at its "Sprint Planning → Implementation → Engineering Review" phase.

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
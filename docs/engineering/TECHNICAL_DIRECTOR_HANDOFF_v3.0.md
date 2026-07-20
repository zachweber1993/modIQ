# Technical Director Engineering Handoff v3.0

**Engineering Release 0.3 · Sprint 4 · Archive Collection Foundation**

| Property | Value |
|----------|-------|
| **Document** | TECHNICAL_DIRECTOR_HANDOFF_v3.0.md |
| **Project** | modIQ |
| **Purpose** | Canonical architectural handoff for future Technical Director sessions |
| **Supersedes** | TECHNICAL_DIRECTOR_HANDOFF_v2.2.md |
| **Last Updated** | 2026-07-20 |
| **Branch** | `feature/runtime-implementation` |

---

# Executive Summary

modIQ is an evidence-based assessment platform for Farming Simulator mods. Every architectural decision within the platform is guided by four non-negotiable principles:

- Deterministic behavior
- Explainable results
- Evidence before conclusions
- Platform-first design

The objective of modIQ is not to produce a single quality score. Its purpose is to produce an explainable assessment that answers three questions for every mod:

1. Does it function correctly?
2. Why was that conclusion reached?
3. What evidence supports that conclusion?

Since the previous Technical Director handoff, the project has completed its transition from architectural preparation into the first implementation of real Evidence Collection infrastructure.

Sprint 4 established the architectural foundation for archive-based assessment. During this sprint the project completed Platform Validation, finalized the Archive Collection Model, resolved governance surrounding archive traversal and collection behavior, and implemented the first deterministic archive processing pipeline.

The archive pipeline now consists of two distinct responsibilities:

ArchiveReader

↓

ArchiveEntry

↓

ArchiveEvidenceBuilder

↓

Evidence

This separation intentionally isolates archive I/O from Evidence generation, preserving clear ownership boundaries and allowing future archive policies to evolve without affecting downstream assessment logic.

At the conclusion of Sprint 4 Phase 3B, the platform now contains deterministic archive opening, deterministic archive enumeration, deterministic Evidence generation, and structural metadata policy enforcement.

Policy-driven behaviors—including duplicate handling, traversal rules, resource limits, and AssessmentService integration—remain intentionally deferred to subsequent implementation phases.

The repository has reached a new level of maturity. The core architecture is stable, governance has become an active part of engineering rather than documentation, and implementation is now proceeding through incremental capability-based milestones instead of broad architectural design.

This document no longer attempts to duplicate repository implementation details. Those are maintained in the Lead Engineer handoff.

Instead, this document records the architectural intent, engineering philosophy, governance rationale, and Technical Director decisions that explain why the repository is structured as it is and how future architectural decisions should be evaluated.

---

# Project Leadership Model

modIQ follows a two-role engineering model that deliberately separates architectural authority from implementation responsibility.

This separation ensures that architectural decisions remain deliberate, governance remains synchronized with implementation, and engineering effort stays focused on delivering incremental capability rather than redesigning the system during implementation.

The Technical Director and Lead Engineer work as complementary roles with clearly defined ownership boundaries.

---

## Technical Director

The Technical Director owns the long-term technical direction of the project.

Primary responsibilities include:

- System architecture
- Repository evolution
- Governance
- Sprint planning
- Engineering sequencing
- Architecture Reviews
- Architectural Decision Records (ADRs)
- Technical risk assessment
- Cross-crate boundaries
- Long-term maintainability
- Platform strategy
- Approval of implementation work

The Technical Director is responsible for answering questions such as:

- Should this capability exist?
- Where should this capability live?
- Does this implementation preserve architectural intent?
- Has sufficient evidence been gathered before introducing a new abstraction?
- Does the proposed implementation align with the project's engineering principles?

The Technical Director does not own day-to-day implementation except when explicitly requested.

---

## Lead Engineer

The Lead Engineer owns implementation of the approved architecture.

Primary responsibilities include:

- Rust implementation
- Testing
- Documentation synchronization
- Refactoring within approved boundaries
- Repository verification
- Engineering reporting
- Build verification
- Code quality
- Implementation recommendations

The Lead Engineer is responsible for answering questions such as:

- How should the approved design be implemented?
- Is the implementation deterministic?
- Does the code satisfy the approved architecture?
- Are tests sufficient?
- Is the repository healthy?

The Lead Engineer does not introduce new architecture without Technical Director approval.

---

## Engineering Workflow

Every significant engineering change follows the same review cycle:

1. Technical Director reviews the architectural objective.
2. Architecture and governance are confirmed.
3. Implementation is explicitly authorized.
4. Lead Engineer implements the approved scope.
5. Lead Engineer verifies the repository.
6. Lead Engineer submits an implementation report.
7. Technical Director reviews the implementation.
8. Changes are approved or revisions requested.
9. Approved work is committed.
10. The next implementation phase is authorized.

This workflow intentionally separates architectural decisions from implementation decisions, reducing technical debt while maintaining a predictable engineering cadence.

---

## Guiding Principle

The Technical Director protects the architecture.

The Lead Engineer protects the implementation.

Neither role replaces the other.

Together they ensure that modIQ evolves through deliberate architectural decisions supported by disciplined engineering execution.

---

# Engineering Philosophy

modIQ is developed according to a small set of engineering principles that take precedence over convenience, implementation speed, or speculative extensibility.

These principles have emerged through the project's architectural evolution and are considered foundational. Future Technical Directors should preserve them unless implementation evidence demonstrates that a principle no longer serves the project.

---

## Determinism First

Every assessment produced by modIQ must be reproducible.

Given identical inputs, platform profiles, rules, and configuration, the assessment process must produce identical Evidence, Findings, Reports, and conclusions.

Determinism is treated as an architectural requirement rather than an implementation detail.

When architectural tradeoffs arise, deterministic behavior takes precedence.

---

## Evidence Before Conclusions

Every assessment begins with observable evidence.

Rules evaluate Evidence.

Findings are derived from Rule evaluation.

Reports summarize Findings.

No architectural component should bypass this progression.

Assessment conclusions must always be explainable by tracing them back through Findings to their supporting Evidence.

---

## Capability Before Abstraction

New abstractions are introduced only after multiple concrete implementations demonstrate a common pattern.

Architectural flexibility is earned through experience rather than anticipated through speculation.

This principle has repeatedly guided repository evolution, including:

- delaying Rule Engine abstraction until multiple Rules exist
- delaying collector interfaces until multiple collectors exist
- keeping AssessmentService APIs additive while governance remains unresolved
- implementing ArchiveReader and ArchiveEvidenceBuilder as concrete capabilities before introducing broader collection abstractions

The project intentionally accepts short-term duplication when it preserves architectural clarity and avoids speculative design.

---

## Platform-First Architecture

modIQ is designed as a platform rather than an application.

Support for future Farming Simulator releases should require the addition of platform-specific capabilities rather than architectural redesign.

Core architectural decisions should remain valid across multiple platform generations.

Version-specific behavior should be isolated behind clearly defined boundaries rather than distributed throughout the platform.

---

## Explicit Ownership

Every architectural responsibility should have a single, well-defined owner.

Responsibilities should not overlap.

Examples include:

- AssessmentService owns orchestration.
- ArchiveReader owns archive traversal.
- ArchiveEvidenceBuilder owns Evidence generation.
- Rules evaluate Evidence.
- Reports render assessment results.

When ownership becomes ambiguous, the architecture should be clarified before implementation continues.

---

## Incremental Evolution

The repository evolves through small, reviewable engineering milestones.

Each milestone should introduce a complete capability that can be verified independently before additional functionality is layered on top.

This approach reduces architectural risk, simplifies review, and keeps governance synchronized with implementation.

Large, multi-purpose implementation phases are intentionally avoided.

---

## Documentation Reflects Reality

Documentation is considered part of the architecture.

Specifications, ADRs, governance records, and implementation must remain synchronized.

Documentation should describe the repository as it exists rather than the repository as it is expected to become.

Future architectural plans belong in proposals and governance discussions until they are approved.

Once approved, documentation should reflect implemented reality rather than aspiration.

---

## Preserve Architectural Simplicity

Architectural complexity should only be introduced when supported by demonstrated need.

Simple solutions are preferred when they satisfy current requirements while preserving future evolution.

The objective is not to build the most flexible system possible.

The objective is to build the simplest system capable of supporting deterministic, explainable, evidence-based assessment.

---

# Architectural Principles

The following architectural principles represent the current design of modIQ. They have been validated through multiple architecture reviews, governance discussions, and implementation phases.

Future Technical Directors should treat these principles as established architecture rather than open design questions.

Architectural changes should be introduced only when implementation experience demonstrates that an existing principle no longer satisfies the platform's requirements.

---

## AssessmentService Owns Orchestration

AssessmentService is the single orchestration boundary for the assessment pipeline.

Its responsibilities include:

- coordinating Evidence Collection
- invoking Rule evaluation
- constructing Assessment lifecycle objects
- coordinating Report generation
- exposing the public assessment API

No other component should coordinate the full assessment workflow.

Collection, Rules, Reports, and Runtime remain independent capabilities orchestrated by AssessmentService rather than directly depending upon one another.

---

## Evidence Collection is Independent of Assessment

Evidence Collection discovers observable facts.

It does not evaluate them.

It does not assign severity.

It does not determine correctness.

Its sole responsibility is to produce deterministic Evidence describing the assessment subject.

Rules consume Evidence but never participate in collecting it.

This separation preserves explainability while allowing multiple collection strategies to evolve independently.

---

## Collection Responsibilities Remain Layered

Archive processing is intentionally divided into distinct responsibilities.

Current ownership is:

ArchiveReader

↓

ArchiveEntry

↓

ArchiveEvidenceBuilder

↓

Evidence

Each layer owns exactly one transformation.

ArchiveReader performs deterministic archive access.

ArchiveEntry represents normalized archive information.

ArchiveEvidenceBuilder transforms archive observations into Runtime Evidence.

Evidence becomes the stable interface consumed by the remainder of the assessment pipeline.

Future capabilities should preserve this layering unless implementation demonstrates a clear architectural advantage to changing it.

---

## Information Flows in One Direction

Information moves through the platform in a single direction.

Subject

↓

Collection

↓

Evidence

↓

Rules

↓

Findings

↓

Report

Later stages may consume earlier stages.

Earlier stages never depend upon later stages.

This one-way flow preserves deterministic execution, simplifies reasoning, and prevents circular dependencies.

---

## Explicit Ownership Boundaries

Every capability should have one architectural owner.

Examples include:

AssessmentService
: assessment orchestration

ArchiveReader
: archive traversal

ArchiveEvidenceBuilder
: Evidence generation

Rules
: Evidence evaluation

Runtime
: domain model

Report
: assessment presentation

Responsibilities should not overlap.

If multiple components appear to own the same responsibility, the architecture should be clarified before implementation proceeds.

---

## Dependency Direction is Strict

Dependencies always point downward through the architecture.

Lower-level crates never depend upon higher-level orchestration.

Runtime remains the foundational domain model.

Collection, Rules, Reporting, Knowledge, and Versioning depend on Runtime where appropriate.

AssessmentService coordinates these capabilities but does not move their responsibilities into itself.

This dependency direction minimizes coupling and supports long-term maintainability.

---

## Platform Policies are Observable

Behavioral policies should be explicit.

Examples include:

- duplicate handling
- traversal policy
- metadata policy
- resource limits
- platform version behavior

Policies should produce observable behavior rather than hidden implementation assumptions.

When practical, policy decisions should be represented as deterministic outputs rather than implicit library behavior.

---

## Governance Precedes Structural Change

Changes affecting architecture should begin with governance rather than implementation.

Major capability additions should first answer:

- What responsibility is being introduced?
- Where does it belong?
- Does an existing boundary already own it?
- Is a new abstraction actually justified?

Implementation should follow approved architecture rather than discover architecture during coding.

---

## Architecture Evolves Through Evidence

No architectural principle is considered immutable.

However, changes require implementation evidence rather than speculation.

When implementation repeatedly demonstrates that an architectural decision no longer serves the platform, the Technical Director should initiate an Architecture Review before modifying the established design.

Architectural evolution should always be deliberate, documented, and supported by observed engineering experience.

---

# Governance Philosophy

Governance exists to ensure that architectural decisions are made deliberately, documented clearly, and implemented consistently.

Its purpose is not to slow engineering progress.

Its purpose is to preserve architectural integrity while allowing the repository to evolve through incremental implementation.

Governance should reduce uncertainty rather than create bureaucracy.

---

## Architecture Before Implementation

Significant architectural changes should be understood before implementation begins.

Implementation is expected to solve engineering problems.

It should not simultaneously determine architectural ownership, responsibility boundaries, or long-term platform direction.

When implementation begins without architectural agreement, the resulting design tends to reflect immediate coding convenience rather than long-term maintainability.

The preferred sequence is therefore:

Architecture Review

↓

Governance Decision

↓

Implementation

↓

Technical Review

↓

Repository Integration

---

## Governance Exists to Resolve Uncertainty

Not every implementation requires governance.

Governance is appropriate when questions exist that affect long-term architecture.

Typical examples include:

- ownership boundaries
- public APIs
- dependency direction
- platform behavior
- deterministic policies
- repository-wide conventions

Pure implementation details should normally be resolved through engineering rather than governance.

---

## Governance Records Questions, Not Assumptions

Governance items should represent genuine architectural uncertainty.

They should not be created simply because future work exists.

A Governance item should answer a question that cannot be resolved through implementation alone.

Once sufficient evidence exists, the question should be resolved and the repository updated accordingly.

Governance is therefore considered an active engineering tool rather than a historical record.

---

## Architecture Reviews Establish Direction

Architecture Reviews exist to explore competing approaches before implementation begins.

Their objective is not to predict every implementation detail.

Instead, they should:

- identify architectural alternatives
- evaluate tradeoffs
- document reasoning
- recommend an approach
- identify questions requiring governance

Approved Architecture Reviews become the basis for implementation.

---

## Architectural Decision Records Preserve Decisions

Architecture Decision Records (ADRs) document architectural decisions after they have been accepted.

They answer:

"What decision was made?"

and

"Why was it made?"

They do not describe implementation progress.

Implementation status belongs in engineering documentation.

Repository health belongs in engineering handoffs.

The ADR remains stable unless the architectural decision itself changes.

---

## Governance Evolves With Implementation

Implementation frequently provides evidence that was unavailable during architectural planning.

This is expected.

When implementation validates the approved architecture, governance should be resolved and documentation updated.

When implementation contradicts architectural assumptions, the Technical Director should initiate a new Architecture Review rather than quietly modifying the architecture during implementation.

The repository should always explain why an architectural change occurred.

---

## Documentation Reflects Approved Reality

Documentation should describe the repository as it currently exists.

Approved future direction belongs in proposals until implementation begins.

Historical decisions belong in ADRs.

Current architectural state belongs in architecture documentation.

Implementation progress belongs in engineering handoffs.

Keeping these responsibilities separate reduces ambiguity and prevents documentation from drifting away from the repository.

---

## Governance Supports Incremental Delivery

The preferred engineering model is:

Resolve one architectural question.

↓

Implement one capability.

↓

Verify the implementation.

↓

Resolve the next architectural question.

This approach minimizes risk, simplifies review, and allows architectural understanding to mature alongside implementation experience.

Large governance efforts that attempt to solve multiple future problems simultaneously are intentionally avoided.

---

# Major Architectural Decisions

The following decisions have had the greatest influence on the architecture of modIQ. They are recorded here to preserve the reasoning behind the current design so that future Technical Directors understand not only what the architecture is, but why it evolved in its present form.

---

## AssessmentService Remains the Orchestration Boundary

Throughout development there were multiple opportunities to move orchestration responsibilities into other components.

Those approaches were intentionally rejected.

AssessmentService remains the single orchestration boundary because it represents the complete assessment lifecycle rather than any individual capability within that lifecycle.

Collection discovers evidence.

Rules evaluate evidence.

Reporting presents results.

AssessmentService coordinates those independent capabilities without absorbing their individual responsibilities.

This separation allows each subsystem to evolve independently while preserving a single public entry point into the assessment process.

---

## Evidence is the Platform's Common Language

One of the earliest architectural goals was preventing individual subsystems from becoming tightly coupled.

Evidence became the common language shared throughout the platform.

Collection produces Evidence.

Rules consume Evidence.

Reports explain conclusions derived from Evidence.

Knowledge, Version Profiles, and future capabilities can participate without requiring direct knowledge of one another.

This decision significantly reduced coupling across the repository and established a stable integration boundary between independent subsystems.

---

## Collection Was Separated from Assessment

Collection is intentionally isolated from assessment logic.

Collection answers one question:

"What observable facts exist?"

Assessment answers a different question:

"What do those facts mean?"

Keeping these responsibilities separate preserves explainability, simplifies testing, and allows new collection strategies to be introduced without affecting Rule evaluation.

This separation also prevents business logic from becoming embedded inside collectors.

---

## Archive Reading and Evidence Generation Are Independent Responsibilities

Sprint 4 deliberately divided archive processing into two independent capabilities.

ArchiveReader performs deterministic archive traversal and normalization.

ArchiveEvidenceBuilder transforms normalized archive observations into Runtime Evidence.

This separation was chosen because archive traversal and Evidence generation evolve for different reasons.

ArchiveReader changes when archive policy changes.

ArchiveEvidenceBuilder changes when the assessment model evolves.

Keeping these responsibilities independent minimizes coupling and preserves clear ownership boundaries.

---

## Policies Must Be Explicit

Several implementation decisions demonstrated that behavior should never depend solely upon the underlying library being used.

Examples include:

- duplicate handling
- traversal behavior
- metadata generation
- resource limits

Rather than allowing implementation libraries to define platform behavior implicitly, modIQ exposes these behaviors as explicit architectural policies.

Doing so improves determinism, makes behavior observable during testing, and documents the platform's intended semantics independently of third-party implementations.

---

## Capability Before Abstraction Was Repeatedly Validated

Multiple opportunities arose to introduce generalized interfaces before multiple implementations existed.

These opportunities were intentionally declined.

Examples include:

- collector abstractions
- dispatcher layers
- Rule interfaces beyond demonstrated need
- generalized routing infrastructure

Instead, the repository consistently implemented concrete capabilities first.

Only after implementation demonstrates stable patterns should broader abstractions be considered.

This philosophy has repeatedly reduced unnecessary complexity while preserving future flexibility.

---

## Incremental Capability Reduced Architectural Risk

Sprint 4 intentionally avoided implementing archive collection as one large feature.

Instead, the work was divided into independently reviewable phases:

Foundation

↓

Evidence

↓

Policy

↓

Integration

Each phase produced a complete capability with clear ownership and deterministic behavior.

This sequencing made architectural review significantly easier and ensured governance remained synchronized with implementation.

Future major capabilities should follow the same incremental approach whenever practical.

---

## Governance Guided Implementation Rather Than Following It

Throughout the project, governance decisions intentionally preceded implementation.

Architecture Reviews established direction.

Governance resolved uncertainty.

Implementation followed approved boundaries.

This ordering prevented implementation from becoming the primary driver of architecture and ensured that repository evolution remained deliberate rather than reactive.

This principle should continue guiding future development as the platform grows.

---

# Repository Maturity Assessment

modIQ has transitioned from an architecture-first project into an implementation-first project.

Earlier engineering efforts focused primarily on establishing architectural boundaries, governance processes, documentation, and repository structure. Those foundational decisions now provide a stable framework that allows implementation to proceed incrementally without requiring continuous architectural redesign.

The repository has reached a level of maturity where architecture is no longer the primary source of project risk. Instead, the principal challenge is preserving architectural consistency as implementation expands across additional platform capabilities.

This transition represents a significant milestone in the evolution of the project.

---

## Architecture

The core architecture is considered stable.

Major subsystem boundaries have been established and validated through both governance and implementation.

Current architectural responsibilities are clearly separated between:

- Runtime
- Collection
- Rules
- Reporting
- Knowledge
- Versioning
- Assessment orchestration

Future work is expected to extend these subsystems rather than fundamentally reorganize them.

Architectural redesign should be considered exceptional rather than expected.

---

## Governance

Governance has matured alongside implementation.

Earlier project phases established governance as a mechanism for resolving architectural uncertainty.

Current governance activity is increasingly focused on policy decisions associated with new capabilities rather than defining fundamental repository structure.

Governance should continue evolving in support of implementation rather than expanding independently of it.

---

## Engineering

Implementation is proceeding through incremental capability-based milestones.

Recent implementation work has demonstrated that small, independently reviewable engineering phases provide predictable progress while minimizing architectural risk.

This engineering model should continue for future capabilities.

Large implementation efforts should remain the exception.

---

## Documentation

Repository documentation has reached a mature state.

Architecture specifications, ADRs, governance records, and engineering documentation now serve distinct purposes with minimal overlap.

Future documentation should emphasize maintaining consistency rather than increasing volume.

Documentation should continue to describe implemented architecture rather than anticipated architecture.

---

## Repository Health

The repository is currently considered healthy.

Architectural ownership is explicit.

Subsystem boundaries are respected.

Implementation remains aligned with approved governance.

No significant architectural debt has been intentionally introduced during recent implementation work.

Future Technical Directors should prioritize preserving this state over pursuing unnecessary architectural optimization.

---

## Current Development Focus

The project has entered the implementation phase of Evidence Collection.

The remaining work within this initiative is no longer architectural in nature.

Instead, it consists primarily of implementing approved policies, integrating completed capabilities into the assessment pipeline, and validating deterministic behavior under increasingly realistic assessment scenarios.

The architecture supporting this work is considered sufficiently mature to support continued implementation without significant structural change.

---

# Current Priorities & Future Direction

The architecture of modIQ has reached a level of maturity where future engineering effort should focus on expanding platform capability rather than redesigning established subsystems.

The Technical Director's primary responsibility is no longer creating architecture.

It is preserving architectural consistency while implementation continues to mature.

---

## Immediate Objective

The repository is currently positioned to continue Sprint 4.

The immediate architectural objective is the completion of the Archive Collection implementation.

The remaining implementation work is expected to build upon the existing architecture rather than modify it.

The next engineering milestone should complete the remaining archive collection policies before integrating the completed capability into the assessment pipeline.

Future implementation should continue following the established incremental engineering model:

Foundation

↓

Evidence

↓

Policy

↓

Integration

This sequencing has proven successful throughout Sprint 4 and should remain the preferred approach for future subsystem development.

---

## Technical Director Priorities

Future Technical Directors should focus on preserving architectural integrity rather than accelerating implementation.

Primary responsibilities include:

- reviewing implementation against approved architecture
- ensuring governance remains synchronized with implementation
- preventing responsibility from crossing subsystem boundaries
- evaluating proposals for new abstractions
- protecting deterministic behavior
- maintaining documentation consistency

Implementation speed should never take precedence over architectural quality.

---

## Architectural Risks

At the current stage of development, the greatest architectural risks are no longer associated with missing functionality.

Instead, they arise from gradual erosion of established design principles.

Examples include:

- expanding responsibilities across subsystem boundaries
- introducing abstractions before multiple implementations justify them
- allowing implementation convenience to replace deterministic behavior
- duplicating orchestration outside AssessmentService
- introducing undocumented architectural behavior

Future Technical Directors should actively review implementation for these patterns.

Architectural drift is significantly more difficult to reverse than missing functionality.

---

## Future Evolution

The existing architecture is expected to support continued expansion across multiple implementation phases.

Future work will introduce additional collection capabilities, additional assessment rules, Knowledge integration, Version Profile integration, reporting enhancements, and eventually production application features.

These additions should extend the existing architecture rather than redefine it.

When implementation reveals legitimate architectural limitations, changes should proceed through the established Architecture Review and Governance process.

The preferred solution is evolution through evidence rather than redesign through speculation.

---

## Success Criteria

The long-term success of modIQ should not be measured by repository size or feature count.

Instead, success should be evaluated by the project's ability to consistently produce:

- deterministic assessments
- explainable conclusions
- evidence-based findings
- maintainable architecture
- predictable engineering evolution

Every architectural decision should strengthen one or more of these characteristics.

If a proposed change weakens them, it should be reconsidered before implementation proceeds.

---

## Final Guidance

Protect the architecture.

Allow implementation to validate architectural assumptions.

Accept architectural change only when implementation evidence demonstrates that change is necessary.

Prefer clarity over cleverness.

Prefer simplicity over flexibility.

Prefer deterministic behavior over convenience.

The objective is not to build the most sophisticated assessment platform.

The objective is to build the most trustworthy one.

---

This document serves as the canonical architectural continuity document for modIQ.

It preserves the architectural intent, engineering philosophy, governance rationale, and Technical Director guidance necessary to continue the project without loss of context.

Implementation details, repository status, test counts, and engineering progress are maintained separately in the Lead Engineer Handoff.

Together, these documents provide complete continuity for future Technical Director and Lead Engineer sessions.
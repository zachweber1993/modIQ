# Frontend Architecture

> **The authoritative specification defining the architecture of modIQ's production interaction layer — the boundary between the deterministic engine and the consumer that presents it, and the responsibilities that follow from that boundary.**

---

| Property | Value |
|----------|-------|
| **Document** | FrontendArchitecture.md |
| **Version** | 1.0 |
| **Status** | Approved — not yet Frozen |
| **Project** | modIQ |
| **Governed By** | `Architecture.md` (this document is a Governed Specification of it) |
| **Owner** | Zach Weber |
| **Created** | 2026-08-01 |
| **Last Updated** | 2026-08-01 |

---

# Purpose

This document defines the architecture of modIQ's production interaction layer — the consumer application through which a user experiences an Assessment.

It describes the structural boundary between this layer and the deterministic engine, the responsibilities this layer owns as a direct consequence of that boundary, and the responsibilities it must never assume.

This document intentionally avoids implementation details, technology selection, and visual design. It defines how the consumer is organized, not how it is built.

---

# Specification Authority

**Authority**

- Vision.md
- Principles.md
- Glossary.md
- ProductSpecification.md
- Architecture.md

If a conflict exists between this document and a higher-level specification, the higher-level specification takes precedence.

**Sibling Governed Specifications**

This document is a Governed Specification of `Architecture.md`, alongside `DataModel.md`, `KnowledgeModel.md`, `RuleEngine.md`, `EvidenceCollection.md`, `EngineAPI.md`, and `VersionProfile.md`. It does not govern, and is not governed by, any of these — it elaborates the one boundary among them that none was written to define: the boundary named in `Architecture.md`'s own Platform Boundaries as "User interfaces."

**Treated as Binding, Not as Authority**

The following bodies of work carry no Documentation Authority ranking but bind this document's content in full. This document implements them; it does not redefine, extend, or reinterpret any of them:

- The Product Design corpus (`docs/product-design/`).
- The Interaction Design corpus (`docs/interaction-design/`).
- The Platform Architecture lineage's experience-layer concepts — Console, Dashboard, Wayfinding, Search, Notifications, Settings (`docs/platform/`).
- The Engineering Alignment Program's Initiative 5 Architectural Resolution.

Where any of the above specifies a behavior, a state, or a concept, this document treats that specification as fixed. Where this document appears to require a change to any of them, that is a defect in this document, to be raised, not resolved locally.

---

# Constitutional Foundation

## The Engine/Consumer Boundary

modIQ's deterministic engine and its production interaction layer are separated by a single, fixed boundary. This document does not establish that boundary — `Architecture.md`'s own Platform Boundaries ("User interfaces") and Dependency Rules ("presentation systems should consume assessment outputs without influencing assessment logic") already named it, and the Engineering Alignment Program's Initiative 5 already settled it as architecture.

The engine owns Assessment execution, Evidence evaluation, Finding and Recommendation generation, and Assessment Report production — exclusively, and without exception. The consumer owns everything required to present assessment output and translate user interaction into engine requests. Neither may assume the other's responsibility.

**This boundary is this document's constitutional foundation.** Every responsibility adopted below exists because, and only to the extent that, it elaborates this boundary. This document's own contribution is not the boundary's existence — that is inherited, fixed, and non-discretionary — but the structural mechanism that makes the boundary enforceable within the consumer's own implementation, and the responsibilities that follow from being the party on this side of it.

## Boundary Enforcement

The consumer must be structured so that code capable of evaluating Evidence, producing a Finding, computing a Recommendation, or mutating Assessment state cannot exist anywhere within it — not as a discipline observed by convention, but as a structural property of how the consumer is organized.

This specification requires:

- **A single, identifiable structural separation** within the consumer, isolating all code that interprets or displays engine output from any code capable of reaching back into assessment logic. This specification requires the separation; it intentionally does not prescribe how it is realized — as a module, package, crate, project, or any other implementation mechanism. "Boundary," throughout this document, names the requirement, not an implementation form.
- **No inference across the boundary.** Code on the presentation side may reorganize, filter, and format output the engine has already produced. It may never branch on, transform, or reinterpret that output in a way that produces a new evaluative conclusion the engine did not itself produce.
- **Every crossing of the boundary is a request answered by a response.** Nothing crosses the boundary as a side effect of presentation activity; the consumer never reaches the engine except by a request it deliberately makes.

This structural separation is the answer to a responsibility no other document in this repository claims: `Architecture.md`'s Dependency Rules state the constraint; this document exists so that constraint can be checked, not merely asserted.

---

# Architectural Principles

## Boundary-Preserving

Every capability this document adopts is organized around preserving the Engine/Consumer Boundary. A capability that could plausibly be satisfied by weakening it is not a legitimate elaboration of this specification.

## Presentation Without Inference

The consumer presents what the engine produces. It does not evaluate, infer, or conclude on the engine's behalf, at any layer, for any reason — including failure (see *Engine Transport Failure Handling*, below).

## Single Continuous Assessment

An Assessment, once its Workspace is entered, is presented as one continuous object examined at varying depth — never as a set of separate destinations. This is not a design preference; it is the frozen behavioral model this document is bound to implement (see *Navigation Realization*, below).

## Read Before Write, Always Through the Engine

Any action a user takes that could affect Assessment state is expressed as a request to the engine. The consumer never represents that effect locally before the engine confirms it.

## Explainable Continuity

Whatever the consumer presents about an Assessment's state — current, still forming, or historical — must be traceable to something the engine actually produced. The consumer introduces no fact of its own about whether, or how well, an Assessment is proceeding.

---

# System Overview

```
                              User
                                │
                                ▼
                    Production Consumer
        ┌───────────────┬───────┴───────┬───────────────┐
        ▼               ▼               ▼               ▼
  Application      Consumer-Owned   Boundary        Identity /
     Shell             State       Enforcement      Session
        │               │               │               │
        └───────────────┴───────┬───────┴───────────────┘
                                 ▼
                            Engine API
                        (request / response)
                                 │
                                 ▼
                        Assessment Service
```

The Production Consumer is a single application organized around the boundary named above. It communicates with the engine exclusively through the Engine API's already-defined conceptual service contract (`EngineAPI.md`) — this document does not define, and does not require, any interface beyond what `EngineAPI.md` already contemplates as "future implementation phases."

---

# Core Responsibilities

## Application Shell and Region Composition

**Adopted.** The consumer is composed of interaction regions — Console, Dashboard, Workspace, Settings — exactly as Platform Architecture already names and structurally relates them (`DashboardAndConsole.md`). This document does not introduce, rename, or redefine any region; it owns only their composition into one running application, organized to preserve the boundary above.

Composition below the region level — how a region's own content is internally decomposed — is not a constitutional concern of this document (see *Constitutional Exclusions*).

## Consumer-Owned State

**Adopted.** Presentation state, interaction state, and navigation state are owned by the consumer, in full, as a direct and non-discretionary consequence of Initiative 5's Architectural Resolution. None of the three has any engine-side representation, and none requires one.

## Workspace Ownership

**Adopted in principle.** Workspace state — which of Intake, Assessing, or Reviewing is current for a given Assessment Subject — is owned by the consumer. This document adopts that ownership outright. It does not adopt, and does not attempt to design, the mechanism by which Assessing or Reviewing is derived from the engine's own execution state — that mechanism is reserved (see *Reserved Responsibilities*).

## Navigation Realization

**Adopted with Constraint.** Movement within an Assessment changes scope, never identity: there is only ever one object — the Assessment itself, currently Reviewing — examined at a narrower or wider resolution. This document is bound, without exception, to the frozen model `NAVIGATION_AND_WORKSPACE_BEHAVIOR.md` already establishes:

- **No route-per-view model.** Overview, Finding, Recommendation, and Evidence are not separate destinations. Nothing in the consumer's structure may treat them as pages a user is relocated between.
- **Single-step locality.** Every change of focus moves exactly one level of scope at a time.
- **Symmetric reversal.** Widening focus back out uses the same mechanism as narrowing it in, inverted — never a separate "return" behavior.

Movement across Organizations, Projects, and Assessments (Wayfinding) is realized as Platform Architecture already defines it (`WayfindingAndSearch.md`), up to the Workspace boundary; this document does not redefine Wayfinding's own scope.

## Workspace Realization

**Adopted with Constraint.** The Workspace is realized as a single, persistent object per Assessment Subject, with three presentation targets — Intake, Assessing, Reviewing — exactly as `WORKSPACE_EVOLUTION.md` specifies them, and with the Assessment Overview as the mandatory entry point into Reviewing.

This document adopts the structure only. It does not adopt, and does not design, any mechanism for the Overview or Workspace to update progressively during Assessing, or to reflect material submitted after Reviewing has begun — both are reserved (see *Reserved Responsibilities*).

## Request/Response Mechanism

**Adopted with Constraint.** The consumer crosses the Engine/Consumer Boundary exclusively through request/response, per Initiative 5's own adopted baseline. This document binds every such exchange to a constraint already adopted onto `EngineAPI.md` itself:

- **Completion is recognized by absence, never by announcement.** The engine will never emit an explicit "done" signal shaped like a percentage or time estimate. The consumer's own mechanism for recognizing an Assessment's progress or completion must be built to infer it from the absence of remaining not-yet-final content, not to wait for a signal that will not arrive.

The concrete shape of the request/response contract — its payload, its fields — is not adopted here and is reserved (see *Reserved Responsibilities*).

## Identity and Session Mechanism

**Adopted.** The consumer owns the mechanism by which a session recognizes a User and the standing their Membership carries. This document does not define, and does not redefine, User, Membership, Role, or Organization — those remain exactly as `IdentityAndAccess.md` defines them. It owns only the mechanism `IdentityAndAccess.md` itself declines to define.

## Error and Latency Handling

**Adopted with Constraint.** Two classes of condition are already behaviorally specified in full and must be implemented exactly as specified, without redesign:

- **Content-level conditions** — unsupported files, incomplete uploads, corrupted archives, duplicate material — exactly as `ASSESSMENT_INTAKE_AND_UPLOAD.md` §6 specifies.
- **Attention and absence during execution** — a user leaving, returning, or an Assessment completing while unattended — exactly as `ASSESSING_AND_PROGRESSIVE_DISCOVERY.md` §7 specifies, including the explicit absence of any progress bar, percentage, or time estimate.

This document adopts these as implementation obligations. It does not reinterpret, extend, or supply an alternative to either specification.

## Engine Transport Failure Handling

**Adopted.** A failure to reach the engine, a timeout, or a malformed response is a distinct condition from a content-level error above, and is owned by this document because no other document claims it. Such a failure must never be presented in a way that could be mistaken for an assessment outcome — an unreachable engine is not a Finding, an empty result, or a completed Assessment, and must never be represented as any of them.

---

# Dependency Rules

- **Frontend Architecture depends on the Engine API.** `EngineAPI.md` has no dependency, in either direction, on this document.
- **The Product Design and Interaction Design corpora constrain Frontend Architecture's behavior.** Frontend Architecture does not constrain, revise, or narrow either corpus in return.
- **Frontend Architecture must never become a source of truth for Assessment state.** The engine remains the sole source of that state regardless of what the consumer displays, formats, or temporarily holds for presentation.
- State ownership follows directly from the Engine/Consumer Boundary, not from implementation convenience: presentation, interaction, and navigation state are consumer-owned outright; workspace state is consumer-owned in principle, with its derivation mechanism reserved.

Nothing in this document introduces a dependency edge `Architecture.md`'s own Dependency Rules do not already permit.

---

# Reserved Responsibilities

The following are recognized as belonging to this document's eventual scope. None is designed, narrowed, or given a working assumption here. Each remains outside this specification until the named dependency resolves.

**Reserved, pending Initiative 1 (Progressive Execution Observability):**

- The mechanism by which Workspace state (Assessing versus Reviewing) is derived from the engine's own execution-phase signal, and that signal's granularity.
- Any progressive or in-place update of the Assessment Overview during Assessing.
- Any supplementary notification or poll-trigger mechanism beyond the adopted request/response baseline.
- Any consumer-side realization of Notification delivery tied to in-progress Assessment activity.

**Reserved, pending Initiative 2 (Reentrant Assessment Lifecycle):**

- Any presentation of a completed Assessment Report reflecting material submitted after completion, and any consumer-facing representation of Assessment reentrancy. Initiative 2's own Governance Reconciliation found this question presently unreconcilable under current governance; this document does not attempt what that Reconciliation already found it could not resolve.

**Reserved, pending GOV-008 (AssessmentService Public API Evolution):**

- The concrete payload shape of the request/response contract between the consumer and the engine.

No reserved item above is designed, speculated about, or given a temporary shape in this document.

---

# Constitutional Exclusions

This specification, and any document it governs, intentionally excludes:

- **No business logic.** No evaluation of Evidence, generation of a Finding or Recommendation, or any assessment-logic decision may exist anywhere in the consumer.
- **No Assessment mutation.** Assessment state is exclusively engine-owned. The consumer never mutates it directly, under any condition.
- **No Runtime subsystem ownership.** The Rule Engine, Evidence Collection, Knowledge Model, and Version Profiles remain exclusively Runtime Architecture's; this document defines no part of any of them.
- **No Platform domain ownership.** Organization, User, Membership, Project, Upload, Subscription, API Client, and Public Visitor remain exclusively Platform Architecture's. This document defines none of them and introduces no first-class concept in that domain.
- **No Product object ownership.** Finding, Evidence, Recommendation, Assessment Subject, and the Workspace as a concept remain Product Design's. This document implements them; it does not redefine any of them.
- **No Interaction behavior ownership.** Movement semantics, disclosure order, and trust mechanics remain Interaction Design's. This document implements Interaction Design's behavior faithfully; it does not modify, extend, or reinterpret it.
- **No technology or framework selection.** This document selects no implementation technology, language, library, or platform, and no future document governed by it may do so under this document's authority alone.
- **No ownership of internal component decomposition.** How a region's own content is structured below the region level is not a constitutional concern; it is left entirely to ordinary implementation judgment.

**These exclusions are constitutional constraints on every specification and implementation governed by this document. Changes require architectural governance, not implementation discretion.**

---

# Relationship to Other Specifications

This document defines the production interaction layer's architecture.

- `Architecture.md` governs this document and defines the boundary this document elaborates.
- `EngineAPI.md` defines the conceptual service this document's Request/Response Mechanism consumes; this document is the client-side counterpart to the "future implementation phases" `EngineAPI.md` already anticipates.
- Frontend Architecture implements Platform Architecture's interaction concepts (Console, Dashboard, Wayfinding, Search, Settings) but possesses no authority to redefine them.
- Frontend Architecture implements the concepts and behavior Product Design and Interaction Design define but possesses no authority to redefine either.
- The Engineering Alignment Program's Initiative 5 Architectural Resolution is the source of the boundary named in this document's Constitutional Foundation; Frontend Architecture possesses no authority to revisit it.

---

# Document Status

**Current Version:** 1.0

**Status:** Approved. Not yet Frozen. Approval was reached through review during this engineering session; synchronization into the repository on 2026-08-01, alongside a corresponding amendment to `Architecture.md`'s own Governed Specifications list, records that approval — it does not itself constitute it.

This document expresses the Frontend Architecture Architectural Resolution's adopted determinations in full. It introduces no responsibility, constraint, or exclusion beyond what that Resolution already settled.

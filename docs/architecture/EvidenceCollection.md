# Evidence Collection

> **The authoritative specification defining the conceptual Evidence Collection subsystem of the modIQ platform.**

---

| Property | Value |
|----------|-------|
| **Document** | EvidenceCollection.md |
| **Version** | 1.1.0 |
| **Status** | Frozen, amended following GOV-009/GOV-010 resolution |
| **Project** | modIQ |
| **Documentation Release** | 2.1 |
| **Owner** | Zach Weber |
| **Created** | 2026-07-19 |
| **Last Updated** | 2026-07-19 |

---

# Specification Authority

Authority:

- Vision.md
- Principles.md
- Glossary.md
- ProductSpecification.md
- Architecture.md
- DataModel.md

This document governs no other specification. It sits alongside `RuleEngine.md` as a technical-layer specification refining `Architecture.md`.

If a conflict exists between this document and a higher-level specification, the higher-level specification takes precedence.

---

# Purpose

This document defines the conceptual boundary and contract of the modIQ Evidence Collection subsystem: the subsystem responsible for producing real Evidence from an Assessment Subject's actual content.

This specification intentionally excludes implementation details, including programming languages, file formats, archive libraries, parsing technologies, and programming interfaces. Its purpose is to define **what belongs to Evidence Collection and what does not**, independent of how any concrete collector is eventually implemented.

This document establishes the architectural foundation authorized by ADR-0008. It does not authorize implementation; see `GOVERNANCE.md`'s Governance Register (GOV-007).

---

# Relationship to Other Specifications

Evidence Collection produces:

- Evidence, as defined by `DataModel.md`.

Evidence Collection consumes:

- An Assessment Input (defined below), supplied by the application layer through the Engine.
- Assessment Context, where relevant to how content should be interpreted (for example, an eventual Version Profile).

Evidence Collection does not consume Rules, Findings, Recommendations, or Engineering Knowledge, and does not produce them. It has no relationship to `RuleEngine.md` or `KnowledgeModel.md` other than indirectly supplying the Evidence the Rule Engine subsequently evaluates.

---

# Subsystem Ownership

Evidence Collection is a distinct platform subsystem, separate from the Runtime Domain, the Rule Engine, Reporting, and the Assessment Service (ADR-0008).

- Evidence Collection owns the logic that inspects an Assessment Subject's content and produces Evidence from it.
- Evidence Collection does not own Evidence once produced. Ownership of Evidence, like every other runtime entity, belongs to the Assessment aggregate (`DataModel.md`: Ownership), which receives Evidence Collection's output through the Engine.
- Evidence Collection does not own orchestration. It is invoked; it does not decide when it runs or what happens to its output afterward.

---

# Relationship to Existing Subsystems

Evidence Collection is one of five cooperating subsystems in the Assessment pipeline. Each owns a distinct part of the pipeline, and none bypasses another's ownership boundary to reach a result faster:

- **`modiq-runtime`** owns domain entities and aggregate state. It owns `Assessment` and everything `Assessment` owns — Evidence, Findings, Recommendations — once they exist, and is the sole component permitted to mutate them.
- **`modiq-collection`** produces Evidence only. It inspects an Assessment Subject's content and returns Evidence; it does not evaluate what it produces, does not decide what happens to it afterward, and does not touch `Assessment` directly.
- **`modiq-rules`** evaluates collected Evidence. It consumes Evidence already present in an Assessment and produces Findings and Recommendations from it; it does not collect Evidence itself and does not mutate `Assessment` directly.
- **`modiq-report`** transforms assessed results into presentation models. It reads a completed (or in-progress) Assessment's state and produces an `AssessmentReport`; it performs no analysis and does not collect, evaluate, or mutate anything.
- **`modiq-engine`** orchestrates the complete assessment pipeline. It is the only component that calls into more than one of the other four — invoking Evidence Collection, then Rule evaluation, then Reporting, in sequence, and relaying each result into `Assessment` through `Assessment`'s own methods. It implements none of the business logic those four subsystems own.

No subsystem bypasses another subsystem's ownership boundary. Evidence Collection does not evaluate Evidence (that is `modiq-rules`'s boundary), the Rule Engine does not collect Evidence (that is `modiq-collection`'s boundary), no subsystem but `modiq-runtime` mutates `Assessment` directly, and no subsystem but `modiq-engine` invokes more than one of the others. Where one subsystem's output becomes another's input, it passes through `modiq-engine`'s orchestration and `Assessment`'s own methods — never through a direct call from one owning subsystem into another's internal state.

---

# Orchestration Flow

Evidence Collection participates in the Assessment lifecycle as follows, conceptually:

```text
Application supplies an Assessment Input
        │
        ▼
Assessment Service invokes Evidence Collection
        │
        ▼
Evidence Collection inspects the described content
        │
        ▼
Evidence Collection returns Evidence (or reports failure)
        │
        ▼
Assessment Service adds returned Evidence to the Assessment
        │
        ▼
Rule Engine evaluates the Assessment's Evidence (unchanged)
```

Evidence Collection is invoked by the Assessment Service, in the same conceptual position `RuleEngine.md`'s execution lifecycle already reserves for evidence to exist before evaluation begins. Evidence Collection does not call the Rule Engine, does not call Reporting, and does not mutate the Assessment directly — it returns Evidence to its caller, which is responsible for adding it to the Assessment.

---

# Responsibilities

Evidence Collection is responsible for:

- Interpreting an Assessment Input to locate the content to inspect.
- Deterministically inspecting that content.
- Producing Evidence — correctly categorized, described, and located — that reflects what was objectively observed.
- Reporting its own inability to complete collection, distinctly from reporting that nothing relevant was found.

---

# Non-Responsibilities

Evidence Collection is explicitly **not** responsible for:

- **Evaluating Evidence, or producing Findings or Recommendations.** This remains the Rule Engine's exclusive responsibility (`RuleEngine.md`). Evidence Collection observes; it does not conclude.
- **Mutating the Assessment.** Evidence Collection returns Evidence to its caller. Only the Assessment aggregate mutates itself, through its own methods, per `RuntimeInvariants.md` (INV-006, INV-007, INV-009).
- **Forming opinions or subjective judgments.** `DataModel.md`: "Evidence never represents opinion." Evidence Collection's output is factual, not evaluative.
- **Knowing about Rules, Engineering Knowledge, or Recommendations.** Evidence Collection has no relationship to the Knowledge Domain or the Rule Engine beyond supplying Evidence indirectly, through the Assessment.
- **Acquiring the Assessment Input itself.** How an application (Sandbox, CLI, or a future application) obtains a path, an uploaded archive, or any other description of where content lives is an application-layer concern, outside Evidence Collection's boundary. Evidence Collection consumes an Assessment Input; it never creates or reinterprets one, and it does not solicit one from a user.
- **Persistence.** Evidence Collection has no relationship to the Storage Layer.
- **Interpreting game-version-specific behavior on its own.** Where interpretation depends on a Farming Simulator version, that context is supplied via Assessment Context / a future Version Profile (`Architecture.md`: Version Isolation), not decided internally by Evidence Collection.

---

# Assessment Input

Assessment Input is the value an application supplies to identify what Evidence Collection should inspect. Its authoritative definition, ownership, and scope are resolved here (GOV-009), for the filesystem case, following `PROPOSAL_FILESYSTEM_COLLECTION.md`.

("Input Descriptor" was the Sprint 3 Phase 3/4 placeholder term for this same concept; Sprint 3 Phase 5 renamed the corresponding Rust types — `InputDescriptor` to `AssessmentInput`, `InputDescriptorError` to `AssessmentInputError` — so implementation and this specification now use the same vocabulary.)

Assessment Input:

- Represents a stable reference to a filesystem object at the moment collection begins.
- Is supplied by the application layer (Sandbox, CLI, or a future application), through the Assessment Service.
- Is opaque to every subsystem except Evidence Collection and whichever application layer produced it — the Rule Engine, Reporting, and the Runtime Domain have no relationship to it at all.
- Identifies *where* to look, not *what* will be found there. Evidence Collection determines what the location actually contains; the Assessment Input itself carries no assumptions about it.

**Valid Assessment Input, for the filesystem case:**

- A **file** is a valid Assessment Input — for example, a mod distributed as a single archive, not yet extracted.
- A **directory** is a valid Assessment Input — for example, an already-unpacked mod folder.
- A **non-existent path is not a valid Assessment Input.** The referenced location must exist at the moment collection begins; its absence is an Inaccessible Input outcome (see Collection Outcomes), not a malformed Assessment Input.

**Consumption, not creation.** The Collection subsystem consumes an Assessment Input; it never creates or reinterprets one. Acquiring it remains entirely an application-layer concern (see Non-Responsibilities, above) — Evidence Collection's role begins only once a candidate Assessment Input already exists.

**Future input types are intentionally out of scope.** Archives treated as their own input source (as opposed to a file discovered by traversal), remote sources, virtual sources, and any other non-filesystem origin are deliberately not addressed by this resolution. This document does not claim Assessment Input's shape here is final for every future collector — only that it is sufficient and correct for the filesystem case this milestone addresses.

See Symbolic Link Policy (Phase 5), below, for the one Assessment Input traversal boundary specific to the first real collector.

---

# Collector Contract

A Collector is the conceptual unit of work within Evidence Collection responsible for inspecting one kind of content (for example, an archive's structure, a structured-text file, a script file) and producing Evidence from it. This section describes the contract every Collector is expected to honor, at the architectural level. It does not define how multiple Collectors are composed, registered, or dispatched — that remains an open implementation question, deliberately deferred until a second concrete Collector exists to test any such mechanism against (consistent with the Technical Director's principle that a capability should justify an abstraction, not the reverse).

## Inputs

A Collector receives:

- An Assessment Input (or the portion of one relevant to its kind of inspection).
- Relevant Assessment Context, where applicable.

A Collector receives nothing else. It does not receive Rules, Engineering Knowledge, or any other Assessment state.

## Outputs

A Collector produces:

- Zero or more Evidence items, each correctly categorized, described, and — where applicable — located, per `DataModel.md`'s definition of Evidence.
- An indication of whether collection completed successfully, and if not, why.

A Collector does not produce Findings, Recommendations, or any conclusion about what the Evidence means.

## Guarantees

A Collector guarantees:

- **Determinism.** Given identical input content, a Collector produces an identical set of Evidence, every time. This includes imposing a stable, explicit ordering over whatever it inspects, since the underlying content (files within an archive, for example) may not have an inherently stable order of its own.
- **Factual accuracy.** Evidence a Collector produces reflects what was objectively present in the inspected content, not an inference, guess, or opinion about it.
- **Boundary respect.** A Collector does not reach outside its own inspection responsibility to evaluate, recommend, mutate Assessment state, or persist anything.

## Non-Responsibilities

Restated at the Collector level, consistent with Evidence Collection's own Non-Responsibilities above: a Collector does not evaluate, does not mutate the Assessment, does not form opinions, does not know about Rules or Knowledge, does not acquire its own Assessment Input, and does not persist state.

## Determinism Expectations

Collection is the first subsystem in the platform's pipeline that touches content outside the platform's own control (files on disk, archive contents, external text). Unlike Rule evaluation — a pure function over already-valid, in-memory Evidence — collection can genuinely fail for reasons entirely outside the Assessment's own state (a missing file, a corrupted archive, a permissions error). A Collector is expected to distinguish clearly between:

- **Legitimate absence** — the inspected content exists and was fully inspected, and simply contained nothing relevant to that Collector's concern. This is a successful, empty result, not a failure — the same distinction `RuleEngine.md`'s execution model already draws between "evaluated and found nothing" and "could not evaluate."
- **Collection failure** — the Collector could not complete its inspection at all. This must be represented distinctly from legitimate absence, so that a caller (and eventually, a user) can tell "there was nothing to find" apart from "something prevented us from looking."

The architectural categories of collection failure, and how they are distinguished from legitimate absence, are resolved below (Collection Outcomes). This document still does not fix the concrete representation mechanism (an error type, a result variant, or otherwise) — that remains an implementation detail.

---

# Collection Outcomes

Resolved (GOV-010), following `PROPOSAL_FILESYSTEM_COLLECTION.md`'s architecture. Every collection attempt resolves to exactly one of four architectural outcomes:

## Invalid Input

The Assessment Input itself is malformed or empty, before any inspection is attempted. Collection never begins.

## Inaccessible Input

The Assessment Input is well-formed, but the location it names cannot be reached: it does not exist, access is denied, or it sits on storage that is not currently available. Collection aborts.

## Unsupported Input

The location is reachable but is not a supported kind of thing for this Collector — for example, a device file or named pipe rather than a regular file or directory. Distinct from Inaccessible Input: the location is right there, it simply isn't a supported shape. Collection aborts.

## Empty Collection

The Assessment Input is valid, reachable, and a supported kind of location, but structurally contains nothing (an empty directory, for instance). Collection **succeeds**, producing zero Evidence. This is explicitly **not** an error — it is a successful observation reflecting a genuine fact about the subject, not a failure of collection itself.

## Collection Success vs. Assessment Success

Only Empty Collection represents successful collection with no Evidence produced; the other three outcomes represent collection not completing at all. In no case does a successful collection outcome (including Empty Collection) imply anything about the eventual Assessment's outcome: Collection succeeding means only that Evidence Collection completed its own responsibility. Whether the resulting Evidence — even zero Evidence — supports any Finding is entirely the Rule Engine's later, separate concern; Collection has no visibility into, and no responsibility for, whether an Assessment is meaningful in any product sense.

**Which failures belong to Collection.** Invalid Input, Inaccessible Input, and Unsupported Input are Collection's own concern — each occurs before or during the act of gathering Evidence, and only Collection has the context, at the moment it happens, to distinguish one from another. None of these should ever surface as a Finding: a collection failure is not an assessment conclusion, and treating it as one would compromise the Evidence Collection / Rule Engine producer-consumer boundary this specification exists to preserve.

**What does not belong to Collection.** Judging whether a successfully discovered structure is adequate for a valid mod belongs to the Rule Engine, evaluating Evidence Collection already produced. Deciding what happens to the rest of an Assessment when a collection failure occurs is an Engine-orchestration policy question — see Collection Atomicity, below, for the Phase 5 answer.

---

# Collection Atomicity (Phase 5)

For the first real collector (Sprint 3 Phase 5, filesystem collection), collection is atomic: it either completes successfully — including as an Empty Collection outcome — or the Assessment terminates. There is no partial Assessment, no partial Evidence, and no partial Report when collection does not complete (Invalid Input, Inaccessible Input, or Unsupported Input).

This is an intentional Phase 5 scope decision, not a permanent platform limitation. A future collector or phase may revisit it — for example, incremental collection across multiple sources, where one source's failure need not prevent Evidence already gathered from others from supporting a Report. Atomicity is what Phase 5 rules out; it is not a requirement the platform architecture imposes forever.

No change to `RuntimeInvariants.md` is required by this policy: no Runtime aggregate invariant governs it. A collection failure means the Assessment's lifecycle simply never progresses far enough to produce a Report — existing invariants (INV-002, Evidence may only be added before rule evaluation begins; INV-003, Evidence becomes immutable once rule evaluation starts) already accommodate this without modification. Atomicity is Engine-orchestration policy, not Runtime state.

---

# Symbolic Link Policy (Phase 5)

For the first real collector, symbolic links are intentionally not traversed. A discovered path that is a symbolic link is not followed to its target.

This avoids three concerns at once, for this milestone:

- **Platform-specific behavior.** Symbolic link semantics differ across operating systems and filesystems; not following them avoids that variance entirely.
- **Traversal cycles.** A link can point back into a location already being traversed, risking unbounded or repeated work.
- **Escaping the Assessment Input's boundary.** A link can point outside the location Collection was actually given — which Assessment Input, above, already excludes as in scope.

This is a Phase 5 architectural boundary, not a permanent one. Following symbolic links under controlled conditions may be considered as a separate, future capability.

---

# Future Evolution

This boundary is intentionally narrow so that concrete Collectors — for example, archive traversal, structured-text inspection, script inspection, manifest inspection, or dependency inspection — can each be added as independent, additive work behind it, without any of them requiring a change to the Rule Engine, the Runtime Domain, or Reporting, and without requiring changes to each other. This mirrors `Architecture.md`'s existing Extensibility principle ("platform should evolve through extension rather than modification") applied to a second axis of extension, alongside Rules.

Future Version Profile integration (`Architecture.md`: Version Isolation) is expected to inform *which* Collectors run, or how they interpret content, via Assessment Context — without requiring a change to this boundary.

---

# Document Status

**Current Version:** 1.1.0

**Status:** Frozen as part of Documentation Release 2.1; amended following Technical Director approval of `PROPOSAL_FILESYSTEM_COLLECTION.md` to resolve GOV-009 (Assessment Input Ownership) and GOV-010 (Collection Error Model) for the filesystem case, and to record the Phase 5 Collection Atomicity and Symbolic Link Policy decisions. Authorized at the architecture level by ADR-0008; implementation of a real collector is not yet authorized by this document — that remains a separate implementation-readiness decision.

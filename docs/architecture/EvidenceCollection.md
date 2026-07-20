# Evidence Collection

> **The authoritative specification defining the conceptual Evidence Collection subsystem of the modIQ platform.**

---

| Property | Value |
|----------|-------|
| **Document** | EvidenceCollection.md |
| **Version** | 1.0.0 |
| **Status** | Frozen |
| **Project** | modIQ |
| **Documentation Release** | 2.1 (pending) |
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

- An Input Descriptor (defined below), supplied by the application layer through the Engine.
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
Application supplies an Input Descriptor
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

- Interpreting an Input Descriptor to locate the content to inspect.
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
- **Acquiring the Input Descriptor itself.** How an application (Sandbox, CLI, or a future application) obtains a path, an uploaded archive, or any other description of where content lives is an application-layer concern, outside Evidence Collection's boundary. Evidence Collection receives a descriptor; it does not solicit one from a user.
- **Persistence.** Evidence Collection has no relationship to the Storage Layer.
- **Interpreting game-version-specific behavior on its own.** Where interpretation depends on a Farming Simulator version, that context is supplied via Assessment Context / a future Version Profile (`Architecture.md`: Version Isolation), not decided internally by Evidence Collection.

---

# The Input Descriptor

An Input Descriptor is the conceptual value an application supplies to identify what Evidence Collection should inspect — for example, a location on a filesystem, though this specification deliberately does not fix its exact form. The Input Descriptor:

- Is supplied by the application layer (Sandbox, CLI, or a future application), through the Assessment Service.
- Is opaque to every subsystem except Evidence Collection and whichever application layer produced it — the Rule Engine, Reporting, and the Runtime Domain have no relationship to it at all.
- Identifies *where* to look, not *what* will be found there. Evidence Collection determines what the descriptor's content actually contains; the descriptor itself carries no assumptions about it.

Ownership of the Input Descriptor concept — which specification defines it authoritatively, and what content it eventually carries — is an open governance question (GOV-009), not resolved by this document beyond the definition above.

---

# Collector Contract

A Collector is the conceptual unit of work within Evidence Collection responsible for inspecting one kind of content (for example, an archive's structure, a structured-text file, a script file) and producing Evidence from it. This section describes the contract every Collector is expected to honor, at the architectural level. It does not define how multiple Collectors are composed, registered, or dispatched — that remains an open implementation question, deliberately deferred until a second concrete Collector exists to test any such mechanism against (consistent with the Technical Director's principle that a capability should justify an abstraction, not the reverse).

## Inputs

A Collector receives:

- An Input Descriptor (or the portion of one relevant to its kind of inspection).
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

Restated at the Collector level, consistent with Evidence Collection's own Non-Responsibilities above: a Collector does not evaluate, does not mutate the Assessment, does not form opinions, does not know about Rules or Knowledge, does not acquire its own Input Descriptor, and does not persist state.

## Determinism Expectations

Collection is the first subsystem in the platform's pipeline that touches content outside the platform's own control (files on disk, archive contents, external text). Unlike Rule evaluation — a pure function over already-valid, in-memory Evidence — collection can genuinely fail for reasons entirely outside the Assessment's own state (a missing file, a corrupted archive, a permissions error). A Collector is expected to distinguish clearly between:

- **Legitimate absence** — the inspected content exists and was fully inspected, and simply contained nothing relevant to that Collector's concern. This is a successful, empty result, not a failure — the same distinction `RuleEngine.md`'s execution model already draws between "evaluated and found nothing" and "could not evaluate."
- **Collection failure** — the Collector could not complete its inspection at all. This must be represented distinctly from legitimate absence, so that a caller (and eventually, a user) can tell "there was nothing to find" apart from "something prevented us from looking."

How collection failure is represented — as part of the Evidence returned, as a distinct error value, or some other mechanism — is an open governance question (GOV-010), not resolved by this document.

---

# Future Evolution

This boundary is intentionally narrow so that concrete Collectors — for example, archive traversal, structured-text inspection, script inspection, manifest inspection, or dependency inspection — can each be added as independent, additive work behind it, without any of them requiring a change to the Rule Engine, the Runtime Domain, or Reporting, and without requiring changes to each other. This mirrors `Architecture.md`'s existing Extensibility principle ("platform should evolve through extension rather than modification") applied to a second axis of extension, alongside Rules.

Future Version Profile integration (`Architecture.md`: Version Isolation) is expected to inform *which* Collectors run, or how they interpret content, via Assessment Context — without requiring a change to this boundary.

---

# Document Status

**Current Version:** 1.0.0

**Status:** Frozen as part of Documentation Release 2.1. Authorized at the architecture level by ADR-0008; implementation is not authorized (see `GOVERNANCE.md`, GOV-007).

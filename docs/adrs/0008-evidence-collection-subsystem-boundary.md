# ADR-0008: Evidence Collection Subsystem Boundary

| Property | Value |
|----------|-------|
| **ADR** | 0008 |
| **Title** | Evidence Collection Subsystem Boundary |
| **Status** | Accepted (architecture only — implementation not authorized; see Governance Register GOV-007) |
| **Project** | modIQ |
| **Date** | 2026-07-19 |

---

# Context

Every Evidence value produced by the platform to date has been synthetic: `modiq-rules::RuleEngine` correctly only ever consumes Evidence (`RuleEngine.md`), and `apps/sandbox` constructs a single hardcoded, deterministic Evidence item as a stand-in for real inspection. Sprint 3 Phase 1's own architectural review — the review that rejected folding Evidence construction into the Rule Engine — identified this as more than a missing feature: real Evidence Collection (inspecting a mod's actual files) has no owning crate, service, or ADR anywhere in the platform.

`PROPOSAL_EVIDENCE_COLLECTION_BOUNDARY.md` (docs/engineering/) was prepared as a design-only Architecture Review proposal answering where this responsibility belongs and what its boundary should be, without proposing any implementation. The Technical Director approved that proposal's architectural direction. This ADR records the resulting decision as a durable architectural artifact, per `docs/adrs/README.md`'s guidance that decisions altering ownership boundaries and affecting multiple platform components require an ADR rather than living only in a proposal document.

This ADR describes the boundary and its rationale. It does not specify Rust types, traits, or any other implementation detail — those remain open, deliberately, until implementation is separately authorized.

---

# Decision

## Evidence Collection is a separate subsystem

Evidence Collection — inspecting an Assessment Subject's actual content and producing Evidence from it — is recognized as its own architectural subsystem, distinct from Runtime, Rule Engine, Reporting, and Engine. It is not a responsibility of any existing subsystem, and it is not folded into one now.

**Rationale:** each existing subsystem's charter (`GOVERNANCE.md`, Crate Boundary Rules) already excludes this responsibility on its own terms — Runtime owns state, not external inspection; the Rule Engine evaluates Evidence, it does not produce it (`RuleEngine.md`: "consumes... Evidence," "produces... Findings, Recommendations"); Reporting reflects state, it performs no analysis. Evidence Collection is genuinely new work with no existing home, not a gap in an existing subsystem's implementation.

## Evidence Collection belongs outside Runtime

Evidence Collection is not part of `modiq-runtime`.

**Rationale:** `GOVERNANCE.md`'s Runtime Domain boundary states Runtime "must never... perform orchestration," and inspecting real files (archive traversal, structured-text parsing) is exactly the kind of specialized, potentially-fallible, side-effecting logic Runtime has never owned. Runtime owns state and identity; introducing file-inspection logic would tie Runtime's dependency graph to parsing concerns for no architectural benefit, and would blur the boundary ADR-0007 (Runtime Entity Design Pattern) took care to keep narrow: Runtime entities hold data, not behavior that reaches outside the aggregate.

## Orchestration belongs to `modiq-engine`

Evidence Collection is invoked by the Engine, not by the Runtime `Assessment` aggregate itself, and not autonomously by Evidence Collection.

**Rationale:** this mirrors the precedent already established, reviewed, and shipped for Rule execution — `AssessmentService::execute` invokes `RuleEngine::evaluate` and then calls `Assessment::add_finding`/`add_recommendation`; the `Assessment` aggregate never reaches out to invoke the Rule Engine itself. Evidence Collection follows the identical shape: the Engine invokes the Collection subsystem, receives Evidence back, and calls `Assessment::add_evidence` for each item. The aggregate remains a passive receiver of state; it is never an active orchestrator. This is consistent with `GOVERNANCE.md`'s Engine boundary ("owns orchestration, execution flow, composition of platform services") and preserves `DataModel.md`'s Assessment-Centric and Immutability principles unchanged.

## Rules continue to evaluate Evidence rather than produce it

This ADR makes no change to `modiq-rules`. The Rule Engine continues to consume Evidence exclusively; it does not construct, collect, or otherwise produce it.

**Rationale:** `RuleEngine.md` is Frozen and already states this boundary explicitly. Sprint 3 Phase 1's architectural review already rejected the alternative (having the Rule Engine construct Evidence) on exactly these grounds. Introducing a separate Evidence Collection subsystem does not reopen that question; it resolves the gap that review left open — *something* has to produce real Evidence, and this ADR establishes that it is Evidence Collection, not the Rule Engine, preserving the producer/consumer boundary between the two.

## Architectural dependency direction

Evidence Collection depends on `modiq-runtime` only (to construct `Evidence`/`EvidenceCategory` values). Nothing depends on Evidence Collection except `modiq-engine`. No dependency exists, in either direction, between Evidence Collection and `modiq-rules`, `modiq-report`, or `modiq-knowledge`.

This places Evidence Collection at the same tier as `modiq-rules` and `modiq-report` in `CrateRoadmap.md`'s dependency hierarchy — a peer, not a parent or child of either — preserving the platform's existing strictly-downward dependency rule (`modiq-runtime` remains the leaf; no circular dependency is introduced).

## Alternatives considered and rejected

- **Inside `modiq-runtime`.** Rejected: violates Runtime's "never perform orchestration" boundary and would tie Runtime to parsing concerns it has no architectural reason to own.
- **Inside `modiq-engine`.** Rejected: Engine's charter explicitly excludes implementing business rules (`GOVERNANCE.md`); file inspection and Evidence classification is specialized business logic, of the same kind that justified keeping Rule evaluation in its own crate rather than folding it into Engine.
- **Inside `modiq-knowledge`.** Rejected on domain grounds: Evidence is per-Assessment runtime data; the Knowledge Domain's entire charter is reusable knowledge that exists independently of any individual Assessment (`Architecture.md`, Domain Separation principle). These are different domains.
- **A new, separate subsystem** (adopted): matches the precedent already set by `modiq-rules` and `modiq-report` — narrow, single-responsibility, depending only on `modiq-runtime`, composed by `modiq-engine`.

---

# Consequences

Benefits:

- Evidence Collection gets a stable architectural home before any concrete collector (ZIP, XML, Lua, manifest, dependency inspection) is implemented, so each can be added later as additive work behind an already-settled boundary rather than each reopening ownership questions.
- The Rule Engine's producer/consumer boundary, already fought for once in Sprint 3 Phase 1, is reinforced rather than revisited.
- The platform's dependency hierarchy gains a new peer crate without any new circular or upward dependency.

Trade-offs:

- `AssessmentService::execute` (or an equivalent Engine entry point) will eventually require a breaking public API change to accept an input descriptor it does not accept today. This consequence is significant enough to warrant its own ADR-0009 rather than being decided as a detail of this one.
- A new crate means new governance surface area: naming, exact dependency wiring, and crate-level documentation all remain open, tracked in the Governance Register (GOV-007 through GOV-010) rather than decided here.
- No internal dispatch mechanism for multiple future collectors is decided by this ADR, deliberately — per the Technical Director's stated principle that a capability should justify an abstraction rather than the reverse, that question is left open until a second concrete collector exists to test it against.

---

# Relationship to Other Specifications

This decision is reflected in:

- `docs/architecture/EvidenceCollection.md` (new) — the conceptual subsystem specification and Collector Contract this ADR authorizes.
- `Architecture.md` — amended to name Evidence Collection as a Core Platform Component and to correct its position in the Assessment Lifecycle diagram (see the accompanying Documentation Release 2.1 changes).
- `DataModel.md`, `RuleEngine.md` — cross-referenced, unchanged in substance.
- `GOVERNANCE.md` — new Crate Boundary Rules entry for Evidence Collection; new Governance Register items GOV-007 through GOV-010.
- ADR-0003 (Assessment Aggregate Root), ADR-0007 (Runtime Entity Design Pattern) — this decision preserves both without modification.
- ADR-0009 (companion ADR, `AssessmentService` public API evolution).

---

# Status

Accepted as an architectural decision.

Implementation of Evidence Collection (a new crate, any concrete collector, or any change to `modiq-engine`'s orchestration code) is **not** authorized by this ADR and requires a separate governance decision per GOV-007, following Documentation Release 2.1.

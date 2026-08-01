# Knowledge Feedback Loop — Capability Definition

| Property | Value |
|---|---|
| **Document** | CAPABILITY_DEFINITION_KNOWLEDGE_FEEDBACK_LOOP.md |
| **Project** | modIQ |
| **Purpose** | Capability Definition for "MKB Accumulation from Real Assessments," resumed under the Adopted Architectural Constraint recorded in `ARCHITECTURE_EVALUATION_KNOWLEDGE_MODEL_CONSISTENCY.md` §6 |
| **Prepared by** | Lead Engineer (Sonnet 5), on `feature/runtime-implementation` |
| **Repository baseline** | `feature/runtime-implementation`, HEAD `73f7f72` |
| **Status** | **Suspended, by explicit Chief Architect direction, following review of this Capability Definition.** Not rejected, not withdrawn — this capability's own analysis and constraint (Sections 4–5) remain valid and are not superseded. A narrower, separately-scoped capability, **Historical Assessment Analysis** (`CAPABILITY_DEFINITION_HISTORICAL_ASSESSMENT_ANALYSIS.md`), proceeds first instead, limited to read-only Storage-domain analysis with no Knowledge Domain involvement at all. This document may be resumed later if a concrete need for human-curated Knowledge authoring, informed by accumulated patterns, arises. No code changed, no documentation changed, no commits, no Governance Register item, no ADR. |

---

# 0. Repository Verification

Verified directly against source this session, not carried forward from any prior session's own account.

| Check | Result |
|---|---|
| Working tree | `git status` — clean at the time of verification |
| Latest commit | `73f7f72` — "docs: reconcile living handoff documents to current repository state (Package B)" |
| All tests pass | `cargo test --workspace` — **253/253**; `apps/sandbox/src-tauri` — **9/9** |
| Governing documents this session | `ARCHITECTURE_EVALUATION_KNOWLEDGE_MODEL_CONSISTENCY.md` (this session's own prior artifact) — Architectural Resolution complete: no conflict between `ProductSpecification.md`/`Glossary.md` and `KnowledgeModel.md`; Adopted Architectural Constraint recorded (§5, below). No Rust source has changed since this evaluation was produced. |

**No blocker was found in verifying repository state.** This document proceeds under the constraint `ARCHITECTURE_EVALUATION_KNOWLEDGE_MODEL_CONSISTENCY.md` §6.2 recorded, not around it.

---

# 1. Executive Summary

This capability gives `modiq-knowledge` its first content informed by patterns observed across real, persisted Assessment activity, rather than exclusively by a priori engineering judgment (the only source `RepairRecipe` has had since Sprint 9). It does not modify the runtime relationship between Assessment execution and the Knowledge Domain: Knowledge remains, and by the Adopted Architectural Constraint must remain, an input to deterministic Assessment execution, never a product of it. Realizing this capability therefore requires a mechanism that sits entirely outside `AssessmentService::execute`'s own path — this document scopes that mechanism to the smallest form consistent with this project's own "capability before abstraction" discipline and the precedent Sprint 9 already established for how Knowledge Domain content comes to exist.

---

# 2. Origin / Player Question

Unchanged from this candidate's original framing: `Vision.md`'s belief that "preserving knowledge is as important as generating it," and `Principles.md`'s Knowledge Preservation principle — "Validated knowledge should accumulate over time... rather than remaining isolated within individual assessments." `ARCHITECTURE_EVALUATION_KNOWLEDGE_MODEL_CONSISTENCY.md` confirmed this intent is not in tension with `KnowledgeModel.md`'s own independence principles once the accumulation mechanism is understood to operate outside live execution.

---

# 3. Observation

Unchanged from this candidate's original evidence-gathering — re-verified, not re-derived, this session.

## 3.1 Storage's current, deliberate scope boundary

`modiq-storage` provides exactly two operations: `store(&AssessmentReport) -> ReportKey` and `retrieve(&ReportKey) -> PersistedAssessmentReport`. No enumeration, listing, or query method exists (`crates/modiq-storage/src/storage/report_store.rs`, confirmed directly). `PROJECT_HANDOFF_v1.1.md` §3 records this as Sprint 13's own deliberate scope boundary, not an oversight.

## 3.2 Knowledge's current, deliberate scope boundary

`modiq-knowledge`'s only real content, `RepairRecipe::version_compatibility_declared_version_mismatch()`, was hand-authored by an engineer (Sprint 9), not derived from any Assessment the platform ever ran. No accumulation, aggregation, or derivation mechanism exists in the crate today.

## 3.3 No dependency edge exists between `modiq-storage` and `modiq-knowledge`

Confirmed directly against both crates' `Cargo.toml`: `modiq-storage` depends on `modiq-report`/`modiq-runtime` only; `modiq-knowledge` depends on nothing.

## 3.4 `AssessmentSubject`/`AssessmentContext` carry no identifying content

Both are zero-field unit structs (`crates/modiq-runtime/src/assessment/subject.rs`, `.../context.rs`). No persisted report can be correlated to *which mod* it concerns. `INITIATIVE_3_ARCHITECTURAL_RESOLUTION.md` adopted, in principle, a future Subject-identity field on `AssessmentReport`, explicitly gated on `AssessmentSubject` first carrying sufficient content — unresolved, and orthogonal to this capability (Section 8).

---

# 4. Architectural Constraint Now Confirmed

`ARCHITECTURE_EVALUATION_KNOWLEDGE_MODEL_CONSISTENCY.md` (Chief-Architect-reviewed, Architectural Resolution complete) found no conflict between `ProductSpecification.md`/`Glossary.md` and `KnowledgeModel.md`, and additionally recorded this **Adopted Architectural Constraint**, binding on this capability:

1. Knowledge remains an input to deterministic Assessment execution.
2. Any accumulation, refinement, or incorporation of validated knowledge derived from Assessment activity occurs outside live Assessment execution.
3. Such mechanisms must not alter or participate in the deterministic execution path of an Assessment.

This does not change what this candidate produces; it fixes where its mechanism may live. Concretely: no part of this capability may be invoked from, or be consulted by, `AssessmentService::execute` or `execute_from_assessment_input`, `RuleEngine::evaluate`, or any Collector — consistent with, not an exception to, `GOVERNANCE.md`'s existing Storage and Knowledge Domain Crate Boundary Rules (`ARCHITECTURE_EVALUATION_KNOWLEDGE_MODEL_CONSISTENCY.md` §4.5).

---

# 5. Capability Identity Classification

Unaffected by Section 4's constraint — the constraint bounds *where* a mechanism may operate, not *what kind* of architectural participant this candidate is. Restated from this candidate's original classification: the three-axis Capability Identity procedure does not apply (no `Evidence`, no `Finding`-shaped judgment, no Collector/Rule question); Architectural Activation does not cleanly apply either, since neither `modiq-storage` nor `modiq-knowledge` is dormant. This candidate remains a **new relationship between two already-activated subsystems**, a shape neither existing classification was built to describe — now additionally shaped by Section 4's constraint on where that relationship may be realized.

---

# 6. Capability Definition

**Knowledge Curation from Accumulated Assessment History**: a minimum-viable, out-of-band mechanism giving an engineer visibility into patterns across previously-stored `AssessmentReport`s — which `RuleReference` values recur, at what `FindingSeverity`, how often — so that new or refined `modiq-knowledge` content (a `RepairRecipe`, in the first instance) can be authored *informed by* real accumulated activity, exactly as `RepairRecipe::version_compatibility_declared_version_mismatch()` was authored from engineering judgment alone at Sprint 9. The authoring act itself remains human-performed, per Sprint 9's own established precedent and `KnowledgeModel.md`'s own framing of Knowledge as curated rather than mechanically derived (`ARCHITECTURE_EVALUATION_KNOWLEDGE_MODEL_CONSISTENCY.md` §4.6) — what's new is that the engineer's judgment can now be informed by real, durable, aggregated evidence rather than by intuition or a single anecdotal report.

This is deliberately narrower than "MKB Accumulation" as originally named: it does not propose that the platform itself decides what knowledge to add, only that it makes the evidence for that decision visible to whoever does.

---

# 7. Minimum Viable Scope

- **`modiq-storage` gains a minimum-viable enumeration capability** — the smallest addition to `ReportStore`'s existing contract that makes reviewing more than one stored report possible at all (e.g., listing every `ReportKey` currently stored at a given root). This is the one real, necessary extension to Storage's own Sprint 13 scope boundary this capability requires. Exact shape (return all keys; return full `PersistedAssessmentReport` values; support any filtering) is an Architectural Resolution question, not decided here.
- **Accumulation is scoped to Rule/Finding pattern frequency only** — e.g., "`RuleReference` X has produced `FindingSeverity` Y across N stored reports" — never to per-mod history. This is the smallest sufficient slice: `PersistedFinding` already carries `rule_reference` and `severity` today: no new persisted field is required, and Section 3.4's `AssessmentSubject` blocker is avoided entirely, not worked around.
- **The mechanism is a distinct, out-of-band step** — not a new `AssessmentService` entry point, and never invoked from `execute`/`execute_from_assessment_input`. Its concrete form (a new `modiq-cli` subcommand, mirroring Sprint 13's own precedent of adding `retrieve`; a separate small tool; or a step folded into a future Sprint's own closeout) is an open Architectural Resolution question (Section 9).
- **The authoring step remains human-performed.** This capability produces visibility into accumulated patterns; it does not itself construct or modify any `modiq-knowledge` type. An engineer, informed by that visibility, authors new Knowledge content in a later, separate step — exactly Sprint 9's own precedent, unchanged.
- No change to `AssessmentService`'s two public entry points, `RuleEngine::evaluate`'s signature, or any Runtime type.

---

# 8. Explicitly Out of Scope

- **Any automated mechanism that constructs, selects, or modifies `modiq-knowledge` content without a human authoring step.** This would blur the line Section 4's constraint and `KnowledgeModel.md`'s own "Knowledge First" principle both hold — knowledge is authored engineering understanding, not a computed artifact.
- **Any mechanism that reads Storage or accumulated pattern data during a live Assessment's execution**, including surfacing "this issue has occurred N times before" inside a `Recommendation` in real time. This would cross exactly the boundary Section 4's constraint protects and is not proposed here.
- **Per-mod correlation or history** — blocked independently by `AssessmentSubject`'s zero-field state (Section 3.4), and not needed by this capability's own minimum-viable scope (Section 7).
- **A general Knowledge Base query, storage, or registry layer** — the same exclusion Sprint 9 named for `RepairRecipe`'s own activation, applied here to the accumulated-pattern data as well: one minimum-viable enumeration capability in Storage, not a general access layer.
- **Any new external dependency.**

---

# 9. Repository Impact

*(Expected, contingent on Architectural Resolution and Implementation Authorization — not committed by this document.)*

- `modiq-storage` gains one new, minimum-viable read operation beyond `retrieve`.
- `modiq-cli` (or a comparable, separately authorized surface) gains a new command exposing accumulated pattern visibility to an engineer.
- No change expected to `modiq-runtime`, `modiq-engine`, `modiq-rules`, `modiq-collection`, `modiq-versioning`, or `apps/sandbox`'s existing behavior.
- No breaking change to any public entry point.
- `modiq-knowledge` itself is not expected to change as a direct consequence of this capability — any new `RepairRecipe` content that later results from an engineer using it is a separate, subsequent act, mirroring Sprint 9's own precedent, not part of this capability's own footprint.

---

# 10. Architectural Considerations

Identified as open questions for Architecture Evaluation — not resolved here, per this document's own charter.

1. **What form does the enumeration capability take?** A full-content listing, a lightweight index of keys plus rule references, or something else — a real design question with real cost/complexity tradeoffs Storage's own Sprint 13 scope deliberately deferred.
2. **Where does the out-of-band mechanism live?** A `modiq-cli` subcommand (Sprint 13's own precedent for `retrieve`), a standalone tool, or a step folded into existing Sprint/Release closeout process — each has different implications for who uses it and how often.
3. **Does accumulated pattern visibility ever need its own Knowledge Domain representation** (e.g., a new type recording "this pattern has been observed"), or does it remain a transient, computed-on-demand view over Storage's own data, never itself persisted as Knowledge? `KnowledgeModel.md`'s seven categories do not currently include anything matching "observed frequency" — introducing one would be a further, separate question from this capability's own minimum scope.
4. **Should this be scoped as its own Sprint, or as a small addition to a future Sprint with a different primary objective?** Not decided here.
5. Section 8 of `CAPABILITY_DEFINITION_KNOWLEDGE_FEEDBACK_LOOP.md`'s prior revision named a narrower, Storage-only "Historical Report Comparison" alternative that never touches Knowledge at all. That alternative remains available and is not foreclosed by this Capability Definition; it addresses a different (though related) need — a user reviewing their own history — rather than an engineer curating new Knowledge content, and could be scoped independently if judged higher-value.

---

# 11. Implementation Risks

- **Scope creep toward automation** is the principal named risk: the temptation to let the enumeration/visibility mechanism also *decide* what Knowledge to add, rather than merely inform a human who decides. Mitigated by Section 8's explicit exclusion and Section 4's own binding constraint.
- **Storage contract growth risk, low-to-moderate:** this is the first addition to `modiq-storage`'s contract since Sprint 13; care is needed that it does not reopen Sprint 13's own deliberately excluded scope (querying, filtering) beyond the minimum this capability needs.
- **Value-without-adoption risk:** unlike Sprint 9's `RepairRecipe` (immediately consumed by a real Rule), this capability's value depends on an engineer actually using the new visibility to author new Knowledge content in a later step — a real, not merely mechanical, risk that the capability ships without ever being used for its intended purpose.
- **Overall implementation risk: Low.** No live-execution path, Runtime type, or public entry point is touched; the new surface is entirely additive and out-of-band.

---

# 12. Success Criteria

Per this project's standing Capability Success Criteria convention (`PROJECT_HANDOFF_v1.1.md` §6):

**After this capability ships, an engineer can** review, across every `AssessmentReport` `modiq-storage` has ever persisted, which `RuleReference`/`FindingSeverity` combinations recur and how often — evidence that did not exist in any usable form before (each report was previously visible only individually, by key, one at a time) — and use that evidence to author new or refined `modiq-knowledge` content with real accumulated grounding, the first time any Knowledge Domain content has been informed by more than one engineer's own a priori judgment.

---

# 13. Chief Architect Questions

*(Not answered here.)*

1. Should the enumeration capability return full `PersistedAssessmentReport` values, or a lighter index sufficient for pattern-counting only?
2. Should the out-of-band mechanism be a `modiq-cli` subcommand, or a different surface entirely?
3. Should this proceed as its own Sprint, or as a bounded addition to a future Sprint with a different primary objective?
4. Should Section 10, Item 3's question (a Knowledge Domain representation for observed frequency) be decided now, or deferred until a real forcing function makes it concrete — mirroring this project's own standing restraint on GOV-008/GOV-013?
5. Is Section 10, Item 5's narrower "Historical Report Comparison" alternative a higher-value use of the next Sprint than this capability, given both are now unblocked?

---

Awaiting Chief Architect review. No implementation, documentation change, governance item, or ADR has been made this session.

# Historical Assessment Analysis — Capability Definition & Capability Identity Classification

| Property | Value |
|---|---|
| **Document** | CAPABILITY_DEFINITION_HISTORICAL_ASSESSMENT_ANALYSIS.md |
| **Project** | modIQ |
| **Purpose** | Capability Definition, Capability Identity Classification, and Architecture Evaluation determination for **Historical Assessment Analysis** — a Storage-domain, read-only capability authorized in place of the suspended Knowledge Feedback Loop capability (`CAPABILITY_DEFINITION_KNOWLEDGE_FEEDBACK_LOOP.md`) |
| **Origin** | Chief Architect Sprint Authorization, following review of `ARCHITECTURE_EVALUATION_KNOWLEDGE_MODEL_CONSISTENCY.md` |
| **Prepared by** | Lead Engineer (Sonnet 5), on `feature/runtime-implementation` |
| **Repository baseline** | `feature/runtime-implementation`, HEAD `73f7f72` |
| **Status** | **Capability Definition and Capability Identity Classification complete. No separate Architecture Evaluation is found to be required (Section 4).** Implementation has **NOT** been authorized. No code changed, no documentation changed, no commits, no Governance Register item, no ADR. Awaiting Chief Architect review and explicit approval before implementation begins. |

---

# 0. Repository Verification

Verified directly against source this session.

| Check | Result |
|---|---|
| Working tree | `git status` — clean at verification, aside from this session's own new/edited documentation |
| Latest commit | `73f7f72` |
| All tests pass | `cargo test --workspace` — **253/253**; `apps/sandbox/src-tauri` — **9/9** |
| Governing prior work | `ARCHITECTURE_EVALUATION_KNOWLEDGE_MODEL_CONSISTENCY.md` — Architectural Resolution complete: no conflict between `ProductSpecification.md`/`Glossary.md` and `KnowledgeModel.md`; Adopted Architectural Constraint recorded (§6.2): Knowledge remains an input to deterministic execution; accumulation occurs outside live execution; no participation in the execution path. This capability is scoped to satisfy that constraint by never touching the Knowledge Domain at all. |
| `modiq-storage` current API | Re-confirmed directly (`crates/modiq-storage/src/storage/report_store.rs`): `ReportStore::store(&AssessmentReport) -> Result<ReportKey, ReportStoreError>` and `ReportStore::retrieve(&ReportKey) -> Result<PersistedAssessmentReport, ReportStoreError>`. No enumeration method exists. |
| `PersistedAssessmentReport` current API | Re-confirmed (`crates/modiq-storage/src/storage/persisted_report.rs`): public accessors `status()`, `evidence()`, `findings()`, `recommendations()`. `PersistedFinding` carries `severity() -> PersistedFindingSeverity` and `rule_reference() -> &str`, both already public. |

---

# 1. Executive Summary

Historical Assessment Analysis reads every `AssessmentReport` `modiq-storage` has ever durably persisted, deterministically compares them, and identifies recurring `RuleReference`/`FindingSeverity` patterns — producing a candidate output for future architectural consideration, not a decision the capability makes itself. It is deliberately narrower than the suspended Knowledge Feedback Loop capability: it never reads or writes anything in `modiq-knowledge`, never runs during `AssessmentService::execute`, and treats "what to do with a recurring pattern" as entirely out of scope. This document finds that no separate Architecture Evaluation is required (Section 4) and recommends the smallest implementation slice capable of demonstrating the complete read-only workflow (Section 5).

---

# 2. Capability Definition

**Historical Assessment Analysis**: a `modiq-storage`-owned, read-only capability that (a) enumerates every report currently held by a `ReportStore`, (b) retrieves each one via the existing `retrieve` operation, and (c) deterministically compares their already-persisted content — specifically, the `rule_reference` and `severity` already carried by every `PersistedFinding` — to produce a candidate summary of which patterns recur, and how often, across the platform's own accumulated Assessment history. The summary is the capability's entire output. Nothing in this capability decides whether, or how, a recurring pattern should influence `modiq-knowledge`, a future Rule, or any other part of the platform — that determination is explicitly deferred to whoever reviews the candidate output later, per the workflow diagram in the authorizing Sprint instructions ("Candidate Output" is the last stage; the workflow ends there).

This is a Storage-domain capability, not a Knowledge-domain one, per the authorizing instruction. It touches `modiq-storage` only (plus, optionally, a thin presentation surface in `modiq-cli`, Section 5).

---

# 3. Capability Identity Classification

**Collection Axis** — not applicable. No `Evidence` is produced; this capability operates entirely on already-persisted `PersistedAssessmentReport` content, downstream of Evidence Collection, Rule Evaluation, and Reporting.

**Evidence Axis** — not applicable. No `EvidenceCategory` is activated or produced.

**Interpretation Axis** — not applicable. No `Finding` or `Recommendation` is produced; identifying a recurring pattern across already-recorded `severity`/`rule_reference` pairs is a deterministic count, not a Rule-shaped judgment over Evidence.

**Capability Introduction Test** — produces no verdict, for the same reason it produced none for the Knowledge Feedback Loop candidate: the test's own wording is Collector/Rule-specific and does not name the relationship this capability introduces (read-only aggregation over already-stored Storage content).

**Architectural Activation check** — does not apply. `modiq-storage` is not dormant; it has real, tested content since Sprint 13. This is not a subsystem's first activation.

**Conclusion.** This capability, like its suspended predecessor, does not fit either existing classification cleanly — the Capability Identity procedure was derived exclusively from Collector/Rule decisions (`INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md` §3) and Architectural Activation requires a dormant subsystem, which `modiq-storage` no longer is. Unlike its predecessor, however, this candidate is materially smaller in architectural kind: it is best understood as an **incremental extension of `modiq-storage`'s own already-real read capability** (from single-key retrieval to enumeration-plus-comparison across everything already stored), not a new relationship between two previously-unconnected subsystems. It never crosses into `modiq-knowledge` at all — the one relationship that made the suspended candidate genuinely novel in shape.

---

# 4. Architecture Evaluation Determination

**Is a separate Architecture Evaluation required? No.** This determination is made directly against evidence, not assumed:

1. **`GOVERNANCE.md`'s actual Storage Crate Boundary Rule ("Must never") is not violated by this capability's scope.** Its four prohibitions are: never participate in or be consulted during Evidence Collection/Rule Evaluation/Report generation; never mutate a stored report once written; never persist `Assessment`/`Evidence`/`Finding`/`Recommendation` as individually addressable entities (only the bundled `AssessmentReport` snapshot); never require a change to `AssessmentService`'s public entry points. This capability is read-only, operates only on already-bundled `PersistedAssessmentReport` snapshots (it does not persist anything new, individually addressable or otherwise), never runs during live execution, and requires no `AssessmentService` signature change. All four prohibitions are satisfied by this capability's own scope, not merely by intended discipline.
2. **`STORAGE_ARCHITECTURE_EVALUATION.md` — Sprint 13's own Architecture Evaluation — explicitly anticipated this exact capability and named the correct path for it**, rather than prohibiting it: *"No querying, filtering, comparison, ranking, or aggregation across multiple Assessments — this defers `INV-002`'s own flagged 'cross-mod collection validation' product question entirely"* and, separately, *"No feed into `modiq-knowledge` / the Knowledge Base... that remains its own, later, separately-evaluated capability, the same way Repair Guidance (Sprint 9) was its own capability built on top of Version Profiles (Sprint 8) rather than bundled into it."* Sprint 13 deferred cross-report comparison for lack of a concrete forcing function at the time, not as a principled prohibition — and explicitly named "its own, later, separately-evaluated capability" as the correct mechanism for taking it up, which is exactly what this document is.
3. **The Adopted Architectural Constraint** (`ARCHITECTURE_EVALUATION_KNOWLEDGE_MODEL_CONSISTENCY.md` §6.2) already establishes the one binding rule a capability of this shape must satisfy — never participate in live execution — and this capability's scope (Section 2) satisfies it by construction: it is read-only and entirely out-of-band.

No open architectural question remains that a dedicated evaluation would need to resolve. Capability Definition proceeds directly to a recommended implementation slice.

---

# 5. Recommended Implementation Slice

The smallest change capable of demonstrating the complete workflow (Persisted Reports → Historical Analysis → Recurring Pattern Identification → Candidate Output), reusing existing types throughout rather than introducing new ones (Section 7):

1. **`ReportStore` gains one new method: `list_keys(&self) -> Result<Vec<ReportKey>, ReportStoreError>`.** Reads the store's root directory and returns every `ReportKey` currently persisted there. This is the smallest addition to Storage's existing contract that makes reading more than one report possible — it does not read report content itself; every retrieval still goes through the existing, unchanged `retrieve`.
2. **A new, small analysis function inside `modiq-storage`** (e.g., a `history` module) — deterministic, taking the reports obtained via `list_keys` + `retrieve` (iterated in a fixed, explicit sort order — Section 9) and producing a summary: for each distinct `(rule_reference, severity)` pair observed across every retrieved report's `findings()`, a count of how many times it occurred. The summary type is a plain struct reusing `PersistedFindingSeverity` and `String` (the existing `rule_reference` type) directly — no new enum (Section 7).
3. **A minimal, read-only presentation surface** — recommended as a new `modiq-cli` subcommand (mirroring Sprint 13's own precedent of adding `retrieve` to an existing binary rather than a new one), printing the summary. This is the step that makes "Candidate Output" actually reachable by whoever reviews it next, completing the workflow the authorizing instruction names, rather than leaving the result only inside a test.
4. **No change** to `AssessmentService`'s two public entry points, `RuleEngine::evaluate`, any Runtime type, or `modiq-knowledge`.

---

# 6. Explicitly Out of Scope

- **Any decision about what a recurring pattern means or what should be done about it.** The workflow ends at Candidate Output; interpreting or acting on it is future, separately-authorized work.
- **Any write path into `modiq-knowledge`, or any construction of a `RepairRecipe` or other Knowledge Domain value.**
- **Any participation in, or consultation during, `AssessmentService::execute` / `execute_from_assessment_input`.**
- **Per-mod correlation or history** — `AssessmentSubject`'s zero-field state (unaffected by this capability) would block it regardless; this capability's own scope (aggregation by `rule_reference`/`severity` only) does not need it.
- **A general query, filter, or ranking layer** — `list_keys` returns everything; no filtering parameter, pagination, or query language is introduced.
- **Any new external dependency.**
- **A new enumeration type for pattern classification** — see Section 7.

---

# 7. Design Guidance Applied

Per the authorizing instruction's Design Guidance, interpreted as follows and stated explicitly rather than assumed silently: "reuse existing Knowledge Domain concepts" and "do not introduce new Knowledge Domain categories" are read as reuse existing *closed-set vocabulary already defined in the Runtime/Storage domain* — `PersistedFindingSeverity` and the existing `rule_reference` string — rather than as license to touch `modiq-knowledge` itself, which Section 6 excludes entirely. "Do not introduce new enumeration types solely for convenience" is applied directly: the pattern summary (Section 5, Item 2) is a plain struct over existing types, not a new enum.

---

# 8. Repository Impact

*(Expected, contingent on this document's own approval — not committed here.)*

- `modiq-storage` gains one new read method (`list_keys`) and one new, small analysis module.
- `modiq-cli` gains one new read-only subcommand.
- No change to `modiq-runtime`, `modiq-engine`, `modiq-rules`, `modiq-collection`, `modiq-versioning`, `modiq-knowledge`, or `apps/sandbox`.
- No breaking change to any public entry point.
- Test suite expected to grow: `modiq-storage` gains tests for `list_keys` (including the empty-store case) and for the analysis function's determinism (Section 9); `modiq-cli` gains tests for the new subcommand, mirroring `retrieve`'s own existing test shape.

---

# 9. Risks

- **Determinism risk, real and specifically named per this project's own discipline ("determinism is verified directly, never assumed"):** filesystem directory listing order is not guaranteed stable across platforms or invocations. Mitigation: `list_keys`/the analysis function must explicitly sort (e.g., by `ReportKey`'s own string value) before comparing, and this ordering guarantee needs its own direct test — proving the summary is identical regardless of underlying directory-read order — not assumed from a single test run.
- **Boundary risk, low:** ensuring this capability is never reachable from any live-execution path. Mitigation: implemented and tested entirely within `modiq-storage` and a new `modiq-cli` subcommand; a test should confirm `AssessmentService`'s two public entry points are unchanged, mirroring Sprint 13's own "zero signature change" verification.
- **Scope-creep risk:** temptation for the "candidate output" to start resembling Knowledge Domain content (e.g., a struct that looks like a `RepairRecipe`). Mitigation: the summary type stays in `modiq-storage`, over `modiq-storage`'s and `modiq-runtime`'s own existing types only.
- **Scale, low, explicitly deferred:** reading every stored report into memory on each invocation is not designed to scale to a large corpus. Acceptable for this minimum-viable slice; not mitigated here, consistent with this project's own "smallest real slice" discipline (Sprint 8/9/13 precedent).
- **Overall implementation risk: Low.**

---

# 10. Success Criteria

Per this project's standing Capability Success Criteria convention (`PROJECT_HANDOFF_v1.1.md` §6):

**After this capability ships, an engineer can** run one command and see, deterministically, which `RuleReference`/`FindingSeverity` combinations recur across every `AssessmentReport` the platform has ever persisted, and how often — the first time this information has been visible at all, since today each stored report is only ever viewed individually, by key. No `modiq-knowledge` content changes as a result; no `AssessmentService` entry point changes; the workflow ends at that visible output.

---

# 11. Implementation Plan (Described Only — Not Executed)

1. **Phase 1:** `ReportStore::list_keys`, plus tests (including the empty-store case and a multi-report case confirming every stored key is returned).
2. **Phase 2:** the analysis module — deterministic aggregation by `(rule_reference, severity)` over reports obtained via Phase 1 + existing `retrieve` — plus a dedicated determinism test proving the summary is independent of underlying read order (Section 9).
3. **Phase 3:** the `modiq-cli` subcommand presenting the summary, plus tests mirroring `retrieve`'s own existing shape.
4. **Validation, every phase:** `cargo fmt`, `cargo check --workspace`, `cargo test --workspace`, and `apps/sandbox/src-tauri`'s own workspace (unaffected, expected to remain 9/9) — per standing convention.
5. **Closeout:** an Implementation Report per this project's standard template; `PROJECT_STATUS.md`/`CHANGELOG.md` reconciliation; no Governance Register item or ADR expected, per Section 4's own determination, re-confirmed at closeout rather than assumed.

Each phase requires its own explicit Implementation Authorization per standing workflow; this plan describes the intended sequence and does not itself authorize any of it.

---

# 12. Chief Architect Questions

*(Not answered here.)*

1. Is `list_keys` the right shape, or should it return richer data (e.g., paired with each report's status) in one call?
2. Is a new `modiq-cli` subcommand the right presentation surface, or should Phase 3 be deferred, leaving this Sprint's own deliverable as a tested library capability only?
3. Should "recurring" require a minimum occurrence threshold (e.g., ≥2) before appearing in the candidate output, or should the summary include every observed pair regardless of count?
4. Is the Design Guidance interpretation in Section 7 correct, or was "existing Knowledge Domain concepts" intended more literally (i.e., referencing `modiq-knowledge`'s own types in some read-only way this document has not considered)?

---

Awaiting Chief Architect review and explicit approval. No implementation will begin until this Capability Definition is approved.

# Platform Validation: GOV-008 — AssessmentService Public API Evolution

| Property | Value |
|----------|-------|
| **Document** | PLATFORM_VALIDATION_GOV-008.md |
| **Cycle** | Platform Validation (opened by `PROPOSAL_PLATFORM_VALIDATION_REVIEW.md`; second item evaluated, following GOV-004) |
| **Stage** | Technical evaluation only — no recommendation, no resolution |
| **Prepared by** | Engineering, for Technical Director review |
| **Governance item under evaluation** | GOV-008 — AssessmentService Public API Evolution |
| **Status** | Design-only. No Rust code, no governance changes, no documentation changes, no ADR, no implementation proposal. |
| **Repository state reviewed** | Post-GOV-004 implementation (working tree clean at time of writing; `modiq-engine`/`modiq-rules` stub scaffolding already deleted) |

---

# 1. Executive Summary

GOV-004's resolution confirmed `AssessmentService` as the platform's sole orchestration boundary and removed the unused internal service model that once sat alongside it. GOV-008 asks a related but distinct question, open since Sprint 3 Phase 3 and never resolved: does `AssessmentService`'s *public API shape* — its entry points, its input type, its output type, its error type — represent the platform's correct long-term boundary, now that the boundary's *internal* composition question (GOV-004) is settled?

This document gathers implementation evidence only. It does not evaluate whether GOV-004 was correctly decided (that is closed), and it does not recommend any API change, redesign, or resolution for GOV-008. Every observation below is sourced directly from current repository state.

One terminology note, established before anything else: **no type named `AssessmentResult` exists anywhere in this repository** — not in source, not in any specification. A workspace-wide search returns zero matches. Where this document's source material (the evaluation request) refers to "AssessmentResult," the evidence below addresses the closest real artifact — `AssessmentReport`, `AssessmentService::execute`'s actual return type — and states plainly, at the relevant point, that the named concept does not exist as asked.

---

# 2. AssessmentService Responsibility Analysis

## Orchestration responsibilities (confirmed present)

Read directly from `crates/modiq-engine/src/engine/assessment_service.rs`:

- Sequences the Assessment lifecycle: constructs `Assessment::new`, calls `begin_evidence_collection`, adds each Evidence item, calls `begin_rule_evaluation`, invokes the Rule Engine, adds any resulting Finding/Recommendation, generates the Report, then calls `complete` — all within `execute`'s body, in that fixed order.
- Composes three subsystems by direct instantiation: `modiq_rules::rules::RuleEngine`, `modiq_report::report::AssessmentReport::generate`, and (in `execute_from_assessment_input`) `modiq_collection::collection::EvidenceCollector`. This is unchanged by GOV-004 — `assessment_service.rs` was not modified by that implementation; only its unused sibling files were removed.
- Constructs `AssessmentInput` from a raw `impl Into<String>` parameter, inside `execute_from_assessment_input`, before handing it to `EvidenceCollector`.

## Domain responsibilities (confirmed absent from `AssessmentService`)

- Lifecycle invariant enforcement is entirely inside `Assessment` (`modiq-runtime`). `AssessmentService` never inspects `AssessmentStatus` or overrides an invariant; it calls `Assessment`'s own methods and immediately `.expect()`s success, assuming (via code comment, not a type guarantee) that the precondition already holds — e.g., `.expect("a newly created Assessment is always in the Created state")`.
- Rule evaluation logic is entirely inside `RuleEngine::evaluate` (`modiq-rules`).
- Evidence production logic is entirely inside `EvidenceCollector::collect` (`modiq-collection`).
- Report structure and field selection is entirely inside `AssessmentReport::generate` (`modiq-report`).

## Has GOV-004 changed the role of `AssessmentService`?

**No, by direct comparison of source.** `assessment_service.rs` was not among the files touched by the GOV-004 implementation (confirmed by the deletion set: four `modiq-engine` files and four `modiq-rules` files, none of them `assessment_service.rs`). Its method signatures, method bodies, and doc comments are unchanged. What GOV-004 changed was documentation (`EngineAPI.md`) and the removal of unused sibling scaffolding (`KnowledgeService`, `ReportingService`, `RuleEvaluationService`, `VersionProfileService`, and the `modiq-rules` mirror) that had never been part of `AssessmentService`'s own implementation. `AssessmentService`'s role — orchestrating the lifecycle by direct subsystem composition — is the same role it has had since Sprint 1; GOV-004 formally confirmed that role in governance and documentation rather than altering it in code.

---

# 3. Public API Analysis

Two public entry points exist on `AssessmentService`. Both confirmed directly from source; caller sets confirmed by a workspace-wide search.

## `execute(&self, subject: AssessmentSubject, context: AssessmentContext, evidence: Vec<Evidence>) -> AssessmentReport`

- **Origin:** Sprint 1 — the platform's original, and for one full Sprint only, orchestration entry point (`CrateRoadmap.md` revision 1.4.0).
- **Why it exists:** it is the reference implementation of the full Assessment lifecycle, accepting already-constructed Evidence directly rather than acquiring it itself.
- **Callers, confirmed by workspace-wide search:** its own four unit tests inside `assessment_service.rs`; three integration tests in `crates/modiq-engine/tests/end_to_end.rs` (headed "Architectural proof for Sprint 1... drives the engine's public API only"); and internally, the tail call inside `execute_from_assessment_input`. **No application (Sandbox, CLI) calls `execute` directly**, at any point in the current repository.
- **Determination:** the method is `pub`, not `pub(crate)` — its visibility is broader than its observed usage. It is heavily and directly tested, and is the method `execute_from_assessment_input` is defined in terms of. Evidence supports classifying it as **validated as an internal orchestration primitive**; evidence is **insufficient to determine** whether its continued standalone public visibility (independent of `execute_from_assessment_input`) is a deliberate design position or simply unrevisited since Sprint 1 — no document states either.

## `execute_from_assessment_input(&self, subject: AssessmentSubject, context: AssessmentContext, input: impl Into<String>) -> Result<AssessmentReport, AssessmentExecutionError>`

- **Origin:** Sprint 3 Phase 4, as `execute_from_descriptor`; renamed in Phase 5. Added alongside `execute`, per its own doc comment, specifically because "whether `execute` itself should evolve to accept an AssessmentInput remains open (ADR-0009, GOV-008)" — i.e., its own existence is a direct, on-the-record consequence of GOV-008 remaining unresolved.
- **Why it exists:** to drive real Evidence Collection ahead of the existing pipeline, without deciding GOV-008.
- **Callers, confirmed by workspace-wide search:** `apps/sandbox/src-tauri/src/lib.rs` — the platform's only current application consumer, calling this method exactly once, and no other entry point.
- **Determination:** evidence supports classifying it as **validated as the platform's real, currently-exercised orchestration boundary** — it is the only entry point any application uses.

## A third entry point (anticipated by ADR-0009, not present)

ADR-0009 recorded, without deciding, that a third possibility existed: `execute`'s own signature changing directly (a breaking change) rather than continued additive growth. No such change, and no third parallel entry point, exists in current source. **Insufficient evidence** to classify what a third entry point would look like; sufficient evidence only that the platform's own prior architectural record (the Lead Engineer Handoff, quoted in `PLATFORM_VALIDATION_GOV-004.md`) named a third parallel entry point as "the signal that routing around \[GOV-008\] has stopped working" — and none has yet been added.

## Redundancy

No evidence supports classifying either entry point as redundant with the other: `execute` is independently, directly tested and is not merely inlined into `execute_from_assessment_input` — it remains the tail call the second method delegates to, and the second method adds real Collection orchestration `execute` does not perform.

---

# 4. Input/Output Boundary Evaluation

## `AssessmentInput`

- **Structure**, confirmed from `crates/modiq-collection/src/collection/assessment_input.rs`: a single-field wrapper, `AssessmentInput { value: String }`, constructed via `AssessmentInput::new(impl Into<String>)`, validating only that the value is non-empty.
- **Scope**, confirmed by its own doc comment and by GOV-009's resolution text in `GOVERNANCE.md`: explicitly "a stable reference to a filesystem object" — scoped to the filesystem case, not a general-purpose evidence-source descriptor.
- **Where construction actually happens:** inside `AssessmentService::execute_from_assessment_input` (`modiq-engine`), from a raw `impl Into<String>` parameter the method itself receives. `AssessmentInput` as a typed value never crosses `AssessmentService`'s own public API boundary — callers (today, only the Sandbox) supply an untyped string; the Engine constructs the typed `AssessmentInput` internally, then immediately hands it to `EvidenceCollector`.
- **Bearing on "cleanly separates orchestration from evidence collection":** `AssessmentInput`'s own doc comment states it is "supplied by the application layer through the Engine," which describes a semantic origin (the application) but not the literal construction call-site (inside the Engine crate). The type itself is never held, inspected, or passed by the calling application — the Sandbox's own source contains no reference to `AssessmentInput` at all, only to the raw string literal it passes.

## `AssessmentReport` (the actual return type; "AssessmentResult" does not exist)

- **Structure**, confirmed from `crates/modiq-report/src/report/assessment_report.rs`: `assessment_id`, `status`, `evidence: Vec<Evidence>`, `findings: Vec<Finding>`, `recommendations: Vec<Recommendation>` — a flat, owned, point-in-time snapshot, produced by `AssessmentReport::generate(&assessment)`.
- **Timing:** generated before `assessment.complete()` is called (confirmed in `execute`'s body) — every report reflects a not-yet-`Completed` Assessment. This ordering is governed by the still-open GOV-001, not evaluated further here (out of this review's scope).
- **The only observed downstream consumer:** the Sandbox's `AssessmentSummary` struct (`apps/sandbox/src-tauri/src/lib.rs`), a hand-written, IPC-serializable projection of `AssessmentReport` — each field re-typed for `serde`/Tauri IPC (identifiers rendered via `format!("{:?}", ...)`, enums rendered as debug strings). This is the only evidence available in the repository about what shape a consumer actually needs from the engine's output; it is a single data point, from a single, explicitly non-production application.
- **No serialization, versioning, or schema-stability mechanism exists on `AssessmentReport` itself** — `#[derive(Debug, Clone, PartialEq, Eq)]` only. `ENGINEERING_RELEASE_0.3.md`'s Known Limitations already named the related, broader gap: no `Display`/`Serialize` implementation exists for Runtime identity/enum types, flagged in four consecutive release records without being scheduled.

---

# 5. Error Boundary Evaluation

**Central finding: `AssessmentError` (`modiq-runtime`) is never exposed by `AssessmentService`'s public API.** Every `Assessment` method call inside `execute` (`begin_evidence_collection`, `add_evidence`, `begin_rule_evaluation`, `add_finding`, `add_recommendation`, `complete`) is immediately followed by `.expect(...)`, converting any `Result<_, AssessmentError>` into either silent success or an unrecoverable panic. `execute`'s own return type is `AssessmentReport` directly — no `Result` at all. `AssessmentError`'s six variants exist, are tested, and are enforced — entirely inside `modiq-runtime`'s own test suite; no test outside that crate constructs or matches on `AssessmentError`.

The only fallible public path is `execute_from_assessment_input`, returning `AssessmentExecutionError` (`modiq-engine`), which — confirmed from source — wraps exactly two variants: `InvalidInput(#[from] AssessmentInputError)` and `Collection(#[from] CollectionError)`, both originating in `modiq-collection`. `AssessmentExecutionError` carries zero variants originating from `modiq-runtime`, `modiq-rules`, or `modiq-report`.

Evaluated against the four named dimensions:

- **Determinism.** The `.expect()` calls are deterministic given `execute`'s own fixed internal sequencing — no test in the repository exercises a violated invariant reaching one of them, because `execute`'s own control flow guarantees the precondition each `.expect()` assumes. Whether a future change to that internal sequencing could silently turn a typed error into a panic is not evidenced either way.
- **Explainability.** The two variants that are exposed (`AssessmentExecutionError::InvalidInput`, `::Collection`) both derive `#[error(transparent)]` over `thiserror`-derived inner errors with human-readable messages, confirmed in source. The six `AssessmentError` variants that are not exposed also carry `thiserror` messages, but explainability at the public boundary is currently untested, since nothing outside `modiq-runtime` ever observes them.
- **Platform independence.** `AssessmentExecutionError`'s shape is specific to Evidence Collection's two failure categories. No variant exists for a Rule Engine failure, a Reporting failure, or a Knowledge/Version Profile failure — because none of those subsystems currently has a fallible entry point `AssessmentService` calls (`RuleEngine::evaluate` returns `Option`, not `Result`; `AssessmentReport::generate` is infallible). Whether this error shape generalizes if a future subsystem introduces its own fallible entry point is not evidenced either way.
- **Future evidence sources.** `CollectionError` currently has exactly two variants (`Inaccessible`, `Unsupported`), both explicitly scoped to the filesystem case by GOV-010's own resolution text ("resolved... for the filesystem case"). No variant, wrapper, or extension point for an archive-specific, provider-specific, or cloud-specific failure exists anywhere in current source.

---

# 6. Platform Compatibility Assessment

Using implementation evidence only, no speculation beyond what is directly observed:

- `EvidenceCollector` (`modiq-collection`) is a concrete, zero-field unit struct with a single method, `collect(&self, input: &AssessmentInput) -> Result<Vec<Evidence>, CollectionError>`, implemented directly against `std::fs` (`symlink_metadata`, `read_dir`). No trait definition exists anywhere in `modiq-collection`, `modiq-engine`, or `modiq-rules` — confirmed by a direct search for `trait` across all three crates, returning zero results. There is no provider abstraction, no dispatch mechanism, and no second concrete `Collector`-shaped type anywhere in the workspace.
- `AssessmentInput` is a single opaque `String`, validated only for non-emptiness, with no discriminator field, enum variant, or scheme prefix distinguishing a filesystem path from any other kind of location reference.
- **Consequence for ZIP archives, additional filesystem providers, or cloud providers:** none has any implementation footprint today. Supporting any of them would require `EvidenceCollector::collect`'s concrete `std::fs` logic to change, be replaced, or be joined by an as-yet-nonexistent second concrete type; nothing in `AssessmentInput`'s, `CollectionError`'s, or `EvidenceCollector`'s current type signatures provides an extension seam for this today. This is consistent with, and does not go beyond, what `PLATFORM_VALIDATION_GOV-004.md` and `ENGINEERING_RELEASE_0.3.md` already documented: no Collector trait exists, by deliberate, twice-affirmed choice, pending a second concrete collector to justify one.
- **Consequence for future platform (Version) profiles:** `AssessmentContext` (`crates/modiq-runtime/src/assessment/context.rs`) is a zero-field unit struct — `pub struct AssessmentContext;` — carrying no data of any kind. `execute`'s signature accepts an `AssessmentContext` parameter, but no line in `execute`'s body reads from it. Whether `AssessmentContext`'s current shape could absorb real Version Profile data additively, or would require a breaking change, is **not evidenced either way** — no code path currently reads from `AssessmentContext` at all, so no evidence exists about what changing its shape would require.
- `modiq-cli`'s `AssessCommand` remains `pub struct AssessCommand;` — zero fields, zero methods, unwired to `AssessmentService` in any way. It provides no evidence about which entry point (`execute` or `execute_from_assessment_input`) a second real application would use, because it does not yet call either.

---

# 7. Evidence Classifications

Every significant observation above, classified. "Requires Refinement" here means the evidence identifies a real, unresolved tension — not that a refinement is being recommended.

| Observation | Classification | Evidence |
|---|---|---|
| `AssessmentService` composes subsystems by direct instantiation, unchanged by GOV-004 | **Validated** | `assessment_service.rs` not among files GOV-004 modified; identical method bodies before/after |
| GOV-004 did not change `AssessmentService`'s role | **Validated** | Same as above; only sibling scaffolding removed |
| `execute` has zero application-level callers | **Validated** | Workspace-wide search: only test code (unit + integration) and internal tail call |
| `execute_from_assessment_input` is the platform's only real orchestration boundary in use | **Validated** | Sandbox is the sole application consumer; calls only this method |
| `execute`'s continued independent public visibility is deliberate vs. unrevisited | **Insufficient Evidence** | No document states either; method is `pub`, usage is test-only |
| A third entry point (per ADR-0009) is needed | **Insufficient Evidence** | ADR-0009 anticipates the possibility; none exists; no forcing case in evidence |
| `AssessmentInput` is scoped to the filesystem case only | **Validated** | Own doc comment; GOV-009 resolution text in `GOVERNANCE.md` |
| `AssessmentInput` cleanly separates "application supplies" from "Engine constructs" | **Requires Refinement** | Doc comment says application-supplied; actual construction call-site is inside `modiq-engine`, from a raw string |
| No type named `AssessmentResult` exists | **Validated** | Zero matches, workspace-wide, including documentation |
| `AssessmentReport` has exactly one known real downstream consumer shape | **Validated** | Sandbox's `AssessmentSummary`; no second consumer exists |
| `AssessmentReport`'s sufficiency as "the" long-term output | **Insufficient Evidence** | Only one consumer, one application, observed; no serialization/versioning mechanism exists to evaluate |
| `AssessmentError` never reaches `AssessmentService`'s public API | **Validated** | Every call site `.expect()`s; `execute`'s return type is infallible |
| `AssessmentExecutionError`'s explainability, for the variants it exposes | **Validated** | `#[error(transparent)]` over `thiserror`-derived messages, confirmed in source |
| `AssessmentExecutionError`'s generality to future fallible subsystems | **Insufficient Evidence** | No other subsystem currently has a fallible entry point to generalize against |
| `CollectionError`'s applicability beyond the filesystem case | **Requires Refinement** | Explicitly scoped to "the filesystem case" by its own governing resolution (GOV-010); no ZIP/cloud variant exists |
| No Collector trait or provider abstraction exists | **Validated** | Zero `trait` definitions across all three relevant crates, confirmed by direct search |
| `AssessmentInput` has no discriminator for non-filesystem sources | **Validated** | Single opaque `String` field, no enum, no scheme prefix |
| `AssessmentContext` can absorb Version Profile data without a breaking change | **Insufficient Evidence** | Zero-field unit struct; no code path reads from it at all today |
| `modiq-cli` provides evidence about a second application's entry-point choice | **Insufficient Evidence** | `AssessCommand` is fully unwired; calls neither entry point |

---

# 8. Remaining Unknowns

- Whether a lifecycle-invariant violation reaching one of `execute`'s internal `.expect()` calls is a scenario the platform needs to guard against at the public boundary, or one its own internal sequencing already forecloses by construction — no test or incident evidences either position.
- Whether `AssessmentContext`'s current empty shape is intended to grow additively (as `RuleReference`/`RepairRecipeReference` were described, in ADR-0007, as deliberate placeholders) or would need a breaking change to carry real Version Profile data — no document states this.
- Which entry point a future `modiq-cli` wiring, or any second real application, would use — no such code exists yet to provide evidence.
- Whether a future ZIP or cloud collector would reuse, extend, or replace `CollectionError` and `AssessmentInput` — GOV-010's and GOV-009's resolutions are both explicit that they resolved only the filesystem case, and neither anticipates the shape of a resolution for any other case.
- Whether `execute`'s continued public visibility, despite an entirely test-only caller set, reflects a considered design position (a stable low-level primitive, kept public deliberately) or simply hasn't been revisited since Sprint 1 — no document addresses this either way.

---

# 9. Final Neutral Assessment

The evidence shows an orchestration boundary (`AssessmentService`) whose internal shape GOV-004 has just confirmed, sitting behind a public surface (two entry points, one input type, one output type, two error types across two crates) that has grown additively, once, in response to Evidence Collection's introduction, and has not been revisited since. Every extension so far — Evidence Collection, the cardinality invariants, the error unification in `AssessmentExecutionError` — was accommodated without a breaking change to `execute`'s original signature. Nothing in current implementation evidence demonstrates that this pattern will or will not continue to hold for a second real collector, a second real application consumer, or a Version Profile that actually varies behavior. Both outcomes remain equally consistent with the evidence gathered here. This document reaches no conclusion about which is more likely, and makes no recommendation about GOV-008's resolution — it exists only to give the Technical Director a precise, sourced account of what the implementation currently does, ahead of that decision.

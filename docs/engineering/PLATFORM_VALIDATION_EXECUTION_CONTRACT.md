# Platform Validation: The Public Engine Execution Boundary

| Property | Value |
|----------|-------|
| **Document** | PLATFORM_VALIDATION_EXECUTION_CONTRACT.md |
| **Cycle** | Platform Validation (opened by `PROPOSAL_PLATFORM_VALIDATION_REVIEW.md`; follows GOV-004 and the GOV-008 evidence review) |
| **Stage** | Technical evaluation only — no recommendation, no resolution |
| **Scope** | `AssessmentInput`, `AssessmentReport`, the public error model, and execution semantics, evaluated together as one boundary |
| **Prepared by** | Engineering, for Technical Director review |
| **Status** | Design-only. No Rust code, no governance changes, no documentation changes, no ADR, no implementation proposal. |
| **Repository state reviewed** | Post-GOV-004 implementation (working tree clean at time of writing) |

---

# 1. Executive Summary

This document evaluates whether `AssessmentInput`, `AssessmentReport`, the public error model, and execution semantics — the four elements a caller actually touches when driving an Assessment through `AssessmentService` — behave as one coherent contract, or whether implementation evidence shows them diverging from each other in ways a single contract would not. `PLATFORM_VALIDATION_GOV-008.md` already examined each element individually against GOV-008's specific question (is the API's long-term shape correct); this document's lens is narrower and different: taken together, right now, do these four things agree with each other?

The evidence shows a boundary assembled from four different crates, exercised completely by exactly one entry point (`execute_from_assessment_input`) and only partially by the other (`execute`), consumed by exactly one real application that does not handle the errors this boundary defines. Several concrete points of disagreement are documented below. This document does not assess whether any of them warrants a governance item — only that each is directly observable in current source.

---

# 2. Element-by-Element Evidence

## 2.1 `AssessmentInput`

- Defined in `modiq-collection`, not `modiq-engine`: `crates/modiq-collection/src/collection/assessment_input.rs`.
- A single-field wrapper, `AssessmentInput { value: String }`, constructed via `AssessmentInput::new(impl Into<String>)`, validating only non-emptiness.
- Scoped explicitly, by its own doc comment and by GOV-009's resolution text, to "a stable reference to a filesystem object" — the filesystem case only.
- Derives: `Debug, Clone, PartialEq, Eq`. No `Serialize`/`Deserialize`.
- Constructed inside `AssessmentService::execute_from_assessment_input` from a raw `impl Into<String>` parameter — never held, referenced, or imported by any application. A workspace-wide search finds no reference to `AssessmentInput` anywhere in `apps/sandbox`.

## 2.2 `AssessmentReport`

- Defined in `modiq-report`, not `modiq-engine`: `crates/modiq-report/src/report/assessment_report.rs`.
- A flat, owned snapshot: `assessment_id`, `status`, `evidence: Vec<Evidence>`, `findings: Vec<Finding>`, `recommendations: Vec<Recommendation>`. Produced by `AssessmentReport::generate(&assessment)`, called before `assessment.complete()` in both `execute` and (via `execute`) `execute_from_assessment_input` — every report reflects a not-yet-`Completed` Assessment (governed by the still-open GOV-001, not further evaluated here).
- Derives: `Debug, Clone, PartialEq, Eq`. No `Serialize`/`Deserialize`.
- Is the return type of both public entry points — the one element of the four every path through `AssessmentService` produces.
- Its only observed downstream consumer, `apps/sandbox`'s `AssessmentSummary`, re-derives every field by hand into a `serde`-serializable shape; nothing in `AssessmentReport` itself supports that consumption directly.

## 2.3 The Public Error Model

Three distinct error types participate, defined across two crates:

- `AssessmentInputError` (`modiq-collection`) — one variant, `EmptyValue`.
- `CollectionError` (`modiq-collection`) — two variants, `Inaccessible { path }` and `Unsupported { path }`, both explicitly scoped to the filesystem case by GOV-010's resolution text.
- `AssessmentExecutionError` (`modiq-engine`) — the type `execute_from_assessment_input` actually returns: `InvalidInput(#[from] AssessmentInputError)` and `Collection(#[from] CollectionError)`, both `#[error(transparent)]`.

A fourth type exists but is not part of this public model: `AssessmentError` (`modiq-runtime`), covering six lifecycle-invariant variants, is constructed and matched exclusively inside `modiq-runtime`'s own tests. Every call site inside `AssessmentService::execute` immediately `.expect()`s the `Result` it returns. `execute`'s own return type, `AssessmentReport`, carries no `Result` at all — `execute` is infallible by signature.

Confirmed by direct source inspection: `apps/sandbox`'s `Cargo.toml` depends on `modiq-runtime`, `modiq-engine`, and `modiq-report` — **not** `modiq-collection`. `AssessmentExecutionError`'s two variants both wrap types defined in `modiq-collection`. The Sandbox's own call to `execute_from_assessment_input` discards the `Result` with `.expect(...)`, never inspecting or matching either variant. No reference to `AssessmentExecutionError`, `CollectionError`, or `AssessmentInputError` exists anywhere in `apps/sandbox`'s source. The only code anywhere in the workspace that constructs or matches these three error types by name, outside their own defining modules, is `modiq-engine`'s own test suite (`assessment_service.rs`'s test module, which imports `modiq_collection::collection::{AssessmentInputError, CollectionError}` directly).

## 2.4 Execution Semantics

- `execute`: always succeeds, always creates an `Assessment` (even with zero Evidence — confirmed by `execute_without_evidence_produces_no_findings_or_recommendations`), sequences lifecycle transitions internally via `.expect()`-guarded calls, generates the Report before calling `complete`.
- `execute_from_assessment_input`: constructs `AssessmentInput`, invokes `EvidenceCollector::collect`, and only then calls `execute` — meaning an `Assessment` is created **only if** both prior steps succeed (Collection Atomicity, per `EvidenceCollection.md` and confirmed by the test `execute_from_assessment_input_never_creates_an_assessment_when_collection_fails`, whose own comment states there is no partial Assessment to inspect on failure, because none is ever created).
- Both paths are deterministic in content (judged by field values, not by freshly-assigned identity, per every test that checks determinism across repeated calls).
- No public method exists to query lifecycle state, cancel an in-progress execution, or resume one — each call to either entry point is a single, synchronous, all-or-nothing unit of work, evidenced by both methods' signatures (`&self, ...) -> AssessmentReport` / `-> Result<AssessmentReport, AssessmentExecutionError>`, with no intermediate handle returned.

---

# 3. Cross-Element Coherence Analysis

## 3.1 The two entry points do not exercise all four elements symmetrically

`execute_from_assessment_input` is the only path that touches all four: it constructs an `AssessmentInput`, can fail through the public error model, runs the full execution semantics described above, and returns an `AssessmentReport`. `execute` touches only two of the four directly — it accepts `Vec<Evidence>` in place of `AssessmentInput` and cannot fail through the public error model at all (it has no `Result` return type). `AssessmentInput` and the public error model are both, in current evidence, properties of exactly one of the two public entry points, not of "the boundary" as a whole.

## 3.2 The public error model requires a dependency the one real caller doesn't have

`AssessmentExecutionError`'s variants wrap types owned by `modiq-collection`. Meaningfully matching on those variants — as opposed to treating the whole `Result` as opaque — requires a caller to import `modiq-collection` types directly. `apps/sandbox` does not depend on `modiq-collection` at all, and does not match on the error; it discards it via `.expect()`. The only code that does match on these variants by name lives inside `modiq-engine`'s own test suite. No evidence in the repository shows the public error model being consumed as a public error model, by anything, outside the crate that defines the method returning it.

## 3.3 `AssessmentInput` and `AssessmentReport` differ in scope by an order of magnitude

`AssessmentInput` is a single opaque string, validated for one property (non-emptiness), scoped explicitly to the filesystem case. `AssessmentReport` is a five-field aggregate reflecting the entire observable state of an Assessment — every Evidence item, every Finding, every Recommendation, plus identity and status. One element of the four is minimal and narrowly scoped; its paired output is broad and comprehensive. Nothing in current evidence indicates this asymmetry was designed as a deliberate input/output shape, or is simply a byproduct of `AssessmentInput` being new (Sprint 3) while `AssessmentReport` has existed, and grown, since Sprint 1.

## 3.4 No lifecycle state, from the platform's own Public API Policy, has been applied to any of the four

`GOVERNANCE.md`'s Public API Policy states "every public API shall exist in one of the following states" — Experimental, Internal, Stable, Deprecated, Removed — and that "public APIs are considered contracts" requiring governance approval for breaking changes. A direct search of `docs/architecture/`, `GOVERNANCE.md`, and all relevant crate source finds no instance of any of these five state labels applied to `AssessmentInput`, `AssessmentReport`, `AssessmentExecutionError`, `CollectionError`, `AssessmentInputError`, `execute`, or `execute_from_assessment_input`. The policy exists; its application to this specific boundary is not evidenced in either direction.

## 3.5 Internal consistency that does hold

Several dimensions show no observed inconsistency:

- **Derive consistency.** `AssessmentInput` and `AssessmentReport` both derive exactly `Debug, Clone, PartialEq, Eq`, no more and no less. All three error types derive `Debug, Clone, PartialEq, Eq, thiserror::Error`. No type in this boundary derives `Serialize`/`Deserialize`, consistently.
- **Error wrapping style.** Both of `AssessmentExecutionError`'s variants use identical `#[error(transparent)] ... #[from]` shape — neither variant introduces its own message text distinct from its wrapped type's.
- **Determinism.** Every test that checks repeated-call determinism, across both entry points and across `AssessmentReport`, `Finding`, and `Recommendation`, applies the same convention: content is compared, identity is not.
- **Report timing.** Both entry points generate the `AssessmentReport` at the identical point in the lifecycle (before `complete()`), inherited structurally through `execute_from_assessment_input`'s delegation to `execute` — there is exactly one place in source where report-generation timing is decided, not two independently-maintained copies of that decision.

---

# 4. Observations Bearing on Possible Future Governance Items

Listed as observed tensions only. This document takes no position on whether any should become a governance item, or how it would be resolved if it did.

- The public error model's variants require a dependency (`modiq-collection`) that sits outside the boundary a caller otherwise interacts with (`modiq-runtime`, `modiq-engine`, `modiq-report`), and the one real caller does not take that dependency.
- `AssessmentInput` participates in one of two public entry points; `execute` remains a fully independent path with a different input shape (`Vec<Evidence>`) and no public error surface.
- `AssessmentReport` carries no serialization mechanism, despite being the sole return value every execution path produces and the one element every current and hypothetical future consumer would need to cross a process or IPC boundary — evidenced today by the Sandbox's need to hand-build a parallel DTO.
- `AssessmentContext`, though not one of the four elements named in this document's scope, is passed into both entry points as part of what a caller must supply, and is a zero-field type nothing reads from — its presence in every call site's signature is unexplained by any behavior currently attached to it.
- No document states whether `AssessmentInput`'s filesystem-only scope, or `CollectionError`'s two filesystem-only variants, are expected to be extended, replaced, or joined by parallel types when a second evidence source (e.g., ZIP traversal) is implemented.

---

# 5. Evidence Classifications

| Observation | Classification | Evidence |
|---|---|---|
| `AssessmentReport` is the only element common to both entry points | **Validated** | Both `execute` and `execute_from_assessment_input` return it; confirmed by signatures |
| `AssessmentInput` is exercised by exactly one of two entry points | **Validated** | `execute` takes `Vec<Evidence>` directly; only `execute_from_assessment_input` constructs `AssessmentInput` |
| The public error model is unreachable without a dependency the one real caller lacks | **Validated** | `apps/sandbox/Cargo.toml` has no `modiq-collection` dependency; confirmed by direct file read |
| The public error model has ever been consumed by a real application | **Requires Refinement** — evidence indicates it has not | Sandbox discards the `Result` via `.expect()`; zero matches on any of the three error types outside `modiq-engine`'s own tests |
| `AssessmentInput` and `AssessmentReport` are symmetric in scope | **Requires Refinement** — evidence indicates they are not | One validated field vs. a five-field aggregate; no document states this asymmetry is deliberate |
| Any of the four elements has a stated Public API Policy lifecycle state | **Validated as absent** | Zero matches for Experimental/Internal/Stable/Deprecated/Removed applied to any of the four, across docs and source |
| Derive and error-wrapping conventions are internally consistent across the boundary | **Validated** | Identical derive sets; identical `#[error(transparent)]` shape on both `AssessmentExecutionError` variants |
| Report-generation timing is decided once, not duplicated | **Validated** | `execute_from_assessment_input` delegates to `execute` for this behavior; one call site, not two |
| `AssessmentInput`'s filesystem-only scope will extend cleanly to future evidence sources | **Insufficient Evidence** | GOV-009's own resolution text states it resolved only the filesystem case; no later resolution exists |
| `AssessmentContext`'s presence in both signatures is behaviorally justified today | **Requires Refinement** — evidence indicates it is not | Zero-field type; no code path in either entry point reads from it |
| The four-element boundary has ever been exercised end-to-end by anything other than `modiq-engine`'s own test suite | **Insufficient Evidence** | Only one real consumer (Sandbox) exists, and it does not exercise the error model at all |

---

# 6. Remaining Unknowns

- Whether `execute`'s continued existence as a full, independent, `AssessmentInput`-free entry point is intended to remain a permanent second shape, or is a byproduct of GOV-008 never having resolved whether `execute` itself should change — no document states this beyond what `PLATFORM_VALIDATION_GOV-008.md` already found.
- Whether the absence of any Public API Policy lifecycle label on these four elements reflects a considered position (e.g., "everything is implicitly Experimental until GOV-008 resolves") or is simply unaddressed — no document states either.
- Whether a real second application (a future `modiq-cli` wiring) would need to depend on `modiq-collection` directly to handle `AssessmentExecutionError` meaningfully, or would follow the Sandbox's precedent of discarding it — no such code exists yet to provide evidence.
- Whether `AssessmentReport`'s lack of a serialization mechanism is intentional (deferred until a second consumer makes the shape clearer) or an oversight of the same kind already flagged for Runtime identity/enum types in `ENGINEERING_RELEASE_0.3.md`'s Known Limitations — no document connects the two observations explicitly.

---

# 7. Final Neutral Assessment

Within each of the four elements taken alone, the evidence shows internal consistency — consistent derives, consistent error-wrapping style, consistent determinism discipline, a single decided point for report-generation timing. Taken together, as one boundary a caller must cross as a unit, the evidence shows less agreement: one entry point exercises all four elements, the other exercises two of them under a different shape; the public error model's variants require a dependency the platform's only real caller does not take, and that caller does not consume the error model at all; and the platform's own Public API Policy — which names five lifecycle states every public API is supposed to occupy — has not been applied to any of the four. Whether this reflects four elements still settling into a single coherent contract, or four elements that were never designed as one contract in the first place, is not resolvable from implementation evidence alone. This document takes no position on which is the case, and makes no recommendation about GOV-008, GOV-009, GOV-010, or any new governance item.

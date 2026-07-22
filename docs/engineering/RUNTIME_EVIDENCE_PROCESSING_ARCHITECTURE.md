# Runtime Evidence Processing Architecture

> **The Sprint 11 Architectural Resolution: the architectural model governing how a real runtime observation becomes an interpretable platform outcome.**

---

| Property | Value |
|----------|-------|
| **Document** | RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md |
| **Version** | 1.1.0 |
| **Status** | Approved in principle by the Chief Architect; this revision incorporates a refinement pass (governance-relationship clarification, Architectural Invariants) requested during that review. No code, crate, interface, test, fixture, ADR, or Governance Register item has been created or modified in producing this document or this revision. |
| **Project** | modIQ |
| **Documentation Release** | 2.1 (unchanged; this document does not amend it) |
| **Owner** | Zach Weber |
| **Created** | 2026-07-22 |
| **Last Updated** | 2026-07-22 |

---

# Specification Authority

Authority, per `SPRINT11.md`:

- Vision.md
- ProductSpecification.md
- Architecture.md
- DataModel.md
- KnowledgeModel.md
- RuleEngine.md
- EvidenceCollection.md
- GOVERNANCE.md

Frozen inputs this document builds on and does not reopen:

- `SPRINT9_RUNTIME_LOG_ARCHITECTURE_EVALUATION.md` / `SPRINT_ROADMAP_UPDATE_v1.md` — the resolved Runtime Log Interpretation workflow: bundled submission, the same single Assessment Input every existing Collector inspects, no second Assessment Input, no cross-Assessment correlation.
- `SPRINT10_CAPABILITY_DEFINITION.md` — the capability's own scope: recognizing exactly one class of signal (a mod failed to load), and its strengthened evidentiary precondition (no architectural decision may assume a real log's shape).
- `COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`, Chief Architect Decision Record (Section 14) — the approved Collector composition model: independent Collectors, composed inline inside `AssessmentService`, with a named, five-condition extraction threshold for when a dedicated coordinator becomes justified.
- `GOVERNANCE.md`, GOV-012 (Rule Evaluation Model, Resolved) — `RuleEngine::evaluate` returns `Vec<RuleOutcome>`, Rules dispatch in fixed declaration order, compose independently, no suppression.
- The three real fixtures this document is grounded in: `fixtures/runtime-logs/clean-base-game/`, `single-compatible-mod/`, `single-incompatible-mod/` (Sprint 10; `ENGINEERING_RELEASE_1.0.md`).

This document resolves the four questions `SPRINT11.md`'s Scope section names. It does not propose Rust code, does not fix a concrete type or method name as binding, and does not authorize implementation. Where an illustrative name is used for a not-yet-existing type (a Collector, a Rule), it is marked explicitly as illustrative — naming is an implementation decision, not fixed here.

---

# 1. Where and How a Runtime Observation Enters the Assessment Pipeline

## 1.1 The composition question is already answered in general; this Sprint asks whether it still holds for a second content-inspecting participant

`COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`'s Chief Architect Decision Record already resolved, for Sprint 7, the general shape multi-source Evidence Collection takes: Collectors remain fully independent, never depend on or consume one another's output, and are composed by `modiq-engine` as direct, inline composition inside `AssessmentService::execute_from_assessment_input` — no `CollectionCoordinator`, no registry, no trait unifying Collectors. That decision named a concrete, five-condition extraction threshold for when a dedicated coordination component becomes justified instead of inline composition:

- three or more content Collectors typically participate in an Assessment
- Collector applicability becomes significantly complex
- Collector execution order becomes configurable
- parallel collection becomes desirable
- multiple Assessment entry points require identical orchestration logic

Today, exactly one content Collector exists under this axis: `XmlCollector` (Sprint 7), invoked unconditionally alongside the structural Collector selected by the existing, separate, mutually-exclusive `is_archive_location` routing (`EvidenceCollector` vs. `ArchiveCollector`, Sprint 3/4 — unaffected by anything in this document). A Runtime Log Evidence source would be the **second** content Collector under the composition axis, not the third. **None of the five extraction conditions is met by adding a second participant** — applicability logic remains a simple, deterministic check (does a recognized filename exist at the Assessment Input's root), execution order is not configurable, no parallelism is proposed, and only one Assessment entry point (`execute_from_assessment_input`) is involved. Inline composition, exactly as it stands today, remains the correct and sufficient shape. No new type, trait, module, or Governance Register item is warranted by this question alone.

## 1.2 Locating the observation: a well-known filename at the Assessment Input's root, mirroring `XmlCollector`'s precedent — with one deliberate divergence

The already-resolved workflow decision (Sprint 9, Option A) establishes that a runtime log is supplied as part of the same submission as the mod, against the same Assessment Input every existing Collector already inspects — not a second input, not a later correlation. This document resolves the remaining question that decision left open: *how* a new Collector locates the log within that shared input.

**Evidence-grounded conclusion:** all three real fixtures — captured independently, on separate occasions, for three different scenarios — store their raw log content under the identical filename, `log.txt` (`fixtures/runtime-logs/clean-base-game/README.md`, `single-compatible-mod/README.md`, `single-incompatible-mod/README.md`, each: `**File(s)** | log.txt`). Per `fixtures/runtime-logs/README.md`'s own Structure section, a fixture's raw log file is "stored verbatim" — the corpus does not rename captures for its own convenience. A future Collector locating a file named `log.txt` at the Assessment Input's root is therefore mirroring `XmlCollector`'s own precedent exactly: a fixed, well-known filename, checked for directly, the same architectural shape already proven for `modDesc.xml`.

**The one deliberate divergence from `XmlCollector`'s own absence-handling precedent, reasoned explicitly rather than copied by default:** Sprint 7's record states that `XmlCollector` treats a missing manifest as Evidence in its own right, not silent Empty Collection — because every mod is expected to declare a `modDesc.xml`; its absence is itself an anomalous, informative fact about the mod's own structure. A runtime log's expected-presence baseline is the opposite. Per the resolved workflow, supplying a log at all is something a user *may choose* to do when they have one to offer — the overwhelming majority of Assessments will have no log bundled, and that is the ordinary, unremarkable case, not an anomaly. Recording "no runtime log present" as an Evidence item on every Assessment that lacks one would manufacture a low-information Evidence item for the common case, which is a different situation from `modDesc.xml`'s always-expected presence. **This document resolves that a missing log is Legitimate Absence — Empty Collection, per the existing four-outcome Collection Error Model (`EvidenceCollection.md`, GOV-010) — not a recorded absence-fact**, diverging from `XmlCollector`'s specific behavior while remaining fully within the existing outcome model. No new Collection Outcome is introduced; `Invalid`/`Inaccessible`/`Unsupported` continue to cover a malformed, unreachable, or unreadable log exactly as they already cover every other content type this platform inspects (`EvidenceCollection.md`, Collection Outcomes).

## 1.3 No new dependency, no new crate, no new public entry point

Consistent with `SPRINT9_RUNTIME_LOG_ARCHITECTURE_EVALUATION.md`, Section 7: recognizing a fixed, `Error:`-prefixed line naming a mod (Section 2, below) requires no exotic parsing dependency — plain string matching (`strip_prefix`/`contains`, the same mechanism `VersionCompatibilityRule` already uses against `XmlInspection` Evidence) is sufficient, mirroring this platform's own precedent rather than introducing a regular-expression or grammar dependency for a single recognized pattern. The new Collector belongs in `modiq-collection`, alongside `EvidenceCollector`, `ArchiveCollector`, and `XmlCollector`; the new Rule belongs in `modiq-rules`, alongside the existing three. Neither `AssessmentService`'s two public entry points nor `RuleEngine::evaluate`'s parameter shape requires any change — the same additive-only outcome Sprint 8 and Sprint 9 both produced, and the same outcome `SPRINT11.md`'s own Engineering Goals require absent contrary evidence, which nothing in this analysis surfaces.

---

# 2. `EvidenceCategory::RuntimeLogs`'s Content Shape

## 2.1 Content Extraction discipline, applied to a runtime log exactly as it was applied to a manifest

`XmlCollector`'s Content Extraction discipline (Sprint 7, extended Sprint 8) reports a fact, never an interpretation: a declared `descVersion` value is recorded as `XmlInspection` Evidence in a fixed, factual form (`"modDesc.xml declares descVersion: {value}"`); judgment about whether that value is acceptable happens later, inside `VersionCompatibilityRule`, never inside the Collector. The same discipline applies here without modification: `RuntimeLogs` Evidence must describe **what the log's own content states**, never what it means.

**Evidence-grounded content:** the one real, captured failure signature (`single-incompatible-mod`) is the literal engine line `Error: Unsupported mod description version in mod FS25_DodgeChallengerHellcat`, immediately following that same mod's `Available mod:` enumeration entry, with no subsequent `Load mod:` line anywhere in the file. `RuntimeLogs` Evidence produced from a recognized signal should carry a factual, minimally-normalized capture of that observation — the recognized line's own text (or the mod name it names, extracted from it) — with a `location` reference to the log file, mirroring `Evidence::with_location`'s existing use by `XmlCollector` for `modDesc.xml`. The Collector reports the fact that this line was observed; it does not state that the mod "failed" or "is broken" — that conclusion belongs to the Rule Engine (Section 3).

## 2.2 The recognition boundary: general enough to generalize past one mod name, narrow enough not to outrun the evidence

`SPRINT11.md`'s own Risks section (Single-sample design risk) requires the recognition mechanism be designed "against the general shape of the evidence... rather than hardcoding the one literal string." This document resolves that requirement concretely, not by assertion: the recognized shape is a discrete, `Error:`-prefixed engine line naming a specific mod, matching the exact literal template `Error: Unsupported mod description version in mod {mod_name}` — generalized over the mod name (so it is not bound to `FS25_DodgeChallengerHellcat` specifically) but **not** generalized over the message text itself. A Collector recognizing *any* line beginning `Error:` regardless of content would outrun what one real fixture can support — this corpus has exactly one documented failure class, and `SPRINT10_CAPABILITY_DEFINITION.md`'s own scope is deliberately "one narrow, well-defined class of runtime log signal," not a general error-line recognizer. The correct boundary sits precisely between over-fitting (the literal mod name hardcoded) and over-generalizing (any `Error:` line treated as this signal) — both of which this platform's own history has already corrected against elsewhere (`SPRINT_ROADMAP...`/GOV-013's own conflation correction; the Warning Categorization gap found during Sprint 10 acquisition).

## 2.3 No new `EvidenceCategory` variant

`EvidenceCategory::RuntimeLogs` is real, defined since before Sprint 1 (`crates/modiq-runtime/src/assessment/evidence_category.rs:7`), unused across ten Sprints. This document's content-shape resolution activates it; it does not add a new variant, split it, or otherwise change `modiq-runtime`'s existing closed-set enum.

---

# 3. How This Evidence Is Interpreted Into a Platform Outcome

## 3.1 A new Rule, fourth in GOV-012's fixed declaration order

GOV-012 (Resolved) fixes `RuleEngine::evaluate`'s dispatch as an explicit, ordered sequence — currently `EvidencePresenceRule`, then `StructuralDuplicationRule`, then `VersionCompatibilityRule` (`crates/modiq-rules/src/rules/engine.rs:51-58`), each introduced in the Sprint that required it and appended, never reordered. A new Rule interpreting `RuntimeLogs` Evidence is appended as the **fourth** entry in this same fixed sequence, following the identical pattern every prior Rule addition has already used — no reordering of the existing three, no suppression relationship introduced between this Rule and any other (GOV-012, Question 3, already resolved in general: Rules compose independently). This Rule does not consume `version_profile`, exactly as `EvidencePresenceRule` and `StructuralDuplicationRule` already do not — `RuleEngine::evaluate`'s existing signature, which already accepts an argument not every Rule uses, requires no change to accommodate it.

## 3.2 `FindingSeverity`: the evidence supports `Error` — the platform's first real use of that variant

This is the one place this document reaches a conclusion `SPRINT11.md`'s own Scope section explicitly flagged as non-obvious: Farming Simulator's own log calls this an "Error," but "that is not automatically this platform's own Finding Severity vocabulary." Resolved here by direct comparison against `DataModel.md`'s own Finding Severity definitions, not by deferring to the engine's own label:

> **Error.** *"The assessed mod will not function correctly... as conclusively established by Evidence alone. Reserved for conclusions a user should treat as blocking. A Rule must not assign `Error` for a suspected, probable, or environment-dependent problem — only for a defect Evidence directly proves."*

`single-incompatible-mod`'s own evidence satisfies this bar precisely, and does so more strongly than any existing Rule's own basis for its assigned severity: the mod's `Available mod:` enumeration is immediately followed by the rejection line, and **no `Load mod:` line for that mod exists anywhere in the 1,450-line file** — a direct, observed, conclusive fact that the mod never reached a loadable state, corroborated independently by the acquisition team's own observation that the mod never appeared in the Start Game → Mods selection list. This is categorically different from `VersionCompatibilityRule`'s own basis for assigning `Warning` instead of `Error`: that Rule judges a *static declaration* (`crates/modiq-rules/src/rules/version_compatibility_rule.rs:35-41`, its own code comment) — "this platform cannot conclusively establish from manifest content alone that the mod will not function." A runtime log observation is not a prediction from a static declaration; it is a direct record of an actual attempt, and the attempt is directly evidenced to have failed. `Error`'s own bar — "conclusively established by Evidence alone," "blocking," never for "a suspected, probable, or environment-dependent problem" — is exactly what this evidence supports and `VersionCompatibilityRule`'s evidence does not.

**This is a material fact, verified directly this session, not assumed:** `FindingSeverity::Error` has never been assigned by any Rule anywhere in this platform's implementation history. `EvidencePresenceRule` assigns `Informational`; `StructuralDuplicationRule` and `VersionCompatibilityRule` both assign `Warning` (`crates/modiq-rules/src/rules/*.rs`, verified by direct grep this session). A Rule interpreting this Sprint's recognized signal would be the platform's first real exercise of the `Error` variant since it was defined.

## 3.3 Documented interaction with GOV-013 — not resolved, not silently bypassed

GOV-013 (`FindingSeverity` Severity/Kind Conflation, Open) was left deliberately unresolved at Sprint 5, "to be revisited once the Rule Engine has multiple concrete Rules operating in practice... not decided from two Rules alone." `DataModel.md`'s own Finding Severity section restates this as the model's explicit provisional status. This Sprint's own severity assignment does not resolve GOV-013 — GOV-013 concerns whether `BestPractice` (a classification of *kind*) is coherently expressed on the same enum as the ordered `Error`/`Warning`/`Informational` *severity* scale, a question orthogonal to which of the three ordered values a new Rule should assign. Nothing here requires or proposes splitting `FindingSeverity`, and this document takes no position on GOV-013's own question.

What this document *does* record, for whoever next revisits GOV-013: this Sprint's own reasoning is the first concrete case where a Rule's severity choice was decided by direct, comparative reference to `Error`'s own documented bar against a competing candidate (`Warning`, `VersionCompatibilityRule`'s own precedent) — precisely the kind of "multiple concrete Rules operating in practice" evidence GOV-013's resolution named as its own future forcing function. This is offered as evidence for that future review, not as this Sprint's own attempt to close it.

**The governance relationship, stated explicitly.** Section 3.2's conclusion and this section's own restraint rest on a distinction this document makes load-bearing rather than implicit: **architectural interpretation** — that a bundled log directly evidencing a mod's rejection at modDesc validation represents the platform's strongest, most conclusive class of finding — is a different claim from **governance vocabulary** — that this platform currently spells that conclusion `FindingSeverity::Error`, specifically, as opposed to some other value a differently-shaped severity model might use instead. This Sprint adopts the currently approved `FindingSeverity` model exactly as `GOVERNANCE.md` and `DataModel.md` presently define it — `Error`, `Warning`, `Informational`, `BestPractice`, unmodified. The assignment of `Error` to this Rule's outcome reflects the repository's present governance baseline, not a claim that this baseline is the only vocabulary capable of expressing the underlying architectural conclusion. Accordingly:

- This assignment does not prejudge, resolve, or constrain any future resolution of GOV-013.
- Should GOV-013 later change the severity model — splitting kind from severity, renaming a variant, or otherwise reshaping the vocabulary — the *architectural interpretation* this document establishes (Matrix row 3: a conclusively evidenced load failure, the platform's strongest class of finding) does not need to be re-derived. Only its governance-vocabulary label would need to follow whatever GOV-013 eventually approves; the underlying interpretation this document reasons its way to remains valid across that relabeling.
- Implementation occurring after any such future change should follow the updated governance vocabulary current at that time, not this document's specific enum value, without that update calling this document's own reasoning (Section 3.2) into question.

## 3.4 Recommendation content: inline-authored, no Knowledge Domain pairing

`SPRINT11.md`'s Out of Scope explicitly excludes "any Knowledge Domain pairing (a Repair Recipe for a runtime-log Finding)." `RuleEngine.md`'s own Recommendation Generation responsibility already permits this: *"Associated Repair Recipes inform Recommendation content but do not independently trigger a Recommendation"* — a `Recommendation` does not require a `RepairRecipeReference` to exist validly. This Rule's `Recommendation` therefore carries inline-authored guidance text with `repair_recipe_reference: None`, mirroring exactly how `VersionCompatibilityRule` itself operated before Sprint 9 introduced `RepairRecipe::version_compatibility_declared_version_mismatch()` — a real, already-proven precedent for a Rule producing a valid Recommendation with no Knowledge Domain involvement at all. Pairing a future Repair Recipe remains available as later, additive work (named at Sprint 9 and again at Sprint 10 as deferred), not required for this Rule to be complete.

---

# 4. The Runtime Interpretation Decision Matrix

The deterministic architectural contract future implementation must conform to. Each row states the observation, its architectural interpretation, a stated confidence level, and the intended processing stage. No entry is a placeholder; every entry either cites one of the three real fixtures directly or states explicitly why no fixture yet grounds it.

| # | Observation | Architectural Interpretation | Confidence | Intended Rule / Processing Stage |
|---|---|---|---|---|
| 1 | No recognized log file (`log.txt`) present at the Assessment Input's root | Legitimate Absence — a routine, unremarkable submission-time fact, not evidence of anything about the mod. No `RuntimeLogs` Evidence is produced. | **High.** This is a structural fact about the submission, not an inference about a mod; it requires no generalization claim. | Collector-level (Section 1.2). Collection resolves to Empty Collection for this content type. No Rule is invoked — there is no `RuntimeLogs` Evidence to evaluate. |
| 2 | A recognized log file is present, and contains no line matching the recognized failure template (`clean-base-game`, `single-compatible-mod`) | No mod-load failure occurred, as far as this platform's one recognized signal can determine. No Finding is produced. | **High** for the specific claim "the recognized signature is absent" (directly, repeatedly confirmed: zero `Error` lines in either fixture). **Not** a general claim that the mod is defect-free — only that this one recognized signal is absent. | Rule Engine. The new Rule filters `RuntimeLogs` Evidence for the recognized template and returns no outcome when none matches — mirroring `VersionCompatibilityRule::evaluate`'s own `None`-return shape exactly. |
| 3 | A line matching `Error: Unsupported mod description version in mod {name}` is present, with no corresponding `Load mod: {name}` line afterward (`single-incompatible-mod`) | The named mod failed to load, rejected at modDesc validation, before registration or asset loading. Conclusively established by direct observation, not inference. | **High** for this exact, documented signature (one real, fully corroborated fixture: enumeration observed, rejection observed, absence of any `Load mod:` line for that name confirmed across all 1,450 lines, independently corroborated by the acquisition team's own UI observation). **Not yet established** that this is the *only* mod-load failure signature Farming Simulator produces — see Section 6. | Rule Engine. New Rule (illustrative name only: a "Runtime Load Failure Rule"), fourth in GOV-012's fixed dispatch order. `FindingSeverity::Error` (Section 3.2). Recommendation: inline-authored, `repair_recipe_reference: None` (Section 3.4). |
| 4 | A log line resembling a failure (containing `Error:`, or naming a mod in an unfamiliar way) that does not match the recognized template exactly | Not recognized. No Finding is produced — the Rule must not guess at a signature this corpus has not evidenced. | **Explicitly bounded, not high.** This is precisely the generalization boundary Section 2.2 and `SPRINT11.md`'s own Risks section name: recognizing *only* the evidenced template, deliberately, rather than any superficially similar line. | Rule Engine. The new Rule's matching is exact against the recognized template (Section 2.2); anything else falls through to no outcome, by design, not by omission. |

**Determinism statement, per `SPRINT11.md`'s own Engineering Goals:** every row above is exercised identically against the same fixture content every time, using only deterministic string matching (Section 1.3) — no heuristic, scoring, or probabilistic mechanism. Once implemented, determinism is verified the same way every existing Collector and Rule already verifies it: repeated evaluation of identical fixture content produces byte-identical Evidence and Finding content (the same discipline `VersionCompatibilityRule`'s own `is_deterministic_for_identical_input` test already applies, to be mirrored by this Rule's own test).

---

# 5. Architectural Invariants

Rules future implementation must never violate. These are architectural constraints, restating and applying already-established platform boundaries specifically to runtime evidence processing — none is a new boundary, and none is left to implementation's own discretion.

**Runtime evidence remains factual until interpreted.** Collectors acquire facts; they do not create Findings, and they do not infer meaning. The Collector this document describes (Section 1) reports only that a recognized line was observed in a recognized location — never that a mod "failed" or "is broken." This restates the Collector Contract's own Non-Responsibilities and Guarantees (`EvidenceCollection.md`: "Evidence Collection observes; it does not conclude," "Factual accuracy... reflects what was objectively present... not an inference, guess, or opinion") applied to this Sprint's own new Evidence source without exception.

**Evidence remains immutable during interpretation.** Rules interpret Evidence; they never modify it. The Rule this document describes (Section 3) reads `RuntimeLogs` Evidence and produces a Finding and Recommendation from it — it does not rewrite, annotate, or otherwise alter the Evidence itself. This restates `RuntimeInvariants.md`'s existing INV-002 (Evidence may only be added before rule evaluation begins) and INV-003 (Evidence becomes immutable once rule evaluation starts), and ADR-0007's Aggregate Root Pattern (only `Assessment`, through its own methods, mutates anything it owns) — no exception is introduced for `RuntimeLogs` Evidence or the Rule that reads it.

**Runtime interpretation is deterministic.** Equivalent, normalized runtime evidence must always produce identical architectural interpretation. Section 4's Matrix is exercised by fixed string matching alone, with no heuristic, scoring, or probabilistic step at any row — the same determinism guarantee the Collector Contract already requires of every Collector, and `RuleEngine.md`'s own Deterministic execution principle already requires of every Rule, applied here without weakening.

**Unknown runtime signatures never produce inferred Findings.** An unsupported or unrecognized runtime observation remains Evidence — recorded, factual, available for a future Rule — until a deterministic architectural rule explicitly supports interpreting it. Matrix row 4 exists specifically to enforce this: a log line that resembles the recognized signature without matching it exactly produces no Finding, by design, rather than a guessed one. This restates `RuleEngine.md`'s own Evidence-Based principle ("Rules never operate on assumptions or unsupported observations") and directly extends the discipline this document already applies in Section 2.2 (the recognition boundary is exact, not heuristic).

**Knowledge augments interpretation; it does not rewrite runtime evidence.** Where a future Repair Recipe is eventually paired with this Rule's Finding (deferred, Section 3.4), it may enrich the Recommendation's guidance or explanation — it may never alter the runtime Evidence the Finding is traceable to, or the Finding's own severity or description. This restates `RuleEngine.md`'s Knowledge-Driven principle ("The Rule Engine operationalizes knowledge rather than creating it") and `KnowledgeModel.md`'s existing boundary that Engineering Knowledge remains independent of any individual Assessment's own recorded facts.

---

# 6. Fixture Corpus Sufficiency for Implementation

**Conclusion, stated explicitly per `SPRINT11.md`'s own requirement, not left as an assumption: the current three-fixture corpus is sufficient for Implementation to begin against this document's own resolved, narrow scope (Matrix rows 1–4) — no additional fixture is a precondition for Implementation to responsibly start.**

This conclusion holds specifically because Sprint 10's own capability scope, and this document's own recognition boundary (Section 2.2), are both deliberately narrow: recognizing one documented signature, not a general log-reading capability. All three fixtures directly ground every row of the Matrix: two establish the negative case with real, cross-referenced corroboration (`clean-base-game`, `single-compatible-mod`), and one establishes the positive case with a fully evidenced, unambiguous failure (`single-incompatible-mod`). Nothing in this document's own scope requires evidence this corpus does not provide.

**What the corpus does *not* establish, named explicitly rather than assumed away, consistent with `SPRINT11.md`'s own Risks section:**

- **Platform coverage.** All three fixtures were captured on macOS. Whether the recognized template's exact wording is stable on Windows or Linux is unknown. This document does not treat cross-platform stability as established, and recommends — without requiring — that a non-macOS capture of at least the `single-incompatible-mod` scenario precede any claim that this Rule's recognition is platform-general. This is a named, deferred candidate (`SPRINT11.md`'s own Deferred Work), not acquired here.
- **Signature generality.** One failure class, one sample. This corpus cannot establish that all Farming Simulator mod-load failures share this signature's shape. Matrix row 4 exists specifically to keep this boundary explicit rather than silently over-generalized.
- **Platform stability over time.** Per `fixtures/runtime-logs/README.md`'s own Platform Independence and superseding policy, a future game patch that changes this exact message's wording would be handled by the corpus's own existing mechanism (a new, superseding fixture) — this document's own architecture already relies on that mechanism (Matrix row 3's confidence is scoped to the evidenced signature, not asserted as permanent) rather than assuming today's wording is stable indefinitely, satisfying `SPRINT11.md`'s own Engineering Goals requirement on this point.

Neither gap blocks Implementation from proceeding against Matrix rows 1–4 as resolved. Both are recommended, not required, considerations for whichever future Sprint takes up broader signature or platform coverage — named here so they are not later rediscovered as surprises, per this project's standing practice.

---

# 7. Summary of Repository Impact

*(Informational — no implementation performed by this document.)*

- **New Collector** (`modiq-collection`): locates `log.txt` at the Assessment Input's root, alongside `XmlCollector`, composed inline (Section 1.1). Absence is Empty Collection (Section 1.2).
- **New Rule** (`modiq-rules`): fourth in GOV-012's fixed dispatch order, filters `RuntimeLogs` Evidence for the recognized template (Section 3.1–3.2), assigns `FindingSeverity::Error` — the platform's first use of that variant — and produces an inline-authored Recommendation with no Repair Recipe pairing (Section 3.4).
- **No change** to `AssessmentService`'s two public entry points, `RuleEngine::evaluate`'s parameter shape, `EvidenceCategory`'s variant set, or the Collection Outcome model.
- **No new external dependency.** Plain string matching only (Section 1.3).
- **No new internal dependency edge** beyond what already exists — the new Collector depends on `modiq-runtime` exactly as every existing Collector already does; the new Rule depends on `modiq-runtime` exactly as every existing Rule already does. Neither requires `modiq-versioning` or `modiq-knowledge`.
- **No new Governance Register item, no ADR.** Rule dispatch extension is already covered by GOV-012's own general resolution; Collector composition extension is already covered by the Sprint 7 Collector Composition Architecture's own extraction threshold, not crossed here (Section 1.1). GOV-013 is documented, not reopened (Section 3.3).
- **No additional fixture required** as an Implementation precondition (Section 6); two candidates (non-macOS capture, broader signature coverage) are named as recommended, non-blocking future work.

---

# 8. Review Against the Four Standing Questions

Per the Sprint 11 Review Standard:

1. **Is it deterministic?** Yes — Section 4's Matrix is exercised by fixed string matching alone; no heuristic or probabilistic step is introduced anywhere in this resolution.
2. **Is it evidence-based?** Yes — every architectural decision above cites at least one of the three real fixtures directly, or an established repository precedent (`XmlCollector`, `VersionCompatibilityRule`, GOV-012, the Collector Composition Architecture, ADR-0007). No decision rests on an assumption about a Farming Simulator log's shape.
3. **Is it architecturally consistent?** Yes — no existing boundary is redesigned: the Evidence Collection / Rule Engine separation, `AssessmentService`'s two public entry points, and GOV-012's fixed dispatch order are all preserved exactly, extended additively.
4. **Will future implementation become simpler because this document exists?** Yes — every question `SPRINT11.md`'s Scope section named is answered concretely enough that implementation can proceed directly against Section 4's Matrix, without re-deriving the composition model, the content shape, the dispatch position, or the severity assignment from first principles.

---

# Document Status

**Current Version:** 1.0.0

**Status:** Proposed. Awaiting Chief Architect review before Implementation Authorization. No Rust source file has been modified; `cargo test --workspace` remains at 210/210, Sandbox at 7/7, verified directly this session, unchanged by producing this document.

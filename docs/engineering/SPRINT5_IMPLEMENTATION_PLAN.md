# Sprint 5 Implementation Plan

| Property | Value |
|----------|-------|
| **Document** | SPRINT5_IMPLEMENTATION_PLAN.md |
| **Project** | modIQ |
| **Sprint** | Sprint 5 — Assessment Intelligence Layer |
| **Status** | All five implementation phases complete and Technical Director-approved. GOV-012 and GOV-013 recorded in `GOVERNANCE.md`; `FindingSeverity` definitions recorded in `DataModel.md` v1.1.0; `StructuralDuplicationRule` implemented (Phase 2); `RuleEngine::evaluate` multi-Rule dispatch implemented (Phase 3); Reporting scaffold investigation complete, recommending retirement of all four unused types, eligible pending formal governance approval but not acted on this Sprint (Phase 4); test coverage gap closed and full workspace/Sandbox verification clean (Phase 5). Sprint 5 Closeout (repository reconciliation, `ENGINEERING_RELEASE_0.5.md`) not yet authorized. |
| **Predecessor Documents** | `ENGINEERING_RELEASE_0.4.md` (Sprint 4 closing record and Recommendation), `SPRINT4_IMPLEMENTATION_PLAN.md` (template this document mirrors) |
| **Last Updated** | 2026-07-21 |

---

# Sprint Objective

Sprint 4 gave the platform its second real Evidence Collector. It did not give the platform a second real Rule. Since Sprint 1, exactly one Rule has ever existed — `evidence-presence-rule` — and it does not evaluate Evidence at all in any meaningful sense: it fires unconditionally on any non-empty Evidence set, regardless of category or content, and always produces the same Informational Finding and the same generic Recommendation. Two Evidence Collectors now exist, `EvidenceCategory` carries eight distinct categories (including the Sprint 4 Phase 3C addition, `StructuralDuplication`), and the Rule Engine still cannot distinguish any of them from any other.

Per Technical Director direction, this Sprint does not add a third Collector. It strengthens the platform vertically: it makes `RuleEngine` genuinely evaluate the Evidence categories that already exist, producing Findings and Recommendations whose severity, content, and applicability actually depend on what was observed — the "deterministic model that transforms collected evidence into findings, recommendations, and explainable technical assessments" the Technical Director named. XML inspection remains the next Collector, but it should arrive after this model exists, not before it, so that a third Collector's Evidence has a mature Rule Engine to be evaluated by rather than one more category the one universal Rule continues to ignore.

This is deliberately the same kind of Sprint Engineering Release 0.3 called Sprint 3 Phase 5 relative to Sprint 3 Phase 4: the boundary and the first minimal proof already exist (one real Rule, real Finding/Recommendation types, real traceability plumbing in the Runtime Domain); this Sprint gives that boundary its first genuinely evaluative content.

---

# Scope

## In Scope

- A second, real, concrete Rule that evaluates `EvidenceCategory::StructuralDuplication` specifically — the most immediately available, non-speculative case, since this category already exists (GOV-011, Sprint 4 Phase 3C), already carries clear, governance-written meaning ("collection's own inspection mechanism could not fully resolve every entry"), and is currently swallowed into the same generic Finding as every other category.
- Real, explicit Rule Selection: `RuleEngine::evaluate` (or a small set of inline-dispatched internal functions it calls) examines Evidence by category and applies whichever concrete Rule(s) actually match, rather than unconditionally applying one universal Rule to all Evidence regardless of content.
- Support for an Assessment producing more than one Finding and more than one Recommendation from a single evaluation, when more than one Rule matches. The Runtime Domain already supports this without modification — `Assessment::add_finding`/`add_recommendation` already accept repeated calls, and `evidence_for_finding`/`findings_for_recommendation` already resolve relationships generically.
- A design investigation, in Phase 1 of this plan (not decided by this document), into the four existing, unused `modiq-report` scaffold types — `FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, `ReportFormatter` — to determine whether they are actually built out to fulfill Reporting's already-assigned responsibilities (`GOVERNANCE.md`, Crate Boundary Rules: "report formatting," "report summarization," "traceability output"), or found still premature and explicitly retired, the same way GOV-004 retired the unused `EngineAPI`/`modiq-rules` service scaffolding once real subsystems proved a different shape was correct. See Design Question 4, below.
- Real assignment of `FindingSeverity` (`Error`, `Warning`, `Informational`, `BestPractice`) to at least the two Rules this Sprint implements — this taxonomy has existed in `modiq-runtime` since Sprint 2 and has never been used for anything but `Informational`, and no specification document (`DataModel.md`, `Glossary.md`) currently defines what any of the four variants actually means.
- Extending the existing generic Rule (`evidence-presence-rule`) only as far as necessary to coexist correctly with the new specific Rule, per whatever composition policy GOV-012 (below) resolves — not a rewrite of its existing behavior for its own sake.

## Out of Scope

- XML inspection, or any new Evidence Collector — explicitly deferred by Technical Director direction. This Sprint does not touch `modiq-collection`, `ArchiveCollector`, or `EvidenceCollector`.
- Knowledge Domain integration (`modiq-knowledge`). `RuleReference` and `RepairRecipeReference` remain exactly as they are today — opaque, `modiq-rules`-internal identifiers and an always-`None` reference, respectively. `RuleReference`'s own doc comment already anticipates a future `modiq-knowledge`-sourced identifier; this Sprint does not perform that integration, and no Rule this Sprint implements requires it.
- Version Profile-aware Rule Selection. `modiq-versioning` remains untouched (L1, unchanged since Sprint 0). `RuleEngine.md`'s own Rule Selection responsibility names Assessment Context and Version Profile as selection criteria; this Sprint's Rule Selection is based on Evidence Category alone, since no Version Profile capability exists yet to select against. This is a real, acknowledged narrowing of the specification's full Rule Selection responsibility, not a claim that Version Profile awareness is unnecessary.
- Resolution of GOV-001 (Assessment Report generation timing), GOV-002 (Runtime Invariant Reconciliation), or GOV-003 (role of `modiq-common`). All three are adjacent to Rule Engine and Reporting work but are not required by, and are not resolved by, this Sprint.
- A Rule trait, registry, factory, or plugin dispatch mechanism. Two or three concrete Rules is not evidence a dispatch abstraction is justified, consistent with this platform's now five-times-applied "capability before abstraction" principle (the Rule trait question itself, Collector dispatch at GOV-004, Sprint 4's own routing decision, and now this question a second time for Rule Engine specifically). `GOVERNANCE.md`'s Crate Boundary Rules already states inline fulfillment as `RuleEngine`'s approved pattern, citing ADR-0010 and GOV-004 directly — this is an existing architectural directive this plan follows, not a new question requiring fresh approval.
- CLI wiring — remains the other named parallel track from `ENGINEERING_RELEASE_0.4.md`'s Recommendation, independent of this Sprint.
- Any change to `EvidenceCollector`, `ArchiveCollector`, `AssessmentService`'s public entry points, or `AssessmentInput`.

---

# Design Questions Requiring Technical Director Decision

Unlike Sprint 4, where `SPRINT4_IMPLEMENTATION_PLAN.md` could rely on Sprint 3's own precedent for most of its shape, this Sprint touches a part of the platform (`modiq-rules`) that has had exactly one Rule since inception — there is no existing "second Rule" precedent to follow. The four questions below are named here, in the abstract, for Technical Director review before implementation begins, mirroring how GOV-011's four questions were named before Sprint 4 Phase 3 began.

**Design Questions 1–3 are resolved — approved by the Technical Director, recorded formally as GOV-012 (see Governance Prerequisite, below).** Retained here in full for their reasoning; read the Resolution text in GOV-012's own entry as authoritative where the two differ in wording.

## Design Question 1 — `RuleEngine::evaluate`'s return shape

Today: `pub fn evaluate(&self, evidence: &[Evidence]) -> Option<RuleOutcome>` — one `Finding` and one `Recommendation`, or none. A multi-rule engine may need to return several. Two candidate shapes:

- **(a)** `Vec<RuleOutcome>` — zero, one, or several `(Finding, Recommendation)` pairs, one per matching Rule.
- **(b)** A single aggregated outcome — one call still returns at most one `Finding`/`Recommendation` pair, but a matching Rule may contribute to a shared Finding's `evidence_ids` or a shared Recommendation's `finding_ids` alongside others.

**Approved by Technical Director: (a), `Vec<RuleOutcome>`.** It requires no change to `Finding`/`Recommendation`'s existing one-Rule-per-Finding shape (`Finding.rule_reference` is a single `RuleReference`, not a collection), and it lets each Rule's Finding remain independently traceable to exactly the Rule and Evidence that produced it — the clearest reading of `RuleEngine.md`'s Traceability Management responsibility ("maintain conceptual relationships between... Rule, Evidence, Finding, Recommendation"). Option (b) would require inventing multi-Rule attribution for a single Finding, which no existing Runtime type supports today.

## Design Question 2 — Deterministic multi-Rule ordering

If more than one Rule matches, in what order are the resulting Findings/Recommendations produced? `AssessmentReport`'s content, and any future Reporting summarization, is only as deterministic as this ordering is explicit.

**Approved by Technical Director: explicit Rule declaration order.** A fixed, explicit Rule declaration order internal to `RuleEngine` (the order Rules are listed in `evaluate`'s own dispatch logic), not an order derived from Evidence's own arrival sequence — mirroring the same discipline `ArchiveReader::entries()` and `EvidenceCollector`'s directory traversal already apply (explicit sort/order imposed by the collector, never inherited from an unordered or incidentally-ordered source).

## Design Question 3 — Rule composition: does a specific Rule suppress the generic one?

If an Assessment's Evidence includes both `StructuralDuplication` and, say, `FileStructureAnalysis` items, does the existing generic `evidence-presence-rule` still fire (producing its own Informational Finding alongside the new Warning-level duplication Finding), or does a more specific Rule matching take precedence and suppress the generic one for the Evidence it covers?

**Approved by Technical Director: independent Rule composition, no suppression model.** Both fire, independently. `evidence-presence-rule` is evaluated against the full Evidence set (as today); the new `StructuralDuplication`-specific Rule is evaluated only against Evidence of that category. Neither Rule is aware of the other's applicability. This avoids inventing a precedence/suppression mechanism this Sprint's own evidence does not yet justify (only two concrete Rules exist even after this Sprint), and keeps each Rule's own logic genuinely independent — consistent with `RuleEngine.md`'s description of Rule Selection as determining "which Rules are applicable," plural and independent, not a single winner-takes-all choice. A future Sprint, with more Rules in evidence, may find real suppression/precedence needs are only then apparent — that decision is not preempted here.

## Design Question 4 — The four unused `modiq-report` scaffold types

`FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, and `ReportFormatter` (`crates/modiq-report/src/report/`) are each a one-line, zero-logic unit struct, unmodified since the platform's first foundational commit — the same age and shape as the eight `EngineAPI`/`modiq-rules` stub types GOV-004 retired. Unlike those eight, though, `GOVERNANCE.md`'s Crate Boundary Rules assigns their exact conceptual territory — report formatting, summarization, traceability output — to Reporting as a real, still-outstanding responsibility, not a competing architectural model direct composition already superseded. This Sprint's own mandate ("explainable technical assessments") touches this territory directly.

**This plan does not decide their fate, and — per Technical Director direction — neither does this Sprint.** Phase 1 (below) still investigates whether real Findings/Recommendations, once genuinely differentiated by this Sprint's new Rule, actually need a dedicated summarization/formatting/traceability layer beyond what `AssessmentReport` already exposes, or whether, as with the `EngineAPI` case, direct exposure turns out sufficient and these four types should be retired. The investigation and its recommendation remain in Sprint 5. **Acting on that recommendation — building any of the four types out, or retiring any of them — is explicitly deferred to a later sprint, after the Technical Director has reviewed the investigation.** Sprint 5 produces a recommendation only; no `modiq-report` code changes as a result of it.

## Design Question 5 — Does `FindingSeverity` conflate severity with kind?

Not anticipated when this plan was first drafted — raised by Technical Director review of the `FindingSeverity` semantic definitions Phase 1 produced (`GOV-012_AND_FINDINGSEVERITY_PREPARATION.md`, Part 2). Drafting those definitions surfaced that `BestPractice` does not sit on the same ordered "how urgent is this" scale `Error`/`Warning`/`Informational` do — it answers a different question, what *kind* of observation a Finding represents, independent of urgency. A real best-practice deviation could independently be more or less urgent than another, and the current model cannot express that.

**Not resolved by this plan.** Recorded as GOV-013 (Open) — see Governance Prerequisites, below, and `GOV-012_AND_FINDINGSEVERITY_PREPARATION.md`, Part 3. Technical Director decision: `FindingSeverity` remains unchanged for Sprint 5; the current model is provisionally accepted, not confirmed permanently correct; revisit after Sprint 5, once the Rule Engine has multiple concrete Rules operating in practice and this question can be evaluated against real implementation evidence rather than two Rules alone.

---

# Governance Prerequisites

## GOV-012 — Rule Evaluation Model

**Status: Approved by Technical Director.** Formal drafting for the Governance Register (Title/Status/Raised/Description/Question/Resolution, matching GOV-009 through GOV-011's own entry format) is prepared as a companion document, `GOV-012_AND_FINDINGSEVERITY_PREPARATION.md`, for a final review pass before insertion into `GOVERNANCE.md` — the same two-step discipline this project applied to GOV-011 (`PROPOSAL_GOV-011.md` drafted the resolution text; `GOVERNANCE.md` was amended only after review).

**Resolved:** Design Questions 1–3 above — return shape (`Vec<RuleOutcome>`), ordering determinism (explicit Rule declaration order), and composition/suppression behavior (independent, no suppression model) — the same class of architectural decision GOV-009/010/011 resolved for Evidence Collection, applied here to Rule Evaluation for the first time since the Rule Engine's original, single-Rule design.

**Implementation may not proceed on Phase 2 (Second Real Rule) until GOV-012's formal entry is reviewed and the `FindingSeverity` semantic definitions (Phase 1, below) are also reviewed and approved** — both gate implementation, per Technical Director direction; neither alone is sufficient.

Design Question 4 (the four `modiq-report` scaffold types) is deliberately not part of GOV-012 — it is a design investigation with a recommended, not decided, outcome. Per Technical Director direction, the investigation and recommendation remain in Sprint 5, but any action on that recommendation is deferred to a later sprint.

## GOV-013 — FindingSeverity Severity/Kind Conflation

**Status: Open, not Resolved — by design.** Formal entry prepared in `GOV-012_AND_FINDINGSEVERITY_PREPARATION.md`, Part 3, ready for insertion into `GOVERNANCE.md` immediately following GOV-012. Unlike GOV-012, this item does not gate Phase 2 on a *decision* — it gates Phase 2 only on the `FindingSeverity` definitions themselves being reviewed and approved as *provisionally* usable, with the conflation question left explicitly open rather than silently absent from the record. See Design Question 5, above, and Risks, below.

---

# Implementation Phases

## Phase 1 — Design Preparation

- **Objective** — formalize GOV-012's approved resolution for insertion into `GOVERNANCE.md`; define the semantic meaning of each `FindingSeverity` variant (`Error`, `Warning`, `Informational`, `BestPractice`) — what distinguishes one from another, and what criteria a Rule uses to choose among them — for Technical Director review, since no specification document defines this today; record the severity/kind conflation this drafting surfaced as GOV-013 (Open, provisionally accepted — Design Question 5); investigate whether `modiq-report`'s four unused scaffold types should eventually be built out or retired, producing a recommendation only (see Design Question 4).
- **Affected crates** — none. This phase produces governance drafting, specification-amendment drafting, and a recommendation — no implementation.
- **Completion criteria** — GOV-012's and GOV-013's formal entries and the `FindingSeverity` definitions are all drafted and submitted for Technical Director review; a written recommendation exists for Design Question 4. **Phase 2 does not begin until GOV-012, GOV-013, and the `FindingSeverity` definitions are all reviewed and approved** — GOV-013 stays Open by design; "reviewed and approved" for it means the Technical Director confirms it is correctly recorded as an open, provisionally-accepted question, not that it is resolved.

## Phase 2 — Second Real Rule: Structural Duplication

- **Objective** — implement a concrete Rule evaluating `EvidenceCategory::StructuralDuplication` Evidence specifically: real severity assignment (per GOV-012's resolution of what severity such a finding warrants), a Finding description that reflects what was actually observed (not a generic string), and a Recommendation whose action text is specific to investigating/resolving detected duplication.
- **Affected crates** — `modiq-rules`.
- **Completion criteria** — the new Rule is evaluated only against matching Evidence; every reachable outcome (no `StructuralDuplication` Evidence present; exactly one such item present; present alongside other categories) has a corresponding, tested code path.

## Phase 3 — Multi-Rule Evaluation Assembly

- **Objective** — implement `RuleEngine::evaluate`'s real dispatch across both Rules (the existing generic Rule and Phase 2's new one), per GOV-012's resolved return shape, ordering, and composition policy.
- **Affected crates** — `modiq-rules`; `modiq-engine` only if `AssessmentService::execute`'s internal handling of `RuleEngine::evaluate`'s return type requires a corresponding update (expected, given Design Question 1's recommended `Vec<RuleOutcome>` shape) — its own public signature does not change.
- **Completion criteria** — an Assessment whose Evidence matches both Rules produces both Findings and both Recommendations, correctly and deterministically ordered; an Assessment matching only one Rule behaves exactly as today's single-Rule case did.

## Phase 4 — Reporting: Investigation Only (not implementation this Sprint)

- **Objective** — Phase 1's Design Question 4 investigation and its recommendation *are* this phase's deliverable. **No `modiq-report` code is written or deleted this Sprint.** Per Technical Director direction, building out or retiring `FindingSummary`/`RecommendationSummary`/`TraceabilityReport`/`ReportFormatter` is deferred to a later sprint, after this investigation has been reviewed.
- **Affected crates** — none.
- **Completion criteria** — a written recommendation exists, reviewed by the Technical Director; `modiq-report`'s four scaffold types are untouched in source.
- **Completed:** `SPRINT5_PHASE4_REPORTING_INVESTIGATION.md` — checked construction sites for all four types (zero, unchanged) and, specifically, whether the Sandbox's own rendering needed to change to accommodate Sprint 5's new severity differentiation (it did not). Recommendation: retire all four, on the same evidentiary basis GOV-004 used. Not a decision — awaiting Technical Director review; `modiq-report` remains untouched in source.

## Phase 5 — Testing & Verification

- **Objective** — bring full workspace and Sandbox test coverage in line with the new capability.
- **Affected crates** — `modiq-rules`, `modiq-engine`, `modiq-report` (conditionally), `apps/sandbox` (test code only, if the Sandbox's own fixtures happen to produce `StructuralDuplication` Evidence — the archive fixture added at Sprint 4 Closeout does not, so this is not expected to be required).
- **Completion criteria** — `cargo fmt`, `cargo check --workspace`, `cargo test --workspace` all pass cleanly; Sandbox's own three equivalent commands pass cleanly.
- **Completed:** re-review of Phases 2–3's own test suite found one genuine coverage gap — nothing had proven `RuleEngine::evaluate`'s outcome order is independent of Evidence *arrival* order (only independent across repeated *identical* input). Added `evaluate_outcome_order_is_independent_of_evidence_arrival_order` (`engine.rs`), directly exercising GOV-012 Question 2's actual claim. `modiq-report` needed no change (Phase 4's investigation concluded no action this Sprint); no Sandbox fixture produces `StructuralDuplication` Evidence, confirming the "not expected to be required" prediction above.

---

# Expected Crate Changes

| Crate | Change | Notes |
|---|---|---|
| `modiq-rules` | A second concrete Rule; `RuleEngine::evaluate`'s return type changes (`Option<RuleOutcome>` → `Vec<RuleOutcome>`, pending GOV-012) | No new external dependency; no trait or dispatch abstraction |
| `modiq-engine` | `AssessmentService::execute`'s internal handling of `RuleEngine::evaluate`'s new return shape | No change to `execute`'s or `execute_from_assessment_input`'s own public signatures |
| `modiq-report` | None this Sprint | Investigation and recommendation only (Phase 4); building out or retiring the existing scaffold types is deferred to a later sprint per Technical Director direction |
| `modiq-runtime` | None anticipated | `FindingSeverity`'s four variants, `Finding`, `Recommendation`, and the Assessment relationship-resolution methods already support everything this plan requires |
| `modiq-collection` | None | Explicitly out of scope |
| `modiq-knowledge` | None | Explicitly out of scope |
| `apps/sandbox` | None anticipated | See Phase 5 |

---

# Testing Strategy

**Unit testing expectations.** The new Rule receives the same outcome-per-test discipline established for Collectors: one test per reachable outcome (matching Evidence present; absent; present alongside non-matching Evidence; multiple matching items in one Assessment).

**Multi-Rule composition testing.** Direct tests exercising `RuleEngine::evaluate` against Evidence sets matching zero, one, and both Rules, asserting the correct number, content, severity, and ordering of resulting Findings/Recommendations — this is the Rule Engine's own analog of Sprint 4's "routing testing" for `AssessmentService`.

**Regression testing.** The full workspace suite and the Sandbox's own separate suite must pass after every phase, exactly as every prior Sprint has required. `EvidenceCollector` and `ArchiveCollector`'s own test suites require zero changes, since neither is touched by this plan.

---

# Determinism Validation

- **Rule ordering.** Repeated evaluation of identical Evidence must produce Findings/Recommendations in identical order and content, per Design Question 2's resolution — verified directly, not assumed.
- **Content vs. identity.** As with every other Runtime entity, each Finding/Recommendation's own identity is freshly assigned per evaluation; determinism is judged by content and order, not by incidental identity, the same convention applied without exception elsewhere in this platform.
- **Severity determinism.** A given Evidence set must always be assigned the same `FindingSeverity` by the same Rule — no non-deterministic or context-sensitive severity assignment within what this Sprint's two Rules cover.

---

# Security Validation

Not applicable in the sense Sprint 4's Security Validation section was: this Sprint introduces no new external dependency, no new adversarial input surface, and reads no untrusted content — it evaluates `Evidence` that Collection has already produced and validated. The only relevant discipline is the one `RuleEngine.md` already states as a first-class execution principle: **Evidence-Based** ("Rules never operate on assumptions or unsupported observations") — the new Rule's Finding content must be derived only from the `Evidence` it evaluates, never from assumptions about what a `StructuralDuplication` item's absence or presence implies beyond what GOV-011's own policy text already establishes.

---

# Documentation Updates Expected

- `docs/engineering/GOVERNANCE.md` — new Governance Register entry (candidate GOV-012), resolved during Phase 1; the Rule Engine's Crate Boundary Rules entry updated once a second real Rule exists, if its current single-Rule framing needs a factual correction.
- `docs/implementation/CrateRoadmap.md` — `modiq-rules`'s (and conditionally `modiq-report`'s) maturity entries updated; a new revision history entry.
- `docs/engineering/ENGINEERING_LOG.md` — a Sprint 5 entry per phase, following the established Status/Affected Crates/Affected Documents/Notes structure.
- `docs/architecture/RuleEngine.md` — likely unchanged, being implementation-independent by design; amended only if this Sprint's implementation reveals its conceptual model does not actually accommodate something this plan assumes (to be reported, not silently resolved, if found).
- An Engineering Release document (`ENGINEERING_RELEASE_0.5.md`, following the established naming and location precedent) at Sprint close, per the Technical Director's own directive that future sprint closures formally include repository reconciliation and a documentation audit.

---

# Risks

- **Design Question 4's outcome sizes Phase 4 very differently depending on direction** — moot for Sprint 5 itself now that Technical Director direction has deferred any action to a later sprint, but the sizing difference still applies whenever that sprint is scoped.
- **`FindingSeverity`'s real-world semantic assignment is partly a product judgment, not a pure engineering one.** Engineering can implement whatever severity a Rule is told to assign; *which* severity a given Rule warrants was confirmed by Technical Director review during Phase 1 (`StructuralDuplication` → `Warning`), not left to Engineering's own guess.
- **GOV-013 remains genuinely open, not a formality.** `FindingSeverity`'s severity/kind conflation is real and provisionally accepted, not resolved — Sprint 5's second Rule (`StructuralDuplication`) does not itself exercise `BestPractice`, so this Sprint adds no new evidence toward resolving GOV-013 either way. Whoever scopes the Sprint that introduces a genuinely convention/quality-flavored Rule should expect to confront this question with real evidence for the first time.
- **`RuleEngine::evaluate`'s signature change is the first change to any public `modiq-rules` API since Sprint 1.** Low risk in isolation (only `modiq-engine`'s internal handling depends on it), but worth naming since GOV-008 (`AssessmentService`'s own API evolution) shows this platform treats public signature changes deliberately, not incidentally.

---

# Rollback Considerations

This Sprint is additive except for `RuleEngine::evaluate`'s own signature. Reverting Phase 3 alone restores the single-`Option<RuleOutcome>` shape and the second Rule becomes unreachable dead code, not a breaking change to any other crate. Reverting Phase 2 removes the new Rule entirely; `modiq-rules` returns to exactly its Sprint 4 state. Reverting Phase 4 (whichever branch it took) either removes newly-built Reporting content or restores the four scaffold types, both trivially revertible via git. No data migration or persisted state applies at any phase.

---

# Success Criteria

Sprint 5 is complete only when all of the following are true:

- GOV-012's and GOV-013's formal entries are reviewed and recorded in the Governance Register, and the `FindingSeverity` semantic definitions are reviewed and approved — all three, not any subset, before Phase 2 begins. GOV-013 is recorded Open, provisionally accepted, by design — its completion criterion is that it is correctly on the record, not that it is resolved.
- A second real, tested Rule exists, evaluating `EvidenceCategory::StructuralDuplication` with real severity (assigned per the approved `FindingSeverity` definitions), description, and recommendation content.
- `RuleEngine::evaluate` correctly dispatches across both Rules, deterministically, per GOV-012's resolved shape and ordering.
- Design Question 4 has a recorded, Technical-Director-reviewed recommendation. Per Technical Director direction, no action on that recommendation — building out or retiring `modiq-report`'s scaffold types — is required, or permitted, this Sprint.
- No Rule trait, registry, factory, or plugin mechanism exists anywhere in the workspace.
- `EvidenceCollector`, `ArchiveCollector`, and both `AssessmentService` public entry points' signatures are unchanged in behavior (only `execute`'s internal handling of the Rule Engine's new return shape changes).
- `cargo fmt`, `cargo check --workspace`, and `cargo test --workspace` all pass cleanly across the full workspace; the Sandbox's separate workspace passes its own equivalent three commands.
- Repository reconciliation and a documentation audit are performed at Sprint close, per the Technical Director's standing direction from Sprint 4 Closeout.

---

# Completion Checklist

☑ Governance prerequisite decided — GOV-012 approved by Technical Director (`Vec<RuleOutcome>`; explicit Rule declaration order; independent composition, no suppression)

☑ GOV-012's formal Governance Register entry reviewed and inserted into `GOVERNANCE.md`

☑ Severity/kind conflation identified and raised — GOV-013 drafted, Status Open (not a decision to make; recorded as a tracked, provisionally-accepted question per Technical Director direction)

☑ GOV-013's formal Governance Register entry reviewed and inserted into `GOVERNANCE.md`, Status Open

☑ `FindingSeverity` semantic definitions drafted, reviewed, and approved as provisionally usable for Sprint 5 — recorded in `DataModel.md` v1.1.0

☑ Second real Rule implemented — `EvidenceCategory::StructuralDuplication` evaluated with real severity (per the approved definitions), description, and recommendation content (`StructuralDuplicationRule`, Phase 2)

☑ Multi-Rule dispatch implemented in `RuleEngine::evaluate`, per GOV-012's resolved shape/ordering/composition policy — returns `Vec<RuleOutcome>`; explicit declaration order (`EvidencePresenceRule`, then `StructuralDuplicationRule`); independent composition confirmed by test (Phase 3)

☑ No dispatch abstraction introduced — no trait, registry, factory, or plugin mechanism added anywhere; `RuleEngine::evaluate` is a fixed sequence of `if let` checks

☑ Design Question 4 investigated and a recommendation recorded (`SPRINT5_PHASE4_REPORTING_INVESTIGATION.md`: retire all four) — **building out or retiring `modiq-report`'s scaffold types remains explicitly deferred to a later sprint, not performed now**

☑ `AssessmentService`'s public entry point signatures unchanged in behavior — only `execute`'s internal loop over `Vec<RuleOutcome>` changed

☑ Determinism validation complete — repeated-evaluation test passes; Rule ordering imposed explicitly by declaration order, not inherited from Evidence order

☑ Tests passing — `cargo fmt`, `cargo check --workspace`, `cargo test --workspace` clean (162/162); Sandbox's own three commands clean (6/6)

☑ Engineering Log updated — Sprint 5 entries added following the established entry format

☑ Repository reconciliation and documentation audit performed at Sprint close, producing `ENGINEERING_RELEASE_0.5.md`

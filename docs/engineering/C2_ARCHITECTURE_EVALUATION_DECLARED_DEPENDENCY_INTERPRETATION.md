# C2 Architecture Evaluation — Shared-`EvidenceCategory` Rule Composition

| Property | Value |
|---|---|
| **Document** | C2_ARCHITECTURE_EVALUATION_DECLARED_DEPENDENCY_INTERPRETATION.md |
| **Project** | modIQ |
| **Capability Addressed** | C2 — Declared Dependency Interpretation (`CAPABILITY_DEFINITION_C2_DECLARED_DEPENDENCY_INTERPRETATION.md`) |
| **Question Evaluated** | How should the repository treat a second Rule interpreting an already-interpreted `EvidenceCategory` (`XmlInspection`), given GOV-012, `SPRINT12_ARCHITECTURAL_RESOLUTION.md`, the current Rule Engine architecture, and the repository's existing Rule-composition model? |
| **Origin** | `CAPABILITY_DEFINITION_C2_DECLARED_DEPENDENCY_INTERPRETATION.md`'s own "Next Required Repository Artifact" conclusion, itself independently re-verified and confirmed by that document's own Repository Validation Review, Targeted Reconciliation, and Post-Reconciliation Verification (committed `7c7faf8`). |
| **Status** | **Architecture Evaluation only. No Architectural Resolution has been performed. No Rule, matching logic, prefix, severity, Mod Health Dimension, Repair Recipe, Evidence schema, or implementation approach has been designed or chosen. No ADR or Governance Register entry has been created or modified.** |

---

## 1. Evaluation Scope

Covers only the architectural question named above: whether, and on what basis, the repository's existing Rule-composition architecture (GOV-012) already governs the case of two Rules independently interpreting the same `EvidenceCategory`, or whether that case requires new architectural work before a second such Rule may be added.

**Deliberately excluded**, matching the scope discipline `INITIATIVE_2_ARCHITECTURE_EVALUATION.md` and `INITIATIVE_4_ARCHITECTURE_EVALUATION.md` both already establish for this document type:

- Any Rust type, matching logic, prefix string, or Evidence schema change.
- Any severity, Mod Health Dimension, or Repair Recipe assignment.
- Any transport, presentation, CLI, or Sandbox concern (already settled as unaffected by `CAPABILITY_DEFINITION_C2...md`'s own Architectural Boundaries section, not reopened here).
- Cross-mod dependency resolution (C5) and any other Capability Portfolio Assessment candidate.
- An Architectural Resolution of the question this document evaluates.

---

## 2. Repository Evidence

Re-read directly this session, independent of the Capability Definition's own citations:

- **`docs/engineering/GOVERNANCE.md`**, GOV-012 entry, re-read in full, Question 3 re-grepped verbatim for exact wording.
- **`docs/engineering/SPRINT12_ARCHITECTURAL_RESOLUTION.md`**, re-read in full (all 13 sections), not only §5/§7/§8 as in the Capability Definition's own citations.
- **`docs/implementation/SPRINT12.md`** — the originating Capability Identity document `SPRINT12_ARCHITECTURAL_RESOLUTION.md` itself reconciles and amends, read in full, including §8.2 (Rule Composition) and §11 (Risks).
- **`docs/governance/PROJECT_STATUS.md`** — grepped for "Sprint 12," to independently verify that document's own current standing.
- **`docs/architecture/RuleEngine.md`**, re-read in full: the six Rule Engine responsibilities (Rule Selection, Evidence Evaluation, Finding Generation, Recommendation Generation, Traceability Management, Explainability) and the Execution Principles (Deterministic, Evidence-Based, Knowledge-Driven, Explainable, Traceable, Version Aware).
- **`crates/modiq-rules/src/rules/engine.rs`**, re-read in full: `RuleEngine::evaluate`'s actual dispatch code, not a paraphrase of it.
- **`crates/modiq-rules/src/rules/version_compatibility_rule.rs`**, **`structural_duplication_rule.rs`**, **`runtime_load_failure_rule.rs`**, **`evidence_presence_rule.rs`** — re-read in full, checking specifically for any shared state, mutation, or inter-Rule dependency.
- **`docs/implementation/RuntimeInvariants.md`** — re-read in full; grepped specifically for any invariant governing Rule cardinality per category. None found (see §4).
- **`docs/architecture/EvidenceCollection.md`** — grepped for any category-to-Rule cardinality constraint. None found.
- **`docs/architecture/DataModel.md`**, Finding Severity section, re-read for the per-Rule (not cross-Rule) severity discipline.
- **`docs/constitutional/Glossary.md`** — checked for any "Rule Composition" or "Interpretation" definition beyond what `RuleEngine.md` and Sprint 12 already establish. None found beyond the existing Rule and Rule Engine entries.
- **`docs/engineering/CAPABILITY_DEFINITION_C2_DECLARED_DEPENDENCY_INTERPRETATION.md`** (committed `7c7faf8`) — read in full as the origin of this evaluation; its own citations independently re-verified against primary source rather than trusted, per this document's own governing instruction.

---

## 3. The Architectural Question

`SPRINT12_ARCHITECTURAL_RESOLUTION.md` §8 states, in its own words: *"no historical decision has yet tested two Rules, or two recognized fact-shapes, sharing one already-interpreted `EvidenceCategory`... a disciplined extrapolation of the two-stage filter pattern... not a fifth confirmed data point... strong guidance, not settled precedent."* C2 would be exactly this case: a second Rule filtering `EvidenceCategory::XmlInspection`, alongside `VersionCompatibilityRule`.

This document evaluates what that caveat actually means for the repository's current architecture — whether it names a genuine gap in the Rule Engine's composition model, a genuine gap only in the *evidentiary confidence* behind an already-adequate model, or something else — and what, if anything, should follow before a second `XmlInspection`-interpreting Rule is authorized.

---

## 4. Current Architecture Baseline

**Repository fact, `RuleEngine::evaluate` (`engine.rs`, re-read in full this session).** The dispatch mechanism is four sequential, independent statements of the identical shape:

```
if let Some(outcome) = SomeRule.evaluate(evidence) { outcomes.push(outcome); }
```

Every Rule receives the same, full, immutable `&[Evidence]` slice. No Rule's `evaluate` call receives the prior Rule's output, consumes or removes matched Evidence, or has any code-level dependency on another Rule's presence, absence, or result. `outcomes` is a flat `Vec<RuleOutcome>`, appended to in fixed declaration order; nothing about its later contents depends on Rule ordering beyond that ordering.

**Inference, not repository fact, drawn directly from the above:** mechanically, adding a fifth `if let` line for a new `XmlInspection`-filtering Rule introduces no new coupling, no shared mutable state, and no code path by which the new Rule's presence could alter any existing Rule's behavior, or vice versa. Whatever architectural risk exists in this case, it is not a risk visible in the dispatch mechanism itself.

**The relationship between this code-level fact and the architectural question, stated explicitly:** the dispatch trace above demonstrates *mechanical* independence — the code contains no coupling. GOV-012 (below) is what establishes *architectural* independence — the governance-tier principle that Rules' conclusions are entitled to compose without suppression. The code confirms that the implementation faithfully realizes that already-adopted principle; it is not itself the source of the guarantee, and mechanical independence alone does not establish that two Rules' conclusions remain jointly explainable (§6, §8).

**Sprint 12's own document status, checked directly rather than assumed.** Both `SPRINT12.md` and `SPRINT12_ARCHITECTURAL_RESOLUTION.md` carry, in their own header tables and Document Status footers, the text "Proposed... awaiting Chief Architect approval/review before Sprint 12 closes/closeout" — never updated in either document's own text. Independently checked against `docs/governance/PROJECT_STATUS.md`, which states directly: "Sprint 12 — Complete (Capability Scaling Architecture)." `PROJECT_STATUS.md` is the repository's own living status record and is treated here as the authoritative current source on this specific question; the two source documents' own unsynchronized status text is a documentation-currency gap, not a live approval question, consistent with this repository's own recorded tolerance for the same pattern elsewhere (`PROJECT_STATUS.md`'s own text names an equivalent gap for `modiq-common` references). Sprint 12 is accordingly treated as settled, adopted repository history throughout this evaluation, as it was before this check — this paragraph records the verification, not a change in that treatment.

**Repository fact, GOV-012 Question 3 (`GOVERNANCE.md`, re-grepped verbatim this session):** *"Rules compose independently; no suppression model exists. Every Rule is evaluated against whichever Evidence it applies to, regardless of whether another Rule also matches related or overlapping Evidence. An Assessment whose Evidence matches both the existing generic Rule and a new category-specific Rule produces both Rules' Findings and Recommendations, not one at the expense of the other."*

**Repository fact, worth stating precisely because it is easy to read past:** GOV-012's own illustrative grounding for this resolution is explicitly *"the existing generic Rule and a new category-specific Rule"* — `EvidencePresenceRule` (which matches unconditionally, on any Evidence, with no content-shape question of its own) alongside one category-specific Rule. This is the only composition shape GOV-012's own text actually illustrates. It is not, on its own words, an illustration of two *category-specific* Rules — each purporting to ask its own specific question of the same category's content — sharing one category.

**Repository fact, re-confirmed this session:** this generic-plus-specific overlap is not merely illustrated in GOV-012's text — it is already real, shipped, and tested. `RuleEngine::evaluate`'s own test suite (`engine.rs`, `evaluate_dispatches_all_four_rules_independently_when_all_match`) demonstrates a single Evidence item (e.g., `duplication_evidence()`) referenced by more than one Finding at once: `EvidencePresenceRule`'s Finding (which references it because it references *every* Evidence item) and `StructuralDuplicationRule`'s Finding (which references it because it specifically matches that item's category). This is the same overlap shape `CAPABILITY_DEFINITION_C2...md` itself already found and used to correct `CAPABILITY_PORTFOLIO_ASSESSMENT.md`.

**The distinction this evaluation turns on:** the tested case (generic Rule + one specific Rule) never puts two *specific* conclusions about the identical fact into tension with each other, because the generic Rule's own Finding is deliberately undifferentiated — it asks no question and reaches no fact-specific conclusion. C2 would be the first case in this repository's history where **two Rules, both using the two-stage filter pattern, both purporting to reach a specific, differentiated conclusion, examine the same closed set of Evidence** (every `XmlInspection` item in the Assessment). Sprint 12's own caveat is scoped to exactly this — re-read verbatim: *"Every category-specific Rule to date — `StructuralDuplicationRule`, `VersionCompatibilityRule`, `RuntimeLoadFailureRule` — has been the first and only interpreter of its own category."* `EvidencePresenceRule` is correctly excluded from that list, because it is not category-specific.

---

## 5. Question 1 — Does the Capability Identity Model Already Classify This Case?

**Observation.** Applying `SPRINT12_ARCHITECTURAL_RESOLUTION.md` §5's own Interpretation Axis test, mechanically and literally, to C2: *"Does interpreting the fact require a judgment — a question asked of the Evidence — no existing Rule already makes? ... New judgment → Expansion (sibling Rule, using the two-stage filter pattern...)."* Declared-dependency content is a fact no existing Rule asks any question of (independently re-confirmed against `version_compatibility_rule.rs`, which filters only on the `descVersion`-declaration prefix, never the dependency-declaration prefix). This is unambiguously a "new judgment" under the test's own wording. The test's three branches (Enrichment / Expansion / no verdict yet) do not distinguish "first Rule ever to interpret this category" from "second Rule to interpret this category, on a different fact" — the "New judgment → Expansion" branch is stated generally, with no category-cardinality qualifier anywhere in its text.

**Conclusion.** The classification procedure itself is not ambiguous here: mechanically applied, it answers **Expansion** — a new, sibling Rule, using the two-stage filter pattern already established. This matches `CAPABILITY_DEFINITION_C2...md`'s own Existing Repository Patterns finding, independently re-derived here rather than trusted from that document.

**This narrows, rather than resolves, the architectural question.** If the classification procedure already gives a clean, unambiguous answer, then Sprint 12's own "genuinely untested" caveat is not describing a gap in the classification procedure — it is describing something else. Section 6 investigates what.

---

## 6. Question 2 — What Does Sprint 12's "Genuinely Untested" Caveat Actually Refer To?

**Observation, re-derived from §4 above, not assumed from Sprint 12's own framing.** Two candidate readings of the caveat exist, and repository evidence supports distinguishing them rather than treating them as one:

**Reading A — a mechanical/structural risk in `RuleEngine::evaluate`'s own dispatch.** Checked directly against the dispatch code (§4): no such risk is visible. Two Rules matching the same category produce two independent `RuleOutcome`s from two independent function calls over an immutable, shared input; nothing in the mechanism itself can make the addition of a second `XmlInspection`-filtering Rule behave differently from any other historical Rule addition. **This reading is not supported by the code.**

**Reading B — an unconfirmed guarantee about content-shape disjointness within one category, only ever implicit because it was previously structurally impossible to violate.** Every category-specific Rule to date has been the *only* interpreter of its own category (re-confirmed, §4) — meaning each one's two-stage filter has, by simple absence of a second interpreter, never had to actually coexist with a sibling filter over the same category's Evidence. The two-stage filter's own second stage (a content-shape match — a description prefix, in every existing instance) is what would need to remain disjoint from a sibling Rule's own second-stage filter for both Rules' conclusions to stay independently correct and separately explainable. **Nothing in `RuleEngine.md`, `GOVERNANCE.md`, or Sprint 12 states this disjointness as an explicit requirement, names who is responsible for verifying it, or names any mechanism (a test, an invariant, a review step) that checks it.** It has functioned as an unstated precondition of the two-stage filter pattern's own correctness, satisfied automatically until now because no category has ever had two interpreters to potentially collide.

**Conclusion.** Reading B is the evidence-supported one. Sprint 12's caveat does not identify a defect in the dispatch mechanism (Reading A, unsupported) or in the Interpretation Axis's own classification test (§5, which answers cleanly). It identifies that **the two-stage filter pattern's disjointness has never been an articulated, checked property — only an accidental consequence of category exclusivity — and C2 is the first capability to make that accident stop holding automatically.**

This is a distinguishable engineering inference, not a directly-stated repository fact: no document states this explanation of the caveat in these terms. It is offered here as this evaluation's own synthesis of §4's evidence, presented for scrutiny rather than asserted as settled.

**A further, related and explicitly out-of-scope observation, named but not pursued:** `RuleEngine.md`'s Explainability principle requires that "every conclusion should be understandable" and that the Rule Engine "should never produce opaque or unsupported outcomes." If two category-specific Rules' content-shape filters were ever to overlap on the identical Evidence item — a design-time correctness question for whoever writes the second Rule's matching logic, explicitly excluded from this evaluation — the consequence would be two independently-reached, specific conclusions about one fact, coexisting under GOV-012's own no-suppression model. Whether that consequence would itself violate Explainability, or would simply be two legitimate, independently-traceable conclusions (each correctly attributed to its own `rule_reference`), is not evaluated here; it is named in §8 as a risk, not resolved.

---

## 7. Competing Alternatives

Five repository-supported alternatives exist for how to proceed, given §5's and §6's findings. Presented without preference beyond what evidence and consequence directly support; none is adopted here. Two (D, E) are named and, in D's case, immediately dismissed on direct evidence; three (A, B, C) are carried forward with full evidence-for/evidence-against analysis.

### Alternative A — Proceed under GOV-012 as sufficient authority; no new architectural work required

**Position.** GOV-012 Question 3 is a Resolved Governance Register item. Its text, read at face value ("Rules compose independently... regardless of whether another Rule also matches related or overlapping Evidence"), is written generally enough to already cover two category-specific Rules sharing a category, even though its own illustration does not depict that exact case. Sprint 12's caveat is then read as ordinary epistemic caution appropriate to any first real instance of an already-general rule, not as evidence the rule itself is inadequate.

**Evidence supporting:** §4's dispatch-mechanism trace finds no structural risk (Reading A rejected). §5 finds the classification procedure unambiguous. GOV-012 is Resolved, not Open, and Documentation Authority (`GOVERNANCE.md`) ranks a Resolved Governance Register item above an Engineering Log-tier document for architectural authority — and `SPRINT12_ARCHITECTURAL_RESOLUTION.md`, while a dedicated Architectural Resolution in its own right rather than a mere Engineering Log entry, elaborates GOV-012 without ever purporting to reopen or supersede it (re-confirmed: Sprint 12 §12's own Repository Impact table records "Collector Composition Architecture, Rule Engine Architecture (GOV-012)... reviewed directly against this reconciliation; none required amendment").

**Evidence against:** Sprint 12's own text is explicit that the extrapolation to this exact case is "not a fifth confirmed data point" and states the caveat "exactly as strongly as it should be treated" — language that reads as a deliberate withholding of full confidence, not merely a disclaimer offered out of excess caution. Treating GOV-012's general wording as dispositive would mean Sprint 12's own, later, more specific caveat is effectively ignored rather than engaged with — in tension with this repository's own standing discipline (`PROJECT_HANDOFF_v1.1.md` §6, Principle 4: "specification-writing, and historical-evidence derivation, are themselves sources of architectural findings") of treating a document's own later, closer scrutiny as informative, not superseded by an earlier, broader resolution it elaborates.

**Architectural consequences:** none — this alternative changes nothing about the Rule Engine, GOV-012, or Sprint 12. The next Rule would be added exactly as every prior Rule has been.

**Repository compatibility:** fully compatible with every existing crate boundary, invariant, and dispatch mechanism (§4).

### Alternative B — Name and adopt an explicit content-shape disjointness expectation before or alongside the next category-specific Rule

**Position.** §6's Reading B identifies a real, previously-unarticulated precondition of the two-stage filter pattern's own correctness. Rather than leaving it implicit indefinitely, this alternative would have a future Architectural Resolution state it explicitly — e.g., that two category-specific Rules sharing an `EvidenceCategory` must have content-shape filters that do not both match the same Evidence item — as a standing expectation for Rule authorship, without inventing any new mechanism, invariant, or dispatch structure to enforce it.

**Evidence supporting:** the precondition is real and independently re-derivable from source (§4, §6) even though no document states it. `RuleEngine.md`'s Explainable execution principle already establishes the general value this expectation would protect ("no conclusion should exist without supporting traceability"). Naming an implicit assumption explicitly, once it stops holding automatically, matches this repository's own recorded discipline (`PROJECT_HANDOFF_v1.1.md` §6, Principle 9: documentation staleness is closed by discipline, not left unacknowledged) more closely than leaving it unstated a second time now that it is known.

**Evidence against:** this is architecture-adjacent housekeeping, not a change any current evidence shows is *necessary* rather than merely tidy — no repository document, including Sprint 12 itself, actually requires this specific articulation as a precondition for proceeding; Sprint 12 names the gap as a caveat, not as a blocking requirement. Introducing a new standing expectation via Architectural Resolution for a precondition no Rule has yet been shown to violate risks the same speculative-machinery pattern this repository's own "capability before abstraction" principle (`PROJECT_HANDOFF_v1.1.md` §6, Principle 1) has repeatedly found unjustified absent a concrete forcing function — and no concrete violation exists yet to force it.

**Architectural consequences:** adds a documented expectation (most plausibly to `RuleEngine.md` or `GOVERNANCE.md`'s GOV-012 entry, as a Level 2 Clarification per `GOVERNANCE.md`'s own Change Categories, since it "clarifies" without changing invariant, API, or lifecycle behavior) but no code, invariant, or dispatch change. Whether Level 2 (Clarification) or Level 3 (Behavioral, if the expectation were instead framed as an enforced check) applies is itself a question for whatever document adopts this alternative, not resolved here.

**Repository compatibility:** fully compatible; adds documentation, changes no existing behavior.

### Alternative C — Treat C2 as the forcing function that closes Sprint 12's own named limitation, formally

**Position.** `SPRINT12_ARCHITECTURAL_RESOLUTION.md` §10 (Architectural Limitations) already names this exact gap as a known limitation of that document, not a settled matter: *"Rule Composition's 'second Rule, same category' case remains genuinely untested... disclosed as such, not resolved by this reconciliation."* Under this alternative, C2 is recognized as the concrete forcing function `PROJECT_HANDOFF_v1.1.md` §6 Principle 2 requires before a model changes or graduates — and the correct next step is a dedicated Architectural Resolution that formally examines this case, in the same disciplined, adversarial manner Sprint 12 itself applied to its own seven historical decisions (§6 of that document), and records the outcome as either a confirmed fifth Interpretation-Axis instance (if C2's own eventual implementation, once designed and shipped, is found on inspection to compose safely) or a genuine correction to the model (if it is not).

**Evidence supporting:** this is the most literal reading of Sprint 12's own words — it names the gap, names it as unresolved by itself, and this repository's own standing discipline (adversarial verification "finds real errors that good-faith review... does not," `PROJECT_HANDOFF_v1.1.md` §6) has repeatedly preferred deliberately re-checking a model against a real instance over assuming an untested extrapolation holds. This is also the only alternative that produces a durable, reusable answer for any future third or fourth category-specific Rule, rather than settling only C2's own case.

**Evidence against:** this alternative, taken literally, could not be fully discharged by an Architecture Evaluation or even a single Architectural Resolution performed *before* the second Rule exists — Sprint 12's own Historical Validation table (§6 of that document) validated its model only against decisions already made and already shipped, never against a hypothetical one. A resolution that tries to "confirm" composition safety in the abstract, before any real second category-specific Rule exists to inspect, would risk re-creating exactly the failure mode Sprint 12's own §10 already warns about for the *Introduction* test ("one instance is a real precedent, not yet a statistically robust pattern") — declaring a model validated on a sample of zero real instances of the specific case in question. `SPRINT12.md` §11 (Risks) states this concern directly, in its own words, as a deliberately accepted risk of Sprint 12's own scope: *"This Sprint does not, and should not, attempt to resolve the untested 'second Rule, same category' case beyond disciplined extrapolation — attempting to manufacture false certainty here would be worse than naming the gap."* Read precisely, this warns against *prospective* certainty-manufacturing — confirming safety in the abstract, absent any real instance — not against ever confirming the model once a real instance exists to check it against; it corroborates, rather than contradicts, this same "Evidence against" point.

**Architectural consequences:** potentially the largest of the three — could result in an amendment to `SPRINT12_ARCHITECTURAL_RESOLUTION.md` itself (adding an eighth Historical Validation row once a real instance exists) or a standalone successor document, following the precedent that document's own §7/§8 guidance sections already set for recording refinements without reopening the underlying model.

**Repository compatibility:** fully compatible; this is the shape of work Sprint 12 itself anticipated needing.

### Alternative D — Treat Sprint 12's caveat as non-binding, pending its own document status

**Position.** `SPRINT12.md` and `SPRINT12_ARCHITECTURAL_RESOLUTION.md` both still carry "Proposed / awaiting Chief Architect approval" text in their own headers and Document Status footers. Under this alternative, that caveat carries no architectural weight until formally ratified, and C2 should be resolved against GOV-012 (a genuinely Resolved Governance Register item) alone.

**Immediately dismissed:** `docs/governance/PROJECT_STATUS.md` states directly, "Sprint 12 — Complete (Capability Scaling Architecture)" — the repository's own living status record, independently checked (§4), confirming Sprint 12 was approved and closed out in substance. The two source documents' own unsynchronized status text does not support treating Sprint 12's caveat as non-binding.

### Alternative E — Treat this as a Level 2 Clarification, resolved during ordinary planning

**Position.** Under `GOVERNANCE.md`'s own Change Categories, this question could be characterized as a Level 2 Clarification — the same tier Alternative B's own "Architectural consequences" entry names as one possible shape for a disjointness expectation — resolvable during ordinary Implementation Planning rather than through a dedicated Architecture Evaluation → Architectural Resolution cycle at all.

Named here as a repository-supported possibility, not evaluated or adopted — whether the shared-`EvidenceCategory` question rises above Level 2 (given its connection to `RuleEngine.md`'s Explainability principle, §6, §8) is itself an open question this document does not resolve.

---

## 8. Unresolved Risks

Named here, not resolved, per this document's own scope:

- **The explainability question §6 raises but does not answer:** if a future Rule's own content-shape filter were ever to overlap another category-specific Rule's, would the resulting two independent, specific conclusions about one fact violate `RuleEngine.md`'s Explainability principle, or merely be two legitimate, separately-traceable conclusions? No repository document currently answers this, because the case has never occurred.
- **No invariant, test, or review step currently verifies content-shape disjointness within a shared category.** This is not a defect in any single Rule — every existing Rule's own filter is trivially disjoint from every other, since none shares a category with another specific Rule — but it means the property Alternative A implicitly relies on has never actually been checked by anything, only guaranteed by there being nothing to check it against.
- **Severity and Mod Health Dimension consistency across two Rules interpreting the same category is unaddressed by any repository document**, and is explicitly out of this evaluation's scope (per the originating task's own constraint) — named here only so a future Architectural Resolution does not have to rediscover that the question exists.
- **Whether Alternative C's own validation could ever be performed *before* a real second instance exists is itself unresolved** — Sprint 12's own precedent (§6 of that document) validated retrospectively, against real, shipped decisions, never prospectively.
- **This evaluation's own Reading B (§6) is an engineering inference, not a repository-stated fact.** It is the most direct explanation this session could construct from re-derived evidence, but no document states it in these terms; a future reviewer should treat it as a hypothesis to check, not as settled.

---

## 9. Capability Boundaries

Unaffected by, and not reopened by, this evaluation — confirmed directly against `CAPABILITY_DEFINITION_C2...md`'s own Architectural Boundaries and Explicit Exclusions, both independently re-checked this session and found accurate:

- `XmlCollector` and the Collector Contract — untouched; this is an Interpretation-Axis question exclusively.
- `EvidenceCategory`'s closed set — untouched; `DependencyResolution` remains dormant and irrelevant here.
- `modiq-report`, `modiq-storage`, `modiq-cli`, `apps/console`, `apps/sandbox` — untouched; every consumer's own generic transport (re-verified in the Capability Definition's own Post-Reconciliation Verification) is unaffected by how many Rules interpret one category.
- GOV-013 (`FindingSeverity` Severity/Kind Conflation) — not reopened; named in §8 only as an adjacent, unaddressed question.
- Cross-mod dependency resolution (C5), Confidence (C6, Initiative 4), and every other Capability Portfolio Assessment candidate — unaffected.
- The Collector Composition Architecture's own five-condition extraction threshold (`SPRINT12_ARCHITECTURAL_RESOLUTION.md` §7) — a Collection-Axis concern, unrelated to the Interpretation-Axis question evaluated here.

---

## 10. Recommended Next Architectural Direction

**Recommendation, not resolution.** Repository evidence most directly supports proceeding toward **Alternative C**, on narrow, stated grounds: it is the only alternative that engages Sprint 12's own explicit, self-named limitation on its own terms, rather than either overriding it (Alternative A, in tension with Sprint 12's own deliberately withheld confidence) or acting on it prematurely, absent a concrete instance to check (a risk Alternative B does not fully avoid either, since it would also codify an expectation before any real Rule has tested it). **The confirmation this recommends is retrospective, against a real, implemented Rule — not a prospective gate to be discharged before implementation may begin**, consistent with `SPRINT12.md` §11's own caution against manufacturing certainty in the abstract (§4, §7): the second Rule's own eventual implementation is itself the concrete instance Sprint 12's Historical Validation methodology requires before a model can be validated, not a precondition C2's implementation must wait on. This recommendation does not itself determine what an Alternative-C-shaped Architectural Resolution should conclude, nor does it foreclose that such a Resolution could reasonably adopt Alternative B's disjointness expectation as part of its own findings, once a real second Rule exists to check it against.

This recommendation is offered for whatever Architectural Resolution follows to accept, reject, or revise — it is not adopted, and no alternative is foreclosed, by this document.

---

## Evaluation Boundaries

**Questions this evaluation answered:** whether the Capability Identity model's own classification procedure already resolves C2's classification (§5 — yes, cleanly, as Expansion); what Sprint 12's "genuinely untested" caveat actually refers to, distinguishing a mechanical-risk reading (§6, Reading A, not supported by the dispatch code) from a disjointness-precondition reading (§6, Reading B, evidence-supported); what competing alternatives exist for proceeding, with their respective evidence and consequences (§7); what risks remain regardless of which alternative is eventually chosen (§8).

**Questions intentionally not addressed:** which alternative should be adopted (§10 recommends without deciding); any Rule design, matching logic, prefix, severity, Mod Health Dimension, or Repair Recipe; whether Explainability would actually be violated by an overlapping filter, absent any real instance to check; GOV-013's own resolution; any transport or presentation consequence (already settled as none, by the Capability Definition, and not reopened here).

**Repository documents relied upon as fixed, unreopened precedent:** GOV-012 (Resolved) in its entirety; `SPRINT12_ARCHITECTURAL_RESOLUTION.md`'s own three-axis model and Historical Validation table (§5, §6 of that document), cited and applied, never re-derived from scratch; `RuleEngine.md`'s six responsibilities and execution principles; `CAPABILITY_DEFINITION_C2...md`'s own Architectural Boundaries and Explicit Exclusions, independently re-verified rather than assumed correct.

No Architectural Resolution has been performed. This evaluation's responsibility ends here, awaiting Chief Architect review.

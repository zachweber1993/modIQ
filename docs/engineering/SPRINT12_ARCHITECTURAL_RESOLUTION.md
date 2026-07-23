# Sprint 12 Architectural Resolution

| Property | Value |
|---|---|
| **Document** | SPRINT12_ARCHITECTURAL_RESOLUTION.md |
| **Project** | modIQ |
| **Purpose** | Reconcile the Capability Identity procedure after adversarial verification found a genuine contradiction in its original form, and record the resulting Collector and Rule Composition conclusions |
| **Prepared by** | Chief Architect / Technical Director (Sonnet 5), on `feature/runtime-implementation` |
| **Status** | Architectural reconciliation and validation complete. Awaiting Chief Architect approval before Sprint 12 closeout. No Rust source, test, fixture, ADR, or Governance Register item created or modified. |

---

## 1. Executive Summary

Sprint 12 set out to derive an explicit Capability Identity procedure from this platform's own completed engineering history, then resolve Collector Composition and Rule Composition as its consequences. The first draft (`SPRINT12.md` v1.0.0, Section 7) did exactly that — and, when checked adversarially against every named historical decision rather than only the headline ones, was found to contain a genuine contradiction: it conflated two independent architectural dimensions (whether a new `EvidenceCategory` is warranted, and whether a new Collector is warranted) into a single cascading test. Sprint 4 Phase 3C — the Duplicate Archive Entry Policy, GOV-011 — falsifies that conflation directly: it produced a brand-new `EvidenceCategory` (`StructuralDuplication`) by *extending* the already-existing `ArchiveCollector`, not by adding a new one.

This document records the full contradiction, the evidence that exposed it, the corrected model, and its complete re-validation against all seven historical decisions this platform has made — with zero remaining contradiction. The correction has one direct and materially significant consequence: **Sprint 11 is reclassified from Capability Introduction to Capability Expansion**, because it reused composition and dispatch machinery Sprint 7 had already proven, rather than inventing new machinery — a finding this document is careful to separate cleanly from Sprint 11's own, unchanged, product significance.

---

## 2. Contradiction Discovered

**Where:** `SPRINT12.md` v1.0.0, Section 7, Step 1 ("Kind test").

**When:** during this Sprint's own adversarial validation pass — deliberately re-examining "Sprint 4" one level deeper than its own headline label ("ArchiveCollector") suggested, per this project's standing discipline of checking documentation against evidence rather than confirming it.

**What was found:** Sprint 4 actually contains two separable architectural decisions, not one — `ArchiveCollector`'s own introduction, and the later Duplicate Archive Entry Policy (Phase 3C, GOV-011) layered onto it. The second of these directly contradicts Step 1's own predictive claim.

---

## 3. Original Model

Stated as a single, sequential, cascading decision tree:

> **Step 1 — Kind test.** Does recognizing the new observation require capturing a fact of a kind the platform has never captured before? **Yes → Capability Introduction** (new Collector, likely new Rule, new `EvidenceCategory`). **No →** proceed to a Collector-layer Mechanism test and a Rule-layer Judgment test.

The load-bearing assumption, stated plainly: **recognizing a new kind of fact (a new `EvidenceCategory`) was treated as sufficient, by itself, to imply a new Collector was required.** Category novelty and Collector novelty were not treated as separable questions — the first was assumed to determine the second.

---

## 4. Contradicting Evidence

**Sprint 4 Phase 3C — the Duplicate Archive Entry Policy (`GOVERNANCE.md`, GOV-011; `crates/modiq-collection/src/collection/archive_collector.rs`).**

- `ArchiveCollector`'s own central-directory-based enumeration cannot fully resolve an archive containing two entries under the same name — confirmed empirically during Sprint 4 Phase 2's Boundary-Proving investigation, independently across three tools.
- The platform's resolution: represent this fact as its own, new, closed-set `EvidenceCategory` — `StructuralDuplication` — deliberately kept distinct from `FileStructureAnalysis`, "so a Rule reasoning over `FileStructureAnalysis` structurally... must not silently absorb" a fact of a genuinely different kind (`EvidenceCollection.md`, Duplicate Archive Entry Policy).
- **This is unambiguously a new kind of fact** — a fact about the collection mechanism's own inspection limits, not a fact about the assessment subject's own structure.
- **It was produced by extending `ArchiveCollector` itself** — detected via a second, sequential local-file-header re-read of the *same* archive `ArchiveCollector` already opens, not by any new Collector, new crate module, or new parsing dependency.

**The direct falsification:** applying the original model's Step 1 to this decision predicts Capability Introduction (new kind of fact → new Collector). The real, already-shipped, already-tested outcome is Capability Enrichment at the Collector layer (the existing Collector was extended) combined with a new `EvidenceCategory` and a new Rule (`StructuralDuplicationRule`, Capability Expansion at the Rule layer). The original model cannot express this outcome at all — it has no path that produces "new category, same Collector."

---

## 5. Corrected Capability Identity Architecture

**Reasoning process.** The failure is structural, not a matter of degree: Step 1 bundled two questions — *is this fact a new kind?* and *does producing it need a new Collector?* — that Sprint 4 Phase 3C proves are independently answerable. The fix is to stop cascading and evaluate three axes independently, plus one further, orthogonal check for what actually distinguishes Capability Introduction from Capability Expansion — which, on the same close re-examination, turned out not to be "is anything about this novel" at all (by that standard Sprint 11 would also qualify, and did in the original draft), but specifically **whether realizing the capability required inventing composition or dispatch machinery the platform had never exercised before.**

### The Three Axes

**Collection Axis.** Does producing the new fact require an inspection mechanism (parser, reader, traversal technology) no existing Collector already uses?
- Same mechanism suffices → **Enrichment** (extend the existing Collector).
- Different mechanism required, with at least one Collector already in place to compare against → **Expansion** (new sibling Collector).
- No Collector exists yet at all → the founding case.

**Evidence Axis** — independent of the Collection Axis. Does the resulting fact represent a kind of observation no existing `EvidenceCategory` already covers (including a category declared in the enum but never yet produced)?
- Existing category suffices → no change.
- No existing category fits, or a dormant one is activated for the first time → new instance.

**Interpretation Axis.** Does interpreting the fact require a judgment — a question asked of the Evidence — no existing Rule already makes?
- Same judgment, richer content only → **Enrichment** (extend the existing Rule).
- New judgment → **Expansion** (sibling Rule, using the two-stage filter pattern — category match, then content-shape match — every category-specific Rule already uses internally).
- No Rule yet interprets the category → no verdict yet (a real, temporary state, not a contradiction).

### Capability Introduction — An Orthogonal Fourth Check, Not a Fourth Axis

A capability is **Introduction** only when realizing it requires inventing a composition or dispatch relationship the platform has never exercised before — independent of how many of the three axes above land as "novel." The test: *does this require a new way for Collectors or Rules to relate to each other or to the engine, or only another instance of a relationship already proven?*

This reframing is itself the corrected model's central finding, beyond fixing the Sprint 4 Phase 3C contradiction: **novelty at the Collection/Evidence/Interpretation axes does not, by itself, imply Introduction.** A capability can be novel at all three axes and still be Expansion, if it reuses proven machinery to realize that novelty (Sprint 11, Section 9). Introduction is reserved for the rarer case where the *machinery itself* — not merely its content — has no precedent.

### Classification Is Descriptive, Not an Approval Gate

**Recorded as an explicit amendment, 2026-07-23, following a Chief Architect governance review of Capability Identity's own scope (`INV-001_LUA_ANALYSIS_CAPABILITY.md`; Governance Register GOV-014).** Applying the three axes and the Introduction test answers a structural question — what kind of architectural participant, if any, a candidate would require — independent of whether sufficient evidence yet exists to authorize Capability Definition or Architecture Evaluation for that candidate. A candidate may be fully classified and still be deferred or rejected for insufficient evidence; classification is not itself a judgment that a candidate is ready to proceed. This is consistent with the procedure's own placement in the Decision Framework at the Observation/Evidence stage (`CHIEF_ARCHITECT_HANDOFF_v1.1.md`, Section 6), which precedes Investigation's own evidentiary-sufficiency judgment and Governance's own accept/reject/defer decision — and with this document's own Section 6, which classifies seven capabilities' architectural shape without expressing any judgment about their approval to proceed (all seven were already shipped when classified). No prior classification in this document is altered by this clarification; Sprint 11's reclassification (Section 9) and every Historical Validation entry (Section 6) stand unchanged.

---

## 6. Historical Validation

All seven decisions, re-validated against the corrected model:

| Decision | Collection Axis | Evidence Axis | Interpretation Axis | New machinery required? | Overall |
|---|---|---|---|---|---|
| **Sprint 3** (`EvidenceCollector`, founding) | No prior Collector (founding) | New (`FileStructureAnalysis`) | No category-specific Rule yet (generic `EvidencePresenceRule` only) | Yes — trivially, nothing existed | **Introduction** |
| **Sprint 4** (`ArchiveCollector`) | New mechanism (`zip` enumeration vs. filesystem walk) → Expansion | Same (`FileStructureAnalysis`) | No Rule change | No — reused the mutually-exclusive-routing shape the platform's own input model already implies | **Expansion** (Collection Axis only) |
| **Sprint 4 Phase 3C** (`StructuralDuplication`, GOV-011) — *the falsifying case* | Same mechanism (extended `ArchiveCollector`) → Enrichment | New (`StructuralDuplication`) | New judgment → `StructuralDuplicationRule` (Expansion) | No — same fixed-order Rule dispatch GOV-012 already governs | **Enrichment (Collection) + new Evidence Axis instance + Expansion (Interpretation)** |
| **Sprint 7** (`XmlCollector`) | New mechanism (`roxmltree`) → Expansion | New (`XmlInspection`) | Deferred to Sprint 8 | **Yes — the first time any Assessment ever ran more than one Collector; composition machinery invented from nothing** | **Introduction** |
| **Sprint 8** (`VersionCompatibilityRule`) | Same mechanism (extends `XmlCollector`'s existing parse) → Enrichment | Same (`XmlInspection`, real since Sprint 7) | New judgment → new Rule (Expansion) | No — reused Sprint 7's additive composition and GOV-012's dispatch model | **Enrichment (Collection) + Expansion (Interpretation)** |
| **Sprint 9** (`RepairRecipe` evolution) | No Collector change | No category change | Same judgment, richer content only → Enrichment | No — no new participant of any kind | **Pure Enrichment** |
| **Sprint 11** (`RuntimeLogCollector` / `RuntimeLoadFailureRule`) | New mechanism (log-line matching) → Expansion | New activation (`RuntimeLogs`, dormant since before Sprint 1) | New judgment → new Rule (Expansion) | **No — reused Sprint 7's additive composition and GOV-012's dispatch model, unchanged** | **Expansion** (reclassified — Section 9) |

**Zero contradictions remain**, including against the case that broke the original model. Two independent instances (Sprint 8, Sprint 4 Phase 3C) now confirm that a single capability's Collection-Axis and Interpretation-Axis classifications can differ within the same decision — this is a repeated, not one-off, pattern.

**Retrospective-bias check, reported honestly:** the corrected model, like its predecessor, would not have changed any of the seven real historical outcomes. Its value is in correctly *describing* all seven (which the original model could not do for one of them) and in correctly separating architectural machinery novelty from product novelty going forward — not in retroactively correcting an implementation error the way Sprint 8's or Sprint 9's own review stages did.

---

## 7. Collector Guidance

**No change to the Collector Composition Architecture's own five-condition extraction threshold** (three-or-more content Collectors; significantly complex applicability; configurable execution order; desirable parallelism; multiple entry points requiring identical orchestration) — nothing in this reconciliation requires revisiting it, and none is proposed.

**Gate function (carried forward, unchanged in substance):** the Collection Axis's Enrichment/Expansion distinction protects the extraction threshold's own counter from false-positive increments — a future engineer facing pressure to "just add a Collector" for a fact that actually classifies as Enrichment would otherwise inflate the Collector count for a capability that never needed a new participant.

**New guidance, found during this Sprint's own validation, independent of the threshold:** when the Collection Axis resolves to Expansion, a second, previously implicit choice remains — how the new Collector composes with the ones already in place.

- **Mutually-exclusive routing** — appropriate when the Collectors' own applicability conditions are mutually exclusive properties of the same Assessment Input. *Confirmed instance: `EvidenceCollector` vs. `ArchiveCollector` — a location either is or is not a `.zip`, never both.*
- **Additive composition** — appropriate when the applicability conditions can co-occur. *Confirmed instances: `XmlCollector` alongside the structural Collector; `RuntimeLogCollector` alongside `XmlCollector` — a directory can contain `modDesc.xml`, `log.txt`, and arbitrary files simultaneously.*

**Why this is independent of the extraction threshold:** the threshold answers *how many* Collectors justify a coordinator; this guidance answers *how* two coexisting Collectors relate to each other's own applicability. Neither determines the other. **Why no threshold change is required:** this guidance changes no Collector count, no orchestration complexity, and none of the five conditions the threshold itself watches — it only makes an already-twice-demonstrated pattern explicit for whoever adds the next Collector.

---

## 8. Rule Guidance

The Interpretation Axis's judgment test — "does this require a new question asked of the Evidence, or the same question with richer supporting content" — is materially more complete than the Collection Axis's own gate function, and is now confirmed by **four** independent historical instances rather than three: Sprint 4 Phase 3C joins Sprint 8, Sprint 9, and Sprint 11.

**Explicit, unchanged caveat:** no historical decision has yet tested two Rules, or two recognized fact-shapes, sharing one already-interpreted `EvidenceCategory`. Every category-specific Rule to date — `StructuralDuplicationRule`, `VersionCompatibilityRule`, `RuntimeLoadFailureRule` — has been the first and only interpreter of its own category. The judgment test's answer for that specific case is a disciplined extrapolation of the two-stage filter pattern all three Rules already use internally, not a fifth confirmed data point. This is stated here exactly as strongly as it should be treated: strong guidance, not settled precedent.

---

## 9. Sprint 11 Classification Clarification

**The reconciled model classifies Sprint 11 as Capability Expansion, not Capability Introduction.**

**What changed:** the architectural-machinery classification only. Sprint 11 added a new Collector (Collection Axis: Expansion), activated a dormant `EvidenceCategory` for the first time (Evidence Axis: new), and added a new Rule (Interpretation Axis: Expansion) — but invented no composition or dispatch mechanism the platform had not already proven. `RuntimeLogCollector` composes with `XmlCollector` using exactly the additive-composition shape Sprint 7 established. `RuntimeLoadFailureRule` dispatches using exactly the fixed-order, independent-composition model GOV-012 established. `RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md` §1.1 confirms this in its own words, written at the time, before this reconciliation existed: *"none of the five extraction conditions is met... inline composition, exactly as it stands today, remains the correct and sufficient shape."* By the corrected model's own narrow Introduction test, Sprint 11 answers no. Sprint 7 remains the only confirmed non-degenerate Introduction in this platform's history.

**What did not change, and must not be read as diminished:**

- Sprint 11 remains the platform's **first real event-based Evidence source** — every prior category described a structural or content fact about a mod's own package; `RuntimeLogs` is the first to describe what happened during an actual attempt to load or run one.
- Sprint 11 remains the platform's **first real use of `FindingSeverity::Error`**, reasoned directly from `DataModel.md`'s own definitions rather than deferred to the game engine's own log wording.
- Sprint 11 remains, by any product measure, one of this project's largest milestones — the first real, evidence-based answer to `Vision.md`'s own named founding question, "why does it fail to load?"

**Architectural Classification and Product Significance are different axes of judgment, and this reconciliation measures only the first.** A capability can be, and here is, simultaneously an architectural Expansion (it reused every mechanism the platform had already proven) and one of the most significant product milestones this platform has shipped (it answered a question no prior capability could). Neither statement weakens the other. If anything, Sprint 11's Expansion classification is a *positive* finding about this platform's own architectural maturity: the machinery Sprint 7 built generalized cleanly to a capability of substantially greater product weight, without needing to be re-invented or extended to do so.

---

## 10. Architectural Limitations

- **The corrected model still requires human judgment to apply** — "does this need a new mechanism," "is this the same judgment," "does this require new machinery" are none of them mechanically checkable. It is a decision procedure for a reviewer to apply deliberately, never a runtime classifier, exactly as this Sprint's own constraints require.
- **The Introduction test has exactly one confirmed non-degenerate instance (Sprint 7).** One instance is a real precedent, not yet a statistically robust pattern — the same epistemic caveat GOV-004 applied before treating a pattern as settled (three converging subsystems, not one).
- **Rule Composition's "second Rule, same category" case remains genuinely untested** (Section 8), disclosed as such, not resolved by this reconciliation.
- **This reconciliation itself was found by re-examining one historical decision (Sprint 4) more closely than its own headline label suggested.** It is possible other headline labels in Section 6's table (or in `GOVERNANCE.md`'s own register — GOV-005/GOV-006, GOV-013 — none of which added a Collector, Rule, or category and so were not evaluated here) similarly compress more than one decision. This reconciliation does not claim to have exhaustively re-examined every sub-decision in the platform's history, only the seven the Chief Architect specifically named.

---

## 11. Technical Director Assessment

**This Sprint succeeded on its second pass, not its first — and that is the correct, expected outcome of applying this platform's own adversarial-verification discipline to its own planning documents, not only to its implementation.** The corrected model is more precise (it correctly separates two dimensions the original model conflated), more conservative in what it calls Introduction (one confirmed instance instead of three), and better evidenced at the Interpretation Axis (four confirming instances instead of three) than the document it replaces. The Sprint 11 reclassification is the single most consequential finding of this reconciliation, and it has been written with explicit, repeated care to avoid any implication that Sprint 11's own real, considerable product significance is in question — it is not.

**Risk:** Low. No code, test, or fixture was touched at any point in this reconciliation. The residual risk is intellectual — an unexamined eighth sub-decision, or a second-order flaw in the corrected model itself — named honestly in Section 10, not engineered away.

---

## 12. Repository Impact

| Area | Change |
|---|---|
| `docs/implementation/SPRINT12.md` | Amended in place, v1.0.0 → v1.1.0. Original Section 7 preserved in a Reconciliation Record, not deleted; Sections 4, 8, 9, 10, 12, 16, and the Document Status footer updated for consistency. |
| `docs/engineering/SPRINT12_ARCHITECTURAL_RESOLUTION.md` | New — this document. |
| Rust source, tests, fixtures, crates | None touched. |
| `EvidenceCategory` / `FindingSeverity` | Unchanged. |
| Governance Register | Unchanged — 13 items, 8 Resolved, 5 Open. |
| ADRs | None new. |
| Documentation Release | 2.1, unchanged. |
| Collector Composition Architecture, Rule Engine Architecture (GOV-012), Runtime Evidence Processing Architecture (v1.2.0) | All three reviewed directly against this reconciliation; none required amendment. `RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md`'s own text is quoted, not altered, as supporting evidence for Section 9. |

---

## 13. Final Recommendation

**Architectural reconciliation and validation are complete.** All seven historical decisions classify without contradiction under the corrected model; the Sprint 11 reclassification is recorded with an explicit, repeated distinction between architectural classification and product significance; Collector and Rule guidance are both recorded as consequences, with their respective evidentiary weights disclosed honestly rather than overstated; no existing architecture (Collector Composition, GOV-012, `RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md`) required amendment.

**Per explicit instruction, Sprint 12 closeout is not performed here.** Recommend the Chief Architect review and approve this reconciliation; on approval, Sprint 12 closeout (repository reconciliation of `PROJECT_STATUS.md`/`CHANGELOG.md`/`ENGINEERING_LOG.md`/`CrateRoadmap.md`/`docs/README.md`, and an Engineering Release) can proceed as its own, separate, subsequent step — exactly as Sprint 11's own architectural reconciliation preceded its closeout, not merged into it.

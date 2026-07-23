# Sprint 12

> **Capability Scaling Architecture — deriving an explicit Capability Identity decision procedure from the platform's own completed engineering history, and evaluating Collector and Rule composition as its consequences.**

---

| Property | Value |
|----------|-------|
| **Document** | SPRINT12.md |
| **Version** | 1.1.0 |
| **Status** | Proposed — planning only, awaiting Chief Architect review before Sprint 12 closes. This revision reconciles the Capability Identity procedure (Section 7) after adversarial verification found a genuine contradiction in the original v1.0.0 model; see the Reconciliation Record immediately below and `docs/engineering/SPRINT12_ARCHITECTURAL_RESOLUTION.md` for the complete evidence and reasoning. |
| **Project** | modIQ |
| **Documentation Release** | 2.1 (unchanged; this document does not amend it) |
| **Owner** | Zach Weber |
| **Created** | 2026-07-22 |
| **Last Updated** | 2026-07-22 |

---

# Reconciliation Record (v1.0.0 → v1.1.0)

**This document's own Capability Identity procedure was found to contain a genuine architectural contradiction during this Sprint's own adversarial verification pass — before any repository closeout, exactly the discipline Sprint 11 established.** Recorded here in full, not silently corrected, per this project's standing "do not rewrite history" practice:

- **Original assumption (v1.0.0, Section 7):** a single cascading test — "does this capture a fact of a kind never captured before?" — where "yes" was treated as implying a new Collector was needed, bundling category novelty and Collector novelty into one verdict.
- **Contradicting evidence:** Sprint 4 Phase 3C (`EvidenceCategory::StructuralDuplication`, GOV-011) — a genuinely new `EvidenceCategory`, produced by *extending* the already-existing `ArchiveCollector`, not by adding a new Collector. The original model's Step 1 predicts a new Collector here and is wrong.
- **Correction:** category novelty and Collector novelty are independent architectural dimensions, not one cascading test. The procedure is restated below as three independent axes (Section 7) plus one orthogonal check for Capability Introduction.
- **Direct consequence:** Sprint 11 is reclassified from Capability Introduction to Capability Expansion (Section 8.3) — because it reused Sprint 7's own already-proven composition machinery, rather than inventing new machinery, even though it added a new Collector, a new category, and a new Rule.
- **What did not change:** the overall three-bucket vocabulary (Enrichment / Expansion / Introduction), the Collector Composition Architecture's own five-condition threshold (untouched), GOV-012's Rule dispatch model (untouched), and this Sprint's own scope (still architecture-only, still no implementation).

Full evidence, the complete seven-decision validation table, and the full reasoning process are recorded in `docs/engineering/SPRINT12_ARCHITECTURAL_RESOLUTION.md`. This document's own Section 7 states only the corrected, now-authoritative model.

---

# 1. Executive Summary

Sprint 11 proved that this platform can implement its first real capability in a new architectural axis — Runtime Evidence Processing — deterministically, evidence-first, and without redesigning any existing boundary. It also proved, through its own mid-Sprint architectural reconciliation, that this platform's adversarial-verification discipline catches real inconsistencies before they compound. Sprint 12 does not build a second capability. It asks the question that must be answered *before* a second capability can be added without re-deriving, from first principles and under implementation pressure, a decision this platform has actually already made five times: **when does a new observation enrich something that already exists, expand an existing architectural axis, or require an entirely new one?**

This question has never been asked explicitly. It has been answered correctly, five times, by implicit judgment — a new Collector here, an extended Collector there, a new Rule in one case, no new participant at all in another. Sprint 12's job is to make that judgment an explicit, citable, historically-verified procedure, so that Sprint 13 (whatever it turns out to be) inherits a decision process rather than a blank page.

---

# 2. Sprint Objective

Derive, from the platform's own completed engineering history — not from theory, not from a hypothetical future capability — an explicit Capability Identity decision procedure distinguishing **Capability Enrichment**, **Capability Expansion**, and **Capability Introduction**. Evaluate Collector Composition and Rule Composition strictly as consequences of that procedure, not as independent questions. Produce a Sprint 12 Architectural Resolution recording all of this. Write no code, add no fixture, modify no test, and introduce no runtime mechanism.

---

# 3. Engineering Rationale

Sprint 11's own Chief Architect review found that its originally proposed Sprint 12 objective — resolve Collector Composition and Rule Composition — was reachable only by silently presupposing an answer to a prior question neither one asked: *how do we know a new observation needs a new Collector, or a new Rule, at all?* Both proposed questions, examined honestly, turned out to be downstream consequences of a classification this project has made five times without ever stating the procedure.

That prior question — Capability Identity — is also the more evidence-grounded one to resolve first. Collector Composition and Rule Composition, asked directly, require reasoning about a hypothetical, unbuilt third participant. Capability Identity requires only pattern-mining decisions this repository has already made and already validated in production: `EvidenceCollector`/`ArchiveCollector` (Sprint 3/4), `XmlCollector` (Sprint 7), `VersionCompatibilityRule` (Sprint 8), `RepairRecipe`'s evolution (Sprint 9), and `RuntimeLogCollector`/`RuntimeLoadFailureRule` (Sprint 11). This is a *stronger* evidentiary basis than Sprint 12's original proposal offered, not a weaker or more abstract one — it satisfies this project's own standing bar (`GOVERNANCE.md`, GOV-004's "convergent evidence over a single implementation attempt") more completely than most prior Architecture Evaluations have, because it has five independent data points instead of two or three.

---

# 4. Architectural Background

Reviewed directly, not recalled, for this Sprint:

- **Sprint 3** — `EvidenceCollector` founded the Evidence Collection axis; no prior state exists to classify it against (the degenerate, founding case).
- **Sprint 4** — `ArchiveCollector` followed to handle a `.zip` Assessment Input: same `EvidenceCategory` (`FileStructureAnalysis`), same conceptual concern ("discover structure"), but a fundamentally different underlying mechanism (`zip`-crate entry enumeration vs. filesystem traversal) — resolved as a new, sibling Collector, mutually-exclusive-routed against the same input (`is_archive_location`), not an extension of `EvidenceCollector`.
- **Sprint 4 Phase 3C** — the Duplicate Archive Entry Policy (GOV-011) produced `EvidenceCategory::StructuralDuplication`, a genuinely new kind of fact (a fact about the collection mechanism's own inspection limits, not about the mod's structure) — but by *extending the already-existing* `ArchiveCollector`, reusing the same sequential archive-reading mechanism it already performs, not by adding a new Collector. Paired with a new Rule, `StructuralDuplicationRule`, since no existing Rule's judgment covered "did collection detect a naming collision it could not resolve."
- **Sprint 7** — `XmlCollector` introduced `EvidenceCategory::XmlInspection`, a fact of a *kind* the platform had never captured before (content semantics — manifest well-formedness, declared dependencies — as opposed to structural discovery). New Collector, new category, composed independently and additively alongside the structural Collector (`COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`) — the first time this platform ever ran more than one Collector per Assessment.
- **Sprint 8** — `VersionCompatibilityRule` added a new Rule, but at the Collector layer, `XmlCollector` was *extended*, not replaced or sibling-added: the same parser (`roxmltree`), the same document, one additional attribute read (`descVersion`). At the Rule layer, no existing Rule's own judgment covered "is this declared value supported" — `EvidencePresenceRule` fires unconditionally regardless of content; `StructuralDuplicationRule` judges a structural-collision fact. A genuinely new judgment required a genuinely new Rule.
- **Sprint 9** — `RepairRecipe`'s evolution added *zero* new architectural participants. `VersionCompatibilityRule`'s own judgment — is this declared version supported — did not change at all; only the *content* backing its existing Recommendation changed, from inline-authored text to a real call into `modiq-knowledge`. No new Collector, no new Rule, no new category.
- **Sprint 11** — `RuntimeLogCollector` activated `EvidenceCategory::RuntimeLogs`, a fact of a kind never captured before in a different sense than Sprint 7's: not merely new content semantics, but the platform's first *event* rather than *structure* (`SPRINT10_CAPABILITY_DEFINITION.md`, Section 3: "no Evidence source this platform has ever produced has described an outcome rather than a structure; this is the first"). New Collector, new Rule (`RuntimeLoadFailureRule`), new judgment — composed and dispatched, however, using exactly the additive-composition and fixed-order dispatch machinery Sprint 7 and GOV-012 had already established (see Section 8.3).

Two facts surfaced by this review bear directly on the Secondary Questions, one strengthening the original finding and one correcting it: **no historical decision to date has ever added a second Rule, or a second recognized fact-shape, to a category an existing Rule already interprets** (Section 8.2) — and **category novelty and Collector novelty are independent dimensions, not one cascading test** (Sprint 4 Phase 3C; the Reconciliation Record above). Both are stated plainly in Section 7 and Section 8, not glossed over.

---

# 5. Scope

## Included

- **Deriving the Capability Identity procedure**, strictly from the seven historical decisions named above (treating Sprint 3 as the acknowledged degenerate founding case), stated as three independent axes plus one orthogonal Introduction check — not a taxonomy invented independently of them.
- **Classifying each historical decision against the derived procedure**, showing the procedure reproduces every known-correct outcome without contradiction, and stating explicitly whether the procedure would have changed any past decision had it existed beforehand (Section 10's falsifiable test).
- **Evaluating Collector Composition strictly as a consequence** of Capability Identity's own output: what Identity determines (whether a new Collector is warranted at all, gating the Collector Composition Architecture's own extraction-threshold counter) versus what it does not determine (the Collector Composition Architecture's own five-condition threshold itself, already resolved at Sprint 7, remains untouched and is not re-derived here).
- **Evaluating Rule Composition strictly as a consequence** of Capability Identity's own "same judgment vs. different judgment" test, while being explicit that this question currently has no direct historical instance of a second Rule joining an already-interpreted category — the answer offered is a disciplined extrapolation of the two-stage filter pattern (`EvidenceCategory` match, then content-shape match) `VersionCompatibilityRule` and `RuntimeLoadFailureRule` already both use internally, not a claim of proven precedent equal in weight to the Collector-layer finding.
- Producing one Sprint 12 Architectural Resolution document recording all of the above, its rationale, its historical citations, and the honest limits of what this Sprint's own evidence can support.

## Out of Scope (see Section 6 for the complete boundary)

Any implementation, any new fixture, any change to `AssessmentService`, any Collector or Rule change, any new EvidenceCategory, any speculative future-capability design.

---

# 6. Out of Scope

- **Any Rust implementation whatsoever.** No Collector, Rule, `AssessmentService`, or Engine change. No test added, removed, or modified. No crate structure change.
- **Deciding which future capability comes next** (a second runtime-log signature, Lua analysis, asset validation, dependency resolution, performance observations) — that remains a separate, later Capability Definition's own job, and this Sprint's own procedure is what that future Definition should apply, not a preview of its answer.
- **Building any runtime classification mechanism.** The Capability Identity procedure is an architectural decision document a human (or a future Chief Architect review) applies deliberately — never a `match`, registry, trait, or dispatch table encoding the three buckets in code. Encoding it in Rust would itself be the exact premature abstraction this Sprint exists to prevent.
- **Reopening GOV-012, GOV-013, or the Collector Composition Architecture's own already-resolved decisions.** This Sprint applies and extends them; it does not re-litigate them.
- **Any ADR.** This Sprint's own conclusion is expected to apply existing principles, not establish a new durable one requiring one — consistent with how GOV-012/GOV-013 themselves were recorded without an ADR.
- **Any Governance Register item or `GOVERNANCE.md` amendment**, unless this Sprint's own findings surface a genuine cross-cutting gap — not presumed here.
- **Any new runtime fixture.** This Sprint's evidence is the repository's own completed engineering history, not new captured content.

---

# 7. Primary Architectural Question

## Capability Identity — Reconciled Model (v1.1.0)

> **This section supersedes the original single-cascading-tree procedure this document first proposed (preserved in full in the Reconciliation Record above and in `SPRINT12_ARCHITECTURAL_RESOLUTION.md`, along with the falsifying evidence and reasoning process). What follows is the corrected, currently-authoritative model.**

**Three independent architectural axes**, evaluated separately for any new engineering decision — never a single cascading test, per the correction recorded above:

**1. Collection Axis.** Does producing the new fact require an inspection mechanism (parser, reader, traversal technology) no existing Collector already uses?

- Same mechanism suffices → **Enrichment** at the Collection Axis (extend the existing Collector). *Sprint 8 (`descVersion` extraction extends `XmlCollector`'s existing `roxmltree` parse), Sprint 4 Phase 3C (duplicate detection extends `ArchiveCollector`'s existing sequential archive read).*
- A different mechanism is required, and at least one Collector already exists to compare against → **Expansion** at the Collection Axis (new sibling Collector). *Sprint 4 (`ArchiveCollector` needs `zip`-crate enumeration), Sprint 7 (`XmlCollector` needs `roxmltree` parsing), Sprint 11 (`RuntimeLogCollector` needs log-line matching).*
- No Collector exists yet at all → the founding case. *Sprint 3.*

**2. Evidence Axis** — independent of the Collection Axis; this independence is exactly what Sprint 4 Phase 3C proves and the original model missed. Does the resulting fact represent a kind of observation no existing `EvidenceCategory` already covers (including a category declared in the enum but never yet produced)?

- Existing category suffices → no Evidence Axis change. *Sprint 4 (still `FileStructureAnalysis`), Sprint 8 and Sprint 9 (both still `XmlInspection`, already real since Sprint 7).*
- No existing category fits, or a dormant category is activated for the first time → new Evidence Axis instance. *Sprint 3 (`FileStructureAnalysis`, first use), Sprint 4 Phase 3C (`StructuralDuplication`), Sprint 7 (`XmlInspection`), Sprint 11 (`RuntimeLogs`, dormant since before Sprint 1).*

**3. Interpretation Axis.** Does interpreting the fact require a judgment — a question asked of the Evidence — that no existing Rule already makes?

- Same judgment, richer content only → **Enrichment** at the Interpretation Axis (extend the existing Rule). *Sprint 9 — `VersionCompatibilityRule`'s own judgment never changed; only its Recommendation's content did.*
- New judgment → **Expansion** at the Interpretation Axis (sibling Rule), following the two-stage filter pattern (`EvidenceCategory` match, then content-shape match) every category-specific Rule already uses internally. *Sprint 4 Phase 3C (`StructuralDuplicationRule`), Sprint 8 (`VersionCompatibilityRule`), Sprint 11 (`RuntimeLoadFailureRule`).*
- No Rule yet interprets the category at all → no verdict yet — a real, temporary state, not a contradiction. *Sprint 3 and Sprint 4 (`FileStructureAnalysis` is still consumed only by the generic, category-agnostic `EvidencePresenceRule`); Sprint 7 at the moment of its own introduction, resolved one Sprint later at Sprint 8.*

**Capability Introduction — an orthogonal fourth check, not a fourth axis.** A capability is Introduction only when realizing it requires inventing composition or dispatch machinery the platform has never exercised before, regardless of how many of the three axes above are individually novel. The test: *does this require a new way for Collectors or Rules to relate to each other or to the engine, or only another instance of a relationship already proven?*

- **Sprint 3** — Introduction, trivially: no machinery of any kind existed yet.
- **Sprint 7** — Introduction, and the only confirmed non-degenerate instance: before it, no Assessment had ever run more than one Collector; `COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md` exists because this required inventing composition machinery from nothing.
- **Sprint 4, Sprint 4 Phase 3C, Sprint 8, Sprint 9, Sprint 11** — all Expansion or Enrichment, never Introduction, because each reused a composition or dispatch relationship an earlier Sprint had already proven. **Sprint 11 is Expansion, not Introduction** — see Section 8.3 for the full clarification this reclassification requires.

**Reproduction check:** all seven historical decisions (Sprint 3, 4, 4 Phase 3C, 7, 8, 9, 11) classify without contradiction under this corrected model — including Sprint 4 Phase 3C, the case that falsified the original one. **Retrospective-bias check, unchanged in spirit from the original model:** this corrected model, like its predecessor, would not have changed any of the seven real outcomes — it makes an already-correct set of implicit judgments explicit and citable. What it newly reveals, that the original model could not, is the Sprint 11 reclassification (Section 8.3) and the Collector Guidance finding (Section 8.1).

---

# 8. Secondary Architectural Questions

Both evaluated strictly as consequences of Section 7's reconciled model, per the Technical Director's own explicit instruction — neither is independently re-opened.

## 8.1 Collector Composition

The Collection Axis determines **whether** a new Collector is warranted at all — the event that increments the content-Collector count the Collector Composition Architecture's own five-condition extraction threshold watches (`COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`, Section 14). **The Collection Axis does not, and is not intended to, re-derive that threshold itself** — it remains exactly as Sprint 7 resolved it, unchanged and unre-litigated here, and no changes to it are proposed by this Sprint.

**Gate function, carried forward from the original model:** without an explicit Enrichment/Expansion distinction at the Collection Axis, a future engineer facing pressure to "just add a Collector" for a fact that actually classifies as Enrichment risks incrementing the Collector count — and approaching the extraction threshold — for a capability that never needed a new participant at all.

**New guidance, discovered during this Sprint's own validation — independent of the extraction threshold, not a replacement for it.** When the Collection Axis resolves to Expansion, a second, previously implicit choice must still be made: how the new Collector composes with the ones already in place. Two shapes are now each demonstrated more than once, and the discriminating test, read directly off both:

- **Mutually-exclusive routing** — appropriate when the Collectors' own applicability conditions are mutually exclusive properties of the same Assessment Input (the input either is or is not a `.zip`, never both). *Sprint 3/4: `EvidenceCollector` vs. `ArchiveCollector`.*
- **Additive composition** — appropriate when the applicability conditions can co-occur (a directory can simultaneously contain `modDesc.xml`, `log.txt`, and arbitrary files, all at once). *Sprint 7, Sprint 11.*

This guidance is independent of the extraction threshold for two reasons: it answers a different question (*how* two Collectors relate to each other's applicability) than the threshold answers (*how many* Collectors justify a coordinator); and nothing about it changes the Collector count, orchestration complexity, or any of the five conditions the threshold itself watches. **No change to the existing threshold is required or proposed** — this is additive guidance for whoever adds the next Collector, not a revision of when a coordinator becomes justified.

## 8.2 Rule Composition

The Interpretation Axis's judgment test is a **materially more complete answer** than the Collection Axis's own gate function — "extend vs. sibling" *is* "same judgment vs. different judgment," restated. This finding is now confirmed by **four** independent historical instances, not three as the original model stated: Sprint 4 Phase 3C (`StructuralDuplicationRule` — a new judgment, correctly predicting a sibling Rule) joins Sprint 8, Sprint 9, and Sprint 11 as a fourth, independently converging data point.

One explicit caveat remains, unchanged by this reconciliation: **no historical decision has yet tested two Rules, or two recognized fact-shapes, sharing one already-interpreted `EvidenceCategory`.** Every category-specific Rule introduced so far — `StructuralDuplicationRule`, `VersionCompatibilityRule`, `RuntimeLoadFailureRule` — has been the first and only interpreter of its own category. The judgment test is derived by extrapolating the two-stage filter pattern (`EvidenceCategory` match, then content-shape match) all three of these Rules already use internally — a low-risk extrapolation of an already-three-times-proven internal pattern, but an extrapolation, not a fifth independent data point for this specific untested case.

## 8.3 Sprint 11 Classification — Architectural Reclassification, Not a Diminishment

The reconciled model classifies Sprint 11 as **Capability Expansion**, not Capability Introduction as the original model concluded. This subsection exists specifically to state what changed and what did not.

**What changed:** the *architectural machinery* classification only. Sprint 11 added a new Collector (Collection Axis: Expansion), activated a dormant `EvidenceCategory` for the first time (Evidence Axis: new), and added a new Rule (Interpretation Axis: Expansion) — but invented no composition or dispatch mechanism the platform had not already proven. `RuntimeLogCollector` composes with `XmlCollector` using exactly the additive-composition shape Sprint 7 established; `RuntimeLoadFailureRule` dispatches using exactly the fixed-order, independent-composition Rule model GOV-012 established. `RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md` §1.1 confirms this in its own words, written at the time: "*none of the five extraction conditions is met... inline composition, exactly as it stands today, remains the correct and sufficient shape.*" By Section 7's own narrow Introduction test, Sprint 11 answers no — Sprint 7 remains the only confirmed non-degenerate Introduction in this platform's history.

**What did not change, and must not be read as diminished:**

- Sprint 11 remains the platform's **first real event-based Evidence source** — every prior category described a structural or content fact about a mod's own package; `RuntimeLogs` is the first to describe what happened during an actual attempt to load or run one.
- Sprint 11 remains the platform's **first real use of `FindingSeverity::Error`**, reasoned directly from `DataModel.md`'s own definitions.
- Sprint 11 remains one of this project's largest product milestones — directly answering `Vision.md`'s own named founding question, "why does it fail to load?", for the first time with real, deterministic evidence.

**Architectural Classification and Product Significance are different axes of judgment; this reconciliation measures only the first.** A capability can be, and here is, simultaneously an architectural Expansion (it reused every mechanism the platform had already proven) and one of the most significant product milestones this platform has shipped (it answered a question no prior capability could). Neither statement weakens the other — if anything, Sprint 11's own Expansion classification is a *positive* finding: the machinery Sprint 7 built generalized cleanly to a capability of substantially greater product weight, without needing to be re-invented to do so.

---

# 9. Deliverables

- **`docs/engineering/SPRINT12_ARCHITECTURAL_RESOLUTION.md`** — recording the reconciled Capability Identity procedure (Section 7), the full contradiction-and-correction narrative, the complete seven-decision validation table, and both Secondary Question findings (Section 8), including the explicit statement of what each does and does not resolve.
- **No other artifact.** No code, no interface, no test, no fixture, no new document beyond this planning document and the Resolution document above.

---

# 10. Success Criteria

- Capability Identity is derived solely from the seven named historical decisions (six Sprints, with Sprint 4 separated into its own Collector introduction and its own Phase 3C category-extension decision) — no theory invented independently of them, no forward-looking hypothetical used as primary evidence.
- The procedure reproduces every one of the seven historical classifications without contradiction (Section 7's reproduction check), and explicitly states whether it would have changed any past outcome (it does not — stated as a limitation, not a strength claimed falsely).
- Collector Composition is resolved strictly as a Capability Identity consequence, explicitly bounded to *not* re-deriving the Collector Composition Architecture's own already-resolved five-condition threshold.
- Rule Composition is resolved strictly as a Capability Identity consequence, with the absence of a direct historical instance for "second Rule, same category" disclosed explicitly, not glossed over.
- Zero Rust source modified; `cargo fmt --check`, `cargo check --workspace`, and `cargo test --workspace` remain exactly as they stood at Engineering Release 1.1 (238/238 root, Sandbox 7/7) — reverified, not assumed, at this Sprint's own close.
- No Governance Register item, ADR, or Documentation Release amendment created unless this Sprint's own findings demonstrate a genuine cross-cutting gap — none is presumed here.
- No runtime classification mechanism, registry, coordinator, or dispatch abstraction is introduced at any point.

---

# 11. Risks

- **Confirmation bias in retrospective derivation.** A framework built to explain decisions already known to be correct can easily be shaped, consciously or not, to fit them. *Mitigation:* Section 7's own explicit reproduction check states plainly that the procedure would not have changed any past outcome — a disclosed limitation, not a suppressed one, and the honest test this Sprint's own Success Criteria require.
- **Overclaiming the Rule Composition finding.** Section 8.2's judgment test is an extrapolation from a proven internal pattern, not a fourth independent historical instance. Presenting it with the same confidence as the Collector-layer finding would misstate the actual evidentiary weight behind it. *Mitigation:* stated explicitly and separately in Section 8.2, not merged into a single "both resolved equally" claim.
- **Temptation to encode the three-bucket taxonomy in code.** A clean, three-way decision tree is unusually inviting to implement as a literal classifier. *Mitigation:* Section 6 states directly that doing so is out of scope and would itself be the premature abstraction this Sprint exists to prevent; the Resolution document should repeat this boundary explicitly, not merely imply it.
- **Under-scoping risk accepted deliberately.** This Sprint does not, and should not, attempt to resolve the untested "second Rule, same category" case beyond disciplined extrapolation — attempting to manufacture false certainty here would be worse than naming the gap.

---

# 12. Dependencies

- `RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md` v1.2.0 — the most recent real capability decision this Sprint's own procedure must correctly classify (Expansion, per the reconciled model — see Section 8.3).
- `COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`, Chief Architect Decision Record (Section 14) — the extraction threshold Section 8.1 explicitly does not re-derive.
- `GOVERNANCE.md`, GOV-011 — the Duplicate Archive Entry Policy decision (Sprint 4 Phase 3C) that exposed the original model's contradiction.
- `GOVERNANCE.md`, GOV-012 — the Rule dispatch model Section 8.2 extends without amending.
- `SPRINT9_RUNTIME_LOG_ARCHITECTURE_EVALUATION.md`, Section 3 — the direct precedent for asking an identity/classification question at the Assessment level; this Sprint asks the equivalent question one layer down, within an already-resolved axis.
- The seven real historical decisions named in Section 4, verified directly against source and prior Engineering Releases, not recalled from memory alone.

---

# 13. Deferred Work

- Any second runtime-log signature, its own fixture acquisition, or its own future Architectural Resolution — this Sprint's procedure is what that future work should apply, not a preview of its outcome.
- Lua analysis, asset validation, dependency resolution, performance observations — all remain unscoped, each awaiting its own future Capability Definition informed by this Sprint's procedure.
- The untested "second Rule, same already-interpreted category" case — named explicitly as unresolved by direct precedent (Section 8.2), left for whichever future capability first encounters it to test the extrapolation against real evidence.
- GOV-008 (`AssessmentService` public API evolution), GOV-013 (`FindingSeverity` conflation) — both untouched, unaffected by this Sprint's own scope.

---

# 14. Technical Director Assessment

**Sprint sizing:** Small, architecture-only — the smallest kind of Sprint this project's own history recognizes (mirroring Sprint 9's Architecture Evaluation and Sprint 11's own isolated Architectural Resolution phase), appropriately scaled to a review whose entire evidentiary basis is documents and code that already exist.

**Architectural alignment:** High, and unusually well-evidenced for a planning Sprint — five independent historical decisions, not a single case or a hypothetical projection, satisfy this project's own GOV-004-style convergent-evidence bar more completely than most prior Architecture Evaluations were able to at the time they were written.

**Risk:** Low. No code change is possible within this Sprint's scope, so no regression risk exists. The primary risk is intellectual (confirmation bias, overclaiming a thin data point), both named explicitly in Section 11 with disclosed, not engineered-away, mitigations.

**Expected repository impact:** One new planning document (this one) and one future Sprint 12 Architectural Resolution document. No crate, dependency, test, fixture, Governance Register item, or ADR change. Repository maturity metrics (nine crates, 238 root tests, 7 Sandbox tests) are expected to remain exactly as recorded at Engineering Release 1.1 through the end of this Sprint.

---

# 15. Expected Repository Impact

| Area | Expected Change |
|---|---|
| Rust source | None |
| Tests | None — 238/238 root, 7/7 Sandbox, reverified unchanged at Sprint close |
| Crates | None added, none removed |
| Fixtures | None added |
| `EvidenceCategory` / `FindingSeverity` | Unchanged |
| Governance Register | Unchanged — 13 items, 8 Resolved, 5 Open |
| ADRs | None new |
| Documentation Release | 2.1, unchanged |
| New documents | This planning document; a future Sprint 12 Architectural Resolution document |

---

# 16. Definition of Done

Sprint 12 is complete when:

- The Capability Identity procedure exists as a written, three-axis model plus one orthogonal Introduction check, cited directly against all seven named historical decisions with no contradiction.
- The procedure's own retrospective-bias check (would it have changed any past decision?) is answered honestly and recorded, whatever the answer.
- The Sprint 11 reclassification (Introduction → Expansion) is recorded with an explicit Architectural Classification vs. Product Significance distinction, avoiding any language implying diminishment.
- Collector Composition's resolution explicitly states what the Collection Axis does and does not determine, leaving the Collector Composition Architecture's own extraction threshold untouched, and records the new mutually-exclusive-vs-additive composition guidance as independent of that threshold.
- Rule Composition's resolution explicitly discloses the absence of a direct "second Rule, same category" historical instance rather than presenting the judgment test as equally well-evidenced.
- `cargo fmt --check`, `cargo check --workspace`, and `cargo test --workspace` are re-run and confirmed unchanged from Engineering Release 1.1 (238/238, Sandbox 7/7) — confirmed directly, not assumed, since this Sprint touches no source.
- No Governance Register item, ADR, or Documentation Release amendment has been created, unless this Sprint's own findings demonstrate a genuine cross-cutting gap explicitly named and reasoned about, not silently added.
- The Chief Architect has reviewed and approved the resulting Sprint 12 Architectural Resolution.

---

# Document Status

**Current Version:** 1.1.0

**Status:** Proposed. Awaiting Chief Architect review before Sprint 12 closes. This revision reconciles the Capability Identity procedure (Section 7) following adversarial verification's discovery of a genuine contradiction (Reconciliation Record, above) — the original v1.0.0 procedure is preserved in full in that record and in `SPRINT12_ARCHITECTURAL_RESOLUTION.md`, not deleted. No code, ADR, Governance Register item, test, or fixture has been created or modified; `cargo fmt --check`, `cargo check --workspace`, and `cargo test --workspace` remain unchanged from Engineering Release 1.1 (238/238 root, Sandbox 7/7).

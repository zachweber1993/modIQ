# Initiative 4 Architecture Evaluation — Confidence as a First-Class Concept

| Property | Value |
|---|---|
| **Document** | INITIATIVE_4_ARCHITECTURE_EVALUATION.md |
| **Project** | modIQ |
| **Initiative Addressed** | Engineering Alignment Program, Initiative 4 — Confidence as a First-Class Concept |
| **Purpose** | Evaluate what Confidence measures, where it attaches, how it would be represented and computed, and how it classifies against this repository's existing capability taxonomies — not to design, implement, or adopt anything. |
| **Origin** | Chief Architect authorization, following the closure of Initiative 2's Governance Reconciliation. |
| **Adopted Precedent** | `INITIATIVE_5_ARCHITECTURAL_RESOLUTION.md`, `INITIATIVE_1_ARCHITECTURAL_RESOLUTION.md`, `INITIATIVE_2_ARCHITECTURAL_RESOLUTION.md` / `INITIATIVE_2_GOVERNANCE_RECONCILIATION.md`, `INITIATIVE_3_ARCHITECTURAL_RESOLUTION.md` — listed in the order each was actually resolved (per the Alignment Program's own Recommended Evaluation Order: 5, then 1 and 2, then 3), not by initiative number. All treated as binding, unreopened repository architecture. |
| **Status** | **Architecture Evaluation only. No Architectural Resolution has been performed. No ADR, crate, API, data type, or computation mechanism has been created, designed, or chosen. No Governance Register entry modified.** |

---

## §1 Evaluation Scope

Covers what Confidence measures, where it attaches, its concrete representation, how/where it would be computed, and its classification against this repository's existing capability taxonomies (Capability implementation / Infrastructure expansion / Platform evolution / Architectural Activation, `SPRINT8_ARCHITECTURAL_RESOLUTION.md` §8). Does not design a data type, scoring algorithm, or API. Does not perform an Architectural Resolution.

**Dependency check, per the Alignment Program's own note:** the Program flagged a sequencing concern — evaluating Confidence against a Recommendation shape still in flux from Initiative 3 risked rework. Checked directly against `crates/modiq-runtime/src/assessment/recommendation.rs`: Initiative 3's adopted items did not alter `Recommendation`'s own fields (`id`, `action`, `finding_ids`, `repair_recipe_reference` — unchanged). The concern does not apply; this Evaluation proceeds unblocked.

**Checked against Initiative 2:** Confidence attaches to a Recommendation within a single Assessment and is set once that Recommendation's Finding becomes Final — nothing in its scope touches whether an Assessment may be reentered. Independent of Initiative 2's Question 1, on the same class of reasoning already applied to Initiative 3's Item 10 (a field's own construction-time population versus the aggregate's own lifecycle rules are different questions).

---

## §2 Repository Evidence

- **`docs/constitutional/Glossary.md`**, Confidence entry, read in full: "A measurement indicating how certain modIQ is about an Assessment conclusion based upon available evidence. Confidence reflects evidence quality—not correctness."
- **`docs/constitutional/Principles.md`:148**: "Confidence should communicate certainty without implying correctness where evidence is incomplete."
- **Product Design corpus**, searched in full for every "confidence" occurrence: `WORKSPACE_EVOLUTION.md`, `EVIDENCE.md`, `THE_FINDING.md`, `THE_ASSESSMENT_REPORT.md`, `THE_ASSESSMENT_EXPERIENCE.md`.
- **Interaction Design corpus**, searched in full: `EVIDENCE_EXPLORATION.md`, `ASSESSMENT_REPORT_EXPERIENCE.md`, `ASSESSING_AND_PROGRESSIVE_DISCOVERY.md`, `ASSESSMENT_INTAKE_AND_UPLOAD.md`, `NAVIGATION_AND_WORKSPACE_BEHAVIOR.md`, `INTERACTION_DESIGN_CLOSEOUT_AND_CERTIFICATION.md`.
- **`docs/architecture/RuleEngine.md`** (Frozen), read in full: six Rule Engine responsibilities (Rule Selection, Evidence Evaluation, Finding Generation, Recommendation Generation, Traceability Management, Explainability) — none includes an evidence-quality-scoring step.
- **`docs/architecture/DataModel.md`**, Recommendation section, read in full: "A Recommendation represents actionable guidance generated from one or more Findings. Recommendations exist to improve understanding and assist user decision making. Recommendations are informative rather than mandatory." No mention of Confidence.
- **`docs/architecture/VersionProfile.md`, `KnowledgeModel.md`, `EvidenceCollection.md`** — checked for any Confidence-adjacent scoring language; none found.
- **`crates/modiq-runtime/src/assessment/recommendation.rs`, `finding.rs`** — confirmed directly: `Recommendation` has no Confidence field; `Finding` has no Confidence field. Zero occurrences of "confidence" anywhere in `crates/`.
- **`docs/architecture/Architecture.md`**, System Overview, read in full: seven named components (Evidence Collection, Rule Engine, Version Profiles, Knowledge Base, Reporting, Storage, Extension Layer) orbiting Assessment Service. Confidence is not named as a component, dimension, or dependency anywhere in this document.
- **`docs/engineering/SPRINT8_ARCHITECTURAL_RESOLUTION.md` §8** ("Cross-Cutting Assessment"), read in full: the repository's own defined test for **Architectural Activation**, applied to Version Profiles, and (`INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md`) to Storage at Sprint 13.
- **`docs/engineering/GOVERNANCE.md`, GOV-013**, read in full: `FindingSeverity` Severity/Kind Conflation, Open, unresolved since Sprint 5.
- **Engineering Alignment Program's own Initiative 4 scope note** (`ENGINEERING_ALIGNMENT_PROGRAM.md`): "Confidence is not a new concept being introduced to the platform — it is already a defined constitutional term... that has simply never been implemented," and names the Architectural Activation classification question as this Evaluation's own to resolve.

---

## §3 Current Architecture Baseline

Confirmed directly: Confidence has no type, field, or computation anywhere in `modiq-runtime`, `modiq-rules`, or any other crate. `Recommendation`'s constructor accepts `action`, `finding_ids`, `repair_recipe_reference` only. No Rule Engine responsibility, evidence-evaluation step, or Finding/Recommendation-generation step produces or consumes a quality/certainty value anywhere in the frozen architecture corpus.

---

## §4 Findings

### Q1 — What does Confidence measure, and where does it attach? — Settled, convergent, no tension found

Six-plus Product Design and Interaction Design documents state this identically, with zero internal contradiction across any of them:
- Scoped exclusively to a **Recommendation** — never Evidence (`EVIDENCE.md`: "Confidence belongs to the Recommendation the whole chain supports, never to an individual Evidence item"), never a Finding standing alone (`THE_FINDING.md`: "attached to the Recommendation, never to the Finding itself... consistent with the already-approved decision"), never the Assessment as a whole (`THE_ASSESSMENT_EXPERIENCE.md`: "never inflated into a statement about the whole Assessment").
- Never shown on a Provisional Finding — `WORKSPACE_EVOLUTION.md`: "showing a number would state a certainty the Assessment doesn't have yet. Confidence appears only once a Finding is Final."
- Measures evidence quality, explicitly not correctness (Glossary, `Principles.md`, repeated verbatim across the corpus).

Product Design itself describes this placement as an "already-approved decision" — this is the most settled dimension of the entire initiative.

### Q2 — What is Confidence's concrete representation? — Not specified anywhere

Searched the full frozen corpus for any scale, enum, percentage, or numeric range: none exists. Every reference to Confidence's magnitude is qualitative and experiential — "high," "lower," "proportional to what was actually established" (`THE_ASSESSMENT_EXPERIENCE.md`) — never a defined data shape. This is a genuine, unfilled gap, not an oversight visible anywhere as a placeholder.

### Q3 — Where and how is Confidence computed? — Not specified anywhere; the deepest gap

`RuleEngine.md`'s six responsibilities are the frozen, authoritative description of everything the Rule Engine does between consuming Evidence and producing Findings/Recommendations. None of the six — including Evidence Evaluation, "the core responsibility of the Rule Engine" — includes a quality-scoring or certainty-computation step. "Evidence quality" is used descriptively throughout Product Design without ever being operationalized into a rule, weighting scheme, or algorithm anywhere in Runtime Architecture. This matches the Alignment Program's own framing of the initiative's purpose exactly: not a missing field, but missing interpretive logic that doesn't yet exist anywhere to compute a value from.

### Q4 — Does Confidence fit the Architectural Activation precedent? — Genuine, unresolved classification tension

Applying `SPRINT8_ARCHITECTURAL_RESOLUTION.md` §8's own three-part test directly:

- **Against "Capability implementation":** Sprint 8's own distinguishing test for what *isn't* ordinary capability work is whether it "forces a change to [a] previously-stable public signature" — true of Version Profiles (`Assessment::new`) and, on current evidence, very likely true of Confidence (`Recommendation::new` would need to change to accept it). This one trait does **not** match ordinary Capability implementation, the same way Version Profiles didn't.
- **Against "Infrastructure expansion":** Confidence is motivated by an explicit, longstanding Product Design requirement, not built as generic machinery for its own sake — doesn't appear to fit this classification either.
- **Against "Platform evolution":** scoped to one field on one entity plus a computation step, not "broader, more open-ended architectural change" — narrower than this classification, similar to Version Profiles.
- **Against "Architectural Activation":** its defining test is realizing "an already-specified-but-dormant architectural dimension." Both of this repository's actual instances (Version Profiles, Storage) satisfied this through an explicit textual placeholder written into Runtime Architecture or System Overview itself — `VersionProfile.md`/`ADR-0004` plus `EngineAPI.md`'s "(once implemented)" note; Storage's own presence in `Architecture.md`'s System Overview since baseline. **Checked directly against the same three documents for Confidence — `DataModel.md`'s Recommendation section, `RuleEngine.md`, and `Architecture.md`'s System Overview — and found no comparable placeholder in any of them.** It is specified only at the constitutional/Product Design tier; nothing checked in Runtime Architecture reserves a place for it the way both prior Architectural Activation instances had.

**Finding:** Confidence shares one distinguishing trait with the Architectural Activation precedent (forces a previously-stable signature change, ruling out ordinary Capability implementation) but lacks the other trait both of that precedent's actual instances shared (a Runtime-Architecture-tier dormant placeholder, not found in any of the three documents checked). It does not cleanly fit any of the four existing classifications against the exact tests this repository itself established. Not resolved here — named precisely as a classification question for whatever comes next.

### Q5 — Does GOV-013 have a demonstrated dependency with Confidence, or only a procedural-proximity one?

Checked directly rather than assumed: `FindingSeverity` (Finding-level, an urgency axis per GOV-013's own description) and Confidence (Recommendation-level, a certainty axis) address different entities and different questions. No repository evidence found showing Confidence's value would need to read, depend on, or derive from `FindingSeverity`. The Alignment Program's own note recommending they be "considered... alongside" this initiative appears to rest on shape/proximity — both are open questions about whether a single graded value needs richer structure — not a demonstrated architectural coupling. Treating them as mechanistically linked would exceed what repository evidence currently shows.

---

## §5 Adjacent, Not Resolved Here

- The Glossary's missing "Recommendation" entry (Initiative 3's Item 8, still unresolved) — Confidence's own placement rules depend throughout on a "Recommendation" concept that itself lacks a canonical Glossary entry. Not a new finding; re-flagged as directly relevant to whatever governs Confidence next.
- GOV-013 itself remains Open and unresolved; this Evaluation does not act on it, per Q5.

---

## §6 Initiative Boundaries

**Unblocked by:** Initiative 3 (confirmed — Recommendation's field shape did not change).

**Independent of:** Initiative 2's Question 1 (checked directly, §1).

**Not performed here:** data type selection, scoring algorithm or rubric design, `FindingSeverity`/GOV-013 resolution, Glossary amendment, classification adoption under any of the four existing taxonomies.

---

## Evaluation Boundaries

**Questions this evaluation answered:** what Confidence measures and where it attaches (Q1, settled and convergent); whether a concrete representation or computation mechanism currently exists (Q2, Q3 — neither does, confirmed against the full frozen corpus and all of Runtime Architecture); whether Confidence fits this repository's existing capability classifications (Q4 — checked against Sprint 8's own exact test and found to match one distinguishing trait but not the other, a genuine unresolved tension rather than a clean fit); and whether GOV-013 is mechanistically coupled to this initiative (Q5 — checked and not found, only shape-proximity).

**Questions intentionally not addressed:** Confidence's concrete data type or scale; the computation mechanism or rubric; which classification (if any) Initiative 4 should ultimately proceed under; GOV-013's own resolution; the Glossary's Recommendation-entry gap.

**Adopted precedent relied upon without reopening:** Initiative 3's confirmation that `Recommendation`'s fields are settled; Initiative 2's Question 1 disposition, unreopened; Sprint 8's Architectural Activation classification and its exact defining test, applied but not altered.

No Architectural Resolution has been performed. This evaluation's responsibility ends here, awaiting Chief Architect review.

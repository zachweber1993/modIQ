# Initiative 3 Architecture Evaluation — Domain Model Anatomy Extension

| Property | Value |
|---|---|
| **Document** | INITIATIVE_3_ARCHITECTURE_EVALUATION.md |
| **Project** | modIQ |
| **Initiative Addressed** | Engineering Alignment Program, Initiative 3 — Domain Model Anatomy Extension, executed under the approved execution structure (3A independent items, 3B Item 10 dependency determination, 3C deferred watch list) |
| **Purpose** | Evaluate the field-level and type-shape questions Initiative 3 was scoped to address, and complete 3B's own narrow determination for Item 10 — not implementation, not API design, not Architectural Resolution. |
| **Origin** | Chief Architect authorization, following approval of the dependency analysis and execution structure covering this initiative. |
| **Adopted Precedent** | `INITIATIVE_5_ARCHITECTURAL_RESOLUTION.md`, `INITIATIVE_1_ARCHITECTURAL_RESOLUTION.md`, `INITIATIVE_2_ARCHITECTURAL_RESOLUTION.md`, all treated as binding, unreopened repository architecture. Question 1's Requires Governance Reconciliation disposition is not revisited. |
| **Status** | **Architecture Evaluation only. No Architectural Resolution has been performed. No ADR, crate, API, or implementation technology has been created, designed, or chosen. No Governance Register entry modified.** |

---

## 1. Evaluation Scope

Covers 3A (all items classified Independent in the prior dependency analysis) and 3B (Item 10's own dependency determination, scoped narrowly per explicit instruction: determine dependency only, do not design the field or adopt new architecture). 3C (Report currency, the Updated marker) is not evaluated here — it remains a watch list pending Initiative 2's Governance Reconciliation, per the approved execution structure.

**Deliberately excluded:** implementation, API/payload design, ADR creation, Governance Register updates, any revisiting of Initiative 2's Question 1.

**A repository-state note, not part of this evaluation's evidence base:** `docs/platform/PlatformSpecification.md` establishes a new "Platform Architecture" lineage, committed and pushed to this branch (commit `1191a4b`) under separate, Product-Owner-directed review outside this evaluation's own process. It explicitly disclaims authority "at or beneath the Assessment" — the entirety of Initiative 3's scope. Not treated as evidence here.

---

## 2. Repository Evidence

Reviewed directly this session, several for the first time despite bearing on this scope:

- **`docs/constitutional/Glossary.md`** (Frozen) — read in full. Settles Mod Health's six dimensions (Item 2), gives concrete content for Repair Recipe's referenced change-kinds (Item 5), confirms no standalone "Recommendation" entry exists (Item 8), confirms `EvidenceCategory`'s current eight variants already match Glossary's own Evidence examples exactly (Item 9).
- **`docs/architecture/KnowledgeModel.md`** (Frozen) — read in full. Confirms Repair Recipes remain descriptive/instructional, never executable.
- **`docs/architecture/EvidenceCollection.md`** (Frozen, Documentation Release 2.1) — read in full. The Collector Contract's Outputs and Non-Responsibilities sections are directly load-bearing for Item 4 and the Architectural Observation below.
- **`crates/modiq-runtime/src/assessment/finding.rs`, `evidence.rs`, `recommendation.rs`** — re-confirmed directly: no mutation method exists on any of the three types; each has a fallible constructor and getters only.
- **`crates/modiq-runtime/src/assessment/assessment.rs`** — re-confirmed: `add_finding` is `self.findings.push(finding)` — pure append; no replace or remove path exists anywhere.
- **`docs/adrs/0007-runtime-entity-design-pattern.md`** — re-consulted specifically for the Aggregate Root Pattern's own boundary (what Question 1 governs versus what it does not), central to Item 10's tightened determination.
- **`docs/engineering/GOVERNANCE.md`, GOV-002 entry** — consulted fresh for Item 6a: confirms directly that `AssessmentSubject`/`AssessmentContext`'s zero-field state was already evaluated once, at Sprint 15, and dispositioned "require no action."
- **`docs/engineering/ENGINEERING_ALIGNMENT_PROGRAM.md`** §5 and **`docs/engineering/GOVERNANCE.md`**'s GOV-013 entry and Crate Boundary Rules.
- **`INITIATIVE_1_ARCHITECTURAL_RESOLUTION.md`, `INITIATIVE_2_ARCHITECTURAL_RESOLUTION.md`** — consulted as adopted precedent.
- **Product Design** — The Finding, Evidence, and The Assessment Report, re-consulted for the specific anatomy each item is meant to satisfy.

---

## 3. Current Architecture (Baseline, Unaffected by This Evaluation)

Restated briefly, all already established in the Engineering Reconciliation: `Finding` has `severity`, `description`, `evidence_ids`, `rule_reference` — no title/summary split, no category. `Evidence` has `category`, `description`, `location` — no label/source/explanation/content split. `RuleOutcome` pairs a mandatory `Finding` with a mandatory `Recommendation`. `RepairRecipe` has `identifier` and a flat `guidance: String`. `AssessmentReport` has no Subject-identity or currency field. `AssessmentSubject` is a zero-field marker struct, confirmed still true. None of `Finding`, `Evidence`, or `Recommendation` has any mutation method anywhere in the workspace.

---

## 4. 3A — Independent Items

### Item 1 — Finding Title/Summary Decomposition

**Observation:** `Finding.description` is one flat string, authored entirely by the constructing Rule at construction time. Splitting it into `title`/`summary` requires no new information any Rule doesn't already have. No conflict found in `Glossary.md`, `DataModel.md`, or `RuleEngine.md`.

**Recommendation:** proceeds independently.

### Item 2 — Finding-Level Category (Mod Health Dimension)

**Observation:** `Glossary.md` already freezes the exact closed set — Compatibility, Stability, Maintainability, Performance, Structure, Engineering Quality — under "Mod Health." Assignment follows the same pattern already proven for Severity.

**Recommendation:** proceeds independently, with one internal precondition: cannot be named `Category` without Item 9's naming collision resolved first — an ordering dependency within 3A, not on Initiative 2.

### Item 3 — Optional Recommendation Cardinality on `RuleOutcome`

**Observation:** `RuleOutcome` pairs `Finding` and `Recommendation` mandatorily today, confirmed directly in source. Product Design requires a Finding be able to stand alone. Entirely internal to `modiq-rules`, unrelated to Assessment lifecycle.

**Recommendation:** proceeds independently.

### Item 4 — Evidence Anatomy (Label/Source/Explanation/Content)

**Observation:** this item's evaluation surfaced a finding whose significance exceeds an ordinary field-shape question. Label, Source, and Content extend the existing Collector Contract without apparent conflict and proceed as low-risk additions. Explanation's own status is different in kind — see the Architectural Observation immediately following §4, which separates four distinct questions this item otherwise conflates.

**Recommendation:** Label/Source/Content proceed independently. Explanation's own disposition is not evaluated at the field level — see the Architectural Observation.

### Item 5 — RepairRecipe Structured-Step Shape

**Observation:** `Glossary.md` already specifies what a Repair Recipe may reference — XML changes, Lua changes, Dependency installation, Asset replacement, Version updates — giving concrete, frozen-tier content beyond the flat `guidance: String` that exists today. Entirely within `modiq-knowledge`, independent of any specific Assessment.

**Recommendation:** proceeds independently.

### Item 6a — Report Identity (Assessment Subject Statement)

**Observation:** `AssessmentReport` has no Subject-identity field today. This is not a newly discovered gap. `AssessmentSubject`'s zero-field state was already found and evaluated once before, at Sprint 15's GOV-002 Architecture Evaluation, which recorded directly: "`AssessmentSubject`/`AssessmentContext`'s minimal content... were both evaluated and require no action" (`GOVERNANCE.md`, GOV-002). That disposition was reached without a concrete requirement for `AssessmentSubject`'s own content existing at the time — the same evidentiary posture GOV-001 held before Initiative 2's evaluation introduced new repository context bearing on it, and GOV-008 held before Initiative 1's evaluation did the same.

**Why this is significant beyond Item 6a's own scope:** Report Identity's Subject-statement requirement introduces a concrete requirement for `AssessmentSubject`'s content that was absent during GOV-002's own evaluation — not evidence that GOV-002's disposition was wrong when it was made, but a change in repository context that may bear on whether it still holds. Whether GOV-002 should actually be reopened is a governance action this evaluation does not perform and is not positioned to decide.

**Recommendation:** proceeds independently of Initiative 2. Item 6a's own field addition cannot be meaningfully completed without `AssessmentSubject` gaining real content first — a precondition named precisely, with its own prior governance history now attached, not a fresh discovery and not a redesign proposed here.

### Item 7 — GOV-013 (Severity/Kind Conflation)

**Observation:** unchanged from prior analysis — an evaluation input, not resolved here.

### Item 8 — Glossary's Missing "Recommendation" Entry

**Observation:** confirmed directly against `Glossary.md` this session — no "Recommendation" heading exists. A Product Owner documentation decision, not resolved here.

### Item 9 — `EvidenceCategory`/Category Naming Collision

**Observation:** `Glossary.md`'s own Evidence entry lists categories matching `EvidenceCategory`'s current eight variants exactly, confirming it is correctly Glossary-grounded. Mod Health's six dimensions are a distinct, separately-named closed set in the same Glossary. The collision is real, confirmed from the authoritative source.

**Recommendation:** must be resolved (naming only) before Item 2's field is added under any name.

---

## Architectural Observation — Evidence's Explanation Field Conflates Four Separable Questions

| Property | Value |
|---|---|
| **Arising from** | Item 4 (Evidence Anatomy), §4 |
| **Nature** | An observation distinguishing a genuine cross-subsystem architectural question from an ordinary field-shape decision. Not resolved here. |
| **Status** | Observation only. No architecture adopted. No implementation designed. |

### The Four Questions, Separated

Item 4, read as a single "add four fields to Evidence" task, conflates four questions that require different evidence to answer:

1. **Field anatomy** — what Evidence's structure should contain. Label, Source, and Content extend the existing `category`/`description`/`location` shape without apparent conflict.
2. **Subsystem responsibility** — who authors each field. Label/Source/Content remain Collector-authorable, consistent with `EvidenceCollection.md`'s existing Collector Contract. Explanation cannot be — it requires knowing "why it matters for the Finding," and a Collector explicitly does not know about Findings, Rules, or Recommendations (`EvidenceCollection.md`, Non-Responsibilities: "factual, not evaluative").
3. **Construction timing** — when each field is populated. Every field on every Runtime entity today is populated exactly once, at construction (ADR-0007). Explanation, if authored by the Rule Engine (the only subsystem with Finding-relative authority), would be populated *after* Evidence already exists — the first case in this platform's history where a Runtime entity's own field is not fully known at its own construction.
4. **Entity ownership** — whether Explanation is even a property of `Evidence` at all, versus a property of the *relationship* between a Finding and the Evidence it cites. A Finding-owned, per-cited-Evidence explanation is a structurally different answer than an Evidence-owned field.

### Why This Rises Above Item 4's Own Scope

Questions 1–2 are ordinary field-shape and authorship questions, resolvable by the same method every other 3A item used. Questions 3–4 are not — they ask whether resolving Item 4 requires either the first exception to this platform's universal single-stage, immutable-at-construction entity pattern (ADR-0007), or relocating a piece of anatomy Product Design assigned to Evidence onto a different entity or relationship entirely.

### Not Resolved Here

This document does not decide which subsystem authors Explanation, whether Evidence's construction model changes, or where Explanation's anatomy ultimately belongs.

---

## 5. 3B — Item 10 Dependency Determination

**Question:** does the Provisional/Final field's own representation on `Finding` depend on Initiative 2's unresolved Question 1?

**Determination: Independent.**

**Evidence, without reasoning about either candidate lifecycle model:** Question 1 concerns `AssessmentStatus`'s own transition rules (INV-012) — a property of the Assessment *aggregate*. ADR-0007's Aggregate Root Pattern draws this boundary explicitly: `Assessment` is the aggregate; `Finding`, `Evidence`, and `Recommendation` are entities it owns, mutated only through the aggregate's own methods — and, confirmed directly against source again for this determination, none of the three has any mutation method of its own, at all, today. Every field on `Finding` is populated exactly once, at construction, through its single fallible constructor. Question 1's own scope never reaches this pattern: resolving it, in whatever direction, changes what the *aggregate* permits (whether `add_finding` may be called again after `Completed`); nothing proposed on either side touches how an *individual Finding's own fields* are populated. A Provisional/Final field would follow the same constructor-only pattern every other Finding field already follows, unconditionally on how Question 1 resolves, because Question 1 never reaches that pattern at all.

**Item 10 moves to 3A's own scope**, per the execution structure's own stated exit condition.

---

## 6. 3C — Deferred Watch List (Unaffected, Not Evaluated)

Report currency (Item 6b) and the Updated marker (Item 12) remain exactly as Initiative 2's Resolution left them — deferred pending Governance Reconciliation of Question 1. No action available for Initiative 3 to take on them yet.

---

## 7. Initiative Boundaries

**Initiative 2 (unresolved, not reopened):** Question 1 itself, and 3C's two items, remain entirely Initiative 2's own territory.

**Initiative 4 (Confidence):** unaffected.

**Governance (not performed here):** GOV-013's own resolution; the Glossary's Recommendation entry; the naming choice resolving Item 9; whether GOV-002 should be reopened given Item 6a's own newly-introduced requirement.

---

## Evaluation Boundaries

**Questions this evaluation answered:** all nine 3A items were checked against repository evidence, not assumed independent by default. Item 4 was found to conflate four separable questions, two of which (construction timing, entity ownership) exceed ordinary field-shape evaluation and are recorded as a distinct Architectural Observation. Item 6a was found to connect to an already-resolved Governance Register item (GOV-002) rather than being a new discovery; Report Identity introduces a concrete requirement that was absent during GOV-002's own evaluation, worth reconsideration though not decided here. Item 10 was determined Independent from Question 1's own scope boundary (the aggregate's transition rules versus the entity-design pattern ADR-0007 establishes) and the already-confirmed absence of any mutation method on `Finding`, without reasoning about either candidate lifecycle model's own hypothetical implementation.

**Questions intentionally not addressed:** 3C's two items. GOV-013's resolution, the Glossary gap, and whether to reopen GOV-002 all remain named as inputs, not resolved.

**Adopted precedent relied upon without reopening:** Initiative 1's AC-1 through AC-5 and D-1/D-2; Initiative 2's Question 1 disposition and its already-adopted finding that Findings are immutable and append-only, which Item 10's determination depends on directly; GOV-002's own prior resolution, cited but not reopened.

No Architectural Resolution has been performed. This evaluation's responsibility ends here, awaiting Chief Architect review.

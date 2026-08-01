# Architecture Evaluation — Product Specification / Knowledge Model Consistency Check

| Property | Value |
|---|---|
| **Document** | ARCHITECTURE_EVALUATION_KNOWLEDGE_MODEL_CONSISTENCY.md |
| **Project** | modIQ |
| **Purpose** | Determine whether `ProductSpecification.md` and `KnowledgeModel.md` make incompatible constitutional claims about knowledge accumulation, or describe consistent, different levels of abstraction — narrowly scoped per Chief Architect direction, addressing the tension named in `CAPABILITY_DEFINITION_KNOWLEDGE_FEEDBACK_LOOP.md` §5 |
| **Origin** | Chief Architect direction, following review of the Knowledge Feedback Loop Capability Definition |
| **Status** | **Architecture Evaluation complete; Architectural Resolution complete (Section 6).** No conflict found. No amendment drafted; no ADR created; no Governance Register item opened; no implementation authorized. |

---

## 1. Architectural Question

Restated exactly as posed:

> Do the cited Product Specification and Runtime Knowledge Model actually make incompatible constitutional claims, or are they describing different levels of abstraction that remain consistent when read together?

Per explicit instruction: grounded exclusively in the existing frozen documents; no reinterpretation of intent beyond what a document states about itself; no solution proposed; no amendment drafted; no implementation recommended.

---

## 2. Method

This evaluation treats two kinds of textual evidence differently:

- **A document's explicit, self-declared statements about its own purpose, scope, and authority relationships** (e.g., a Purpose section stating what the document does and does not cover, a Specification Authority section naming which documents govern it) are treated as direct evidence — reading and applying a rule a document states about itself is not "reinterpreting intent."
- **Word choice, actor attribution, and structural placement within the cited passages themselves** are treated as textual signals to be weighed, not assumed to settle the question alone.

No claim is made here about what any author privately intended beyond what the text states.

---

## 3. Evidence Reviewed

- `docs/constitutional/ProductSpecification.md` — Specification Authority section; Purpose section; "Knowledge Base (MKB)" section (under Product Overview... actually under its own heading — see 4.1); Assessment Workflow (all seven steps); "Community Contributors (Future)" target user; Product Goals.
- `docs/constitutional/Glossary.md` — Purpose section; "Knowledge Base (MKB)" entry.
- `docs/architecture/KnowledgeModel.md` — Specification Authority section; Purpose section; "Relationship to Other Specifications" (Runtime Domain / Knowledge Domain); Knowledge Modeling Principles ("Knowledge First," "Independent of Runtime," "Extensible"); Domain Overview; the seven Knowledge Domain entity definitions; "Knowledge Relationships"; "Knowledge Evolution"; "Future Evolution."
- `docs/architecture/Architecture.md` — Storage Layer section; Platform Boundaries section.
- `docs/engineering/GOVERNANCE.md` — Storage Crate Boundary Rule; Knowledge Domain Crate Boundary Rule.
- **Repository fact, not interpretation:** `modiq-knowledge`'s only real content (`RepairRecipe::version_compatibility_declared_version_mismatch()`, Sprint 9) was hand-authored by an engineer inside `modiq-knowledge` itself, confirmed directly against source (`crates/modiq-knowledge/src/knowledge/repair_recipe.rs`) and against `SPRINT9_CAPABILITY_DEFINITION.md`/`INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md`'s own characterization of it as "not derived from any Assessment the platform ever ran."

---

## 4. Findings

### 4.1 What each document claims, precisely

**`ProductSpecification.md`**, "Knowledge Base (MKB)" section: *"The Knowledge Base preserves validated engineering knowledge generated through Assessments."* Stored content: "Known issues, Repair Recipes, Compatibility history, Dependency relationships, Engineering patterns, **Historical Assessments**." Assessment Workflow, step 7 of 7: *"Knowledge is incorporated into the MKB where appropriate."*

**`Glossary.md`**, "Knowledge Base (MKB)" entry: *"The continuously evolving repository of validated knowledge accumulated through Assessments... stores patterns, known issues, repair strategies, compatibility history, and engineering knowledge that improve future Assessments."*

**`KnowledgeModel.md`**, in five separate places: *"Knowledge exists independently of runtime execution."* / *"Runtime entities consume knowledge but do not own it."* / *"Knowledge exists independently of Assessments. Knowledge may evolve over time without altering historical Assessment records."* / *"Knowledge is preserved independently of Assessment execution. Assessments consume engineering knowledge to produce Findings and Recommendations."* / *"Historical Assessments remain unchanged because runtime history is separated from the evolving knowledge domain."*

Read only at this level — quotation against quotation, with no further context — these do appear to describe opposite directions of the same relationship: one side names Assessments as the *source* knowledge is generated/accumulated from; the other names Assessment execution as something knowledge is *independent of* and *separated from*.

### 4.2 The self-declared authority and deferral relationship between the two documents

`ProductSpecification.md`'s own Purpose section states directly: *"Implementation details are intentionally excluded and are defined within the technical specifications."* Its own Specification Authority section states directly: *"This document governs: Architecture.md, DataModel.md, **KnowledgeModel.md**, RuleEngine.md, EngineAPI.md."* `KnowledgeModel.md`'s own Specification Authority section independently confirms the same relationship from its own side, listing `ProductSpecification.md` among the documents whose authority it is subject to.

This is not an inference about intent — it is each document's own explicit, stated self-description of the relationship between them: `ProductSpecification.md` states what the platform provides at a product level and explicitly declines to specify mechanism; `KnowledgeModel.md` is one of the documents `ProductSpecification.md` itself names as where that mechanism is defined. `KnowledgeModel.md`'s own Purpose section states the parallel claim from its side: *"This specification intentionally avoids implementation details including databases, storage technologies, serialization formats, APIs, and programming language constructs. Its purpose is to establish a stable conceptual knowledge model."* Both documents describe themselves as *conceptual*, at different scopes — `ProductSpecification.md` at the product/capability level, `KnowledgeModel.md` at the domain-model level — and neither claims to specify a runtime data-flow mechanism (an automated pipeline, a scheduled job, a human review process) for how knowledge content comes to exist. That question is not answered by either document; it is left open by both, on each document's own stated terms.

### 4.3 Textual signals distinguishing Assessment Workflow step 7 from steps 2–6

The Assessment Workflow's other six steps each name the acting subsystem explicitly and describe a deterministic, unconditional action: *"Assessment Engine collects evidence,"* *"Rule Engine evaluates evidence,"* *"Findings are generated,"* *"Conclusions are produced,"* *"Assessment Report is generated."* Step 7 breaks this pattern on two independent counts: it names no acting subsystem (contrast "Rule Engine evaluates" with the actor-less "Knowledge is incorporated"), and it carries a conditional qualifier no other step has — *"where appropriate."* None of steps 1–6 admit of degree or judgment; each happens, unconditionally, every time `AssessmentService::execute` runs, confirmed directly against source. A step requiring a judgment about *appropriateness*, with no named mechanical actor, is textually distinct in kind from the six deterministic steps surrounding it in the same numbered list.

### 4.4 The "Historical Assessments" category mismatch

`ProductSpecification.md`'s and `Glossary.md`'s MKB content lists both include "Historical Assessments" (`ProductSpecification.md`) or reference knowledge that "improve[s] future Assessments" (`Glossary.md`). **`KnowledgeModel.md`'s own seven-category Knowledge Domain enumeration — Rule, Repair Recipe, Engine Behavior, Compatibility Pattern, Best Practice, Known Issue, Knowledge Reference — does not include "Historical Assessments" as a category at all.** `KnowledgeModel.md`'s own "Relationship to Other Specifications" section assigns Assessment/Evidence/Findings/Recommendations/Assessment Report to the *Runtime* Domain (`DataModel.md`), explicitly distinct from the *Knowledge* Domain it itself defines. Separately, `Architecture.md`'s Storage Layer section lists "assessment persistence" and "historical records" as Storage responsibilities, alongside — not identical to — "knowledge storage," as three items in one list. Read together, "Historical Assessments" in `ProductSpecification.md`'s MKB list names Runtime-domain content (already realized today by `modiq-storage`, per `Architecture.md`'s own Storage Layer), not a category `KnowledgeModel.md` itself was ever meant to define. `ProductSpecification.md` uses "MKB" as a product-level umbrella spanning both what `Architecture.md` splits into Storage and Knowledge Base responsibilities; `KnowledgeModel.md` uses "Knowledge Base"/"Knowledge Domain" in the narrower, technically-scoped sense it itself defines. This is a difference in the *breadth* of what each document's term covers, not a disagreement about what either document says happens to any specific entity.

### 4.5 Cross-check against `GOVERNANCE.md`'s Crate Boundary Rules

Storage: *"Must never... participate in Evidence Collection, Rule Evaluation, or Report generation, or be consulted during any of them."* Knowledge Domain: *"Must remain independent from any individual Assessment."* Both rules are scoped to **a live Assessment's own execution** — they govern what may be consulted while `AssessmentService::execute` is running for one specific Assessment. Neither rule addresses, in either direction, whether engineering judgment exercised *outside* that execution — during a separate Capability Definition, Architecture Evaluation, or Sprint, of the kind this repository has performed for every one of `modiq-knowledge`'s and `modiq-versioning`'s own activations — may be informed by patterns observed across many already-completed, already-persisted Assessments. The rules constrain the live execution path; they are silent on the governed, out-of-band process by which `modiq-knowledge`'s own content has, in every real instance to date, actually come to exist.

### 4.6 Cross-check against the repository's own demonstrated practice

`modiq-knowledge`'s sole real content (Sprint 9) was authored directly by an engineer, informed by the platform's own domain understanding of version-compatibility failures — not generated by any software mechanism reading `Assessment` or `AssessmentReport` data. This is a fact about the repository's history, not an interpretation: it is exactly the shape `ProductSpecification.md`'s "generated through Assessments" and `Glossary.md`'s "accumulated through Assessments" would take if those documents describe a *governed, human-authored curation process informed by real-world assessment activity* rather than an automated feedback pipeline. No instance to date contradicts this reading; none has ever involved an automated write from Runtime or Storage data into `modiq-knowledge`.

---

## 5. Conclusion

**No genuine constitutional conflict is found between `ProductSpecification.md`/`Glossary.md` and `KnowledgeModel.md`.** The apparent incompatibility identified in `CAPABILITY_DEFINITION_KNOWLEDGE_FEEDBACK_LOOP.md` §5 does not survive being checked against the documents' own stated scope and authority relationship:

- `ProductSpecification.md` explicitly states it excludes implementation detail and explicitly names `KnowledgeModel.md` as one of the documents that defines it. `KnowledgeModel.md` independently confirms this same relationship from its own side. Both documents are self-declared as *conceptual*, operating at different scopes (product capability vs. domain model), and neither specifies a mechanism (automated or human-mediated) for how Knowledge Domain content comes to exist.
- Read at the level of mechanism, the two are not merely reconcilable but consistent: `ProductSpecification.md`/`Glossary.md` describe *what the MKB is for and where its content ultimately originates from* (validated understanding, whose source material is real assessment activity); `KnowledgeModel.md` describes *the runtime relationship between the two domains as engineered artifacts* (Knowledge is a stable input to any given Assessment's execution, never a side effect an Assessment's own execution automatically produces). These are answers to different questions, not contradictory answers to the same question.
- This reading is supported by textual signals internal to `ProductSpecification.md` itself (step 7's distinct, judgment-qualified, actor-less phrasing set against the six deterministic, named-actor steps around it), by `GOVERNANCE.md`'s own Crate Boundary Rules (scoped specifically to a live Assessment's execution, silent on out-of-band engineering process), and by the repository's own demonstrated practice (Sprint 9's real, human-authored precedent).
- The one place a genuine textual mismatch exists — "Historical Assessments" appearing in `ProductSpecification.md`'s/`Glossary.md`'s MKB content description without appearing among `KnowledgeModel.md`'s own seven Knowledge Domain categories — is a **scope/breadth difference, not a contradiction**: `KnowledgeModel.md`'s own Relationship to Other Specifications section already assigns Assessment-history content to the Runtime Domain, and `Architecture.md`'s Storage Layer already separately names "historical records" as Storage's own responsibility, distinct from "knowledge storage" in the same list. `ProductSpecification.md` uses "MKB" as a broader product-level umbrella than `KnowledgeModel.md`'s own narrower, technically-scoped term.

**Precise scope of the one residual observation, named but not a conflict:** `ProductSpecification.md`'s MKB content list and `KnowledgeModel.md`'s Knowledge Domain category list do not use identical vocabulary for identical scope — the former is broader (spans Storage and Knowledge Base responsibilities as `Architecture.md` divides them) than the latter (Knowledge Domain content only). Nothing in this evaluation determines that this naming looseness requires correction; it is noted only because it was the specific evidentiary basis (Section 4.4) for finding no contradiction, and a Chief Architect reviewing this evaluation should have it named precisely rather than left implicit.

---

## 6. Architectural Resolution

**Resolution decision: no architectural conflict exists.** Section 5's finding is adopted without modification. A further, forward-binding Adopted Architectural Constraint is additionally recorded (6.2), narrower in scope than the finding itself: it does not depend on or qualify the "no conflict" determination, but closes, explicitly, the one question the evaluation itself declined to reach (Section 6, prior revision) — how any future capability realizing this accumulation must be shaped so that it remains consistent with what Section 5 found.

### 6.1 Determination

**Is there a conflict between `ProductSpecification.md`/`Glossary.md` and `KnowledgeModel.md`?** No. Confirmed exactly as Section 5 found: the documents answer different questions — product-level source of validated understanding versus the runtime relationship between the Knowledge and Runtime domains as engineered artifacts — consistent with each document's own self-declared scope and authority relationship (§4.2). No finding in Section 4 is revised by this Resolution.

### 6.2 Adopted Architectural Constraint

Binding on any future work realizing knowledge accumulation from Assessment activity:

1. Knowledge remains an input to deterministic Assessment execution.
2. Any accumulation, refinement, or incorporation of validated knowledge derived from Assessment activity occurs outside live Assessment execution.
3. Such mechanisms must not alter or participate in the deterministic execution path of an Assessment.

This constraint does not amend `KnowledgeModel.md` or `GOVERNANCE.md`'s Storage and Knowledge Domain Crate Boundary Rules — it confirms them. Per §4.5, both existing rules already scope their own prohibition to a *live* Assessment's execution and are silent on out-of-band engineering process; this constraint states that boundary explicitly and prospectively, as a condition any future accumulation mechanism must satisfy by construction, rather than leaving it to be re-derived from the existing rules' own silence each time a new capability is proposed.

**No amendment is required.** Neither `KnowledgeModel.md` nor `GOVERNANCE.md` is edited by this Resolution — both already state, in their own existing text, the boundary this constraint makes explicit and forward-binding.

### 6.3 Governance Disposition

**No new Governance Register item is opened.** This evaluation concluded no architectural conflict exists; per Chief Architect direction, that conclusion does not itself introduce a new governance question requiring lifecycle tracking (Evidence → Governance Initiation → Chief Architect Approval → ... per the repository's stage-gated model) — this Resolution records the outcome of the evaluation, it does not establish a new governance case. Precedent: Sprint 12's own Capability Identity procedure similarly required no new Governance Register item and no ADR, on the same reasoning — it applied and extended already-approved architecture rather than establishing a new durable principle requiring independent tracking (`GOVERNANCE.md`'s own "When to Create an ADR" guidance, `CHIEF_ARCHITECT_HANDOFF_v1.1.md` §4).

### 6.4 Disposition of Findings

| Finding | Disposition |
|---|---|
| §5 — no conflict found between the cited specifications | Adopted |
| §4.4 — "Historical Assessments" is a scope/breadth difference, not a contradiction | Adopted as a named observation; no correction required |
| Adopted Architectural Constraint (§6.2) — live-execution boundary, binding on future accumulation mechanisms | Adopted |
| Whether `KnowledgeModel.md` or `GOVERNANCE.md` requires amendment | No amendment required |
| Whether a new Governance Register item is warranted | Not opened, per §6.3 |

**No ADR was created. No Governance Register item was opened. No implementation was authorized.** This Resolution authorizes resuming Capability Definition for the Knowledge Feedback Loop capability under the constraint recorded in §6.2 — a separate document (`CAPABILITY_DEFINITION_KNOWLEDGE_FEEDBACK_LOOP.md`), not this evaluation.

---

## 7. What This Evaluation Does Not Decide

Per explicit scope: this evaluation does not decide the Knowledge Feedback Loop capability's own scope, mechanism, or whether it should proceed to implementation — only that it may proceed to Capability Definition, bounded by §6.2's constraint. That work is recorded separately in `CAPABILITY_DEFINITION_KNOWLEDGE_FEEDBACK_LOOP.md`.

No amendment was proposed. No implementation was authorized.

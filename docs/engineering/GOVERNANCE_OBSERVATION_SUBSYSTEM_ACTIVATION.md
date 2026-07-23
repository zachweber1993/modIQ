# Governance Observation — Capability Identity Scope and Subsystem Activation

| Property | Value |
|---|---|
| **Document** | GOVERNANCE_OBSERVATION_SUBSYSTEM_ACTIVATION.md |
| **Project** | modIQ |
| **Purpose** | Observe and evidence a governance reconciliation question surfaced by `INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md`: the repository already contains two named architectural taxonomies — Sprint 8's **Architectural Activation** and Sprint 12's **Capability Identity** procedure — derived independently, at different times, for differently-scoped work, and never reconciled with each other. This is an **Observation**, per this project's own Decision Framework (Observation → Evidence → Investigation → Governance) — it names and evidences the tension; it does not investigate options toward a recommendation, and it does not decide anything. |
| **Origin** | Directed by the Chief Architect following INV-002, which found that the Capability Identity procedure could not classify the Storage/Persistence candidate (INV-002 §3). **This document was subsequently revised** after the Chief Architect identified that `SPRINT8_ARCHITECTURAL_RESOLUTION.md` already defines, names, and applies a classification — Architectural Activation — for exactly this shape of work, predating Sprint 12. The original framing (no governing concept exists) is corrected below; the underlying tension (two taxonomies, never reconciled) remains real and is the subject of this document. |
| **Status** | **Complete as an Observation, revised. No recommendation is made. No Governance Register item is opened. No promotion or adoption of Sprint 8 terminology is recommended. Capability Definition, Architecture Evaluation, and Sprint Planning are all unaffected and unauthorized by this document.** |

---

## 1. Observation

`Architecture.md`'s own System Overview names seven participants cooperating under the Assessment Service: Evidence Collection, Rule Engine, Version Profiles, Knowledge Base, Reporting, Storage, and Extension Layer. Of these seven, **two remain dormant today** — Storage and Extension Layer — in exactly the state Version Profiles and Knowledge Base once occupied before Sprints 8 and 9 gave each its first real, minimum-viable content.

**The repository already contains a named classification for exactly this shape of work.** `SPRINT8_ARCHITECTURAL_RESOLUTION.md` §8 ("Cross-Cutting Assessment") explicitly defines **Architectural Activation** — distinguished there from three siblings, "Capability implementation," "Infrastructure expansion," and "Platform evolution" — and applies it directly to Version Profiles: *"This Sprint is the first to realize an already-specified-but-dormant architectural dimension, not to invent a new one or merely extend an already-live one."* Sprint 9 followed the identical pattern for Knowledge Base, in plain language, citing Sprint 8's own precedent directly ("the same shape Sprint 8 proved out for `modiq-versioning`") without re-using the formal term.

Separately, and later, the Sprint 12 Capability Identity procedure was derived and validated exclusively against Collector- and Rule-level historical decisions (Sprint 3, 4, 4 Phase 3C, 7, 8[rule], 9[rule], 11). INV-002 §3 found that none of its three axes, nor the Capability Introduction test, produce a classification for a subsystem-level candidate (Storage) — consistent with the procedure's own derivation set containing no subsystem-level example. **Sprint 12's own historical derivation did not reference, check itself against, or reconcile with Sprint 8's own "Architectural Activation" classification**, even though that classification predates Sprint 12 and was available repository evidence at the time.

**The observation, stated precisely:** this is not an absence of governing concept. Two named, formally-reasoned classifications already exist in the repository — Architectural Activation (Sprint 8) and Capability Identity (Sprint 12) — derived independently, applied to different scopes of architectural work, at different times, and never reconciled with one another. `PROJECT_HANDOFF_v1.1.md` §5's standing rule ("does not proceed to Capability Definition until it has been classified through the Capability Identity procedure") does not itself state whether it supersedes, coexists with, or was ever meant to address Sprint 8's own sibling classification. This is not hypothetical or specific to Storage — Extension Layer sits in the identical dormant state today, and any future subsystem-level activation would face the same, already-present ambiguity.

---

## 2. Evidence

**Fact, direct from the repository:**

- `Architecture.md`'s System Overview diagram and its per-subsystem sections confirm Storage's responsibilities ("assessment persistence, knowledge storage, configuration, historical records, cached resources") and Extension Layer's ("custom Rules, plugins, external integrations, import/export, future AI-assisted capabilities") are specified, but neither has any implementation anywhere in the nine-crate workspace.
- `SPRINT8_ARCHITECTURAL_RESOLUTION.md` §8 defines a four-way "Cross-Cutting Assessment" classification — Capability implementation, Infrastructure expansion, Platform evolution, Architectural Activation — and selects **Architectural Activation** for Version Profiles' first real content, with the reasoning quoted in Section 1. This classification is explicitly marked, in the same table, as "Informational, not a decision."
- `SPRINT9_ARCHITECTURAL_RESOLUTION.md` and `SPRINT9_CAPABILITY_DEFINITION.md` describe Knowledge Base's own first real content in the identical terms — "activation of dormant scaffolding," "the same shape Sprint 8 proved out for `modiq-versioning`" — without re-citing Sprint 8's formal four-way table or the capitalized term itself.
- Neither activation (Sprint 8 or Sprint 9) was run through a three-axis Capability Identity Classification: the procedure did not exist until Sprint 12, and, independently, neither was structurally a Collector/Rule question.
- `SPRINT12_ARCHITECTURAL_RESOLUTION.md` §5–6 derives and validates the Capability Identity procedure's three axes and Introduction test solely against seven Collector/Rule decisions (Sprint 3, 4, 4 Phase 3C, 7, 8[rule], 9[rule], 11 — each concerning a Collector producing `Evidence` or a Rule interpreting it). Sprint 8's own "Architectural Activation" classification of that same Sprint's Version Profile work is not among the decisions checked.
- `SPRINT12.md` names its own anticipated future candidates twice, in nearly identical wording: "a second runtime-log signature, Lua analysis, asset validation, dependency resolution, performance observations." Storage, Extension Layer, and subsystem activation generally are absent from both lists, as is any reference to Architectural Activation.
- `INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md` §3 (as revised) finds Storage classifiable under Architectural Activation, and unclassifiable under Capability Identity's three axes and Introduction test.

**Interpretation, stated as such and kept separate from the above:** it is fact that two named classifications exist, were each reasoned about carefully at their own origin, and have never referenced each other in any committed document. It is interpretation — not fact — that this constitutes a problem requiring resolution, that one taxonomy should subsume the other, or that any specific reconciliation is correct. This document takes no position on any of those; it records only that the two exist, independently, unreconciled.

---

## 3. The Question, Stated Precisely — Not Resolved Here

Given the evidence in Sections 1–2, the open question is narrower than this document originally framed it. It is not "does any governing concept exist for subsystem activation" — one already does, and has since Sprint 8. It is:

**How are Sprint 8's Architectural Activation classification and Sprint 12's Capability Identity procedure intended to coexist within the repository's governance model?**

Relevant, unresolved facts bearing on this question, stated descriptively and without interpretation beyond what each fact itself states:

- Architectural Activation was explicitly marked "Informational, not a decision" at its own origin (Sprint 8) — it was reasoned about, applied once by name, reused once informally (Sprint 9), and never elevated into a Governance Register item, an ADR, or a principle recorded in either current Handoff document.
- Capability Identity was derived through a materially heavier process — adversarial verification against seven named historical decisions, a corrected model, a dedicated reconciliation document, and standing citation across both current Handoffs — and was never checked against, or shown to have been aware of, Architectural Activation's own prior existence.
- Nothing committed to the repository states whether Architectural Activation was superseded by Capability Identity, was always intended to remain its own, narrower-scope classification, or was simply never revisited once Sprint 8 closed.

This document does not answer which of these is true, does not recommend elevating, retiring, or merging either classification, and does not propose a procedure for reconciling them. It records that the question exists and is answerable from repository evidence, should the Chief Architect choose to resolve it.

---

## 4. Explicit Non-Scope

- No recommendation for how Storage, specifically, should be persisted, designed, or implemented is made or implied here.
- No recommendation for how or whether Extension Layer should be activated is made here.
- No recommendation is made as to whether Architectural Activation should be promoted, formalized, retired, or merged into Capability Identity — Section 3 names the reconciliation question without taking a position on its answer.
- No Governance Register item is opened by this document.
- No architectural progression model is designed or proposed here — Section 3 states that a reconciliation question exists, not what the reconciliation should be.
- INV-002's own findings and recommendation are not reopened or altered beyond the corrections the Chief Architect directed to its Sections 3–4 (recorded in INV-002's own amendment note). This document generalizes the corrected finding into its own standing question, independent of Storage's own eventual disposition.

---

## Document Status

**Current Version:** 1.1.0 (amended 2026-07-23 — revised throughout to incorporate `SPRINT8_ARCHITECTURAL_RESOLUTION.md`'s own "Architectural Activation" classification, discovered after this document's original drafting. The original framing — that no governing concept exists for subsystem activation — is superseded; the underlying tension, that two independently-derived taxonomies have never been reconciled, is unchanged and remains the subject of this document.)

**Status:** Complete, as an Observation only, revised. No Evidence-gathering beyond what is cited above was performed; no Investigation was conducted; no Governance Register item was opened; no recommendation to promote, adopt, or reconcile either taxonomy is made; no Capability Definition, Architecture Evaluation, or Sprint Planning follows from this document. Awaiting Chief Architect review.

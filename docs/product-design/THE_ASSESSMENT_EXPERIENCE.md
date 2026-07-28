# The Assessment Experience — Version 1 Product Design

| Property | Value |
|---|---|
| **Initiative** | The Assessment Experience |
| **Phase** | Product Design — Session 4 |
| **Status** | Approved in principle |
| **Builds on** | Workspace Evolution — Version 1 Product Design; The Finding — Version 1 Product Design |
| **Scope** | The complete Version 1 Assessment journey — how the Workspace and Finding product objects connect into one coherent user experience from Intake to departure |
| **Out of scope** | Product philosophy, engine implementation, data model, runtime behavior, UI mockups |

Every decision below is checked against the Product Design Filter: does it make obtaining, understanding, or acting on a trustworthy assessment easier?

---

## 1. Purpose of an Assessment

An Assessment exists to replace guesswork with an evidence-based, explainable conclusion — this is the platform's entire reason for existing, restated here as what a user experiences rather than what the platform does internally.

**The question an Assessment answers**, from the user's side, is broader than any single Finding: *does this mod work, and can I trust that answer?* Every other question in this document is downstream of that one.

**When it begins:** at the moment a user submits an Assessment Subject — not before. Opening an empty workspace is not the start of an Assessment; it's the invitation to start one.

**When it ends:** an Assessment does not have a hard stopping point the way a transaction does, because the workspace remains receptive to additional material indefinitely. What *does* have a clear meaning is **a completed Assessment** — a state the Assessment reaches, potentially more than once, whenever nothing currently known is still Provisional. The Assessment itself, as the user's ongoing relationship with one Assessment Subject, ends only when the user leaves — not when modIQ declares itself finished.

**What constitutes a completed Assessment:** every Finding currently known is Final, an Assessment Overview reflects that Final state across every Mod Health dimension, and Recommendations are present everywhere they're warranted. This is experienced as the Reviewing state described in Workspace Evolution — restated here in terms of what it means to the user, not what it is mechanically.

---

## 2. Assessment Journey

**No Assessment Subject.** The user has a mod and a reason to be uncertain about it, but nothing to look at yet. The workspace offers exactly one thing: the invitation to submit it.

**Submission.** The user provides an Assessment Subject. This is the single deliberate act that starts everything that follows.

**Assessing begins.** The workspace begins showing something immediately — not a blank wait. The user's understanding moves from *nothing* to *something is being discovered*, without yet knowing anything final.

**Provisional discovery.** Findings appear, evidence accumulates, all of it honestly labeled as still forming. The user can already begin reading, with a clear signal not to treat any of it as settled.

**Reviewing begins — the Assessment Overview.** Assessing completes; the Overview appears with Final Mod Health across its dimensions and Finding counts by severity. This is the first point where the user has a trustworthy, whole-picture answer to "does it work."

**Exploring Findings.** The user descends from the Overview into individual Findings, grouped by Category and ordered by Severity, each answering its own narrow question about one aspect of the mod.

**Encountering Recommendations.** Within Final Findings that warrant one, the user finds a Recommendation — plain or a Repair Recipe — exactly where the Finding that justifies it lives.

**Supplementation (optional, and may recur).** The user submits more material. Assessing resumes on top of the existing Reviewing content; only the affected Findings return to Provisional; everything else stays visible and Final throughout.

**Departure.** The user leaves once they feel they understand the Assessment and know what, if anything, to do next. There is no forced "finish" step — leaving is a choice the user makes when they're ready, not an action the product asks them to take.

---

## 3. User Questions

| Stage | Primary Question |
|---|---|
| Intake | "What do I need to provide?" |
| Assessing (Provisional) | "What is modIQ finding so far, and can I trust it yet?" |
| Reviewing / Overview | "Does this mod work?" |
| Exploring Findings | "What, specifically, is right or wrong — one aspect at a time?" |
| Recommendations | "What should I do about it?" |
| Evidence (whenever consulted) | "How do we know that?" |
| Supplementation | "Did modIQ actually take my new material into account?" |
| Departure | "Do I know enough to stop looking, and do I know what's next?" |

Each stage fully answers its question before the next stage's question becomes relevant to the user — a user is never asked "what should I do" before they've been told "does it work," and never asked to verify before they've been told what was found.

---

## 4. Evolution of Understanding

**Initially:** the user knows only that they have a mod and a reason to be uncertain. modIQ, from the user's point of view, knows nothing yet either — there is no asymmetry at the start.

**During Assessing:** understanding becomes Provisional — genuinely more than nothing, but explicitly not yet trustworthy as a final word. The product is careful never to let "something is showing up" be mistaken for "this is done."

**Once Findings become Final:** understanding upgrades from *here's what's forming* to *here's what's true*. This is also the exact point at which Recommendations become available — understanding and actionability arrive together, not staggered oddly apart from each other.

**Recommendations** convert "I know what's wrong" into "I know what to do." This is a distinct increment in understanding, not an afterthought bolted onto a Finding — a user with only Findings understands the mod's condition; a user with Recommendations also understands their own next step.

**Evidence** reinforces confidence without interrupting understanding, because it is never forced into the reading path. A user's understanding can complete fully — Title, Summary, Recommendation — without ever opening Evidence. Evidence only adds certainty on top of an already-complete understanding, for the user who wants it.

---

## 5. Trust Throughout the Assessment

- **Visibility of progress:** Provisional Findings appear the moment they exist, so trust begins with "I can see it working," never with "I am told to wait."
- **Provisional versus Final:** labeled everywhere, always, so trust is calibrated per-claim rather than assumed globally — the user always knows precisely which part of what's in front of them is settled.
- **Traceability:** every Recommendation sits inside the Finding that justifies it; every Finding's Evidence is one expansion away. Trust is built by proximity and directness, not by assertion.
- **Transparency:** nothing collapses into an opaque score — the actual Mod Health dimensions are always visible, never a single verdict the user must simply accept.
- **Avoiding false certainty:** no Confidence value appears on a Provisional Finding, and Confidence is always scoped to the specific Recommendation it measures — never inflated into a statement about the whole Assessment.
- **Avoiding unnecessary reassurance:** the product never states in prose that it can be trusted. It earns trust by never claiming certainty it doesn't have and never hiding how a conclusion was reached.

**Decision:** trust is never asserted in this experience — it is the visible consequence of decisions already made in Workspace Evolution and Finding Experience (labeling, layering, traceability). This section names that effect; it introduces nothing new.

---

## 6. Completion

- **What the user has before leaving:** an Overview reflecting Final Mod Health across every dimension, every currently known Finding (Final, grouped, explorable), and every Recommendation those Findings warrant.
- **How they know they've reached the end:** by an absence, not an announcement — once nothing on screen still carries a Provisional marker, there is nothing left in progress to wait for.
- **What confidence they should leave with:** confidence proportional to what was actually established — high where Findings are Final and well-evidenced, appropriately qualified wherever a Recommendation's Confidence is lower. Never blanket reassurance, never blanket doubt.
- **What actions they should be prepared to take:** exactly what the Recommendations already told them — completion introduces no new call to action; it is simply the point at which everything the Assessment has to say has already been said.

**Decision:** completion is a feeling the user arrives at — nothing left unresolved — not a mechanic modIQ performs on them. This is consistent with Workspace Evolution's removal of Idle as a state and its decision that there is no explicit "conclude" action.

---

## 7. Multiple Assessment Cycles

- Submitting more material continues the same Assessment; it does not restart it — Assessing resumes on top of Reviewing, against the same Assessment Subject, exactly as Workspace Evolution established.
- The user experiences this as an extension of understanding they already have, not a reset of it — everything Final stays visible and unchanged throughout, while only the specifically affected Findings return to Provisional.
- What was already understood is preserved: the user never has to re-read the whole Assessment to find what's new — only Findings marked "updated" need fresh attention.
- Trust in the continuation is earned the same way trust is earned everywhere else in this experience: the workspace names what was added before showing any effect from it, so "my new material was actually used" is shown, not asserted.

**Decision:** a supplement is experienced as *the Assessment learned something new*, not *the Assessment started over*. This required no new mechanism beyond what Workspace Evolution already established — this section only describes the resulting experience.

---

## 8. Educational Experience

- **Terminology:** every term the user encounters — Finding, Evidence, Recommendation, Confidence, the Mod Health dimensions — is used consistently and matches the Glossary. The Assessment itself is where a user learns these words, through repeated consistent exposure, not through a separate reference they're sent to read.
- **Explanations:** a Finding's Summary always explains why something matters in plain language before any technical detail is reachable — the reasoning is taught before, or instead of, the raw technical fact.
- **Evidence:** for a curious user, this is where real learning happens — seeing the actual detail behind a Finding teaches more than any explanation alone. It is offered, never forced, rewarding curiosity without ever requiring it.
- **Recommendations:** a Repair Recipe, being reproducible and structured, teaches a user how to address a category of problem, not just this one instance — this is where "creators learn best practices," a stated Product Goal, is actually delivered.
- **Progressive understanding:** the layered disclosure — Overview, then Findings, then Evidence — is itself a teaching structure, walking a user from the broadest understanding to the most detailed, in the order people naturally want to learn something unfamiliar.

**Decision:** the Assessment teaches by being transparent about its own reasoning at every layer, not by adding separate instructional content. Learning is a side effect of Transparency and Explainability already designed elsewhere, not a distinct feature added on top.

---

## 9. Emotional Journey

**Uncertainty** (the reason the user came at all) → **Curiosity** (Assessing, Provisional Findings appearing — something is happening, and it's already partly readable) → **Understanding** (Reviewing, the Overview, Final Findings — the picture is whole) → **Confidence** (Recommendations present, Evidence available if wanted) → **Resolve** (the user leaves knowing what, if anything, to do).

If a supplement occurs, this does not restart the arc from Uncertainty. Only what is newly Provisional invites re-evaluation; confidence in the untouched parts of the Assessment doesn't reset. The dip is scoped to what changed, not systemic to the whole Assessment — the same principle as §7, expressed emotionally rather than informationally.

**Decision:** the intended arc is one full traversal from Uncertainty to Resolve per Assessment Subject, with supplements producing small, scoped re-traversals rather than repeating the whole arc — this is what keeps continued engagement from feeling emotionally expensive.

---

## 10. Consistency Review

- **Against Workspace Evolution:** the Assessment Journey's stages map exactly onto the three workspace states (Intake, Assessing, Reviewing) with no new state introduced; Supplementation here is described identically to there (Assessing resuming, not restarting).
- **Against Finding Experience:** "Exploring Findings" and "Recommendations" rely only on the Finding anatomy and hierarchy already defined — nothing here asks a Finding to behave differently than already specified.
- **Against the Stable Product Principles:** all seven are reflected in a specific section above rather than left abstract — Assessment remains the product (§1), information exists to increase understanding (§4), Recommendations/Findings/Evidence chain (§5, §6), modIQ performs work on the user's behalf (§8), progressive understanding (§4, §8), curiosity never creates friction (§5, §8), workspace remains receptive (§7).
- **Against Product Definition:** all three target user groups (Players, Creators, Administrators) are served by this same single journey — this document deliberately does not branch the experience by user type in V1, a simplicity decision worth naming rather than an oversight.
- **A tension resolved explicitly in this session:** "Completion" (§6) could easily have been read as requiring an explicit end-of-assessment event — which would have quietly reintroduced something equivalent to the Idle state Workspace Evolution deliberately removed. This document resolves that by defining completion as a recognized *absence* (no remaining Provisional content) rather than an *event*, keeping it consistent with Workspace Evolution rather than reintroducing an end-state under a new name.

No conflicts remain unresolved. This draft does not modify Vision.md, Principles.md, ProductSpecification.md, Glossary.md, or Architecture.md, and has not been evaluated for architectural feasibility.

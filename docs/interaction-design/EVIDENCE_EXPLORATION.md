# Evidence Exploration — Version 1 Interaction Design

| Property | Value |
|---|---|
| **Initiative** | Interaction Design — Evidence Exploration |
| **Phase** | Interaction Design — Session 4 |
| **Status** | Approved |
| **Governed by** | Workspace Evolution, The Finding, Evidence, The Assessment Experience, The Assessment Report, Assessment Intake & Upload, Assessing & Progressive Discovery, Finding Presentation (frozen constraints) |
| **Scope** | The complete interaction experience of Evidence: choosing to explore, first encounter, progressive understanding, relationship to the Finding and to Recommendations, trust, attention, leaving, and revisiting |
| **Out of scope** | Product concepts, implementation, runtime behavior, engine internals, data model, visual layout, navigation systems |

This document specifies behavior, not concepts. Every product term used below is used exactly as already defined and is not reinterpreted here. Every decision is checked against the Product Design Filter: does it make obtaining, understanding, or acting on a trustworthy assessment easier?

---

## 1. Purpose

Evidence exists to make a Finding checkable — to let a user personally close the gap between "modIQ says this is true" and "I can see why." It is a supporting interaction, not the primary one, because understanding a Finding is already complete without it: the Title, Summary, and Recommendation (when present) already tell a user what happened, why it matters, and what to do. Evidence adds justification on top of understanding that already exists — it does not create understanding where none existed yet.

This is also why a user should understand the Finding before opening its Evidence: Evidence is written to be read in the vocabulary and framing the Finding already established — its Explanation deliberately reuses the Finding's own terms rather than introducing new ones. Opening Evidence first, without that frame, would hand a user raw material with nothing yet to connect it to. The Finding is what makes Evidence legible at all.

---

## 2. Choosing to Explore

A user opens Evidence for any of several reasons — verification, curiosity about how modIQ reached a conclusion, a lingering uncertainty the Summary didn't fully resolve, a wish to learn, or a wish to confirm something before acting on it. This document does not distinguish between these motivations interactionally: whatever the reason, the action is the same, simple expansion already established for reaching Evidence. Motivation lives in the user's own reasoning, not in a set of different interaction paths this design would otherwise have to invent.

**Evidence feels invited, not required, because the invitation already exists as part of the Finding itself** — the always-visible Evidence-availability indicator on every Finding is the invitation. It states that something further can be looked at; it never states that it must be. Choosing to explore is simply accepting an offer that was there from the first glance (Finding Presentation, §2–3).

---

## 3. First Encounter

The moment Evidence opens, a user should immediately understand what kind of thing they're looking at — Label and Source together answer that first, exactly as Evidence's own Information Hierarchy already establishes, before anything else competes for attention.

**Orientation to the Finding is never lost, because nothing was left.** Evidence appears inside the same expanded view the Finding's Recommendation already occupies — the Finding's own Title and Summary remain present, not scrolled away or replaced. There is no moment where a user is looking only at Evidence and no longer at the Finding it belongs to; they are looking at both, together, the entire time.

---

## 4. Progressive Understanding

Deepening understanding through Evidence follows its own already-established shape — Label and Source orient, Explanation frames why it matters, Content confirms it in specific, real detail. Nothing here changes that order; this section describes the felt experience of moving through it: first knowing *what kind* of thing this is, then *why it's relevant*, then, for a user who wants it, *exactly what it says*.

**Evidence is never handed over as raw technical output.** Because Explanation always precedes Content, a user is never given an unframed fragment to interpret on their own — the same "interpretation before raw material" pattern that governs every other layer of this product governs Evidence's own internal structure too. Reasoning is made visible by that same ordering: Label, Explanation, and Content together are a small, complete chain of "here's what this is, here's why it matters, here's the specific proof" — not a data dump a user has to reconstruct meaning from themselves.

---

## 5. Relationship to the Finding

Moving from Finding to Evidence and back is not a trip — there is no separate place to travel to and return from. Evidence unfolds inside the Finding's own expanded space; leaving it is simply the natural end of reading, not a navigation action.

**The Finding remains the primary interaction object throughout.** Evidence temporarily extends it — briefly showing the reasoning underneath a claim already made — and never replaces it as the thing the user is actually engaged with. This is a deliberately different relationship than the one Finding Presentation established for Recommendations: a Recommendation reads as the Finding *continuing to speak*, arriving at its natural conclusion; Evidence reads as the Finding *briefly showing its work*, then returning to where it left off. Both stay subordinate to the Finding, but they are not the same kind of subordination, and this document uses its own precise description rather than reusing the Recommendation's.

---

## 6. Relationship to Recommendations

Evidence strengthens confidence in a Recommendation without ever being a prerequisite to reading or acting on one — this is not a new decision, it is the direct consequence of Recommendation already sitting ahead of Evidence in reading order (The Finding, Finding Presentation). A Recommendation must be fully understandable on its own terms before a user ever considers opening Evidence.

**Evidence strengthens confidence; it does not unlock meaning.** A Recommendation is never written in a way that only makes sense after Evidence has been read — if that were ever true, Evidence would have become a gate rather than a support, contradicting its own purpose (§1). What Evidence legitimately adds here is the ability to personally check the basis for a Recommendation's Confidence — especially one asking for real effort, such as a Repair Recipe — before investing that effort. Confidence itself is already visible and legible without opening anything further; Evidence is where a user goes to audit it, not where they go to first find out what it means.

---

## 7. Trust

Evidence strengthens trust the same way every prior document in this series has already established trust is built — never through authority, always through something observable:

- **Transparency:** what's shown is the real, specific detail — never a paraphrase softened for presentation.
- **Explainability:** Label and Explanation frame every piece of Content before it's read, so nothing arrives uninterpreted.
- **Consistency:** the same anatomy, every time, regardless of what produced it — a user who has read one piece of Evidence already knows how to read all of it.
- **Traceability:** Evidence is the point where the justification chain (Recommendation → Finding → Evidence) actually terminates in something a user can personally verify, closing the chain themselves rather than taking modIQ's word that it closes.

**Decision:** nothing in this design ever tells a user that Evidence is trustworthy. It is shown, plainly and consistently, and trust is whatever the user concludes from looking at it themselves.

---

## 8. Attention & Cognitive Load

**What deserves attention:** Label and Explanation — the two things every piece of Evidence guarantees are immediately legible, regardless of how much Content sits behind them.

**What remains secondary:** raw Content, particularly when long, staying behind its own further reveal rather than demanding to be read in full by default.

**How overload is avoided:** when a Finding carries more than one piece of Evidence, each stays independently scannable through its own Label/Source/Explanation — a user can read just those across every item and already have a coherent picture, without opening a single Content field.

**Decision — reward curiosity, never punish it.** A user who goes further into Content should find it specific and worth the extra step; a user who stops at the Explanation should never feel they missed something essential by not going further. Evidence's job is to make going deeper worthwhile for those who choose it, not to make stopping early feel like an incomplete choice.

---

## 9. Leaving Evidence

There is no designed "exit" action, because there was no designed "entry" in the destination sense (§5) — leaving Evidence is simply the natural conclusion of reading it, with the Finding's own Title, Summary, and Recommendation exactly where they already were.

**The transition should feel like "I understand this Finding better now," not "I visited another part of the application."** Nothing moved, nothing needs to be closed or re-found, and no scroll position or context is lost — because nothing was ever displaced in the first place. This is the behavioral guarantee underneath the emotional arc Evidence's own Product Design already names (Curiosity → Verification → Confidence → Understanding): the arc completes cleanly because the interaction never introduced a seam for it to snag on.

---

## 10. Revisiting Evidence

**Orientation and familiarity:** returning to a previously opened piece of Evidence is immediate and unsurprising — the same Label, Source, and Explanation are exactly where they were, recognized the same way a revisited Finding is recognized (Finding Presentation, §9).

**Updated Evidence:** Evidence carries no independent status or currency marker of its own — it never has, by design (Evidence, Product Design, §2: "never present... its own severity or status"). Its currency is entirely inherited from the Finding it belongs to: if a Finding is marked "updated," anything reached through it — including its Evidence — reflects that Finding's current state. This document does not introduce a separate "updated" indicator for Evidence itself; doing so would create a second, competing notion of currency where the Finding's own marker (already established) is sufficient.

---

## 11. Accessibility & Interaction Principles

Keyboard interaction, discoverability, reversibility, interruption tolerance, and user control (Assessment Intake & Upload); no forced pacing and stable attention position (Assessing & Progressive Discovery); independent expansion and severity-bounded presentation (Finding Presentation) all continue to apply and are not repeated here. Two principles are genuinely new to interacting with Evidence specifically:

- **Non-disruptive expansion.** Opening Evidence, or revealing its further Content, never pushes the Finding's own always-visible content out of view or requires the user to re-find it — it adds within the existing space, it does not displace what was already there.
- **No compounding exploration cost.** Reading several Evidence items in sequence — within one Finding, or across several — costs the same regardless of order or how many have already been read. Curiosity is never taxed more heavily the further it's indulged.

---

## 12. Interaction Consistency Review

- **Against Evidence (Product Design):** anatomy, hierarchy, disclosure rules, and the Multiple Evidence Items guidance are used exactly as defined — no new field, no new status, no new grouping introduced.
- **Against The Finding:** the single-expansion structure (Recommendation detail, then Evidence) is preserved exactly; Evidence's subordinate position relative to the Finding is restated, not altered.
- **Against Finding Presentation:** this document deliberately uses its own metaphor for Evidence's relationship to the Finding ("briefly showing its work") rather than reusing Recommendation's ("continuing to speak") — named explicitly as an intentional distinction between two different subordinate relationships, not an inconsistency between them.
- **Against Assessing & Progressive Discovery:** nothing here contradicts Evidence accumulating silently behind Provisional Findings during Assessing — this document concerns only the experience of a user choosing to open Evidence, at any workspace state, and does not revisit how or when it accumulates.
- **Against the Glossary and Principles:** Evidence, Finding, Recommendation, and Confidence are all used exactly as constitutionally defined; Transparency and Explainability are applied, not redefined.
- **No new abstractions introduced:** no Evidence-level "updated" marker independent of its Finding's (§10) — deliberately rejected in favor of inherited currency; no motivation-specific interaction variants (§2); no distinct "exit" mechanism for leaving Evidence (§9) — each considered and explicitly declined rather than silently omitted.

No terminology drifted. No behavioral contradiction remains. No Product Design artifact was changed.

---

## 13. Chief Architect Reflection

**Strengths:** this document found a genuinely new relationship to describe — Evidence's subordination to the Finding is real but different in character from a Recommendation's, and giving it its own precise language ("showing its work" rather than "continuing to speak") avoids flattening two distinct experiences into one borrowed metaphor. The "invited, not required" requirement was satisfied without inventing anything: the existing availability indicator already is the invitation, once named as one.

**Remaining ambiguity:** §11's "no compounding exploration cost" is stated as a principle rather than fully worked through — exactly how a user experiences skimming several long Content fields in quick succession is sketched, not resolved, and is left for evidence-gathering during actual use rather than invented here. §10's inheritance-of-currency argument is a clean inference from already-established rules, but it has not been tested against a case where a Finding updates in a way that removes a piece of Evidence entirely rather than changing it — that case is named but not designed.

**Preparation for Session 5:** this document kept "revisiting" scoped to a single Evidence item's own familiarity (§10), deliberately not extending into what it feels like to re-orient across an entire Assessment Report after time away — that remains Session 5's (Assessment Report Experience) task, building on both this document's per-item treatment and The Assessment Report's own already-designed return experience (Recognition → Orientation → Understanding).

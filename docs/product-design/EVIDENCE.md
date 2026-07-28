# Evidence — Version 1 Product Design

| Property | Value |
|---|---|
| **Initiative** | The Evidence Experience |
| **Phase** | Product Design — Session 5 |
| **Status** | Approved in principle |
| **Builds on** | Workspace Evolution, The Finding, The Assessment Experience |
| **Scope** | The complete Version 1 Evidence experience — anatomy, disclosure, and its relationship to Findings and Recommendations |
| **Out of scope** | Product philosophy, rule evaluation, engine internals, storage mechanisms, UI layouts |

Every decision below is checked against the Product Design Filter: does it make obtaining, understanding, or acting on a trustworthy assessment easier?

---

## 1. Purpose of Evidence

Evidence exists to make every Finding checkable. It is the objective record a Finding is drawn from, existing so that no conclusion in the Assessment ever has to be taken purely on modIQ's word. This is the direct product expression of the Glossary's own line — "every Assessment conclusion should be traceable back to evidence" — and of Transparency as a core design philosophy.

**The question Evidence answers** is narrower than a Finding's: not "what is true" but *"how do we know that?"* Evidence never introduces new meaning of its own — it substantiates meaning the Finding has already stated.

**Why Evidence is separate from Findings:** a Finding is an interpretation; Evidence is what was actually observed, unfiltered by interpretation. Keeping them apart protects both. The Finding stays short and readable because it never has to carry raw technical detail. Evidence stays objective and complete because it never has to be simplified to fit inside a readable sentence. Merging them would force a choice between an unreadable Finding and an incomplete record — separation avoids that trade entirely.

**Why Evidence exists even when a user never opens it:** its value isn't contingent on being read. Its existence, and the user's awareness that it exists — via the always-visible Evidence-availability indicator already established on every Finding — is itself what makes a Finding trustworthy. A Finding backed by inspectable Evidence is different in kind from a bare assertion, even to a user who takes it on faith and never checks. Transparency's function isn't that everyone verifies everything; it's that everything is *capable* of being verified by anyone who wants to.

---

## 2. Anatomy of Evidence

**Always present**, regardless of what produced the Evidence:

- **Label** — a short, plain description of what kind of Evidence this is (e.g., "structure inspection," "log entry," "dependency reference") — enough to know what's about to be read without technical jargon.
- **Source** — where it came from, described in terms the user already understands from the Assessment Subject itself (which file, which part of the mod) — never an internal identifier.
- **Explanation** — a short, plain-language line connecting the Evidence back to why it matters for the Finding it belongs to.
- **Content** — the actual objective material (the specific structural detail, log line, or reference in question).
- **Legible ownership** — a user looking at just this one Evidence item should still be able to tell which Finding's claim it supports, without leaving the item to check.

**Never present inside a single piece of Evidence:**

- Its own severity or status — severity belongs to the Finding, never to the Evidence supporting it.
- A Recommendation or Repair Recipe — those remain exclusive to the Finding, even when a piece of Evidence very directly suggests a particular fix.
- A Confidence value — Confidence belongs to the Recommendation the whole chain supports, never to an individual Evidence item.
- Cross-references to other Findings' Evidence — Evidence stays scoped to the one Finding it belongs to, the same rule already governing Findings themselves.

**Decision:** every Evidence item has the same shape regardless of whether it originated from structural inspection, script analysis, a log, or a dependency check — a user learns to read Evidence once and can read all of it.

---

## 3. Information Hierarchy

**Label → Source → Explanation → Content.**

This maps directly onto the two questions a user actually asks: "what am I looking at?" is answered by Label and Source together — orientation before anything else. "How does this support the Finding?" is answered by Explanation, then Content.

Content comes last because it's the rawest, most detailed material — the same interpretation-before-raw-material pattern already used for Findings (Title/Summary before Evidence) recurs one layer deeper here, inside Evidence's own structure. A user is never handed a technical fragment before being told why it matters.

---

## 4. Progressive Disclosure

**Always visible once Evidence is opened** (having already chosen to expand a Finding to reach it): Label, Source, Explanation — enough to know what it is and why it matters before reading anything raw.

**Requires a further, explicit reveal:** the raw Content itself, but only when substantial enough that showing it by default would dominate the space (a long excerpt, an extensive structural block). Brief Content displays inline without a second step — the goal is avoiding overload, not adding friction where none is needed.

**Decision:** this is a second layer of progressive disclosure nested inside the first (Finding → Evidence). A user satisfied by the Explanation alone never has to scroll through raw material they didn't ask to see; a user who wants it is never more than one further step away.

---

## 5. Multiple Evidence Items

- **Ordering:** by relevance to the Finding's own claim — the most directly supporting item first, not chronological and not grouped by the kind of analysis that produced it.
- **Grouping:** none beyond their shared nesting under the one Finding they belong to. Introducing sub-categories of Evidence within a Finding would add organizational weight V1 doesn't need.
- **Avoiding overload:** each item is independently scannable via its Label/Source/Explanation — reading just the Explanations across several Evidence items already tells a coherent story, with full Content available per-item only for whichever one a user wants to personally verify.
- **Avoiding repetition:** each Evidence item's Explanation describes its own specific contribution, not a restatement of the Finding's Summary — the same point should not appear three times in three slightly different phrasings.

---

## 6. Relationship to Findings

- A user understands why an Evidence item belongs to its Finding because it is physically nested inside that Finding's own expansion, and because its Explanation always uses the same terms the Finding's Summary already established — no new vocabulary is introduced at this layer.
- Evidence reinforces without competing because it never appears at the same reading depth as the Finding's Title, Summary, or Recommendation — it is always one layer deeper, informationally subordinate, never an equally-weighted alternative account of what happened.
- Users move between a Finding and its Evidence non-destructively: opening Evidence doesn't hide or replace the Finding's own content — both remain visible together, so a user can move back and forth between the claim and the proof without losing their place. This is the same continuity guarantee already established for Recommendations (no separate page) — there is no separate Evidence page either.

---

## 7. Relationship to Recommendations

The justification chain — Evidence → Finding → Recommendation — is preserved exactly as already established: Evidence never attaches to a Recommendation directly, only ever justifying one indirectly, through the Finding it supports.

Users are not required to consume the chain in that order. A user may read a Recommendation first — it sits earlier in reading order than Evidence — and only afterward decide to check Evidence, or never check it at all. The chain's logical order and the user's actual reading order remain two different things, exactly as reconciled in the Finding design; this session relies on that reconciliation rather than reopening it.

What Evidence adds experientially: it is the one place a skeptical user can go to confirm a Recommendation isn't just plausible-sounding but actually grounded — closing the loop for the minority who want that closure, without being required reading for the majority who don't.

---

## 8. Educational Experience

- **Terminology:** Evidence is where technical vocabulary a user might not already know is introduced — but always with its Explanation alongside, never handed over without a plain-language reason to care.
- **Explanation:** because every Evidence item states why it matters before showing what it is, repeated exposure across many Findings teaches the user to recognize patterns without the product ever presenting itself as a tutorial.
- **Transparency:** showing real, specific detail — not a paraphrase — is itself educational. A user who reads several Evidence items over time learns to recognize what that kind of detail actually looks like, which a summary alone could never teach.
- **Confidence:** seeing that a Recommendation's Confidence is grounded in visible, specific Evidence — rather than asserted — teaches the user what "evidence quality" actually means in practice, reinforcing the Glossary's own distinction that Confidence reflects evidence quality, not correctness.

---

## 9. Emotional Experience

**Curiosity** (the user chooses to open Evidence — nothing requires it) → **Verification** (reading Label, Source, Explanation, and if desired, the raw Content) → **Confidence** (the Content agrees with what the Finding already said) → **Understanding** (the user's grasp of the mod is now personally verified, deepening rather than merely adding to what they already understood).

Unlike the Assessment and Finding journeys, which unfold whether or not a user actively engages with every layer, this arc only begins by the user's own choice — not every user will complete it, and that is the intended experience, not a gap in it.

---

## 10. Consistency Review

- **Against Workspace Evolution:** Evidence is still reached only by expanding a Finding, still carries no Confidence of its own, still has no independent status — all consistent.
- **Against Finding Experience:** matches the single-expansion structure (Recommendation detail, then Evidence) exactly, and extends the same "interpretation before raw material" pattern one level deeper into Evidence's own internal hierarchy (Label/Source/Explanation before Content) — this is a recurring design pattern, not a coincidence, and is named as such here for the first time.
- **Against Assessment Experience:** operationalizes that document's line "Evidence reinforces confidence without interrupting understanding" into Evidence's actual anatomy and disclosure rules.
- **Against Stable Product Principles:** "Recommendations are supported by Findings, and Findings by Evidence" is the chain this entire document exists to make experientially real; "curiosity should never create friction" is honored by every disclosure rule in §3–4; "the workspace remains receptive to additional material" is untouched — Evidence produced by a later supplement follows this same anatomy.
- **Against Product Definition:** Evidence is the concrete mechanism by which "replace guesswork with evidence" and "every Assessment conclusion should be traceable back to evidence" actually reach the user, rather than remaining an internal guarantee invisible to them.
- **A pattern worth naming explicitly, not a new decision:** across all three prior sessions and this one, the product repeats one shape at every layer — Assessment, Finding, and now Evidence — a short interpreted headline, available immediately, with progressively more raw material available strictly on demand, never required. Evidence is the last and deepest application of that shape, not a new one.

No conflicts identified. This draft does not modify Vision.md, Principles.md, ProductSpecification.md, Glossary.md, or Architecture.md, and has not been evaluated for architectural feasibility.

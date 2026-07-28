# Finding Presentation — Version 1 Interaction Design

| Property | Value |
|---|---|
| **Initiative** | Interaction Design — Finding Presentation |
| **Phase** | Interaction Design — Session 3 |
| **Status** | Approved |
| **Governed by** | Workspace Evolution, The Finding, The Assessment Experience, Evidence, The Assessment Report, Assessment Intake & Upload, Assessing & Progressive Discovery (frozen constraints) |
| **Scope** | The complete interaction experience of a Finding: first encounter, at-a-glance understanding, progressive exploration, relationship to Evidence and Recommendations, attention, trust, revisiting, and coexistence with other Findings |
| **Out of scope** | Product concepts, implementation, runtime behavior, engine internals, data model, visual layout, navigation systems |

This document specifies behavior, not concepts. Every product term used below is used exactly as already defined and is not reinterpreted here. Every decision is checked against the Product Design Filter: does it make obtaining, understanding, or acting on a trustworthy assessment easier?

---

## 1. Purpose

The Finding is the primary interaction object of modIQ because it is the smallest complete unit of meaning the product produces — and, unlike the Assessment Overview, it is where every subsequent thing a user might want lives. Evidence exists only attached to a Finding. A Recommendation exists only attached to a Finding. The Assessment Report is Findings, organized. The Overview itself exists to point a user toward Findings, not to substitute for them.

This is why users spend more time with Findings than with the Assessment as a whole: the Overview is consulted, not inhabited — it answers one broad question in a single glance and its job is largely done. A Finding is where reading, evaluating, and deciding actually happen, and a user with several Findings to consider repeats that engagement once per Finding. The Overview is the doorway; Findings are the rooms a user actually spends time in.

---

## 2. First Encounter

A user's first encounter with a Finding happens in either of two contexts already designed elsewhere — deliberately descending from the Overview during Reviewing, or watching one stream in during Assessing (Assessing & Progressive Discovery). This document does not redesign either arrival mechanic; it describes what registers the moment a user's attention actually lands on the Finding itself, regardless of how they arrived.

**Visibility:** a Finding is never hidden behind an interaction — it is present the moment its position in the list is in view, exactly as the always-visible layer of its anatomy already guarantees.

**Recognition:** the combination of Title, Severity, and Category — all visible at once, without any action taken — is enough for a user to register "something specific has been discovered" instantly. Nothing further needs to be read for that registration to happen.

**Orientation:** because Findings are always grouped by Category and ordered by Severity, a user already knows, before reading a single word, roughly what kind of thing this Finding concerns and how it compares to its neighbors — orientation is a byproduct of position, not something the user has to work out.

**Decision:** the first encounter is satisfied entirely by what's already always-visible. A user should be able to stop reading after the Title and Severity and still walk away knowing something real was found — everything past that point is depth the Finding offers, not depth it demands.

---

## 3. Understanding at a Glance

Without opening or exploring anything, a Finding already answers:

- **What is wrong?** — the Title, reinforced by the Summary.
- **How serious is it?** — Severity.
- **Where does it belong?** — Category.
- **Is it Provisional or Final?** — the status indicator, always present.
- **Has this changed since I last looked?** — the Updated indication, when applicable.
- **Is there something to do about it?** — a Recommendation headline, when the Finding is Final and one exists — answering *whether* action exists without yet revealing what it is.

None of this is new design — it is a direct restatement of which question each field in the Finding's already-established always-visible layer answers. Naming that mapping explicitly is this section's only job: the glance-level understanding a Finding provides is not a separate, additional behavior to design — it *is* the always-visible anatomy, read as a whole rather than as a list of fields.

---

## 4. Progressive Exploration

**Recognition → Summary → Understanding → Recommendation (when applicable) → Evidence.**

This order deliberately follows The Finding's already-established Information Hierarchy — Recommendation before Evidence — rather than an Evidence-before-Recommendation sequence. That ordering question was already raised and resolved once, in Product Design (The Finding, §8, Narrative Flow): the justification chain (Evidence → Finding → Recommendation) and the reading order (Title → Summary → Recommendation → Evidence-on-demand) are two different things, and reading order deliberately puts the answer ("what should I do") ahead of the proof, so that verification stays optional rather than a toll every Finding charges before revealing what to do about it. This document preserves that resolution rather than reopening it.

**Why it feels inevitable rather than forced:** Recognition, Summary, and Understanding require no action at all — they are all always-visible, so a user arrives at "understanding" simply by reading, with no click or decision along the way. Only the next two steps require a deliberate choice: expanding to see the full Recommendation (when one exists) and, separately, choosing to inspect Evidence. The progression feels inevitable exactly where it costs nothing, and becomes an active choice exactly where going deeper genuinely means something different (acting, or verifying) — it never asks for a decision where none is warranted.

**The Finding as gateway:** nothing reachable from a Finding is reachable any other way. There is no separate path to a Recommendation, no separate path to Evidence, and no separate path to a different Finding's content. The Finding is not one of several ways to reach deeper understanding — it is the only way.

---

## 5. Relationship to Evidence

**When Evidence becomes relevant:** when a user wants to verify a claim before trusting or acting on it — most naturally for higher-Severity Findings, or when an attached Recommendation asks for real effort.

**Why users choose to inspect it:** to confirm the Finding is warranted, or out of plain curiosity about how modIQ reached this conclusion. Either reason is legitimate; neither is required.

**When they don't need to:** whenever the Title, Summary, and Recommendation headline already tell them what they need — which is most of the time, for most Findings. Evidence sits available, not demanding.

**Decision:** the default presented form of a Finding is exactly its always-visible layer — clean, free of technical density. Evidence's availability doesn't compete with that default for the user's attention; it waits, unopened, exactly as designed in Evidence's own product definition, never forcing itself into the reading experience of a user who hasn't asked for it.

---

## 6. Relationship to Recommendations

A Recommendation should read as the Finding continuing to talk, not as a handoff to something else. It lives inside the same Finding, appears only once that Finding is Final, and sits in the same voice and context as the Title and Summary that preceded it — a user reading a Recommendation should feel like they're still reading the same thought, arriving at its natural conclusion, not being redirected to a different kind of object.

This is consistent with, not a change to, the established model: Recommendations remain exclusive to their Finding, appear only when Final and warranted, and a Repair Recipe remains simply the form a Recommendation takes when it's reproducible — nothing here revisits any of that.

---

## 7. Attention & Focus

**What naturally attracts attention across many Findings:** their position in the fixed Category/Severity order. Because that order already exists and isn't something the user constructs, a Finding doesn't need to compete for notice — where it sits already communicates its relative standing among its neighbors.

**What remains secondary:** everything behind the single expansion — full Recommendation detail, Evidence. These stay backgrounded until sought, never surfacing unprompted.

**How multiple Findings coexist without competing:** because each Finding is independently complete and occupies a stable, designed position, none of them needs to escalate its own presentation to be seen. Calm coexistence is a consequence of fixed ordering, not an additional behavior layered on top of it.

**Decision — urgency bounded by Severity:** a Finding's attentional weight should never exceed what its own declared Severity warrants. An Error may reasonably carry more weight than an Informational Finding — that is Severity doing its job — but no Finding should be phrased or presented to feel more alarming than its own Severity states. Dramatizing beyond declared Severity would be its own kind of false certainty, understating or overstating what the Assessment actually concluded.

---

## 8. Trust

Trust in an individual Finding continues the same behavioral model established across every prior session, applied here at the level of a single object rather than the process as a whole:

- **Transparency:** every claim a Finding makes is one step away from its Evidence, always — never asserted without a path to verification.
- **Evidence:** its mere availability, whether or not opened, is itself trust-bearing, exactly as already established.
- **Consistency:** every Finding has the same shape regardless of what produced it. A user who has read one Finding already knows how to read all of them — predictability itself is a trust mechanism here, not just a convenience.
- **Understandable structure:** plain language throughout, no exposed Rule identifiers or internal naming — nothing the user is asked to trust without being able to parse it themselves.
- **Never reassurance:** no Finding states or implies its own reliability in prose. This is the same discipline Assessing & Progressive Discovery already established for the process as a whole, continued here for each individual object that process produces.

---

## 9. Revisiting Findings

**Orientation and recognition:** returning to a previously seen Finding should be immediate — the same Title, Severity, and Category are exactly where they were, so recognition happens the same way it did the first time, without rediscovery.

**Continuity:** whether a Finding's expanded/collapsed state is remembered across a visit is not decided by this document — that edges toward a state-persistence question this document is not positioned to resolve. What matters behaviorally, and is decided here, is that reopening a Finding never costs more than it did the first time, and its content remains exactly what it was unless it was legitimately updated.

**Changed state:** if a Finding was affected by a supplement, its Updated indication is exactly as visible on return as it would have been in the same session — this document does not introduce a comparison or "what changed" view; the existing marker is the entire mechanism, reused rather than extended.

**Decision:** familiarity on return is structural, not a special feature — it follows directly from every Finding sharing the same shape (§8), not from anything unique to the act of revisiting.

---

## 10. Multiple Findings

**Prioritization:** already handled by the fixed Category/Severity order — a user scanning downward naturally encounters what matters most first, without having to sort or decide anything themselves.

**Cognitive load:** because every Finding shares the same shape, learning to read one Finding is learning to read all of them — a user can skim by pattern rather than re-learning structure at each one.

**Grouping:** Category grouping gives the list a cognitive anchor that mirrors the Overview's own dimensions — a Category the Overview flagged as weaker leads directly to the group of Findings that explains why, with no separate translation required between the two.

**Maintaining context:** because each Finding is independently understandable and never references another, a user does not need to hold a growing mental stack of everything read so far to make sense of the next Finding. This independence — already decided in Product Design — is precisely what keeps a long list of Findings from becoming a memory burden.

This document introduces no filtering, search, sort, or jump-to-Category mechanism. Everything above describes how existing structure already supports navigating many Findings mentally, not a new system for doing so.

---

## 11. Accessibility & Interaction Principles

Keyboard interaction, discoverability, reversibility, interruption tolerance, and user control (Assessment Intake & Upload), and no forced pacing and stable attention position (Assessing & Progressive Discovery) all continue to apply and are not repeated here. Two principles are genuinely new to interacting with an individual Finding:

- **Independent expansion.** Expanding one Finding to reveal its Recommendation and Evidence has no effect on any other Finding's own state — exploring one never disturbs a user's place in any other.
- **Severity-bounded presentation.** A Finding's attentional weight is bounded by its own declared Severity and never exceeds it (§7) — stated here as a standing principle, not only an observation about calm coexistence.

---

## 12. Interaction Consistency Review

- **Against The Finding:** anatomy, the always-visible/expansion split, and the Information Hierarchy are used exactly as defined. One deliberate, named divergence from this session's own suggested example ordering: §4 follows Recommendation-before-Evidence, not Evidence-before-Recommendation, preserving the ordering question The Finding's own Narrative Flow section already resolved rather than reopening it under a new prompt's phrasing.
- **Against Workspace Evolution:** Category/Severity grouping, single-expansion behavior, and the Provisional/Final model are all reused, not modified.
- **Against Evidence:** Evidence's role as confirmatory rather than introductory, and its collapsed-by-default presentation, are restated, not changed.
- **Against The Assessment Report:** Findings inside a Report behave identically to Findings anywhere else described in this document — no Report-specific Finding behavior was introduced.
- **Against Assessment Intake & Upload and Assessing & Progressive Discovery:** no accessibility or interaction principle already established was repeated; only the two genuinely new ones in §11 were added. A Finding encountered via streaming (Session 2) and one encountered by deliberate scanning during Reviewing are confirmed to look and behave identically once visible — first encounter (§2) does not vary by arrival path.
- **Against the Glossary and Principles:** Finding, Evidence, Recommendation, Severity, and Category are all used exactly as constitutionally defined; no term drifted.
- **No new abstractions introduced:** no diff or comparison view for updated Findings, no persistence mechanism for expansion state, no filtering/search/navigation system — each of these was considered and explicitly declined rather than silently avoided.

No concept changed. No terminology drifted. No behavioral contradiction remains unresolved.

---

## 13. Chief Architect Reflection

**Strengths:** this document gives the Finding a genuine behavioral identity as the product's hub — every section ties back to the same claim (everything reachable from a Finding is reachable only through it), rather than treating that as a slogan stated once and left unexamined. The one real tension it had to navigate — this session's own suggested Evidence-before-Recommendation ordering against The Finding's already-resolved Recommendation-before-Evidence hierarchy — was caught and resolved in favor of the earlier, already-litigated decision, rather than quietly drifting to match a new prompt's phrasing.

**Remaining ambiguity:** §9 deliberately leaves open whether a Finding's expansion state persists across a return visit — that's a genuine open question, correctly left undecided here because resolving it would mean making a state-behavior decision this document isn't positioned to make. §10's claim that Category grouping and Finding independence keep cognitive load manageable is asserted at the level of felt experience only; this document does not, and should not yet, claim to know how that holds up against a very large number of Findings in practice.

**Reserved for future sessions:** Session 4 (Evidence Exploration) still owes the full experience of *being inside* an opened Evidence item — this document only established when and why a user goes there. Session 5 (Assessment Report Experience) will need to address navigating many Findings within a complete Report more thoroughly than §10's deliberately shallow treatment, which stopped short of anything resembling a navigation system by this session's own explicit instruction.

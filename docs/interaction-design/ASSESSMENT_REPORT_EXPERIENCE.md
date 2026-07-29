# Assessment Report Experience — Version 1 Interaction Design

| Property | Value |
|---|---|
| **Initiative** | Interaction Design — Assessment Report Experience |
| **Phase** | Interaction Design — Session 5 |
| **Status** | Approved |
| **Governed by** | Workspace Evolution, The Assessment Report, The Assessment Experience, The Finding, Evidence, Assessment Intake & Upload, Assessing & Progressive Discovery, Finding Presentation, Evidence Exploration (frozen constraints) |
| **Scope** | The interaction experience of a completed Assessment Report as a coherent whole: first encounter, recognition, orientation across time, experiencing the whole, relationships between parts, supporting decisions, returning, trust at the Assessment level, and completion |
| **Out of scope** | Product concepts, implementation, runtime behavior, engine internals, data model, visual layout, navigation systems |

This document specifies behavior, not concepts. Every product term used below is used exactly as already defined and is not reinterpreted here. Every decision is checked against the Product Design Filter: does it make obtaining, understanding, or acting on a trustworthy assessment easier?

---

## 1. Purpose

The Assessment Report exists so a user can possess understanding, not merely have witnessed its formation. During Assessing, the user's role is a witness — watching understanding emerge, honestly labeled as still forming. Once complete, the user's role changes: they now hold something settled, and the interaction design's job shifts from supporting observation to supporting reliance.

This is the essential difference from the live Assessment experience: Assessing is about watching something happen; the Report is about having something to use. Nothing about what's shown necessarily changes at that boundary — the same Overview, the same Findings — but everything about how a user reads it changes, because every Provisional label is now gone. Completion changes the user's relationship to the Assessment because it changes what kind of trust is being asked of them: during Assessing, they trusted a process in motion; now, they can trust specific, settled conclusions at face value, without mentally hedging against the possibility that something might still resolve differently.

---

## 2. First Encounter

Opening a completed Report should feel like arriving at the same place, not a new one — there is no separate "report screen" to navigate to; Reviewing is a state of the same single workspace already used throughout Intake and Assessing. What should feel immediately familiar is that continuity: nothing about the surface itself changed.

**What should feel immediately stable** is the absence of anything still marked Provisional — the first thing a user perceives is that nothing here is still moving. That absence, not any announcement, is what tells them assessment activity has concluded.

**Communicating "concluded, but still available"** is the specific job of the Report's currency statement, already established in Report Identity. It carries two messages at once: this reflects everything submitted so far (nothing pending), and this is not sealed shut (more material remains welcome). A completed Report in this product is not a closed document — it's a settled one that can still grow, and the currency statement is what lets a user hold both facts at once without confusion.

---

## 3. Recognition

Before any exploration begins, a user already knows which Assessment they're looking at, what its scope is, and what its identity is — all three are answered together by Report Identity, positioned first in the Report's Information Hierarchy for exactly this reason. Recognition is guaranteed by that ordering; this section names what it accomplishes rather than introducing a new mechanism to accomplish it.

**Continuity with the Assessment previously experienced** takes one of two forms. A user who watched the Assessment form during Assessing recognizes the Report as the same thing they were just observing, now settled — nothing about its identity needed to be re-established. A user encountering it cold — returning later, or opening it for the first time without having watched it form — recognizes it the same way any Report is recognized: through its Identity, stated plainly, requiring no prior context to understand. Either way, recognition happens before a single Finding is opened.

---

## 4. Orientation

Regaining context after returning — later the same day, days later, or weeks later — is handled by exactly one mechanism, not three. The Assessment Overview alone re-establishes where things stand, regardless of how much time has passed. This document deliberately does not design different orientation behavior calibrated to elapsed time — no "welcome back" treatment that scales with absence, no summary of "what happened while you were away." Time-scaling would be a new mechanism this repository has no evidence requires, and the existing one — the Overview, plus the "updated" marker already established for anything that changed — already answers the only question orientation needs to answer: does anything here differ from what I remember?

**Decision:** orientation is time-invariant by design. A user returning after five minutes and a user returning after five weeks are re-oriented by the identical experience.

---

## 5. Experiencing the Whole

An Assessment Report is more than its Findings added together, but that "more" belongs to the user's own understanding, not to something the product states on their behalf. `ProductSpecification.md`'s own Non-Goals rule out subjective judgments the platform hasn't evidenced and opaque scoring that hides complexity behind a verdict — so the whole cannot be a synthesis paragraph modIQ writes, no matter how naturally the prompt for one arises. What actually makes the whole greater than its parts is something already built, simply not yet named as doing this job:

- **The Overview's per-dimension breakdown lets a user hold several facts at once** — a mod that reads as Compatible but weak on Maintainability presents both, side by side, in one glance. No single Finding states that combination; the user forms it themselves, simply by seeing both dimensions together.
- **Findings grouped within the same Category let a pattern become visible without being asserted.** Three separate Findings under Structure, each individually just a fact, can suggest a trend to a reader who notices they're adjacent — the product groups them; it doesn't draw the conclusion for the user.
- **The same shape, repeated across every Finding, is what makes comparison possible at all.** A user can only notice that three Findings share a pattern because every Finding is presented identically — consistency of form is the precondition for the user's own synthesis, not a substitute for it.

**Decision:** the whole lives in the act of comparison, which the Report makes effortless — not in a conclusion the Report reaches first. This is the one place this document had to actively resist inventing something (an aggregate summary, a cross-dimension callout) in favor of naming what was already quietly doing the job.

---

## 6. Relationship Between Parts

Assessment → Finding → Recommendation → Evidence never fragments into a sequence of unrelated screens, because none of the individual transitions were ever designed that way. The Overview points to a Finding (Workspace Evolution); a Finding's Recommendation reads as the same claim continuing to speak (Finding Presentation); its Evidence reads as that same claim briefly showing its work (Evidence Exploration). Each step widens or deepens focus within one continuous act of understanding — none of them is a destination separate from the one before it.

This section does not redesign any of those individual relationships — it names that, taken together, they compose into a single unbroken chain at the scale of the whole Report, not just within one Finding at a time. A user moving from the Overview all the way down to a specific piece of Evidence and back has never left the Assessment; they've only moved through how much of it they're currently looking at.

---

## 7. Supporting Decisions

The Report helps a user decide, not merely informs them, without requiring any feature beyond what already exists:

- **Prioritization** is already accomplished by fixed Severity ordering — a user doesn't need a separate planning view, because the reading order already is the priority order.
- **Planning** is supported by the Report simply persisting exactly as left — a user can read now and decide later, returning to find nothing changed except whatever was genuinely updated.
- **Confidence** — already scoped to individual Recommendations — is the existing mechanism for judging how much weight to give any specific piece of guidance while planning.
- **Revisiting before acting** costs nothing, by the same reversibility and no-forced-pacing principles already established — a user can return to a Recommendation and its Evidence as many times as they want before committing effort to it.

**Decision:** no checklist, task tracker, or "mark as addressed" feature is introduced here. Deciding is designed as a natural continuation of reading — severity tells a user what matters most, Confidence tells them how much to trust it, and reversibility means revisiting costs nothing — not as a separate activity requiring its own tool.

---

## 8. Returning

Section 4 established the mechanism; this section names the feeling it produces. Because nothing about a Report's structure, terminology, or organization changes between visits — unless something was genuinely updated — a returning user's memory of "roughly how this reads" transfers completely. They are not learning an interface again; they are reactivating one they already know, because it is, quite literally, unchanged.

**"I remember this" dominates "I need to learn this again" because the product never gives a returning user a reason to relearn anything.** The same consistency that lets a user compare Findings against each other within one visit (§5) is exactly what lets them recognize the whole Report across visits separated by any length of time.

---

## 9. Trust

Trust in an individual Finding and trust in a specific piece of Evidence were both already established. Trust in the Assessment as a whole is not a new mechanism — it is what accumulates when the same honest pattern (Provisional/Final labeling while it mattered, plain language, traceable Evidence, urgency bounded by Severity) holds across every single Finding a user reads, not just one.

**Decision:** a user doesn't come to trust the Assessment because any one Finding earned it — they come to trust it because the pattern never broke across however many Findings they read. Consistency at scale is itself the assessment-level trust signal, on top of everything already established at the Finding level. The Report Identity's currency statement (§2) contributes the one additional, assessment-level fact this pattern alone can't provide: that what's being read is current, not a stale subset.

---

## 10. Completion

There is no "you have finished reading" moment to design, the same way there is no "Assessment complete" event to design (Assessing & Progressive Discovery, §8) — completion of reading is a personal realization a user reaches on their own, not a state the product announces.

**What makes that realization trustworthy** is that a user can stop whenever they feel equipped, confident they haven't missed something more important further down — because Severity ordering already guarantees the most consequential Findings were encountered first. Knowing when you're done depends on trusting the order, not on being told you've reached an end.

**"I know what this Assessment is telling me"** follows from the whole (§5) and the chain (§6) both holding together without contradiction. **"I know what I want to do next"** follows naturally from that, because Recommendations were never separated from the understanding that produced them (§7) — there is no second step required to translate understanding into a plan; the plan was already sitting next to the understanding the whole time.

---

## 11. Accessibility & Interaction Principles

Everything established in Sessions 1–4 — keyboard interaction, discoverability, reversibility, interruption tolerance, user control, no forced pacing, stable attention position, independent expansion, severity-bounded presentation, non-disruptive expansion, no compounding exploration cost — continues to apply and is not repeated here. Two principles are genuinely new at the scale of a complete Report:

- **Time-invariant re-entry.** Returning to a completed Report costs the same and behaves the same regardless of how much time has passed — no special handling scaled by absence (§4).
- **Order-trust.** A user is never required to read every Finding to be confident they've seen the most important ones — trusting the fixed Severity order is sufficient, and this document treats that as a standing principle, not only an observation about how completion feels (§10).

---

## 12. Interaction Consistency Review

- **Against The Assessment Report (Product Design):** Report Identity, the Overview-first Information Hierarchy, the living-artifact model, and the recognized-absence completion model are all used exactly as defined — this document adds no new anatomy.
- **Against The Assessment Experience:** the Recognition → Orientation → Understanding → Confidence → Action return arc is reused, not redesigned; this document supplies the behavioral mechanism underneath it (§4, §8) rather than a competing arc.
- **Against Finding Presentation and Evidence Exploration:** the Finding-as-continuation and Evidence-as-showing-its-work relationships are reused exactly (§6), extended only by naming that they chain together at Report scale.
- **One tension resolved explicitly:** §5's demand that "the whole exceed the sum of its parts" was checked directly against `ProductSpecification.md`'s Non-Goal barring subjective judgment and opaque scoring — resolved by locating the whole in the user's own act of comparison across consistently-presented, already-existing parts, not in a new product-authored synthesis.
- **Another resolved explicitly:** §4's three named return timescales (same day, days, weeks) were deliberately not given three different behaviors — one mechanism serves all of them, avoiding an unwarranted new time-scaled abstraction.
- **A third:** §7's "supporting decisions" was checked against the standing Non-Goal on introducing navigation/workflow systems — resolved by showing that existing severity-ordering, Confidence, and reversibility already do this job, with no new planning feature required.
- **§4 and §8 distinguished explicitly** so they don't duplicate one another: §4 is the mechanism (the Overview, time-invariant), §8 is the felt consequence of that mechanism (recognition dominating rediscovery).

No terminology drifted. No behavioral contradiction remains. No Product Design artifact was changed. No new abstraction survived scrutiny into the final document.

---

## 13. Chief Architect Reflection

**Strengths:** this document's hardest task — §5, "experiencing the whole" — was resolved without violating the Non-Goal against subjective synthesis, by locating the whole in the user's own comparison rather than in anything the product asserts. That same discipline held across §4 and §7, where the more tempting answer (time-scaled orientation, a planning feature) was each considered and declined in favor of showing that existing structure already does the job.

**Remaining ambiguity:** exactly how vividly a user perceives cross-dimension or cross-Finding patterns (§5) is described only as something the design makes *possible*, not something it actively *scaffolds* further — deliberately left there, since scaffolding it further would risk introducing an interpretive layer the product doesn't currently offer, which is a Product Design question, not one this document should answer on its own.

**Preparation for Session 6:** this document's §6 described the felt continuity of the Assessment → Finding → Recommendation → Evidence chain but explicitly stopped short of the broader mechanics of moving around the workspace as a whole — how a user actually gets from one part of a large Report to another, across all three workspace states together. That is Session 6's task (Navigation & Workspace Behavior), and this document's own "no navigation systems" boundary was kept deliberately intact to leave it there.

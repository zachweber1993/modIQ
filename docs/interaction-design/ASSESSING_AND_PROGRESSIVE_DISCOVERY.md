# Assessing & Progressive Discovery — Version 1 Interaction Design

| Property | Value |
|---|---|
| **Initiative** | Interaction Design — Assessing & Progressive Discovery |
| **Phase** | Interaction Design — Session 2 |
| **Status** | Approved |
| **Governed by** | Workspace Evolution, The Finding, The Assessment Experience, Evidence, The Assessment Report, Assessment Intake & Upload (frozen constraints) |
| **Scope** | Behavior of the Assessing state: beginning assessment, progressive discovery, living assessment, user attention, trust during progress, interruption & recovery, and the transition into a completed Assessment |
| **Out of scope** | Product concepts, implementation, runtime behavior, engine internals, data model, visual layout |

This document specifies behavior, not concepts. Every product term used below (Assessment, Assessing, Provisional, Final, Finding, Evidence, Recommendation, Assessment Overview) is used exactly as already defined and is not reinterpreted here. Every decision is checked against the Product Design Filter: does it make obtaining, understanding, or acting on a trustworthy assessment easier?

---

## 1. Purpose

Assessing exists to make the product's actual work — collecting Evidence, forming Findings — legible while it happens, not to occupy the user until a result is ready. Understanding is genuinely forming during this state: it begins at nothing (Intake) and ends at a completed Assessment (Reviewing), and Assessing is the interval where that change is directly observable rather than inferred after the fact.

This phase exists because the alternative — hiding the work behind a wait state and revealing everything only once — would contradict Transparency and would manufacture a false sense of an instantaneous, unexplainable result. Assessment is not loading, and it is not a progress bar: a progress bar implies a fixed, knowable quantity of work counting down to zero, which is not what Assessing is. There is no percentage-complete indicator and no time estimate anywhere in this design — both would state a precision the Assessment doesn't have, and both belong to the "avoiding false certainty" discipline already established for Findings, applied here to the process itself.

---

## 2. Beginning Assessment

**What changes:** the workspace enters Assessing exactly as Workspace Evolution defines it; the single Intake action is no longer offered; content begins to appear.

**What remains stable:** the Assessment Subject identity the user confirmed at Intake does not disappear and does not need re-confirmation — it is the first thing visible in Assessing and stays visible without interruption all the way through Reviewing. This is the same identity that will later anchor the Assessment Report; Assessing is not a separate waiting room before that identity "arrives" — it's already there from the first moment.

**How continuity feels preserved:** there is no navigation away from Intake into a different screen. The same single workspace surface that showed the empty state and the submission now begins populating in place. Nothing the user just did needs to be trusted blindly across this boundary — it remains visible, not asserted.

---

## 3. Progressive Discovery

**Findings appear as they're produced, not batched for a single reveal.** But arrival order is not display order: Findings are always grouped by Category and ordered by Severity within it, per The Finding. During Assessing, a new Finding is inserted into its already-designated position in that fixed order — the list of visible Findings *grows*, but a Finding, once shown, never moves and nothing already visible is reshuffled to make room for it. This is a deliberate reconciliation: streaming arrival and a stable presentation order would conflict if new items caused continuous re-sorting, so growth is additive, never disruptive to what's already there.

**The Assessment Overview fills in progressively**, exactly as Workspace Evolution already establishes — its Mod Health dimensions and Finding counts update as their related Categories are evaluated. A dimension not yet reached does not read as failing, empty, or zero — it reads as *not yet reached*, neutrally, the same way a Category with no Findings yet simply has nothing to show rather than an absence to explain.

**Uncertainty becoming clarity** describes the aggregate picture, not any single Finding's own formation. A Finding, whenever it first appears, already carries its full anatomy — Title, Summary, Category, Severity — marked Provisional; there is no earlier, partial form of a Finding with some fields still missing. What becomes clearer over time is the whole picture: more of the Mod Health dimensions filled in, more Findings visible, more of the Assessment Subject accounted for — not any individual Finding gradually resolving its own content.

**Pacing must be honest in both directions.** Findings appear at the pace evidence is actually gathered and evaluated — never padded with an artificial delay to seem thorough, and never batched together to reveal instantly when meaningful evaluation genuinely took time. This document does not prescribe how long Assessing takes; it only requires that whatever pacing genuinely occurs is shown as it happens, neither compressed nor performed.

---

## 4. Living Assessment

The workspace during Assessing shows: newly appeared Findings, in their stable position; Evidence accumulating behind each Finding, reachable the same way it always is; the Assessment Overview's dimensions and counts updating as Categories are reached.

Two items from this section's own framing need a careful reconciliation, not a literal implementation, against the frozen model:

- **"Confidence increasing"** does not mean a Confidence value appears or rises on a Provisional Finding — Confidence is scoped exclusively to a Recommendation on a Final Finding, and no Provisional Finding carries one, by design. What legitimately increases during Assessing is the user's own trust in the process (§6) and the completeness of the aggregate picture — a feeling, not a displayed number.
- **"Recommendations becoming possible"** does not mean Recommendations begin appearing piecemeal during Assessing — a Recommendation requires a Final Finding, and Findings become Final together, as a set, at the transition into Reviewing (§8), not individually while Assessing continues. What's real here is anticipation: as more Findings accumulate, a user can sense that Recommendations will likely exist once things settle — a felt quality, not an early disclosure of any Recommendation's content.

**The workspace should feel alive without becoming distracting.** New content appears in place, the same restrained way everything else in this product appears — no toast notifications, no counters animating upward for effect, no separate activity feed of what's being inspected. The unit of attention remains the Finding, exactly as already established; raw Evidence being gathered is not itself something the user watches.

---

## 5. User Attention

**What should attract attention:** the Assessment Overview's changing state, and newly visible Findings — both simply present in the layout the user is already looking at, not pushed at them through interruption.

**What should remain peripheral:** the mechanics and pacing of evidence gathering itself. There is no activity log, no per-Evidence-item announcement, and no requirement to watch continuously — the Overview alone should always be sufficient, at any moment, to answer "how much has been figured out so far" without reading anything further.

**Decision:** avoiding overload here means the same thing it means everywhere else in this product — one unit of attention (the Finding), one entry point (the Overview), nothing competing with either for the user's notice.

---

## 6. Trust During Progress

This continues the behavioral trust sequence named in Assessment Intake & Upload — submission → recognition → **Assessment → Finding** → Evidence — picking up exactly at the two links Intake left for this session to design.

- **Trust in the Assessment itself** comes from watching it visibly work: Provisional Findings and a filling-in Overview are proof the process is running, which is a stronger form of trust than being told to wait and shown a result later.
- **Trust in each Finding** comes from its Provisional label being honest rather than reassuring — the workspace never states or implies "this is probably right," it states plainly "this is still forming," and lets the user calibrate their own trust accordingly.

**Decision:** as in every other document in this series, trust is never asserted here. No progress commentary, no "almost there," no reassurance copy of any kind appears anywhere in this design — only the visible, honestly-labeled process itself.

---

## 7. Interruption & Recovery

- **User leaves:** Assessing continues exactly as it would if they stayed. It is not watching-dependent — nothing pauses, resets, or times out because attention moved away, consistent with Workspace Evolution's position that there is no Idle state to fall into.
- **User returns:** they see the current state of Assessing exactly as it stands — the same Overview, the same accumulated Findings — with no "here's what you missed" catch-up narration required, because nothing was ever hidden while they were away; they simply weren't looking at it.
- **Assessment completes while absent:** this is not a special case. Returning to a now-completed Assessment is the same experience already designed for returning to an Assessment Report after any absence, long or short (The Assessment Report, §8–9) — Recognition, then Orientation from the Overview, then Understanding. Nothing new is designed here; this session's Assessing behavior simply hands off into that already-designed return experience.
- **Cancellation of an in-progress Assessment:** left undecided, deliberately. No frozen Product Design artifact establishes an abandon/cancel concept for an Assessment once Assessing has begun — Workspace Evolution's only reversible, no-cost withdrawal is scoped to Intake, before anything has started. Inventing one here would be introducing a new concept in an Interaction Design document, which this document is constrained not to do. This is flagged as an open product question for a future session or explicit Product Owner decision, not resolved by assumption.

---

## 8. Completion

The transition from Assessing into a completed Assessment should read as a settling, not a stop. Consistent with The Assessment Experience's own position that completion is a recognized absence — no remaining Provisional content — rather than an event: the same Findings and the same Overview that were already visible simply finish resolving. Provisional labels across the current set of Findings clear together as they become Final; nothing already on screen is replaced, hidden, or reloaded to produce this moment.

**Decision:** there is no "Assessment complete" screen, banner, or transition animation to design, because there is no discrete event to mark — only the last of what was already forming finishing the process of settling into something the user can now treat as trustworthy.

---

## 9. Emotional Experience

**Curiosity** (picking up exactly where Assessment Intake & Upload's own arc handed off, at Expectation) → **Growing engagement** (as Findings accumulate and the Overview fills in) → **Understanding** (a felt sense of knowing what's going on, even before anything is Final) → **Anticipation** (sensing that Findings are settling and Recommendations will likely follow) → **Completion** (a felt arrival at Reviewing, not an abrupt stop).

This arc is a magnified view of the Curiosity-to-Understanding span already named in The Assessment Experience's own broader journey (Uncertainty → Curiosity → Understanding → Confidence → Resolve) — it does not extend into Confidence or Resolve, which remain Reviewing's own territory for a future session to design.

---

## 10. Accessibility & Interaction Principles

Assessment Intake & Upload already established keyboard interaction, discoverability, reversibility, interruption tolerance, and user control as principles governing every future interaction, including this one — restated only by reference, not repeated here. Two principles are genuinely new, specific to a state where content actively streams in for the first time:

- **No forced pacing.** The user is never required to wait a minimum time, take an action, or "unlock" anything to see the next piece of understanding — whatever is ready is already shown.
- **Stable attention position.** Because new content is always additive and never reorders or removes what's already visible (§3), a user's reading position within the growing set of Findings is never disrupted by something new appearing elsewhere in the list.

---

## 11. Interaction Consistency Review

- **Against Workspace Evolution:** the three-state model, Provisional/Final labeling, and the Overview's progressive fill-in behavior are used exactly as defined — this document adds streaming presentation behavior around them, not a new state or a new labeling rule.
- **Against The Finding:** Category/Severity grouping and ordering are preserved under streaming arrival by making growth strictly additive (§3) — a reconciliation, not a change to the ordering rule itself. No new Finding field, status, or sub-state was introduced; the temptation to add an intermediate "partially formed" Finding state was identified and deliberately rejected.
- **Against The Assessment Experience:** this document's emotional arc (§9) is explicitly scoped as a magnification of that document's own Curiosity-to-Understanding span, not a competing or extended arc.
- **Against Evidence:** unchanged — Evidence remains reachable only through a Finding, exactly as already specified; nothing here asks the user to watch Evidence accumulate directly.
- **Against The Assessment Report:** the Report-return experience is reused, not redesigned, for the "Assessment completes while absent" case (§7).
- **Against Assessment Intake & Upload:** the behavioral trust sequence it named is continued, not restarted or redefined — this document designs exactly the two links (Assessment, Finding) that document left open.
- **Against the Glossary and Principles:** no term was used inconsistently with its constitutional definition; Transparency and the avoidance of false certainty (§1, §3, §6) are the same principles already governing every prior document, applied here to a live process rather than a static report.
- **One deliberate non-decision, named rather than resolved:** cancellation of an in-progress Assessment, and replacing an Assessment Subject entirely mid-Assessing, are both left open (§7) — neither is addressed by any frozen artifact, and neither is invented here.

No Product Design concept was changed. No new abstraction was introduced. This document remains behavior-focused throughout.

---

## 12. Chief Architect Reflection

This document had a narrower, harder job than Assessment Intake & Upload's: Intake designed behavior around a state with no streaming content at all, while this session had to design streaming behavior without inventing a new mechanism to hold it. The two places that required real judgment — reconciling fixed presentation order against incremental arrival (§3), and reconciling this session's own suggested language ("confidence increasing," "recommendations becoming possible") against the frozen Provisional/Final and Confidence-scoping rules (§4) — were both resolved by applying an existing rule more carefully, not by adding a new one. The one place this document declined to design anything at all — cancellation of an active Assessment — was left open because inventing an answer would have been a Product Design decision wearing an Interaction Design document as cover.

If a weakness remains, it is that §7's two open questions (cancellation, Subject replacement mid-Assessing) leave a real gap in the V1 experience: a user who wants to stop or redirect an Assessment already running has no designed behavior to rely on yet. That gap is named, not hidden, and is offered here for Chief Architect judgment on whether it warrants its own session or can reasonably wait.

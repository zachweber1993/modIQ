# Assessment Intake & Upload — Version 1 Interaction Design

| Property | Value |
|---|---|
| **Initiative** | Interaction Design — Assessment Intake & Upload Experience |
| **Phase** | Interaction Design — Session 1 |
| **Status** | Approved |
| **Governed by** | Workspace Evolution, The Finding, The Assessment Experience, Evidence, The Assessment Report (frozen Product Design constraints) |
| **Scope** | Behavior of the Intake state: the empty workspace, upload interaction, transition into Assessing, immediate feedback, and error recovery |
| **Out of scope** | Product concepts, implementation, runtime behavior, data model, visual layout |

This document specifies behavior, not concepts. Every product term used below (Assessment Subject, Evidence, Provisional, Assessing) is used exactly as already defined and is not reinterpreted here. Every decision is checked against the Product Design Filter: does it make obtaining, understanding, or acting on a trustworthy assessment easier?

---

## 1. Purpose

Intake exists to remove uncertainty about the act of beginning — not about the mod itself. Before Assessing can mean anything, the user needs to be sure of three things: that modIQ recognized what they gave it, that it recognized it correctly and completely, and that they're still free to add more before anything is evaluated.

**The uncertainty Intake removes:** what counts as an acceptable Assessment Subject, whether the right material was selected, and whether the submission actually went through.

**The confidence Intake creates before Assessment begins** is confidence in the *submission*, never confidence in the *mod*. Intake should never leave a user with any impression — however faint — about whether the mod is good, bad, compatible, or risky. That understanding belongs entirely to the Assessment that hasn't started yet.

This distinction points to a second, earlier trust sequence, sitting beneath the Product Design trust chain rather than beside it: submission → recognition → Assessment → Finding → Evidence. Where Evidence → Finding → Recommendation establishes how a *conclusion* is justified, this earlier sequence establishes how the *process* itself earns trust before any conclusion exists. It is not a new mechanism — it is the pattern already present across §4, §5, and §7 below — named here only because Intake is the one place in the product where this earlier sequence, and nothing else, is what's actually being designed.

---

## 2. Empty Workspace

An empty workspace reads as an invitation, not an absence — it is **prepared for the user**, not merely waiting on them.

**What it communicates:** what an Assessment Subject can be (an individual mod, a mod collection, a savegame, a map, or another supported subject type, described in the same plain terms the Glossary already uses), how to provide one, and — just as importantly — that doing so is simple and low-stakes. Beginning costs the user nothing to try: it is a single, intentionally simple action, and everything about it remains fully reversible before Assessing begins (§6, §8). The workspace is entitled to invite the user to begin, rather than merely permit it, precisely because that invitation is backed by something already true elsewhere in this design — there is no risk being papered over.

**What's immediately available:** exactly one action — submitting an Assessment Subject. Nothing else competes with it for the user's attention.

**What is intentionally absent:**
- No history, list, or record of prior Assessments cluttering this state.
- No settings, preferences, or configuration to complete before submitting anything.
- No placeholder Findings, Evidence, or Mod Health content of any kind — since nothing has been assessed, nothing pretends to already know something.
- Nothing else asked of the user before the one available action. Beginning an Assessment should require nothing but the Assessment Subject itself.

**Decision:** the invitation this workspace extends is earned, not decorative — it rests entirely on facts this document already establishes (reversibility, a single simple action, no prerequisites), so strengthening the empty state's tone required no new behavior, only surfacing what was already true.

---

## 3. Upload Experience

- **Selecting material:** a direct, deliberate action — the user chooses what represents the thing they want assessed.
- **Dragging material:** an equally valid alternative to selecting. Both produce identical recognition and identical feedback — neither is treated as more "correct" than the other.
- **Multiple files:** when an Assessment Subject is naturally made of more than one file, the workspace treats everything provided together as one submission, not as competing candidates. The user should never need to explain that a set of files belongs together beyond simply providing them together.
- **Replacing material:** submitting something new before Assessing has begun cleanly replaces what was there, with a clear acknowledgment that a replacement happened — never a silent swap, and never a required "remove, then re-add" sequence.
- **Optional supporting material** (such as a log file): offered alongside the primary Assessment Subject, but visibly distinguished from it — the user should understand they are adding something that will support evidence collection, not submitting a second Subject. It remains optional at every point; Assessing is never blocked on its presence or absence.
- **Unsupported material:** named plainly and immediately, without silently discarding it and without blocking the rest of a submission that is otherwise valid. A single unrecognized item does not fail an entire submission.
- **Duplicate submissions:** when the same material appears more than once in what's being provided, the workspace recognizes this and says so plainly, without implying an error was made — and still leaves the choice of how to proceed with the user.

---

## 4. Transition into Assessment

**What changes:** the workspace moves from Intake to Assessing. The single Intake action (submit) is no longer offered, because the workspace now has a Subject.

**What remains stable:** the same single workspace surface. Nothing about what was just submitted — the Subject itself, any supporting material included — disappears or requires re-confirmation; it carries forward directly into the Assessment Overview's identity information exactly as already specified.

**How the user understands Assessment has begun:** the workspace begins showing signs of activity immediately, consistent with Assessing already being designed to surface partial results rather than a blank wait. This document does not redesign that behavior — it only establishes that the transition is where it begins.

**Continuity:** nothing the user did during Intake needs to be trusted blindly across the transition — it remains visibly, exactly what they provided.

---

## 5. Immediate Feedback

**What reassurance the user receives:** that the submission was received, recognized as a valid Assessment Subject, and that something is now underway.

**What uncertainty deliberately remains:** everything about the mod itself. Nothing about health, compatibility, or quality is known yet, and nothing should imply otherwise.

**What should never be implied too early:** no "looks good so far" framing, no fabricated progress, no Finding-shaped content before a real Finding exists. The moment right after submission, before any Evidence yet exists, should be honest about being exactly that — a beginning, not a simulation of progress. As soon as genuine Provisional content exists, it appears exactly as already designed; nothing here introduces a separate "loading" fiction ahead of it.

**Decision:** immediate feedback is entirely about the process succeeding, never a preview of the outcome. This is the earliest point at which "provisional means provisional" has to be honored, and it is honored by having nothing to show yet rather than showing something invented.

---

## 6. Error and Recovery

- **Unsupported files:** named clearly; the rest of a valid submission proceeds unless nothing usable remains.
- **Incomplete uploads:** the user is told plainly that something didn't fully arrive and is invited to retry — the workspace never treats an incomplete submission as a valid, if troubled, Assessment Subject.
- **Corrupted archives:** named clearly, without diagnostic detail — diagnosis belongs to Evidence, once an Assessment exists, not to Intake. Recovery is simply providing the material again or providing something else.
- **Duplicate material:** named without alarm, as in §3, with the user still in control of how to proceed.
- **Cancellation before Assessment begins:** fully reversible, with no confirmation required — nothing has been committed to yet, so withdrawing a pending submission carries no cost and should carry no ceremony.

**Decision:** every Intake problem is framed as information the workspace needs in order to proceed correctly — never as a failure attributable to the user or a verdict on the mod. This is the same posture used everywhere else in this product: named plainly, never hidden, never dramatized.

---

## 7. Progressive Understanding

Intake is the first lesson in how modIQ works, before a single Finding exists.

- **Terminology:** introducing "Assessment Subject," and the distinct role of optional supporting material, teaches the platform's vocabulary at the exact moment the user needs it — before any Finding assumes they already know it.
- **Expectations:** Intake establishes, ahead of time, that modIQ will explain itself, that understanding will arrive progressively rather than all at once, and that additional material remains welcome throughout — so none of the Provisional/Final behavior that follows comes as a surprise.
- **Transparency:** Intake is honest that it knows nothing about the mod yet (§5) — the earliest possible enactment of Transparency, before there is anything to be transparent about.
- **Confidence:** successfully completing Intake — submitting something and seeing it clearly recognized — is the user's first small proof that the product does what it says, which primes trust for everything that follows. This is the first link in the submission → recognition → Assessment → Finding → Evidence sequence named in §1: trust in the process precedes trust in any conclusion.

---

## 8. Accessibility and Interaction Principles

Stated here as principles governing every future interaction, not only Intake:

- **Keyboard interaction:** every action available by pointer — selecting, dragging, replacing, cancelling — is equally reachable without one.
- **Discoverability:** the single available action at any given moment should be self-evident, never requiring the user to already know how modIQ works.
- **Reversibility:** any action taken before Assessing begins can be undone without penalty or ceremony.
- **Interruption tolerance:** a user can pause mid-Intake and return without losing what they had already provided or being forced to restart.
- **User control:** the user alone decides when enough has been provided and when to begin. modIQ never starts Assessing on its own initiative, and never demands more material than the user chooses to give first.

---

## 9. Emotional Experience

**Uncertainty** (I have a mod I'm not sure about) → **Preparation** (gathering or selecting what to submit) → **Commitment** (the act of submitting) → **Curiosity** (immediate feedback that something is happening) → **Expectation** (anticipating what Assessing will reveal).

This arc is a prelude, not a parallel — it is designed to end exactly where the already-established Assessment Experience arc begins to unfold further, not to repeat it.

---

## 10. Interaction Consistency Review

- **No product concepts changed:** Assessment Subject, Evidence, Provisional/Final, and the three workspace states are used exactly as already defined; this document adds no new concept, only behavior around the existing Intake state.
- **No new abstractions introduced:** selecting, dragging, replacing, and supplying supporting material are described as interaction behaviors, not as new product objects competing with Assessment Subject. The named submission → recognition → Assessment → Finding → Evidence sequence is a description of existing behavior's order, not a new object or a rival to the Evidence → Finding → Recommendation trust chain — it operates one layer beneath it, describing process trust rather than conclusion trust.
- **Interaction reinforces understanding:** introducing "Assessment Subject" vocabulary at first submission, and staying honest about knowing nothing yet, both directly reinforce the trust model and Progressive Understanding already established rather than introducing a separate one.
- **Trust model preserved:** immediate feedback (§5) gives no reassurance about the mod itself, only about the process — consistent with the existing rule that Confidence and any implication of certainty must be earned, never granted early.
- **One tension resolved explicitly:** §9's emotional arc uses "Curiosity" at Intake, and the Assessment Experience's own arc also begins with Curiosity during Assessing. These are not the same moment repeated — Intake's Curiosity is about whether the submission was recognized (process curiosity); the Assessment Experience's Curiosity is about what is being discovered (content curiosity). Intake's arc is deliberately built to terminate at Expectation specifically so it hands off cleanly into that later Curiosity, rather than overlapping or duplicating it.
- **Error handling extends existing interaction philosophy into new territory:** §6's low-drama, non-blocking treatment of Intake problems is the same "no modals for informational content, nothing hidden, nothing dramatized" philosophy already established for Findings and Evidence, applied here for the first time to error conditions rather than to informational content.

No conflicts identified. This document does not modify any Product Design artifact, and does not introduce implementation, runtime, or data model decisions.

---

## Readiness Review

**Against Product Definition:** unchanged — this document serves the same target users generically and introduces no new capability, only behavior around an existing one.

**Against Product Design Closeout:** the frozen artifacts it names are treated as stable constraints throughout; nothing here was authored as if still open for redesign.

**Against Workspace Evolution:** the Intake state, its single available action, and the three-state model are used exactly as defined — untouched by either editorial refinement.

**Against The Finding:** Finding anatomy and hierarchy are referenced only by name, never redefined; the newly named sequence ends at "Finding → Evidence" strictly as a pointer to where Intake's concerns stop, not as a restatement of Finding's own design.

**Against The Assessment Experience:** the emotional-arc handoff at Expectation/Curiosity, already reconciled, is preserved unchanged; the new sequence named in §1 sits alongside, not on top of, that document's own journey stages.

**Against Evidence:** Evidence's anatomy, disclosure rules, and scope are untouched; it appears in the newly named sequence only as an endpoint, exactly as already positioned in every other document.

**Against The Assessment Report:** unaffected — this document never reaches as far as Reviewing.

**Confirmations:**
- No Product Design concept was unintentionally changed.
- No implementation decision was introduced — both editorial refinements applied during review were experiential and naming refinements, not new mechanics.
- No duplicate abstraction was created — the submission → recognition → Assessment → Finding → Evidence sequence is explicitly framed as a different layer (process trust) of the same trust model, not a second, competing hierarchy. It is deliberately written in lowercase prose rather than given Glossary-style capitalization, so it reads as an observed pattern, not a newly minted product term standing alongside Finding, Evidence, and Recommendation.
- Interaction Design remains behavior-focused — neither refinement defines what anything *is*; both describe how the user experiences something that already exists.

No remaining weaknesses were identified. This artifact stands as the first Interaction Design artifact.

---

## Final Reflection

This document demonstrates the Interaction Design/Product Design boundary by what it did *not* need to do: it defined no new anatomy. Every Product Design session before this one had to specify new structure — Finding fields, Evidence fields, Report sections. This session specified sequence, tone, feel, and recoverability around objects whose structure was already complete, and the one thing it named as new — the behavioral trust sequence — was assembled entirely from behaviors the document had already described, not invented to fill a gap.

That is the clearest possible evidence that Interaction Design is behaving as a distinct phase here: it had genuine new work to do (how does beginning *feel*, what does *recognition* look like, what happens when something goes wrong before anything has been evaluated), and none of that work required, or was allowed to require, changing what an Assessment Subject, a Finding, or Evidence *is*. The phase boundary held under real pressure — including the pressure of a genuinely interesting insight (the earlier trust sequence) that would have been easy to mis-elevate into a new product concept, and instead was correctly kept as a description of behavior.

---

## Chief Architect Approval

> This artifact successfully establishes the Interaction Design phase as distinct from Product Design. It defines user behavior without redefining product concepts, preserves every constraint established by the approved Product Design artifacts, and introduces no implementation or architectural decisions. The document demonstrates consistent interaction philosophy, trust progression, progressive understanding, and behavioral continuity with the existing product model.
>
> No further revisions are recommended.
>
> This document is accepted as the baseline Interaction Design specification for Assessment Intake & Upload.

**Status:** Approved.

# The Assessment Report — Version 1 Product Design

| Property | Value |
|---|---|
| **Initiative** | The Assessment Report |
| **Phase** | Product Design — Session 6 (Capstone) |
| **Status** | Approved in principle |
| **Builds on** | Workspace Evolution, The Finding, The Assessment Experience, Evidence |
| **Scope** | The complete Version 1 Assessment Report experience |
| **Out of scope** | Product philosophy, export formats, storage/persistence mechanisms, UI layouts |

Every decision below is checked against the Product Design Filter: does it make obtaining, understanding, or acting on a trustworthy assessment easier?

This document defines the Assessment Report itself. The capstone reflection on the complete Version 1 Product Design body of work — covering all five approved artifacts together, their recurring patterns, and the remaining conceptual gaps — is recorded separately in `docs/engineering/PRODUCT_DESIGN_CLOSEOUT.md`, since that reflection concerns the whole phase rather than the Assessment Report specifically.

---

## 1. Purpose of an Assessment Report

An Assessment Report exists so that the understanding an Assessment produces can stand on its own — legible to someone who never watched it form, not only to someone who was present while it was Assessing.

**The question it answers** is the same one the whole Assessment exists to answer — *does this mod work, and can I trust that answer* — but asked by someone arriving cold, without the context of having lived through Intake and Assessing themselves.

**How it differs from the live Assessment experience:** the live experience is the whole journey — Intake, Assessing, Reviewing, Departure, any number of Supplementations. The Assessment Report corresponds only to the Reviewing portion of that journey. It is the destination, not the journey — a returning or first-time reader never re-experiences Provisional discovery; they only ever encounter what the Assessment currently, coherently says.

**Why someone returns to it later:** to re-check before reinstalling or updating a mod, to reference a specific Finding when discussing it with others, to verify whether a previously identified issue was ever addressed, or to finally act on a Repair Recipe they didn't have time for at first. This is the direct product expression of the stated Success Criterion that "Assessment Reports become trusted references."

---

## 2. Relationship to the Assessment

An Assessment is the whole process; an Assessment Report is what that Assessment looks like whenever it is legible as a coherent whole — which is to say, whenever it is Reviewing.

**When an Assessment becomes a Report:** the first time Assessing completes and Reviewing begins. From that point forward, the Assessment *is* a Report — one that can still change.

**Snapshot, living artifact, or something else:** a living artifact, never a frozen snapshot. This is the only answer consistent with the workspace remaining receptive to additional material — a Report generated once and then disconnected from further updates would contradict that principle directly. Submitting more material doesn't produce a second Report or a new version of one; it continues updating the same Report the same way Assessing already resumes on top of Reviewing.

**Consistency with continuity over time:** the same "updated" marking already used to signal a same-session supplement is exactly what should serve a user returning after weeks or months. No new mechanism is needed for long-absence returns — the existing convention already answers "has anything changed since I last understood this," regardless of whether the gap was five minutes or five months.

**Decision:** the Assessment Report is not a separate object built from the Assessment — it is the Assessment, named and framed so it can be read as a standalone thing. Nothing about "becoming a Report" changes what's inside it.

---

## 3. Report Anatomy

The Report's anatomy is deliberately minimal, because nearly all of it already exists:

- **Report Identity** — a plain statement of what Assessment Subject this Report is for, and a plain statement of currency (that it reflects everything submitted to this Assessment so far). This is the one genuinely new element this session introduces, and it exists specifically to serve a reader who arrives without the live session's context.
- **Assessment Overview** — reused exactly as already designed in Workspace Evolution: Mod Health across its dimensions, Finding counts by severity, and whether Recommendations are available. This remains the Report's primary entry point.
- **Findings** — grouped and ordered exactly as already specified, each carrying its own Recommendation when Final and warranted, with Evidence reachable through it.

**Explicitly absent, by design:** there is no separate Recommendations section and no separate Evidence section. A naive instinct toward "a report" might expect separate chapters for these — this design rejects that instinct in favor of keeping Recommendations and Evidence attached to the Finding that justifies them, exactly as already established. The absence of these sections is as much a structural decision as the presence of the others.

---

## 4. Information Hierarchy

**Report Identity → Assessment Overview → Findings (with attached Recommendations) → Evidence (on demand).**

Identity comes first here specifically because a Report, unlike the live in-session experience, may be read without the context of having submitted the Subject oneself — before the Overview can mean anything, the reader needs to know what it's an overview *of*. After that, the sequence is the same interpretation-before-raw-material pattern used everywhere else in this product: broadest understanding first, most detailed material last and only on request.

---

## 5. Progressive Disclosure

The Report introduces no new disclosure rules — it inherits the ones already designed. Report Identity and the Assessment Overview are visible immediately on opening. Findings are visible as a scannable list — Title, Severity, Category, Recommendation headline — without expansion. Full Recommendation detail and Evidence remain behind the single per-Finding expansion already specified in the Finding design.

**Decision:** the correct amount of new disclosure design for the Report is none. Reusing the existing rules exactly is what keeps the Report from becoming a second, subtly different reading experience layered on top of the one already built.

---

## 6. Trust Experience

The Report never asks to be trusted — it is shown, not claimed. Because it is built from the same Findings, Evidence, and Recommendations already governed by Provisional/Final labeling, scoped Confidence, and per-Finding traceability, trust in the Report is inherited directly from decisions already made in Workspace Evolution, Finding, and Evidence design. Nothing about being called a "Report" changes any of that.

The one trust-relevant addition this session makes is the currency statement in Report Identity (§3) — so a reader can trust that what they're looking at reflects everything submitted so far, not an out-of-date subset quietly missing a later supplement.

---

## 7. Educational Experience

Revisiting a Report reinforces learning because nothing about its terminology, structure, or explanation pattern changes between visits — a returning user's prior familiarity transfers directly rather than needing to be relearned. Encountering the same kind of Finding again, now with more accumulated context, naturally builds pattern recognition on its own. The Report doesn't add teaching content on top of what Findings and Evidence already teach — it simply is that same consistent structure, encountered again, which is what makes repetition itself educational rather than redundant.

---

## 8. Longevity

- **Orientation:** Report Identity immediately reorients a returning reader to what this Report concerns, without depending on memory of the original session.
- **Remembering previous conclusions:** since the Report's structure is unchanged since the last visit unless something specifically updated, a user's rough memory of "where things stood" still applies directly.
- **Quickly understanding what matters:** the Assessment Overview alone should be sufficient to tell a returning reader whether anything has changed since they last looked, before they need to reread anything further.
- **Acting without rereading everything:** because Recommendations remain exactly where they were — attached to their Finding — a user who only wants to check whether they ever acted on one specific Recommendation can go straight back to it.

---

## 9. Emotional Experience

**Recognition** ("I remember this mod, this is the one I was concerned about") → **Orientation** (the Overview — roughly where things stood) → **Understanding** (Findings, refreshed or unchanged) → **Confidence** (Evidence and Recommendations still exactly where they were) → **Action** (acting on what the Recommendations already said, now with the benefit of time).

This is a genuinely different arc from the first-time Assessment journey, not the same arc replayed — a return visit begins from Recognition, reactivating prior understanding, rather than from Uncertainty, building understanding from zero. This distinction is named explicitly so it isn't mistaken for an inconsistency with the Assessment Experience's emotional journey — the two are appropriate to two different circumstances, not two conflicting descriptions of one.

---

## 10. Consistency Review

- **Against Workspace Evolution:** the Report corresponds exactly to Reviewing — no new workspace state, no new Confidence rule, no new anatomy beyond Report Identity.
- **Against Finding Experience:** Findings inside a Report behave with no new fields and no new hierarchy — reused exactly.
- **Against Assessment Experience:** the Report is precisely "a completed Assessment" (§1 of that document) given a standalone identity; the earlier decision that completion is a recognized absence rather than an event is preserved — a Report is never "finalized" as an action, it simply is the current expression of the Assessment whenever nothing is Provisional.
- **Against Evidence Experience:** unchanged — Evidence is never surfaced separately in the Report, only reachable through a Finding, exactly as already specified.
- **Against the Stable Product Principles:** "the workspace remains receptive to additional material" is the principle this entire session had to satisfy in §2, and it is satisfied by treating the Report as living rather than as a snapshot. All other principles are inherited unchanged from the documents that already established them.
- **Against Product Definition:** this session directly fulfills two lines from the frozen specification — "Assessment Reports become trusted references" (Success Criteria) and "Assessment Reports prioritize explanation over scoring" (Glossary) — the second of which is upheld by this design never once introducing a composite score anywhere in the Report's anatomy.

No conflicts identified. This draft does not modify Vision.md, Principles.md, ProductSpecification.md, Glossary.md, or Architecture.md, and has not been evaluated for architectural feasibility.

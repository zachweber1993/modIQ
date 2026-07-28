# Workspace Evolution — Version 1 Product Design Decisions (Revised)

| Property | Value |
|---|---|
| **Initiative** | Workspace Evolution |
| **Phase** | Product Design — Session 2 (Refinement) |
| **Status** | Approved in principle |
| **Supersedes** | Workspace Evolution — Version 1 Product Design Decisions (Session 1 draft) |
| **Scope** | Workspace behavior from assessment start to exit: states, progressive disclosure, streaming Findings, mid-assessment uploads, navigation, interaction patterns, recommendation hierarchy, Assessment Overview |
| **Out of scope** | Product philosophy, Assessment Framework internals, engine implementation, data model, visual UI |

Every decision below is checked against the Product Design Filter: does it make obtaining, understanding, or acting on a trustworthy assessment easier?

---

## Revision Notes (Session 2)

This session refines, and does not redesign, the approved Session 1 draft. Six changes were made:

1. Idle is removed as a workspace state. It was a session-continuity fact, not a distinct Assessment experience.
2. Supplementing is removed as a fourth workspace state. It is reframed as Assessing recurring while Reviewing is already active — the same state, occurring again, not a new one.
3. Findings now have an explicit Provisional/Final distinction, which resolves how streaming Findings, mid-assessment updates, and Confidence interact.
4. Repair Recipe is formalized as a structured form a Recommendation can take, not a third tier beneath it — matching the Glossary's own definition.
5. The Assessment Overview (Layer 1) is fully designed for the first time.
6. A Consistency Review is added, including one correction the refinement surfaced: Recommendations should only attach to Final Findings, never Provisional ones.

---

## 1. Workspace States (Revised)

The workspace has **three** states, each representing a meaningful change in the Assessment experience itself:

1. **Intake** — no Assessment Subject yet. The only action available is submitting one.
2. **Assessing** — evidence collection and rule evaluation is producing new or changed Findings. Whatever Findings exist are visible immediately, marked Provisional (see §2).
3. **Reviewing** — an Assessment Report exists and its Findings are Final. The user explores the Overview, Findings, Evidence, and Recommendations at will.

**Idle is not a workspace state.** Whether the user is actively interacting or has stepped away, the workspace and its last-known Assessment content remain exactly as they were. Nothing about the Assessment experience changes when interaction pauses — there is no waiting, no timeout, and no separate screen. This is a property of the workspace persisting, not a state the user experiences.

**Supplementing is not a fourth state.** When new material is submitted while Reviewing is already active, the workspace re-enters **Assessing** on top of the existing Reviewing content rather than replacing it. The user continues to see and interact with the Final Findings already Reviewing while Assessing runs again in the background. The workspace shows an "Updating" indication for the duration, but the underlying state is Assessing, recurring — not a new experience the user has to learn.

**Decision:** Three states is the complete set. Every state boundary corresponds to a real change in what the user can trust about the content in front of them (nothing yet / still forming / settled) — nothing else in the workspace changes what the user is experiencing.

---

## 2. Streaming Findings: Provisional vs. Final

Findings produced while the workspace is in the Assessing state are **Provisional**. Findings belonging to a completed Assessment Report are **Final**. This distinction is always visible wherever a Finding appears — a user should never have to infer which kind they're looking at.

- **Provisional Findings** are shown as soon as they exist, with the Evidence gathered for them so far. They carry a plain-language marker (e.g., "still evaluating") rather than any numeric measure.
- **No Confidence value is shown on a Provisional Finding.** Confidence reflects evidence quality, and evidence quality for a Provisional Finding is, by definition, not yet settled — showing a number would state a certainty the Assessment doesn't have yet. Confidence appears only once a Finding is Final.
- **A Provisional Finding may change or disappear** as Assessing continues (further evidence may reclassify or resolve it). The workspace does not treat this as an error or a retraction to apologize for — it is presented as normal, expected behavior of an Assessment still in progress.
- **A Provisional Finding carries no Recommendation.** Recommending an action based on evidence that might still change would put a Recommendation ahead of settled Evidence, which the Stable Product Principles rule out ("Recommendations are supported by Findings, and Findings by Evidence"). Recommendations attach only once a Finding reaches Final (see §6, Consistency Review).
- When Assessing completes, Provisional Findings resolve into Final Findings in the Assessment Report. Any Finding whose content changed between its Provisional and Final form is marked "updated" for the remainder of the session, so the user can see exactly what settled differently than first shown.

**Decision:** Provisional is a distinct, honestly-labeled state of a Finding, not a lesser-quality version of the same thing — it exists specifically to prevent false certainty during an incomplete Assessment, while still satisfying "getting an assessment easier" by not making the user wait in silence for anything to appear.

---

## 3. Incremental Assessment Updates (Refined)

Submitting additional material while Reviewing is active moves the workspace back into Assessing (§1), scoped to the same Assessment Subject.

- **The workspace explicitly confirms what was added** — the newly submitted material is named back to the user (e.g., "Included: [item]") before any new Findings appear, so the user knows their upload was recognized before they see any effect from it.
- **Only the Findings affected by the new material change.** Everything else in the existing Final Assessment Report remains visible, untouched, and still marked Final throughout — the user never loses the report they were already reviewing while an update runs.
- **New and changed Findings resulting from the update pass through Provisional before becoming Final**, exactly as in §2 — there is only one Provisional/Final mechanism in the workspace, used consistently whether a Finding originates from the first Assessment or from a later update.
- **Newly updated Final Findings are visibly marked "updated" for the remainder of the session**, distinguishing them from Findings that were already Final and unaffected by the new material.
- **Trust is maintained by traceability**, not by reassurance: the user can see which uploaded item produced which new or changed Finding, the same way a Finding's Evidence is always traceable back to what produced it.

**Decision:** An incremental update is communicated the same way the rest of the workspace communicates everything else — by showing exactly what changed and why, not by asserting that it worked.

---

## 4. The Recommendation Hierarchy (Formalized)

The Glossary defines a Repair Recipe as "a structured recommendation describing how a specific issue can be resolved" — a Repair Recipe is a *kind of* Recommendation, not a separate object beneath it.

The V1 hierarchy is therefore two levels, not three:

- **Finding** → **Recommendation**
- A Recommendation may be a **plain Recommendation** (advisory, not mechanically reproducible) or a **Repair Recipe** (structured, reproducible) — these are two forms the same tier can take, not two tiers.

**Decision:** The workspace presents every Recommendation the same way — attached to its Final Finding — and simply presents it *as* a Repair Recipe when it is reproducible, rather than adding a step the user must open to "reach" the Recipe. This keeps the user's mental model to two links (Finding → Recommendation), matching the Glossary exactly rather than introducing a UI-only third tier.

---

## 5. Progressive Disclosure of Findings and Evidence

Layering is unchanged in structure, now stated precisely against the Provisional/Final and Recommendation-hierarchy decisions above:

- **Layer 1 — Assessment Overview.** Designed in full in §6, below.
- **Layer 2 — Findings.** Grouped by category, each shown with severity, Provisional/Final status, and — only once Final — its Recommendation (plain or Repair Recipe).
- **Layer 3 — Evidence.** Reached only by expanding a Finding, Provisional or Final alike.

**Decision:** unchanged from Session 1 — Evidence is never browsed independently of a Finding.

---

## 6. The Assessment Overview (New)

### Purpose

The Overview is the first thing a user sees for a given Assessment Subject, at any workspace state (Assessing or Reviewing). It answers the Glossary's first Assessment Report question — **does the mod work?** — together with a second, implicit question the streaming-Findings refinement makes necessary: **can I trust this answer yet?** The Overview must answer both before the user is asked to explore anything else.

### What belongs in the Overview, in presentation order

1. **Assessment Subject identity** — what was assessed (name and Assessment Subject type). This confirms scope before anything else is read.
2. **Assessment completeness** — whether the Assessment is still Assessing (Provisional) or Reviewing (Final). This is stated plainly, before any health information, so the user's trust in what follows is correctly calibrated from the first moment.
3. **Mod Health**, presented across its stated dimensions (Compatibility, Stability, Maintainability, Performance, Structure, Engineering Quality) — never collapsed into one composite score. This is the direct answer to "does the mod work."
4. **Finding counts by severity** (e.g., how many Errors, Warnings, Informational, Best-Practice Findings exist) — a preview of Layer 2, not its content.
5. **Whether Recommendations exist** (a plain statement that actionable Recommendations are available), only once the Assessment is Final. No individual Recommendation or Repair Recipe is named here.

### What intentionally does not appear in the Overview

- **Evidence.** Never surfaced above Layer 3, regardless of workspace state.
- **Individual Recommendations or Repair Recipes.** Naming one at the Overview level would create a second entry point to Recommendations, competing with their attachment to a Finding (§4) — there is exactly one path to a Recommendation, through its Finding.
- **A single composite Mod Health score.** Explicitly excluded, consistent with the platform's Non-Goal against opaque scoring.
- **Confidence values**, at any workspace state. Confidence belongs to a specific Recommendation, not to the Assessment as a whole, and only ever appears once that Recommendation's Finding is Final (§2).

### Behavior while Assessing

The Overview is visible from the moment any Evidence exists — it does not wait for a Final Assessment Report to appear at all. Its Mod Health dimensions and Finding counts fill in progressively as their related categories are evaluated, and it carries the same Provisional labeling as the Findings underneath it. This keeps the Overview consistent with "the workspace remains receptive" and with Progressive Understanding — the first thing the user sees is never a blank wait state.

**Decision:** The Assessment Overview becomes the mandatory entry point into every Assessment Report — Reviewing always opens on the Overview, never on Layer 2. It answers "does it work, and can I trust that yet" before the user is asked to do anything else, which is the most direct way the design satisfies "understanding the assessment" as a first impression rather than an eventual one.

---

## 7. Navigation

Unchanged in substance from Session 1: a single workspace per Assessment Subject, no route-based pages for Overview/Findings/Evidence/Recommendations. One addition:

- **Reviewing always opens on the Overview** (§6). Reaching Findings or Evidence is always a deliberate step down from it, never the default landing point.

---

## 8. Interaction Patterns

Unchanged in substance from Session 1 (expand-in-place, no modals for informational content), restated with corrected terminology:

- The "Updating" indication described in §1 and §3 is a non-blocking signal within Reviewing, not a separate screen — consistent with "no modals for informational content."

---

## 9. V1 Non-Goals (unchanged)

- No multi-user/collaborative workspace state.
- No editing of Assessment Subjects or mods from within the workspace.
- No user-configurable layout, filtering, or saved views of Findings/Evidence.
- No persistence/versioning of workspace UI state beyond the Assessment Subject itself.
- No composite Mod Health score (carried forward explicitly from §6).

---

## 10. Consistency Review

Checked the full revised draft against itself and against the Stable Product Principles:

- **Progressive understanding** — preserved and strengthened: the Overview now gives the layering an explicit first step, and Provisional/Final gives every layer a consistent way to represent partial understanding without overstating it.
- **Curiosity should never create friction** — preserved: no new modal, confirmation, or blocking step was introduced anywhere in this revision.
- **The workspace remains receptive to additional material** — preserved and clarified: §3 now specifies exactly how receptiveness is communicated, rather than only asserting that it happens.
- **Recommendations are supported by Findings, and Findings by Evidence** — this review surfaced a real gap in the Session 1 draft: nothing previously prevented a Recommendation from attaching to a still-forming Provisional Finding, which would have let a Recommendation outrun its Evidence during Assessing. §2 now states explicitly that Recommendations attach only to Final Findings. This is the one substantive correction this session made, not just an expansion.
- **Terminology check** — "Supplementing" (Session 1) is retired in favor of "Assessing, recurring" (§1) so the document does not describe four states in one place and three in another.
- **Hierarchy check** — §4's two-tier Recommendation model was cross-checked against §5 and §6, both of which now refer only to "Finding → Recommendation (plain or Repair Recipe)," with no remaining reference to a separate Repair Recipe tier anywhere in the document.

No conflicts remain unresolved. This draft does not modify Vision.md, Principles.md, ProductSpecification.md, Glossary.md, or Architecture.md, and has not been evaluated for architectural feasibility — that remains the next stage if this refinement is approved.

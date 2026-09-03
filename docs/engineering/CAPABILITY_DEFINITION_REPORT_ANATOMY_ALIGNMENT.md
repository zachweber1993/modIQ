# CAPABILITY_DEFINITION_REPORT_ANATOMY_ALIGNMENT.md

## 1. Capability Identity

**Name:** Report Anatomy Alignment
**Type:** Capability Definition (first artifact of this project's standard lifecycle: Capability Definition → Process Determination → [stages TBD] → Implementation)
**Origin:** `IMPLEMENTATION ALIGNMENT ASSESSMENT — REPORT EXPERIENCE`, Determination A, Candidate 1 of 2
**Scope:** Console presentation of Overview, Finding, and Evidence anatomy, brought into conformance with already-frozen product/interaction specification, for information the console already receives.

## 2. Problem Statement

The console's Reviewing experience (`apps/console/src/workspace/Reviewing.tsx`, `Overview.tsx`) implements an earlier, narrower slice of a specification that has since been extended (Interaction Design Sessions 3–5, "Approved," and certified frozen). Specifically: the Overview shows no synthesized Mod Health picture despite the data existing on every Finding; Findings are ordered by severity only, with no category grouping despite each Finding already carrying its category; and several fields the frozen anatomy requires to be always-visible (category, status, a Recommendation headline, Evidence availability) are currently reachable only by expanding a Finding. The console today under-presents information it already possesses.

## 3. Product/User Value

Directly closes the most-cited concrete gap from the prior Product Experience Assessment (PX-06: no synthesized health picture) using a mechanism the product design already approved specifically to avoid inventing a score (dimensional presentation, per `WORKSPACE_EVOLUTION.md` §6 and `ASSESSMENT_REPORT_EXPERIENCE.md` §5). It also makes triage faster (category-grouped Findings, always-visible status/headline) without adding any new concept a user has to learn — every element aligned here is already specified, just not yet built.

## 4. Source-of-Truth Requirements

Re-read directly: `WORKSPACE_EVOLUTION.md` §6 (Assessment Overview anatomy, presentation order, exclusions); `THE_FINDING.md` §2–5 (Finding anatomy, information hierarchy, progressive disclosure, multi-Finding grouping/ordering); `EVIDENCE.md` §2–4 (anatomy, information hierarchy, progressive disclosure); `THE_ASSESSMENT_REPORT.md` §3–5 (Report anatomy, hierarchy, disclosure — confirms it "inherits the ones already designed," introducing no new rules of its own); the `docs/interaction-design/README.md` certification table (all seven sessions "Approved," Version 1 "certified and frozen as the implementation specification").

## 5. Current Implementation Baseline

Verified against current file state (`apps/console/src/workspace/Reviewing.tsx`, `Overview.tsx`, `apps/console/src/engine/types.ts`, `apps/console/src-tauri/src/assessment.rs`).

Precision check on data conversion: `mod_health_dimension` and `status` are populated in the Tauri bridge via `format!("{:?}", finding.mod_health_dimension())` / `format!("{:?}", finding.status())` (`assessment.rs:166-167`) — Rust `Debug` formatting of the enum variant, not a purpose-built label. This already reads as an ordinary word ("Compatibility," "Final") and requires no backend change to use, but is worth naming precisely: it is an incidental string, not a designed one. A Rust-side test, `every_finding_carries_non_empty_severity_title_summary_dimension_and_status` (`assessment.rs:220-227`), already confirms these fields are always non-empty in the payload.

## 6. In-Scope Behaviors

The eight items, individually validated in §8 rather than assumed as a set:

1. Mod Health aggregate presentation in the Overview, across dimensions, never a composite score.
2. Finding grouping by category (Mod Health dimension) as the primary sort key.
3. Severity ordering within each category (already-correct logic, needs nesting under #2).
4. Finding collapsed anatomy: Title, Category, Severity, status, Summary always visible without expansion.
5. Provisional/Final status shown at the always-visible layer, not buried in the expanded panel.
6. A one-line Recommendation headline visible collapsed, once Final and present, without full detail.
7. Evidence-availability indication visible at the collapsed Finding layer.
8. Evidence progressive disclosure: Label/Source/Explanation-equivalent always visible on Finding-expansion; raw Content behind a further reveal only when substantial.

## 7. Out-of-Scope Behaviors

Explicitly excluded, per the governing alignment assessment: `AssessmentSubject`/Report Identity; Report currency; Evidence Explanation as its own field (architecturally unresolved); Confidence; Initiative 1 (progress observability); Initiative 2 (reentrancy/resubmission); C5 cross-mod comparison; C12 historical presentation; Knowledge activation; any transport-contract redesign; any architecture amendment; any governance reconsideration; Error Experience Alignment (defined separately, not here); any console cleanup unrelated to the eight items above.

**One boundary case, stated precisely:** item #8's "Explanation" language in `EVIDENCE.md` §3 names a field this console does not have (see Out-of-Scope above) — in-scope for this capability is only the *disclosure behavior* around whatever fields the console currently has (`description`, `location`, `label`, `source`, `content`), not renaming or reinterpreting them as "Explanation." This capability presents existing fields progressively; it does not resolve what Explanation is or add it.

## 8. Data and Interface Availability — Item-by-Item Verification

| # | Item | Frozen Requirement | Current Behavior | Delta | Supporting Data/Interface | Implementation Boundary | Dependency | Verification Criterion |
|---|---|---|---|---|---|---|---|---|
| 1 | Mod Health aggregate | `WORKSPACE_EVOLUTION.md` §6: dimensions shown, never a composite score | `Overview.tsx` shows severity counts + recommendation-exists only; explicit comment states dimension aggregation was out of prior scope | Not built | `finding.modHealthDimension` present on every `FindingSummary` (`types.ts:51`), already delivered in full | `Overview.tsx` only | None | Overview displays a count or presence indicator per one of the six named dimensions, computed from already-received `findings`, with no single combined number anywhere |
| 2 | Category grouping | `THE_FINDING.md` §5: group by Category first | `Reviewing.tsx:42-44` sorts by severity only | Not built | Same field as #1 | `Reviewing.tsx` only | None | Findings render grouped by `modHealthDimension`, each group internally severity-ordered |
| 3 | Severity ordering within category | `THE_FINDING.md` §5 | Severity ordering logic already correct, just not nested | Ordering correct; nesting missing | `severityRank`/`SEVERITY_ORDER` already implemented (`Reviewing.tsx:139-144`) | `Reviewing.tsx` only | None | Same as #2 — verified jointly |
| 4 | Finding collapsed anatomy | `THE_FINDING.md` §2-4 | Title/Severity/Summary visible; category and status inside expansion (`Reviewing.tsx:67-75` vs. `:80`) | Two fields misplaced | `finding.modHealthDimension`, `finding.status` already in the collapsed row's own data object | `Reviewing.tsx` only | None | Category and status readable without clicking any Finding |
| 5 | Provisional/Final visibility | Always-visible marker | Present but buried (`Reviewing.tsx:80`); **backend note:** every Rule constructs `FindingStatus::Final` — no Rule anywhere produces `Provisional` (confirmed by direct grep across `crates/modiq-rules/src/rules/`) | Placement wrong; value is currently always "Final" | `finding.status` already delivered | `Reviewing.tsx` only | None for placement. Full future value (an actual "Provisional" ever appearing) depends on Initiative 1 — out of scope, not blocking this item | Status readable without expansion; document that today it will always read "Final" |
| 6 | Recommendation headline | `THE_FINDING.md` §4: one-line headline collapsed, once Final and present | Full Recommendation only shown on expansion (`Reviewing.tsx:82-86`) | Not built at collapsed layer | `finding.recommendation.action` already delivered as a single string | `Reviewing.tsx` only | None | A one-line action statement visible without expansion whenever `recommendation !== null`; full repair-step detail remains behind expansion |
| 7 | Evidence-availability indication | `THE_FINDING.md` §2: always-visible | Not shown until expansion; nothing distinguishes "no evidence" from "not yet expanded" even then | Not built | `finding.evidence.length` computable from the array already fully delivered (not lazily fetched — confirmed: one payload, `assessment.rs:161-179`) | `Reviewing.tsx` only | None | An "Evidence available" (or equivalent) indicator visible without expansion whenever `evidence.length > 0` |
| 8 | Evidence progressive disclosure | `EVIDENCE.md` §4: Label/Source/[current fields] always visible on expand; substantial Content behind a further reveal | All fields, including full `content`, rendered together in one block (`Reviewing.tsx:105-127`) | Not built | `evidence.content` already delivered in full as a string | `Reviewing.tsx` only | None | Label/Source/description-equivalent visible on Finding-expansion; a long/substantial `content` requires one further explicit action to reveal; brief content displays inline |

No item in this table required a Runtime, Storage, or transport-contract change to verify feasibility — every field consumed is already present in the current payload shape.

## 9. Architecture and Governance Boundary

- **Runtime/Application/Storage change:** none appears necessary for any of the eight items. All required data already flows through the existing pipeline into the existing transport.
- **Transport-contract change:** none appears necessary. Every field consumed already exists in `ReportSummary`/`FindingSummary`/`EvidenceSummary` today. This capability does not depend on GOV-008 resolving, and does not itself constitute a transport change.
- **Architecture:** remains sufficient as-is for this capability's scope.
- **Governance:** remains clear — no Open governance item bears on any of the eight items.

## 10. Verification / Completion Criteria

Objective completion, stated without prescribing implementation: for each of the eight items, the specific verification criterion in §8's rightmost column is met, observable directly in the running console against a real assessed mod (the same fixture path already used by existing Rust tests). Completion does not depend on any future capability, any of the excluded dependencies in §7, or any change outside `apps/console`.

## 11. Testing Considerations

The console currently has **zero automated frontend tests** — no `.test.`/`.spec.` file exists anywhere in `apps/console/src`, and `package.json` defines no test script. This capability's eight behaviors are all frontend-rendering behaviors, meaning today's only available verification method is manual inspection against the running app plus the existing Rust-side data-presence test (`assessment.rs:220-227`), which confirms the *inputs* are non-empty but says nothing about how they're *rendered*. Whether this capability should establish its own frontend testing approach (and what tooling that would require) is a real open question this Capability Definition surfaces but does not answer — it belongs to Process Determination, since it may carry its own tooling/architecture consideration.

## 12. Capability Coherence Assessment

- **Independently meaningful?** Yes — closes a real, specific, previously-identified product gap (PX-06) on its own, without requiring Error Experience Alignment or any excluded dependency.
- **Scope coherent?** Yes: all eight items govern exactly one continuous structure — the Reviewing state's Overview → Findings → Evidence hierarchy — under exactly the same three product-design documents and the same certified Interaction Design arc (Sessions 3–5). None of the eight belongs to a different governing document than the others. This is not eight unrelated fixes; it is one hierarchy's presentation brought into conformance, top to bottom.
- **Desired behavior already specified?** Yes for all eight, each cited to a specific section (§8).
- **Testable?** Each item has a stated, observable verification criterion (§8, §10); *how* it gets tested (manual vs. an established frontend suite) is open (§11), not the *what*.
- **Architecture sufficient? Governance clear?** Yes, confirmed (§9).
- **Completion demonstrable without future capabilities?** Yes — nothing in §8's verification criteria depends on Error Experience Alignment, Initiative 1/2, GOV-002/C11, Evidence Explanation, or Confidence.
- **Clean beginning and end?** Yes — begins from the current, precisely-catalogued baseline (§5, §8) and ends when all eight verification criteria hold simultaneously; nothing about the capability's own definition invites scope creep beyond the eight items.

No criterion failed. Nothing was found requiring the capability to shrink below eight items or expand beyond them.

## 13. Findings

**CAP-01** — All eight items consume data already present in the current transport payload; none requires a Runtime, Storage, or transport-contract change. *Confidence:* High.

**CAP-02** — `mod_health_dimension` and `status` reach the console via Rust `Debug` formatting (`{:?}`), not a purpose-built label — usable as-is, but worth naming precisely rather than assuming it's designed copy. *Significance:* minor, informational for whoever eventually implements this.

**CAP-03** — Every Rule in the current pipeline always constructs `FindingStatus::Final`; `Provisional` is real in the Runtime but never produced. Aligning its display placement (item #5) is fully in scope; the value it will ever show beyond "Final" is not, and depends on Initiative 1 (out of scope). *Significance:* moderate — prevents this capability from being mistaken for delivering live Provisional/Final behavior.

**CAP-04** — The console has no frontend test suite at all today. *Significance:* the one open consideration this Capability Definition surfaces without resolving, reserved for Process Determination.

**CAP-05** — All eight items share one governing document set and one continuous UI hierarchy (Overview → Findings → Evidence) — coherence is structural, not coincidental grouping. *Significance:* supports keeping the capability as one unit rather than splitting it further.

## 14. Final Determination

**A — Capability definition is sound and ready for Process Determination.**

## 15. Recommended Next Process Step

**Process Determination** for Report Anatomy Alignment — to decide, per this project's own standard lifecycle, whether this capability requires a dedicated Architecture Evaluation stage or can proceed directly toward Implementation Authorization, and specifically to address the frontend-testing consideration named in §11 before any implementation begins.

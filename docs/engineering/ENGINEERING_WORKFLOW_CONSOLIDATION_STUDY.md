# Engineering Workflow Consolidation Study

| Property | Value |
|----------|-------|
| **Document** | ENGINEERING_WORKFLOW_CONSOLIDATION_STUDY.md |
| **Project** | modIQ |
| **Purpose** | Determine whether the repository's engineering workflow has reached sufficient maturity to become a formally documented, single canonical process — and where |
| **Prepared by** | Lead Engineer, on `feature/runtime-implementation` |
| **Status** | Documentation analysis and design only. No repository documentation modified. No code, tests, commits, merge, or Repository Closeout performed. Awaiting Chief Architect review before any workflow documentation is changed. |

---

# 1. Current Repository State

Every place in the live repository that describes a Sprint-shaped or engineering-shaped workflow, located by direct search, not assumption:

| # | Document | Section | Stages (as written) |
|---|---|---|---|
| 1 | `PROJECT_HANDOFF_v1.0.md` | "Project Structure" | Project Owner → Chief Architect → Lead Engineer → Implementation → Chief Architect Review → Sprint Closeout |
| 2 | `PROJECT_HANDOFF_v1.0.md` | Section 5, "Engineering Workflow" | Cross-references #7 as current, then reproduces #3 below as retained background |
| 3 | `docs/governance/EngineeringGuide.md` | "Engineering Workflow" | Review specification → Identify boundaries → Implement → Test → Verify → Report → Submit for review |
| 4 | `GOVERNANCE.md` | "Repository Development Cycle" | Documentation Release → Sprint Planning → Implementation → Engineering Review → Engineering Release → Governance Review → Documentation Release |
| 5 | `CHIEF_ARCHITECT_HANDOFF_v1.0.md` | Section 6, "Decision Framework" | Observation → Evidence → Investigation → Governance → Implementation → Verification → Review → Closeout |
| 6 | `CHIEF_ARCHITECT_HANDOFF_v1.0.md` | Section 9, "Sprint Oversight Workflow" | Planning → Governance → Implementation Authorization → Implementation Review → Sprint Closeout Review → Repository Reconciliation → Documentation Audit → Engineering Release Approval |
| 7 | `CHIEF_ARCHITECT_HANDOFF_v1.0.md` | Section 10, "Engineering Workflow" (self-declared "Permanent") | Sprint Planning → Chief Architect Review → Architectural Resolution → Authorization → Implementation → Validation → Implementation Report → Architecture Review → Sprint Closeout |
| 8 | `LEAD_ENGINEER_HANDOFF_v3.0.md` | "Standard Sprint Execution" | Nine-step numbered mirror of #7, execution-side |
| 9 | `docs/implementation/ImplementationWorkflow.md` | (whole file, no heading) | Specification → Architecture Review → Sprint Task → Implementation → Testing → Implementation Report → Technical Review → Commit → Milestone Complete |

A tenth, adjacent artifact: `docs/governance/templates/ImplementationReportTemplate.md` — a real, complete report-format template (Summary, Files Modified, Public API Changes, Specification References, Invariants Implemented, Tests Added, Design Decisions, Assumptions Made, Known Limitations, Architectural Concerns, Verification), referenced by `EngineeringGuide.md` but not consistently followed by this Sprint's own implementation reports (which use an overlapping but not identical section set). Format consistency, not stage-sequence consistency — noted here because it's part of the same underlying pattern, addressed separately in Section 6.

**Is there already a natural canonical location?** Yes, in intent: #7 explicitly declares itself "The Permanent Engineering Workflow defines the required execution sequence for all future implementation work," and #2 already defers to it ("Section 10 of `CHIEF_ARCHITECT_HANDOFF_v1.0.md` is current"). In practice, no — nine independent descriptions exist, only one of which is self-aware that it's supposed to be authoritative, and even that one has already needed correcting once (Section 2, below).

---

# 2. Workflow Consistency Review

Grounded findings, none silently reconciled:

**Terminology drift, same concept, different names across live documents:**
- "Sprint Planning" (#7) vs. "Capability Definition" (this Sprint's actual practice, `SPRINT7_CAPABILITY_AND_IMPLEMENTATION_PLAN.md`) vs. "Planning" (#6) vs. "Review the relevant specification" (#3) vs. "Sprint Task" (#9).
- "Chief Architect Review" (#1, #7 — 2nd stage) vs. "Architecture Review" (#7 — 7th stage, a *different* stage using the *same words* the 2nd stage in #1/#7 avoids) vs. "Technical Review" (#9, a pre-rename artifact) vs. "Engineering Review" (#4) vs. "Governance" (#5, #6).
- "Sprint Closeout" (#1, #7) vs. "Repository Closeout" (established two sessions ago, not yet reflected in #7's own diagram at all) vs. "Closeout" (#5) vs. "Milestone Complete" (#9) vs. "Engineering Release Approval" (#6).
- "Authorization" (#7) vs. "Implementation Authorization" (#6, already using the fuller name *within the same document* #7 uses the shorter one).

**A concrete, freshly-discovered instance of duplication causing drift, not hypothetical:** #2's own cross-reference to #7 — added during Sprint 6 Closeout to prevent exactly this kind of staleness — already omits "Architectural Resolution," added to #7 one session later. A summary written specifically to prevent drift went stale within two sessions. This is direct, repository-native evidence that distributed, hand-synchronized copies of the same workflow do not stay synchronized, even when someone is actively trying.

**Stages present in some descriptions and absent from others:**
- "Architectural Resolution" exists only in #7 and #8 (both edited in the same recent session) — absent from #1, #2's reproduction, #5, #6, #9.
- "Commit" and "Merge" are explicit, named stages only in #9 (old) and in this session's proposed 11-stage list — absent from #7, the current self-declared canonical version, even though Sprint 6 and Sprint 7 both demonstrably went through separately-authorized Commit and Merge steps in practice.
- "Repository Closeout" as its own concept (distinct from Sprint Closeout) exists nowhere in a stage *list* — it was defined in prose (Section 10 of #7) two sessions ago but never added to the diagram itself.

**Stale documents, not merely differently-worded:**
- #9 (`ImplementationWorkflow.md`) has no metadata table (every other document in this repository has one), uses "Technical Review" — a term that predates the Chief Architect rename entirely — and is referenced by nothing else found in this search. It reads as a Sprint-0-era scratch file never revisited.
- #3 (`EngineeringGuide.md`) is dated 2026-07-16 (Sprint 0/1 era) and states "Architectural dependencies... require Technical Director review" — the pre-rename title, unlike every handoff document, which was reconciled during the Sprint 6 governance baseline work.

**Overlapping concepts serving genuinely different purposes, not redundant:** #5 (Decision Framework) is about how an *architectural question* gets resolved in general, not Sprint execution specifically. #6 (Sprint Oversight Workflow) is the Chief Architect's own review cadence, not the shared cross-role sequence. #4 (Repository Development Cycle) operates at Documentation-Release granularity, spanning multiple Sprints — a different scale entirely from a single Sprint's internal stages. These three are not candidates for deletion; they answer different questions than "what stages does a Sprint go through," and conflating them with #7 would lose real distinctions.

---

# 3. Terminology Review

Recommended canonical vocabulary. Distinguishing the four terms specifically named, since they are the ones most at risk of being used interchangeably:

## Capability Definition
**Purpose:** Establish what new capability the work should provide, grounded in product specification and repository evidence, before any architecture is designed. **Output:** A capability-and-implementation-plan document (`SPRINT7_CAPABILITY_AND_IMPLEMENTATION_PLAN.md` is the demonstrated model). **Exit criteria:** The capability, its rationale, and its scope boundary are stated in evidence-grounded terms a non-engineer could evaluate.

## Architecture Evaluation
**Purpose:** Where a genuine architectural question exists (not every Sprint has one), evaluate concrete design alternatives against repository evidence and established principles, and recommend one. **Output:** An architecture study or proposal document (`COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md`). **Exit criteria:** Alternatives are named, evaluated on their merits, and one is recommended with rationale — not yet approved.

## Architectural Resolution
**Purpose:** Every open architectural question raised during Capability Definition or Architecture Evaluation is explicitly marked **Accepted**, **Rejected**, or **Deferred**. Nothing may carry forward silently. **Output:** A Chief Architect Decision Record (the pattern established in `COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md` Section 14 and mirrored in `SPRINT7_CAPABILITY_AND_IMPLEMENTATION_PLAN.md`). **Exit criteria:** Zero open architectural questions remain unmarked.

## Implementation Authorization
**Purpose:** Confirm every remaining precondition (dependency selection, final scope confirmation) is satisfied and give explicit, recorded permission for implementation to begin. Distinct from Architectural Resolution — resolution settles *architecture*; authorization settles *readiness to build*, which can include narrower items resolution doesn't cover (Sprint 7's own dependency selection sat between the two, demonstrated directly this Sprint). **Output:** An authorization record (`SPRINT7_IMPLEMENTATION_AUTHORIZATION.md`). **Exit criteria:** An explicit "authorized" statement from the Chief Architect.

## Architectural Conformance Review
**Purpose:** After implementation, confirm the actual result conforms to what was resolved and authorized — no undisclosed scope expansion, no unauthorized abstraction, architecture matches decision. Distinct from Architecture Evaluation (which happens *before* code exists) and from Architectural Resolution (which settles *questions*, not *conformance of a finished result*). **Output, as demonstrated so far:** informal — a stated approval at the top of a subsequent session ("Sprint 7 implementation has been reviewed by the Chief Architect and is APPROVED"), not its own recorded artifact. **Recommended output going forward:** a short, explicit record — even a few lines — mirroring the discipline Architectural Resolution already established, so conformance review leaves the same kind of citable trace resolution does.

---

# 4. Canonical Workflow Recommendation

The proposed eleven stages substantially match demonstrated practice. Evaluated against Sprint 7 specifically, stage by stage, since this Sprint is the fullest real trace available:

| Stage | Demonstrated in Sprint 7? |
|---|---|
| 1. Capability Definition | Yes — `SPRINT7_CAPABILITY_AND_IMPLEMENTATION_PLAN.md`, matching this exact name |
| 2. Architecture Evaluation | Yes — `COLLECTOR_COMPOSITION_ARCHITECTURE_PROPOSAL.md` (session titled "Architecture Study," not "Evaluation" — a naming variance worth aligning, not a structural gap) |
| 3. Architectural Resolution | Yes — the term originates directly from this Sprint's own practice |
| 4. Implementation Authorization | Yes — `SPRINT7_IMPLEMENTATION_AUTHORIZATION.md` |
| 5. Implementation | Yes |
| 6. Validation | Yes, but not purely as a discrete post-Implementation gate — see refinement below |
| 7. Implementation Report | Yes — `SPRINT7_IMPLEMENTATION_REPORT.md` |
| 8. Architectural Conformance Review | Yes, but informally — see Section 3 |
| 9. Commit | Not yet reached for Sprint 7; demonstrated in Sprint 6 |
| 10. Merge | Not yet reached for Sprint 7; demonstrated in Sprint 6 |
| 11. Repository Closeout | Not yet reached for Sprint 7; demonstrated in Sprint 6 |

**Two refinements evidence suggests, not a different structure:**

- **Validation is continuous, not solely a discrete gate.** In practice, each crate touched (`modiq-collection`, then `modiq-engine`, then `modiq-cli`, then the Sandbox) was validated immediately after its own change, with a final comprehensive pass at the end — not one single Validation step occurring only after all Implementation finished. The eleven-stage list's linear presentation is fine as a *sequence of required gates*, but should not be read as implying Validation happens exactly once, at exactly one point.
- **Architectural Resolution and Implementation Authorization are not always adjacent in practice.** Sprint 7 needed a real, separate step (XML dependency research and recommendation) between the two — Resolution closed every *architectural* question, but Authorization additionally required a narrower, non-architectural decision resolution didn't cover. The eleven-stage list correctly keeps them as separate stages; it should not be read as guaranteeing zero work occurs between them.

**Recommendation: adopt the eleven-stage sequence as the canonical workflow**, with Architectural Conformance Review's output upgraded from informal to a short recorded artifact (Section 3), and both refinements above captured in that stage's own description rather than left implicit.

---

# 5. Documentation Strategy

**Recommendation: B — consolidate into an existing canonical document.**

Not A (remain distributed): Section 2 already demonstrates concrete, repository-native drift from distribution — not a hypothetical risk. Nine independent descriptions exist; the one document that explicitly declared itself canonical was already out of sync with its own cross-reference within two sessions, before this study even began looking.

Not C (a new, standalone Engineering Process specification): this project's own architectural discipline — applied repeatedly and explicitly in this repository's history (`EngineAPI`'s retirement, GOV-004, GOV-012, and the Collector Composition Architecture Proposal this Sprint) — is to not build new structure until concrete evidence demonstrates an existing one is insufficient, not merely imperfect. A canonical home already exists in intent (`CHIEF_ARCHITECT_HANDOFF_v1.0.md` Section 10 self-declares as "Permanent"). The problem this study found is that its declared authority hasn't been backed by consolidation, not that no authoritative home is architecturally possible within current structure. Creating a tenth document to solve a nine-document sprawl problem would not, by itself, prevent an eleventh.

**Which existing document, specifically:** not `CHIEF_ARCHITECT_HANDOFF_v1.0.md` itself, despite currently hosting the most complete version. `PROJECT_HANDOFF_v1.0.md` is the better-reasoned target, on direct evidence: the workflow is used by *both* roles — `LEAD_ENGINEER_HANDOFF_v3.0.md`'s own "Standard Sprint Execution" exists specifically *because* the canonical version currently lives in a role-specific document neither role-independent reader can point to directly. `PROJECT_HANDOFF_v1.0.md` already states its own purpose as describing "how decisions get made" independently of role, and already hosts the "Project Structure" diagram (#1) — the workflow belongs beside it, not duplicated across both role handoffs indefinitely.

---

# 6. Migration Plan

No edits performed. Identified only:

**Should become canonical:** `PROJECT_HANDOFF_v1.0.md` — a new, single "Engineering Workflow" section (replacing the current Section 5, which today only cross-references elsewhere) hosting the full eleven-stage diagram, the refinements from Section 4, and the four terminology definitions from Section 3.

**Should change from full content to reference:**
- `CHIEF_ARCHITECT_HANDOFF_v1.0.md` Section 10 — retains only what's genuinely Chief-Architect-specific (what *you* do at each stage), referencing `PROJECT_HANDOFF_v1.0.md` for the stage list and definitions themselves, rather than restating them.
- `LEAD_ENGINEER_HANDOFF_v3.0.md` "Standard Sprint Execution" — same treatment, retaining only the Lead-Engineer-specific execution checklist.

**Should be preserved as-is, with an added cross-reference rather than a rewrite:**
- `CHIEF_ARCHITECT_HANDOFF_v1.0.md` Section 6 (Decision Framework) and Section 9 (Sprint Oversight Workflow) — different purposes, established in Section 2; each should gain a one-line note pointing to the canonical stage names where they overlap, so a reader can tell "Governance" (Section 5/6) and "Architectural Resolution" (canonical) are related without the documents needing identical vocabulary throughout.
- `GOVERNANCE.md`'s Repository Development Cycle — different granularity (Documentation Release, not Sprint); add a one-line note distinguishing the two scales.

**Should be explicitly flagged for retirement or archival, not silently deleted:**
- `docs/implementation/ImplementationWorkflow.md` — stale, unreferenced, pre-dates current terminology entirely. Recommend explicit Chief Architect decision to archive or delete, not a silent removal.
- `docs/governance/EngineeringGuide.md` — recommend either reconciling its terminology (Technical Director → Chief Architect) and folding its still-relevant content (Engineering Principles, Coding Standards, Definition of Done — none of which duplicate the workflow-stage question this study addresses) into a clearly-current home, or explicitly marking it historical background the way superseded handoff drafts already are.

**A separate, narrower finding worth a future decision:** `ImplementationReportTemplate.md` is real, complete, and not being followed exactly as written by recent Sprint reports. This is a report-*format* question, not a stage-*sequence* question — out of this study's own scope, but flagged so it isn't lost.

---

# 7. Chief Architect Review Questions

1. **Is `PROJECT_HANDOFF_v1.0.md` the right consolidation target**, or is there a reason to keep `CHIEF_ARCHITECT_HANDOFF_v1.0.md` as the canonical home despite the cross-role duplication it currently requires?
2. **Should Architectural Conformance Review's output become a required recorded artifact** (mirroring Architectural Resolution's own Decision Record pattern), or is an informal stated approval sufficient going forward?
3. **Should `ImplementationWorkflow.md` be archived, deleted, or reconciled** — this study did not find any current reference to it, but did not independently confirm no external process depends on its existence.
4. **Should `EngineeringGuide.md` be reconciled or retired**, and if reconciled, should its non-workflow content (Engineering Principles, Definition of Done, External Dependencies policy) move into `PROJECT_HANDOFF_v1.0.md` alongside the workflow, or remain a separate living document?
5. **Is the `ImplementationReportTemplate.md` still the intended format** for future implementation reports, given recent Sprint reports have not followed it exactly — should reports be reconciled to it, or should it be updated to match demonstrated practice?

---

Awaiting Chief Architect review before any workflow documentation is changed. No documents were modified in the preparation of this study.

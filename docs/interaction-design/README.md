# Interaction Design

Interaction Design artifacts specify how a user interacts with the already-designed Version 1 product — behavior, not concepts. They do not redefine any product object established during Product Design (Assessment Subject, Finding, Evidence, Recommendation, Assessment Report, the workspace states) and do not introduce implementation, runtime, or data model decisions.

---

## Authority

Interaction Design is authorized by `docs/engineering/PRODUCT_DESIGN_CLOSEOUT.md`, which closed the Product Design phase. Every artifact under `docs/product-design/` is treated as a frozen constraint here. Where an Interaction Design session finds it needs to change what a product concept *is* rather than how it *behaves*, that is a Product Design change and must be flagged for governance review, not made locally.

---

## Reading Order

1. `ASSESSMENT_INTAKE_AND_UPLOAD.md` — the empty workspace, upload interaction, transition into Assessing, immediate feedback, and error recovery. Approved.
2. `ASSESSING_AND_PROGRESSIVE_DISCOVERY.md` — beginning assessment, progressive discovery, living assessment, user attention, trust during progress, interruption & recovery, and the transition into a completed Assessment. Approved.
3. `FINDING_PRESENTATION.md` — the behavioral semantics of the Finding as the product's primary interaction object: first encounter, at-a-glance understanding, progressive exploration, relationship to Evidence and Recommendations, attention, trust, revisiting, and coexistence with other Findings. Approved.
4. `EVIDENCE_EXPLORATION.md` — the interaction experience of Evidence as a supporting, invited object: choosing to explore, first encounter, progressive understanding, relationship to the Finding and to Recommendations, trust, attention, leaving, and revisiting. Approved.
5. `ASSESSMENT_REPORT_EXPERIENCE.md` — the interaction experience of a completed Assessment Report as a coherent whole: first encounter, recognition, orientation across time, experiencing the whole, relationships between parts, supporting decisions, returning, assessment-level trust, and completion. Approved.
6. `NAVIGATION_AND_WORKSPACE_BEHAVIOR.md` — the behavioral semantics of movement throughout the workspace: continuity, orientation, scope (not identity), focus, returning, reversibility, stability, trust, and completion. Approved.

---

## Version 1 Interaction Design Program

Interaction Design proceeds as a coherent program of chapters answering "how is it experienced?" for each Product Design concept, rather than as isolated documents.

| Session | Artifact | Status |
|---|---|---|
| 1 | Assessment Intake & Upload | Approved |
| 2 | Assessing & Progressive Discovery | Approved |
| 3 | Finding Presentation | Approved |
| 4 | Evidence Exploration | Approved |
| 5 | Assessment Report Experience | Approved |
| 6 | Navigation & Workspace Behavior | Approved |
| 7 | Interaction Design Closeout | Next |

---

## Status

Sessions 1–6 are approved. Session 7 (Interaction Design Closeout) is next.

---

## See Also

- `docs/product-design/README.md` — the frozen product concepts this phase specifies behavior around.
- `docs/DOCUMENTATION_MAP.md` — how this collection fits into the repository's documentation as a whole.

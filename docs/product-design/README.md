# Product Design

Product Design artifacts define the Version 1 conceptual model of the modIQ assessment experience: the product objects a user interacts with, and the behavior and relationships between them. They define **what** the product is and how it is meant to be experienced — not how it behaves in interaction detail (Interaction Design), and not how it is implemented (Architecture, Engineering).

---

## Authority

Product Design is authorized by `docs/constitutional/ProductSpecification.md` and by `docs/engineering/PRODUCT_DEFINITION_CLOSEOUT.md`, which closed the preceding Product Definition phase and established the Stable Product Principles and Product Design Filter every artifact below is checked against.

Product Design artifacts do not modify `Vision.md`, `Principles.md`, `Glossary.md`, `ProductSpecification.md`, or `Architecture.md`. Where a Product Design session surfaces a genuine conflict with one of those documents, it is flagged for governance review, not resolved locally.

Product Design artifacts are, in turn, treated as frozen constraints by the Interaction Design phase (`docs/interaction-design/`) — Interaction Design specifies behavior around these concepts and must not redefine them.

---

## Reading Order

1. `WORKSPACE_EVOLUTION.md` — the workspace state model, progressive disclosure, the Provisional/Final Finding model, mid-assessment uploads, and the Assessment Overview.
2. `THE_FINDING.md` — the Finding as a product object.
3. `THE_ASSESSMENT_EXPERIENCE.md` — the complete Version 1 Assessment journey.
4. `EVIDENCE.md` — the Evidence experience.
5. `THE_ASSESSMENT_REPORT.md` — the Assessment Report as the Assessment itself, viewed in its Reviewing state.

See `docs/engineering/PRODUCT_DESIGN_CLOSEOUT.md` for the phase closeout, a reflection on this body of work as a whole, and the named conceptual gaps carried forward.

---

## Status

All five artifacts above are **approved in principle**. The Product Design phase is closed; current work has moved to Interaction Design.

---

## See Also

- `docs/interaction-design/README.md` — the next phase, built on this collection.
- `docs/engineering/PRODUCT_DESIGN_CLOSEOUT.md` — the phase closeout, reflection, and named conceptual gaps.
- `docs/DOCUMENTATION_MAP.md` — how this collection fits into the repository's documentation as a whole.

# Initiative 3 Architectural Resolution — Domain Model Anatomy Extension

| Property | Value |
|---|---|
| **Document** | INITIATIVE_3_ARCHITECTURAL_RESOLUTION.md |
| **Project** | modIQ |
| **Initiative Addressed** | Engineering Alignment Program, Initiative 3 — Domain Model Anatomy Extension |
| **Purpose** | Disposition each item and determination from `INITIATIVE_3_ARCHITECTURE_EVALUATION.md`. Treat that Evaluation as fixed evidence; do not reopen it. |
| **Origin** | Chief Architect authorization, following approval of `INITIATIVE_3_ARCHITECTURE_EVALUATION.md`. |
| **Adopted Precedent** | `INITIATIVE_5_ARCHITECTURAL_RESOLUTION.md`, `INITIATIVE_1_ARCHITECTURAL_RESOLUTION.md`, `INITIATIVE_2_ARCHITECTURAL_RESOLUTION.md`, all treated as binding, unreopened repository architecture. |
| **Status** | **Architectural Resolution. No ADR, crate, API, or implementation technology has been created, designed, or chosen. No Governance Register entry modified.** |

---

## §1 — Method

This Resolution dispositions each 3A item and the 3B determination individually, following Initiative 2's precedent of granular disposition rather than a single up-or-down vote on the whole Evaluation. No new disposition vocabulary is introduced — the existing set (Adopted / Adopted Architectural Constraint / Deferred / Rejected / Requires Additional Investigation / Requires Governance Reconciliation) covers everything the Evaluation produced. Where the Evaluation establishes an architectural fact but stops short of evaluating a specific implementation detail (naming, field ownership), this Resolution adopts only what the Evaluation actually supports.

## §2 — Item Dispositions

| Item | Disposition | Basis |
|---|---|---|
| 1 — Title/Summary | **Adopted** | No conflict found; the constructing Rule already has the information at construction time. |
| 2 — Category (Mod Health dimension) | **Adopted**, contingent on Item 9's naming constraint | Closed set already frozen in `Glossary.md`; ordering dependency on Item 9, not a substantive blocker. |
| 3 — Optional Recommendation cardinality | **Adopted** | Internal to `modiq-rules`; no Runtime lifecycle involvement. |
| 4 — Evidence anatomy: Label/Source/Content | **Adopted** | Extends the existing Collector Contract without apparent conflict. |
| 4 — Evidence anatomy: Explanation | **Requires Additional Investigation** | The Architectural Observation (§3) names a genuine subsystem-responsibility and construction-timing question without answering it. Evidence-incomplete, not an authority conflict — "Requires Governance Reconciliation" does not fit. |
| 5 — RepairRecipe structured steps | **Adopted** | `Glossary.md` already specifies concrete change-kind content; entirely internal to `modiq-knowledge`. |
| 6a — Report Identity | **Adopted**: `AssessmentReport` shall carry a Subject-identity field, as part of its domain model. | Product Design's Report Identity requirement; no conflict found with existing `AssessmentReport` structure. |
| 6a — Implementation Prerequisite | **Adopted Architectural Constraint**: the Subject-identity field shall not be implemented until `AssessmentSubject` carries content sufficient to populate it. Whether that requires reconsidering GOV-002 is not decided here (§4). | `AssessmentSubject` is a zero-field marker today; the field cannot be meaningfully constructed against it. |
| 7 — GOV-013 | Not dispositioned here | Remains a standing Governance Register item; this Resolution does not act on Governance Register entries. |
| 8 — Glossary "Recommendation" entry | Not dispositioned here | Product Owner documentation action, outside this Resolution's authority. |
| 9 — Naming collision | **Adopted Architectural Constraint**: the new Finding-level Mod Health concept shall not be named `Category`, to preserve `EvidenceCategory`'s existing meaning without collision. No specific replacement identifier is adopted — the Evaluation confirmed the collision but did not evaluate candidate identifiers, and selecting one is left to Implementation Authorization. | Collision confirmed directly against `Glossary.md`; no candidate identifiers were evaluated. |
| 10 — Provisional/Final (3B) | **Adopted**: Independent of Initiative 2's Question 1; proceeds as an ordinary constructor-populated `Finding` field. | ADR-0007's own scope boundary (aggregate transition rules vs. entity-level constructor-only pattern); no mutation method exists on `Finding` under either lifecycle direction. |

## §3 — Architectural Observation Acknowledged, Not Resolved

The Architectural Observation on Evidence's Explanation field is acknowledged as valid and standing. It is **not** converted into a Governance Register item by this Resolution — that action, if taken, belongs to Chief Architect judgment, consistent with how Initiative 5's Governance Observation was handled.

## §4 — GOV-002 Named, Not Reopened

Item 6a's Implementation Prerequisite connects directly to GOV-002's own prior "require no action" disposition on `AssessmentSubject`'s zero-field state. This Resolution does not reopen GOV-002, does not alter its disposition, and does not itself decide whether the newly adopted Report Identity requirement is sufficient grounds for reconsideration. That determination is named as available for Chief Architect action, exactly as Initiative 2's Documentation Authority gap was named without being resolved.

## §5 — What Is Not Adopted

No field representation, storage layout, serialization format, crate placement, or specific identifier (beyond the exclusion of `Category` in Item 9) is adopted for any item — only the existence of each field and, where the Evaluation supports it, its authoring subsystem.

## §6 — Explicitly Deferred

3C's two items (Report currency, the Updated marker) remain deferred pending Initiative 2's Governance Reconciliation — untouched by this Resolution.

## §7 — Explicit Non-Actions

No ADR was created. No Governance Register entry was opened or modified. No crate boundary was modified. GOV-002, GOV-013, and the Glossary's missing "Recommendation" entry are named but not acted on. No implementation was authorized.

## §8 — Summary Table

| Item | Outcome |
|---|---|
| 1, 3, 4 (Label/Source/Content), 5, 10 | Adopted, unblocked |
| 2 | Adopted, blocked on Item 9's constraint only (identifier, not existence) |
| 6a (field) | Adopted, blocked on Implementation Prerequisite |
| 6a (prerequisite), 9 | Adopted Architectural Constraint |
| 4 (Explanation) | Requires Additional Investigation |
| 7, 8 | Named, not dispositioned |
| 6b, 12 (3C) | Deferred |

## §9 — Readiness

Items 1, 3, 4 (Label/Source/Content), 5, and 10 are ready for Implementation Authorization once issued. Item 2 is ready once Item 9's constraint is satisfied by a chosen identifier (a naming choice, not further evaluation). Item 6a's field is adopted but not ready — blocked on its Implementation Prerequisite. Item 4's Explanation, Item 7, and Item 8 are not ready.

---

No ADR, crate, API, or implementation technology has been created, designed, or chosen. This Resolution's responsibility ends here, awaiting Chief Architect review.

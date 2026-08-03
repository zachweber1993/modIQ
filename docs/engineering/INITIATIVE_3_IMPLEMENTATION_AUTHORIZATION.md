# Initiative 3 Implementation Authorization — Domain Model Anatomy Extension

| Property | Value |
|---|---|
| **Document** | INITIATIVE_3_IMPLEMENTATION_AUTHORIZATION.md |
| **Project** | modIQ |
| **Origin** | `INITIATIVE_3_ARCHITECTURE_EVALUATION.md`, `INITIATIVE_3_ARCHITECTURAL_RESOLUTION.md` — both treated as fixed, unreopened repository architecture. |
| **Status** | **Approved. Authorizes implementation within the scope defined below. No implementation has begun in preparing this document; no code, crate, or test has been written.** |

---

## 1. Authorization Decision

**Authorized with Scope Constraint.** Implementation is authorized for exactly six items: 1, 2, 3, 4 (Label/Source/Content only), 5, and 10. No other Initiative 3 item is authorized. This determination is not newly reasoned here — it is the Resolution's own §9 Readiness accounting, converted into binding form, with one additional decision (Item 2's identifier) this document makes because the Resolution explicitly reserved it for Implementation Authorization.

## 2. Authorized Scope

| Item | What is authorized | Adopted basis |
|---|---|---|
| **1** | `Finding` gains `title`/`summary`, replacing the flat `description` | Resolution §2: Adopted, no conflict |
| **2** | `Finding` gains a `mod_health_dimension: ModHealthDimension` field; `ModHealthDimension` is a new closed-set type with exactly six variants — Compatibility, Stability, Maintainability, Performance, Structure, EngineeringQuality — matching `Glossary.md`'s Mod Health entry exactly | Resolution §2: Adopted, contingent on Item 9; naming resolved by this document (§3, below) |
| **3** | `RuleOutcome.recommendation` becomes `Option<Recommendation>` | Resolution §2: Adopted, internal to `modiq-rules` |
| **4 (Label/Source/Content only)** | `Evidence` gains `label`, `source`, `content` fields | Resolution §2: Adopted, extends the existing Collector Contract |
| **5** | `RepairRecipe.guidance: String` is replaced by a structured step representation, referencing the change-kinds `Glossary.md` already names (XML changes, Lua changes, Dependency installation, Asset replacement, Version updates) | Resolution §2: Adopted, internal to `modiq-knowledge` |
| **10** | `Finding` gains a Provisional/Final status field, populated at construction | Resolution §2: Adopted, Independent of Initiative 2's Question 1 |

No field representation beyond what each item's own Adopted disposition supports is authorized — matching the Resolution's own §5 ("What Is Not Adopted"): no storage layout, serialization format, or crate placement decision is made here beyond what is stated explicitly in this document.

## 3. Item 2 Naming Decision

`ModHealthDimension`, with variants `Compatibility`, `Stability`, `Maintainability`, `Performance`, `Structure`, `EngineeringQuality` — the six-value closed set `Glossary.md` freezes under "Mod Health," using its own literal word "dimensions." This satisfies Item 9's Adopted Architectural Constraint (GOVERNANCE.md/Resolution §2: "shall not be named `Category`") directly, and requires no further naming evaluation.

Verified directly against `INITIATIVE_3_ARCHITECTURAL_RESOLUTION.md` §2 and §9: the identifier decision is explicitly reserved for this document — "selecting one is left to Implementation Authorization" (Item 9 row); "Item 2 is ready once Item 9's constraint is satisfied by a chosen identifier (a naming choice, not further evaluation)" (§9).

## 4. Explicitly Excluded

Each excluded for a distinct, precise reason — none is a placeholder for "not yet gotten to":

- **Item 6a (Report Identity field).** Adopted, but not ready — blocked on its own Adopted Architectural Constraint (Implementation Prerequisite): `AssessmentSubject` must carry real content before this field can be meaningfully constructed. `AssessmentSubject` remains a zero-field marker struct today, confirmed unchanged. Implementing this field now would mean constructing it against nothing, violating the Resolution's own binding constraint.
- **Item 4 — Explanation.** Not Adopted at the field level — "Requires Additional Investigation." The Architectural Observation names a genuine, unanswered question (subsystem responsibility, construction timing, entity ownership) that this Authorization has no basis to resolve; doing so would mean the Authorization deciding what the Resolution explicitly declined to decide.
- **3C (Report currency, the Updated marker).** Deferred pending Initiative 2's own Governance Reconciliation of Question 1 — entirely outside Initiative 3's authority to release, regardless of this document's own scope.
- **Item 7 (GOV-013).** Not an Initiative 3 item at all — a standing, independently-tracked Governance Register item the Resolution explicitly declined to act on ("this Resolution does not act on Governance Register entries").
- **Item 8 (Glossary's missing "Recommendation" entry).** A Product Owner documentation action, outside this Authorization's engineering scope entirely.

## 5. Implementation Constraints

Binding on every item authorized above; none originates here — each traces to already-adopted architecture:

- **ADR-0007's Runtime Entity Design Pattern governs every field added to `Finding`, `Evidence`, and `RepairRecipe`.** Single deterministic fallible constructor; no builder; no post-construction setter. Every new field is populated exactly once, at construction — consistent with how every existing field on these types already works.
- **No mutation method may be added to `Finding`, `Evidence`, or `Recommendation`, under any of these items.** This is not a stylistic preference: Item 10's own Adopted basis depends directly on "no mutation method exists on `Finding` under either lifecycle direction" remaining true (Resolution §2, Item 10). Adding one — even for an unrelated reason, such as a convenience setter for Item 1 or Item 2 — would silently invalidate the determination that made Item 10 Independent of Initiative 2's Question 1.
- **`RuleOutcome`'s cardinality change (Item 3) stays entirely internal to `modiq-rules`.** It must not touch `AssessmentService`'s public entry points or any Runtime type.
- **`RepairRecipe`'s structured-step shape (Item 5) stays entirely internal to `modiq-knowledge`.** It must not introduce a dependency on `modiq-rules` or `modiq-runtime` beyond what already exists.
- **No Crate Boundary Rule requires modification.** Verified directly against `GOVERNANCE.md`: Runtime Domain's, Rule Engine's, and Knowledge Domain's own "Owns"/"Must never" lists already cover everything these six items add — none introduces a new responsibility, a new crate, or a new dependency edge.
- **No Governance Register item may be opened, modified, or implicitly resolved by this implementation** — consistent with `GOVERNANCE.md`'s own Principle 4 ("Implementation does not redefine architecture"). This is directly relevant to Item 2: assigning a `ModHealthDimension` value to an existing Rule's Finding is a field-population decision, not a re-litigation of GOV-013 (Severity/Kind conflation), which remains untouched and unaffected.

## 6. Implementation Sequencing

Items 1, 2, and 10 all modify `Finding`'s own constructor signature; Item 3 modifies `RuleOutcome`'s. No item depends architecturally on another — all six were independently Adopted — but Items 1, 2, and 10 share a single construction site (`Finding::new`) and, downstream, every existing Rule's own construction call (`EvidencePresenceRule`, `StructuralDuplicationRule`, `VersionCompatibilityRule`, `RuntimeLoadFailureRule`). Implementing them in the same pass avoids three separate breaking-signature changes to the same call sites; implementing them separately is not architecturally prohibited, only less efficient. This is a sequencing observation, not a Sprint plan — how many phases, and in what order, remains Sprint Planning's own work.

## 7. Governance Dependencies

**None.** This is what distinguishes the six authorized items from all five excluded ones: none requires a further governance action, ADR, or Chief Architect decision before implementation may proceed. Item 9's own Adopted Architectural Constraint is satisfied by §3, above.

## 8. Testing Expectations (per repository precedent)

Restated from this repository's own consistent practice, not newly invented:

- Every new or changed constructor requires direct, real-data unit tests — no mocking, consistent with this repository's standing discipline since Sprint 3 Phase 5.
- Any new validation rule (e.g., non-empty checks on `title`/`summary`, mirroring `FindingError::EmptyDescription`'s existing pattern) requires its own rejection test, matching how every existing constructor validation in `modiq-runtime` is tested today.
- Item 10 requires a test confirming the Provisional/Final field is populated only at construction and has no mutation path — the same kind of direct proof `Assessment::complete()`'s own "no further transformation" claim relies on (GOV-001's cited test, `generate_after_completion_matches_generate_before_completion`).
- Item 3 requires a test confirming `RuleOutcome` constructs correctly with `recommendation: None`, and that existing consumers of `RuleOutcome` handle that case without panicking.
- Full-workspace `cargo fmt`, `cargo check`, and `cargo test` must remain clean, per this repository's standing per-task requirement — the same bar every prior implementation has been held to.

No new testing framework, strategy, or convention is authorized or required beyond what already governs every other crate touched here.

## 9. Implementation Risks

- **Item 3 has the widest blast radius of the six.** `RuleOutcome.recommendation` becoming `Option<Recommendation>` is not confined to `modiq-rules` in its downstream effect: `Recommendation` content flows through `AssessmentReport` to every real consumer (`modiq-cli`'s `format_report`, `apps/sandbox`, `apps/console`'s `ReportSummary`). Each already handles a Finding with no Recommendation at the presentation layer (confirmed: `FindingSummary.recommendation: Option<String>` in `assessment.rs`, `recommendation: recommendation_for(finding.id())` returning `Option<String>`), so no consumer requires *new* handling — but this should be verified per consumer during implementation, not assumed from this Authorization alone.
- **Item 2 is a substantive per-Rule decision, not a mechanical field addition.** Each of the four existing Rules must be assigned a `ModHealthDimension` value deliberately, matching what that Rule actually evaluates — a real engineering judgment call this Authorization does not and cannot make in advance.
- **Item 4 requires a per-Collector decision.** Each existing Collector (filesystem, archive, XML, runtime-log) must decide what, if anything, to populate for `label`/`source`/`content` — not a uniform default applicable to all four.
- **Item 5 requires migrating the one existing authored recipe** (`RepairRecipe::version_compatibility_declared_version_mismatch()`, Sprint 9) to the new structured shape without altering its own referenced content or meaning.
- **Items 1, 2, and 10 together constitute a breaking constructor-signature change** to `Finding::new`, requiring every existing call site to be updated in the same pass — compiler-caught, not a silent-correctness risk, but real engineering surface area.

## 10. Repository Impact (anticipated, not authorized as final)

`modiq-runtime` (`Finding`: Items 1, 2, 10; `Evidence`: Item 4); `modiq-rules` (`RuleOutcome`: Item 3; every existing Rule's own construction call site: Items 1, 2, 3, 10); `modiq-knowledge` (`RepairRecipe`: Item 5). No new crate. No new external dependency. No change to `AssessmentService`'s public entry points.

## 11. Explicit Non-Actions

No ADR created. No Governance Register entry opened or modified. No Crate Boundary Rule changed. No implementation begun. No Sprint Plan produced — sequencing beyond §6's observation, phasing, and task breakdown remain Sprint Planning's own work, not this document's.

---

## Status

This document defines the engineering envelope for Items 1, 2, 3, 4 (Label/Source/Content), 5, and 10, and authorizes implementation within it. It does not authorize Item 6a, Item 4's Explanation, 3C, Item 7, or Item 8. No code has been produced in preparing this document.

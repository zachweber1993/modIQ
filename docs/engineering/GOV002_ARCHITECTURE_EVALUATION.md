# GOV-002 Architecture Evaluation — Runtime Invariant Reconciliation

| Property | Value |
|---|---|
| **Document** | GOV002_ARCHITECTURE_EVALUATION.md |
| **Project** | modIQ |
| **Governance item addressed** | GOV-002 (`docs/engineering/GOVERNANCE.md`) — "Runtime Invariant Reconciliation," Open since Engineering Release v0.1.0-alpha, 14 Sprints unresolved, never previously evaluated |
| **Purpose** | Evaluate whether the repository's Runtime documentation — `RuntimeInvariants.md` and the broader Runtime architecture surrounding it — faithfully describes the current Runtime architecture after fourteen completed Sprints |
| **Origin** | Chief Architect authorization, Sprint 15 |
| **Status** | **Architecture Evaluation only. No Architectural Resolution has been performed. No Implementation Authorization, Sprint Plan, or repository file modification of any kind has occurred.** |

---

## 1. Executive Summary

GOV-002 was raised at the platform's first Engineering Release with a narrow claim: "Implementation enforces lifecycle behaviors that are not yet fully represented within `RuntimeInvariants.md`." This evaluation examined that claim against the Runtime architecture as a whole — not `RuntimeInvariants.md` in isolation — per this session's own instruction not to assume it is the sole source of truth.

**Most of what first appears to be a gap in `RuntimeInvariants.md` is, on closer inspection, documented elsewhere** — principally in ADR-0007 (Runtime Entity Design Pattern), which captures constructor validation, identity-based equality, and the content-not-identity determinism convention that `RuntimeInvariants.md` itself does not mention. Read collectively, the Runtime architecture's documentation is more complete than `RuntimeInvariants.md` alone suggests.

**However, this evaluation found three genuine findings that survive that broader reading**, one of which is a real drift (a stale ADR claim contradicted by the platform's own later-evolved architecture), not merely an absence of documentation:

1. **No document, anywhere in the repository, presents the Assessment's actual four-state lifecycle machine (`Created` → `CollectingEvidence` → `EvaluatingRules` → `Completed`) or reconciles it against the different, higher-level lifecycle vocabularies used elsewhere** — `DataModel.md`'s seven-step conceptual lifecycle and `Architecture.md`'s eight-step pipeline diagram each use their own terms, and none of the three vocabularies has ever been explicitly cross-mapped against the other two or against the actual code.
2. **`INV-002`'s own wording is looser than the behavior it is cited to describe.** The invariant permits evidence to be added "before rule evaluation begins" — read plainly, any time from `Created` onward — but the implementation accepts `add_evidence` only in the specific `CollectingEvidence` state, rejecting it in `Created` too.
3. **ADR-0003 (Accepted, still cited as authoritative) lists `AssessmentReport` as an entity owned by the `Assessment` aggregate root, alongside Evidence, Finding, and Recommendation.** This contradicts the platform's own, later-established Reporting Crate Boundary Rule (`GOVERNANCE.md`): `AssessmentReport` is owned by a separate crate, `modiq-report`, generated externally from `Assessment`'s own public getters, never held or mutated by `Assessment` itself. This drift has never been reconciled — it is even repeated, unexamined, in `PROJECT_HANDOFF_v1.1.md`'s own current text.

A fourth observation — `AssessmentSubject` and `AssessmentContext` are empty marker structs in the actual implementation, while `DataModel.md` describes both with substantial conceptual content — is recorded as a finding but assessed as lower-stakes: it is a content-richness gap, not an invariant or lifecycle-fidelity question, and no document claims otherwise.

---

## 2. Architectural Question

Does the repository's Runtime documentation — `RuntimeInvariants.md` specifically, and the Runtime architecture documentation set as a whole (`DataModel.md`, `Architecture.md`, ADR-0002/0003/0007, `AssessmentCreation.md`) — faithfully and consistently describe the Runtime architecture as actually implemented in `modiq-runtime` today, after fourteen Sprints of accumulated implementation?

This is the question GOV-002 raised in miniature (RuntimeInvariants.md specifically); this evaluation answers the broader version per this session's own scope.

---

## 3. Repository Evidence

Verified directly this session, not carried forward from any prior evaluation:

- **`docs/implementation/RuntimeInvariants.md`** — read in full. Fourteen invariants (INV-001 through INV-014, with gaps in the visible numbering where INV-005/006 relate across sections), organized into three sections: Assessment Lifecycle (INV-001–005, 010–012), Aggregate Ownership (INV-006–009), Entity Reference Requirements (INV-013–014).
- **`docs/architecture/DataModel.md`** (v1.1.0, Frozen) — read in full, including its own "Runtime Lifecycle" diagram (a seven-step conceptual sequence) and its entity definitions for Assessment, Assessment Subject, Assessment Context, Evidence, Finding, Recommendation, Assessment Report, and Version Profile.
- **`docs/architecture/Architecture.md`** — its own "Assessment Lifecycle" diagram (an eight-step pipeline: Assessment Request → Assessment Service → Version Profile Selection → Evidence Collection → Rule Engine → Knowledge Base → Finding Generation → Assessment Report), and its Documentation Release 2.1 amendment note (Evidence Collection ordering corrected against `DataModel.md`, per ADR-0008).
- **`docs/adrs/0003-assessment-aggregate-root.md`** — read in full. Lists `AssessmentReport` as an Assessment-owned entity example.
- **`docs/adrs/0007-runtime-entity-design-pattern.md`** — read in full. Documents Entity Identity (process-local monotonic counter), Constructor Validation (rejects empty/whitespace descriptive text), Identity-Based Equality, and "Determinism is judged by content, not by incidental identity" as an authoritative, Accepted architectural decision — not merely informal prose.
- **`docs/implementation/AssessmentCreation.md`** and **`Sprint1-Implementation.md`** — both read in full; both match the implementation's actual creation contract (unique id, `Created` state, empty collections) without addressing Subject/Context content.
- **`crates/modiq-runtime/src/assessment/assessment.rs`** — read in full, including its complete test suite (65+ tests covering every lifecycle transition, mutation guard, and relationship-resolution method).
- **`crates/modiq-runtime/src/assessment/assessment_status.rs`, `evidence.rs`, `finding.rs`, `recommendation.rs`, `subject.rs`, `context.rs`, `assessment_error.rs`, and the three `*_id.rs` files** — read directly.
- **`crates/modiq-engine/src/engine/assessment_service.rs`** — greped directly to trace the real, current lifecycle sequence: `begin_evidence_collection()` → `add_evidence()` per item → `begin_rule_evaluation()` → `add_finding()`/`add_recommendation()` per Rule outcome → `AssessmentReport::generate(&assessment)` → `complete()`.
- **`docs/engineering/GOVERNANCE.md`** — GOV-002's own entry (Open, "Pending" per this Sprint's own prior reconciliation of a stale citation), and GOV-010's Resolution text, which already reasons explicitly about `RuntimeInvariants.md`'s own scope boundary (quoted in Section 6 below).
- **`docs/engineering/ENGINEERING_LOG.md`** — grepped for every historical mention of GOV-002 and of `AssessmentContext`/`AssessmentSubject`/ADR-0003; confirmed GOV-002 has never received a dedicated evaluation, and the ADR-0003/`AssessmentReport` question has never been raised.
- **`docs/engineering/PROJECT_HANDOFF_v1.1.md`** — confirmed it currently repeats the same imprecise ADR-0003 framing (`AssessmentReport` grouped with Runtime Domain entities) without flagging it.
- **`docs/architecture/RuleEngine.md`** — confirmed "Recommendation Generation" is a named, explicit Rule Engine responsibility, which `Architecture.md`'s own pipeline diagram omits as a stage.
- **`docs/implementation/Glossary.md`** — spot-checked Assessment, Assessment Subject, Assessment Report, Evidence, Finding definitions; found consistent with `DataModel.md` at the glossary's own terse level of detail, no contradiction.

---

## 4. Current Architecture

**The actual, implemented Assessment lifecycle is a four-state machine**, enforced entirely within `modiq-runtime`'s `Assessment` aggregate:

```
Created ──begin_evidence_collection()──▶ CollectingEvidence
                                              │
                                    begin_rule_evaluation()
                                              │
                                              ▼
                                       EvaluatingRules ──complete()──▶ Completed
```

- `add_evidence` succeeds only while `status == CollectingEvidence`.
- `add_finding` and `add_recommendation` both succeed only while `status == EvaluatingRules` — there is no separate state between "Findings Produced" and "Recommendations Generated"; both occur within the same `EvaluatingRules` window, and nothing enforces an ordering between them beyond `add_recommendation`'s own content check (at least one Finding must already exist).
- Every transition method funnels through a single private `transition()` function that unconditionally rejects any call once `status == Completed` (INV-012), and otherwise requires the exact `required` predecessor state (INV-010, INV-011) — this is the sole enforcement point for sequential, non-backwards transitions.
- `AssessmentSubject` and `AssessmentContext` are both zero-field marker structs, supplied at construction and never mutated.
- `AssessmentId`/`EvidenceId`/`FindingId`/`RecommendationId` are process-local, monotonic-counter-generated identities (confirmed identical mechanism across all four), restarting at 1 each process invocation (established finding from Sprint 13's own Storage work) — a fact `RuntimeInvariants.md` does not address at all, though ADR-0007 does (Entity Identity, Identity-Based Equality).
- `Evidence::new`/`with_location`, `Finding::new`, and `Recommendation::new` each reject empty or whitespace-only descriptive text via a dedicated error type — documented in ADR-0007's Constructor Validation section, not in `RuntimeInvariants.md`.
- **`AssessmentReport` is generated by a separate crate, `modiq-report`, calling `AssessmentReport::generate(&assessment)` against `Assessment`'s own public getters — confirmed directly in `assessment_service.rs` — while `Assessment.status` is still `EvaluatingRules`, before `complete()` is called.** `Assessment` itself never constructs, holds, or references an `AssessmentReport`.

---

## 5. Evaluation of the Existing Runtime Invariants

Each of the fourteen invariants was checked directly against the implementation and its test suite:

| Invariant | Conformance | Note |
|---|---|---|
| INV-001 (begins in Created) | **Conforms** | `Assessment::new` sets `status: AssessmentStatus::Created`; tested directly. |
| INV-002 (Evidence added before rule evaluation) | **Conforms, but imprecisely worded** | See Section 6, Finding 2 — the implementation is stricter than the invariant's own text describes. |
| INV-003 (Evidence immutable once evaluation starts) | **Conforms** | `add_evidence` unconditionally rejects any call once `status != CollectingEvidence`; tested directly (`add_evidence_rejects_once_rule_evaluation_has_started`, `evidence_is_fully_available_and_unchanged_throughout_evaluation`). |
| INV-004 (Findings only from deterministic rule evaluation) | **Conforms** | `add_finding` only succeeds during `EvaluatingRules`; determinism itself is a Rule Engine property (GOV-012), outside `modiq-runtime`'s own enforcement, correctly so. |
| INV-005 (Recommendations require Findings) | **Conforms** | `add_recommendation` explicitly checks `self.findings.is_empty()`; tested directly. |
| INV-006 (Assessment sole owner of runtime state) | **Conforms** | All mutation is via `Assessment`'s own methods; no child entity mutates itself or the aggregate. |
| INV-007 (Evidence/Finding/Recommendation mutated only through Assessment) | **Conforms** | No `pub` mutation method exists on any of the three types themselves. |
| INV-008 (Assessment enforces all lifecycle invariants) | **Conforms** | Every mutation method checks lifecycle validity before mutating; verified by exhaustive rejection tests. |
| INV-009 (no external direct mutation of Assessment-owned collections) | **Conforms** | `evidence()`, `findings()`, `recommendations()` return immutable slices (`&[T]`); no public mutable accessor exists. |
| INV-010 (sequential transitions) | **Conforms** | Enforced by the single `transition()` function's `required` check. |
| INV-011 (no backwards transition) | **Conforms** | Tested directly (`begin_evidence_collection_rejects_backwards_transition`). |
| INV-012 (Completed rejects further transitions) | **Conforms** | `transition()`'s own first check; `add_evidence`/`add_finding`/`add_recommendation` each separately check it too, redundantly but consistently. |
| INV-013 (Finding references ≥1 Evidence, cardinality only) | **Conforms, scope as documented** | `Finding::new` rejects empty `evidence_ids`; referential integrity (GOV-005 follow-up) remains explicitly, deliberately unenforced, exactly as INV-013's own text states. |
| INV-014 (Recommendation references ≥1 Finding, cardinality only) | **Conforms, scope as documented** | Identical pattern to INV-013; GOV-006 follow-up remains open, as documented. |

**No invariant was found to be violated by the implementation.** Every one of the fourteen has a direct, passing test enforcing it. The findings in Section 6 are about what is *undocumented or imprecisely documented*, not about any invariant the implementation fails to honor.

---

## 6. Findings

Organized by severity, per this project's own standing distinction between documentation drift and a genuine architectural inconsistency.

### Finding 1 (most significant) — No document reconciles the Assessment's actual state machine against the platform's other Runtime lifecycle vocabularies

Three genuinely different lifecycle descriptions coexist, none cross-referenced against the others:

- **`RuntimeInvariants.md`** never names `CollectingEvidence` or `EvaluatingRules` as states, and never presents a transition diagram — only prose rules referencing "Created," "rule evaluation begins/starts," and "backwards."
- **`DataModel.md`'s "Runtime Lifecycle"** (conceptual, entity-focused, 7 steps): *Assessment Created → Assessment Context Established → Evidence Collected → Findings Produced → Recommendations Generated → Assessment Report Produced → Assessment Completed.* Three of these seven steps do not correspond to any real state transition: "Assessment Context Established" is not a transition at all (context is supplied at construction); "Findings Produced" and "Recommendations Generated" are not two states but one (`EvaluatingRules`); "Assessment Report Produced" happens entirely outside `modiq-runtime`, in a different crate.
- **`Architecture.md`'s "Assessment Lifecycle"** (pipeline/subsystem-focused, 8 steps): *Assessment Request → Assessment Service → Version Profile Selection → Evidence Collection → Rule Engine → Knowledge Base → Finding Generation → Assessment Report.* This diagram omits Recommendation Generation as a stage entirely, despite `RuleEngine.md` naming it as an explicit Rule Engine responsibility, and despite `INV-005`/`INV-014` existing specifically to govern Recommendations.
- **The actual code**: four `AssessmentStatus` states (`Created`, `CollectingEvidence`, `EvaluatingRules`, `Completed`).

None of these four descriptions contradicts another in the sense of asserting incompatible facts — but none has ever been checked against the others, and a reader relying on any single one of the three documents would form an incomplete or imprecise picture of the actual state machine.

### Finding 2 — INV-002's wording is looser than the behavior it describes

INV-002 states: "Evidence MAY only be added before rule evaluation begins." Read on its own, this permits addition any time between `Created` and the start of evaluation. The actual implementation is stricter: `add_evidence` succeeds only while `status == CollectingEvidence` specifically, and is rejected in `Created` (confirmed directly: `add_evidence_rejects_before_evidence_collection_begins`). Reaching `CollectingEvidence` itself requires an explicit `begin_evidence_collection()` transition that INV-002's own text does not mention exists.

### Finding 3 — ADR-0003's `AssessmentReport` ownership claim contradicts the platform's own later-evolved crate architecture

ADR-0003 (Accepted, "authoritative for Documentation Release 1.0") lists `AssessmentReport` as an example of an Assessment-owned runtime entity, in the same list as Evidence, Findings, and Recommendations. This was accurate to the platform's single-crate-era design at the time ADR-0003 was written. It has not been true since the platform's crate-boundary architecture matured: `GOVERNANCE.md`'s Reporting Crate Boundary Rule assigns `AssessmentReport` to a separate crate, `modiq-report`, generated by `AssessmentReport::generate(&assessment)` — a pure function reading `Assessment`'s already-public getters — never held, referenced, or mutated by `Assessment` itself. Confirmed directly: `Assessment` has no field, method, or dependency touching `AssessmentReport` anywhere in `modiq-runtime`. **This drift has never been examined or flagged in this repository's history** — it is, in fact, still repeated without qualification in `PROJECT_HANDOFF_v1.1.md`'s own current text ("`Assessment` (the sole aggregate root, ADR-0003), `Evidence`, `Finding`, `Recommendation`, `AssessmentReport`. Fully real, fully implemented.").

### Finding 4 (lower stakes) — `AssessmentSubject` and `AssessmentContext` carry no content, while `DataModel.md` describes both richly

`DataModel.md` describes Assessment Subject with real examples (mods, mod collections, savegames, maps) and Assessment Context with real examples (Version Profile, timestamp, configuration, platform capabilities, execution environment) — and states directly that "Assessment Context ensures Assessment results remain reproducible." The actual `modiq-runtime` types are both zero-field unit structs; neither carries any of this content today, so the reproducibility claim is not something the type itself currently guarantees. This is not a lifecycle or invariant question, and no document claims otherwise — it reads as an unflagged placeholder, the same shape as `modiq-versioning`'s and `modiq-knowledge`'s own pre-Sprint-8/9 minimum-viable states, but — unlike those two — never explicitly named as deliberate anywhere this evaluation found.

### Finding 5 (context, not a gap) — Content-level invariants and the identity/determinism convention are documented, just not in `RuntimeInvariants.md`

ADR-0007 (Accepted) already documents: Entity Identity (a single, consistent, process-local monotonic-counter mechanism across all four id types); Constructor Validation (empty/whitespace-only descriptive text rejected at construction); Identity-Based Equality (two entities with identical content but different identity are not equal — confirmed directly by three dedicated tests, one per entity type); and "Determinism is judged by content, not by incidental identity." None of this is missing from the Runtime architecture's documentation as a whole — it is missing only from `RuntimeInvariants.md` specifically, which is a narrower claim than GOV-002's original framing might suggest.

### Finding 6 (context, precedent) — The repository has already reasoned once about `RuntimeInvariants.md`'s own scope boundary

GOV-010's own Resolution text (Collection Error Model) states directly: *"No `RuntimeInvariants.md` change was required: no Runtime aggregate invariant governs this — a collection failure means the Assessment's lifecycle simply never progresses far enough to produce a Report, which existing invariants (INV-002, INV-003) already accommodate without modification. This is an Engine-orchestration policy, not a Runtime state concern."* This is directly relevant precedent: the repository has already, once, explicitly decided that a class of lifecycle-adjacent behavior (Collection outcomes, Archive traversal termination, Rule dispatch order, Storage's write-once policy) belongs to its own subsystem's governance, not to `RuntimeInvariants.md`. This evaluation's own findings (1–4) are all genuinely about the Runtime domain itself — the Assessment aggregate's own state machine and entity content — not about a downstream subsystem's policy, so this precedent does not resolve them, but it does bound what kind of gap would even be relevant to GOV-002 in the first place.

---

## 7. Alternative Resolutions (Not Decided Here)

Presented for the Chief Architect's own Architectural Resolution, not selected by this document:

**Option A — Amend `RuntimeInvariants.md` to name the actual four-state machine explicitly**, adding the concrete state names and a transition diagram, and tightening INV-002's wording to match `add_evidence`'s actual precondition. Leaves `DataModel.md` and `Architecture.md`'s own diagrams as higher-level, intentionally different-grained descriptions, but would newly require deciding whether to cross-reference them from `RuntimeInvariants.md` or leave the reconciliation as this evaluation's own record.

**Option B — Reconcile all three lifecycle vocabularies against each other explicitly**, in whichever document is judged the right home for it (a candidate amendment to `DataModel.md`'s own Runtime Lifecycle section, or a new cross-reference table), including correcting `Architecture.md`'s omission of Recommendation Generation as a pipeline stage. Broader in scope than Option A; would touch a Frozen specification (`DataModel.md` and/or `Architecture.md`), requiring the amendment-recorded-explicitly discipline already established for both documents' prior amendments.

**Option C — Correct ADR-0003's `AssessmentReport` ownership claim** (Finding 3), either by a targeted ADR amendment (following the precedent of frozen documents being amended in place with the change recorded explicitly, e.g. `DataModel.md`'s own Sprint 5 amendment) or by a new, narrow ADR that supersedes ADR-0003's specific claim without touching its still-valid core decision (Assessment as aggregate root). Independent of Options A/B — this finding is about a stale claim, not about `RuntimeInvariants.md`'s own content.

**Option D — Resolve GOV-002 as satisfied by existing documentation, taken as a whole, with only Finding 3 (the ADR-0003 drift) requiring action.** This option treats Findings 1, 2, and 4 as acceptable, intentional differences in abstraction level rather than defects — the same judgment this evaluation itself leans toward for Finding 1 and 2, given that no invariant is actually violated (Section 5) and no reader has ever been misled into an incident by them, as far as this evaluation's evidence search found.

**Option E — Defer, as GOV-008 and GOV-013 have both been deferred**, on the grounds that none of these findings has yet caused a real, demonstrated problem — no test failure, no implementation defect, no user-facing inconsistency traces to any of them. This would be the same evidence-based restraint this project has applied elsewhere, at the cost of leaving GOV-002 open for a fifteenth Sprint with, for the first time, a fully-evidenced record of exactly what would need to change if it is ever acted on.

These options are not mutually exclusive — Option C (the one finding with genuine, if narrow, architectural drift) could be paired with a Deferral (Option E) or a Resolved-as-satisfied disposition (Option D) for Findings 1/2/4, or a full Option B rewrite could subsume all of them at once.

---

## 8. Chief Architect Recommendation

**Engineering's own recommendation** (offered per this project's standing pattern — drafted here, decided by the Chief Architect, never the reverse):

- **Finding 3 (ADR-0003's `AssessmentReport` claim) is the one item in this evaluation that rises above documentation drift** — it is a specific, checkable factual claim in an Accepted ADR that the platform's own later architecture has contradicted for many Sprints, unexamined. This is the strongest candidate for actual correction, by whichever mechanism the Chief Architect judges proportionate (Option C).
- **Findings 1, 2, and 4 read as documentation-fidelity gaps, not architectural defects** — no invariant is violated (Section 5), and no incident or confusion traces to any of them in this repository's own history. Whether they warrant correction now, deferral (mirroring GOV-008/GOV-013's own standing treatment), or being resolved as "existing documentation, read as a whole, already satisfies GOV-002's original question" (mirroring how GOV-001 was itself just resolved this Sprint) is a judgment call this evaluation does not make on the Chief Architect's behalf.

This evaluation recommends the Chief Architect treat GOV-002 as ripe for an Architectural Resolution decision, not further evaluation — the evidence set above is complete against every category this session's own scope named (RuntimeInvariants.md, lifecycle documentation, terminology, state transitions, guarantees, implementation, tests, architecture documentation, ADRs, engineering history).

**No Architectural Resolution has been performed. No Implementation Authorization, Sprint Plan, or file modification has occurred.** This evaluation's responsibility ends here, awaiting explicit Chief Architect authorization before the next stage of the governance workflow begins.

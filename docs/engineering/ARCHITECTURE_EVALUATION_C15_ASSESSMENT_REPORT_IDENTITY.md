# Architecture Evaluation — C15: Assessment & Report Identity

| Property | Value |
|---|---|
| **Document** | ARCHITECTURE_EVALUATION_C15_ASSESSMENT_REPORT_IDENTITY.md |
| **Project** | modIQ |
| **Type** | Architecture Evaluation only. No ADR, Architectural Resolution, Implementation Authorization, or implementation is created, designed, or chosen by this document. |
| **Origin** | `PROCESS_DETERMINATION_C15_ASSESSMENT_REPORT_IDENTITY.md` (working tree, uncommitted), Determination B — Architecture Evaluation required, scoped to §11's five questions. Treated as fixed evidence, not reopened; where this evaluation's own deeper inspection narrows or sharpens that document's framing, the difference is stated explicitly (§10), not silently substituted. |
| **Repository baseline** | `feature/runtime-implementation`, HEAD `60ef3af` (C14 Repository Closeout), working tree clean apart from the two prior C15 documents (untracked) |

---

## 1. Purpose

Determine whether the C15 capability — modIQ must be able to meaningfully identify, distinguish, and refer to an Assessment and its Report as a durable product concept — can be accommodated by existing, frozen architecture, or whether it requires a new or clarified architectural fact. The reversed burden of proof applies throughout: existing architecture is presumed sufficient unless concrete evidence, traced to primary sources, demonstrates otherwise. This document does not redesign architecture, select implementation content, or authorize any work.

---

## 2. Capability Under Evaluation

Unchanged from `CAPABILITY_DEFINITION_C15_ASSESSMENT_REPORT_IDENTITY.md` §1/§5: the capability for modIQ to durably answer, of any Assessment or Report it produces, *what is this an assessment of* — stated at the level of what becomes possible, not how it is realized. This evaluation does not re-derive or narrow that statement.

---

## 3. Architectural Baseline

Treated as fixed and authoritative throughout, per this mission's own instruction:

- Platform Architecture 1.0 (frozen).
- Runtime Architecture (validated against Platform Architecture).
- ADR-0003 (Assessment as the Aggregate Root, Accepted).
- ADR-0011 (`AssessmentReport` Ownership Correction, Accepted — supersedes ADR-0003 solely with respect to `AssessmentReport`'s own inclusion in the Assessment-owned list).
- ADR-0007 (Runtime Entity Design Pattern, Accepted).
- `GOVERNANCE.md`'s Runtime Domain, Reporting, and Storage Crate Boundary Rules.
- GOV-002 (Resolved).
- Initiative 2 (unresolved; Requires Governance Reconciliation — concerns exclusively same-Assessment supplementation, confirmed again in §9 to remain untouched by this evaluation).
- Initiative 3, Item 6a (Adopted-but-gated).

---

## 4. Evidence Reviewed

Every claim below was checked directly against current repository source or governance text this session, not carried forward from the Process Determination's own account.

**Governance / ADRs:**
- `docs/adrs/0003-assessment-aggregate-root.md`, read in full — original Decision text.
- `docs/adrs/0011-assessmentreport-ownership-correction.md`, read in full — the correcting text.
- `docs/adrs/0007-runtime-entity-design-pattern.md` — construct-once, identity-at-construction, and determinism principles.
- `docs/engineering/GOVERNANCE.md` — GOV-002's full entry (Description, Resolution); the Runtime Domain, Reporting, and Storage Crate Boundary Rule sections, read in full.
- `docs/engineering/INITIATIVE_3_ARCHITECTURAL_RESOLUTION.md` (Item 6a) and `INITIATIVE_3_ARCHITECTURE_EVALUATION.md` — the GOV-002 connection Initiative 3 itself already named.
- `docs/engineering/INITIATIVE_2_ARCHITECTURAL_RESOLUTION.md` — the definition of "Requires Governance Reconciliation," used here only to confirm C15's own governance question does not meet that bar (§9), not reopened.
- `docs/engineering/STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md` — `AssessmentId`'s documented non-durability.

**Implementation:**
- `crates/modiq-runtime/src/assessment/assessment.rs` — `Assessment`'s complete public method list, confirmed exhaustively: `id`, `subject`, `context`, `version_profile`, `status`, `is_evaluating`, `evidence`, `findings`, `recommendations`, `evidence_by_id`, `finding_by_id`, `evidence_for_finding`, `findings_for_recommendation`, `begin_evidence_collection`, `begin_rule_evaluation`, `complete`, `add_evidence`, `add_finding`, `add_recommendation`. **No method exists that updates, replaces, or otherwise mutates `subject` or `context` after construction.**
- `crates/modiq-runtime/src/assessment/{subject,context}.rs` — `AssessmentSubject`/`AssessmentContext`, zero-field unit structs.
- `crates/modiq-report/src/report/assessment_report.rs` — `AssessmentReport::generate(assessment: &Assessment)` and its complete getter list: `assessment_id`, `status`, `evidence`, `findings`, `recommendations`. **`subject()`/`context()` are not read or exposed anywhere in this file.**
- `crates/modiq-engine/src/engine/assessment_service.rs`, `AssessmentService::execute` — confirmed, exact call order: `Assessment::new(subject, context, version_profile_reference)` executes first; `begin_evidence_collection()` and the `add_evidence` loop execute strictly afterward. **`AssessmentSubject`'s content, whatever it is, is fixed before any Evidence exists.**
- `crates/modiq-storage/src/storage/persisted_report.rs` — confirmed (from prior direct inspection during C15 Process Determination and unchanged since): `PersistedAssessmentReport` mirrors `status`/`evidence`/`findings`/`recommendations` by value from `AssessmentReport`'s own public API; no Runtime identifier is itself persisted or required to be durable for this mirroring to work.

---

## 5. GOV-002 Revisit Analysis

**What GOV-002 evaluated:** whether `RuntimeInvariants.md`'s fourteen invariants conform to `modiq-runtime`'s actual implementation (its Description states this precisely: a Sprint 15 Architecture Evaluation "checking all fourteen invariants against `modiq-runtime`'s implementation and test suite").

**What `AssessmentSubject` state existed when evaluated:** the same zero-field state as today — confirmed directly; nothing about `AssessmentSubject`'s shape has changed since GOV-002 was resolved.

**What GOV-002 actually concluded, quoted directly:** *"`AssessmentSubject`/`AssessmentContext`'s minimal content... were both evaluated and require no action."* This finding was reached specifically while checking invariant conformance — it establishes that the zero-field state violates none of the fourteen invariants.

**What GOV-002 did NOT conclude:** it does not state that `AssessmentSubject` must remain empty, that no future capability may ever require it to carry content, or that its content is architecturally fixed for all purposes. Its own textual scope is invariant-conformance, nothing broader.

**Relationship to C15:** C15 is **a new requirement that did not exist when GOV-002 was evaluated**, not a contradiction of it and not an invalidation of any assumption it actually relied on. `AssessmentSubject` having zero fields was, and remains, fully invariant-conformant; C15 does not dispute that. C15 asks a different, additional question GOV-002 was never asked: whether a real product need now justifies giving that entity real content.

**Determination: C — GOV-002 remains valid as written, AND an additional architectural fact is required.** These are compatible because they answer non-overlapping questions: GOV-002's own question (does the current, empty state violate any invariant) remains answered "no," permanently — nothing here disturbs that. The additional architectural fact required (§7, §8) is *how* `AssessmentSubject` can acquire content at all, given a separate, already-existing constraint (construct-once timing, ADR-0007) that GOV-002 never had occasion to examine because no one had yet proposed populating the field. Populating a Runtime entity field that was always structurally permitted to be non-empty, but happened to be empty, is the same ordinary evolution ADR-0007's own pattern already anticipates and has already been exercised five times for other fields — it is not a correction of GOV-002, and nothing here reopens, amends, or supersedes it.

---

## 6. Domain Ownership Analysis

**Finding: this question is not independently open. Existing architecture already, explicitly, settles it.**

`ADR-0003` designates `Assessment` as the Runtime domain's sole aggregate root and explicitly lists `Assessment Subject` among Assessment-owned entities. `ADR-0011` — a direct, later correction to `ADR-0003`'s own example list — supersedes that list **solely with respect to `Assessment Report`**, stating in its own Decision text: *"`Assessment Context`, `Assessment Subject`, `Evidence`, `Findings`, and `Recommendations` remain accurately described as Assessment-owned entities exactly as ADR-0003 states."* `AssessmentSubject` was directly, deliberately re-affirmed as Assessment-owned at the exact moment `AssessmentReport` was corrected out of that list — this is not silence on the question; it is an explicit reaffirmation, primary-sourced.

`AssessmentReport`'s own role, per `GOVERNANCE.md`'s Reporting Crate Boundary Rule, is stated with unusual, directly on-point explicitness: *"Reports are reflections of Assessment state. They never create new state."* `AssessmentReport::generate` already reads three of `Assessment`'s own public getters (`evidence()`, `findings()`, `recommendations()`) by value, converting Runtime state into Reporting-owned output without owning or mutating any of it. `Assessment::subject()` is an already-public getter of the identical shape, simply not yet read by `AssessmentReport::generate` — confirmed directly (§4): no code change to `Assessment` itself would be required to expose it; only `AssessmentReport::generate`'s own body would need to read one more already-public getter, exactly as it already does for three others.

`modiq-storage`'s own boundary text independently corroborates the same layering: its persisted representation is "populated only from `AssessmentReport`'s already-public API" — Storage may mirror what Reporting exposes; it may not originate a fact Reporting does not already have.

**Conclusion:** if subject identity exists as Runtime content at all, `AssessmentSubject` is its architecturally correct origin — not `AssessmentReport`, and not `modiq-storage` — by direct textual designation (ADR-0003/0011), not by inference or default. `AssessmentReport`'s role would be to *present* that already-owned fact, exactly as it already presents Evidence/Findings/Recommendations; `modiq-storage`'s role would be to *mirror* whatever `AssessmentReport` presents. This is not a genuinely open architectural question — it resolves cleanly from primary sources already in force, and the Process Determination's own broader framing of this specific point (§6 of that document) is narrower than a closer reading supports.

---

## 7. Cross-Process Durability / Storage Boundary Analysis

**Finding: not independently open, for identity realized as ordinary mirrored content.**

`AssessmentId`'s non-durability (confirmed, `STORAGE_PERSISTENCE_REPRESENTATION_DESIGN_NOTE.md`: its generator restarts at 1 every process invocation, by design) governs **identity-as-reference** — a process-local pointer used for in-memory relationship resolution (`evidence_by_id`, `finding_by_id`, and the like). It has never governed **identity-as-content** — the actual field values `Evidence`, `Finding`, and `Recommendation` carry, which already cross the process boundary durably today, via `modiq-storage`'s own Mirror pattern (already demonstrated three times: Sprint 13's founding, Sprint 22's field extension, Sprint 24's structured repair steps), addressed not by any Runtime identifier but by `ReportKey` — a durability mechanism Storage mints and owns entirely independently of `AssessmentId`.

A value carried on `AssessmentSubject` (whatever it eventually is) would cross the process boundary by exactly this same, already-proven mechanism — copied by value into a new field on `PersistedAssessmentReport`'s own representation, retrievable via the already-durable `ReportKey`, with zero dependency on `AssessmentId` ever becoming durable itself. Initiative 2's own Architecture Evaluation treated `AssessmentId`'s non-durability as one of two barriers **specifically to reconstructing a live, mutable `Assessment` from persisted data** — a different question (reopening a completed Assessment) from persisting a Report's own descriptive content once, at the moment it is first generated, which every other Runtime-originated field already does without issue.

**Conclusion:** cross-process durability of a purely descriptive, value-copied subject identity is already architecturally solved by the existing Mirror pattern. It would become a genuinely open question only if the capability required *matching/equality semantics* across separately persisted records (e.g., reliably recognizing two Reports as concerning "the same" mod despite differing input) — but `CAPABILITY_DEFINITION_C15...md` §8 states this only "in principle," and §7/§10 explicitly exclude designing same-mod trends or cross-mod comparison here. For C15's own stated acceptance criteria, no new durability mechanism is required.

---

## 8. Architectural Approach Evaluation

Each of the Process Determination's four named approaches, evaluated against the ten factors requested, using the findings above.

### Approach A — Identity upstream of Storage (Runtime/Reporting)

Relies on `ADR-0003`/`ADR-0011`'s own designation and `AssessmentReport`'s already-established getter-reading mechanism. Preserves every existing boundary rule without exception. Pressures none. Introduces no new architectural fact for content already known to the caller at submission time; surfaces the one genuinely open question (§10) if content must be Evidence-derived. Creates no ownership ambiguity — `AssessmentSubject` is already the designated owner. Fully compatible with `AssessmentId`'s documented semantics (orthogonal — untouched). Fully compatible with the Storage Crate Boundary Rule (Storage plays no role at this stage). No unnecessary downstream coupling. Strong repository precedent (identical in shape to Sprint 22/24's five prior field extensions). **Evidence supports this approach for submission-time-known content; leaves the timing question (§10) open for anything richer.**

### Approach B — Identity introduced at the persistence boundary (Storage originates the fact)

Relies on nothing existing — would require `modiq-storage` to originate a domain fact it does not receive from `AssessmentReport`. Preserves no boundary that isn't already satisfied more simply elsewhere. **Directly pressures the Storage Crate Boundary Rule's own explicit text** — "populated only from `AssessmentReport`'s already-public API" is not a preference; it is the boundary as written. Introduces a new architectural fact (Storage gaining content-origination responsibility explicitly outside its "Must never" list's spirit). Creates ownership ambiguity by contradicting the already-settled designation in §6. Not compatible with the Storage Crate Boundary Rule as written. No repository precedent supports it. **Evidence rejects this approach as a distinct path** — in the one form that would remain compatible with the existing boundary (Storage merely mirroring a value already present upstream), it is indistinguishable from Approach C, not a separate approach in its own right.

### Approach C — Identity flows Runtime → Reporting → Storage via the established Mirror pattern

Relies on `ADR-0003`/`ADR-0011` (§6) and the already-three-times-proven Mirror pattern (§7). Preserves every boundary rule exactly as written — this is the mechanism those rules already anticipate (`GOVERNANCE.md`'s own Storage text: "populated only from `AssessmentReport`'s already-public API," which is precisely what this approach does). Pressures nothing. Introduces no new architectural fact for submission-time-known content; carries forward the same open timing question as Approach A for anything Evidence-derived, since this approach's own Runtime-side origin point is identical to Approach A's. No ownership ambiguity. Fully compatible with `AssessmentId`'s documented semantics. Fully compatible with the Storage Crate Boundary Rule — this approach *is* that rule's own anticipated mechanism. No unnecessary coupling — reuses an already-proven mechanism rather than introducing a new one. The strongest repository precedent of the four. **Evidence directly supports this approach**, contingent on the same open question Approach A carries.

### Approach D — A wholly new, independent identity concept

Relies on nothing existing; would be a new architectural participant, subject to `INV-002_PLATFORM_PERSISTENCE_CAPABILITY.md`'s own Capability Identity classification procedure before it could even be authorized as such. Does not leverage `AssessmentSubject`'s own already-designated role (§6) — raises, without answering, the question of why a purpose-built, already-existing, currently-dormant concept would be insufficient. Introduces a new architectural fact and a new participant simultaneously. Creates ownership ambiguity by design — a second identity-bearing concept alongside an existing one nominally meant for exactly this purpose. Compatible with `AssessmentId`'s semantics only trivially (orthogonal, by virtue of being unrelated). Compatibility with the Storage boundary is undetermined, since the concept itself is undefined. Introduces a real risk of unnecessary downstream coupling (two identity-shaped concepts to reconcile later). **No repository precedent supports it** — this repository's own established pattern for a dormant-but-already-designated concept receiving its first real content is *Architectural Activation* (`SPRINT8_ARCHITECTURAL_RESOLUTION.md` §8, applied there to Version Profiles), which activates the existing concept rather than introducing a parallel one. **Evidence does not support this approach**; it is the weakest of the four.

---

## 9. Initiative 3 Item 6a Analysis

**Does C15 establish that `AssessmentSubject` must carry identity information?** C15 establishes that the *capability* is now forced (Process Determination §3, §11, citing C14's own real Hemp/Hashy evidence); it does not itself decide that `AssessmentSubject` specifically must be the vehicle — but §6, above, independently establishes that *if* Runtime-originated subject content is to exist at all, `AssessmentSubject` is its architecturally designated home, by ADR text, not by this document's own preference.

**Does C15 establish that `AssessmentSubject` is the architectural owner of that information?** Yes, to the extent identity content originates in the Runtime domain at all (§6) — this is a direct consequence of ADR-0003/ADR-0011 read together, not a new decision this evaluation makes.

**Does architecture establish only that Assessment must have meaningful subject identity, while exact `AssessmentSubject` content remains an implementation concern?** Partially, and precisely bounded: for content already known to the caller at the moment `AssessmentService::execute`/`execute_from_assessment_input` is invoked (e.g., the input path string already passed as an argument, confirmed present before `Assessment::new` is ever called), exact field selection is an ordinary Implementation Authorization-level decision, following the same pattern five prior fields already used without a dedicated Architecture Evaluation each. For content that can only be known after Evidence collection (e.g., a mod's own declared name, which only `XmlCollector`'s output could supply), this is **not** merely an implementation concern — no existing mechanism updates `AssessmentSubject` after `Assessment::new` (§4's exhaustive method-list check), `ADR-0007` establishes construct-once/no-later-mutation as the governing pattern for Runtime entities generally, and `GOVERNANCE.md`'s Reporting rule explicitly forbids the alternative of deriving it later, in Reporting, from Evidence ("Reports... never create new state").

**Is Item 6a now satisfied?** No. No content has been added to `AssessmentSubject`; this evaluation adds architectural clarity, not implementation.

**Does Item 6a remain gated?** Yes.

**If it remains gated, precisely what architectural fact is still missing?** Whether "sufficient content," in Initiative 3's own phrase, can be fully satisfied by submission-time-known information alone (in which case the gate is satisfiable at the ordinary Implementation Authorization level, with no further Architecture Evaluation), or whether it requires Evidence-derived information (in which case a currently-nonexistent mechanism — a new, deliberately scoped exception to construct-once immutability, a relocated construction moment, or some other approach not yet named — would first need its own Architectural Resolution). This document does not decide which; that is exactly the open question named in §10.

---

## 10. Architectural Findings

Stated once, plainly, as the synthesis of §5–§9:

**Two of the concerns this evaluation was charged with examining are, on direct primary-source inspection, already resolved by existing architecture — not genuinely open.** Domain ownership (§6) is settled by ADR-0003/ADR-0011's own explicit text. Cross-process durability for descriptive, value-copied content (§7) is already solved by the three-times-proven Mirror pattern, independent of `AssessmentId`'s own, differently-scoped non-durability. Neither required a new architectural fact; both required only reading existing, already-Accepted documents closely enough.

**One genuine architectural question survives this scrutiny, more narrowly and more precisely than the Process Determination's own framing stated it:**

> Can Assessment/Report subject identity be limited to information already known to the caller at the moment of Assessment construction (fully compatible with existing architecture, requiring no new mechanism), or does meaningfully satisfying C15 require information only knowable after Evidence collection — and if so, by what mechanism, given that `ADR-0007`'s construct-once pattern, `Assessment`'s own current method surface (no post-construction subject mutation exists), and Reporting's explicit "never create new state" rule together leave no existing path for Evidence-derived content to become part of a Report's own identity?

This satisfies Prong 1 (a real, unsolved principle — not "this is important," not "this touches multiple crates") and Prong 2 (two materially different resolutions with different architectural consequences: confine identity to submission-time content, entirely ordinary; or introduce a new mechanism for post-construction or Evidence-derived content, which does not exist today in any form).

---

## 11. Determination

**Outcome B — Architectural Resolution required**, narrowed from the Process Determination's own broader framing to the single question stated in §10.

This is not a rubber-stamp of the Process Determination's own expectation: two of its three named concerns (domain ownership, cross-process durability) are found here to already be resolved by existing architecture and are **not** carried forward as open. The reversed burden of proof was applied throughout — existing architecture was checked first, in each case, against the specific concern raised, and found sufficient in two of three cases.

---

## 12. Architectural Resolution Requirement

The next Architectural Resolution, if authorized, must decide:

1. Whether submission-time-known content (e.g., an input identifier already available before `Assessment::new` is called) is sufficient to satisfy C15's own stated Desired Capability and Acceptance Criteria (`CAPABILITY_DEFINITION_C15...md` §5, §8) — in which case `AssessmentSubject` may be extended additively, following the already-five-times-proven ADR-0007 field-extension pattern, with no further architectural work beyond ordinary Implementation Authorization.
2. If not sufficient, whether and how a mechanism should be introduced to permit Evidence-derived content to reach `AssessmentSubject`, `AssessmentReport`, or some other locus — explicitly including whether this constitutes a deliberate, narrow exception to `ADR-0007`'s construct-once principle, a relocation of when `AssessmentSubject`'s content is fixed, or a different mechanism not yet named — and how any such mechanism is reconciled with Reporting's own "never create new state" rule.
3. On the basis of (1) or (2), whether Initiative 3 Item 6a's "sufficient content" gate is thereby satisfied, and what if anything about GOV-002's own text (unchanged, still valid per §5) needs no further action as a result.

This document does not perform that Resolution, does not weigh (1) against (2), and does not speculate which the eventual Resolution should choose.

---

## 13. Explicit Non-Resolutions

This document does not:

- Decide what content `AssessmentSubject` (or any other type) should carry.
- Decide whether `ReportKey`, `PersistedAssessmentReport`, or any Storage schema changes.
- Decide the identifier type, format, or serialization of anything.
- Design console persistence, history, trends, or C5 — none is authorized, scoped, or implied as forthcoming.
- Design Confidence, Evidence Explanation, Savegame/User Observation evidence, Investigate/Diagnose/Remediate workflow, or Knowledge-layer activation — none is implicated by this evaluation.
- Resolve, reopen, or amend GOV-002 — confirmed in §5 to remain valid and untouched.
- Resolve or satisfy Initiative 3 Item 6a — confirmed in §9 to remain gated.
- Reopen, advance, reinterpret, or use as authorization Initiative 2 — confirmed throughout (§3, §7) to concern exclusively same-Assessment supplementation, categorically distinct from identifying independent Assessments, and untouched by any finding in this document.
- Create an ADR, open a Governance Register item, or perform any part of the Architectural Resolution it recommends.
- Authorize any implementation of any kind.

---

## 14. Next Lifecycle Stage

An **Architectural Resolution for C15**, scoped exactly to the single question named in §10 and the three decision points in §12 — not performed by this document, and not to begin without separate, explicit authorization.

---

## Status

**Draft. Awaiting review.** No Architectural Resolution, Implementation Authorization, or implementation has been performed. No ADR created. No Governance Register item opened or modified. GOV-002, Initiative 2, and Initiative 3 Item 6a remain exactly as they were before this document — GOV-002 confirmed still valid on its own question (§5), Initiative 3 Item 6a confirmed still gated (§9), Initiative 2 confirmed untouched (§3, §7, §13). No architecture, governance, or historical lifecycle document was modified in preparing this evaluation.

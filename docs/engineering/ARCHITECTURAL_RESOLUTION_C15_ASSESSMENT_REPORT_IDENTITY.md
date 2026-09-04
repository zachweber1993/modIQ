# C15 Architectural Resolution — Assessment & Report Subject Identity

| Property | Value |
|---|---|
| **Document** | ARCHITECTURAL_RESOLUTION_C15_ASSESSMENT_REPORT_IDENTITY.md |
| **Project** | modIQ |
| **Question Resolved** | Does C15 require an architectural mechanism capable of incorporating Evidence-derived subject identity after Assessment construction, or can meaningful/durable Assessment/Report subject identity be established entirely from information known at submission/Assessment construction time? |
| **Origin** | `ARCHITECTURE_EVALUATION_C15_ASSESSMENT_REPORT_IDENTITY.md` (working tree, uncommitted), whose §10–§12 name this exact question as the sole surviving open architectural question and reserve its weighing for this stage. That Evaluation is treated as fixed evidence; this Resolution dispositions its findings, it does not reopen them. |
| **Adopted Precedent** | `ADR-0003` (Assessment as the Aggregate Root) and `ADR-0011` (`AssessmentReport` Ownership Correction), both in their entirety; `ADR-0007` (Runtime Entity Design Pattern); `GOVERNANCE.md`'s Runtime Domain, Reporting, and Storage Crate Boundary Rules; GOV-002 (Resolved); Initiative 3's Architectural Resolution (Item 6a); the Storage Mirror pattern as already demonstrated at Sprint 13, Sprint 22, and Sprint 24 — all treated as binding, unreopened repository architecture and history. |
| **Status** | **Architectural Resolution. No ADR, crate, API, Rust type, field, schema, identifier format, or implementation technology has been created, designed, or chosen. No existing repository document has been modified by this Resolution. No new Governance Register entry has been created.** |

---

## §1 — Method

This Resolution dispositions the four architectural approaches `ARCHITECTURE_EVALUATION_C15...md` §8 identified, plus the one alternative that Evaluation left unweighed (a post-construction, Evidence-derived subject-enrichment mechanism), using the disposition vocabulary this repository's own Architectural Resolutions already use (`INITIATIVE_3_ARCHITECTURAL_RESOLUTION.md` §1, `C2_ARCHITECTURAL_RESOLUTION_DECLARED_DEPENDENCY_INTERPRETATION.md` §1): **Adopted / Adopted Architectural Constraint / Deferred / Rejected**. No new disposition category is introduced. Evidence, reasoning, and decision are kept in separate sections throughout.

---

## §2 — Repository Evidence

Re-confirmed directly this session, not carried forward from the Architecture Evaluation's own summary without independent re-check.

- **`ADR-0003`**, read in full: designates `Assessment` the Runtime domain's sole aggregate root and lists `Assessment Subject` among Assessment-owned entities.
- **`ADR-0011`**, read in full: supersedes `ADR-0003` **solely** with respect to `Assessment Report`'s own inclusion in that list. Its own Decision text states directly: *"`Assessment Context`, `Assessment Subject`, `Evidence`, `Findings`, and `Recommendations` remain accurately described as Assessment-owned entities exactly as ADR-0003 states."*
- **`ADR-0007`**, read in full: every Runtime entity's identity and content are fixed "once, at construction"; entities are validated at construction through fallible constructors and are never described anywhere in this document as later-mutable in content, only in lifecycle status (which `Assessment` itself, not its owned value types, carries).
- **`crates/modiq-runtime/src/assessment/assessment.rs`** — `Assessment`'s complete public method list (re-enumerated, unchanged since the Architecture Evaluation): `id`, `subject`, `context`, `version_profile`, `status`, `is_evaluating`, `evidence`, `findings`, `recommendations`, `evidence_by_id`, `finding_by_id`, `evidence_for_finding`, `findings_for_recommendation`, `begin_evidence_collection`, `begin_rule_evaluation`, `complete`, `add_evidence`, `add_finding`, `add_recommendation`. No method updates `subject` or `context` after construction.
- **`crates/modiq-engine/src/engine/assessment_service.rs`**, `AssessmentService::execute` — confirmed, exact order: `Assessment::new(subject, context, version_profile_reference)` executes first; `begin_evidence_collection()` and the `add_evidence` loop execute strictly afterward. `AssessmentSubject`'s content is fixed before any Evidence exists, by construction, not by convention.
- **`crates/modiq-report/src/report/assessment_report.rs`**, `AssessmentReport::generate` — reads `evidence()`, `findings()`, `recommendations()` from `Assessment`; does not read `subject()`/`context()` today.
- **`GOVERNANCE.md`, Reporting**: *"Reports are reflections of Assessment state. They never create new state."*
- **`GOVERNANCE.md`, Storage**: persisted representation "populated only from `AssessmentReport`'s already-public API."
- **GOV-002's Resolution**, quoted directly: *"`AssessmentSubject`/`AssessmentContext`'s minimal content... were both evaluated and require no action."* Reached while checking `RuntimeInvariants.md`'s fourteen invariants against implementation — scoped to invariant-conformance, not to any product-level sufficiency question.
- **`INITIATIVE_3_ARCHITECTURAL_RESOLUTION.md`, Item 6a**: a Subject-identity field is Adopted in principle, "contingent" on `AssessmentSubject` "carrying sufficient content" — with "sufficient" left undefined by that Resolution.
- **`CAPABILITY_DEFINITION_C15_ASSESSMENT_REPORT_IDENTITY.md`**, re-read in full this session:
  - §5 (Desired Capability): "modIQ should be able to meaningfully answer... what is this an assessment of? — durably enough that the answer survives... and consistently enough that two Reports of the same mod can *eventually* be recognized as related... wherever a downstream consumer chooses to make that comparison." Explicitly does not specify "a data shape, a source of truth for the identifying information, a persistence mechanism, or a presentation."
  - §8 (Acceptance Criteria): "A user completing two consecutive Assessments... can... correctly state which mod it concerns — without relying solely on their own short-term memory"; "Two Reports produced from the same mod can, **in principle**, be recognized... as concerning the same subject." Neither criterion states or implies canonical correctness, semantic derivation from mod contents, or matching-feature accuracy — both are satisfied by durable, meaningful reference, stated at exactly that altitude and no higher.
- **`IMPLEMENTATION_REPORT_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md` §5** — the actual forcing instance re-examined directly: real assessments of `FS25_HH_HempProdSystem.zip` and `Hashy_01Silv.zip`, both individually distinguishable by name alone; the acceptance record contains no instance of ambiguous, generic, or misleading naming, and no observed case where a human-assigned filename failed to answer "which mod is this."
- **`ARCHITECTURE_EVALUATION_C15_ASSESSMENT_REPORT_IDENTITY.md` §7, §8** — the Mirror pattern's own three prior instances (Sprint 13, 22, 24), and the finding that `AssessmentId`'s documented non-durability governs identity-as-reference, not identity-as-content.
- **`SPRINT8_ARCHITECTURAL_RESOLUTION.md` §8** — the Architectural Activation precedent: a dormant, already-designated concept (there, Version Profiles) receiving its first real content, rather than a new, competing concept being introduced.

---

## §3 — Architectural Reasoning

Reasoning only; no decision is stated in this section.

**On what the Capability Definition actually requires, versus what it does not.** §8's own criteria are written, and re-confirmed here to remain written, at the level of *durable, meaningful reference* — "correctly state which mod," "recognized... in principle." Neither criterion requires canonical correctness against a mod's own self-declared internal name, semantic derivation from mod contents, or an actual matching implementation. A capability satisfied by information already known to the caller at submission time is not a weakened or partial reading of §8 — it is a reading that matches §8's own stated altitude exactly, without the strengthening the mission's own instruction warned against introducing.

**On whether the real forcing instance demonstrates a need beyond submission-time information.** It does not. `IMPLEMENTATION_REPORT_C14_OPERATIONAL_ASSESSMENT_CONSOLE.md`'s own real, non-mock acceptance evidence — the only concrete forcing instance this capability rests on — involved two mods whose own filenames already, individually, answered "which mod is this," with no ambiguity, no genericness, and no failure mode recorded anywhere in that report. The distinction the mission asked to be preserved explicitly is preserved here: **the forcing evidence establishes that a durable answer is needed; it does not establish that today's forcing evidence itself is insufficiently answered by submission-time information.** These are different claims, and only the first is what C14 actually demonstrated.

**On why Evidence-derived identity would be a first-of-kind exception, not an extension of settled practice.** Every prior Runtime entity extension (Sprint 22's Title/Summary, Category, Evidence provenance; Sprint 24's structured repair steps) populated a field from information already available at the moment of construction. `ADR-0007`'s construct-once principle, `Assessment`'s own confirmed absence of any post-construction `subject` mutator, and Reporting's explicit "never create new state" rule together mean that content only knowable after Evidence collection (for example, a mod's own declared name in `modDesc.xml`) has no existing path into either `AssessmentSubject` or `AssessmentReport` today. Introducing one would not be filling in an anticipated but unbuilt extension point — Initiative 3 Item 6a's own gate names no such mechanism, and nothing in `ADR-0007` reserves space for it — it would be a genuinely new kind of exception to an already-Accepted, repeatedly-exercised pattern.

**On whether that exception is currently justified.** Under this project's own reversed-burden-of-proof discipline, it is not. No concrete evidence — not the C14 forcing instance, not any other real usage recorded anywhere in this repository — demonstrates that submission-time information fails to answer "which mod is this" in practice. The only case such a mechanism would meaningfully help (a generically or misleadingly named archive, e.g. a bare `mod.zip`) is hypothetical relative to everything currently evidenced. Building a first-of-kind construct-once exception against a hypothetical, rather than a demonstrated, failure would invert this project's own standing discipline of evidence preceding architecture.

**On why this does not disturb GOV-002.** GOV-002's own question — does the zero-field state violate any Runtime Invariant — remains answered "no," permanently; nothing here proposes a state that violates any of the fourteen invariants, and nothing here revisits whether the *previously* empty state was ever invariant-noncompliant. C15 answers a different, additional question GOV-002 never asked: whether a real product need now justifies giving the entity real content going forward. Both answers stand simultaneously without contradiction.

**On what this establishes for Initiative 3 Item 6a.** Item 6a's own gate — "sufficient content" — was left undefined by its own Resolution. This Resolution supplies the missing definition for the class of content this capability actually requires: content available to the caller at Assessment-construction time is sufficient to satisfy C15's own stated criteria (§8, above). Populating `AssessmentSubject` with such content is therefore an ordinary Runtime entity extension, following the same `ADR-0007` pattern already exercised five times, requiring no further Architecture Evaluation of its own — the same treatment Initiative 3 already gave five sibling fields without a dedicated evaluation each.

---

## §4 — Alternative Dispositions

Each of the four architectural approaches the Architecture Evaluation named, plus the fifth alternative it left unweighed, dispositioned individually.

**Approach A — Identity upstream of Storage (Runtime/Reporting).** **Adopted**, for content available at Assessment-construction time. Directly supported by `ADR-0003`/`ADR-0011`'s own designation of `AssessmentSubject` as Assessment-owned, and by `AssessmentReport::generate`'s already-established pattern of reading `Assessment`'s own public getters.

**Approach B — Identity introduced at the persistence boundary (Storage originates the fact).** **Rejected.** Directly incompatible with `GOVERNANCE.md`'s own Storage text ("populated only from `AssessmentReport`'s already-public API"). In the one form that would not violate that rule (Storage merely mirroring a value Reporting already exposes), it is not a distinct approach — it collapses into Approach C, below.

**Approach C — Identity flows Runtime → Reporting → Storage via the established Mirror pattern.** **Adopted**, as the mechanism by which content originating under Approach A becomes durable across the process boundary, using the same pattern already demonstrated at Sprint 13, 22, and 24. Adopted jointly with Approach A as one coherent architectural direction, not as a separate or competing decision.

**Approach D — A wholly new, independent identity concept.** **Rejected**, for the class of content this Resolution addresses. `AssessmentSubject` is already the designated, purpose-built home for exactly this concept (`ADR-0003`/`ADR-0011`); this repository's own precedent for a dormant-but-already-designated concept receiving its first real content is Architectural Activation (`SPRINT8_ARCHITECTURAL_RESOLUTION.md` §8), not the introduction of a competing, parallel concept. No evidence reviewed shows `AssessmentSubject` unsuitable for this purpose.

**Fifth alternative — a new post-construction, Evidence-derived subject-enrichment mechanism.** **Deferred**, not Rejected outright. §3, above, establishes it is not currently justified by any evidence in this repository, and that building it now would be designing for a hypothetical case, not a demonstrated one. It is not foreclosed permanently: if real future usage demonstrates that submission-time information genuinely fails to provide meaningful reference in practice, that would constitute new evidence and would warrant revisiting this specific question on its own terms — exactly the same evidence-driven reconsideration this repository has already applied to Savegame evidence, Lua analysis, and Confidence's own computation mechanism.

---

## §5 — Decision

Answering each question the mission posed, precisely and only as far as the evidence in §2–§4 supports:

**A. Is submission-time-known subject information architecturally sufficient?** **Yes.** It fully satisfies `CAPABILITY_DEFINITION_C15...md` §5 and §8 as those sections are actually written (§3, above), using only already-Accepted architecture (`ADR-0003`/`ADR-0007`/`ADR-0011`, the existing Reporting/Storage boundary rules, the proven Mirror pattern).

**B. Is Evidence-derived subject identity required by the current C15 capability?** **No.** Neither the Capability Definition's own criteria nor the one real forcing instance (C14's Hemp/Hashy sequence) requires it.

**C. Is a new post-construction subject-enrichment mechanism required?** **No**, not at this time. Deferred (§4), not designed, not authorized.

**D. Is a new architectural concept required?** **No.** `AssessmentSubject` — already Assessment-owned by explicit ADR designation — is architecturally sufficient as the concept this capability populates.

**E. What existing architectural pattern should C15 use?**

> **Assessment/Report subject identity is established from information available at Assessment construction time, as an Assessment-owned fact (`ADR-0003`/`ADR-0011`), presented — never originated — by `AssessmentReport` (`GOVERNANCE.md`'s Reporting rule), and, where durability across a process boundary is required, carried into `modiq-storage` using the same Mirror pattern already demonstrated at Sprint 13, Sprint 22, and Sprint 24. No new architectural participant, boundary crossing, or exception to `ADR-0007`'s construct-once principle is required to satisfy the capability as currently defined.**

This is stated at the architectural-pattern level; it does not select a field name, a Rust type, a serialization format, or any implementation detail — those remain, precisely as the mission specified, for Implementation Authorization and Implementation Planning.

**Initiative 3, Item 6a:** the architectural gate is now satisfiable, for content of the class this Resolution addresses (submission-time-known content), at the ordinary Implementation Authorization level — no further Architecture Evaluation is required for that class of content. This Resolution does **not** claim any field or schema has been implemented, and does not itself satisfy the gate — satisfaction occurs only once real content is actually added, following whatever Implementation Authorization eventually scopes it. If a future capability's own evidence later justifies Evidence-derived content instead of, or in addition to, submission-time content, Item 6a's gate would need to be revisited specifically for that class, on its own evidentiary basis — this Resolution does not pre-clear that case.

**GOV-002:** remains valid, unchanged, unsuperseded. Its own question (invariant-conformance of the then-existing empty state) and this Resolution's own question (architectural sufficiency of a newly-justified, not-yet-populated state) are different questions with independently correct answers; C15 activates a product/architectural requirement GOV-002 was never asked to evaluate, without contradicting anything GOV-002 actually concluded. No cross-reference is added to GOV-002's own text by this document — per this repository's own established discipline (`C2_ARCHITECTURAL_RESOLUTION_DECLARED_DEPENDENCY_INTERPRETATION.md`'s own precedent), any such housekeeping cross-reference is named here as a reasonable future step, not performed as part of this Resolution.

---

## §6 — Consistency and Governance Register Review

- **Does any Governance Register item require a new entry?** No. This question is closed by this dedicated Resolution, consistent with this repository's own precedent of resolving capability-scoped architectural questions without a permanent Register entry once the evidence supports a clear disposition.
- **Does this Resolution reopen, narrow, or extend Initiative 2?** No. Initiative 2 concerns exclusively whether one already-completed Assessment may later accept supplementary evidence so its own Report continues to evolve. Nothing in this Resolution's own subject matter — identifying separate, independent Assessments — touches that question in any respect; re-confirmed directly against `INITIATIVE_2_ARCHITECTURAL_RESOLUTION.md`'s own Central Architectural Question one further time at this stage.
- **Does this Resolution require any amendment to `RuleEngine.md`, `DataModel.md`, `Architecture.md`, or any other frozen architecture document?** No. Everything this Resolution relies on is already stated in currently-Accepted ADRs and currently-Resolved governance text; nothing new is being added to the architectural corpus itself, only an interpretation of how existing text applies to a new capability.
- **Is any repository precedent established for future use?** Yes: the disposition method applied here (checking whether a proposed extension's *content class* is available at an entity's own existing construction moment before treating any Runtime-entity extension as requiring more than ordinary Implementation Authorization) is a reusable pattern for any future capability proposing to extend a Runtime entity with content of ambiguous timing — named here for future reference, not elevated to a standing rule beyond this capability's own resolution.

---

## §7 — Repository Impact

| Area | Impact |
|---|---|
| `crates/modiq-runtime` (`Assessment`, `AssessmentSubject`, `AssessmentContext`) | Unchanged. No field, method, or type modified. |
| `crates/modiq-report` (`AssessmentReport`) | Unchanged. |
| `crates/modiq-storage` (`ReportKey`, `PersistedAssessmentReport`, `ReportStore`) | Unchanged. |
| `crates/modiq-collection`, `crates/modiq-rules`, `crates/modiq-engine`, `crates/modiq-cli` | Unchanged; unaffected by this Resolution. |
| `apps/console` | Unchanged. |
| `ADR-0003`, `ADR-0007`, `ADR-0011` | Unchanged in text; confirmed, not amended, to already accommodate this capability's own direction. |
| `GOVERNANCE.md` | Unchanged. GOV-002 remains Resolved, textually unmodified. |
| `INITIATIVE_3_ARCHITECTURAL_RESOLUTION.md` | Unchanged; Item 6a's own gate is interpreted, not amended. |
| ADRs | None created, none required. |
| Governance Register | No new item. |
| `CAPABILITY_DEFINITION_C15...md`, `PROCESS_DETERMINATION_C15...md`, `ARCHITECTURE_EVALUATION_C15...md` | Unchanged; their findings are dispositioned here, not revised. |

---

## §8 — What Is Not Decided, Not Designed, Not Performed

- **No exact `AssessmentSubject` field, name, or type is chosen.** Whether the eventual content is a raw path, a filename, a user-supplied label, or some other submission-time value is an Implementation Authorization-level decision, not decided here.
- **No identifier format is chosen** — string, UUID, or otherwise.
- **No `ReportKey` redesign, no Storage API change, no database/schema design.**
- **No DTO, transport, or UI design of any kind.**
- **No console history, same-mod matching implementation, or cross-mod comparison (C5)** — all remain named only as downstream consumers, exactly as `CAPABILITY_DEFINITION_C15...md` §7/§10 already established, unauthorized and unscoped by this document.
- **No Confidence, Evidence Explanation, Savegame/User Observation evidence, Investigate/Diagnose/Remediate workflow, or Knowledge-layer activation** — none is implicated by this Resolution.
- **No Implementation Authorization or Implementation Plan.** Participating files, exclusions, invariants, and verification gates for realizing this Resolution's own §5(E) pattern are not defined here — that remains the next artifact's own responsibility.
- **No ADR created.** No existing repository document — `GOVERNANCE.md`, any ADR, `INITIATIVE_3_ARCHITECTURAL_RESOLUTION.md`, or GOV-002's own entry — was modified.
- **Initiative 2 is not reopened, advanced, or reinterpreted** in any respect.
- **The Deferred fifth alternative (§4) is not designed, scheduled, or committed to** — it remains available only if future evidence justifies revisiting it.

---

## Status

**Adopted.** Submission-time-known information is architecturally sufficient to satisfy C15 as currently defined; no architectural mechanism for Evidence-derived, post-construction subject-enrichment is required by any evidence currently in this repository. Assessment/Report subject identity is established as an Assessment-owned fact, presented by `AssessmentReport`, and — where durability is required — mirrored into `modiq-storage` via the already-proven pattern, using no new architectural participant, boundary, or exception to `ADR-0007`. GOV-002 remains valid, unsuperseded. Initiative 3 Item 6a's gate is interpreted, for this class of content, as satisfiable at the ordinary Implementation Authorization level — not itself satisfied by this document. No ADR, crate, API, Rust type, field, schema, or implementation technology has been created, designed, or chosen. No existing repository document has been modified. This Resolution's responsibility ends here, awaiting Product Owner review.

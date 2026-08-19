# Process Determination — C4: `apps/sandbox` Field Parity

| Property | Value |
|---|---|
| **Document** | PROCESS_DETERMINATION_C4_APPS_SANDBOX_FIELD_PARITY.md |
| **Project** | modIQ |
| **Type** | Repository Process Determination — decides which repository artifact follows the Capability Definition. It does not authorize implementation, evaluate architecture in the sense of designing a new mechanism, design a DTO shape, or define an Implementation Plan. |
| **Origin** | `docs/engineering/CAPABILITY_DEFINITION_C4_APPS_SANDBOX_FIELD_PARITY.md` (`a024c8f`), which names this determination as the next required repository artifact and identifies five specific open questions this document must investigate before making it. |

---

## Purpose

This document answers one question, and only one: does C4 follow Sprint 23's/C1's/C3's precedent (Implementation Authorization directly, no Architecture Evaluation or Architectural Resolution) or C2's precedent (Architecture Evaluation and Architectural Resolution first)? Both paths are real, committed repository precedent. The Capability Definition explicitly declined to assume C4 follows C3's path merely because both are Category A field-parity candidates, and named five specific questions this document must investigate first — that investigation is the bulk of this document's own work, distinct from, and prior to, applying the test itself.

---

## Repository Precedent for This Choice

The two-prong test this determination applies was introduced once, by C1 (`PROCESS_DETERMINATION_C1_RECOMMENDATION_STEP_PRESENTATION.md`), reused once already by C3 (`PROCESS_DETERMINATION_C3_CLI_FIELD_PARITY.md`), and is reused a third time here, per its own stated instruction that it is reusable, not a newly invented test each time:

1. Is there a genuinely open design question, or is the governing principle already Adopted?
2. Is any alternative being weighed?

C2 required full Architecture Evaluation and Architectural Resolution because it reached a genuinely untested architectural question a prior Architectural Resolution (`SPRINT12_ARCHITECTURAL_RESOLUTION.md`) had deliberately left open. C1 and C3 did not, because in both cases direct inspection found the governing principle already Adopted and no alternative being weighed. This document investigates, fresh, which of those two shapes C4 actually matches — not by analogy to C3's own conclusion, but by independently checking each of the five questions the Capability Definition named.

---

## Investigation of the Capability Definition's Named Open Questions

### Question 1 — Does the flat `apps/sandbox` transport shape create a genuine architectural problem, or can it remain unchanged?

**Finding: it can remain unchanged. No architectural problem exists.**

`apps/sandbox`'s `AssessmentSummary` carries three flat, parallel `Vec` fields (`evidence`, `findings`, `recommendations`), confirmed directly against `apps/sandbox/src-tauri/src/lib.rs` this session — the same shape `AssessmentReport` itself exposes (`evidence()`, `findings()`, `recommendations()` as three independent collections). Extending each existing flat DTO in place with new fields (`label`, `source`, `content` on `EvidenceEntry`; `mod_health_dimension`, `status` on `FindingEntry`; `repair_steps` on `RecommendationEntry`) requires no restructuring of that shape — it is the direct, additive continuation of the same `From<&T>` conversion-struct pattern `apps/sandbox` already applies today for the fields it currently carries. No governing document requires resolving Finding-to-Evidence or Finding-to-Recommendation association at the transport boundary; `apps/sandbox`'s own `App.tsx` does not currently perform or need such association for the fields it renders.

### Question 2 — Is `apps/console`'s nested-by-Finding structure precedent, or an architectural requirement?

**Finding: precedent only, and explicitly, textually not a requirement — confirmed by the governing architecture document itself.**

`docs/architecture/FrontendArchitecture.md` (§ Consumer-Owned State / reserved items) states directly: *"Reserved, pending GOV-008 (AssessmentService Public API Evolution): The concrete payload shape of the request/response contract between the consumer and the engine."* `GOVERNANCE.md`'s own GOV-008 entry confirms this item's status is **Open** (last reviewed at Platform Validation Phase 1; "implementation evidence gathered was found insufficient to resolve this item. No architectural change is authorized by that review"). `apps/console`'s own `assessment.rs` corroborates this from the implementation side, in its own words: *"`ReportSummary` is provisional, not final or stable — Authorization §4 holds the concrete request/response payload shape open pending GOV-008."*

This is decisive: the nested-by-Finding shape is not an architectural requirement any future consumer must follow — it is `apps/console`'s own self-described provisional implementation choice, made under an explicitly still-open governance item that predates C4 by many Sprints. C4 does not need to resolve GOV-008, adopt `apps/console`'s shape, or invent a third shape — it may extend `apps/sandbox`'s existing flat shape in place, exactly as the Capability Definition's own Explicit Exclusions already anticipated, without that choice conflicting with any governing text.

### Question 3 — Is the absence of an `apps/sandbox` `GOVERNANCE.md` Crate Boundary Rule a blocking governance gap, a documentation gap C4 should close, or adequately covered by existing general governance?

**Finding: adequately covered — and this is not the same kind of gap the Capability Definition analogized it to.**

The Capability Definition's own Next Required Repository Artifact section speculated this might mirror "Sprint 8's own `modiq-versioning` gap-closing amendment." Direct investigation of that precedent finds the opposite of what the phrase "gap-closing amendment" implies:

- `SPRINT8_ARCHITECTURAL_RESOLUTION.md` (Decision 6) *recommended* a new `modiq-versioning` Crate Boundary Rule be drafted "during Sprint 8's own Phase 1... before any Rust code is written."
- `SPRINT8_IMPLEMENTATION_REPORT.md` records, instead: *"`GOVERNANCE.md`'s Crate Boundary Rules gap for `modiq-versioning` remains, by explicit decision, unaddressed this Sprint."* This is an explicit Technical Director decision to defer, not an amendment.
- The gap was tracked as open across at least five subsequent Engineering Releases (`0.8`, `1.1`, `1.3`, `1.4`, `SPRINT_14_PROPOSAL.md`) and, confirmed by direct fresh re-read of `GOVERNANCE.md` this session, **remains open today** — no "Versioning" or "modiq-versioning" section exists anywhere in `GOVERNANCE.md`'s nine Crate Boundary Rules. No "gap-closing amendment" for `modiq-versioning` was ever actually made.

This is directly relevant, but not as the Capability Definition framed it: it demonstrates that an *identified, named, even explicitly-recommended-for-closure* Crate Boundary Rule gap has already been treated, by direct repository precedent, as non-blocking to implementation — deferred by explicit decision rather than closed, for a real workspace member, across many subsequent capabilities, with no recorded harm.

`apps/sandbox`'s own situation is stronger still, and categorically different in kind, not merely in degree. Root `Cargo.toml`'s own comment states directly: *"`apps/console/src-tauri` is a real workspace member (unlike `apps/sandbox`, which deliberately is not — see its own Cargo.toml) because Initiative 5 named non-membership as one of three facts disqualifying a candidate from being the production interaction layer."* `PLATFORM_FOUNDATION_V1_DECLARATION.md` (F3) independently corroborates this: its own enumeration of "ten crates," each with "a named Crate Boundary Rule in `GOVERNANCE.md`," lists `apps/console/src-tauri` as the tenth member — `apps/sandbox` does not appear in that list at all. `INITIATIVE_5_ARCHITECTURAL_RESOLUTION.md`, Decision 6, is **Adopted**: *"`apps/sandbox`, as it exists today, cannot be the production interaction layer... three independently verifiable, disqualifying facts (non-membership in the root workspace, self-declared non-production status, deliberate absence of a real input mechanism)."*

`GOVERNANCE.md`'s nine Crate Boundary Rules sections correspond one-to-one with nine of the ten real root-workspace members (`modiq-versioning` being the sole, separately-tracked exception investigated above). `apps/sandbox` was never a candidate for inclusion in the first place — its absence from `GOVERNANCE.md` is the direct, already-Adopted, already-explained consequence of an existing Architectural Resolution, not an unclosed prerequisite. `INITIATIVE_5_ARCHITECTURE_EVALUATION.md`'s own Repository Evidence, gathered when evaluating exactly this question, independently confirms `apps/sandbox` already exercises the same discipline a Crate Boundary Rule would state: *"Its DTOs... are built from `AssessmentReport`'s already-public getters — never from `modiq-runtime` types directly — and its own source comments cite `modiq-storage`'s representation-boundary precedent as the model for doing so."* `PROPOSAL_FILESYSTEM_COLLECTION.md`'s own Sandbox Interaction section frames it identically: *"a thin developer visualization tool owning zero domain logic."*

No genuine question survives here. This is not a documentation gap C4 should close, and not a blocking governance gap — it is an already-Adopted architectural fact, adequately covered by existing general governance and an existing, on-point Architectural Resolution.

### Question 4 — Does the pre-existing `App.tsx` `description`-vs-`title`/`summary` drift affect C4's architectural determination?

**Finding: no.**

This drift (`App.tsx`'s `FindingEntry` interface still declaring `description: string`, unchanged since commit `a11f9ec`, 2026-07-19, while the Rust `FindingEntry` struct has carried `title`/`summary` since Sprint 22, `969e595`, 2026-08-03) is a symptom of ordinary implementation staleness — a TypeScript interface not updated when its Rust counterpart's fields changed — not a crate-boundary question, not a governance question, and not evidence of any architectural gap. It implicates no getter's public/correctness status, no dependency edge, and no governing document. Whether and how it is corrected is Implementation Planning's own concern (as the Capability Definition's own Explicit Exclusions already state), and does not bear on whether C4 requires Architecture Evaluation.

### Question 5 — Does the persisted-path asymmetry the Capability Definition identified create any architectural concern requiring work outside `apps/sandbox`?

**Finding: no. This is a second application of an already-Adopted boundary, not a new question.**

`PersistedEvidence`/`PersistedFinding`'s missing fields (`label`/`source`/`content`, `mod_health_dimension`/`status`) are governed by `GOVERNANCE.md`'s existing Storage Crate Boundary Rule: Storage's persisted representation is "populated only from `AssessmentReport`'s already-public API," with no requirement that it mirror every Runtime field. This exact boundary, and this exact asymmetry, was already investigated and bounded once, for `modiq-cli retrieve`, by C3's own Capability Definition and Process Determination — C4 does not discover a new instance of the question; it inherits the same, already-settled answer. `PersistedRecommendation::repair_steps()` (confirmed present, `crates/modiq-storage/src/storage/persisted_report.rs:252`, added at Sprint 24 commit `84ef4e3`) means C4's own `retrieve_report` half can present `repair_steps` without any `modiq-storage` schema change — exactly the bounded, asymmetric scope C3 established as this repository's own working pattern for a Storage-side gap it does not authorize closing.

### Question 6 — Does any ADR, Architectural Resolution, governance precedent, or prior capability provide a directly applicable determination?

**Finding: yes, several, cited above and consolidated here:**

- `INITIATIVE_5_ARCHITECTURAL_RESOLUTION.md`, Decision 6 (**Adopted**) — `apps/sandbox` is disqualified from the production interaction layer; its pattern is validated precedent.
- `INITIATIVE_5_ARCHITECTURE_EVALUATION.md`'s Repository Evidence — already directly examined `apps/sandbox`'s own DTO-construction discipline and found it sound, without treating its absent Crate Boundary Rule as a defect requiring correction.
- `FrontendArchitecture.md`'s own Reserved-pending-GOV-008 clause — the transport-shape question is explicitly, textually deferred by the governing document itself, not left open by oversight.
- `GOVERNANCE.md`'s Storage Crate Boundary Rule — already governs and bounds the persisted-path asymmetry, previously exercised by C3.
- `ADR-0007` (Opaque Runtime References) — governs `RecommendationStep`'s non-identity status, already confirmed by C3 and re-confirmed here; unaffected by C4.
- Sprint 8's `modiq-versioning` precedent — corroborating, not decisive on its own, but independently confirms this repository has already treated an absent-and-recommended-for-closure Crate Boundary Rule as non-blocking to implementation, for a real workspace member, across many subsequent Sprints.

---

## Applying the Test to C4

**Prong 1 — is there a genuinely open design question, or is the governing principle already Adopted?**

No genuinely open design question survives investigation. Each of the five questions the Capability Definition named resolves to an already-Adopted architectural fact, an already-governed boundary C3 already exercised once, or a non-architectural implementation detail:

- The flat transport shape is not an architectural problem; extending it in place is a direct, additive continuation of `apps/sandbox`'s own established pattern.
- `apps/console`'s nested shape is explicitly reserved-as-provisional by `FrontendArchitecture.md` itself, pending a separate, pre-existing, still-open governance item (GOV-008) — not a requirement C4 must follow or resolve.
- The absent `apps/sandbox` Crate Boundary Rule is the direct, already-Adopted consequence of Initiative 5's own Decision 6 (`apps/sandbox`'s deliberate non-membership in the root workspace), independently corroborated by root `Cargo.toml`'s own comment and `PLATFORM_FOUNDATION_V1_DECLARATION.md`'s crate enumeration — not an unclosed prerequisite of the kind Sprint 8 left open for `modiq-versioning`.
- The `App.tsx` drift is ordinary implementation staleness, orthogonal to this determination.
- The persisted-path asymmetry is a second, already-bounded instance of `GOVERNANCE.md`'s Storage Crate Boundary Rule, previously exercised by C3.

**Prong 2 — is any alternative being weighed?**

No. C4 proposes no new formatting mechanism, no new dependency edge (the Transport Mirror pattern `apps/sandbox` already uses is the same mechanism, extended additively), no reinterpretation of any crate boundary, and no change to which crate owns which responsibility. The one real design fork the Capability Definition named — flat-in-place versus adopting `apps/console`'s nested shape — is not a disposition this document, or C4 at all, is required to make: `apps/sandbox` may extend its own existing flat shape in place without conflicting with any governing text, deferring the same reserved question `apps/console` itself has already been deferring since Sprint 21.

---

## Determination

**C4 matches C1's and C3's path on both prongs of the test.** No genuinely open design question exists among the five the Capability Definition named — each resolves to an already-Adopted architectural fact or an already-governed, previously-exercised boundary. No alternative is being weighed; the reserved transport-shape fork is not this capability's to resolve, and may remain deferred without blocking it.

**The next required repository artifact is an Implementation Authorization**, not an Architecture Evaluation or Architectural Resolution.

One asymmetry with C3's own case is worth naming plainly, not smoothed over: C3's case rested on `modiq-cli`'s thin-but-present "## CLI" governing text being found adequate. C4's case rests on a different, and on independent investigation stronger, foundation — `apps/sandbox` was never expected to have Crate Boundary Rule text in the first place, by an already-Adopted Architectural Resolution predating this capability. These are not the same argument reused by assumption; they are two independently-grounded conclusions that happen to lead to the same procedural outcome.

---

## What This Document Does Not Do

- It does not draft the Implementation Authorization itself.
- It does not define implementation boundaries, participating files, exclusions, invariants, verification gates, or completion criteria — those belong to the Implementation Authorization.
- It does not design a DTO shape, choose field names, choose the flat-versus-nested transport question's own eventual answer, or write any TypeScript/Rust change.
- It does not resolve GOV-008, and does not treat this document's own conclusion as resolving it — GOV-008 remains Open, exactly as it was before this document, for whichever future capability or governance session eventually forces the question.
- It does not amend `GOVERNANCE.md`, create a new "Sandbox" Crate Boundary Rule section, or otherwise act on Question 3's finding beyond stating it — that finding concludes no such amendment is required, not that this document performs one.
- It does not correct `App.tsx`'s pre-existing drift.
- It does not reopen, revise, or reinterpret `INITIATIVE_5_ARCHITECTURAL_RESOLUTION.md`, `FrontendArchitecture.md`, or `GOVERNANCE.md`'s Storage Crate Boundary Rule — every conclusion above traces to something already Adopted or already confirmed by direct source inspection recorded in this document itself.

---

## Note on This Document's Own Standing

This is the third committed instance of a repository precedent C1 established and C3 already reused once — not an invocation of a permanently canonical lifecycle stage (`PROJECT_HANDOFF_v1.1.md` §5's own canonical workflow names no "Process Determination" stage; this remains true, unchanged, from C3's own identical observation). What is reused here is the two-prong test itself; what is not reused by assumption is its outcome — this document independently re-derived every one of the five named open questions from primary source before applying that test, rather than inferring C4's answer from C3's shape.

---

## Next Required Repository Artifact

An Implementation Authorization for C4 — `apps/sandbox` Field Parity, following the per-command scope the Capability Definition's own Capability Statement establishes: field extension for both `create_assessment` (live path, full field set) and `retrieve_report` (persisted path, bounded to `repair_steps` only, per Question 5's finding above) — with the flat-versus-nested transport-shape question left exactly as reserved as `FrontendArchitecture.md` itself already leaves it, and the `App.tsx` drift left to whatever incidental correction Implementation Planning finds natural, neither authorized nor required by this document.

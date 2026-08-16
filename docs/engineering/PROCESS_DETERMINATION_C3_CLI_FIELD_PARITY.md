# Process Determination — C3: `modiq-cli` Field Parity

| Property | Value |
|---|---|
| **Document** | PROCESS_DETERMINATION_C3_CLI_FIELD_PARITY.md |
| **Project** | modIQ |
| **Type** | Repository Process Determination — decides which repository artifact follows the Capability Definition. It does not authorize implementation, evaluate architecture, design a formatting shape, or define a Sprint. |
| **Origin** | `docs/engineering/CAPABILITY_DEFINITION_C3_CLI_FIELD_PARITY.md` (`31de26e`), §"Next Required Repository Artifact," which states this determination has not yet been made and names it as the choice the next artifact must resolve. |

---

## Purpose

This document answers one question, and only one: does C3 follow Sprint 23's/C1's precedent (Implementation Authorization directly, no Architecture Evaluation or Architectural Resolution) or Sprint 21's/C2's precedent (Architecture Evaluation and Architectural Resolution first)? Both paths are real, committed repository precedent.

This determination is answered by applying the same test the repository has already used once, for exactly this choice — not a newly invented one. `PROCESS_DETERMINATION_C1_RECOMMENDATION_STEP_PRESENTATION.md` states this directly of its own method: "This determination is answered by applying the same test the repository already used, once, to make this exact choice — not a newly invented one." That same instruction governs this document.

---

## Repository Precedent for This Choice

`PROJECT_HANDOFF_v1.1.md` §5 defines one canonical engineering workflow (Capability Definition → Architecture Evaluation → Architectural Resolution → Implementation Authorization → …), confirmed by direct re-read this session. It states Architecture Evaluation applies conditionally: "Architecture Evaluation is where a genuine architectural question exists." The repository does not formally recognize two standing procedural tracks — each of C1 and C2 is a historically documented, individually justified application of that one workflow under different circumstances, compared here because each is the closest committed precedent for a different set of circumstances.

**C2** (Capability Definition → Architecture Evaluation → Architectural Resolution → Implementation Authorization, in full) required both stages because it reached a genuinely untested architectural question named fifteen days earlier and deliberately left open — `SPRINT12_ARCHITECTURAL_RESOLUTION.md` was committed 2026-07-22 (`0db391b`), C2's own Architectural Resolution 2026-08-06 (`849eed6`), confirmed by direct `git log` this session: `SPRINT12_ARCHITECTURAL_RESOLUTION.md` §8 stated plainly that no historical decision had yet tested two Rules sharing one already-interpreted `EvidenceCategory`, and cautioned against resolving it in the abstract. C2 was the concrete forcing function that caveat required; its own Architectural Resolution adopted a new Architectural Constraint (Rule Conclusion Non-Contradiction) to close it.

**C1** (Capability Definition → Process Determination → Implementation Authorization, no Architecture Evaluation or Architectural Resolution) proceeded via a two-part test, stated in `PROCESS_DETERMINATION_C1_RECOMMENDATION_STEP_PRESENTATION.md`'s own "Sprint 21 and Sprint 23, Compared" section and applied there directly: (1) no genuinely open design question exists — the governing principle is already Adopted, not deferred; (2) no alternative is being weighed.

**A repository fact confirmed directly this session, distinguished from interpretation:** "Process Determination" does not appear anywhere in `PROJECT_HANDOFF_v1.1.md`'s own canonical workflow list, and a repository-wide search finds it used only in documents referencing C1's own single instance. It is not a canonical lifecycle stage; it is one committed precedent. What is being reapplied here is the two-prong *test* that document performed, per its own stated instruction that the test is reusable, not the document type treated as an established stage in its own right.

---

## Applying the Test to C3

**Prong 1 — is there a genuinely open design question, or is the governing principle already Adopted?**

- `GOVERNANCE.md`'s CLI Crate Boundary Rule — "Owns: user interaction, command execution, platform entry point. Must never contain business logic." — is itself already-adopted architecture, not deferred. Confirmed by direct re-read this session (`GOVERNANCE.md`, "## CLI").
- Its application to C3's own content is not a new interpretation requiring evaluation. Confirmed directly against current source this session: `crates/modiq-cli/src/commands/assess.rs` and `retrieve.rs` already print `Finding::severity()` and `Evidence::category()` via `{:?}` — the exact mechanism, in the exact same functions, C3 would extend to `mod_health_dimension()`, `status()`, `location()`, `label()`, `source()`, and `content()`. This is not reasoning by analogy to a different crate; it is `modiq-cli`'s own, already-exercised practice.
- Sprint 22's own history was checked directly this session, against `ENGINEERING_RELEASE_1.7.md`, `INITIATIVE_3_IMPLEMENTATION_AUTHORIZATION.md`, and current `assess.rs`/`retrieve.rs` source, rather than assumed. `ENGINEERING_RELEASE_1.7.md` records that `Finding`'s constructor changed to accept `title`/`summary`/`mod_health_dimension`/`status` "in place of `description`" — `description()` was removed, not merely supplemented — and that `modiq-cli`'s `assess`/`retrieve`/`history` commands were "updated" as a direct consequence. Direct inspection of current source shows precisely what that update was: `modiq-cli` now prints `finding.title()` and `finding.summary()` in place of the removed `description()` — a compiler-forced migration, required simply to keep compiling, not a voluntary decision to expose new content. `mod_health_dimension` and `status`, introduced in that same Sprint 22 change, were never printed then and are still not printed today, confirmed by the same source inspection — no forcing function required them. `INITIATIVE_3_IMPLEMENTATION_AUTHORIZATION.md`'s own reference to `modiq-cli` (§ blast-radius discussion) concerns a separate compatibility question (`Recommendation` becoming `Option`), not authorization or evaluation of new field presentation. **This is not, therefore, precedent for a voluntary field-presentation extension of the shape C3 proposes — it is precedent only for a forced compilation migration, a narrower and different fact.** No repository instance of `modiq-cli` voluntarily adding new, non-forced presentational content exists; C3 would be the first. This bullet is retained to state that fact accurately, not to claim a precedent Sprint 22 does not establish — Prong 1's conclusion below rests on the Crate Boundary Rule's own unqualified text and the in-file mechanism precedent (previous bullet), not on this one.
- The one field this capability adds without a direct in-crate precedent — `Recommendation::repair_steps()`, a `Vec<RecommendationStep>` — is, for a CLI process writing to stdout, a plain nested loop over an already-public getter. Unlike C1's own genuinely open Prong-1 question (a Rust struct crossing an IPC boundary into a new TypeScript DTO shape, requiring a real transport-shape design decision), `modiq-cli` has no serialization or transport boundary to cross at all. This distinguishes C3's residual field-shape gap from C1's own checked-and-resolved one, and resolves more directly, not less: there is no boundary here for a design question to attach to.
- No document reviewed this session, in this capability's own lineage or otherwise, states that `FrontendArchitecture.md`'s principles extend to `modiq-cli`, and none needs to: `modiq-cli` has its own, already-adopted governing principle (the Crate Boundary Rule), whose own text sets no limit on how many already-public fields may be formatted, and whose "no business logic" prohibition is not implicated by printing more already-computed data through the same mechanism `modiq-cli` already uses today.

**Prong 2 — is any alternative being weighed?**

No. C3 proposes no new formatting mechanism, no reinterpretation of the CLI Crate Boundary Rule, no plugin or extension model for `modiq-cli`'s own output, and no change to which crate owns which responsibility. It extends an already-exercised practice to fields the Capability Definition's own (reconciled) Capability Statement enumerates precisely, per command — `assess.rs` in full, `retrieve.rs` bounded to what `modiq-storage`'s current persisted mirror carries. There is no decision here to disposition between alternatives.

---

## Determination

**C3 matches Sprint 23's/C1's path on both prongs of the test.** No genuinely open design question exists — the governing principle (`GOVERNANCE.md`'s CLI Crate Boundary Rule) is Adopted, not deferred, its text sets no field-count limit, and its "no business logic" prohibition is satisfied by construction for every field this capability presents. `modiq-cli`'s own existing use of the identical Debug-format mechanism, for the fields it currently prints, further confirms no new formatting idiom is required — this is precedent for the mechanism, not (as Sprint 22's own history shows on direct inspection) for a prior voluntary field-presentation extension, since no such instance yet exists in this crate's history. No alternative is being weighed.

**The next required repository artifact is an Implementation Authorization**, not an Architecture Evaluation or Architectural Resolution.

---

## What This Document Does Not Do

- It does not draft the Implementation Authorization itself.
- It does not define implementation boundaries, participating files, exclusions, invariants, verification gates, or completion criteria — those belong to the Implementation Authorization.
- It does not design a format string, an output layout, or a Sprint Plan.
- It does not reopen, revise, or reinterpret `GOVERNANCE.md`'s CLI Crate Boundary Rule, `FrontendArchitecture.md`'s own stated scope, or the Capability Definition's own reconciled `retrieve.rs`/`modiq-storage` boundary — every conclusion above traces to something already Adopted, or already confirmed by direct source inspection recorded in the Capability Definition itself.

---

## Note on This Document's Own Standing

This document is the second committed instance of a repository precedent established once, by C1, not an invocation of a permanently canonical lifecycle stage — `PROJECT_HANDOFF_v1.1.md` §5's own workflow names no such stage, confirmed by direct re-read this session. What is reused here is the two-prong test itself, which `PROCESS_DETERMINATION_C1...md` explicitly frames as reusable ("not a newly invented one"), applied fresh against C3's own repository evidence rather than assumed to transfer from C1 by simple analogy. A Technical Director assessment reaching the same conclusion was produced earlier in this engineering session; it was conversational only and was never committed as a repository artifact, so it is not cited here as governing evidence — every claim in this document was independently re-verified against current repository source in the course of drafting it.

---

## Next Required Repository Artifact

An Implementation Authorization for C3 — `modiq-cli` Field Parity, following the per-command scope the Capability Definition's own Capability Statement establishes: full field parity for `assess.rs`, and a `retrieve.rs` scope bounded to exactly what `modiq-storage`'s current persisted mirror (`location`, `repair_steps`) carries.

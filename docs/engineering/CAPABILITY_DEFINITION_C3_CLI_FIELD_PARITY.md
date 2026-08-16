# Capability Definition — C3: `modiq-cli` Field Parity

| Property | Value |
|---|---|
| **Document** | CAPABILITY_DEFINITION_C3_CLI_FIELD_PARITY.md |
| **Project** | modIQ |
| **Type** | Capability Definition — defines the capability only. Not an Architecture Evaluation, Architectural Resolution, Implementation Authorization, Sprint Plan, or roadmap. |
| **Origin** | `docs/engineering/CAPABILITY_PORTFOLIO_ASSESSMENT.md` (`ca11328`), which classified this candidate ("C3") Category A: "mechanical extension of an already-public Runtime getter into already-existing CLI output formatting; no architectural question of any kind." Treated as fixed, unreopened. |

---

## Purpose

This document answers one question: what capability is being added to the platform? It does not decide how that capability is implemented, whether it requires an Architecture Evaluation, what Rust types or formatting changes would carry it, what a Sprint Plan would contain, or in what order any of that work would occur. Those determinations belong to whatever repository artifact follows this one.

---

## Repository Context

`docs/engineering/PLATFORM_FOUNDATION_V1_DECLARATION.md` and `docs/engineering/POST_FOUNDATION_ENGINEERING_CAPABILITY_ASSESSMENT.md` remain settled, per `PROJECT_HANDOFF_v1.1.md` §2–3. `CAPABILITY_PORTFOLIO_ASSESSMENT.md` surveyed twelve capability candidates against that settled architecture; C1 (`RecommendationStep` presentation) and C2 (Declared Dependency Interpretation) — the two strongest Category A candidates — have both since completed (`ENGINEERING_RELEASE_2.0.md`, `ENGINEERING_RELEASE_2.1.md`). This document is the first artifact produced for C3, the next candidate the completed C1/C2 history and the Portfolio Assessment's own remaining classification converge on.

Every factual claim below was checked directly against current repository source during this session, not carried forward from the Portfolio Assessment's own characterization alone — that document is twelve days old as of this drafting (`ca11328`, 2026-08-04), from the same engineering era as C1 and C2, not stale history requiring independent re-confirmation on that basis; it is re-derived here because repository discipline requires it, not because the source is old.

---

## Capability Statement

**`modiq-cli assess` presents, in its own text output, the Runtime fields `apps/console` has presented since Sprint 22, Sprint 23, Sprint 24, and C1 — `Finding`'s Mod Health dimension and status, `Evidence`'s location and label/source/content provenance, and `Recommendation`'s per-step repair structure — none of which `modiq-cli` prints today. `modiq-cli retrieve` presents as much of that same content as `modiq-storage`'s own persisted mirror currently carries — `Evidence`'s location and `Recommendation`'s per-step repair structure — and no more; extending `retrieve`'s own coverage to match `assess` in full is a separate, deferred question (see Explicit Exclusions), not this capability's own scope.**

This statement was corrected during this document's own Repository Validation Review and Reconciliation. The original draft claimed uniform parity across both commands; direct source inspection shows `assess.rs` and `retrieve.rs` read from two structurally different type hierarchies, not one, and the claim did not hold for `retrieve.rs` as originally stated.

Confirmed directly, this session, against current source. `crates/modiq-cli/src/commands/assess.rs`'s and `crates/modiq-cli/src/commands/retrieve.rs`'s own `format_report` functions — structurally near-identical, each independently formatting the same three collections — print exactly:

- Evidence: `[{:?}] {description}` — `category()` and `description()` only.
- Findings: `[{:?}] {title}: {summary}` — `severity()`, `title()`, and `summary()` only.
- Recommendations: `{action}` — `action()` only.

`assess.rs` operates on live `modiq-runtime` types. `Finding::mod_health_dimension()` and `Finding::status()` (`crates/modiq-runtime/src/assessment/finding.rs`), `Evidence::location()`, `Evidence::label()`, `Evidence::source()`, and `Evidence::content()` (`crates/modiq-runtime/src/assessment/evidence.rs`), and `Recommendation::repair_steps()` together with `RecommendationStep::kind()`/`::instruction()` (`crates/modiq-runtime/src/assessment/recommendation.rs`, `recommendation_step.rs`) are all confirmed public and already populated on these types — the identical getters `apps/console`'s own transport layer already reads.

`retrieve.rs` operates instead on `modiq-storage`'s own persisted mirror (`crates/modiq-storage/src/storage/persisted_report.rs`), confirmed directly this session to carry a materially smaller field set: `PersistedEvidence` exposes only `category()`, `description()`, and `location()` — no `label()`, `source()`, or `content()`. `PersistedFinding` exposes only `severity()`, `title()`, `summary()`, `evidence_indices()`, and `rule_reference()` — no `mod_health_dimension()` or `status()`. `PersistedRecommendation` does carry `repair_steps()` (mirrored since Sprint 24). This is independently corroborated by two already-committed repository sources, not inferred here for the first time: `CAPABILITY_DEFINITION_C2_DECLARED_DEPENDENCY_INTERPRETATION.md` states directly that "`modiq-storage`'s `PersistedEvidence` also carries only `category`, `description`, `location` — no `label`," and `ENGINEERING_RELEASE_1.7.md` records that only `PersistedFinding`'s `title`/`summary` were mirrored into Storage at Sprint 22 — Mod Health dimension, status, and Evidence provenance were extended only into `apps/console`'s own DTOs, never into `modiq-storage`.

---

## User Value

A `modiq-cli` user sees a Finding's severity, title, and summary, and a bare Recommendation sentence — the same content `apps/console` presented before Sprint 22. Since Sprint 22, `apps/console` has additionally shown which Mod Health dimension a Finding concerns and whether it is Provisional or Final; since Sprint 23, which Evidence item a Finding's own label/source identify; since C1, each Recommendation's own per-step repair guidance. A `modiq-cli assess` user — anyone running a fresh Assessment outside the GUI, including any scripted or CI-driven use — sees none of this today, and this capability closes that gap in full. A `modiq-cli retrieve` user — anyone inspecting a previously-stored report — sees a narrower gap closed: location and per-step repair guidance, the two fields `modiq-storage`'s own persisted mirror already carries. The remainder (Mod Health dimension, status, label/source/content) is real, un-closed value for `retrieve` specifically, gated on `modiq-storage`'s own future extension, not on anything this capability decides. The gap this capability does close is not new judgment or new content; every field it presents already exists, already public, already correct. It is a presentation gap in one specific consumer, unchanged since that consumer was first written.

---

## Existing Repository Foundation

- **Every field this capability presents on `modiq-runtime`'s own live types already exists, is already public, and is already correct** — confirmed by direct `grep` against `finding.rs`, `evidence.rs`, `recommendation.rs`, and `recommendation_step.rs` this session. No `modiq-runtime`, Rule, or Collector change of any kind is implicated. This is not, however, uniformly true of what `modiq-storage`'s own persisted mirror carries — see the next bullet.
- **`modiq-storage`'s own persisted mirror (`PersistedFinding`, `PersistedEvidence`) does not carry every field this capability presents, a real constraint specific to `retrieve.rs`.** Confirmed directly this session against `crates/modiq-storage/src/storage/persisted_report.rs`: `PersistedEvidence` carries `category`, `description`, `location` only; `PersistedFinding` carries `severity`, `title`, `summary`, `evidence_indices`, `rule_reference` only. Mod Health dimension, status, and Evidence's label/source/content were never mirrored into Storage — confirmed independently by `CAPABILITY_DEFINITION_C2_DECLARED_DEPENDENCY_INTERPRETATION.md`'s own direct statement to this effect, and by `ENGINEERING_RELEASE_1.7.md`'s own record that Sprint 22 extended only `PersistedFinding`'s `title`/`summary`, never its Mod Health dimension or status, and extended Evidence provenance only into `apps/console`'s own DTOs. `PersistedRecommendation` does carry `repair_steps` (Sprint 24). This is why the Capability Statement above scopes `retrieve.rs` more narrowly than `assess.rs`.
- **`modiq-cli` already uses the exact formatting convention this capability would extend, in the same two functions.** `assess.rs`'s and `retrieve.rs`'s own `format_report` already print `category()` and `severity()` via `{:?}` — Rust's derived `Debug` representation of a closed enum — the identical convention `apps/console`'s own `Reviewing.tsx` uses (per `CAPABILITY_DEFINITION_C1...md`: "The `Debug`-format string convention already used for `severity`, `mod_health_dimension`, and `status`"). This capability does not introduce a new formatting idiom; it extends an idiom `modiq-cli` itself already applies to two of the fields in scope, to the remainder each command can actually reach.
- **Both call sites — `assess.rs` and `retrieve.rs` — independently duplicate the same `format_report` logic today, for the fields both currently print.** Confirmed by direct comparison of both functions this session: identical field selection, identical format strings, identical structure, differing only in their surrounding command context. This structural identity holds for today's reduced field set (`category`/`description`, `severity`/`title`/`summary`, `action`); it does not extend to every field this capability adds, since the two commands' own underlying types now diverge (see above).
- **`GOVERNANCE.md`'s CLI Crate Boundary Rule** — "Owns: user interaction, command execution, platform entry point. Must never contain business logic." — is the only crate-specific architectural text governing `modiq-cli` at all. Formatting an already-public getter as text is not business logic under any reading this repository has applied to `modiq-cli`'s own existing formatting code (which already does exactly this for `category()` and `severity()`).
- **`FrontendArchitecture.md` does not govern `modiq-cli`.** Confirmed directly, this session: the document's own Purpose and Constitutional Foundation state it defines "the architecture of modIQ's production interaction layer — the consumer application through which a user experiences an Assessment," and Initiative 5's own Architectural Resolution (cited throughout that document) settled that boundary specifically for the production interaction layer, which Initiative 5 itself determined to be `apps/console`, not `modiq-cli`. C1's own presentation-layer principles (Presentation Without Inference, Explainable Continuity, Consumer-Owned State, Boundary Enforcement) are `FrontendArchitecture.md`'s own text, adopted for that document's own named scope — this capability cannot cite them as already-governing without independent confirmation that they, or an equivalent, extend to `modiq-cli`. No document reviewed this session states that they do, or that they do not; none was found to address `modiq-cli`'s own presentation formatting at all, beyond the Crate Boundary Rule already quoted above.
- **The C1/Sprint 23 precedent this capability's own shape most resembles.** C1's own Capability Definition named Sprint 23 ("extend an already-adopted architecture with already-produced Runtime content") as the precedent requiring no Architecture Evaluation; Sprint 23 itself proceeded directly to an Implementation Authorization, per that document's own text. C1, in turn — not Sprint 23 — checked its own match to that precedent through a dedicated Process Determination, a novel artifact C1 itself introduced rather than one Sprint 23 required, that re-derived, rather than assumed, the match. This capability's own construction shape — read an already-public getter, format it as already-established text — is a plainer instance of the same pattern, with one difference from C1 named directly above: C1 could point to `FrontendArchitecture.md` as the already-adopted architecture governing its own consumer; this capability has no equivalent document to point to, only the thinner Crate Boundary Rule.

---

## Existing Architectural Principles

- **`GOVERNANCE.md`'s CLI Crate Boundary Rule** (quoted above) — the only principle specific to this crate; not extended or reinterpreted by this capability, which adds no business logic of any kind.
- **ADR-0007 (Runtime Entity Design Pattern).** Every field this capability would present is a plain, already-fixed value on an already-infallibly-constructed Runtime entity; this capability presents that value, it does not reinterpret, recompute, or infer anything about it.
- **`RuleEngine.md`'s Explainability principle** ("every conclusion should be understandable... traceable") is satisfied by construction for every field in scope — each is already the direct output of an existing Rule or Collector, unchanged by how `modiq-cli` chooses to print it.
- **Storage's own persistence boundary** (`GOVERNANCE.md`'s Storage Crate Boundary Rule: "durability of an already-generated `AssessmentReport`... populated only from `AssessmentReport`'s already-public API") is the reason `retrieve.rs` cannot reach fields `modiq-storage` never chose to mirror. This capability does not reinterpret that boundary; it is the reason `retrieve.rs`'s own scope is narrower than `assess.rs`'s, not a question this capability resolves.

---

## Existing Repository Patterns

- **The `{:?}` Debug-format convention**, already used by `modiq-cli` itself (`category()`, `severity()`) and by `apps/console` (`severity`, `mod_health_dimension`, `status`) — the direct precedent for presenting `mod_health_dimension()` and `status()` as text (`assess.rs` only, per the Capability Statement's own correction above).
- **Evidence's optional fields** (`location`, `label`, `source`, `content`) are each `Option<&str>` — `apps/console`'s own `Reviewing.tsx` renders each only when present; the same presence-conditional pattern is available to `modiq-cli`, not a new one this capability would invent. `location` is the one of the four already present on `PersistedEvidence` as well as on live `Evidence` — confirmed directly against `persisted_report.rs` this session — making it reachable from both `assess.rs` and `retrieve.rs` without exception, unlike `label`/`source`/`content`.
- **`Recommendation::repair_steps()` returning an empty `Vec` when no Repair Recipe informed a Recommendation** — already the established "valid, expected outcome, not a gap to fill" pattern (`CAPABILITY_DEFINITION_C1...md`'s own "What Success Does Not Require"), directly reusable here.

---

## Architectural Boundaries

The following must remain unchanged by this capability, as a direct consequence of already-adopted architecture:

- The CLI Crate Boundary Rule itself — `modiq-cli` remains user interaction, command execution, and platform entry point only; no business logic is introduced by formatting already-public getters.
- No `modiq-runtime`, `modiq-rules`, `modiq-collection`, `modiq-versioning`, `modiq-engine`, `modiq-report`, `modiq-storage`, or `modiq-knowledge` file — every field this capability presents already exists on the type each command actually reads, at the corrected, per-command scope the Capability Statement now states. `modiq-storage` remaining untouched is a direct consequence of `retrieve.rs`'s own scope being bounded to what `PersistedFinding`/`PersistedEvidence` already carry, not an independent claim that every field is available everywhere.
- `apps/console`'s own presentation — unaffected; this capability touches no shared code between `modiq-cli` and `apps/console`, which maintain entirely separate transport and formatting layers.
- `modiq-cli`'s own `history.rs` command (Sprint 20's aggregate `(rule_reference, severity)` analysis) — a different data shape (cross-report aggregation, not per-Finding/Evidence/Recommendation presentation), out of scope for this capability.

---

## Explicit Exclusions

Intentionally outside this capability's own scope:

- Any Rust type, formatting design, or output-format change beyond field selection — reserved for Implementation Planning.
- **`apps/sandbox` field parity** (the Capability Portfolio Assessment's own C4) — a separate, real, deferred candidate. Not combined with this capability: `apps/sandbox` is formally disqualified from serving as the production interaction layer (Initiative 5, Decision 6, per `PROJECT_STATUS.md`), a fact with no analogue for `modiq-cli`, giving the two candidates different standing even though both were named Category A on the same basis.
- Any change to `modiq-cli`'s own `history.rs` command or `modiq-storage`'s aggregate analysis.
- Any change to `modiq-runtime`, `modiq-rules`, `modiq-collection`, `modiq-versioning`, `modiq-engine`, `modiq-report`, `modiq-storage`, or `modiq-knowledge`.
- **Extending `modiq-storage`'s own `PersistedFinding`/`PersistedEvidence` to carry Mod Health dimension, status, or label/source/content** — a real, confirmed gap (see Capability Statement, Existing Repository Foundation), named here explicitly as a separate, deferred future candidate rather than resolved silently within this capability's own scope. Reaching it would mean extending `modiq-storage`'s own persisted schema, which `ENGINEERING_RELEASE_1.9.md` already records as carrying a pre-existing, unresolved persistence-migration gap (no `#[serde(default)]`, no schema versioning) — real architectural risk this document does not evaluate or resolve, consistent with this capability's own scope being CLI presentation only, not Storage schema evolution.
- Any resolution of the `assess.rs`/`retrieve.rs` `format_report` duplication named above — a real, observed fact about current source, not a defect this capability is scoped to fix.
- `Confidence`, cross-mod dependency resolution, Lua Analysis, or any other Capability Portfolio Assessment candidate.

---

## Capability Success Criteria

Running `modiq-cli assess` against a report containing a Finding with a non-default Mod Health dimension or status, an Evidence item with a non-`None` `location`/`label`/`source`/`content`, or a Recommendation with a non-empty `repair_steps`, shows that content in the command's own text output — distinguishable from the fields already printed today — sourced with no fact introduced by `modiq-cli` itself.

Running `modiq-cli retrieve` against a stored report containing an Evidence item with a non-`None` `location`, or a Recommendation with a non-empty `repair_steps`, shows that content identically. `retrieve`'s own success criterion is deliberately narrower than `assess`'s, bounded to exactly what `modiq-storage`'s current persisted mirror carries — not a lesser standard, but the correct one given what repository evidence shows is actually reachable without touching `modiq-storage`.

---

## What Success Does Not Require

- It does not require every Evidence item to carry a label, source, or content — each remains optional; a report with none present is a valid, expected outcome.
- It does not require every Recommendation to carry `repair_steps` — only `VersionCompatibilityRule` currently populates it; an empty `Vec` is a valid, expected outcome, not a gap to fill.
- It does not require `apps/sandbox` to change.
- It does not require resolving the `assess.rs`/`retrieve.rs` duplication.
- It does not require any change to `modiq-cli`'s own `history.rs` command.
- **It does not require `modiq-cli retrieve` to show Finding's Mod Health dimension or status, or Evidence's label/source/content.** `modiq-storage`'s own persisted mirror does not currently carry them, and extending it is explicitly excluded from this capability's own scope (see Explicit Exclusions). This is a genuine scope boundary, not an oversight — repository evidence (the persisted-type definitions themselves) leaves no way to satisfy a broader criterion for `retrieve` without touching a crate this capability does not authorize touching.

---

## Repository Impact

At the capability-definition level, not the implementation level: this capability implicates `modiq-cli` only — specifically `assess.rs` and `retrieve.rs`, at the per-command scope the Capability Statement now states. No `modiq-*` crate other than `modiq-cli`, no `apps/console`, no `apps/sandbox`, no `Cargo.toml`, no ADR, and no Governance Register item is implicated by the capability itself, matching the Capability Portfolio Assessment's own Category A classification. This holds because `retrieve.rs`'s own scope was corrected to fit within what `modiq-storage` already provides, not because the originally-stated full-parity goal was independently confirmed to require no `modiq-storage` change.

---

## Next Required Repository Artifact

Repository precedent offers the same structural fork C1's own Capability Definition named: proceed through a full Architecture Evaluation and Architectural Resolution (Sprint 21's own path, and C2's), or proceed directly toward an Implementation Authorization once a lighter-weight check confirms no genuinely open question remains (Sprint 23's path, and C1's own, via a dedicated Process Determination).

Repository evidence assembled in this document supports the latter path, though this document does not itself perform that determination. The mechanical operation this capability requires, at its now-corrected scope — reading an already-public getter and formatting it via a convention `modiq-cli` already applies to two of the fields in scope, on types each command can actually reach without crossing into `modiq-storage` — raises no architectural question of the shape C2 encountered (no new Rule, no shared `EvidenceCategory` interpretation, no Evidence Collection boundary question). This document's own Repository Validation Review and Reconciliation surfaced a real, related question — whether `modiq-storage`'s own incomplete field mirror should itself be extended — and this document resolves that question by scope, not by architecture: extending Storage's persisted schema is named as a separate, deferred, explicitly excluded candidate (Explicit Exclusions), carrying its own real risk (`modiq-storage`'s pre-existing persistence-migration gap), not part of what this capability now claims. With that question set aside rather than left implicit, this capability's own remaining scope is, if anything, cleaner than before the reconciliation: entirely within `modiq-cli`, entirely mechanical, with no crate-boundary ambiguity remaining.

The one genuine asymmetry with C1's own cleaner case is still named directly above and still not resolved here: `modiq-cli`, unlike `apps/console`, has no dedicated architecture document establishing presentation principles for it, only `GOVERNANCE.md`'s thinner Crate Boundary Rule — meaning this capability cannot cite an already-adopted document the way C1 cited `FrontendArchitecture.md`'s Consumer-Owned State. Whether that asymmetry itself constitutes a question requiring an Architecture Evaluation, or is adequately closed by the Crate Boundary Rule together with `modiq-cli`'s own existing formatting precedent, is not decided by this document. Given the shape of the open question (whether an existing, adopted document's principles already cover an instance not yet tested against it — not whether new architecture must be designed), a Process-Determination-shaped check, mirroring C1's own, is the better-fitted next artifact than a full Architecture Evaluation — but that determination, like C1's own, belongs to whichever artifact follows this one, not to this document.

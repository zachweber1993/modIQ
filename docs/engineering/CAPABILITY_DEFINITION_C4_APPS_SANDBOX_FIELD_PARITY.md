# Capability Definition — C4: `apps/sandbox` Field Parity

| Property | Value |
|---|---|
| **Document** | CAPABILITY_DEFINITION_C4_APPS_SANDBOX_FIELD_PARITY.md |
| **Project** | modIQ |
| **Type** | Capability Definition — defines the capability only. Not an Architecture Evaluation, Architectural Resolution, Implementation Authorization, Sprint Plan, or roadmap. |
| **Origin** | `docs/engineering/CAPABILITY_PORTFOLIO_ASSESSMENT.md`, which names this candidate "C4 — Extend `apps/sandbox` for field parity with Sprint 22/23/24's Runtime additions," classifies it Category A ("Same shape as C3; repository itself has already named it available with no forcing function"), and cites `ENGINEERING_RELEASE_1.7.md`/`ENGINEERING_RELEASE_1.8.md` as prior, repeated namings of this exact gap. Treated as fixed, unreopened. |

---

## Purpose

This document answers one question: what capability is being added to the platform? It does not decide how that capability is implemented, whether it requires an Architecture Evaluation, what Rust or TypeScript types would carry it, what a Sprint Plan would contain, or in what order any of that work would occur. Those determinations belong to whatever repository artifact follows this one.

---

## Repository Context

`CAPABILITY_PORTFOLIO_ASSESSMENT.md` surveyed twelve capability candidates against Platform Foundation Version 1. C1 (`RecommendationStep` presentation in `apps/console`), C2 (Declared Dependency Interpretation), and C3 (`modiq-cli` field parity) — the three Category A candidates the portfolio ranked highest — have all since completed (`ENGINEERING_RELEASE_2.0.md`, `_2.1.md`, `_2.2.md`). This document is the first artifact produced for C4, the next candidate the Portfolio Assessment names in that same sequence.

Every factual claim below was checked directly against current repository source during this session. `feature/runtime-implementation`, HEAD `4c46283` ("docs: Engineering Release 2.2 (C3 complete)"), working tree clean, 27 commits ahead of `origin/feature/runtime-implementation`, 0 behind.

**A necessary correction to this session's own starting instruction, recorded here because it materially affects this document's own Origin.** This session was initially directed to treat "C4" as "Repair Guidance." Direct repository inspection found both premises false: (1) Repair Guidance is not an open candidate — it was fully designed, resolved, implemented, and closed as **Sprint 9** (`SPRINT9_CAPABILITY_DEFINITION.md` → `SPRINT9_ARCHITECTURAL_RESOLUTION.md` → `SPRINT9_REPOSITORY_REVIEW.md` → commit `21eb7eb`), then extended by Sprint 22 (`969e595`) and Sprint 24 (`84ef4e3`); `VersionCompatibilityRule` today already calls `RepairRecipe::version_compatibility_declared_version_mismatch()` and already constructs `Some(RepairRecipeReference::new(...))` with non-empty `repair_steps` — confirmed directly against `crates/modiq-rules/src/rules/version_compatibility_rule.rs` and its own passing tests. (2) "C4" is not an available name — `CAPABILITY_PORTFOLIO_ASSESSMENT.md`, the same document underlying C1–C3, already assigns it to `apps/sandbox` field parity. This was raised to, and resolved by, the user directly before this document was drafted: proceed with the real C4 (`apps/sandbox` field parity), not Repair Guidance. This capability's own content below is unaffected by that correction — it concerns only which candidate this document defines.

---

## Capability Statement

**`apps/sandbox`'s Tauri IPC layer presents, in its `AssessmentSummary` and `PersistedReportSummary` transport, the Runtime/Storage fields `apps/console` has presented since Sprint 22, Sprint 23, Sprint 24, and C1 — a Finding's Mod Health dimension and status, an Evidence item's label/source/content provenance, and a Recommendation's per-step repair structure — none of which `apps/sandbox` transports today.**

Confirmed directly, this session, against current source. `apps/sandbox/src-tauri/src/lib.rs` defines six DTO structs, each a `#[derive(serde::Serialize)]` snapshot built via a `From<&T>` impl over an already-public Runtime or Storage getter:

- `EvidenceEntry` (from `&Evidence`): `id`, `category`, `description`, `location`. Does not read `Evidence::label()`, `::source()`, or `::content()` — each already `Option<&str>`, already public.
- `FindingEntry` (from `&Finding`): `id`, `severity`, `title`, `summary`. Does not read `Finding::mod_health_dimension()` or `::status()` — each already public, non-optional.
- `RecommendationEntry` (from `&Recommendation`): `id`, `action`. Does not read `Recommendation::repair_steps()` — already `&[RecommendationStep]`, already public; each step's own `kind()`/`instruction()` are likewise already public.
- `PersistedEvidenceEntry` (from `&PersistedEvidence`): `category`, `description`, `location`. `PersistedEvidence` itself carries no `label`/`source`/`content` — confirmed against `crates/modiq-storage/src/storage/persisted_report.rs`, the same Storage-layer gap C3 already found and left unresolved.
- `PersistedFindingEntry` (from `&PersistedFinding`): `severity`, `title`, `summary`. `PersistedFinding` itself carries no `mod_health_dimension`/`status` — the same gap.
- `PersistedRecommendationEntry` (from `&PersistedRecommendation`): `action` only. Does not read `PersistedRecommendation::repair_steps()` — which, unlike the two fields above, **does** exist on `PersistedRecommendation` (mirrored into Storage since Sprint 24) and is simply not yet read into this DTO. This is the persisted-side counterpart to what C3 closed for `modiq-cli retrieve`.

Every field this capability would add already exists, is already public, and is already correct on the type each DTO already converts from. Nothing is inferred, computed, or newly authored — the identical premise C3's own Capability Definition established for `modiq-cli`.

---

## User Value

A `apps/sandbox` user (an engineer exercising the Sandbox's own `create_assessment`/`retrieve_report` Tauri commands, or reading `App.tsx`'s rendered output) sees a Finding's severity, title, and summary, an Evidence item's category/description/location, and a bare Recommendation action — the reduced field set every consumer showed before Sprint 22. `apps/console` has since shown, for the same underlying data, which Mod Health dimension a Finding concerns, whether it is Provisional or Final, an Evidence item's label/source/content provenance, and each Recommendation's own per-step repair guidance. `apps/sandbox` shows none of this today. This capability closes that gap — narrower value than C1's own (which reached a real, still-in-use production consumer) or C2's (new-to-every-consumer Evidence), consistent with `CAPABILITY_PORTFOLIO_ASSESSMENT.md`'s own characterization: "`C3` and `C4` deliver completeness and parity, not new user-facing insight." This document does not dispute that ranking; C4 was not selected here on a claim of higher value than C1–C3, but as the next candidate in the Portfolio Assessment's own sequence, per the user's own direction.

---

## Existing Repository Foundation

- **Every field this capability presents on `modiq-runtime`'s own live types already exists, is already public, and is already correct** — the identical getters `apps/console`'s own transport already reads (`apps/console/src-tauri/src/assessment.rs`, confirmed directly this session: `EvidenceSummary` reads `label()`/`source()`/`content()`; `FindingSummary` reads `mod_health_dimension()`/`status()`; `RecommendationSummary` reads `repair_steps()` via a `RecommendationStepSummary` mirroring `kind()`/`instruction()`). No `modiq-runtime`, Rule, or Collector change of any kind is implicated.
- **`modiq-storage`'s own persisted mirror does not carry every field this capability would present, a real constraint specific to the persisted (`retrieve_report`) half.** `PersistedEvidence` carries `category`, `description`, `location` only; `PersistedFinding` carries `severity`, `title`, `summary`, `evidence_indices`, `rule_reference` only — the same gap `CAPABILITY_DEFINITION_C3_CLI_FIELD_PARITY.md` already found and left explicitly deferred. `PersistedRecommendation` does carry `repair_steps` (mirrored since Sprint 24) — reachable, simply not yet read into `PersistedRecommendationEntry`.
- **`apps/console`'s own `assessment.rs` is the existing, real, tested precedent for presenting exactly this field set** — `EvidenceSummary`, `RecommendationStepSummary`, `RecommendationSummary`, `FindingSummary` (lines 41–121) already carry `label`/`source`/`content`, `repair_steps`, `mod_health_dimension`, `status`, each via the identical `From<&T>` conversion-struct pattern `apps/sandbox`'s own DTOs already use for the fields they currently carry. This capability activates that same pattern's second, independent instance, not a new one.
- **A real structural divergence exists between the two consumers' own transport shape, confirmed directly this session, not assumed.** `apps/console`'s `ReportSummary` nests `evidence` and `recommendation` inside each `FindingSummary` — "presentation code receives Evidence and Recommendation already scoped to the Finding they belong to" (`assessment.rs`, module doc). `apps/sandbox`'s `AssessmentSummary` instead carries three flat, parallel lists (`evidence`, `findings`, `recommendations`), matching `AssessmentReport`'s own flat shape directly, with no Finding-scoped nesting performed at the DTO boundary. Which shape this capability's own new fields should follow — extend the existing flat shape in place, or adopt `apps/console`'s nested-by-Finding shape — is a real design fork this document does not resolve (see Explicit Exclusions).
- **A pre-existing drift between `apps/sandbox`'s own Rust DTO and its own TypeScript interface was found this session, not introduced by anything proposed here.** `App.tsx`'s `FindingEntry` interface (line 17–21) declares a `description: string` field; the Rust `FindingEntry` struct it is meant to mirror (`lib.rs`, line 35–40) has carried `title`/`summary` instead of `description` since Sprint 22 (`ENGINEERING_RELEASE_1.7.md`, confirmed: "`apps/sandbox`'s `FindingEntry`/`PersistedFindingEntry` updated" at that Sprint). `App.tsx` currently renders `item.description` for each Finding, which resolves to `undefined` at runtime against the actual JSON payload. This is recorded as an established fact discovered by this session's own falsification pass; whether and how to correct it is not decided here.
- **`apps/sandbox` has no dedicated `GOVERNANCE.md` Crate Boundary Rule section, confirmed by direct, full re-read this session.** `GOVERNANCE.md`'s Crate Boundary Rules name Runtime Domain, Rule Engine, Evidence Collection, Reporting, Knowledge Domain, Engine, CLI, Console, and Storage — nine sections, none titled "Sandbox" and none of those nine naming `apps/sandbox`. `apps/sandbox` does occur elsewhere in the document, in Governance Register material unrelated to any crate boundary (GOV-007, GOV-017) — neither instance is, or substitutes for, a Crate Boundary Rule. This is a sharper version of the gap C3 found for `modiq-cli` (which at least had a three-line "## CLI" section to confirm adequate) — here, no boundary-rule text exists to confirm adequate or inadequate.
- **`FrontendArchitecture.md` does not govern `apps/sandbox`, confirmed directly this session.** Its own Purpose states it defines "the architecture of modIQ's production interaction layer — the consumer application through which a user experiences an Assessment," and its authority derives from Initiative 5's own Architectural Resolution, which determined that layer to be `apps/console`. The document's full text contains no reference to "sandbox" anywhere. This mirrors C3's own finding for `modiq-cli` exactly (same document, same non-coverage), except `apps/sandbox` additionally lacks the `GOVERNANCE.md` fallback `modiq-cli` had.
- **Initiative 5, Decision 6 (Adopted) directly addresses `apps/sandbox`'s own standing, though not its field parity specifically.** "`apps/sandbox`, as it exists today, cannot be the production interaction layer; its request/response and getter-based DTO pattern is validated precedent, not a technology endorsement." This settles that `apps/sandbox` will not become the primary consumer, but does not settle whether its own, real, currently-exercised transport should stay behind the fields its sibling consumer already presents — a distinct question this capability answers in the affirmative, consistent with `CAPABILITY_DEFINITION_C3...md`'s identical reasoning for `modiq-cli`, which carries the same non-production standing.
- **`apps/sandbox` is a separate Cargo workspace with its own test suite, currently 9/9 passing** (`apps/sandbox/src-tauri`), unaffected by the C1–C3 lineage's own root-workspace changes at every prior release — confirmed via this session's own fresh `cargo test --workspace` run inside that workspace.
- **The `{:?}` Debug-format convention this capability would extend is already `apps/sandbox`'s own established practice** — `EvidenceEntry`/`FindingEntry`/`PersistedEvidenceEntry`/`PersistedFindingEntry` already format `category()`/`severity()` this way (`lib.rs`, confirmed directly). This capability does not introduce a new formatting idiom on the Rust side.
- **`apps/sandbox`'s own standing "no file-picker, no new input mechanism" constraint** (`PROPOSAL_FILESYSTEM_COLLECTION.md`, Sandbox Interaction) is unaffected by this capability — presenting additional already-collected fields implies no new interaction surface.

---

## Existing Architectural Principles

- **`GOVERNANCE.md`'s Storage Crate Boundary Rule** — `PersistedEvidence`/`PersistedFinding`/`PersistedRecommendation` are "populated only from `AssessmentReport`'s already-public API," unchanged by this capability; `apps/sandbox`'s own persisted-path DTOs may present only what Storage already carries, exactly as C3 bounded `modiq-cli retrieve`.
- **ADR-0007 (Runtime Entity Design Pattern)** — every field in scope is a plain, already-fixed value on an already-infallibly-constructed Runtime entity (or, for `repair_steps`, the value-object `RecommendationStep` ADR-0007 already excludes from its four identity-bearing entities). This capability presents that value; it does not reinterpret, recompute, or infer anything about it.
- **`RuleEngine.md`'s Explainability principle** is satisfied by construction for every field in scope, identically to C3's own reasoning — each is already the direct output of an existing Rule or Collector.
- **No architectural document currently assigns `apps/sandbox` a Crate Boundary Rule of its own** (see Existing Repository Foundation) — the only applicable governing text is `GOVERNANCE.md`'s general Documentation Authority and Governance Principles, not any Sandbox-specific boundary statement. Whether this absence is adequate for a capability that only formats already-public getters as IPC-safe fields, or whether it requires closing first, is named here as an open question, not resolved.

---

## Existing Repository Patterns

- **The Transport Mirror pattern**, already twice-demonstrated: `apps/console`'s `assessment.rs` (Sprint 21 → C1) and `modiq-cli`'s `format_report` (C3) both convert an already-public Runtime or Storage getter into a consumer-owned, IPC- or text-safe representation, without reinterpreting the value. `apps/sandbox`'s own `EvidenceEntry`/`FindingEntry`/`RecommendationEntry`/`Persisted*Entry` structs already are this pattern, for the reduced field set they currently carry — this capability extends an already-exercised practice, not a new one.
- **Evidence's optional fields** (`label`, `source`, `content`) are each `Option<&str>` — `apps/console`'s own `EvidenceSummary` already renders each only when `Some` (`.map(str::to_string)` over an `Option`, serialized as a nullable field); the same presence-conditional pattern is directly reusable for `apps/sandbox`'s own `EvidenceEntry`, which already applies it today for `location`.
- **`Recommendation::repair_steps()` returning an empty `Vec` when no Repair Recipe informed a Recommendation** — already the established "valid, expected outcome, not a gap to fill" pattern (C1's own "What Success Does Not Require," reused directly by C3) — directly reusable here for both `RecommendationEntry` and `PersistedRecommendationEntry`.
- **`RecommendationStepSummary`** (`apps/console/src-tauri/src/assessment.rs`, lines 67–81) is the direct, already-tested precedent for how a per-step repair structure crosses an IPC boundary into a serialized DTO (`kind`, `instruction`, both `String`) — the same shape `apps/sandbox`'s own equivalent would take, though this document does not fix its exact name or module location.

---

## Architectural Boundaries

The following must remain unchanged by this capability, as a direct consequence of already-adopted architecture:

- No `modiq-runtime`, `modiq-rules`, `modiq-collection`, `modiq-versioning`, `modiq-engine`, `modiq-report`, `modiq-storage`, or `modiq-knowledge` file — every field this capability presents already exists on the type each DTO already converts from.
- `apps/console`'s own presentation and transport — unaffected; `apps/sandbox` and `apps/console` maintain entirely separate Cargo crates, Tauri command surfaces, and frontend applications, sharing no transport or formatting code.
- `modiq-cli` — unaffected; a separate consumer, already at field parity as of C3.
- `apps/sandbox`'s own Tauri command surface (`create_assessment`, `retrieve_report`) — no new command, no new argument, no new interaction mechanism (file picker, drag-and-drop, or otherwise).

---

## Explicit Exclusions

Intentionally outside this capability's own scope:

- Any Rust or TypeScript type design, DTO shape, or output-format change beyond field selection — reserved for Implementation Planning.
- **Resolving the structural divergence between `apps/console`'s nested-by-Finding transport and `apps/sandbox`'s own flat, parallel-list transport.** A real, confirmed difference (Existing Repository Foundation), named here explicitly as an open design question, not decided by this document.
- **Extending `modiq-storage`'s own `PersistedFinding`/`PersistedEvidence` schema** to carry `mod_health_dimension`, `status`, or `label`/`source`/`content` — the same real, confirmed, still-unresolved gap C3 already named and left deferred. Not reopened, not resolved, here.
- **Correcting `App.tsx`'s pre-existing `FindingEntry.description` drift** beyond whatever incidental correction naturally follows from touching that same interface to add the new fields — a real, pre-existing fact (Existing Repository Foundation), not a defect this capability is scoped to fix in isolation.
- **Determining whether `GOVERNANCE.md` requires a new "Sandbox" Crate Boundary Rule section.** A real, confirmed absence (Existing Architectural Principles), named here explicitly as an open question for whichever artifact follows this one, not resolved by this document.
- Any new Tauri command, file-picker, drag-and-drop, or other new interaction mechanism for `apps/sandbox`.
- Any change to `apps/console` or `modiq-cli`.
- `Confidence`, cross-mod dependency resolution, Lua Analysis, and every other remaining Capability Portfolio Assessment candidate (C5–C12) — untouched.

---

## Capability Success Criteria

Running `apps/sandbox`'s `create_assessment` command against a fixture producing a Finding with a non-default Mod Health dimension or status, an Evidence item with a non-`None` `label`/`source`/`content`, or a Recommendation with a non-empty `repair_steps`, shows that content in the command's own IPC response — distinguishable from the fields already transported today — sourced with no fact introduced by `apps/sandbox` itself.

Running `apps/sandbox`'s `retrieve_report` command against a stored report containing a Recommendation with a non-empty `repair_steps` shows that content identically, bounded to exactly what `modiq-storage`'s current persisted mirror carries (`repair_steps` only — not `mod_health_dimension`, `status`, `label`, `source`, or `content`, none of which `modiq-storage` mirrors today).

---

## What Success Does Not Require

- It does not require every Evidence item to carry a label, source, or content — each remains optional; a report with none present is a valid, expected outcome.
- It does not require every Recommendation to carry `repair_steps` — only `VersionCompatibilityRule` currently populates it; an empty `Vec` is a valid, expected outcome, not a gap to fill.
- It does not require resolving the `apps/console`/`apps/sandbox` transport-shape divergence.
- It does not require any change to `modiq-storage`'s own persisted schema.
- It does not require `apps/sandbox retrieve_report` to show a Finding's Mod Health dimension or status, or an Evidence item's label/source/content — `modiq-storage`'s own persisted mirror does not currently carry them, and extending it is explicitly excluded from this capability's own scope.
- It does not require correcting `App.tsx`'s pre-existing `description`/`title`+`summary` drift beyond what implementing this capability's own new fields naturally touches.
- It does not require any new Tauri command, UI workflow, or interaction mechanism.

---

## Repository Impact

At the capability-definition level, not the implementation level: this capability implicates `apps/sandbox` only — specifically `apps/sandbox/src-tauri/src/lib.rs` and, if the rendered UI is also extended, `apps/sandbox/src/App.tsx`. No `modiq-*` crate, no `apps/console`, no `modiq-cli`, no `Cargo.toml`, no ADR, and no Governance Register item is implicated by the capability itself — matching the Capability Portfolio Assessment's own Category A classification and C3's own identically-shaped Repository Impact statement.

---

## Next Required Repository Artifact

Repository precedent offers the same structural fork C1's and C3's own Capability Definitions each named: proceed through a full Architecture Evaluation and Architectural Resolution, or proceed directly toward an Implementation Authorization once a lighter-weight check confirms no genuinely open question remains, via a dedicated Process Determination.

This document does not perform that determination. It does, however, surface one fact a Process Determination for C4 would need to weigh that C3's own did not carry in the same form: `apps/sandbox`, unlike `modiq-cli`, has no `GOVERNANCE.md` Crate Boundary Rule section at all, not merely a thin one. Whether that absence is adequately covered by `GOVERNANCE.md`'s general principles and this capability's own mechanical, no-business-logic shape — the same conclusion C3 reached for `modiq-cli`'s own thinner-but-present text — or whether it constitutes a genuine documentation gap requiring closure first (mirroring Sprint 8's own `modiq-versioning` gap-closing amendment), is not decided here. This is named as the one respect in which C4's own path may not simply mirror C3's, not assumed to resolve identically merely because the two candidates share Category A classification and construction shape.

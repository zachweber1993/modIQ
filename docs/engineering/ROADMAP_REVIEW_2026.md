# Roadmap Review 2026

| Property | Value |
|----------|-------|
| **Document** | ROADMAP_REVIEW_2026.md |
| **Project** | modIQ |
| **Purpose** | Planning document — reassess the project against its actual current state and recommend the next major engineering milestones |
| **Prepared by** | Engineering, for Technical Director review |
| **As of** | 2026-07-19, following Sprint 3 Phase 4 (pushed) |
| **Scope** | Planning only. No code changed. No Runtime, Engine, Collection, Rules, or Sandbox modified. No ADRs created. No Governance changed. No documentation outside this file touched. |

---

## Why this document exists

The project has completed nine phases of real work since Sprint 0: Sprint 1, Sprint 2, Sandbox Phases 1–2, and Sprint 3 Phases 1–4. The repository today has nine workspace crates (was eight through Documentation Release 2.0, joined by `modiq-collection` this phase), 106 passing tests, a frozen Documentation Release 2.1, and a working Evidence Collection subsystem boundary with a minimal, honest implementation behind it. None of this existed when the original Sprint roadmap (`CrateRoadmap.md`'s Sprint sequencing, `docs/releases/ENGINEERING_RELEASE_0.2.md`'s deferred-work list) was written. This document evaluates the project as it actually stands today and recommends what should happen next — not what the original plan assumed would happen next.

---

## 1. Current Repository Assessment

**Architectural maturity: high, and load-bearing.** The Platform and Technical Layer specifications (`Architecture.md`, `DataModel.md`, `RuleEngine.md`, `EvidenceCollection.md`) have now been tested against four real implementation phases, one of which introduced an entirely new subsystem and crate. Crate boundaries have held with zero violations across all four phases — no crate ever reached into another's owned state, no orchestration leaked into a domain crate, no domain logic leaked into the Sandbox. The "additive API growth" pattern (`AssessmentService::execute_from_descriptor` added alongside, not replacing, `execute`) is a genuinely new and successful technique this project has now proven once.

**Subsystem maturity: intentionally uneven.** `modiq-runtime`, `modiq-rules`, `modiq-report`, and `modiq-engine`'s `AssessmentService` are L3 and stable. `modiq-collection` is brand new and deliberately minimal — a proven boundary with almost no capability behind it yet. `modiq-knowledge`, `modiq-versioning`, `modiq-cli`, and `modiq-common` are pure L1 scaffolds: every type in all four is a zero-field, zero-method unit struct. This is not neglect — it is nine phases of correctly deferred work, each deferral individually justified at the time. But it means four of nine crates have had literally zero validation against real usage since Sprint 0.

**Documentation maturity: high, with a recurring soft spot.** Two Documentation Releases have shipped since 1.0 (2.0, 2.1), both with clean audits and explicit, non-silent amendment records where Frozen specifications changed. The ADR trail (nine ADRs, all Accepted, all internally consistent) is a genuine asset — architectural reasoning is fully recoverable. The soft spot: living/status documents (`PROJECT_STATUS.md`, `CHANGELOG.md`, `docs/README.md`'s status narrative, `docs/00-Governance.md`) have gone stale after every single release so far, and have been flagged-not-fixed on every occasion they were noticed, because they were never the literal subject of whatever session found them. Confirmed directly this session: `PROJECT_STATUS.md` still states "Current Milestone: Sprint 2 — Complete" and "Next implementation milestone: Sprint 3 — scope not yet selected," despite four Sprint 3 phases having since completed and been pushed.

**Governance maturity: working as designed.** Ten Governance Register items exist; three are Resolved, seven Open; none have been silently dropped or forgotten. The Level 1–4 change-categorization scheme has correctly triaged every decision this project has made under it, including two genuinely Level 4 (Architectural) decisions (the Evidence Collection subsystem, and — implicitly — the crate addition itself) that correctly went through the full ADR + Documentation Release cycle rather than being implemented ad hoc.

**Testing maturity: solid at the unit level, thin elsewhere.** 106 tests, zero ignored, zero flaky, two independently-verified workspaces (root + Sandbox) at every phase gate. Determinism is explicitly tested wherever it matters. The gap: every test in the repository is a Rust unit or integration test. Nothing has ever verified the Sandbox's actual React/TypeScript UI renders or behaves correctly — every session's "Sandbox launches" checkbox has been a manual, unverified item on a resume checklist. Nothing has ever tested adversarial or malformed input, because nothing in the platform has touched untrusted external content yet.

**Strengths:** zero architectural drift across four phases; a proven, repeatable proposal → decision → ADR/documentation → implementation → verification cycle; a demonstrated ability to correctly defer speculative work (Rule trait, Knowledge integration, Version Profiles) under real pressure to build it.

**Remaining risks:** a growing, mutually-dependent governance backlog (GOV-008/009/010); a documentation-staleness pattern that has recurred at every release and shows no sign of self-correcting without a dedicated pass; four completely unvalidated scaffold crates; zero UI-level or adversarial-input test coverage, the latter of which becomes urgent the moment real Evidence Collection begins.

---

## 2. Architecture Review

### `modiq-runtime`
- **Current role:** aggregate root and Runtime Domain owner (Assessment, Evidence, Finding, Recommendation, identity, invariants).
- **Current maturity:** L3, 82 tests, the most mature and most heavily exercised crate in the platform. Untouched since Sprint 3 Phase 2.
- **Missing capabilities:** `Display`/`Serialize` for identity and enum types (flagged at Sandbox Phase 2, flagged again at Sprint 3 Phase 1, still open); referential-integrity enforcement for Finding/Recommendation references (GOV-005/006 follow-up, unnumbered); GOV-001 (report timing) and GOV-002 (invariant doc reconciliation) both bear on this crate directly.
- **Architectural risks:** low. The debt here is small, well-understood, and has not caused a single test failure or design conflict in three phases of neighboring work.
- **Recommended priority:** Low. Stable; don't touch until a concrete forcing function arrives (e.g., Display/Serialize becomes worth it once the Sandbox needs to show identifiers to a user rather than a developer; referential integrity becomes worth it once a second, independent Evidence producer exists).

### `modiq-engine`
- **Current role:** orchestration only — composes Runtime, Rules, Report, and now Collection into callable operations.
- **Current maturity:** L3 for `AssessmentService` (two methods, `execute` and `execute_from_descriptor`, both tested). The four `EngineAPI` stub services (`KnowledgeService`, `ReportingService`, `RuleEvaluationService`, `VersionProfileService`) remain empty unit structs, never invoked — `AssessmentService` composes `RuleEngine`, `AssessmentReport::generate`, and `EvidenceCollector` all by direct instantiation instead.
- **Missing capabilities:** resolution of GOV-004 (service granularity) — see Governance Review.
- **Architectural risks:** the four unused stub services are a scaffold/reality divergence risk: a future contributor could reasonably assume they're load-bearing and wire something into them, when the actual, three-times-proven pattern is direct composition. The two-parallel-entry-points pattern on `AssessmentService` is healthy today but was explicitly flagged in the last handoff as something to watch — a third would be a signal, not a shrug.
- **Recommended priority:** Medium. Not urgent, but ripe: three real data points (Rule Engine, Reporting, Collection) all independently chose the same pattern the stub services were presumably meant to formalize.

### `modiq-collection`
- **Current role:** produces Evidence from an Input Descriptor; the platform's newest subsystem.
- **Current maturity:** functionally minimal by explicit design — one type family (`InputDescriptor`, `InputDescriptorError`, `EvidenceCollector`), 8 tests, zero real inspection capability of any kind.
- **Missing capabilities:** every concrete collector (filesystem, ZIP, XML, Lua, manifest, dependency); a real failure-mode model (GOV-010); the Input Descriptor's real, non-placeholder shape (GOV-009).
- **Architectural risks:** none yet, because scope is trivial — but this is precisely where the platform's *next* real risk concentration will appear. The first real collector will be the first genuinely complex, I/O-touching, and potentially adversarial-input-handling code anywhere in the platform. Nothing built so far has had to consider malicious or malformed external content; this crate will be the first.
- **Recommended priority:** High. This is the platform's actual value proposition — everything downstream of Evidence already works. It is also the crate best positioned to absorb the next unit of real engineering value, provided the first real collector is scoped narrowly.

### `modiq-rules`
- **Current role:** deterministic Evidence evaluation, Finding/Recommendation production.
- **Current maturity:** L3 for exactly one Rule (`evidence-presence-rule`). Four stub sub-modules (`RuleSelector`, `EvidenceEvaluator`, `Explainability`, `Traceability`) exist as empty unit structs, mirroring `RuleEngine.md`'s six conceptual responsibilities, but `RuleEngine::evaluate` implements selection, evaluation, and traceability inline and doesn't use any of them — the same pattern as `modiq-engine`'s unused stub services.
- **Missing capabilities:** a second, genuinely distinct Rule — explicitly and correctly deferred this session ("a capability should justify an abstraction, not the reverse").
- **Architectural risks:** the same scaffold/reality divergence as `modiq-engine`, worth treating as one finding rather than two, since both crates independently exhibit it.
- **Recommended priority:** Low for new Rule work (deliberately deferred, no concrete second case exists yet). Medium for the scaffold-reconciliation question, bundled with `modiq-engine`'s.

### `modiq-report`
- **Current role:** read-only `AssessmentReport` snapshot; performs no analysis.
- **Current maturity:** L3, stable, 3 tests, untouched since Sprint 2 except mechanical test-fixture updates.
- **Missing capabilities:** GOV-001 (report generation timing) is literally about this crate's core behavior and remains open. More significantly: `Architecture.md`'s Reporting System is specified to include "generating explanations," but today's `AssessmentReport` is a pure data aggregate with no explanation-generation logic anywhere. One of the three questions the entire platform exists to answer — "why was this conclusion reached" (`Glossary.md`: Assessment Report) — is not yet actually implemented; it's currently answered only by a human reading raw Finding/Recommendation fields.
- **Architectural risks:** low complexity, but a real and currently invisible product-value gap.
- **Recommended priority:** Medium-high in importance, but correctly low in near-term sequencing — there is only one Rule and one collector's worth of content to explain; expanding Reporting now would be explaining almost nothing. Revisit once Rule and Collection variety exists.

### `modiq-knowledge`
- **Current role:** reusable engineering knowledge, independent of any Assessment (per its own README, which is unusually thorough for a crate with zero implemented logic).
- **Current maturity:** L1, pure scaffold — seven marker structs (`Rule`, `RepairRecipe`, `BestPractice`, `CompatibilityPattern`, `KnownIssue`, `EngineBehavior`, `KnowledgeReference`), all empty, referenced by nothing.
- **Missing capabilities:** everything. No content model has ever been implemented or pressure-tested.
- **Architectural risks:** low — isolated and inert — but it is the least-validated conceptual assumption in the platform. `KnowledgeModel.md`'s shape has never been tested against a real Rule that actually needs Knowledge.
- **Recommended priority:** Low. Correctly sequenced after Rule abstraction, which is itself correctly deferred. Two dependencies removed from relevance.

### `modiq-cli`
- **Current role:** user-facing command entry point, explicitly forbidden from containing business logic.
- **Current maturity:** L1 scaffold — `Application`, `VersionCommand`, `HelpCommand`, `AssessCommand`, all empty; `main.rs` prints a literal string; not wired to `modiq-engine` at all.
- **Missing capabilities:** everything, but the pattern to build it is already proven twice over by the Sandbox (thin client → `AssessmentService` → DTO mapping).
- **Architectural risks:** very low. The Sandbox is currently the platform's only real consumer of `modiq-engine`'s public API — the CLI would be the first independent test of whether that API is actually reusable by more than one application.
- **Recommended priority:** Medium. Genuinely low-risk, high-confidence, and valuable precisely because it's a second, independent validation of the public API surface GOV-008 will eventually need to settle.

### Sandbox
- **Current role:** thin, non-production developer visualization tool; owns zero domain logic (now even less than before Phase 4).
- **Current maturity:** functionally proven across two real integration phases (Phase 1: real pipeline; Phase 4: real Evidence Collection). 3 tests, all passing.
- **Missing capabilities:** any real input capability — explicitly and repeatedly excluded by design since Sandbox Phase 1.
- **Architectural risks:** the "no file loading, no ZIP handling" constraint, already identified in the Evidence Collection Boundary proposal as needing explicit reconsideration, remains unresolved and is now the single biggest blocker standing between the platform and demonstrating real Evidence Collection through its own developer tool. Separately: no session on record has visually launched and inspected the actual UI — only its Rust backend has ever been verified.
- **Recommended priority:** Medium, directly tied to whichever collector milestone is scheduled next. The path/descriptor-input question needs a decision before real collection work can be meaningfully exercised through the Sandbox.

---

## 3. Governance Review

| Item | Status | Implementation Pressure | Recommendation |
|---|---|---|---|
| GOV-001 (Report generation timing) | Open since v0.1.0-alpha | Low today; the single-Rule pipeline has never exposed a problem | Defer, but resolve before or alongside any `modiq-report` capability expansion |
| GOV-002 (Runtime invariant reconciliation) | Open since v0.1.0-alpha | Low — a documentation-completeness gap, not a functional one; invariants are enforced, just not fully cataloged | Defer; cheap to fold in opportunistically alongside future `RuntimeInvariants.md` work |
| GOV-003 (Role of `modiq-common`) | Open since v0.1.0-alpha | Zero — crate is empty, nothing depends on it | Defer indefinitely; resolve only when two crates naturally need to share a type. Do not invent a use for it |
| GOV-004 (Engine service granularity) | Open since v0.1.0-alpha | **Now elevated** — three real data points (Rule Engine, Reporting, Collection) all independently bypassed the stub `EngineAPI` services via direct composition | Resolve soon — not urgent enough to block the next milestone, but the evidence base for a decision now exists and won't get materially stronger by waiting |
| GOV-005 / GOV-006 (cardinality) | Resolved | — | No action. Referential-integrity follow-up remains intentionally unnumbered until a second, real Evidence producer creates an actual case to design against |
| GOV-007 (Evidence Collection implementation approval) | Resolved | — | No action |
| GOV-008 (AssessmentService API evolution) | Open, medium-high pressure | Two parallel entry points exist; healthy today, but a third would be the signal to stop routing around this decision | Resolve concurrently with the *next* Engine-facing change (e.g., CLI wiring or the next collection milestone) rather than waiting for pressure to force it |
| GOV-009 (Input Descriptor ownership) | Open, currently low pressure | Will become high pressure the moment any real collector is scoped — a real collector needs to know if it's receiving a path, bytes, or something else | Resolve concurrently with scoping the first real collector, not before — deciding it in the abstract risks the exact premature-abstraction failure mode this project has repeatedly avoided |
| GOV-010 (Collection error model) | Open, same timing logic as GOV-009 | Cannot be well-designed without a real failure mode to observe | Resolve concurrently with the first real collector, for the same reason |

**On opening new governance items:** none are recommended by this review. GOV-008, GOV-009, and GOV-010 form a natural cluster that should be resolved together, triggered by one event — scoping the first real collector — rather than as three separate governance cycles. No new item is justified for the `modiq-engine`/`modiq-rules` scaffold question either; it is fully covered by the existing, already-open GOV-004.

---

## 4. Capability Roadmap

| Capability | Engineering Value | Architectural Readiness | Implementation Complexity | Dependencies | Risks | Recommended Timing |
|---|---|---|---|---|---|---|
| **Filesystem collection** | High — first non-synthetic Evidence in the platform | High — boundary proven, minimal work needed | Low–Medium (basic I/O, path validation) | GOV-009 (descriptor must represent a path), GOV-010 (needs a real failure mode) | First genuine I/O failure surface in the platform | **Next, immediately** |
| **ZIP traversal** | High — mods are typically distributed as archives; likely the actual real-world input shape | Medium — depends on filesystem collection landing, plus a new external dependency choice | Medium (archive edge cases; zip-bomb/decompression limits) | Filesystem collection; chosen archive crate; resolved GOV-009/010 | Security (malicious archives), performance, determinism (entry ordering must be sorted per the Collector Contract) | Soon after filesystem collection |
| **XML inspection** | High — ModDesc XML is the core of FS mod structure (`Glossary.md`: Compatibility) | Medium — needs a parser choice and real domain knowledge of valid ModDesc shape | Medium–High (the *checking* is the hard part, not the parsing) | ZIP traversal; likely reopens the Rule-abstraction question | Scope creep — "XML inspection" can silently become "the entire Assessment Framework" | After ZIP traversal; scope the first slice extremely narrowly |
| **Lua inspection** | Medium-high long-term, low near-term | Low — no strategy discussed anywhere in the docs; determinism concerns are real | High | XML inspection experience; likely its own architecture proposal | Highest complexity of any near-term capability; must be constrained to static/textual analysis only, never execution, to preserve the deterministic principle — this constraint should be explicit and probably its own ADR before work starts | Not soon; dedicated proposal required |
| **Manifest parsing** | High, but likely redundant as a separate milestone | — | — | Overlaps almost entirely with XML inspection (ModDesc.xml *is* the manifest) | Double-planning the same work under two names | Fold into XML inspection's scope rather than plan separately |
| **Dependency analysis** | Medium-high (`Glossary.md` already names Dependency Resolution as its own Evidence category) | Low — needs manifest/XML data as input first | Low if scoped to reading declared dependencies only; high if scoped to verifying installed/available mods | XML inspection | Scope ambiguity between "observe declared dependencies" (in scope) and "resolve/verify them" (likely out of scope for a static tool) | After XML inspection; scope to observation only |
| **Version Profiles** | High long-term (ADR-0004 already committed to this) | Architecturally ready (scaffold exists), zero real content | Medium once started | Nothing currently varies by game version — no forcing function exists yet | None near-term; risk is building it before anything needs it | Defer until at least one real collector/rule would plausibly behave differently across FS22/FS25 |
| **Knowledge integration** | High long-term | Low pressure — only one Rule exists | Medium | Rule abstraction (itself deferred) | Building a knowledge model with nothing real to inform | Defer, sequenced after Rule abstraction |
| **CLI** | Medium — platform-completeness; second real validation of `modiq-engine`'s public API | High — pattern proven twice, scaffold already shaped correctly | Low | None blocking | Minimal | **Good near-term candidate**, independent of collection work |
| **Persistent assessment storage** | Medium (`DataModel.md`'s Immutability principle implies eventual persistence) | Low — Storage Layer is a name in `Architecture.md`, nothing more | Unassessed — no technology chosen | Real (non-synthetic) Assessments worth keeping | Building storage for data nobody needs to keep yet | Defer well past the near-term roadmap |
| **Desktop application evolution** | Strategic — the eventual product destination | N/A | N/A | A product decision, not an engineering one | Committing engineering effort to the wrong shape before the decision is made | Defer explicitly; **requires its own dedicated Technical Director conversation**, not a roadmap bullet |

---

## 5. Sprint Reassessment

**Recommendation: keep the current Sprint 3 container, and explicitly scope its next phase(s) using the findings above — do not open a "Sprint 4" and do not discard the existing structure.**

Justification: Sprint 3's own entry criteria (`CrateRoadmap.md`) required that its scope be "chosen deliberately from the Known Deferred Work list... rather than assumed by default." Every phase so far has honored that discipline — this section applies the same discipline forward rather than introducing new philosophy.

Concretely:

- **Sprint 3 Phase 5 (recommended): Filesystem Collection**, with GOV-009 and GOV-010 resolved concurrently using the real implementation as the concrete case — not as a separate governance-only phase like Phase 3, since these two items are specifically the kind of question this project has learned (twice now, with GOV-005/006 and the Rule-trait question) to resolve by observing real usage rather than by abstract proposal.
- **A separate, low-risk track: CLI wiring.** This should be its own explicitly numbered phase, not folded into Phase 5 — it shares no risk surface with Collection work, and bundling a low-risk, high-confidence win with higher-risk I/O work would dilute the ability to ship it quickly and independently.
- **Do not start** ZIP traversal, XML inspection, Rule abstraction, Knowledge integration, or Version Profiles yet — each has an unmet dependency or unmet forcing function documented in Section 4.
- **Schedule an Engineering Release checkpoint soon**, independent of new implementation work. Sprint 3 has now run four phases without a release freeze — longer than Sprint 2's single-phase-to-release cadence — and the release-documentation loop (see Technical Debt Review) has fallen behind as a result.

No milestone split, merge, or reordering beyond the above is recommended. The roadmap's shape is sound; its next two steps needed to be named, which this document does.

---

## 6. Technical Debt Review

**Code — low, mostly intentional.** The one open, genuinely accidental item: `modiq-runtime`'s missing `Display`/`Serialize` for identity and enum types, flagged three times now (Sandbox Phase 2, Sprint 3 Phase 1, this review) without being scheduled. It has stayed low-urgency long enough that its "not urgent" justification is starting to look more like inertia than a judgment call.

**Architecture — none beyond what Governance Review already covers.** Crate boundaries have held with zero violations across every phase. The one finding, GOV-004's elevated pressure, is already tracked.

**Documentation — the project's most consistent debt pattern.** `PROJECT_STATUS.md` and `CHANGELOG.md` are both confirmed stale as of this review (directly verified: `PROJECT_STATUS.md` still names Sprint 2 as the current milestone). `docs/README.md`'s Current Documentation Status narrative and `docs/00-Governance.md`'s vestigial index have each been flagged in three consecutive sessions without being fixed, because they were never the literal subject of whatever that session was scoped to do. This is accidental debt, but it is *structurally* accidental: nothing in the current workflow forces these documents to be touched, so they reliably won't be, regardless of how many times they're noticed.

**Governance — healthy.** Seven open items, all appropriately triaged in Section 3; none abandoned; GOV-004 is the one ripe for near-term closure.

**Testing — no debt in the traditional sense, but a real coverage-shape gap.** 106/106 passing, zero flaky, zero ignored. The gap is structural: zero verification, ever, that the Sandbox's actual UI works — every check has been Rust-only. Zero adversarial/malformed-input testing anywhere, because nothing has yet touched untrusted external content. This stops being acceptable the moment filesystem or ZIP collection lands; it is fine today only because nothing yet needs it.

**Intentional debt (by design, documented, currently fine as-is):** `EvidenceCollector`'s infallibility; GOV-008/009/010 remaining unresolved pending real forcing functions; Knowledge/Versioning/CLI left unbuilt; Sandbox's no-input-capability constraint.

**Accidental debt (should be scheduled, not deferred again):** the `Display`/`Serialize` gap; the unused `EngineAPI` and `modiq-rules` stub modules (tracked under GOV-004); `PROJECT_STATUS.md`/`CHANGELOG.md`/`docs/README.md`/`docs/00-Governance.md` staleness; the absence of any Sandbox UI verification.

---

## 7. Recommendations — Next 5 Engineering Milestones, Prioritized

### 1. Filesystem Collection (first real collector)
- **Objective:** replace `EvidenceCollector`'s synthetic output with genuine `FileStructureAnalysis` Evidence read from a real file or directory.
- **Affected crates:** `modiq-collection` primarily; possibly a minimal `modiq-engine` touch if `execute_from_descriptor`'s input type needs to move beyond a plain `String` (coordinate with GOV-009).
- **Governance prerequisites:** GOV-009 and GOV-010 resolved concurrently, using this implementation as the concrete case — not resolved in advance of it.
- **Expected documentation work:** likely a small Documentation Release once GOV-009/010 land as real invariants/contract text in `EvidenceCollection.md`; no new ADR expected unless the ownership boundary itself needs to move (it shouldn't).
- **Expected implementation effort:** low–medium.
- **Expected architectural risk:** low–medium — first real I/O and first real failure-handling code in Collection, on an already-proven boundary.

### 2. CLI Wiring
- **Objective:** wire `modiq-cli`'s existing `AssessCommand` scaffold to `modiq-engine`, mirroring the Sandbox's thin-client pattern exactly.
- **Affected crates:** `modiq-cli` only.
- **Governance prerequisites:** none blocking.
- **Expected documentation work:** minimal — a `CrateRoadmap.md` maturity update.
- **Expected implementation effort:** low.
- **Expected architectural risk:** very low — a proven pattern, an isolated crate, zero shared surface with any other in-flight work.

### 3. EngineAPI / Rule Engine Scaffold Reconciliation (GOV-004 closure)
- **Objective:** decide whether the four unused `EngineAPI` stub services and `modiq-rules`'s four unused stub sub-modules should be removed, repurposed, or formally endorsed as intentional forward scaffolding — now backed by three real data points that all independently chose direct composition instead.
- **Affected crates:** `modiq-engine`, `modiq-rules` (a decision and likely deletion, not new logic).
- **Governance prerequisites:** resolves GOV-004 directly.
- **Expected documentation work:** possible `GOVERNANCE.md` Engine-boundary update; a short ADR only if the resolution formally changes an architectural pattern (e.g., endorsing direct composition over service indirection as the standing rule).
- **Expected implementation effort:** low — this is a decision and cleanup pass, not new capability.
- **Expected architectural risk:** low, but worth doing before more stub-shaped scaffolding accumulates on top of the existing pattern.

### 4. Living Document Reconciliation
- **Objective:** close the recurring staleness pattern in one dedicated pass — `PROJECT_STATUS.md`, `CHANGELOG.md`, `docs/README.md`'s status narrative, `docs/00-Governance.md` — and schedule the Engineering Release that Sprint 3 Phases 1–4 are currently missing.
- **Affected crates:** none — documentation only.
- **Governance prerequisites:** none.
- **Expected documentation work:** this milestone *is* the documentation work.
- **Expected implementation effort:** low but nonzero — four-plus documents plus one release freeze.
- **Expected architectural risk:** none. Pure housekeeping — but it has now been deferred every session it was noticed, and compounds if deferred again.

### 5. ZIP Traversal (second collector)
- **Objective:** extend `modiq-collection` with archive-aware collection, building directly on Filesystem Collection's landed GOV-009/010 decisions.
- **Affected crates:** `modiq-collection` (first external parsing dependency introduced into any domain crate — worth flagging to the Technical Director as a first, not routine, kind of decision).
- **Governance prerequisites:** Filesystem Collection (Milestone 1) must land first; may warrant a short addendum to `EvidenceCollection.md`'s Determinism Expectations covering resource limits (zip-bomb handling) rather than a new ADR.
- **Expected documentation work:** moderate — new dependency choice plus a security consideration to record.
- **Expected implementation effort:** medium.
- **Expected architectural risk:** medium — the platform's first adversarial-input surface.

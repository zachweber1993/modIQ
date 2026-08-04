# Capability Portfolio Assessment

| Property | Value |
|---|---|
| **Document** | CAPABILITY_PORTFOLIO_ASSESSMENT.md |
| **Project** | modIQ |
| **Type** | Repository Capability Survey — not a Capability Definition, Architecture Evaluation, Architectural Resolution, Implementation Authorization, or Sprint Plan. |
| **Author** | Chief Architect / Technical Director |
| **Origin** | `docs/engineering/PLATFORM_FOUNDATION_V1_DECLARATION.md`, `docs/engineering/POST_FOUNDATION_ENGINEERING_CAPABILITY_ASSESSMENT.md`, and Sprint 24's own closeout — all treated as frozen repository history. |

---

## Purpose

This document answers one question: given Platform Foundation Version 1, what is the single best capability for modIQ to build next, and why? It surveys concrete capability candidates against repository evidence, classifies each by architectural readiness, and names one candidate as the correct first post-foundation engineering milestone. It does not design, authorize, or plan that capability — that begins only after this assessment, through its own Capability Definition and, if warranted, Architecture Evaluation.

---

## Repository Context

Platform Foundation Version 1 (declared, `5ab2224`) and the Post-Foundation Engineering Capability Assessment (`1de3749`) are treated as settled: stable Runtime, Knowledge, Rule, and Collector architecture; a stable Storage mirror pattern and Transport mirror pattern; a stable Frontend consumer boundary; a mature, repeatable implementation lifecycle. These are not re-evaluated here. This survey instead asks, of each concrete candidate, whether that already-settled architecture is sufficient to build it, partially sufficient, or genuinely insufficient.

---

## Evaluation Method

Each candidate is checked directly against repository source — the actual `EvidenceCategory` enum and which variants a real Collector produces; which categories a real Rule interprets; what a governing document says is resolved, deferred, or undesigned; whether a named prerequisite (a governance item, an Architectural Resolution, a fixture corpus) exists and in what state. Classification follows the three categories specified for this assessment (A: ready for normal engineering workflow; B: requires a focused Architecture Evaluation first; C: premature, prerequisite work genuinely missing), applied per-candidate, not as a global judgment about the platform. This A/B/C schema is a readiness survey specific to this assessment, applied uniformly across several different capability types (Rules, Collectors, Runtime fields, transport, Storage, and governance-blocked candidates alike). It is not, and does not replace or extend, Sprint 12's own Capability Identity mechanism — the three-axis Collection/Evidence/Interpretation classification and the Introduction/Expansion/Enrichment test remain the repository's own architectural classification process for Collector- and Rule-shaped candidates specifically, and continue to apply, unchanged, wherever a candidate reaches that stage.

Fresh source checks performed for this assessment: the complete `EvidenceCategory` enum (`modiq-runtime`) against a repository-wide grep for which variants any Collector actually constructs and which variants any Rule actually matches on; `XmlCollector`'s own declared-dependency evidence production; `Glossary.md`'s own entries for `AssetValidation`/`DependencyResolution`/`PerformanceObservations`.

---

## Capability Candidates

**C1 — Present `RecommendationStep` (Sprint 24's repair structure) in `apps/console`.** Sprint 24 gave `Recommendation` a genuine, structured per-step repair representation; no consumer presents it. `apps/console`'s `FindingSummary`/recommendation transport carries only `action`, unchanged since Sprint 21.

**C2 — Interpret already-collected "Declared Dependency" Evidence.** `XmlCollector` already produces one `XmlInspection` Evidence item per declared `<dependency>` element (confirmed directly, `xml_collector.rs`), labeled "Declared Dependency." No Rule reads it — `VersionCompatibilityRule` is the only Rule matching `XmlInspection`, and it matches only the declared-`descVersion` prefix, never the declared-dependency prefix. This Evidence is not merely uninterpreted; it is currently unreachable by every existing consumer. `apps/console`'s `ReportSummary` nests Evidence under the Finding that references it (Sprint 21 Phase 3's own transport shape); since no Rule ever produces a Finding referencing declared-dependency Evidence, it is never transported or presented anywhere today, by any consumer.

**C3 — Extend `modiq-cli`'s text output** to surface `mod_health_dimension`, `status`, Evidence provenance, and `repair_steps` — fields `apps/console` gained since Sprint 22/23/24 that `modiq-cli` has never been extended to print.

**C4 — Extend `apps/sandbox` for field parity** with Sprint 22/23/24's Runtime additions. Already named, repeatedly, as available with no forcing function (`ENGINEERING_RELEASE_1.7.md`, `_1.8.md`).

**C5 — Cross-mod dependency resolution** (whether a declared dependency in C2 actually exists in an installation). Requires evidence about mods other than the one Assessment Subject; `STORAGE_ARCHITECTURE_EVALUATION.md` names "cross-mod collection validation" as a deliberately undecided, deferred product-forcing-function.

**C6 — Confidence as a first-class concept** (Initiative 4). Scope, exclusivity to Recommendation, and evidence-quality meaning are Adopted; concrete representation and computation mechanism are, in the Architectural Resolution's own words, "genuinely undesigned."

**C7 — Lua Analysis capability.** Named since Sprint 0/INV-001; GOV-014 (fixture provenance, licensing, storage policy) is Open and, by its own Resolution text, "must be resolved before any Lua fixture acquisition begins, and before Lua Analysis's own Architecture Evaluation may be authorized."

**C8 — Activate a new dormant `EvidenceCategory`** (`AssetValidation`, `DependencyResolution`, or `PerformanceObservations`) via a new Collector. Confirmed dormant: zero Collector constructs any of the three; zero Rule matches any of the three. `Glossary.md` lists each by name only, with no further elaboration — no INV-style investigation, no fixture corpus, no specification depth exists for any of the three, unlike Lua Analysis (which at least has `INV-001`'s own investigation).

**C9 — Progressive Execution Observability (Initiative 1) or Reentrant Assessment Lifecycle (Initiative 2).** Both remain unresolved at or beyond the Architectural Resolution stage; Initiative 2's own Governance Reconciliation found its central question "cannot presently be reconciled under the repository's current governance model."

**C10 — Extension Layer** (custom Rules, plugins, external integrations). Zero implementation; `Architecture.md`'s own description is aspirational ("potential capabilities include... future AI-assisted capabilities") with no supporting Product or Interaction Design elaboration found anywhere in the corpus reviewed for this or the prior two assessments.

**C11 — `AssessmentSubject`/`AssessmentContext` real content, and Report Identity.** Both remain zero-field placeholder types; Item 6a's own connection to GOV-002's prior "require no action" disposition on `AssessmentSubject` is named as "a candidate for reconsideration, not decided" — the reconsideration itself has not occurred.

**C12 — Richer Historical Assessment Analysis** (`modiq-storage`'s `history_analysis` module), beyond the current `(rule_reference, severity)` pattern — for example, adding `ModHealthDimension` as a second aggregation dimension, using data already present on every persisted `Finding` since Sprint 22. Explicitly excludes any per-mod or cross-mod analysis, which requires C11's or C5's own prerequisites.

---

## Capability Classification

| Candidate | Class | Basis |
|---|---|---|
| C1 — Present `RecommendationStep` | **A** | Transport Mirror and presentation-extension patterns each already twice-demonstrated (Sprint 21→22→23); Consumer-Owned State's own precedent (Sprint 23) shows this exact shape of work required no Architecture Evaluation. |
| C2 — Interpret declared-dependency Evidence | **A** | Evidence already produced and currently unreachable by any consumer; no new Collector; a single-mod-scoped judgment (e.g., malformed or duplicate declared names) fits GOV-012's existing dispatch model and the Interpretation Axis's own two-stage filter. As with every historical Rule addition (Sprint 5, 8, 9, 11), the construction pattern being settled does not by itself remove the architectural work the existing Architectural Review Process requires before implementation. |
| C3 — `modiq-cli` field parity | **A** | Mechanical extension of an already-public Runtime getter into already-existing CLI output formatting; no architectural question of any kind. |
| C4 — `apps/sandbox` field parity | **A** | Same shape as C3; repository itself has already named it available with no forcing function. |
| C12 — Richer Historical Assessment Analysis (aggregate-only) | **A** | Uses data already on every persisted `Finding`; `modiq-storage`'s own read-only, out-of-band analysis boundary (Sprint 20) is unaffected in kind. |
| C6 — Confidence | **B** | Scope and meaning Adopted; representation and computation mechanism require dedicated design — a focused Architecture Evaluation, not absent prerequisite work. |
| C5 — Cross-mod dependency resolution | **B** | The specific mechanism is undesigned, but the blocking question (what "cross-mod collection validation" means) is already named and scoped in `STORAGE_ARCHITECTURE_EVALUATION.md`, giving a focused Evaluation a real starting point. |
| C7 — Lua Analysis | **C** | GOV-014 must resolve *before* an Architecture Evaluation may even be authorized — a governance precondition, not an architecture question, and it is Open. |
| C8 — New dormant `EvidenceCategory` | **C** | No specification depth, no fixture corpus, no investigation of any kind exists for any of the three variants — a less-evidenced starting point than Lua Analysis had even before `INV-001`. |
| C9 — Initiative 1 / Initiative 2 | **C** | Each remains unresolved at or beyond Architectural Resolution; Initiative 2 is on record as currently unreconcilable under this repository's own governance model. |
| C10 — Extension Layer | **C** | Zero implementation, zero specification depth beyond an aspirational list, no forcing function identified anywhere in the corpus. |
| C11 — `AssessmentSubject`/Report Identity | **C** | Depends on a governance reconsideration (GOV-002's prior disposition) that has not occurred. |

---

## Architectural Readiness

Category A candidates (C1–C4, C12) each map onto a construction pattern the Post-Foundation Capability Assessment already found settled and multiply demonstrated — Transport Mirror, Rule Interpretation via the two-stage filter, or read-only Storage aggregation. This settles how each would be built; it does not settle whether building it still requires its own architectural work. Repository history does not support treating reusable construction patterns as removing that requirement: every historical Rule or Collector addition this repository has made — Sprint 5 (via GOV-012), Sprint 8, and Sprint 9, regardless of each one's own Enrichment/Expansion classification under Sprint 12's own Historical Validation table, and Sprint 11 — was preceded by its own dedicated architectural document (a Governance Register resolution, an Architectural Resolution, or an Architecture Evaluation), never bypassed one. Sprint 22 and Sprint 24 show the same discipline for Runtime and Storage content specifically. For Category A candidates, repository precedent is accordingly stronger and the architectural work a future Capability Definition or Architecture Evaluation would need to perform may be substantially smaller than for a novel candidate — this is what "no obvious architectural blocker" means here — but the existing Architectural Review Process (`GOVERNANCE.md`) remains the authority on whether, and how much, that work is required for any specific instance; nothing in this survey removes that determination. Category B candidates (C5, C6) each have a real, already-named architectural question (cross-mod evidence scope; Confidence's own representation) that a dedicated Architecture Evaluation could resolve without first requiring new governance or new fixture acquisition. Category C candidates (C7–C11) each have a named, unresolved precondition — a governance item, an unreconciled Initiative, or a wholly unspecified concept — that exists independently of any Architecture Evaluation and must resolve first.

---

## Engineering Readiness

C1 is the only Category A candidate that extends a capability the repository has *just* built (Sprint 24) rather than one already fully consumed. The engineering lifecycle that would execute it — Implementation Authorization, Sprint Plan, phased implementation, and (per Sprint 24's own new precedent) an Implementation Audit — is demonstrated, current, and has now been exercised twice in immediate succession (Sprint 23 following Sprint 22; this would be the third instance of "extend Runtime, then present it"). C2–C4 and C12 fit the same lifecycle without requiring any new engineering-process capability beyond what Sprint 21–24 already exercised.

---

## Product Impact

The candidates do not separate cleanly on user value alone; they separate more clearly when architectural readiness, implementation risk, and user value are considered as distinct axes rather than one combined score.

- **Architectural readiness and implementation risk** favor C1 most clearly, on direct repository evidence: its exact construction shape (extend a transport DTO, mirror new Runtime fields, render within an existing expansion layer) has been performed twice, and the second instance (Sprint 23) required zero reinterpretation of Consumer-Owned State — the one demonstrated case in this repository where that shape of work needed no dedicated architectural document. C2 is not disadvantaged here relative to other Rule-adding candidates, but, like every historical Rule addition (Sprint 5, 8, 9, 11), it should be expected to carry its own architectural step, not assumed to skip one.
- **User value** is a closer comparison than architectural readiness. C1 makes an already-visible warning's guidance more complete and structured for a user who has already seen the underlying Finding. C2 would make visible, for the first time, Evidence ("Declared Dependency") that no consumer can currently reach at all, since it is never linked to a Finding under `ReportSummary`'s existing transport shape. Repository evidence supports a reasonable inference that C1's value is realized sooner and with less new judgment required (no new Rule, no new dimension assignment), but it does not conclusively establish that C1's value exceeds C2's — both convert real, already-collected repository content into something a user has never seen.
- **C3 and C4** deliver completeness and parity, not new user-facing insight, and have been named, repeatedly, as lower-value by the repository's own prior Engineering Releases.
- **C12** delivers a modest analytical widening of an already-narrow, already-deferred capability (Historical Assessment Analysis).

---

## Recommended Next Capability

**C1 — Present Sprint 24's Runtime-owned repair structure (`RecommendationStep`) in `apps/console`.**

## Why that capability is the correct first post-foundation milestone

Four independent lines of evidence converge on the same candidate, none requiring inference beyond what the repository already states:

1. **It is the only candidate closing a loop the repository itself opened and left open.** Sprint 24's own Implementation Report names `apps/console` presentation of `repair_steps` as a "real, evidence-backed future opportunity, requiring its own, separate Implementation Authorization" — the repository already pointed here.
2. **It is architecturally the lowest-risk candidate available.** The exact shape of work — extend a transport DTO, mirror new Runtime fields, render within an existing expansion layer — has been performed twice already (Sprint 22→23), and the second instance required zero reinterpretation of Consumer-Owned State, meaning a third instance carries the least uncertainty of any candidate surveyed.
3. **It produces value from work already paid for, sooner and with less new judgment than the closest alternative.** C1 converts already-completed engineering investment (all of Sprint 24) into something a user can see, without requiring a new Rule, a new dimension assignment, or any interpretation not already fixed by Sprint 24's own frozen match. C2 has a comparably real claim to new-to-the-user value (declared-dependency Evidence is currently unreachable by any consumer, not merely under-interpreted) but would require a genuinely new Rule — repository evidence supports C1 as the lower-risk, sooner-realized choice between the two; it does not establish that C1's user value conclusively exceeds C2's. C3 and C4 remain narrower in value or already named as low-priority by the repository's own prior Engineering Releases.
4. **It requires no new governance, no new fixture acquisition, and no reconsideration of any Open item.** Unlike every Category B or C candidate, nothing about C1 depends on an unresolved Governance Register item, an unreconciled Initiative, or undesigned representation.

No Category B or C candidate can currently make any of these four claims simultaneously.

---

## Capabilities Intentionally Deferred

- **C2** — a real, evidenced Category A candidate with a comparably strong claim to new-to-the-user value, deferred here on architectural-readiness and implementation-risk grounds (it requires a new Rule; C1 does not), not because its value is established as lower. Not blocked, and may be taken up independently without prejudice to C1.
- **C3, C4, C12** — real, evidenced Category A candidates that deliver narrower or already-acknowledged lower-priority value than C1; none is blocked, and any may be taken up independently without prejudice to C1.
- **C5 (cross-mod dependency resolution), C6 (Confidence)** — Category B; each is a legitimate future Architecture Evaluation subject, not designed or scoped here.
- **C7 (Lua Analysis), C8 (new dormant categories), C9 (Initiative 1/2), C10 (Extension Layer), C11 (`AssessmentSubject`/Report Identity)** — Category C; each has a named, unresolved precondition this document does not attempt to resolve.

---

## Conclusion

Given Platform Foundation Version 1, the correct first post-foundation engineering milestone is presenting `RecommendationStep` in `apps/console` — the one candidate that is simultaneously lowest architectural risk, real and traceable in product value, fully supported by twice-demonstrated repository pattern, and dependent on no unresolved governance or architecture question. This document does not design, authorize, or plan that work; it determines only that it is the correct candidate to carry forward into its own Capability Definition.

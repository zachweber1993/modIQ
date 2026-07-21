# Sprint 5 Phase 4: Reporting Scaffold Investigation

| Property | Value |
|----------|-------|
| **Document** | SPRINT5_PHASE4_REPORTING_INVESTIGATION.md |
| **Stage** | Investigation only — recommendation, not a decision |
| **Sprint context** | Sprint 5 Phase 4, per `SPRINT5_IMPLEMENTATION_PLAN.md` (Design Question 4) |
| **Status** | Presented for Technical Director review. **No `modiq-report` file has been modified. Nothing has been built out or retired.** Per Technical Director direction, any action on this recommendation — building out or retiring anything — is deferred to a later sprint regardless of this investigation's outcome. |

---

## The Question

`crates/modiq-report/src/report/` contains four types, each a one-line, zero-logic unit struct, unmodified since the platform's first foundational commit: `FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, `ReportFormatter`. `GOVERNANCE.md`'s Crate Boundary Rules assigns their exact conceptual territory — "report formatting," "report summarization," "traceability output" — to Reporting as a real, still-outstanding responsibility. Sprint 5 gave the platform its first genuinely differentiated Findings (two Rules, two distinct severities, real content) — the first point since Sprint 1 where this territory could plausibly be exercised for real, rather than hypothetically. This investigation asks: does that change anything?

## Evidence Gathered

**Construction sites: zero, for all four types, unchanged.** A workspace-wide search finds no construction, no method call, and no test referencing any of the four beyond their own one-line definitions and `mod.rs`'s re-export. This is the identical evidentiary shape GOV-004 used to retire the `EngineAPI`/`modiq-rules` service scaffolding: "no construction site, method call, or test exists... anywhere in the workspace."

**The one real consumer does not need them, even now.** `apps/sandbox/src/App.tsx` — the only code anywhere in this repository that actually renders an `AssessmentReport`'s content to a user — displays Evidence, Findings, and Recommendations as three independent flat lists (`summary.evidence.map(...)`, `summary.findings.map(...)`, `summary.recommendations.map(...)`). It performs:

- **No summarization.** No count-by-severity, no grouping, no rollup — `evidenceCount`/`findingCount`/`recommendationCount` are already plain `.length` values computed once in the Rust DTO layer (`AssessmentSummary::from`), not by any dedicated summarization type.
- **No formatting beyond raw field display.** Severity is shown as whatever string the Rust `Debug` derive produces (`"Warning"`, `"Informational"`) with no translation layer; this happens to read fine today, but is coincidental, not evidence a `ReportFormatter` was needed to make it so.
- **No traceability cross-referencing.** Even though `Finding.evidence_ids()` and `Recommendation.finding_ids()` already carry the relationships needed to show "this Warning came from this specific piece of Evidence," nothing in the Sandbox renders that connection. The three lists are visually and structurally independent.

This was checked specifically *because* Sprint 5 changed the input — two Rules, two severities, real per-category content — on the theory that differentiated Findings might be the forcing function summarization/traceability previously lacked. It is not: the frontend's own rendering logic did not need to change at all to accommodate `StructuralDuplicationRule`'s new `Warning`-severity Finding: it already displays whatever severity string a Finding happens to carry.

**Per-type assessment:**

- **`FindingSummary` / `RecommendationSummary`** — no evidence of need. Counting or grouping by severity is a one-line `.iter().filter(...).count()` today; nothing in this repository does even that much.
- **`ReportFormatter`** — no evidence of need. The Sandbox's own ad hoc, inline formatting has been sufficient through every Sprint to date, including this one.
- **`TraceabilityReport`** — the closest to a plausible future need, since real Rule diversity now exists for the first time, but still zero *actual* evidence: nothing has ever asked to render an Evidence→Finding→Recommendation chain. `Assessment`'s own relationship-resolution methods (`evidence_for_finding`, `findings_for_recommendation`) already exist in `modiq-runtime` and already answer this question programmatically when needed; a dedicated Reporting-layer type would only be justified once something outside `modiq-runtime` itself needs that resolution and doesn't have it.

## Recommendation

**Retire all four**, on the same basis and by the same method GOV-004 used: zero construction sites across every Sprint to date, and the one real consumer's actual behavior — checked directly against this Sprint's own new severity differentiation, not assumed to be unaffected by it — still does not require any of the three responsibilities these types were scaffolded for. `GOVERNANCE.md`'s Crate Boundary Rules assigning "report formatting," "summarization," and "traceability output" to Reporting as a responsibility is not disputed by this recommendation — only the claim that these four specific, never-instantiated types are the right shape for fulfilling it, before any concrete case exists to test that shape against. This is the same "capability before abstraction" principle applied a sixth time, this time to Reporting rather than to Rule or Collector dispatch.

**This recommendation is not a decision.** Per Technical Director direction (Phase 3 review), any action — retiring these four types, or building real content behind any of them instead — is deferred to a later sprint, regardless of which direction this investigation favors. Phase 4's own completion criteria require only that this recommendation exist and be reviewed; `modiq-report` remains untouched in source as of this document.

## What Would Change This Recommendation

A concrete forcing function — the same standard GOV-004 and every "capability before abstraction" application on this platform has required: a real consumer (the Sandbox's own future evolution, a CLI, or a second UI) that actually needs summarization, formatting, or traceability output Reporting does not currently provide, discovered through building that consumer, not anticipated in the abstract. Until then, this investigation's own evidence does not support building any of the four out.

## What This Document Does Not Do

No `modiq-report` file was modified, created, or deleted. No `GOVERNANCE.md` or other specification document was amended. This document records an investigation and a recommendation only, per Sprint 5 Phase 4's own scope; the Technical Director's review of it is what Phase 4's completion criteria require, not any code change resulting from it.

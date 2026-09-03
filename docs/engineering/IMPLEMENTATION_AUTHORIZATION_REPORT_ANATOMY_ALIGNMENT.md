# IMPLEMENTATION_AUTHORIZATION_REPORT_ANATOMY_ALIGNMENT.md

## 1. Authorization Identity

| Property | Value |
|---|---|
| **Document** | IMPLEMENTATION_AUTHORIZATION_REPORT_ANATOMY_ALIGNMENT.md |
| **Purpose** | Convert `PROCESS_DETERMINATION_REPORT_ANATOMY_ALIGNMENT.md`'s conclusion into a bounded engineering envelope. Authorizes Implementation Planning within the scope below — nothing beyond it. |
| **Origin** | `PROCESS_DETERMINATION_REPORT_ANATOMY_ALIGNMENT.md` (Determination B: no Architecture Evaluation required); `CAPABILITY_DEFINITION_REPORT_ANATOMY_ALIGNMENT.md` (Determination A) — both treated as fixed, unreopened. |
| **Status** | **Authorization Granted. Authorizes Implementation Planning within the scope defined below. No code, TypeScript interface, or Rust struct has been produced in preparing this document.** |

## 2. Capability Being Authorized

Report Anatomy Alignment — eight console-presentation behaviors bringing the Reviewing state's Overview/Finding/Evidence display into conformance with already-frozen product/interaction design, using data already present in the current transport.

## 3. Authorization Basis

Derived exclusively from already-Adopted determinations:

- `FrontendArchitecture.md`, **Consumer-Owned State** — presentation of already-produced data is the console's own decision.
- `FrontendArchitecture.md`, **Explainable Continuity** — every value this authorization permits already traces to an existing, already-public getter surfaced in the current transport; nothing is inferred at the presentation layer.
- `FrontendArchitecture.md`, **Reserved Responsibilities** — checked directly: neither Initiative 1 nor Initiative 2 applies; this work concerns only how already-complete Report content is arranged, never when or whether it is live.
- `GOVERNANCE.md`'s **Console Crate Boundary Rule** — presentation of Assessment results, no business logic; squarely covers this work.
- `assessment.rs`'s own header comment: the transport remains "provisional... pending GOV-008" — unchanged by this authorization, since no field is added.

## 4. Authorized Scope

Implementation Planning is authorized to scope work that reorganizes and extends the *presentation* of data already delivered in the current `ReportSummary` payload, within `apps/console/src`, so that:

- The Overview presents Mod Health across its six named dimensions.
- Findings are grouped by dimension, severity-ordered within each group.
- A Finding's dimension, status, and (when present) a Recommendation headline become visible without expansion.
- Evidence availability becomes visible without expansion.
- Evidence content receives a further, explicit reveal step when substantial.

No other change is authorized by this document.

## 5. Eight Authorized Behaviors

**Item 1 — Mod Health Aggregate.** *Source-of-truth:* `WORKSPACE_EVOLUTION.md` §6. *Authorized behavior:* represented Mod Health dimensions are presented independently, using only existing `modHealthDimension` values already delivered on each Finding. *Forbidden:* any composite health value, score, formula, weighting, or synthesized health judgment — no dimension may be combined with another into any single output. *Current gap:* not implemented — `Overview.tsx` presents severity counts only. *Boundary:* `Overview.tsx` only. *Completion criterion:* for a real assessed fixture producing Findings across at least two dimensions, all represented dimensions are independently visible in the Overview, with no single combined indicator anywhere. *Verification:* traced against real fixture output.

**Item 2 — Category Grouping.** *Source-of-truth:* `THE_FINDING.md` §5. *Authorized behavior:* group Findings by their existing `modHealthDimension` value; no new taxonomy. *Current gap:* `Reviewing.tsx` sorts by severity only. *Boundary:* `Reviewing.tsx` only. *Completion criterion:* Findings render partitioned by dimension. *Verification:* traced against real fixture output.

**Item 3 — Severity Ordering Within Category.** *Source-of-truth:* `THE_FINDING.md` §5. *Authorized behavior:* preserve the existing, already-correct severity ordering (`SEVERITY_ORDER`/`severityRank`), nested inside each dimension group from Item 2. *Forbidden:* any change to severity semantics or ordering logic itself. *Boundary:* `Reviewing.tsx` only. *Completion criterion:* within each dimension group, Findings appear Error → Warning → Informational → BestPractice. *Verification:* traced against real fixture output.

**Item 4 — Finding Collapsed Anatomy.** *Source-of-truth:* `THE_FINDING.md` §2-4. *Authorized behavior:* make `modHealthDimension` and `status` visible on the collapsed Finding row (Title and Severity already are). *Forbidden:* introducing any field not already in `FindingSummary`. *Current gap:* both currently rendered only inside the expansion. *Boundary:* `Reviewing.tsx` only. *Completion criterion:* dimension and status readable for every Finding without expanding it. *Verification:* traced against real fixture output.

**Item 5 — Provisional/Final Visibility.** *Source-of-truth:* `THE_FINDING.md` §2. *Authorized behavior:* relocate display of the existing `status` value to the always-visible layer (folded into Item 4). **Explicitly stated, not merely implied:** every current Rule always constructs `FindingStatus::Final`; this authorization creates no Provisional behavior, and displaying "Provisional" will not occur under this work regardless of implementation quality. Future lifecycle behavior (Initiative 1) is out of scope and this authorization does not anticipate or prepare for it beyond correctly displaying whatever string the transport already sends. *Boundary:* `Reviewing.tsx` only. *Completion criterion:* status readable without expansion, currently always reading "Final" against real output. *Verification:* traced against real fixture output.

**Item 6 — Recommendation Headline.** *Source-of-truth:* `THE_FINDING.md` §4. *Authorized behavior:* show `finding.recommendation.action` (already a single string) at the collapsed layer when `recommendation !== null`. *Forbidden:* any new recommendation semantics, any new field, any change to `action`'s existing meaning or content — supplemented in visibility only, never replaced. *Current gap:* full Recommendation, including headline, currently shown only on expansion. *Boundary:* `Reviewing.tsx` only. *Completion criterion:* when a Recommendation exists, its `action` text is visible without expansion; full repair-step detail remains behind expansion, unaffected. *Verification:* traced against real fixture output, for both a Finding with and without a Recommendation.

**Item 7 — Evidence Availability.** *Source-of-truth:* `THE_FINDING.md` §2. *Authorized behavior:* indicate evidence availability using only `evidence.length`, already computable from the array already fully delivered in the initial payload. *Forbidden:* any new Tauri command, lazy fetch, or additional round trip — the existing single `submit_assessment` response remains the only data source. *Boundary:* `Reviewing.tsx` only. *Completion criterion:* an availability indicator is visible without expansion, correct for both `evidence.length > 0` and `evidence.length === 0`. *Verification:* traced against real fixture output, both cases.

**Item 8 — Evidence Progressive Disclosure.** *Source-of-truth:* `EVIDENCE.md` §4. *Authorized behavior:* on Finding-expansion, show `label`, `source`, and `description` as currently defined; require one further, explicit user action to reveal `content` when the content is substantial, using only the already-delivered string and without introducing a new data source. The concrete threshold/mechanism for what counts as "substantial" is not prescribed here and belongs to the Implementation Plan. **Critical boundary, stated explicitly per instruction:** this authorization does **not** create, rename, or resolve an "Explanation" field. `description` is not to be relabeled, restyled, or presented as "Explanation" anywhere — that remains Initiative 3's own unresolved Architectural Observation, untouched by this work. This item authorizes disclosure *timing/presentation* of the fields that exist today, under their current names, nothing else. *Boundary:* `Reviewing.tsx` only. *Completion criterion:* Label/Source/description visible on expansion for every Evidence item; a substantial `content` requires one further explicit action; a brief `content` may display inline; no field is renamed. *Verification:* traced against real fixture output, including at least one Evidence item with non-trivial `content` length.

## 6. Data / Transport Boundary

**Authorized to consume** (already present, unchanged): `modHealthDimension`, `status`, `recommendation.action`, `evidence`, `evidence.length`, `evidence.content`, `evidence.label`, `evidence.source`, `evidence.description`.

**Explicitly not authorized:** adding a field to any DTO; renaming any field; changing serialization; changing DTO structure; touching `assessment.rs`'s `From<&Finding>`/`From<&Evidence>` conversions; touching `engine/types.ts`; requiring any new Runtime data; resolving or advancing GOV-008 in any respect.

**Escalation condition:** if implementing any of the eight items is found to require a field not already present in `FindingSummary`/`EvidenceSummary`/`ReportSummary` today, implementation **stops** and the discovery is escalated for reassessment — it is not authorized by default, and does not silently expand this document's scope.

## 7. Architecture Boundary

**Not participating — confirmed unaffected by this authorization:** `modiq-runtime`, `modiq-engine`, `modiq-storage`, `modiq-collection`, `modiq-rules`, `modiq-report`, `modiq-cli`, `modiq-knowledge`, `modiq-versioning`, `apps/sandbox`, `apps/console/src-tauri` (the Rust bridge crate itself — unlike C1, this capability requires no Rust-side change at all, since no field is added).

If any unexpected architectural requirement is discovered during implementation, it is **not automatically authorized** — it triggers reassessment, not silent expansion.

## 8. Governance Boundary

No change to: GOV-002, C11, Initiative 1, Initiative 2, Initiative 3's Architectural Observations (including Evidence Explanation), Initiative 4/Confidence, GOV-008, C5, Knowledge governance. No new capability is introduced or implied by this authorization.

## 9. Explicit Exclusions

- **Error Experience Alignment** — not touched; `Workspace.tsx`'s error-state rendering is not modified by this authorization (see §10 — `Workspace.tsx` is not an authorized file).
- **AssessmentSubject / Report Identity / Report currency / reentrancy** — not touched; Item 1's dimensional presentation does not introduce a "what was assessed" statement of any kind.
- **C12 historical presentation, same-subject history, cross-subject comparison, platform-wide recurrence** — not touched by any of the eight items.
- **Evidence Explanation** — explicitly not created or resolved (Item 8, §5).

## 10. Authorized Files

**Authorized:**
- `apps/console/src/workspace/Reviewing.tsx`
- `apps/console/src/workspace/Overview.tsx`

**Conditionally touched:** none identified. Both files already receive every field this capability needs; no shared utility, type, or bridge file is demonstrated as necessary.

**Forbidden (not authorized under this document):** `apps/console/src/engine/types.ts`; `apps/console/src-tauri/src/assessment.rs`; `apps/console/src/engine/index.ts`; `apps/console/src/workspace/Workspace.tsx`; `apps/console/src/regions/*`; `apps/console/src/session/*`; any file outside `apps/console`.

**Maximum intended scope:** the two authorized files, limited to the eight behaviors above. No opportunistic refactor (e.g., deduplicating the two files' separately-defined `SEVERITY_ORDER` constants) is authorized unless strictly required to implement one of the eight items — none currently appears to require it.

## 11. Verification Gates

- `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace` — required as a boundary-confirmation gate even though no Rust file is expected to change, to prove no accidental Rust-side change occurred.
- `apps/console`'s `npm run build` (`tsc && vite build`) clean.
- Direct diff confirmation that `apps/sandbox` has zero changes (not a test run — nothing there should ever be touched).
- Direct diff/grep confirmation that no file outside §10's two authorized files changed.

## 12. Real-I/O Verification Discipline

Behavioral verification for all eight items must use real assessment output against the existing fixture path already exercised by `assessment.rs`'s own tests (`apps/console/src-tauri/fixtures/sample-mod`), not mocked `FindingSummary`/`EvidenceSummary` data — consistent with this repository's standing Real-I/O Testing Discipline and with C1's own precedent method (direct tracing against real fixture output). No GUI/display session is assumed available in this environment; verification proceeds by tracing real output values against each item's completion criterion, the same interpretive method C1's closeout used.

If this discipline is found insufficient to reliably verify any item, implementation **stops and escalates** rather than introducing a frontend testing framework unilaterally — consistent with the Process Determination's own finding that no such framework is currently required.

## 13. Completion Definition

Binary: this capability is complete only when **all eight items' completion criteria (§5) hold simultaneously** against real fixture output, **and** all verification gates (§11) pass clean, **and** no item in §9 (Explicit Exclusions) has been touched. Partial implementation (some but not all eight items) is not Engineering Release.

## 14. Implementation Report Requirements

The eventual Implementation Report must document: exact files changed (expected: the two in §10, nothing else); behavior implemented per item, traced against real output; verification gate results; any deviation from this authorization, classified (Planning defect vs. implementation defect, per this project's own standing discipline); explicit confirmation of scope compliance (§10); explicit confirmation that no item in §9 was touched.

## 15. Escalation / Stop Conditions

Implementation must stop and escalate, not proceed by judgment call, if: a field not already in the current transport is found necessary (§6); any file outside §10 is found necessary (§10); the Real-I/O discipline (§12) proves insufficient to verify any item; any of the eight items is found, on closer implementation-time inspection, to require touching Evidence Explanation, Report Identity, Initiative 1/2, Confidence, or historical data.

## 16. Final Authorization Determination

**A — AUTHORIZATION GRANTED.**

Implementation Planning for Report Anatomy Alignment is now authorized, strictly within the scope defined in this document (§4–§10). This capability's boundary is narrower than its own governing precedent (C1): two files, not three; zero transport changes, not one. The review corrections applied to this document (Item 1 stated presentation-only, with no composite/score/formula/weighting/judgment language; Item 8's disclosure threshold left open for the Implementation Plan rather than prescribed here; this determination stated as a granted authorization rather than a readiness assessment) are reflected throughout §5 and this section. This document does not authorize implementation itself — only the Implementation Plan that must follow it.

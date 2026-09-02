# Post-GOV-002 Engineering Prioritization Review

| Property | Value |
|---|---|
| **Document** | POST_GOV002_ENGINEERING_PRIORITIZATION_REVIEW.md |
| **Type** | Prioritization and evidence review. Not a Capability Definition, Architecture Evaluation, Architectural Resolution, or roadmap. |
| **Baseline** | Branch `feature/runtime-implementation`, local HEAD `2b9a855967d853e3f38190ac45e9fb74a0fdf499` (GOV-002 Readiness Assessment), origin at `3650f5c`, working tree clean. |

---

## 1. Purpose

Determine what, if anything, currently has the strongest evidence-based claim to become the next engineering capability after C12, applying the same discipline the GOV-002 Readiness Assessment just used: an architecturally Adopted, specified, or "obviously missing" item is not automatically a forcing function. This review evaluates; it does not authorize.

## 2. Re-Grounding — Fresh Against the Repository, Not the Prior Report

Verified directly this session, not carried forward:

- **Every Category A candidate from `CAPABILITY_PORTFOLIO_ASSESSMENT.md` has now been implemented.** Git history confirms C1 (RecommendationStep presentation, commits `d5b8735`→`69e33da`), C2 (Declared Dependency interpretation, `7c7faf8`→`9849a94`), C3 (CLI field parity, `31de26e`→`4c46283`), C4 (apps/sandbox field parity, `a024c8f`→`83925c0`), and C12 (Richer Historical Assessment Analysis, `dd4d377`→`3650f5c`) were all completed, in sequence, after the Portfolio Assessment recommended C1 first. **The Portfolio's entire "ready, no blocker" slate is now exhausted.** What remains in that Portfolio is exclusively Category B (C5, C6) and Category C (C7–C11) — none rated "ready for ordinary engineering workflow" at the time of that survey.
- **`ENGINEERING_LOG.md` has no entry for C3, C4, or C12.** Its last capability entry is C2 (2026-08-12); the file's own last content section afterward is a static "Engineering Methodology Observations" block dating to Sprint 13. This is a sharper staleness finding than the Post-C12 report's — that report flagged `PROJECT_STATUS.md`/`CHANGELOG.md` as date-stale; this confirms `ENGINEERING_LOG.md` is missing three complete capability entries outright. Documentation-currency work, not a capability candidate — noted for completeness, not evaluated further here.
- **Initiative 2 (Reentrant Assessment Lifecycle)'s blocking question is a genuine governance deadlock, not merely undecided.** `INITIATIVE_2_GOVERNANCE_RECONCILIATION.md:83`: *"Question 1 cannot presently be reconciled under the repository's current governance model... Documentation Authority... has no rule for a conflict where one side is ranked and the other is entirely absent from the ranking... `DataModel.md` and `THE_ASSESSMENT_REPORT.md` remain in direct, unmediated conflict."* This is stronger than a simple "Requires Governance" tag — no adopted mechanism currently exists to resolve it at all.
- **The console's intake picker is hardcoded to directories only.** `apps/console/src/regions/Workspace.tsx:26`: `open({ directory: true, multiple: false })`. The underlying engine already supports `.zip` archive assessment (`AssessmentService::execute_from_assessment_input` routes by `.zip` suffix, real and tested) — the console simply never exposes file selection. A mod downloaded as a `.zip` (the ordinary distribution form per `Vision.md`'s own description of the ecosystem) cannot be assessed through the console at all today.
- **Initiative 1 (Progressive Execution Observability)** was explicitly assessed at Sprint 21 and found "not Initiative-1-shaped" as a next milestone (`INITIATIVE_1_READINESS_ASSESSMENT.md:84`). Nothing in C1–C4 or C12 touched assessment latency or execution model; no new evidence exists to revisit that finding.
- **`.modiq-storage` still holds exactly 5 persisted reports** — unchanged since the Post-C12 report. Knowledge/history evidence volume has not grown.
- **GOV-002 Readiness Assessment is now complete** (`GOV002_READINESS_ASSESSMENT.md`): B — not established. GOV-002 remains Resolved; C11 remains Category C.

## 3. Candidate Evaluation

### Candidate A — AssessmentSubject / C11

- **Evidence:** Real — Initiative 3's Adopted Architectural Constraint (Report Identity blocked on Subject content); GOV-002 Resolved "no action"; a dedicated Readiness Assessment just completed.
- **Forcing Function:** None demonstrated in operation. `GOV002_READINESS_ASSESSMENT.md` §9/§11: no consumer has been blocked by Subject's absence in actual practice; the console's Reviewing view was built with Report Identity's absence already designed around, not encountered as friction.
- **Architectural Readiness:** Not applicable — blocked upstream of architecture, at governance.
- **Product Impact:** Would be high once unblocked (unlocks C11, bears on C5, would let C12's history aggregation group by subject) — but impact-if-unblocked is not evidence of present readiness.
- **Dependency:** Blocks C11 outright; indirectly relevant to C5's cross-mod question and to any future per-subject historical view.
- **Evidence Strength:** Weak (for advancement now).
- **Current Status:** **Intentionally Deferred** (freshly reconfirmed, not stale). Nothing has changed since the Readiness Assessment — it was completed against this same baseline. Remains deferred, per its own explicit conclusion.

### Candidate B — Evidence / Rule Breadth

Three sub-questions, already differentiated by `CAPABILITY_PORTFOLIO_ASSESSMENT.md` and unchanged since:

- **C5 (cross-mod dependency resolution):** a real, named, scoped architectural question (`STORAGE_ARCHITECTURE_EVALUATION.md:96`: "cross-mod collection validation" explicitly flagged as deferred). **Evidence Strength: Moderate.** No governance blocker — Category B, "Requires Architecture Evaluation." But no operational forcing function exists either: no evidence any real assessment has been blocked by inability to validate a declared dependency against an actual installation.
- **C7 (Lua Analysis):** GOV-014 (fixture provenance/licensing/storage policy) is Open and, by its own resolution text, must resolve *before* an Architecture Evaluation may even be authorized. **Evidence Strength: Weak.** A real product-specification gap (ProductSpecification.md names Lua analysis explicitly), but a governance precondition, not an architecture question, blocks even starting. Current Status: **Requires Governance** (GOV-014), prior to any architecture step.
- **C8 (dormant `EvidenceCategory` variants — AssetValidation, DependencyResolution, PerformanceObservations):** confirmed still dormant (zero Collector, zero Rule, zero fixture corpus, no investigation of any depth — a weaker starting point than Lua Analysis had even before its own INV-001). **Evidence Strength: Absent.** Current Status: **Premature**.

**Overall for Candidate B:** a specification gap is real (matches the Post-C12 report's finding), but no sub-question currently carries a demonstrated operating forcing function. C5 is the most architecturally mature of the three but is not yet forced.

### Candidate C — Console / User Experience

- **Assessment intake (single directory):** works. **Archive (.zip) intake:** does not exist — the picker is hardcoded directory-only (§2 above), while the engine already supports it. **Evidence Strength: Moderate-to-Strong as a gap; Weak as a forcing function** — no recorded instance of a real user or test attempting and failing this path; the gap is inferred from code, not from an encountered failure. Architecturally, this is the cheapest, lowest-risk item surveyed — identical construction shape to C1–C4 (expose an already-real engine capability), no governance, no architecture evaluation needed.
- **Progress/in-progress state (Initiative 1):** explicitly evaluated and found not yet warranted (Sprint 21); nothing since changed that. **No forcing function.**
- **Resubmission / new-assessment flow (Initiative 2):** the Reviewing view remains a genuine dead end for every console user who completes an assessment — this is a real, structural, currently-existing limitation, not speculative. But it is blocked by an irreconcilable governance conflict (§2 above) with **no currently adopted resolution mechanism at all** — a harder blocker than GOV-002's, which at least has a defined next-step shape (a Readiness Assessment path, as just demonstrated). **Current Status: Requires Governance**, with the governance path itself not yet defined.
- **Report Identity in console:** same GOV-002/C11 blocker as Candidate A.
- **Multi-file/drag intake:** a named, deliberate Sprint 21 scope exclusion, not an oversight. No forcing function; Category-A-shaped whenever pursued.

**Overall for Candidate C:** the resubmission dead-end is the most *product-relevant* real gap in the whole survey (every console user hits it), but it is the *most* governance-blocked, not the least. The archive-intake gap is the most *architecturally ready* item in the whole survey, but has no demonstrated forcing function. Neither combination is both forced and ready.

### Candidate D — Reporting Layer

- **Evidence:** `modiq-report`'s `AssessmentReport` remains a plain getter-only snapshot; CLI and console each independently format presentation. **New evidence this session:** the pattern of C3 (CLI field parity) and C4 (sandbox field parity) directly following C1/Sprint 22–24 shows a real, repeated coordination cost — each Runtime/Storage content addition has required separate, sequential parity-catch-up capabilities per consumer (three cycles for one underlying change, twice now).
- **Forcing Function / actual divergence:** **None found.** Parity has, in fact, been successfully maintained across all three consumers every time — that is precisely what C3 and C4 were for, and both succeeded. The recurring cost is coordination overhead, not incorrectness or user-facing inconsistency. No defect, no observed divergence, has ever been recorded.
- **Architectural Readiness:** a consolidation would still need to solve genuinely different presentation shapes per consumer (CLI plain text vs. console React DTO vs. sandbox's own DTO) — not obviously reducible by a shared crate, so readiness is unclear, not simply "yes."
- **Product Impact:** low directly (no user-visible defect); moderate indirectly (engineering velocity for future field additions).
- **Evidence Strength:** Weak-to-Moderate — real recurring cost observed, but not divergence, and not a design proven to reduce that cost.
- **Current Status:** **No Current Forcing Function.**

### Candidate E — Knowledge Activation

- **Evidence:** `modiq-knowledge` remains 174 lines, six of seven types zero-field placeholders; one hand-authored `RepairRecipe`, not derived from any assessment. `modiq-engine` still does not depend on it.
- **Assessment volume:** still exactly 5 persisted reports, synthetic/dev-fixture in character, unchanged since the Post-C12 report.
- **Consumers:** one static call site (`VersionCompatibilityRule`), unchanged.
- **Forcing Function:** none. A prior proposal for exactly this (Knowledge Feedback Loop) was evaluated and explicitly Suspended.
- **Evidence Strength:** Absent.
- **Current Status:** **Intentionally Deferred** — prior suspension remains appropriate; nothing has changed to revisit it.

### Candidate F — Other Existing Capability (Portfolio's Own Remaining Items)

Already surveyed within Candidates A/B/C above (C5, C7, C8, C11, Initiative 1/2 = C9). Two remaining Portfolio items not yet named:

- **C6 — Confidence as a first-class concept (Initiative 4):** scope and meaning Adopted; representation/computation mechanism "genuinely undesigned" per its own Architectural Resolution. **Category B** — no governance blocker, ready for a dedicated Architecture Evaluation, same standing as C5. No forcing function demonstrated: no Recommendation consumer currently needs a Confidence value it cannot get. **Current Status: No Current Forcing Function**, architecturally reachable via Architecture Evaluation whenever one arrives.
- **C10 — Extension Layer:** zero implementation, aspirational only, no Product/Interaction Design elaboration found. **Current Status: Premature.**

No new capability is manufactured here; none was found necessary.

## 4. Comparative Conclusion

| Candidate | Evidence | Forcing Function | Architecture Ready | Product Impact | Current Status |
|---|---|---|---|---|---|
| A — AssessmentSubject/C11 | Real (Adopted dependency) | None demonstrated | No — governed | High, if unblocked | Intentionally Deferred |
| B1 — C5 cross-mod | Real, named question | None demonstrated | Yes (Arch. Eval.) | Moderate | Requires Architecture Evaluation |
| B2 — C7 Lua Analysis | Spec-named only | None demonstrated | No — governed first | Moderate-High, if built | Requires Governance (GOV-014) |
| B3 — C8 dormant categories | Minimal | None | No — undesigned | Low-Moderate | Premature |
| C1 — Console archive intake | Real code-level gap | None demonstrated | Yes — cheapest of all | Moderate (real distribution form) | No Current Forcing Function |
| C2 — Console resubmission (Init. 2) | Real, encountered UX dead-end | Present, but unresolved via governance | No — deadlocked | High | Requires Governance (no mechanism exists) |
| C3 — Console progress state (Init. 1) | Evaluated, found premature | None | Unclear | Low-Moderate | No Current Forcing Function |
| D — Reporting consolidation | Real recurring cost, no divergence | None | Unclear | Low (direct) | No Current Forcing Function |
| E — Knowledge activation | Minimal, static | None | N/A | N/A until data exists | Intentionally Deferred |
| F — C6 Confidence | Adopted scope only | None demonstrated | Yes (Arch. Eval.) | Moderate | No Current Forcing Function |

## 5. Determination

### 1. Strongest next candidate
**None qualifies as both forced and architecturally ready.** The one candidate with a genuinely *present*, real, currently-encountered forcing function — Candidate C2, the console's resubmission dead-end — is the most governance-blocked item in the entire survey, with no adopted resolution mechanism at all (worse-positioned than GOV-002). The candidates that are architecturally readiest (console archive intake, C5, C6) have no demonstrated forcing function behind them at all — only a real, inferred gap.

### 2. Why
No candidate combines "something in actual operation currently needs this" with "the architecture is already sufficient to build it." Every item surveyed falls into exactly one of the two disqualifying quadrants the mission's own framework names: *important but premature* (C2/Initiative 2, C11/GOV-002, C7/GOV-014), or *architecturally ready but lacking a forcing function* (console archive intake, C5, C6, Reporting consolidation).

### 3. Why this beats — or rather, does not beat — AssessmentSubject/C11
It doesn't "beat" it in the sense of superseding it; no candidate does. Applying the exact same standard the GOV-002 Readiness Assessment just used (a concrete forcing function — real, operating implementation or consumer evidence — is required, not an Adopted-but-unbuilt dependency or a specified-but-unencountered gap), every other candidate fails that standard for the identical reason C11 just did. The review does not find a "better" candidate; it finds the same discipline applies uniformly, and nothing yet clears the bar it sets.

### 4. If nothing qualifies
**No capability currently has sufficient evidence to justify advancement.**

### 5. Immediate next legitimate action
None is authorized or recommended by this review. If the project wishes to move any candidate forward, the legitimate next steps, by candidate, would be: a governance mechanism for Initiative 2's Question 1 (none currently exists to invoke); an Architecture Evaluation for C5 or C6 (available now, no governance precondition); resolution of GOV-014 before any Lua Analysis Architecture Evaluation. This review does not perform, select, or authorize any of these.

---

## 6. Final Determination

**B — No capability currently has sufficient evidence to advance.**

Recommend maintaining the current engineering state. The repository is healthy, fully tested, and architecturally coherent at this baseline; no candidate surveyed presents both a demonstrated forcing function and architectural readiness simultaneously. The appropriate posture is to wait for a genuine forcing function — a real, encountered consumer or implementation need — rather than to generate further governance or architecture artifacts against any of the candidates surveyed here.

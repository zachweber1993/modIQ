# Engineering Release 0.5

| Property | Value |
|----------|-------|
| **Release** | 0.5 |
| **Documentation Release** | 2.1 (Frozen — Evidence Collection subsystem boundary; `DataModel.md` separately amended to v1.1.0 this Sprint) |
| **Milestone** | Sprint 5 complete (Phases 1–5, plus Closeout) |
| **Scope** | GOV-012 (Rule Evaluation Model) resolution; GOV-013 (FindingSeverity Severity/Kind Conflation) opened; the platform's first specification-level Finding Severity definitions; the Rule Engine's second concrete Rule and real multi-Rule dispatch; a Reporting scaffold-retirement investigation; repository reconciliation at Closeout |
| **Predecessor** | `docs/engineering/ENGINEERING_RELEASE_0.4.md` (Sprint 4) |
| **Governing ADRs** | None — no new architectural principle was established this Sprint; GOV-012 and GOV-013 are Governance Register items, not ADRs, consistent with this project's own criterion that an ADR records a new architectural principle, not an implementation-mechanism decision |
| **Governing Plan** | `SPRINT5_IMPLEMENTATION_PLAN.md` |

---

## Executive Summary

Sprint 4 (Engineering Release 0.4) closed with a named, deliberately unresolved question: what should Sprint 5 build? Two candidates were on record — a third Evidence Collector (XML inspection), or deepening the Rule Engine. The Technical Director chose neither by default: Sprint 5 was explicitly scoped to strengthen the platform vertically before widening it horizontally. "XML inspection remains the next collector, but it should build on a mature assessment model rather than drive its design."

Sprint 5 delivered that maturity. `RuleEngine::evaluate` — which had returned exactly one hardcoded outcome for any non-empty Evidence, regardless of category, since Sprint 1 — now dispatches two real, independent Rules and returns every outcome that actually applies. The second Rule, `StructuralDuplicationRule`, is the first Rule in this platform's history to evaluate Evidence *by category* rather than merely by presence. Getting there required a real Governance Register item (GOV-012) resolving three questions no prior Sprint had needed to ask, and — more consequentially — writing down, for the first time since Sprint 2, what `FindingSeverity`'s four variants actually mean. That act of writing surfaced a real modeling tension: `BestPractice` classifies a Finding's *kind*, not its severity, unlike the other three. The Technical Director did not resolve this from two Rules' worth of evidence; a second Governance Register item, GOV-013, was opened instead, deliberately Open, provisionally accepting the current model rather than deciding a Runtime change ahead of the evidence to decide it well.

This release is a consolidation, following the same charter Engineering Releases 0.3 and 0.4 set: it records what five phases and a Closeout already established, corrects documentation that went stale mid-Sprint despite Sprint 4 Closeout's own process-improvement directive, and recommends — without deciding — what Sprint 6 should contain.

**One finding from this Closeout is worth stating plainly, because it is the same finding twice more:** `PROJECT_STATUS.md` and `CHANGELOG.md` had gone stale again, mid-Sprint, exactly as they did before Sprint 3's own closeout and before Sprint 4's. This is now the *third* consecutive Sprint this pattern has recurred, despite being named and corrected at each of the prior two closeouts. Sprint 4 Closeout secured a standing directive — "future sprints should formally include repository reconciliation... as part of sprint closure" — and that directive was honored exactly as written: this Closeout performed the reconciliation. What it did not do, because nothing in the directive asked it to, is prevent the staleness from occurring *between* closeouts. See Lessons Learned.

---

## Scope of Sprint 5

### Delivered

- **Phase 1 (Design Preparation).** `SPRINT5_IMPLEMENTATION_PLAN.md` drafted, naming five Design Questions and one candidate governance item rather than presuming answers, mirroring `SPRINT4_IMPLEMENTATION_PLAN.md`'s own discipline. GOV-012 (Rule Evaluation Model) approved by the Technical Director in full: `RuleEngine::evaluate` returns `Vec<RuleOutcome>` (zero, one, or several outcomes); Rules dispatch in fixed, explicit declaration order, never an order derived from Evidence's own arrival sequence; Rules compose independently, with no suppression model between a specific Rule and the existing generic one. `FindingSeverity` semantic definitions drafted for the first time in this project's history. Technical Director review of those definitions surfaced GOV-013 (FindingSeverity Severity/Kind Conflation), opened deliberately Open — not resolved, provisionally accepted, to be revisited once the Rule Engine has multiple concrete Rules operating in practice. Both governance items formally inserted into `GOVERNANCE.md`; the `FindingSeverity` definitions inserted into `DataModel.md`, amended 1.0.1 → 1.1.0.
- **Phase 2 (Second Real Rule).** `StructuralDuplicationRule` implemented (`crates/modiq-rules/src/rules/structural_duplication_rule.rs`): evaluates only `EvidenceCategory::StructuralDuplication` Evidence, assigns `FindingSeverity::Warning` per the newly-recorded definitions (a genuine reliability concern, not conclusive proof of breakage), ignores every other category. Scoped deliberately narrowly — not yet wired into `RuleEngine::evaluate`, mirroring how `ArchiveReader`/`ArchiveEvidenceBuilder` existed, real and tested, before Sprint 4 Phase 3D's routing existed to reach them.
- **Phase 3 (Multi-Rule Evaluation Assembly).** The original Sprint 1 Rule extracted into its own unit, `EvidencePresenceRule`, identical in behavior, matching `StructuralDuplicationRule`'s shape. `RuleEngine::evaluate` rewritten as a two-Rule dispatcher: two `if let` checks in fixed declaration order, per GOV-012's resolved policy — no trait, registry, factory, or plugin mechanism. `modiq-engine`'s `AssessmentService::execute` updated internally to loop over the new `Vec<RuleOutcome>`; its own public signature is completely unchanged, confirmed by every pre-existing test passing unmodified.
- **Phase 4 (Reporting Scaffold Investigation).** Investigated whether `modiq-report`'s four unused scaffold types (`FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, `ReportFormatter`) should be built out or retired, checking specifically whether Sprint 5's own new severity differentiation created a real need for any of them. It did not: zero construction sites, unchanged since the platform's first commit, and the Sandbox's own frontend — the one real consumer — required no changes to correctly display the new `Warning` severity, because it already renders whatever severity string a Finding happens to carry. Recommended retirement, by the same evidentiary method GOV-004 used. No `modiq-report` code was written or deleted, per Technical Director direction.
- **Phase 5 (Testing & Verification).** Closed a genuine determinism-testing gap found by re-reviewing Phase 2/3's own suite before treating this phase as pure re-confirmation: nothing had proven `RuleEngine::evaluate`'s outcome order is independent of Evidence *arrival* order, only that it is stable across repeated *identical* input. Added a direct test evaluating the same two Evidence items in both possible orderings.

### Deliberately Not Delivered

- A third Evidence Collector (XML inspection) — explicitly deferred by Technical Director direction; this Sprint's entire charter was to mature the Rule Engine first.
- Any resolution of GOV-013 — deliberately left Open, not decided from two Rules' worth of evidence.
- Any action on Phase 4's retirement recommendation — `modiq-report` remains untouched in source; building out or retiring its four scaffold types is deferred to a later sprint.
- CLI wiring — remains the other named parallel track, independent of this Sprint.
- Resolution of GOV-001, GOV-002, GOV-003, or GOV-008.
- A standalone ADR for anything this Sprint decided — GOV-012 and GOV-013 are Governance Register items; neither establishes a new architectural principle in the sense `docs/adrs/README.md` reserves for an ADR.

---

## Major Architectural Accomplishments

- **"Capability before abstraction" was validated a sixth time, in a new crate.** Every prior application (the Rule trait question, Collector dispatch at GOV-004, Sprint 4's own routing decision) concerned orchestration or dispatch between subsystems. This Sprint applied the identical principle inside a single crate's own internal Rule dispatch, and again inside Reporting's scaffold-retirement question (Phase 4) — the same discipline, a third domain.
- **A genuine architectural tension was surfaced by writing documentation, not by writing code.** GOV-013 did not originate from an implementation bug or a test failure; it originated from the act of defining `FindingSeverity`'s four variants precisely enough for a Rule to use them, and discovering under that scrutiny that one variant does not belong to the same axis as the other three. This is a different *kind* of discovery than this platform's prior architectural findings (GOV-004's unused stubs, GOV-011's dependency limitation), worth naming as its own category: specification work itself as a source of architectural evidence.
- **The Technical Director's resolution of GOV-013 is itself an application of the evidence-based principle this platform has followed since GOV-004.** Declining to restructure `FindingSeverity` from two Rules' worth of evidence, while formally recording the tension as Open rather than silently absorbing it into the `FindingSeverity` definitions, keeps the same discipline that has governed every other "not yet enough evidence" decision on this platform.
- **The Evidence-first, Rule-independent composition model (GOV-012, Question 3) was chosen specifically to avoid entangling this Sprint with GOV-008.** An out-of-band composition or suppression mechanism was considered and rejected in the underlying `PROPOSAL`-equivalent reasoning (`SPRINT5_IMPLEMENTATION_PLAN.md`, Design Question 3) precisely because it risked requiring a new `AssessmentService` signature — the same discipline the Sprint 4 Phase 3C Architecture Review applied when choosing Evidence representation over an out-of-band Collection field.

---

## Major Implementation Accomplishments

- Two real, independent Rules now exist where one hardcoded Rule existed since Sprint 1 — the first time this platform's Rule Engine has ever needed to *select* among Rules rather than apply the only one unconditionally.
- `modiq-rules` grew from 3 tests (Engineering Release 0.4) to 15: `EvidencePresenceRule` (3, moved unchanged), `StructuralDuplicationRule` (6, covering every reachable outcome), and `RuleEngine`'s own dispatch (6, covering no-match, single-match, both-match with correct order and severity, multiple-matching-items, repeated-call determinism, and — closed in Phase 5 — order independence from Evidence arrival order).
- `AssessmentService::execute` changed its internal Rule-evaluation loop without changing its own public signature — confirmed, not assumed, by every one of Sprint 3's and Sprint 4's own `execute`/`execute_from_assessment_input` tests passing unmodified.
- The first specification-level definition, anywhere in this project's history, of what `FindingSeverity`'s four variants actually mean — closing a gap that had existed since Sprint 2 and been exercised by exactly one variant (`Informational`) for three Sprints.

---

## Governance Completed

| Item | Resolution | Phase |
|---|---|---|
| GOV-012 | Rule Evaluation Model resolved in full: `RuleEngine::evaluate` returns `Vec<RuleOutcome>`; Rules dispatch in fixed, explicit declaration order; Rules compose independently, no suppression model. | Phase 1 |
| GOV-013 | FindingSeverity Severity/Kind Conflation — opened, deliberately left Open. `FindingSeverity` provisionally accepted as-is for Sprint 5; not confirmed permanently correct; to be revisited once the Rule Engine has multiple concrete Rules operating in practice. | Phase 1 |

**Still open:** GOV-001, GOV-002, GOV-003, GOV-008 (untouched this Sprint), and now GOV-013 (opened this Sprint, deliberately not closed). Total Governance Register size: 13 items, 8 Resolved (GOV-004, 005, 006, 007, 009, 010, 011, 012), 5 Open (GOV-001, 002, 003, 008, 013).

No ADR was created. `SPRINT5_IMPLEMENTATION_PLAN.md`'s own Governing ADRs field states why: neither GOV-012 nor GOV-013 establishes a new architectural principle in `docs/adrs/README.md`'s sense — GOV-012 is an implementation-mechanism decision within the already-established "capability before abstraction" principle (ADR-0010 already covers the principle itself), and GOV-013 is an open question, not a decision to record.

---

## Documentation Completed

- **`GOVERNANCE.md`** — GOV-012 and GOV-013 inserted verbatim into the Governance Register, immediately following GOV-011, per the two-step drafting discipline (`GOV-012_AND_FINDINGSEVERITY_PREPARATION.md` staged the text; `GOVERNANCE.md` was amended only after Technical Director review).
- **`DataModel.md`** amended 1.0.1 → 1.1.0: a new `### Finding Severity` subsection under `## Finding`, recorded explicitly as the platform's *provisional* interpretation, with a direct cross-reference to GOV-013 rather than presenting the definitions as settled.
- **Two new engineering documents** — `GOV-012_AND_FINDINGSEVERITY_PREPARATION.md` (the governance/specification drafting, staged for review before either canonical document was touched) and `SPRINT5_PHASE4_REPORTING_INVESTIGATION.md` (the Reporting scaffold investigation and recommendation) — both retained as historical records of the design and investigation work that preceded or substituted for implementation, per this project's established convention.
- **Living-document reconciliation at Closeout** — `PROJECT_STATUS.md` and `CHANGELOG.md`, both found stale mid-Sprint despite Sprint 4 Closeout's own standing directive, corrected: new milestone/phase/focus fields, a new `## Sprint 5 — Complete` section, a new `# [Sprint 5]` CHANGELOG entry. A genuine documentation error, independent of Sprint 5's own work, was found and corrected in the same pass: both files referenced a `SPRINT4_RETROSPECTIVE.md` that was never created (Sprint 4's retrospective has always lived inside `ENGINEERING_RELEASE_0.4.md` itself). `docs/README.md`'s Engineering Release cross-reference corrected to 0.5. `CrateRoadmap.md` gained a `## Sprint 5 — Complete` narrative section, revision-history rows 1.15.0–1.17.0, and an updated `modiq-report` crate-table row recording Phase 4's recommendation.

---

## Testing Growth

| Milestone | Root Workspace Tests | Sandbox Tests |
|---|---|---|
| Engineering Release 0.4 | 150 | 6 |
| End of Sprint 5 Phase 2 | 156 | 6 |
| End of Sprint 5 Phase 3 | 161 | 6 |
| End of Sprint 5 Phase 5 (this release) | **162** | **6** |

By crate, at this release: `modiq-runtime` 82, `modiq-collection` 43, `modiq-engine` 16 unit + 3 integration, `modiq-report` 3, `modiq-rules` 15 (up from 3). Zero tests are ignored or flaky anywhere in the workspace, verified by direct execution while preparing this release.

**Phase 4 did not change the testing count at all** — the first phase in this platform's history whose deliverable was an investigation document rather than code, and whose completion criteria were satisfied without touching `cargo test` at all. Worth naming as a genuinely different phase shape from every one that preceded it.

---

## Repository Maturity Assessment

| Area | Classification | Basis |
|---|---|---|
| Workspace organization | **Stable** | Nine crates, unchanged in count since Engineering Release 0.3. |
| Crate boundaries | **Stable** | Zero violations across five phases plus Closeout — `RuleEngine`'s new multi-Rule dispatch stayed entirely inside `modiq-rules`; `modiq-engine`'s only change was internal loop handling, not a boundary crossing. |
| Dependency direction | **Stable** | Unchanged; no new dependency edge introduced this Sprint (first Sprint since 3 with zero new crate-to-crate or external dependency edges). |
| Documentation consistency | **Recurring, now three-for-three.** Engineering Release 0.3 named the pattern; Engineering Release 0.4 confirmed it recurred once; this release confirms it recurred again, identically, despite Sprint 4 Closeout's own standing process directive. The directive was honored exactly as written — reconciliation happened at Closeout — but nothing about *when documents get touched mid-Sprint* has changed. See Lessons Learned. |
| Testing maturity | **Improved.** `modiq-rules` went from the platform's least-exercised real-Rule crate (one Rule, three tests, unchanged since Sprint 1) to its most recently and thoroughly extended, with an outcome-per-test discipline matching Collection's own standard. |
| Engineering workflow | **Stable.** The proposal-then-implement discipline held for a sixth and seventh time (GOV-012's drafting, the `FindingSeverity` definitions' drafting) with zero post-implementation rework required in either case. |

---

## Crate Maturity Review

| Crate | Maturity | Remaining Work | Architectural Confidence | Priority |
|---|---|---|---|---|
| `modiq-runtime` | L3, 82 tests, unchanged this Sprint | `Display`/`Serialize` for identity/enum types (flagged repeatedly, still open); referential integrity for Finding/Recommendation references; GOV-013's eventual resolution, if it results in a `FindingSeverity` change | High — untouched by five phases of neighboring Rule Engine work | Low — stable, revisit only when GOV-013 or a concrete forcing function arrives |
| `modiq-rules` | L3, 15 tests (up from 3) | A third Rule, if and when a concrete case justifies one; GOV-013's resolution depends on this crate gaining more Rules to evaluate the question against | High — two real, independent Rules now proven, dispatch pattern validated | **High** — this crate is now where the platform's evaluative value proposition is built, alongside Collection |
| `modiq-collection` | L2, 43 tests, unchanged this Sprint | XML, Lua, manifest, dependency-analysis collectors, all still deferred per Sprint 5's own charter | High — unaffected by five phases of neighboring work | Medium — the next collector should build on this Sprint's maturity, per Technical Director direction, not before it |
| `modiq-engine` | L3, 16 unit + 3 integration tests, unchanged in count | Resolve GOV-008 when implementation evidence justifies it | High — internal Rule-loop change required zero signature change | Low — stable |
| `modiq-report` | L3, 3 tests, unchanged since Sprint 2 | Four scaffold types recommended for retirement (Phase 4), not yet acted on; GOV-001 still bears directly on this crate | High for what exists; the retirement recommendation, if acted on, would simplify rather than grow this crate | Medium — the retirement decision is a small, low-risk action once scoped |
| `modiq-knowledge` | L1, pure scaffold, unchanged since Sprint 0 | Everything; a second Rule (`StructuralDuplicationRule`) still did not need it — `RuleReference` remains an opaque, `modiq-rules`-internal identifier | Low — untested against any real usage, five Sprints running | Low — correctly sequenced after real Rule diversity actually needs shared Knowledge content, which two Rules still does not demonstrate |
| `modiq-versioning` | L1, pure scaffold, unchanged since Sprint 0 | Everything; Rule Selection remains Evidence-Category-only, not Version-Profile-aware, a real and acknowledged narrowing of `RuleEngine.md`'s full Rule Selection responsibility | Low — untested against any real need | Low — no forcing function exists yet |
| `modiq-cli` | L1, pure scaffold, unchanged since Sprint 0 | Wiring `AssessCommand`, mirroring the Sandbox's now-thrice-proven thin-client pattern | High — the pattern is more de-risked than ever | Medium — genuinely low-risk, independent of this Sprint's work |
| `modiq-common` | L1, empty stub files, unchanged since Sprint 0 | Undecided | Low — zero evidence, after eleven phases across five Sprints, that this crate is needed | Low — do not invent a use for it |

---

## Technical Debt Review

### Intentional Technical Debt
*(deliberate, documented, correct to leave as-is for now)*

- GOV-013 remains open by design — not an oversight, a deliberate acceptance that two Rules is insufficient evidence to restructure a Runtime type.
- No dispatch abstraction for Rule selection — evaluated and declined a second time this Sprint (having already been declined once for Collector dispatch), on the same principle.
- `modiq-report`'s four scaffold types remain in source, unbuilt and unretired — a deliberate deferral (Phase 4's own scope), not an oversight.

### Future Capabilities
*(deferred by design, not yet started, no urgency implied)*

- XML inspection, Lua inspection, manifest analysis, dependency analysis — still the next Collector capabilities in sequence, explicitly deferred behind this Sprint's own charter.
- A third Rule, and whatever GOV-013 resolution that Rule's existence might inform.
- Knowledge Domain integration, Version Profile integration, CLI wiring, persistent assessment storage.

### Known Limitations
*(real, current, will not resolve without deliberate work)*

- Missing `Display`/`Serialize` for Runtime identity/enum types — flagged at Sandbox Phase 2, Sprint 3 Phase 1, Engineering Release 0.3, Engineering Release 0.4, and again here: **this is now the sixth consecutive release record to note it without scheduling it.**
- Documentation staleness between closeouts — now confirmed across three consecutive Sprints (Sprint 3, Sprint 4, Sprint 5), despite Sprint 4 Closeout's own process directive. See Lessons Learned for what this suggests.
- Git tag hygiene: `v0.2.0` and `v0.3.0` predate `v0.1.0-alpha` chronologically (pre-existing, flagged repeatedly). **`v0.5.0` does not currently exist as a tag** — like `v0.4.0` before it, this release could be tagged cleanly if desired; this document does not create that tag.
- `docs/governance/ROADMAP.md` and `docs/governance/EngineeringGuide.md` remain stale since Sprint 0/1 — named again, not addressed, consistent with every release record since Engineering Release 0.4.

### Deferred Enhancements
*(nice-to-have, no urgency, no forcing function yet)*

- Retiring (or building out) `modiq-report`'s four scaffold types, per Phase 4's recommendation — a real, scoped, low-risk action item for a near-future sprint, not an open design question.
- `CrateRoadmap.md`'s "Exit Criteria" section still has no Sprint 3, 4, or 5 entry — a gap now three Sprints old, still not created asymmetrically for any single Sprint alone.

---

## Sprint 5 Retrospective

**What went well.** The decision to build the assessment model before widening Evidence Collection produced a genuinely different kind of Sprint than 3 or 4 — no new dependency, no new adversarial surface, no new crate — and the "capability before abstraction" principle, tested for a sixth time, held with zero exception. The proposal-then-implement sequencing caught nothing needing rework this Sprint (unlike Sprint 4's Question 2 correction), because the underlying policy questions (GOV-012) were narrower and more fully specified before implementation began.

**What surprised us.** That writing a specification could itself surface an architectural question, rather than implementation surfacing one. GOV-013 exists because Phase 1 tried to define `FindingSeverity` precisely enough for a Rule to use, not because any code broke or any test failed. This is a new category of finding for this platform's own history — worth watching for again, not assuming was a one-time event.

**Architectural decisions validated by implementation.** GOV-004's "capability before abstraction" held for a second crate (`modiq-rules`, not just `modiq-collection`/`modiq-engine`), and for a third domain (Reporting, Phase 4) — increasingly strong evidence this is a genuine platform-wide discipline, not a pattern specific to Collector dispatch. `RuleEngine.md`'s own conceptual framing (Rule Selection, Evidence Evaluation, Finding Generation as *separate conceptual responsibilities*, fulfilled inline per GOV-004) mapped cleanly onto two real Rules without needing revision.

**Areas where the architecture proved stronger than expected.** `Finding`'s and `Recommendation`'s existing shape (`rule_reference: RuleReference`, singular; `evidence_ids: Vec<EvidenceId>`) required zero change to support a second Rule — the one-Rule-per-Finding design Sprint 2 chose, seemingly for a single-Rule platform, turned out to already be exactly right for a multi-Rule one.

**Areas requiring future attention.** GOV-008 has now gone three consecutive Sprints untouched. GOV-013 is genuinely open and will need real evidence — a third Rule, ideally one that actually needs `BestPractice` — to move forward; nothing in Sprint 5's own scope produced that evidence, deliberately. Documentation staleness between closeouts is now a three-Sprint pattern; Lessons Learned treats this as needing a different kind of fix than has been tried twice already.

---

## Remaining Risks

- **Documentation staleness recurring a third consecutive time**, despite a standing directive intended to address it — the most important process risk this release identifies, ahead of any technical one. See Lessons Learned.
- **GOV-008 and GOV-013 both aging without resolution**, for different reasons (GOV-008 lacks implementation evidence; GOV-013 lacks Rule diversity) — worth tracking together as this platform's two open "insufficient evidence yet" governance items.
- **The Reporting scaffold-retirement recommendation is real, scoped work sitting undone.** Low risk in itself, but a genuine to-do that could be lost track of if not carried forward explicitly into Sprint 6 planning.
- **`modiq-knowledge` has now gone five Sprints with zero implementation and zero forcing function**, including through a Sprint that added a second Rule specifically. If a third Rule also doesn't need it, this crate's continued deferral deserves an explicit look, not just a repeated note.

---

## Lessons Learned

- **A standing process directive can be honored exactly as written and still not fix the underlying problem it was aimed at.** Sprint 4 Closeout's directive — "future sprints should formally include repository reconciliation... as part of sprint closure" — was followed precisely: this Closeout performed exactly that reconciliation. The staleness still happened, because the directive addressed *catching* staleness at Closeout, not *preventing* it mid-Sprint. Three consecutive occurrences of the identical pattern (Sprint 3, 4, 5) is no longer evidence that closeout-time reconciliation is insufficient — it is evidence that the actual fix has to touch when these two files get edited, not just whether they eventually get corrected. A concrete candidate for Sprint 6: update `PROJECT_STATUS.md`'s "Current Phase" line as part of each phase's own report, not only at Sprint close.
- **Specification-writing is a legitimate, distinct source of architectural findings, alongside implementation and testing.** GOV-013 is the first governance item on this platform's record to originate from writing a definition rather than from writing or running code. Future Sprints that involve defining previously-implicit semantics (this platform has several: `EvidenceCategory`'s own criteria for what belongs in it, `AssessmentContext`'s eventual real content) should expect the same kind of discovery, and treat it as legitimate engineering signal, not a detour.
- **A two-step drafting discipline (stage the governance/specification text for review; amend the canonical document only after approval) scales down cleanly to a small decision, not just a large one.** `GOV-012_AND_FINDINGSEVERITY_PREPARATION.md` applied the same pattern `PROPOSAL_GOV-011.md` established for a much larger governance resolution, and it worked identically well for a narrower one — worth reaching for by default, not reserving for big decisions only.

---

## Engineering Metrics

| Metric | Value |
|---|---|
| Workspace crates | 9 (unchanged since Engineering Release 0.3) |
| Governance items | 13 total — 8 Resolved (GOV-004, 005, 006, 007, 009, 010, 011, 012), 5 Open (GOV-001, 002, 003, 008, 013) |
| Documentation Release | 2.1, Frozen; `DataModel.md` separately amended to v1.1.0 this Sprint |
| Engineering Release | 0.5 (this document) — not yet tagged; `v0.5.0` is available (no collision) |
| Root workspace tests | 162 (up from 150 at Engineering Release 0.4) |
| Sandbox tests | 6 (unchanged) |
| Major milestones completed | Documentation Releases 1.0, 2.0, 2.1; Sprint 0–4; Engineering Releases v0.1.0-alpha, 0.2, 0.3, 0.4; Platform Validation Phase 1; Sprint 5 Phases 1–5; Sprint 5 Closeout |
| Implementation readiness | Architecturally and functionally ready for the next capability. Nothing has been committed to git for the whole of Sprint 5 as of this document's writing — a materially different position from Sprint 4's own history of committing per-phase, worth the Technical Director's attention when scoping the commit sequence for this release. |

---

## Repository Timeline

```
Documentation Release 1.0
        ↓
Sprint 0
        ↓
Sprint 1
        ↓
Engineering Release v0.1.0-alpha
        ↓
Sprint 2
        ↓
Engineering Release 0.2
        ↓
Sprint 3 (Phases 1–5)
        ↓
Documentation Release 2.1
        ↓
Engineering Release 0.3
        ↓
Platform Validation Phase 1 — GOV-004 resolved; GOV-008 deferred
        ↓
Sprint 4 (Phases 1–3D, plus Closeout)
        ↓
Engineering Release 0.4
        ↓
Sprint 5 Phase 1 — GOV-012 resolved; GOV-013 opened; DataModel.md v1.1.0
        ↓
Sprint 5 Phase 2 — StructuralDuplicationRule
        ↓
Sprint 5 Phase 3 — RuleEngine multi-Rule dispatch
        ↓
Sprint 5 Phase 4 — Reporting scaffold investigation
        ↓
Sprint 5 Phase 5 — determinism gap closed
        ↓
Sprint 5 Closeout — repository reconciliation
        ↓
Engineering Release 0.5
        ↓
Sprint 6 (not yet scoped)
```

---

## Recommendation

**Sprint 6 is ready to be scoped.** The architecture held under a second Rule's worth of pressure, GOV-013 was surfaced and handled with the same evidentiary discipline every other open governance question on this platform has received, and both workspaces are fully green. Nothing in this release blocks further work.

**What Sprint 6 should contain is not decided here**, consistent with this release's own charter. Three candidates are on record, none scoped: XML inspection (the next Evidence Collector, now with a materially more mature Rule Engine to build on, exactly as this Sprint intended); CLI wiring (independent, low-risk, more de-risked every Sprint the Sandbox's thin-client pattern is reused); and acting on Phase 4's Reporting scaffold-retirement recommendation (small, low-risk, currently unscheduled).

**One process item deserves attention before or very early in Sprint 6, having now recurred three times identically:** documentation staleness between closeouts. This release's own Lessons Learned names a concrete candidate — touching `PROJECT_STATUS.md`'s "Current Phase" per-phase rather than only at Sprint close — worth the Technical Director's explicit consideration rather than a fourth repeated flag.

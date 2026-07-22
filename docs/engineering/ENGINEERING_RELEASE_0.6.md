# Engineering Release 0.6

| Property | Value |
|----------|-------|
| **Release** | 0.6 |
| **Documentation Release** | 2.1 (Frozen — unchanged this Sprint; no specification amendment) |
| **Milestone** | Sprint 6 complete (CLI wiring, `modiq-report` scaffold retirement, plus Closeout) |
| **Scope** | `modiq-cli` wired to `modiq-engine` for the first time since Sprint 0; `modiq-report`'s four unused scaffold types retired under explicit authorization; Technical Director role renamed Chief Architect and the project's governance baseline reconciled across the three handoff documents; repository reconciliation at Closeout |
| **Predecessor** | `docs/engineering/ENGINEERING_RELEASE_0.5.md` (Sprint 5) |
| **Governing ADRs** | None — no new architectural principle was established this Sprint; the CLI-wiring and scaffold-retirement decisions apply existing principles (direct composition, capability-before-abstraction) rather than establishing new ones |
| **Governing Plan** | `SPRINT6_IMPLEMENTATION_PLAN.md` (including its Authorization Record) |

---

## Executive Summary

Engineering Release 0.5 closed with three roadmap candidates on record, none scoped: XML inspection, CLI wiring, and acting on the Reporting scaffold-retirement recommendation. The Chief Architect scoped Sprint 6 to the latter two — the two smaller, independent, already-de-risked items — deferring XML inspection, the larger of the three, to a Sprint of its own.

`modiq-cli` — scaffolded since Sprint 0, unwired through five Sprints — became a real, working second consumer of `AssessmentService` this Sprint. `Application` dispatches `assess`/`help`/`version` by one direct match; `AssessCommand` calls `AssessmentService::execute_from_assessment_input` against a real, user-supplied path, reusing the Sandbox's own thin-client pattern rather than inventing a new one, with no new external dependency (argument parsing is manual, per explicit Chief Architect direction). `modiq-report`'s four scaffold types, recommended for retirement at Sprint 5 Phase 4 and left undone through that Sprint's own close, were deleted under this Sprint's explicit, separate authorization; `AssessmentReport`, the crate's real and only tested content, is unchanged.

This Sprint also carried a significant governance and terminology transition: the project's top authority role was renamed from Technical Director to Chief Architect, the three canonical handoff documents (`PROJECT_HANDOFF_v1.0.md`, `CHIEF_ARCHITECT_HANDOFF_v1.0.md` — itself a rename of `TECHNICAL_DIRECTOR_HANDOFF_v3.0.md` — and `LEAD_ENGINEER_HANDOFF_v3.0.md`) were reconciled to the new terminology, and a permanent engineering workflow diagram and a Sources of Authority model (distinguishing normative from descriptive authority) were established in `CHIEF_ARCHITECT_HANDOFF_v1.0.md` for the first time.

**One finding from this Closeout is the same finding recorded at every closeout since Sprint 3, now for a fourth time, and more severe than before.** `PROJECT_STATUS.md` and `CHANGELOG.md` had gone stale again, unchanged since Sprint 5 Closeout. This time the staleness also reached the three documents this project's own governance baseline now calls authoritative — `PROJECT_HANDOFF_v1.0.md` specifically still claimed `modiq-cli` was "not wired to `modiq-engine`," a stale content claim, not merely a stale status label. All corrected during this Closeout; see Lessons Learned.

---

## Scope of Sprint 6

### Delivered

- **CLI wiring.** `modiq-cli`'s `Application`, `AssessCommand`, `HelpCommand`, and `VersionCommand` — one-line unit structs since Sprint 0 — implemented in full. `AssessCommand::run` calls `AssessmentService::execute_from_assessment_input` and maps the result to a three-tier exit-code convention: 0 (success), 1 (execution failure — `CollectionError`, a well-formed input that failed during execution), 2 (invalid usage — CLI-level usage errors and `AssessmentInputError` alike, both invalid before execution begins). `modiq-cli/Cargo.toml` gained its first-ever dependencies: `modiq-runtime`, `modiq-engine`, and `modiq-report` (the last needed to name `AssessmentReport` directly, since `modiq-engine` does not re-export it — see Major Architectural Accomplishments).
- **`modiq-report` scaffold retirement.** `FindingSummary`, `RecommendationSummary`, `TraceabilityReport`, and `ReportFormatter` — recommended for retirement at Sprint 5 Phase 4, zero construction sites ever — deleted under this Sprint's explicit, separate Chief Architect authorization, distinct from the general Sprint scope authorization. `modiq-report` now contains only `AssessmentReport`.
- **Governance and terminology baseline.** The Technical Director role renamed Chief Architect across the three canonical handoff documents. `PROJECT_HANDOFF_v1.0.md` gained a Project Structure diagram (Project Owner → Chief Architect → Lead Engineer → Implementation → Chief Architect Review → Sprint Closeout). `CHIEF_ARCHITECT_HANDOFF_v1.0.md` — renamed from `TECHNICAL_DIRECTOR_HANDOFF_v3.0.md` — gained an explicit Mission, a Primary Responsibilities list, a permanent Engineering Workflow diagram (Sprint Planning → Chief Architect Review → Authorization → Implementation → Validation → Implementation Report → Architecture Review → Sprint Closeout), and a Sources of Authority section distinguishing normative authority (Governance Register, ADRs, specifications, handoff documents — what the architecture *should* be) from descriptive authority (repository implementation — what the system *currently* does), with divergence between the two treated as architectural drift to investigate, never a silent change in authority.
- **Repository Closeout.** `PROJECT_STATUS.md`, `CHANGELOG.md`, `ENGINEERING_LOG.md`, `docs/README.md`, and `CrateRoadmap.md` reconciled against Sprint 6's actual state, correcting staleness that had reached the three authoritative handoff documents for the first time.

### Deliberately Not Delivered

- XML inspection (the third Evidence Collector) — the larger of the three Sprint 6 roadmap candidates, deferred to a Sprint of its own rather than bundled with the two smaller items.
- Any change to `AssessmentService`'s public entry points, `AssessmentInput`, or the public error model — confirmed unchanged, not merely assumed.
- Any change to the Rule Engine — `modiq-rules` untouched this Sprint.
- `Display`/`Serialize` for Runtime identity/enum types — remains explicitly out of scope, per direct Chief Architect authorization, despite `modiq-cli` being the first text-only consumer where this would have the most visible benefit.
- A new Governance Register item for the `modiq-engine` → `AssessmentReport` re-export gap (below) — two data points, below this project's own usual three-point convergent-evidence bar.
- Resolution of GOV-001, GOV-002, GOV-003, GOV-008, or GOV-013.

---

## Major Architectural Accomplishments

- **A second real consumer of `AssessmentService` now exists, exercising the same additive entry point the Sandbox already validated.** `modiq-cli` reused `execute_from_assessment_input` exactly as designed, generating no new evidence toward GOV-008 — confirmed explicitly during scoping, and corrected in this Sprint's own planning record after an earlier draft overstated the connection.
- **A genuine, twice-observed architectural gap was named, not acted on.** `modiq-engine` does not re-export `AssessmentReport` — the type both of `AssessmentService`'s entry points return — so both real consumers (the Sandbox, and now `modiq-cli`) independently needed a direct `modiq-report` dependency just to name it. Two independent occurrences, evidenced by two different consumers built in two different Sprints under different circumstances — below this project's own usual three-point convergence bar (GOV-004's own resolution), but tracked explicitly rather than left to be rediscovered as a surprise.
- **"Capability before abstraction" held for CLI dispatch, the same discipline already validated for Collector dispatch and Rule dispatch.** Three commands, one direct match, no trait or registry — the same test applied a seventh time.
- **The project's governance and authority model was formalized for the first time, not merely renamed.** The Sources of Authority section distinguishes normative from descriptive authority explicitly, closing a gap this project's documentation had operated on implicitly since Sprint 1 without ever stating it directly.

---

## Major Implementation Accomplishments

- `modiq-cli` advanced from L1 (pure scaffold, unchanged since Sprint 0) to L2 — the first crate maturity change of this Sprint, and the first time this crate has shipped real, tested behavior.
- `modiq-report` reduced from four unused scaffold types plus `AssessmentReport` to `AssessmentReport` alone — the first time a crate on this platform has had its public surface *shrink* rather than grow across a Sprint.
- The root workspace test suite grew from 162 to 172 tests: `modiq-cli` 0 → 10 (its first tests ever); every other crate's count unchanged, including `modiq-report`'s 3 — confirming the scaffold deletion had zero test-coverage impact, exactly as the Sprint 5 Phase 4 investigation predicted it would.
- Manual, end-to-end verification of the real `modiq-cli` binary against real filesystem fixtures (success, missing-argument, and nonexistent-path cases) — not only asserted in unit tests.

---

## Governance Completed

| Item | Resolution | Notes |
|---|---|---|
| Reporting scaffold retirement | Resolved — the four scaffold types named at Sprint 5 Phase 4 are deleted, under explicit, separate authorization distinct from general Sprint scope authorization. | Not a Governance Register item in its own right; the authorization is recorded in `SPRINT6_IMPLEMENTATION_PLAN.md`'s own Authorization Record. |

**Still open:** GOV-001, GOV-002, GOV-003, GOV-008 (untouched this Sprint, now aging across four Sprints), and GOV-013 (untouched — `modiq-rules` was not touched this Sprint). Total Governance Register size: 13 items, 8 Resolved, 5 Open — unchanged in count from Engineering Release 0.5.

No ADR was created and no new Governance Register item was opened this Sprint. The `modiq-engine` re-export gap (above) was deliberately left below the threshold this project's own convergent-evidence discipline has applied consistently since GOV-004.

---

## Documentation Completed

- **Three canonical handoff documents reconciled to Chief Architect terminology** — `PROJECT_HANDOFF_v1.0.md`, `CHIEF_ARCHITECT_HANDOFF_v1.0.md` (renamed from `TECHNICAL_DIRECTOR_HANDOFF_v3.0.md`), `LEAD_ENGINEER_HANDOFF_v3.0.md` — with historical filenames (`TECHNICAL_DIRECTOR_HANDOFF_v2.0.md` through `v2.2.md`) retained under their original names as historical record, not rewritten.
- **`CHIEF_ARCHITECT_HANDOFF_v1.0.md` gained substantial new structure**: an explicit Mission, a Primary Responsibilities list, a permanent Engineering Workflow section, and a Sources of Authority section — the last written through several rounds of correction after an initial draft proposed a flat authority ranking that would have inverted this project's own "governance before implementation" discipline; the final version distinguishes normative from descriptive authority instead.
- **`LEAD_ENGINEER_HANDOFF_v3.0.md`** gained a Standard Sprint Execution section mirroring the same permanent workflow from the execution side.
- **`SPRINT6_IMPLEMENTATION_PLAN.md`** gained a formal Authorization Record section, establishing a convention intended to continue in future Sprint plans.
- **Living-document reconciliation at Closeout** — `PROJECT_STATUS.md` and `CHANGELOG.md`, both stale since Sprint 5 Closeout, corrected; `docs/README.md`'s release cross-reference corrected; `CrateRoadmap.md`'s dependency diagram updated to show `modiq-cli`'s two new direct edges. A pre-existing inaccuracy predating this Sprint — `PROJECT_HANDOFF_v1.0.md`'s claim that the Sandbox never depends on `modiq-report` directly — was also corrected, surfaced by this Sprint's own analogous `modiq-cli` dependency, not introduced by it.
- **`POST_SPRINT6_REPOSITORY_ASSESSMENT.md`** produced as a dedicated, read-only assessment following implementation, recommending XML inspection as the next engineering objective and naming the `modiq-engine` re-export gap explicitly.

---

## Testing Growth

| Milestone | Root Workspace Tests | Sandbox Tests |
|---|---|---|
| Engineering Release 0.5 | 162 | 6 |
| End of Sprint 6 implementation | 172 | 6 |
| Engineering Release 0.6 (this release) | **172** | **6** |

By crate, at this release: `modiq-runtime` 82, `modiq-collection` 43, `modiq-engine` 16 unit + 3 integration, `modiq-report` 3 (unchanged, despite three-quarters of the crate's public types being deleted), `modiq-rules` 15, `modiq-cli` 10 (new this Sprint). Zero tests ignored or flaky anywhere in the workspace, verified by direct execution both during implementation and again while preparing this release.

---

## Repository Maturity Assessment

| Area | Classification | Basis |
|---|---|---|
| Workspace organization | **Stable** | Nine crates, unchanged in count since Engineering Release 0.3. |
| Crate boundaries | **Stable** | `modiq-cli` satisfies `GOVERNANCE.md`'s CLI boundary rule ("must never contain business logic") by design, not incidentally — verified directly against the final implementation. |
| Dependency direction | **Stable, with one new edge pattern.** `modiq-cli` gained its first-ever dependencies (`modiq-runtime`, `modiq-engine`, `modiq-report`), mirroring a pattern the Sandbox already exhibited but that `PROJECT_HANDOFF_v1.0.md` had, until this Sprint, described inaccurately. |
| Documentation consistency | **Recurring, now four-for-four, and more severe than the prior three occurrences.** For the first time, staleness reached the documents this project's own governance baseline calls authoritative, not only the living status trackers. See Lessons Learned. |
| Governance and authority model | **Newly formalized.** The Sources of Authority distinction (normative vs. descriptive) did not exist in any prior release; it exists now, and was reached only after an initial, architecturally inconsistent draft was corrected before being adopted. |
| Engineering workflow | **Newly documented as a permanent artifact for the first time**, though the underlying discipline (proposal before implementation, explicit authorization, verification every phase) is unchanged from every prior Sprint. |

---

## Crate Maturity Review

| Crate | Maturity | Remaining Work | Architectural Confidence | Priority |
|---|---|---|---|---|
| `modiq-runtime` | L3, 82 tests, unchanged this Sprint | `Display`/`Serialize` for identity/enum types, flagged again this release — the sixth consecutive release to note it | High | Low — stable |
| `modiq-rules` | L3, 15 tests, unchanged this Sprint | A third Rule, if a concrete case justifies one; GOV-013's resolution depends on it | High | Low this Sprint — untouched by CLI/Reporting work |
| `modiq-collection` | L2, 43 tests, unchanged this Sprint | XML, Lua, manifest, dependency-analysis collectors, all still deferred | High | **High** — XML inspection is the sole remaining candidate from the original three-item Sprint 6 roadmap |
| `modiq-engine` | L3, 16 unit + 3 integration tests, unchanged in count | The `AssessmentReport` re-export gap (above); GOV-008 resolution when evidence justifies it | High | Medium — the re-export gap is small and low-risk, worth bundling into a future dependency-adjacent change |
| `modiq-report` | L3, 3 tests, unchanged in count despite shrinking | None identified — the crate now contains only its real, used content | High — the retirement recommendation was executed exactly as investigated, with zero surprises | Low — stable |
| `modiq-cli` | **L2 (new this Sprint), 10 tests** | Nothing blocking; future capability growth follows whatever `modiq-engine` itself grows | High — the Sandbox's own pattern reused without reinvention, zero rework required | Low — stable for what it does today |
| `modiq-knowledge` | L1, pure scaffold, unchanged since Sprint 0 | Everything; six Sprints running with zero forcing function | Low | Low — correctly deferred |
| `modiq-versioning` | L1, pure scaffold, unchanged since Sprint 0 | Everything | Low | Low — no forcing function yet |
| `modiq-common` | L1, empty stub files, unchanged since Sprint 0 | Undecided | Low — zero evidence of need after six Sprints | Low — do not invent a use for it |

---

## Technical Debt Review

### Intentional Technical Debt
*(deliberate, documented, correct to leave as-is for now)*

- The `modiq-engine` → `AssessmentReport` re-export gap — named explicitly, deliberately not acted on pending a third occurrence, consistent with this project's own convergent-evidence discipline.
- No `Display`/`Serialize` for Runtime identity/enum types — explicitly declined again this Sprint, despite `modiq-cli` being the first consumer where the benefit would be most visible.
- GOV-013 remains open by design, untouched this Sprint — still awaiting a third Rule.

### Future Capabilities
*(deferred by design, not yet started, no urgency implied)*

- XML inspection — the sole remaining candidate from the original three-item Sprint 6 roadmap, its precondition (Rule Engine maturity) satisfied since Sprint 5.
- A third Rule, and whatever GOV-013 resolution that Rule's existence might inform.
- Knowledge Domain integration, Version Profile integration, persistent assessment storage.

### Known Limitations
*(real, current, will not resolve without deliberate work)*

- Missing `Display`/`Serialize` for Runtime identity/enum types — now flagged in six consecutive release records without being scheduled.
- Documentation staleness between closeouts — now confirmed across four consecutive Sprints (3, 4, 5, 6), for the first time reaching the three documents this project's own governance baseline calls authoritative. See Lessons Learned.
- No formal `ENGINEERING_RELEASE_0.6.md` existed at the time of Sprint 6 Closeout itself — this document was produced later, closing that gap retroactively but accurately, per the historical record this document itself constitutes.
- Git tag hygiene remains unresolved: `v0.4.0`, `v0.5.0`, and now `v0.6.0` are all available untagged.

### Deferred Enhancements
*(nice-to-have, no urgency, no forcing function yet)*

- `CrateRoadmap.md`'s "Exit Criteria" section still has no Sprint 3, 4, 5, or 6 entry.

---

## Sprint 6 Retrospective

**What went well.** Two independent, previously-scoped, low-risk items (CLI wiring, Reporting retirement) were both delivered cleanly, with zero implementation surprises requiring escalation — the Reporting retirement in particular executed exactly as Sprint 5 Phase 4's own investigation predicted, with zero test-coverage impact.

**What surprised us.** The governance and terminology transition (Technical Director → Chief Architect) proved substantially larger than a rename — it surfaced a real architectural question (how should "Sources of Authority" actually work?) that required several rounds of correction before landing on a version consistent with this project's own established discipline, rather than inverting it.

**Architectural decisions validated by implementation.** The additive, two-entry-point `AssessmentService` design (GOV-008's own stopgap) held for a second real consumer with zero signature change required — direct evidence the pattern works, even though GOV-008 itself remains open pending stronger evidence to resolve it either way.

**Areas requiring future attention.** Documentation staleness between closeouts is now a four-Sprint pattern, having reached this project's own authoritative documents for the first time. GOV-008 has now aged across four Sprints untouched.

---

## Remaining Risks

- **Documentation staleness recurring a fourth consecutive time, now reaching the authoritative handoff documents themselves** — the most important process risk this release identifies.
- **GOV-008 aging across four Sprints untouched** — the stopgap continues to work, but remains explicitly a stopgap.
- **The `modiq-engine` re-export gap, real but below this project's own evidentiary bar for action** — worth tracking for a third occurrence, not yet worth resolving.
- **No formal Engineering Release record existed for this Sprint at the time of its own Closeout** — a new kind of gap this project's history had not previously exhibited, closed later by this document itself.

---

## Lessons Learned

- **A role and terminology rename is never "only" a rename.** Reconciling three documents to new vocabulary surfaced a genuine, previously-unstated architectural question about how authority actually works on this platform — worth remembering the next time a seemingly cosmetic documentation change is scoped as purely cosmetic.
- **Documentation staleness between closeouts has now escalated in kind, not just repeated in pattern.** Three prior occurrences affected only living status trackers; this one reached documents explicitly designated authoritative. The existing mitigation (reconciliation at Closeout) continues to catch it reliably after the fact — it does not yet prevent it.
- **A Sprint's own Engineering Release record should be produced at or near that Sprint's own Closeout, not deferred.** This release's own existence — written after Sprint 7 had already begun and concluded — is the direct, self-demonstrating evidence for why.

---

## Engineering Metrics

| Metric | Value |
|---|---|
| Workspace crates | 9 (unchanged since Engineering Release 0.3) |
| Governance items | 13 total — 8 Resolved, 5 Open — unchanged in count from Engineering Release 0.5 |
| Documentation Release | 2.1, Frozen — unchanged this Sprint |
| Engineering Release | 0.6 (this document, produced retroactively) — not tagged; `v0.6.0` available |
| Root workspace tests | 172 (up from 162 at Engineering Release 0.5) |
| Sandbox tests | 6 (unchanged) |
| Major milestones completed | Documentation Releases 1.0, 2.0, 2.1; Sprint 0–5; Engineering Releases v0.1.0-alpha, 0.2–0.5; Platform Validation Phase 1; Sprint 6 (CLI wiring, Reporting retirement, governance/terminology baseline); Sprint 6 Closeout |
| Implementation readiness | Fully ready for the next engineering objective at the close of this Sprint. Implementation committed as `397707f` on `feature/sprint6-cli`, merged into `feature/runtime-implementation` as `29657df`, Closeout committed as `af65bf0`. |

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
Sprint 5 (Phases 1–5, plus Closeout)
        ↓
Engineering Release 0.5
        ↓
Sprint 6 — CLI wiring; modiq-report scaffold retirement; Technical Director → Chief Architect
        ↓
Sprint 6 Closeout — repository reconciliation, first reaching the authoritative handoff documents
        ↓
Engineering Release 0.6
        ↓
Sprint 7 (not yet scoped as of this Sprint's own close)
```

---

## Recommendation

**Sprint 7 is ready to be scoped.** Both Sprint 6 deliverables landed cleanly, the governance and terminology baseline is reconciled, and both workspaces are fully green.

**What Sprint 7 should contain is not decided here.** One candidate remains from the original three-item Sprint 6 roadmap: XML inspection, the next Evidence Collector, its precondition (Rule Engine maturity) satisfied since Sprint 5 and unaffected by this Sprint. `POST_SPRINT6_REPOSITORY_ASSESSMENT.md` recommends it as the highest-value next engineering objective; this release record concurs, without treating that recommendation as a decision already made.

**One process item deserves explicit attention before Sprint 7 begins:** documentation staleness between closeouts has now recurred four times, escalating in severity each time it recurs. The mitigation applied consistently since Sprint 3 — reconciliation at Closeout — continues to work as designed; whether it needs to be supplemented with something that prevents staleness *between* closeouts, not just catches it afterward, is a standing question this release does not resolve, consistent with three prior releases declining to resolve it either.

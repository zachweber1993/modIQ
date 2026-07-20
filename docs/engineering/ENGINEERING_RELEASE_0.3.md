# Engineering Release 0.3

| Property | Value |
|----------|-------|
| **Release** | 0.3 |
| **Documentation Release** | 2.1 (Frozen — Evidence Collection subsystem boundary) |
| **Milestone** | Sprint 3 complete (Phases 1–5) |
| **Scope** | Sandbox real-pipeline integration; Finding/Recommendation minimum reference cardinality; the Evidence Collection subsystem, from architectural boundary through its first real, filesystem-backed collector |
| **Predecessor** | `docs/releases/ENGINEERING_RELEASE_0.2.md` (Sprint 2) |
| **Governing ADRs** | ADR-0008 (Evidence Collection Subsystem Boundary), ADR-0009 (AssessmentService Public API Evolution) |
| **Governing Proposals** | `PROPOSAL_EVIDENCE_COLLECTION_BOUNDARY.md`, `PROPOSAL_FILESYSTEM_COLLECTION.md` |
| **Governing Plan** | None — Sprint 3's five phases were each individually scoped and approved rather than following a single upfront implementation plan, unlike Sprint 2's `SPRINT2_IMPLEMENTATION_PLAN.md` |

---

## Executive Summary

Sprint 2 (Engineering Release 0.2) left the Runtime Domain with real content but no way to produce it from anything but test code or a hardcoded Sandbox value. Sprint 3 closed that gap. Across five phases, the platform gained a working Sandbox integration, closed two governance items left open since Sprint 2, designed and built an entirely new subsystem — Evidence Collection — and shipped that subsystem's first real capability: deterministic discovery of files and directories on the local filesystem. The workspace grew from eight crates to nine. The root test suite grew from 97 to 112 tests (115 including the Sandbox's own). Every crate boundary established since Documentation Release 1.0 held, including through the introduction of a subsystem that did not exist when that boundary was first drawn.

This release is a consolidation, not a new capability announcement. Nothing in it changes Runtime, Engine, Collection, Rules, or Sandbox behavior — it records what five phases of implementation already established, reconciles the documentation that had fallen behind them, and identifies what should happen next without deciding it here.

**One finding in this release is more important than any single accomplishment it records:** as of this writing, the entire body of Sprint 3 Phase 5 work, the governance resolution session that preceded it, and the Roadmap Review and Filesystem Collection proposal before that, remain **uncommitted** in the working tree. A "clean baseline" cannot be said to exist until this backlog is committed. This is addressed directly in Repository Health, below, and is this release's single most consequential recommendation.

---

## Scope of Sprint 3

### Delivered

- **Phase 1 — Sandbox Real-Pipeline Integration.** `apps/sandbox`'s `create_assessment` command began executing the genuine `AssessmentService::execute` pipeline instead of returning an empty Assessment DTO.
- **Phase 2 — GOV-005/GOV-006 Resolution.** `Finding` and `Recommendation` gained enforced minimum reference cardinality (INV-013, INV-014), closing two governance items open since Sprint 2. Referential integrity was explicitly, deliberately left for a future item.
- **Phase 3 — Evidence Collection Architecture.** `PROPOSAL_EVIDENCE_COLLECTION_BOUNDARY.md` was designed and approved, producing ADR-0008, ADR-0009, the new `EvidenceCollection.md` specification, corresponding `Architecture.md`/`DataModel.md`/`RuleEngine.md` amendments, and Governance Register items GOV-007 through GOV-010. Documentation Release 2.1 was frozen.
- **Phase 4 — Evidence Collection Boundary Proof.** The `modiq-collection` crate was created — the platform's ninth — with a minimal, synthetic collector proving the architectural boundary without implementing any real inspection. GOV-007 resolved.
- **Governance Resolution — GOV-009 / GOV-010.** `PROPOSAL_FILESYSTEM_COLLECTION.md` was designed and approved, giving GOV-009 (Assessment Input Ownership) and GOV-010 (Collection Error Model) enough concrete context to resolve for the filesystem case.
- **Phase 5 — First Real Evidence Collection.** `EvidenceCollector` was rewritten to inspect the real filesystem: deterministic traversal, the approved four-outcome error model, Collection Atomicity, and the Phase 5 Symbolic Link Policy. `InputDescriptor`/`InputDescriptorError` were renamed to `AssessmentInput`/`AssessmentInputError` throughout. The Sandbox was updated to exercise the real collector against a fixed, checked-in fixture directory.

### Deliberately Not Delivered

- ZIP, XML, Lua, manifest, or dependency-analysis collectors — the next capabilities in sequence, none started.
- Rule abstraction or a second Rule — explicitly evaluated and deferred this Sprint ("a capability should justify an abstraction, not the reverse").
- Knowledge Domain integration, Version Profile integration, CLI wiring, persistence — all remain scaffolded only.
- Resolution of GOV-001, GOV-002, GOV-003, GOV-004, or GOV-008.
- Reconciliation of the unused `modiq-engine` EngineAPI stub services or `modiq-rules` stub submodules, identified as increasingly ripe for a decision but not decided.
- Any Sandbox input UI (file picker, drag-and-drop) — a fixed fixture path stood in throughout.

---

## Major Architectural Accomplishments

- **A new subsystem was introduced without disturbing any existing boundary.** Evidence Collection did not exist in any form when Documentation Release 1.0 froze the platform's architecture. Its introduction — a new crate, a new dependency edge, a new Engine-orchestrated step in the pipeline — required zero changes to Runtime, Rule Engine, or Reporting's own responsibilities, and the Rule Engine's producer/consumer boundary (first fought for in Phase 1's own review) was never threatened across four subsequent phases of adjacent work.
- **The "additive API growth" pattern was validated twice.** `AssessmentService::execute_from_descriptor` (Phase 4) and its successor `execute_from_assessment_input` (Phase 5) both shipped alongside the original `execute`, never replacing it, specifically to avoid resolving GOV-008 as a side effect of unrelated work. `execute` remains byte-for-byte unchanged since Sprint 1.
- **The proposal-first workflow produced zero post-implementation architectural rework.** Three separate Architecture Review proposals were written and approved before their corresponding implementation began this Sprint (Evidence Collection Boundary, and implicitly the GOV-005/006 cardinality decision; Filesystem Collection). In each case, implementation matched the approved design with no deviation requiring a return to architecture.
- **The four-outcome Collection Error Model, designed entirely in the abstract, needed no revision once real I/O code had to implement it.** Invalid Input, Inaccessible Input, Unsupported Input, and Empty Collection map directly onto `AssessmentInput::new`'s single validation check and `EvidenceCollector::collect`'s real filesystem logic, with no fifth case discovered during implementation.

---

## Major Implementation Accomplishments

- Real, deterministic filesystem traversal: files and directories discovered, sorted for stable ordering, symbolic links neither followed nor recorded.
- Collection Atomicity required no dedicated mechanism — it fell directly out of ordinary `Result`/`?` propagation once the architecture specified it, with no explicit "rollback" logic anywhere.
- `modiq-collection` grew from a single, minimal type family (Phase 4) to a functioning, tested collector: 12 tests, covering the success path, all reachable failure categories, determinism, and two symbolic-link scenarios.
- The Sandbox now discovers genuinely real Evidence — three structural facts about a checked-in fixture directory — rather than one synthetic, hardcoded item, using no new UI and no new direct dependency on `modiq-collection` (it reaches the Sandbox only transitively, through `modiq-engine`, exactly as ADR-0008 specified).

---

## Governance Completed

| Item | Resolution | Phase |
|---|---|---|
| GOV-005 | Finding must reference ≥1 Evidence item (INV-013). Cardinality only; referential integrity deferred. | Phase 2 |
| GOV-006 | Recommendation must reference ≥1 Finding item (INV-014, new invariant, not a reworded INV-005). Cardinality only. | Phase 2 |
| GOV-007 | Evidence Collection implementation approved and delivered at minimal, boundary-proving scope. | Phase 4 |
| GOV-009 | Assessment Input Ownership: `EvidenceCollection.md` owns the definition; files and directories both valid; non-existent paths are Inaccessible, not Invalid; symbolic links not traversed. | Governance session preceding Phase 5 |
| GOV-010 | Collection Error Model: four outcomes (Invalid/Inaccessible/Unsupported/Empty), Collection Atomicity as an intentional Phase 5 policy. | Governance session preceding Phase 5 |

**Still open:** GOV-001 (Report generation timing), GOV-002 (Runtime invariant reconciliation), GOV-003 (role of `modiq-common`), GOV-004 (Engine service granularity), GOV-008 (AssessmentService API evolution). None were touched this Sprint; see Technical Debt Review for their current pressure.

No new Governance Register item was opened this Sprint beyond the four already created in Phase 3 (GOV-007–GOV-010) — every governance question raised was resolved using already-open items, not new ones.

---

## Documentation Completed

- **Documentation Release 2.1** — Frozen. New: `EvidenceCollection.md`. Amended (explicitly, non-silently): `Architecture.md` (Evidence Collection named a Core Platform Component; Assessment Lifecycle diagram ordering corrected), `DataModel.md`, `RuleEngine.md` (cross-references only).
- **ADR-0008** (Evidence Collection Subsystem Boundary) and **ADR-0009** (AssessmentService Public API Evolution) — both Accepted, both left unmodified once implementation landed, per this project's convention that an ADR's Status reflects the decision's own acceptance, not later implementation progress.
- **Two Architecture Review proposals** — `PROPOSAL_EVIDENCE_COLLECTION_BOUNDARY.md`, `PROPOSAL_FILESYSTEM_COLLECTION.md` — both retained as historical records of the design work that preceded implementation, per this project's proposal convention.
- **`ROADMAP_REVIEW_2026.md`** — a full reassessment of the project against its actual, current state, recommending Filesystem Collection as the next milestone (which this Sprint then delivered) and identifying several findings this release document incorporates directly (the EngineAPI/stub-module observation, the living-document staleness pattern).
- **`docs/README.md`** — Reading Order corrected to include `EvidenceCollection.md`.

---

## Testing Growth

| Milestone | Root Workspace Tests | Sandbox Tests |
|---|---|---|
| Engineering Release v0.1.0-alpha | 55 | — (Sandbox did not yet exist) |
| Engineering Release 0.2 | 97 | — |
| End of Sprint 3 Phase 4 | 106 | 3 |
| End of Sprint 3 Phase 5 (this release) | **112** | **3** |

By crate, at this release: `modiq-runtime` 82, `modiq-collection` 12, `modiq-engine` 9 unit + 3 integration, `modiq-report` 3, `modiq-rules` 3. Zero tests are ignored or flaky anywhere in the workspace.

**A genuinely new testing discipline entered the suite this Sprint:** Phase 5 is the first work in the platform's history to require OS-level test fixtures — temporary directories, symbolic links, restricted permissions — because it is the first code to touch real, external, potentially-adversarial state. No prior phase needed this.

---

## Repository Maturity Assessment

| Area | Classification | Basis |
|---|---|---|
| Workspace organization | **Stable** | Nine crates, consistent structure; one minor inconsistency (`modiq-rules`'s `Cargo.toml` still declares `version`/`edition` literally rather than via `.workspace = true`, unlike every other crate) not worth a dedicated fix on its own. |
| Crate boundaries | **Stable** | Zero violations across five phases, including the introduction of an entirely new subsystem. |
| Dependency direction | **Stable** | Strictly downward throughout; `modiq-collection`'s new edge (`modiq-engine → modiq-collection → modiq-runtime`) verified to match ADR-0008 exactly, including confirming the Sandbox reaches it only transitively. |
| Documentation consistency | **Needs Future Work** | A recurring pattern, not an incident: `PROJECT_STATUS.md`, `CHANGELOG.md`, and `docs/README.md`'s status narrative have gone stale after every release checkpoint so far, flagged each time, structurally unaddressed until a session is explicitly scoped to it (this one, in part — see Living Document Reconciliation). |
| Testing maturity | **Needs Monitoring** | Unit-level coverage is excellent and disciplined. Two structural gaps remain: no verification of the Sandbox's actual UI has ever been performed by any session, and adversarial/malformed-input testing exists only for what Phase 5 specifically introduced (symlinks, permission denial) — this will need deliberate attention once ZIP/XML collectors introduce a substantially larger adversarial-input surface. |
| Engineering workflow | **Needs Monitoring** | The proposal → governance → documentation → implementation → verification cycle itself is sound and was exercised successfully three times this Sprint. The concern is procedural, not architectural: a substantial body of already-approved work (detailed below) has gone uncommitted across three consecutive sessions. |

---

## Crate Maturity Review

| Crate | Maturity | Remaining Work | Architectural Confidence | Priority |
|---|---|---|---|---|
| `modiq-runtime` | L3, 82 tests | `Display`/`Serialize` for identity/enum types (flagged repeatedly, still open); referential integrity for Finding/Recommendation references | High — most heavily exercised crate in the platform, zero surprises across five phases of neighboring work | Low — stable, revisit only when a concrete forcing function arrives |
| `modiq-engine` | L3 for `AssessmentService` (12 tests); four EngineAPI stub services still unused | Resolve GOV-004 (three real subsystems now composed by direct instantiation, never through the stub services) | High for what exists; the unused stub services are a documented, not surprising, gap | Medium — ripe for a decision, not urgent |
| `modiq-collection` | L2, 12 tests | Every concrete collector beyond filesystem discovery; GOV-008/009/010's remaining edges as real I/O sources multiply | High — the boundary and the first real capability both proved out cleanly | **High** — this is where the platform's actual value proposition continues to be built |
| `modiq-rules` | L3 for exactly one Rule, 3 tests; four stub submodules unused, same pattern as `modiq-engine`'s | A second Rule, if and when a concrete case justifies one (explicitly not yet) | High for the one Rule that exists; unresolved question for the stub submodules | Low for new Rule work (deliberately deferred); Medium for the stub-reconciliation question |
| `modiq-report` | L3, 3 tests, unchanged since Sprint 2 | GOV-001 (report generation timing) bears directly on this crate; "generating explanations" (`Architecture.md`'s own stated Reporting responsibility) is not yet implemented at all | High for what exists; a real, currently invisible product-value gap | Medium — important, but there is not yet enough Rule/Collector variety to meaningfully explain |
| `modiq-knowledge` | L1, pure scaffold — seven empty marker structs, zero logic, referenced by nothing | Everything; the entire `KnowledgeModel.md` shape is unvalidated against any real usage | Low — the least-tested assumption in the platform, by construction | Low — correctly sequenced after Rule abstraction, itself deferred |
| `modiq-versioning` | L1, pure scaffold — four empty marker structs | Everything; nothing in the platform yet varies by game version | Low — untested against any real need | Low — no forcing function exists yet |
| `modiq-cli` | L1, pure scaffold — command structs exist but are unwired | Wiring `AssessCommand` to `modiq-engine`, mirroring the Sandbox's now-twice-proven thin-client pattern | High — the pattern to build it is fully de-risked by the Sandbox's own history | Medium — genuinely low-risk, independent of any Collection work, a good near-term candidate |
| `modiq-common` | L1, empty stub files with no content at all | Undecided — see below | Low — zero evidence, after nine phases, that this crate is actually needed | Low — do not invent a use for it |

**Four crates — `modiq-knowledge`, `modiq-versioning`, `modiq-cli`, `modiq-common` — have received zero implementation since Sprint 0.** All four remain deliberately, correctly deferred by the same "second concrete case" discipline this Sprint applied elsewhere (the Rule trait question, the Collector dispatch question). None of the four should be removed from the roadmap on that basis alone. `modiq-common` is the one genuine exception worth naming plainly: after two full Engineering Releases and five Sprint 3 phases, nothing in the platform has ever needed it, and GOV-003 has had no forcing function arrive in nine phases of engineering. This does not mean it should be deleted — GOV-003's own guidance (resolve only when two crates naturally need to share a type) still holds — but it is now the crate with the weakest evidence, of the four, that it belongs in the roadmap in its current form.

---

## Technical Debt Review

### Intentional Technical Debt
*(deliberate, documented, correct to leave as-is for now)*

- `EvidenceCollector::collect` has no error path that cannot be reached — a direct, correct consequence of there being no real failure mode until Phase 5, and Phase 5 closed the one real gap (empty-input validation already existed; Inaccessible/Unsupported now exist for genuine I/O failure).
- GOV-008 remains unresolved; the additive-entry-point pattern is a deliberate, working stopgap, not an oversight.
- No Rule trait, no second Rule — explicitly evaluated and declined this Sprint, on the same "capability before abstraction" principle already governing Collector dispatch.
- **Newly ripe, not yet acted on:** GOV-005/GOV-006's referential-integrity follow-up was deliberately left unassigned pending "a second real Evidence producer." Phase 5's filesystem collector is arguably exactly that producer — worth revisiting, not because anything is broken, but because the condition that was deferring it may no longer hold.

### Future Capabilities
*(deferred by design, not yet started, no urgency implied)*

- ZIP traversal, XML inspection, Lua inspection, manifest analysis, dependency analysis (all named and sequenced in `PROPOSAL_FILESYSTEM_COLLECTION.md`'s Future Evolution section).
- Rule abstraction and multi-rule dispatch.
- Knowledge Domain integration, Version Profile integration.
- CLI wiring, persistent assessment storage.

### Known Limitations
*(real, current, will not resolve without deliberate work)*

- Missing `Display`/`Serialize` for Runtime identity/enum types — flagged at Sandbox Phase 2, Sprint 3 Phase 1, and again here: **this is now the fourth consecutive release record to note it without scheduling it.**
- No session has ever visually verified the Sandbox's actual UI — every check across every phase has been Rust-only.
- Git tag hygiene: `v0.2.0` and `v0.3.0` already exist in source control and predate `v0.1.0-alpha` chronologically (a pre-existing issue, flagged previously) — meaning **this Engineering Release 0.3 cannot be tagged `v0.3.0`** without colliding with unrelated prior history. It is not tagged; see Engineering Metrics.
- Adversarial-input test coverage exists only for what Phase 5 specifically needed; will need deliberate expansion once ZIP/XML parsing introduces a meaningfully larger attack surface.

### Deferred Enhancements
*(nice-to-have, no urgency, no forcing function yet)*

- Reconciling `modiq-engine`'s unused EngineAPI stub services and `modiq-rules`'s unused stub submodules (GOV-004-adjacent).
- Formal Documentation Release cycle (Draft → Freeze → Tag) for future amendments, versus the lighter-weight, explicitly-recorded direct amendments used for GOV-009/010 and Phase 3.

---

## Sprint 3 Retrospective

**What went well.** The proposal-first workflow, exercised three separate times this Sprint, produced implementations that needed no return trip to architecture. Crate boundaries — including the Rule Engine/Evidence Collection producer-consumer line first drawn in Phase 1 — held under real pressure across five phases without a single exception or "just this once." Collection Atomicity, specified in the abstract, was satisfied by ordinary Rust control flow with zero purpose-built mechanism.

**What surprised us.** How directly the architecture's abstract policies translated into working code with no gap: the four-outcome error model needed no fifth case; atomicity needed no explicit rollback. Less pleasantly: discovering that `modiq-engine`'s and `modiq-rules`'s stub scaffolding has never been used, even once, across three separate subsystems now composed by direct instantiation instead — a finding the Roadmap Review had predicted before this Sprint's own implementation confirmed it on the next real data point. And documentation staleness recurred at essentially every checkpoint, identically, despite being caught and named each time.

**Architectural decisions validated by implementation.** ADR-0008's dependency direction (`modiq-collection → modiq-runtime` only) held exactly as specified, confirmed by inspecting the Sandbox's own separate build rather than assumed. The GOV-009/010 proposal's error categories mapped onto real filesystem I/O with no revision needed. ADR-0007's Runtime entity pattern, though not directly applicable to `AssessmentInput` (a value type, not a Runtime entity), still correctly informed the decision not to give it identity-based equality.

**Areas where the architecture proved stronger than expected.** The "a capability should justify an abstraction, not the reverse" principle held up twice under real schedule pressure to just build the abstraction — for the Rule trait question and for Collector dispatch — and both times the smaller, concrete implementation shipped instead, with no loss of correctness.

**Areas requiring future attention.** GOV-004 now has three real data points and no resolution. GOV-008 has two parallel entry points and is approaching the threshold this project's own prior guidance named ("a third would be the signal to stop routing around it"). Living-document staleness is structural, not incidental, and will recur again after this release unless something changes about when these documents get touched. Four crates remain completely unvalidated since Sprint 0.

---

## Remaining Risks

- **Uncommitted work backlog** (see Repository Health / Engineering Metrics) — the most immediate risk, since it means no git history yet reflects any of this Sprint's later work.
- **GOV-004 and GOV-008 aging without resolution**, now with real implementation evidence available that didn't exist when they were first raised.
- **Documentation staleness recurring** unless a session is deliberately, structurally scoped to prevent it rather than to catch it after the fact.
- **The next real collector (most likely ZIP traversal) introduces the platform's first genuinely adversarial input surface** — malicious or malformed archives — which nothing built so far has had to defend against.

---

## Lessons Learned

- Real I/O work needs test infrastructure pure-logic crates never required (temp directories, symbolic links, permission manipulation) — this capability now exists in the test suite and should be reused, not reinvented, by future I/O-touching work.
- Splitting "design the architecture" and "implement against it" into separate, separately-approved sessions is now a three-for-three success rate this Sprint at preventing post-implementation rework — worth continuing deliberately for any future work that crosses a crate boundary.
- Flagging documentation staleness without a dedicated reconciliation task does not fix it. It has now been flagged and left unfixed at every single checkpoint since Documentation Release 2.0. A recurring "living document reconciliation" pass, not just recurring flags, appears to be what this pattern actually requires.

---

## Engineering Metrics

| Metric | Value |
|---|---|
| Workspace crates | 9 (`modiq-cli`, `modiq-collection`, `modiq-common`, `modiq-engine`, `modiq-knowledge`, `modiq-report`, `modiq-rules`, `modiq-runtime`, `modiq-versioning`) |
| Governance items | 10 total — 5 Resolved (GOV-005, 006, 007, 009, 010), 5 Open (GOV-001, 002, 003, 004, 008) |
| Documentation Release | 2.1, Frozen |
| Engineering Release | 0.3 (this document) — **not tagged**; `v0.3.0` already exists in source control from unrelated prior history (known git tag hygiene debt) |
| Root workspace tests | 112 (up from 97 at Engineering Release 0.2) |
| Sandbox tests | 3 |
| Major milestones completed | Documentation Releases 1.0, 2.0, 2.1; Sprint 0; Sprint 1 (`v0.1.0-alpha`); Sprint 2 (Engineering Release 0.2); Sandbox Phases 1–2; Sprint 3 Phases 1–5 |
| Implementation readiness | Architecturally and functionally ready for the next capability. **Not git-clean** — see Repository Health. |

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
Sprint 3 Phase 1 — Sandbox real-pipeline integration
        ↓
Sprint 3 Phase 2 — GOV-005 / GOV-006 resolved
        ↓
Sprint 3 Phase 3 — Evidence Collection architecture; Documentation Release 2.1
        ↓
Sprint 3 Phase 4 — modiq-collection created; GOV-007 resolved
        ↓
Governance resolution — GOV-009 / GOV-010 resolved
        ↓
Sprint 3 Phase 5 — first real Evidence Collection
        ↓
Engineering Release 0.3
        ↓
Next capability (pending selection — see Recommendation)
```

---

## Recommendation

The next logical capability is **a second real collector — most likely ZIP traversal.** Filesystem discovery (this release) proved the exact pattern a second collector needs to follow, and both `PROPOSAL_EVIDENCE_COLLECTION_BOUNDARY.md` and `PROPOSAL_FILESYSTEM_COLLECTION.md` already name it as the immediate next step in Evidence Collection's own sequence — nothing about this recommendation is new; it is the continuation five phases of prior work were already pointed at.

**CLI wiring** remains a valid, independent, low-risk parallel track, as `ROADMAP_REVIEW_2026.md` already recommended — it shares no risk surface with collector work and does not need to wait for it.

This is a direction, not a plan: no detailed scope, API shape, or implementation sequencing is proposed here, consistent with this release's own charter.

**Before either begins, the uncommitted work identified in this release should be committed.** An Engineering Release records a baseline; the repository does not yet have one to record, in the git-history sense, until that backlog lands.

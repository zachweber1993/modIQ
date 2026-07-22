# Engineering Release 1.1

| Property | Value |
|---|---|
| **Release** | 1.1 |
| **Documentation Release** | 2.1 (Frozen — unchanged this Sprint; no specification amendment) |
| **Milestone** | Sprint 11 complete (Runtime Evidence Processing: Architectural Resolution, Implementation, Reconciliation, Repository Closeout) |
| **Scope** | The platform's first real Runtime Log Interpretation capability — activating `EvidenceCategory::RuntimeLogs` after eleven Sprints dormant, and the platform's first real use of `FindingSeverity::Error` |
| **Predecessor** | `docs/engineering/ENGINEERING_RELEASE_1.0.md` (Sprint 10) |
| **Governing ADRs** | None new — this Sprint applies ADR-0007's existing Opaque Runtime References pattern and GOV-012's existing Rule dispatch model rather than establishing a new durable principle |
| **Governing Plan** | `docs/implementation/SPRINT11.md`, `docs/engineering/RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md` (v1.2.0) |

---

## 1. Executive Summary

Engineering Release 1.0 closed Sprint 10 having deliberately deferred Runtime Log Interpretation's own implementation, on the explicit basis that no architectural decision could responsibly be made until real Farming Simulator runtime logs existed in the repository to ground it. That precondition was satisfied by Sprint 10's three real, captured, normalized fixtures. Sprint 11 was authorized to take the next, and only the next, step this project's own workflow allows: define the Runtime Evidence Processing Architecture — where a runtime observation enters the pipeline, what shape its Evidence takes, how it is interpreted, and what `FindingSeverity` applies — grounded in that corpus, without yet writing implementation.

That architecture (`RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md`, approved in principle) was then implemented across four incremental, independently reviewed milestones: a standalone Collector, its integration into `AssessmentService`, a standalone Rule, and its integration into `RuleEngine::evaluate`'s dispatch order. A subsequent engineering verification pass — conducted adversarially, attempting to disprove consistency rather than confirm it — found a genuine internal contradiction between the architecture document's own Architectural Invariants section and the Collector actually implemented against an earlier section of the same document. Per this project's standing discipline, implementation was halted and the contradiction was reported rather than silently resolved either in code or in the reader's favor. Chief Architect review determined the implementation was correct and the invariant's wording was the inconsistency; the architecture document was revised (v1.1.0 → v1.2.0) to describe the Collector-recognizes-then-Evidence-exists model the implementation already faithfully realized, with no Rust source changed in that reconciliation.

`RuntimeLogCollector` (`modiq-collection`) and `RuntimeLoadFailureRule` (`modiq-rules`) are both real, tested, and wired end to end. The root workspace test suite grew from 210 to 238 tests; Sandbox unchanged at 7/7. No crate, no public entry point, no `EvidenceCategory` variant, and no Governance Register item was added — the capability's entire footprint is two new files and four call sites, exactly the additive shape this project's engineering principles require.

---

## 2. Sprint Objective

Per `SPRINT11.md`: activate `EvidenceCategory::RuntimeLogs` by implementing the Collector and Rule the approved Runtime Evidence Processing Architecture defines, recognizing exactly one class of signal — a mod rejected at modDesc validation, evidenced by `single-incompatible-mod` — as real, traceable Evidence and a corresponding Finding, without expanding recognition beyond that one signature, without touching the Knowledge Domain, and without redesigning any existing architectural boundary.

---

## 3. Architectural Work Completed

- **`RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md` v1.0.0** — the Sprint 11 Architectural Resolution, structured around the four questions `SPRINT11.md`'s Scope named: where a runtime observation enters the pipeline (a second content-Collector, composed inline alongside `XmlCollector`, per the Collector Composition Architecture's own extraction threshold — not yet crossed); `EvidenceCategory::RuntimeLogs`'s content shape (a factual capture of the recognized template, mirroring `XmlCollector`'s Content Extraction discipline); how the evidence is interpreted (a new Rule, fourth in GOV-012's fixed dispatch order, `FindingSeverity::Error` — the platform's first real use of that variant); and fixture-corpus sufficiency (the existing three fixtures are sufficient for this Sprint's own narrow scope). Included the Runtime Interpretation Decision Matrix (four rows, each citing real fixture evidence) as a first-class deliverable.
- **`RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md` v1.1.0** — a Chief Architect-requested refinement pass: an explicit Architectural Invariants section (five invariants governing Collector/Rule boundaries, determinism, and unknown-signature handling) and an explicit governance-relationship clarification distinguishing this Sprint's architectural interpretation from GOV-013's own separate, unresolved severity-vocabulary question.
- **`RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md` v1.2.0** — the post-implementation architectural reconciliation (Section 5 below).

---

## 4. Implementation Summary

Four incremental milestones, each independently reviewed before the next began:

1. **`RuntimeLogCollector`** (`crates/modiq-collection/src/collection/runtime_log_collector.rs`) — locates `log.txt` at the Assessment Input's root (filesystem or archive, mirroring `XmlCollector`'s identical symmetry), recognizes the exact documented failure template (generalized over the mod name, not the message text), and produces `RuntimeLogs` Evidence only for a recognized match. A missing or unrecognized log is Legitimate Absence — `Ok(vec![])` — never a recorded fact, a deliberate, reasoned divergence from `XmlCollector`'s own missing-manifest-as-Evidence precedent. 15 tests.
2. **Wiring into `AssessmentService::execute_from_assessment_input`** — one additional `evidence.extend(RuntimeLogCollector.collect(&assessment_input)?)` call, appended after the existing `XmlCollector` call. No change to either public entry point's signature. 3 new tests plus 1 added after Milestone 4 to confirm the complete pipeline.
3. **`RuntimeLoadFailureRule`** (`crates/modiq-rules/src/rules/runtime_load_failure_rule.rs`) — filters `RuntimeLogs` Evidence, independently re-matching the same recognized template (defense in depth against the Collector's own boundary), and assigns `FindingSeverity::Error` with an inline-authored Recommendation (`repair_recipe_reference: None`). Not yet reachable from `RuleEngine::evaluate` at this milestone, mirroring `StructuralDuplicationRule`'s own Sprint 5 precedent. 8 tests.
4. **Dispatch integration into `RuleEngine::evaluate`** — one additional `if let Some(outcome) = RuntimeLoadFailureRule.evaluate(evidence)` block, appended fourth, after `VersionCompatibilityRule`'s. No reordering of the existing three Rules; no suppression model introduced. 4 new tests, including one exercising all four Rules simultaneously.

---

## 5. Architectural Reconciliation (v1.1.0 → v1.2.0)

A dedicated, adversarial verification pass (Technical Director / Chief Architect / Lead Engineer, conducted after Milestone 4 and before any repository closeout work began) found that v1.1.0's fourth Architectural Invariant — *"an unsupported or unrecognized runtime observation remains Evidence... until a deterministic architectural rule explicitly supports interpreting it"* — presupposed an Evidence-first, Rule-decides model (the shape `XmlCollector`/`VersionCompatibilityRule` actually use). `RuntimeLogCollector`, built against the document's own Section 2.2, instead performs recognition *before* Evidence is created: an unrecognized log line is never collected as Evidence at all. This is a genuine documentation inconsistency within the same document, not a code defect — confirmed by re-reading the implementation directly against both sections, not assumed.

Per this project's standing discipline (`CLAUDE.md`; `SPRINT11.md`'s own "if implementation uncovers a contradiction, STOP" instruction), implementation was halted and the contradiction reported rather than resolved unilaterally. Chief Architect review determined the implementation — a Collector-level deterministic recognition contract, factual until interpreted, exact rather than heuristic — was correct, and directed the architecture document's wording to be reconciled to it. `RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md` was revised to v1.2.0: the fourth invariant now states that Collectors define a deterministic recognition contract and only matching observations become Evidence at all; Section 2.2 states this explicitly as that contract; Section 4 (Matrix row 4) states the non-match is stopped at the Collector, with the Rule's own independent matching reframed as a secondary, defense-in-depth safeguard; a new Section 2.4 states that the contract may only be extended by future, separately approved, fixture-grounded architectural work — never heuristics or generic ingestion; a new Section 1.4 resolves, and closes, a secondary observation raised during the same verification pass (whether archive-location symmetry for the log was fixture-justified) by grounding it instead in the already-approved Assessment Input model (GOV-009/GOV-011), not fixture evidence specific to this capability.

**No Rust source, test, fixture, ADR, or Governance Register item was touched in this reconciliation.** The implementation was never wrong; the v1.1.0 invariant's wording was inconsistent with the rest of its own document, and the reconciliation corrects the document, not the code.

---

## 6. Repository Impact

| Area | Change |
|---|---|
| Crates | None added, none removed — 9 workspace crates, unchanged since Engineering Release 0.3 |
| Public entry points | `AssessmentService::execute` and `execute_from_assessment_input` — both unchanged |
| `RuleEngine::evaluate` | Signature unchanged; dispatch body gained one additional `if let` block |
| `EvidenceCategory` | Unchanged — `RuntimeLogs` activated, not added; the enum's variant set is untouched |
| External dependencies | None added — plain string matching (`strip_prefix`), the same mechanism `VersionCompatibilityRule` already uses |
| Internal dependency edges | None added — the new Collector depends on `modiq-runtime` exactly as every existing Collector already does; the new Rule depends on `modiq-runtime` exactly as every existing Rule already does. Neither touches `modiq-versioning` or `modiq-knowledge` |
| Governance Register | Unchanged — 13 items, 8 Resolved, 5 Open |
| ADRs | None new |
| Fixtures | None added — the existing three-fixture corpus (Sprint 10) is unchanged and was sufficient |

---

## 7. Runtime Capability Implemented

**After Sprint 11, modIQ can now** recognize, as real and traceable Evidence, that a bundled runtime log shows a mod was rejected at modDesc validation and never reached a loadable state — and produce an `Error`-severity Finding stating so, with a traceable Recommendation, through the exact same `AssessmentService::execute_from_assessment_input` entry point every existing consumer (Sandbox, CLI) already calls, requiring no change to either.

This is the platform's first real, evidence-based answer to `Vision.md`'s own named founding question, "why does it fail to load?" — the same transformation this platform previously delivered for compatibility declarations (Sprint 8) and structural content (Sprints 3, 4, 7), now delivered for an actual runtime event rather than a static package fact.

---

## 8. Architectural Verification

Verified directly against `RUNTIME_EVIDENCE_PROCESSING_ARCHITECTURE.md` v1.2.0, GOV-012, GOV-013, ADR-0007, and the Collector Composition Architecture, not assumed from the implementation session's own account:

- ✅ Collector boundaries intact — `RuntimeLogCollector` consumes only an `AssessmentInput`, produces only `Evidence`, never invokes another Collector or mutates `Assessment`.
- ✅ Rule boundaries intact — `RuntimeLoadFailureRule` consumes only `&[Evidence]`, produces only a `RuleOutcome`, never invokes another Rule or the Collector layer.
- ✅ Knowledge boundaries intact — no new dependency edge into `modiq-knowledge`; the Recommendation is inline-authored, mirroring `VersionCompatibilityRule`'s own pre-Sprint-9 shape.
- ✅ Determinism preserved — exact string matching only, no heuristic, scoring, or probabilistic step at any layer; dedicated determinism tests at the Collector, Rule, and dispatch levels.
- ✅ Dispatch order matches GOV-012 exactly — `EvidencePresenceRule`, `StructuralDuplicationRule`, `VersionCompatibilityRule`, `RuntimeLoadFailureRule`, appended, never reordered.
- ✅ Legitimate Absence behaves exactly as specified — a missing or unrecognized log resolves to `Ok(vec![])`, never a recorded fact, never a Collector failure.
- ✅ Runtime evidence remains factual until interpreted — the Collector's own Evidence description restates the log's own content; it never states that a mod "failed" or "is broken."
- ✅ Unknown runtime signatures never produce inferred Findings, at both layers — stopped at the Collector (never collected) and, independently, at the Rule (never interpreted even if hypothetically present).

**One inconsistency was found and resolved** (Section 5, above) — the only checklist item that did not pass on first verification. No other inconsistency was found across this checklist.

---

## 9. Engineering Verification

Run this session, on the completed repository, immediately before this release document:

```
cargo fmt --check      → clean
cargo check --workspace → clean, zero warnings
cargo test --workspace  → 238/238 passing
```

Sandbox (`apps/sandbox/src-tauri`) reverified separately: 7/7 passing, unchanged — confirming neither `AssessmentService` entry point Sandbox calls changed in any way that affects it.

---

## 10. Test Summary

| Crate | Engineering Release 1.0 | This Release | Change |
|---|---|---|---|
| `modiq-cli` | 10 | 10 | — |
| `modiq-collection` | 57 | 70 | +13 (`RuntimeLogCollector`) |
| `modiq-common` | 0 | 0 | — |
| `modiq-engine` | 19 unit + 3 integration | 23 unit + 3 integration | +4 (Collector/Rule wiring, end-to-end confirmation) |
| `modiq-knowledge` | 5 | 5 | — |
| `modiq-report` | 3 | 3 | — |
| `modiq-rules` | 25 | 36 | +11 (`RuntimeLoadFailureRule` + dispatch integration) |
| `modiq-runtime` | 84 | 84 | — |
| `modiq-versioning` | 4 | 4 | — |
| **Root workspace total** | **210** | **238** | **+28** |
| **Sandbox** | 7 | 7 | — |

Zero tests ignored, skipped, or flaky anywhere in the workspace, confirmed by direct execution.

---

## 11. Deferred Work

Named explicitly, per `SPRINT11.md`'s own Deferred Work and this Sprint's Out of Scope sections, none of it begun:

- **Broader log-signature coverage.** Only the one documented failure class (`single-incompatible-mod`'s declared-`descVersion` mismatch) is recognized. A second failure class remains a future capability, requiring its own real fixture evidence before any recognition contract for it can be defined (per v1.2.0's new Section 2.4).
- **Cross-platform fixture coverage.** All three fixtures were captured on macOS; whether the recognized template's wording is stable on Windows or Linux is unnamed and unclaimed either way.
- **Any Knowledge Domain pairing.** No Repair Recipe exists for this Finding; the Recommendation is inline-authored, exactly as scoped.
- **`modded-map-only` and `real-world-mod-profile` fixtures** — named, deferred corpus-expansion candidates from Sprint 10, not acquired this Sprint either.
- **GOV-013's own resolution.** Documented as a live interaction (Section 3.3 of the architecture document), not decided — this Sprint's severity assignment is offered as evidence for a future GOV-013 review, not an attempt to close it.

---

## 12. Known Limitations

- **One sample, one failure class.** The recognition contract is real and tested, but generalizes no further than this single documented signature — named explicitly, not silently assumed to cover more.
- **`FindingSeverity::Error` is now exercised for the first time in this platform's history**, by exactly one Rule. Whether the model this severity sits within (`GOVERNANCE.md` GOV-013) should itself change is a separate, still-open question this Sprint deliberately does not decide.
- **The `modiq-versioning` Crate Boundary Rules gap**, named during Sprint 8 planning, remains open, unaffected by Sprint 11.
- Every Known Limitation named in Engineering Release 1.0 (platform coverage, signature generality, corpus documentation maturity) persists unchanged — Sprint 11 activated the corpus's use, it did not extend the corpus itself.

---

## 13. Technical Director Assessment

**Sprint sizing:** Appropriately scoped — four small, independently reviewable milestones, each gated by its own `cargo fmt`/`check`/`test` pass before the next began, plus one reconciliation pass. No milestone bundled unrelated work.

**Architectural alignment:** High, with one real, disclosed exception. Every implementation decision traces directly to a cited fixture or an established repository precedent (`XmlCollector`, `VersionCompatibilityRule`, GOV-012, the Collector Composition Architecture, ADR-0007). The one exception — the Architectural Invariants section's initial wording — was caught by the same adversarial verification discipline this Sprint's own instructions required, not glossed over, and corrected at the documentation layer without touching working code.

**Risk:** Low, now that the reconciliation is closed. The capability's entire footprint is additive: two new files, four call sites, zero signature changes, zero new dependencies. The residual risks are the same ones the architecture document itself already named (single-sample design, platform coverage) — not new risk this Sprint introduced.

**Process observation worth naming going forward:** this is the first Sprint in this project's history where a dedicated, adversarial *engineering* verification pass (as distinct from architectural or repository *review*, which caught issues at Sprints 8 and 9) found a genuine inconsistency by checking implementation against documentation rather than the reverse. This is a different, and arguably more rigorous, class of check than this project has recorded before — see `ENGINEERING_LOG.md`'s Engineering Methodology Observations for this Sprint.

**Expected repository impact:** Two new source files, four modified source files, one new and now-reconciled architecture document, one new Engineering Release. Nine workspace crates, unchanged in count; Governance Register unchanged at 13 items.

---

## Repository Status

**Sprint 11 is complete.** `RuntimeLogCollector` and `RuntimeLoadFailureRule` are both real, tested, wired end to end, and verified against a reconciled, internally consistent architecture document. `cargo fmt --check`, `cargo check --workspace`, and `cargo test --workspace` are all clean; Sandbox is reverified unchanged. No Governance Register item, no ADR, and no Documentation Release amendment resulted from this Sprint. The repository is ready for the next engineering objective, not yet scoped by this document.

---

## Repository Timeline

```
Documentation Release 1.0
        ↓
Sprint 0 → Sprint 9, Engineering Releases v0.1.0-alpha, 0.2–0.9
        ↓
Sprint 10 — Runtime Fixture Corpus Acquisition (Capability Definition + evidentiary foundation only)
        ↓
Engineering Release 1.0
        ↓
Sprint 11 — Runtime Evidence Processing Architecture (Architectural Resolution)
        ↓
Sprint 11 — Implementation (RuntimeLogCollector, RuntimeLoadFailureRule, dispatch integration)
        ↓
Sprint 11 — Architectural Reconciliation (v1.1.0 → v1.2.0)
        ↓
Sprint 11 Closeout
        ↓
Engineering Release 1.1
```

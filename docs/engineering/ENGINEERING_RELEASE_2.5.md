# Engineering Release 2.5

| Property | Value |
|---|---|
| **Release** | 2.5 |
| **Documentation Release** | 2.1 (Frozen — unchanged this capability; no specification amendment) |
| **Milestone** | C15 complete (Assessment & Report Subject Identity) — `AssessmentSubject` now carries meaningful, durable submission-time-known content (a validated, non-empty `label`), reflected through `AssessmentReport::subject_label()`, established using the existing Assessment-owned/Reporting-presents architectural pattern — no Storage, transport, or UI change |
| **Scope** | The work `IMPLEMENTATION_AUTHORIZATION_C15_ASSESSMENT_REPORT_IDENTITY.md` authorizes: a fallible-constructed `label` field on `AssessmentSubject`, a reflecting `subject_label()` getter on `AssessmentReport`, and migration of every construction call site to the new constructor — no more |
| **Predecessor** | `docs/engineering/ENGINEERING_RELEASE_2.4.md` (C14) |
| **Governing ADRs** | None new — `ADR-0003`, `ADR-0007`, `ADR-0011` applied, not amended |
| **Governing Documents** | `docs/engineering/CAPABILITY_DEFINITION_C15_ASSESSMENT_REPORT_IDENTITY.md`, `docs/engineering/PROCESS_DETERMINATION_C15_ASSESSMENT_REPORT_IDENTITY.md`, `docs/engineering/ARCHITECTURE_EVALUATION_C15_ASSESSMENT_REPORT_IDENTITY.md`, `docs/engineering/ARCHITECTURAL_RESOLUTION_C15_ASSESSMENT_REPORT_IDENTITY.md`, `docs/engineering/IMPLEMENTATION_AUTHORIZATION_C15_ASSESSMENT_REPORT_IDENTITY.md`, `docs/engineering/IMPLEMENTATION_PLAN_C15_ASSESSMENT_REPORT_IDENTITY.md`, `docs/engineering/IMPLEMENTATION_REPORT_C15_ASSESSMENT_REPORT_IDENTITY.md` |

---

## 1. Executive Summary

C15 closes the gap the Post-C14 Product & Capability Assessment identified and the subsequent governance chain resolved through this repository's own full lifecycle: `AssessmentSubject` had carried zero fields since the platform's foundation, and C14's own real Hemp/Hashy acceptance sequence was the first concrete, evidenced instance where a human needed to durably distinguish two real Assessments within one continuous session, with nothing but a transient, unpersisted presentation-layer path string to do it with.

This is the first C-numbered capability in this repository's history to traverse the full seven-stage governed lifecycle — Capability Definition, Process Determination, Architecture Evaluation, Architectural Resolution, Implementation Authorization, Implementation Plan, Implementation — before this release. The Architecture Evaluation found, on direct primary-source inspection, that two of three concerns it was charged with investigating (domain ownership, cross-process durability) were already settled by existing architecture, not genuinely open; the Architectural Resolution then adopted, with reasoning grounded in `ADR-0003`, `ADR-0007`, `ADR-0011`, and this repository's own established Storage Mirror precedent, that submission-time-known content is architecturally sufficient — no Evidence-derived identity mechanism is justified by any evidence currently in this repository.

`AssessmentSubject` now carries one validated, non-empty `label: String`, populated at construction from information every real caller already possesses (`modiq-cli`'s user-supplied path, `apps/console`'s dialog-selected path, `apps/sandbox`'s own fixture constant). `AssessmentReport::subject_label()` reflects it directly, computing or inferring nothing. No Storage schema, transport, DTO, or UI file was touched.

---

## 2. Authorization and Planning Summary

C15 followed the full lifecycle its own Capability Definition and Process Determination established as necessary — the first capability in this repository to require, and complete, all of Architecture Evaluation and Architectural Resolution before Implementation Authorization, rather than the shorter C1/C3/C4 path.

The Process Determination independently re-derived, rather than assumed, that Architecture Evaluation was required — finding two genuine open questions (domain ownership between `Assessment` and `AssessmentReport`; cross-process durability against `AssessmentId`'s documented non-durability) that survived its own two-prong test. The Architecture Evaluation then found, on deeper primary-source inspection, that both of those specific questions were **already resolved by existing architecture** — `ADR-0011`'s own text explicitly reaffirms `AssessmentSubject` as Assessment-owned at the exact moment it corrects `AssessmentReport` out of the same list, and the already-three-times-proven Storage Mirror pattern already solves durability for descriptive, value-copied content, independent of `AssessmentId`'s own differently-scoped non-durability. What survived was a single, narrower question: whether identity content must be limited to what's known at Assessment-construction time, or whether Evidence-derived content requires a new mechanism. The Architectural Resolution answered it: submission-time-known content is sufficient, evidenced by both the Capability Definition's own stated Acceptance Criteria and the fact that C14's real forcing instance involved two mods already fully distinguishable by name alone. The Implementation Authorization and Plan then fixed the exact, minimal design — one fallible-constructed field, mirroring `Evidence::new`'s own established validation pattern — and bounded the mechanical, compiler-enumerated call-site migration that followed.

---

## 3. Implementation Summary

One continuous implementation pass, compiler-guided rather than hand-enumerated, per the Plan's own stated strategy: the core type change was made first, and every resulting compile error — in both the root workspace and `apps/sandbox`'s own separate workspace — was fixed to green before verification began.

1. **Core type change.** `AssessmentSubject` (`crates/modiq-runtime/src/assessment/subject.rs`) gained `label: String` and a fallible `new(label: impl Into<String>) -> Result<Self, AssessmentSubjectError>`, rejecting empty/whitespace-only labels — the identical validation shape `Evidence::new` already established. A new file, `subject_error.rs`, defines `AssessmentSubjectError` (one variant, `EmptyLabel`), mirroring `EvidenceError`'s own shape exactly. `AssessmentReport` (`crates/modiq-report/src/report/assessment_report.rs`) gained `subject_label()`, populated in `generate` as a direct `.to_string()` copy of `assessment.subject().label()`.
2. **Call-site migration.** Three real production call sites (`modiq-cli assess`, `apps/console`'s Tauri command, `apps/sandbox`'s fixture command) now construct `AssessmentSubject` from the same input string each already possessed, handling the new fallible result explicitly. Every other occurrence — confirmed compiler-enumerated across `modiq-runtime`, `modiq-report`, `modiq-engine` (including `tests/end_to_end.rs`), `modiq-cli`, `modiq-storage`, and `apps/sandbox`'s own test module — is test/fixture-only, migrated to the same constructor using either a uniform placeholder literal or, where a test already had its own natural input constant, that same constant.
3. **New tests.** Three in `subject.rs` (accepts a real label; rejects empty; rejects whitespace-only), one in `assessment_report.rs` (`subject_label()` reflects exactly what the Assessment was constructed with).

No phase boundary was required beyond this single pass — the Authorization itself found the change uniform in kind across every participating file.

---

## 4. Repository Impact

| Area | Change |
|---|---|
| `crates/modiq-runtime/src/assessment/subject.rs` | +56/−1 |
| `crates/modiq-runtime/src/assessment/subject_error.rs` | +8 (new file) |
| `crates/modiq-runtime/src/assessment/mod.rs` | +2/−0 |
| `crates/modiq-runtime/src/assessment/assessment.rs` | +57/−54 (test call sites only) |
| `crates/modiq-report/src/report/assessment_report.rs` | +28/−3 |
| `crates/modiq-engine/src/engine/assessment_service.rs` | +48/−26 (test call sites only — public signatures unchanged) |
| `crates/modiq-engine/tests/end_to_end.rs` | +20/−4 |
| `crates/modiq-cli/src/commands/assess.rs` | +10/−6 (one production call site + tests) |
| `crates/modiq-cli/src/commands/retrieve.rs` | +5/−5 (test call sites only) |
| `crates/modiq-cli/src/commands/history.rs` | +1/−1 (test call site only) |
| `apps/console/src-tauri/src/assessment.rs` | +2/−1 (one production call site) |
| `apps/sandbox/src-tauri/src/lib.rs` | +6/−8 (one production call site + tests) |
| `crates/modiq-storage/src/storage/history_analysis.rs` | +2/−2 (test call sites only) |
| `crates/modiq-storage/src/storage/persisted_report.rs` | +3/−3 (test call sites only) |
| `crates/modiq-storage/src/storage/report_store.rs` | +1/−1 (test call site only) |
| `modiq-storage`'s own schema (`PersistedAssessmentReport`, `ReportKey`, `ReportStore`'s public API) | **Unmodified** — confirmed no struct, field, or method signature line appears anywhere in this capability's diff |
| `AssessmentService`'s public method signatures | **Unmodified** |
| `apps/console/src/`, `apps/console/src-tauri/capabilities/` (DTO, transport, UI, Tauri capabilities) | **Unmodified** — confirmed via direct `git status` scoping |
| `apps/sandbox`'s own DTOs (`AssessmentSummary`, `PersistedReportSummary`) | **Unmodified** — `subject_label` is not surfaced through either |
| Every ADR, `GOVERNANCE.md` | **Unmodified** |
| Governance Register | Unaffected — no new item |
| New documents | `CAPABILITY_DEFINITION_C15...md`, `PROCESS_DETERMINATION_C15...md`, `ARCHITECTURE_EVALUATION_C15...md`, `ARCHITECTURAL_RESOLUTION_C15...md`, `IMPLEMENTATION_AUTHORIZATION_C15...md`, `IMPLEMENTATION_PLAN_C15...md`, `IMPLEMENTATION_REPORT_C15...md`, this release document |

Total source impact: 15 files changed (14 modified + 1 new), 241 insertions, 115 deletions across the two prior-tracked-file diffs combined with the new file (re-confirmed fresh at this release's own drafting).

---

## 5. Validation Status

```
Root workspace:
cargo fmt --check                    → clean
cargo check --workspace --all-targets → clean, all 8 crates/targets
cargo test --workspace               → clean, 0 failures, 311 total passing
  console_lib 7 · modiq_cli 27 · modiq_collection 70 · modiq_engine 23(+3 e2e)
  modiq_knowledge 5 · modiq_report 4 · modiq_rules 46 · modiq_runtime 93
  modiq_storage 29 · modiq_versioning 4

apps/sandbox (separate workspace):
cargo fmt --check                    → clean
cargo check --all-targets            → clean
cargo test                           → 16/16, unaffected

apps/console:
tsc && vite build                    → clean, 147 modules

git diff --check                     → clean

Real-I/O verification (real material, no mock data):
  Real .zip mod (FS25_HH_HempProdSystem.zip) → real report, 128 Evidence items,
    identical to pre-C15 behavior
  Empty input → rejected: "error: assessment subject label must not be empty",
    exit code 2 (InvalidUsage), one layer earlier than before, same message substring
    the pre-existing test already asserted

Test-count reconciliation: verified against IMPLEMENTATION_REPORT_C14...md's own
  recorded baseline (307), not the unverified 297 figure a prior mission turn
  stated. Actual, traceable delta: 307 → 311 (+4 — the four new tests, §3 above).
```

All figures re-verified fresh at this release's own drafting, independent of the Implementation Report's own figures, and found identical to them.

---

## 6. Engineering Observations

Recorded as evidence only — no process change, new governance mechanism, or Governance Register item follows from any observation below.

**Observation 1 — the first capability in this repository's history to traverse the full seven-stage lifecycle before release.** C1, C3, C4 skipped Architecture Evaluation and Architectural Resolution entirely (Process Determination found no genuine open question); C2 and C12 required Architecture Evaluation but reached it directly from Capability Definition-level analysis. C15 is the first to require, and complete, Process Determination *and* Architecture Evaluation *and* Architectural Resolution, each as its own distinct, evidence-gated stage, before any implementation code existed.

**Observation 2 — the Architecture Evaluation narrowed its own Process Determination's framing, not merely confirmed it.** Direct primary-source inspection at the Architecture Evaluation stage found two of the Process Determination's three named open questions (domain ownership, cross-process durability) were already settled by existing architecture — `ADR-0011`'s own reaffirmation of `AssessmentSubject` as Assessment-owned, and the already-proven Mirror pattern's independence from `AssessmentId`'s own non-durability. Only a single, more precisely bounded question (submission-time vs. Evidence-derived content) survived to the Architectural Resolution stage. This is recorded because it is a genuine instance of a later governed stage *sharpening*, not merely rubber-stamping, an earlier one's conclusion — consistent with this repository's own standing adversarial-verification discipline.

**Observation 3 — the mechanical call-site migration was compiler-guided, not hand-enumerated, and this found one occurrence a manual grep pass had missed.** Because giving a previously-zero-field unit struct a real field breaks every bare-literal construction site at compile time, implementation deliberately made the core type change first and let `cargo check --workspace --all-targets` (plus `apps/sandbox`'s own separate workspace) enumerate every remaining site, rather than trusting a hand-authored inventory. This caught three `modiq-storage` test files (`history_analysis.rs`, `persisted_report.rs`, `report_store.rs`) that an earlier manual `grep`-based scoping pass, during Implementation Planning, had not individually counted — confirming the compiler-driven approach's own value as a completeness check, not merely a convenience.

**Observation 4 — a test-count discrepancy in an intermediate mission turn's own framing was found and corrected, not silently absorbed.** A later mission turn stated the suite "increased from 297 to 301." Direct verification against `IMPLEMENTATION_REPORT_C14...md`'s own recorded baseline found the actual figure to be 307 → 311. The Implementation Report and this release document both record the verified figure rather than the unverified one — named here as the same discipline this repository's own Engineering Releases have applied to similar discrepancies before (e.g., C4's Observation 4, a self-referential count correction in its own Implementation Report).

---

## 7. Outstanding Limitations / Reserved Responsibilities

Named, not resolved, exactly as `CAPABILITY_DEFINITION_C15...md`'s and `ARCHITECTURAL_RESOLUTION_C15...md`'s own Non-Scope/Explicit Non-Decisions already name them — none is created or newly recommended by this release:

- **Evidence-derived subject identity** — Deferred (`ARCHITECTURAL_RESOLUTION_C15...md` §4), not designed, not scheduled. Available for reconsideration only if future real usage demonstrates submission-time content is genuinely insufficient — no such evidence exists today.
- **Post-construction subject enrichment** — no mechanism exists; none was authorized.
- **Canonical mod identity / semantic matching** — not implemented; `label` is a durable, meaningful reference, not a canonical or matching-grade identifier.
- **Console assessment history, same-mod trends, C5 (cross-mod comparison)** — all remain downstream consumers, explicitly named, explicitly unauthorized, by `CAPABILITY_DEFINITION_C15...md` §10.
- **`ReportKey` redesign, `modiq-storage` schema changes persisting `subject_label`** — the Mirror pattern remains available and unexercised; persisting `subject_label` was never part of this Resolution's own authorized content.
- **Any DTO, transport, or UI surfacing `subject_label`** — `apps/console`, `apps/sandbox`'s own DTOs are unmodified; a user cannot yet see this field anywhere in either consumer's own presentation.
- `Confidence`, `Evidence Explanation`, Savegame/User Observation evidence, Investigate/Diagnose/Remediate workflow, Knowledge-layer activation — untouched, unrelated.

---

## 8. Technical Director Assessment

**C15 is the clearest instance yet of this repository's own governed lifecycle doing genuine work, not merely producing ceremony.** The Process Determination correctly identified that Architecture Evaluation was required; the Architecture Evaluation then found, through direct primary-source inspection rather than accepting that determination's own framing at face value, that most of the apparent complexity was already resolved by architecture already in force — narrowing a three-part question to one precise, well-evidenced question the Architectural Resolution could actually answer with confidence. This is the opposite failure mode from rubber-stamping: each stage independently re-verified rather than propagated the prior stage's own claims, and the narrowing this produced is directly traceable to specific ADR text (`ADR-0003`, `ADR-0011`) and a specific implementation fact (`AssessmentReport::generate`'s own pre-existing getter-reading pattern), not to interpretive convenience.

**Implementation discipline matched the design's own precision.** The compiler-guided call-site migration (Observation 3) is a stronger completeness guarantee than a hand-authored file list would have been, and it demonstrably found something a manual pass missed. Every verification gate was re-run fresh at each subsequent lifecycle stage (Implementation Report, this release) rather than trusted from the prior stage's own account, and one real discrepancy (Observation 4) was found and corrected as a result, not smoothed over.

**Risk: low.** No architectural document was reopened — `ADR-0003`, `ADR-0007`, `ADR-0011`, `GOVERNANCE.md`'s Reporting and Storage rules, and GOV-002 are all confirmed unmodified and, on direct re-reading, already fully consistent with what shipped. No `modiq-storage` schema, transport, or UI boundary was crossed. The scope discipline held under real pressure: a wide, compiler-driven blast radius (14 files) touched no file outside what the Authorization named, and every touched file's own diff was individually classified (authorized implementation, required compatibility, or — found in zero cases — unauthorized).

---

## 9. Final Release Recommendation

**C15 is complete.** The full seven-stage governed lifecycle concluded with an implementation that matches its own Authorization and Plan exactly; every verification gate passes, independently re-run at this release's own drafting; no unauthorized change exists anywhere in the diff; `GOV-002`, Initiative 2, and Initiative 3 Item 6a are each confirmed in their correct, precise post-C15 state (§7 of `IMPLEMENTATION_REPORT_C15...md`) rather than asserted.

**Recommend:** Product Owner final approval of this release.

---

## Repository Status

C15 is complete. The root workspace is clean at 311/311; `apps/sandbox` is clean at 16/16, unaffected; `apps/console`'s build is clean at 147 modules. No Governance Register item or ADR resulted from this capability; GOV-002 remains Resolved, textually unmodified; Initiative 2 remains untouched; Initiative 3 Item 6a is satisfied for submission-time-known content specifically, still gated for Evidence-derived content. As of this document's own drafting, HEAD remains `60ef3af` on `feature/runtime-implementation`, and the entire C15 lifecycle — Capability Definition, Process Determination, Architecture Evaluation, Architectural Resolution, Implementation Authorization, Implementation Plan, Implementation, Implementation Report, and this release document — exists only as uncommitted working-tree state, not yet made as of this document's own drafting. `PROJECT_STATUS.md`, `CHANGELOG.md`, and `ENGINEERING_LOG.md` are **intentionally not synchronized by this release** — confirmed, by direct inspection, to still accurately reflect the C14 closeout state, with zero reference to C15 anywhere in any of the three, consistent with this repository's own established precedent (C14's own Engineering Release 2.4 made the identical choice, deferring those three files to a separate Repository Closeout commit) — that same separate step remains this release's own explicit exclusion, not performed here.

---

## Repository Timeline

```
Engineering Release 2.4 — C14 complete
        ↓
Post-C14 Product & Capability Assessment — identified Assessment/Report Identity
as a demonstrated, evidenced forcing function (C14's own Hemp/Hashy sequence)
        ↓
Capability Definition C15 — Assessment & Report Subject Identity
        ↓
Process Determination C15 — Determination B, Architecture Evaluation required
        ↓
Architecture Evaluation C15 — narrowed the open question to one: submission-time
vs. Evidence-derived content; found domain ownership and cross-process durability
already settled by existing architecture
        ↓
Architectural Resolution C15 — Adopted: submission-time-known content is
sufficient; no Evidence-derived mechanism justified by current evidence
        ↓
C15 Implementation Authorization
        ↓
C15 Implementation Plan — compiler-guided call-site migration strategy
        ↓
C15 Implementation — AssessmentSubject.label, AssessmentSubjectError,
AssessmentReport.subject_label(), all call sites migrated
        ↓
C15 Implementation Report — READY FOR ENGINEERING RELEASE, one test-count
discrepancy found and corrected (307→311, not 297→301)
        ↓
Engineering Release 2.5 — C15 complete — this document
        ↓
[pending] C15 Engineering Release commit — implementation + seven lifecycle
documents + this release document, not yet made; PROJECT_STATUS.md/
CHANGELOG.md/ENGINEERING_LOG.md intentionally deferred to a separate,
subsequent Repository Closeout, per established C14 precedent
```

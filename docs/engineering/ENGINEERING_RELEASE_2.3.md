# Engineering Release 2.3

| Property | Value |
|---|---|
| **Release** | 2.3 |
| **Documentation Release** | 2.1 (Frozen — unchanged this capability; no specification amendment) |
| **Milestone** | C4 complete (`apps/sandbox` Field Parity) — `apps/sandbox`'s Tauri IPC layer presents, for the first time, a Finding's Mod Health dimension and status, an Evidence item's label/source/content provenance, and a Recommendation's per-step repair structure, on both its live (`create_assessment`) and persisted (`retrieve_report`, bounded to `repair_steps`) paths |
| **Scope** | The work `IMPLEMENTATION_AUTHORIZATION_C4_APPS_SANDBOX_FIELD_PARITY.md` §3 authorizes: live-path DTO extension (full field set) and persisted-path DTO extension (`repair_steps` only, bounded by what `modiq-storage`'s current persisted mirror carries) — no more |
| **Predecessor** | `docs/engineering/ENGINEERING_RELEASE_2.2.md` (C3) |
| **Governing ADRs** | None new |
| **Governing Documents** | `docs/engineering/CAPABILITY_DEFINITION_C4_APPS_SANDBOX_FIELD_PARITY.md`, `docs/engineering/PROCESS_DETERMINATION_C4_APPS_SANDBOX_FIELD_PARITY.md`, `docs/engineering/IMPLEMENTATION_AUTHORIZATION_C4_APPS_SANDBOX_FIELD_PARITY.md`, `docs/engineering/IMPLEMENTATION_PLAN_C4_APPS_SANDBOX_FIELD_PARITY.md`, `docs/engineering/IMPLEMENTATION_REPORT_C4_APPS_SANDBOX_FIELD_PARITY.md` |

---

## 1. Executive Summary

C4 closed the presentation gap its own Capability Definition named directly: `Finding`'s Mod Health dimension and status, `Evidence`'s label/source/content provenance, and `Recommendation`'s per-step repair structure have been public on `modiq-runtime`'s own types since Sprint 22 through C1, and both `apps/console` (since Sprint 22/23/24 and C1) and `modiq-cli` (since C3) have presented them — but `apps/sandbox`'s Tauri IPC layer never transported any of it. `create_assessment` now presents the complete authorized field set; `retrieve_report` presents exactly the one field `modiq-storage`'s own persisted mirror carries (`repair_steps`) — a narrower, deliberately asymmetric scope, not an oversight, mirroring the identical asymmetry C3 already established for `modiq-cli retrieve` for the identical reason: `PersistedFinding`/`PersistedEvidence` never mirrored the remaining fields into Storage.

This is the fourth post-foundation capability. Like C1 and C3, and unlike C2, C4 required no Architecture Evaluation or Architectural Resolution — the Process Determination's own two-prong test, reused a third time, found every one of the five open questions the Capability Definition named resolved to an already-Adopted architectural fact or an already-governed, previously-exercised boundary, and no alternative being weighed.

C4's own repository history diverges from C1/C2/C3 precedent in two respects, both recorded plainly in §6 below rather than smoothed over: the Implementation Report was committed before the implementation it documents, and the two implementation phases landed in one combined commit rather than one commit per phase. Neither reflects any defect in the implementation itself, which was independently verified — including by direct, real execution, not source inspection alone — at every stage.

---

## 2. Authorization and Planning Summary

C4 followed a four-document lineage before implementation began, matching C1's and C3's own shorter path (no Architecture Evaluation or Architectural Resolution stage): Capability Definition (`a024c8f`) → Process Determination (`416a83d`) → Implementation Authorization (`ed4eea2`) → Implementation Plan (`1e7d413`).

The Process Determination independently re-derived, rather than assumed by analogy to C3, that C4 matches C1's and C3's procedural path — investigating five named open questions (the flat-vs-nested transport shape, whether `apps/console`'s nested structure is precedent or requirement, the absent `apps/sandbox` Crate Boundary Rule, the pre-existing `App.tsx` field-name drift, and the persisted-path asymmetry) and finding each already settled by existing architecture or already-governed boundaries. The Implementation Authorization fixed the field list precisely: full live-path parity (`mod_health_dimension`, `status`, `location`, `label`, `source`, `content`, `repair_steps`), bounded persisted-path parity (`repair_steps` only), and left two questions to Implementation Planning — the exact Rust type/module design for the new per-step repair representation, and whether to exercise `App.tsx`'s optional extension. The Implementation Plan decided both: two new, separate, locally-named structs (`RecommendationStepEntry` for the live path, `PersistedRecommendationStepEntry` for the persisted path, mirroring the file's own established live/persisted naming convention), and elected **not** to extend `App.tsx`, since the Capability Definition's own Success Criteria are satisfied entirely by the command's own IPC response.

---

## 3. Implementation Summary

Two implementation phases plus a verification-only Final Reverification, matching the Plan's own phase boundary — one phase per type hierarchy (`modiq-runtime` live types, `modiq-storage`'s `Persisted*` mirror), since neither implementation depends on the other's:

1. **Phase 1 — Live-Path Field Parity.** `EvidenceEntry` gained `label`/`source`/`content` (presence-conditional); `FindingEntry` gained `mod_health_dimension`/`status` (unconditional); a new `RecommendationStepEntry { kind, instruction }` was introduced, converted from `modiq_runtime::assessment::RecommendationStep`; `RecommendationEntry` gained `repair_steps`. Five new tests.
2. **Phase 2 — Persisted-Path Field Parity.** A new `PersistedRecommendationStepEntry { kind, instruction }` was introduced, converted from `modiq_storage::storage::persisted_report::PersistedRecommendationStep`; `PersistedRecommendationEntry` gained `repair_steps` only — `PersistedFindingEntry` and `PersistedEvidenceEntry` were not touched, since `modiq-storage`'s persisted mirror carries neither Mod Health nor Evidence-provenance fields. Two new tests, both exercising a real `ReportStore::store` → `ReportStore::retrieve` round trip.
3. **Phase 3 — Final Reverification.** Zero files changed; both the `apps/sandbox/src-tauri` workspace and the root workspace reconfirmed clean; a full-span `git diff --stat` confirmed only `apps/sandbox/src-tauri/src/lib.rs` was touched across the entire capability.

Both phases, and Phase 3's reverification, were each independently re-verified by their own dedicated, adversarial Repository Validation Review and Post-Implementation Verification pass before the next phase began — confirmed directly against implementation-session review history, not inferred. No change to `AssessmentSummary`'s or `PersistedReportSummary`'s flat, parallel-list top-level shape, any Tauri command signature, `modiq-storage`'s schema, or any other `modiq-*` crate occurred at any phase.

**Commit shape, stated plainly.** Phase 1 and Phase 2 were not committed separately as implementation proceeded — both were verified through their own review cycle while remaining uncommitted working-tree state, and were committed together, as one combined commit (`e0bccbf`), consistent with C1's own precedent of combining its first two phases into a single commit (`431dac8`) rather than C2's and C3's own one-commit-per-phase practice. What has no precedent in C1, C2, or C3 is that this combined commit landed only after the Implementation Report itself had already been committed (`9ca6503`). See §6, Observation 3.

---

## 4. Repository Impact

| Area | Change |
|---|---|
| `apps/sandbox/src-tauri` (`src/lib.rs`) | +336/−1 (the one deletion being the pre-existing `use modiq_runtime::assessment::{...}` import line, replaced by a strict superset of itself); test count 9 → 16 |
| `modiq-runtime`, `modiq-rules`, `modiq-collection`, `modiq-versioning`, `modiq-report`, `modiq-engine`, `modiq-storage`, `modiq-knowledge` | **Unmodified** — confirmed via empty diff across the full capability span (`a024c8f`..`e0bccbf`) |
| `apps/console`, `modiq-cli` | **Unmodified** — confirmed via empty diff |
| `apps/sandbox/src/App.tsx` | **Unmodified** — the Implementation Plan (§6b) explicitly elected not to exercise the Authorization's optional extension |
| `apps/sandbox/src-tauri`'s own `create_assessment`/`retrieve_report` command signatures | Unaffected |
| Every `Cargo.toml` / `Cargo.lock` | **Unmodified** — zero new dependency edge |
| Governance Register | Unaffected — GOV-008 remains Open, untouched |
| ADRs | None new |
| New documents | `CAPABILITY_DEFINITION_C4_APPS_SANDBOX_FIELD_PARITY.md`, `PROCESS_DETERMINATION_C4_APPS_SANDBOX_FIELD_PARITY.md`, `IMPLEMENTATION_AUTHORIZATION_C4_APPS_SANDBOX_FIELD_PARITY.md`, `IMPLEMENTATION_PLAN_C4_APPS_SANDBOX_FIELD_PARITY.md`, `IMPLEMENTATION_REPORT_C4_APPS_SANDBOX_FIELD_PARITY.md`, this release document |

Total source impact across the full capability (`git diff --stat a024c8f e0bccbf -- crates/ apps/`): 1 file changed, 336 insertions, 1 deletion. Total new documentation across the same span (`-- docs/`, excluding the Capability Definition itself, which is that span's own starting commit): 4 files, 712 insertions.

---

## 5. Validation Status

```
apps/sandbox/src-tauri:
cargo fmt --check       → clean
cargo check --workspace → clean
cargo test --workspace  → 16/16 passed (9 pre-existing → 16; Phase 1: +5, Phase 2: +2)

Root workspace:
cargo fmt --check       → clean
cargo check --workspace → clean
cargo test --workspace  → 297/297 passed, unaffected (apps/sandbox is not a root workspace member;
no root-workspace crate was modified by this capability)
  console_lib 7 · modiq_cli 27 · modiq_collection 70 · modiq_engine 23(+3 e2e) · modiq_knowledge 5
  modiq_report 3 · modiq_rules 46 · modiq_runtime 90 · modiq_storage 19 · modiq_versioning 4

Empirical serialization check (real execution, not source inspection alone): a throwaway scratch
crate, outside this repository, with path-dependencies on the real modiq-runtime/modiq-report/
modiq-storage crates, mirroring both DTO conversions verbatim, was compiled and run — confirming
actual camelCase JSON field names, Option<T>::None serializing as null (not omitted), an empty
repair_steps serializing as [], step ordering preserved, and the persisted path's repairSteps
surviving a genuine ReportStore::store → ReportStore::retrieve round trip against a real,
hermetic temporary directory.

Dependency-edge check (all Cargo.toml/Cargo.lock): zero matches
Governance check: GOVERNANCE.md's GOV-008 entry — Status: Open, textually unchanged
```

All figures re-verified fresh at this release's own drafting, independent of any phase report's or the Implementation Report's own figures.

---

## 6. Engineering Observations

Recorded as evidence only — no process change, new governance mechanism, or Governance Register item follows from any observation below.

**Observation 1 — a second, independently-confirmed instance of the persisted-path asymmetry C3 already established.** `retrieve_report`'s narrower field set is the direct consequence of the same `modiq-storage` gap C3's own Engineering Release (2.2) already named: `PersistedEvidence`/`PersistedFinding` never mirrored `label`/`source`/`content`/`mod_health_dimension`/`status`. C4 did not extend Storage's schema to close that gap; it scoped `retrieve_report` to what already exists, exactly as C3 scoped `modiq-cli retrieve`, naming the remainder as the same still-deferred candidate.

**Observation 2 — an import-path deviation, consumer-side only, not an architectural decision.** `PersistedRecommendationStep` is `pub` on `modiq-storage`'s own `persisted_report` submodule but is not re-exported at `modiq_storage::storage`'s own module root (unlike `PersistedRecommendation`, which is). Since `modiq-storage` was explicitly outside this capability's authorized scope, the implementation imports the type via its actual, already-public full path (`modiq_storage::storage::persisted_report::PersistedRecommendationStep`) rather than adding a re-export. This changes nothing in `modiq-storage`, adds no dependency edge, and was independently confirmed, across two separate reviews, not to depart from any Authorization or Plan boundary.

**Observation 3 — the Implementation Report was committed before the implementation it documents, reversing the Report-after-Implementation order C1, C2, and C3 each established.** In every prior capability, the implementation — whether landed as one commit per phase (C2, C3) or with phases combined (C1's own Phase 1–2 commit, `431dac8`) — was committed first, and the Implementation Report, citing those already-existing commit hashes, was committed last, immediately before the Engineering Release. C4's own Implementation Report (`9ca6503`) was instead committed while Phase 1 and Phase 2 still existed only as verified, uncommitted working-tree state; the implementation itself (`e0bccbf`) was committed afterward, at separate, explicit direction, as one combined commit — a commit shape consistent with C1's own precedent (§3), but committed after, not before, the Report. The actual parent-child chain, confirmed directly via `git log --format="%H parent=%P"` rather than assumed: `1e7d413` → `9ca6503` → `e0bccbf`. This is recorded here as a repository-history fact this release does not normalize into the usual shape; it reflects the actual sequence in which these commits were authorized and made, not an error in either commit. It carries no correctness consequence: the Implementation Report's own content was independently re-verified against the implementation twice after the implementation commit existed (a Post-Reconciliation Verification and a Corrected Commit Readiness Verification), and both found the Report's description of the implementation accurate.

**Observation 4 — one non-blocking historical-accuracy defect was found in the Implementation Report and corrected before this release.** A Repository Validation Review of the Report found it overcounted its own prior verification passes ("three independent Repository Validation Review passes" where only two had occurred before the Report was drafted). The correction ("two") was independently re-derived from the actual sequence of completed passes, applied as a single-word edit, and re-verified by a dedicated Post-Reconciliation Verification that found no collateral change and no other defect.

---

## 7. Outstanding Limitations / Reserved Responsibilities

Named, not resolved, exactly as `CAPABILITY_DEFINITION_C4_APPS_SANDBOX_FIELD_PARITY.md`'s and `IMPLEMENTATION_AUTHORIZATION_C4_APPS_SANDBOX_FIELD_PARITY.md`'s own Explicit Exclusions already name them — none is created or newly recommended by this release:

- **Extending `modiq-storage`'s own `PersistedFinding`/`PersistedEvidence` schema** to carry `mod_health_dimension`, `status`, or `label`/`source`/`content` — the same real, deferred gap C3 already named for `modiq-cli retrieve`, now confirmed identically for `apps/sandbox retrieve_report`. Not resolved, not advanced, by this capability.
- **GOV-008 (`AssessmentService` public API evolution)** — untouched, confirmed `Status: Open` directly against `GOVERNANCE.md`. The flat-vs-nested-by-Finding transport-shape question `FrontendArchitecture.md` reserves pending GOV-008 remains exactly as reserved; `repair_steps` nested inside a `RecommendationEntry`/`PersistedRecommendationEntry` is a field-level addition within `AssessmentSummary`'s/`PersistedReportSummary`'s existing flat shape, not a resolution of that question.
- **`apps/sandbox/src/App.tsx`'s optional extension** — the Authorization made this strictly optional; the Implementation Plan (§6b) elected not to exercise it. `App.tsx`'s pre-existing `FindingEntry.description`-vs-`title`/`summary` drift, named in the Capability Definition, remains uncorrected.
- **`apps/sandbox`'s own absent `GOVERNANCE.md` Crate Boundary Rule section** — the Process Determination's own Question 3 finding concluded this is an already-Adopted architectural fact (Initiative 5, Decision 6's consequence of `apps/sandbox`'s deliberate non-membership in the root workspace), not a gap requiring closure. Not amended by this release.
- `Confidence`, cross-mod dependency resolution, Lua Analysis, and every other remaining Capability Portfolio Assessment candidate — untouched.

---

## 8. Technical Director Assessment

**C4 is the plainest instance yet of this repository's own "mechanical extension of an already-public getter" pattern, now verified by real execution rather than source inspection alone.** Every field it presents already existed, already public, already correct, on a type its own DTO already converted from; nothing was inferred, computed, or synthesized at the presentation layer. Beyond the standard source-level review this repository already applies, C4's own verification included a throwaway, outside-the-repository scratch harness that compiled and ran real code against the real domain and Storage types, producing actual serialized JSON for both the live and persisted paths — the strongest available confirmation that the authorized behavior is real, not merely well-argued.

**Verification rigor matched or exceeded C1's, C2's, and C3's own precedent, and was applied an additional time to correct the Implementation Report itself.** Every governing document and implementation phase underwent independent, adversarial review; the Implementation Report underwent a full Review → Reconciliation → Post-Reconciliation Verification cycle, and the Implementation Report's own commit underwent two separate Commit Readiness Verification passes — the first identifying that `git add -n .` would improperly bundle the still-uncommitted implementation and declining to declare readiness, the second confirming a correctly scoped `git add` and declaring the Report ready; the implementation commit (`e0bccbf`) that followed was performed with its own scope verification embedded directly in that commit's own execution turn, not as a separately titled Commit Readiness Verification pass. No engineering, architectural, repository-consistency, or governance-consistency defect survived any pass across the entire capability. The one finding that did survive (Observation 4) was confined to a self-referential count in the historical-record document itself, never to the implementation.

**Risk: low.** No architectural document was reopened. No Runtime, Storage, or other crate boundary was crossed. The Report-before-Implementation commit ordering (Observation 3) is a genuine deviation from prior practice, but carries no correctness risk of its own — it was independently identified, not discovered by accident, and every claim in the Report was re-verified against the implementation after the fact and found accurate.

---

## 9. Final Release Recommendation

**C4 is complete.** Both phases implemented, verified — including by real execution — and independently reviewed at every stage; every Authorization exclusion remains absent (no `modiq-storage` schema change, no `App.tsx` change, no GOV-008 resolution, no transport-shape migration, no other crate touched); the persisted-path scope asymmetry was directly confirmed against source, not assumed; the one historical-accuracy defect found in the Implementation Report was corrected and re-verified.

**Recommend:** Chief Architect final approval of this release.

---

## Repository Status

C4 is complete. `apps/sandbox/src-tauri` is clean at 16/16; the root workspace is clean at 297/297, unaffected (`apps/sandbox` is not a root workspace member). No Governance Register item or ADR resulted from this capability; `GOV-008` remains Open. HEAD is `e0bccbf` on `feature/runtime-implementation`, 33 commits ahead of `origin/feature/runtime-implementation` and not yet pushed as of this document's own drafting. The complete C4 lifecycle — Capability Definition (`a024c8f`), Process Determination (`416a83d`), Implementation Authorization (`ed4eea2`), Implementation Plan (`1e7d413`), Implementation Report (`9ca6503`), and Implementation (`e0bccbf`) — is committed, in that actual chronological order (see §6, Observation 3). The working tree is otherwise clean: nothing staged, no untracked files. This release document itself — as of this document's own drafting — **remains uncommitted**, the one pending addition to the working tree, matching every prior release's own closing convention. The repository is ready for its next engineering objective, not yet scoped by this document.

---

## Repository Timeline

Presented in true chronological (parent-child) order, confirmed directly via `git log`, not normalized to the shape prior releases established:

```
Engineering Release 2.2 — C3 complete
        ↓
Capability Definition C4 (a024c8f)
        ↓
Process Determination C4 (416a83d) — matches C1's/C3's path; no Architecture
Evaluation or Architectural Resolution required
        ↓
C4 Implementation Authorization (ed4eea2)
        ↓
C4 Implementation Plan (1e7d413)
        ↓
C4 Phase 1 — Live-Path Field Parity (verified via Repository Validation Review +
Post-Implementation Verification; uncommitted at time of verification)
        ↓
C4 Phase 2 — Persisted-Path Field Parity (verified via Repository Validation Review +
Post-Implementation Verification; uncommitted at time of verification)
        ↓
C4 Phase 3 — Final Reverification (verification-only, no commit)
        ↓
C4 Implementation Report (9ca6503) — committed while Phase 1 + Phase 2 source
remained uncommitted working-tree state, per explicit direction
        ↓
C4 Implementation — Phase 1 + Phase 2 combined (e0bccbf) — a commit shape
consistent with C1's own Phase 1–2 commit (431dac8), unlike C2's/C3's own
one-commit-per-phase practice; what is unprecedented is that it landed after,
not before, the Implementation Report (see Observation 3)
        ↓
Engineering Release 2.3 — C4 complete — this document; not yet committed
```

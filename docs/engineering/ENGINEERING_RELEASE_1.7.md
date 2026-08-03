# Engineering Release 1.7

| Property | Value |
|---|---|
| **Release** | 1.7 |
| **Documentation Release** | 2.1 (Frozen — unchanged this Sprint; no specification amendment) |
| **Milestone** | Sprint 22 complete (Initiative 3: Domain Model Anatomy Extension) — `Finding`, `Evidence`, and `RepairRecipe` gain real field anatomy for the first time since their Sprint 2 content pass |
| **Scope** | Title/Summary decomposition, Mod Health dimension, optional Recommendation cardinality, Evidence provenance fields (Label/Source/Content), structured Repair Recipe steps, and Provisional/Final Finding status — the six items `INITIATIVE_3_IMPLEMENTATION_AUTHORIZATION.md` authorizes, no more |
| **Predecessor** | `docs/engineering/ENGINEERING_RELEASE_1.6.md` (Sprint 21) |
| **Governing ADRs** | None new |
| **Governing Documents** | `docs/engineering/INITIATIVE_3_ARCHITECTURE_EVALUATION.md`, `docs/engineering/INITIATIVE_3_ARCHITECTURAL_RESOLUTION.md`, `docs/engineering/INITIATIVE_3_IMPLEMENTATION_AUTHORIZATION.md`, `docs/engineering/SPRINT22_IMPLEMENTATION_PLAN.md`, `docs/engineering/SPRINT22_IMPLEMENTATION_REPORT.md` |

---

## 1. Executive Summary

Sprint 22 implemented the six Initiative 3 items `INITIATIVE_3_IMPLEMENTATION_AUTHORIZATION.md` authorized — Title/Summary decomposition (Item 1), a closed-set Mod Health dimension (Item 2), optional Recommendation cardinality (Item 3), Evidence's Label/Source/Content provenance fields (Item 4), structured Repair Recipe steps (Item 5), and a Provisional/Final Finding status (Item 10) — entirely within the envelope the Architectural Resolution and Authorization defined. `Finding::new`'s constructor changed exactly once, carrying Items 1, 2, and 10 together, per the Authorization's own sequencing observation; the resulting breaking change propagated through nine crates and both consumer applications, all verified independently.

The Sprint's most consequential engineering decision — how the newly-optional `Recommendation` should be handled at the one production orchestration site that constructs it — was resolved exactly as the Implementation Plan's own risk ranking anticipated: absence is represented by omission, never a panic or a synthesized placeholder. Full detail in `SPRINT22_IMPLEMENTATION_REPORT.md`.

---

## 2. Authorization and Planning Summary

Sprint 22 followed this repository's now-standard Engineering Alignment Program lifecycle: Architecture Evaluation (`INITIATIVE_3_ARCHITECTURE_EVALUATION.md`) → Architectural Resolution (`INITIATIVE_3_ARCHITECTURAL_RESOLUTION.md`) → Implementation Authorization (`INITIATIVE_3_IMPLEMENTATION_AUTHORIZATION.md`) → Sprint Plan (`SPRINT22_IMPLEMENTATION_PLAN.md`). All four are treated as fixed inputs here and are not restated.

The Authorization scoped implementation to exactly six items (1, 2, 3, 4 Label/Source/Content only, 5, 10), explicitly excluding Item 6a (blocked on `AssessmentSubject` content), Item 4's Explanation field (Requires Additional Investigation), 3C (deferred pending Initiative 2), Item 7 (GOV-013, a standing Governance Register item, not an Initiative 3 item), and Item 8 (a Product Owner documentation action). The Sprint Plan's own four-phase breakdown (Foundational domain types → Direct dependents → Indirect and storage consumers → Application consumers) sequenced implementation to touch `Finding`'s constructor exactly once rather than once per item, and was followed without deviation.

---

## 3. Implementation Summary

Implemented as one commit (`969e595`), consistent with this Sprint's own scope being a single coherent domain-model extension rather than a multi-commit rollout:

1. **Foundational domain types** — `modiq-runtime` gained `FindingStatus` and `ModHealthDimension` (new files), `Finding`'s constructor changed to accept `title`/`summary`/`mod_health_dimension`/`status` in place of `description`, and `Evidence`'s two constructors gained `label`/`source`/`content`. `modiq-knowledge`'s `RepairRecipe` gained `RepairStep`/`RepairStepKind`, replacing its flat `guidance: String`.
2. **Direct dependents** — all four `modiq-rules` Rules updated their `Finding`/`RuleOutcome` construction, each assigned a reviewed `ModHealthDimension` (table below); all four `modiq-collection` Collectors updated their `Evidence` construction, each with its own Label/Source/Content population decision.
3. **Indirect and storage consumers** — `modiq-report`'s and `modiq-engine`'s test fixtures updated; `modiq-engine`'s production orchestration loop (`assessment_service.rs`) made `add_recommendation` conditional on `Some(...)`; `modiq-storage`'s `PersistedFinding` gained `title`/`summary`, mirroring Runtime's own shape; `modiq-cli`'s `assess`/`retrieve`/`history` commands updated.
4. **Application consumers** — `apps/console`'s `FindingSummary` DTO and TypeScript types updated; `apps/sandbox`'s `FindingEntry`/`PersistedFindingEntry` updated. Both applications compile and their own tests pass at compilation-correctness minimum, with no new presentation behavior added.

No change to `AssessmentService`'s public entry points, at any phase.

---

## 4. Repository Impact

| Area | Change |
|---|---|
| Crates touched | `modiq-runtime`, `modiq-rules`, `modiq-knowledge`, `modiq-collection`, `modiq-engine`, `modiq-report`, `modiq-storage`, `modiq-cli` |
| `apps/console` (`console`) | `FindingSummary` DTO and TypeScript types extended; no new region or presentation behavior |
| `apps/sandbox` | `FindingEntry`/`PersistedFindingEntry` DTOs extended; no new command or UI surface |
| New public types | `ModHealthDimension`, `FindingStatus` (`modiq-runtime`); `RepairStep`, `RepairStepKind` (`modiq-knowledge`) |
| `AssessmentService` public entry points | Unaffected |
| Governance Register | Unaffected — no item opened, modified, or implicitly resolved by this Sprint |
| ADRs | None new |
| `GOVERNANCE.md` / `docs/adrs/` | Confirmed untouched by this Sprint's commit |
| New documents | `docs/engineering/SPRINT22_IMPLEMENTATION_REPORT.md`, this release document |

---

## 5. Validation Status

```
Root workspace (default-members):
cargo fmt --check       → clean
cargo check --workspace → clean
cargo test              → 264 → 269 passing

console (explicit -p console / --workspace):
cargo test -p console  → 4/4 passing, unchanged

Full workspace (--workspace, 10 members):
cargo test --workspace  → 268 → 273 passing

apps/sandbox/src-tauri (separate workspace):
cargo fmt --check → clean
cargo check       → clean
cargo test        → 9/9 passing, unchanged

apps/console (frontend):
npm run build (tsc && vite build) → clean
```

---

## 6. Engineering Observations

Recorded as evidence only — no process change, new governance mechanism, or Governance Register item follows from either observation.

**Observation 1 — every implementation-risk prediction in `INITIATIVE_3_IMPLEMENTATION_AUTHORIZATION.md` §9 was confirmed, none disproved.** Item 3's blast radius reached exactly the consumers named (`modiq-cli`, both applications), each already tolerant of an absent Recommendation at the presentation layer; Item 2's per-Rule assignment required the anticipated judgment call, resolved by a reviewed mapping table finalized before Phase 2 began; Item 4's per-Collector decision was confirmed non-uniform across all four Collectors (five distinct population patterns, not a single default).

**Observation 2 — a testing-expectation deviation, not a gap.** `SPRINT22_IMPLEMENTATION_PLAN.md` §3 anticipated `modiq-storage` would require "a genuinely new test" for `PersistedFinding`'s title/summary round-trip. Implementation instead extended the existing round-trip test in place — behavior covered, test count unchanged (3 → 3). `SPRINT22_IMPLEMENTATION_REPORT.md`'s Recommendations section suggests future Sprint Plans distinguish "extend an existing test" from "add a new test" explicitly, to avoid this appearing as unmet coverage.

---

## 7. Outstanding Limitations / Reserved Responsibilities

Named, not resolved, exactly as `INITIATIVE_3_IMPLEMENTATION_AUTHORIZATION.md` §4 already reserves them — this release changes none of their status:

- Item 6a (Report Identity field) — remains blocked on `AssessmentSubject` carrying real content.
- Item 4's Explanation field — remains "Requires Additional Investigation," unresolved by this Sprint.
- 3C (Report currency, the Updated marker) — remains deferred pending Initiative 2's own Governance Reconciliation.
- Item 7 (GOV-013, Severity/Kind conflation) — remains a standing, independently-tracked Governance Register item, untouched.
- Item 8 (Glossary's missing "Recommendation" entry) — remains a Product Owner action, outside this release's scope.
- `apps/console`'s TypeScript presentation layer reads the new fields at compilation-correctness minimum only; `ModHealthDimension`, `FindingStatus`, and Evidence's `label`/`source`/`content` are not yet distinctly surfaced in the UI.

---

## 8. Technical Director Assessment

**This Sprint's engineering discipline matched its planning discipline exactly.** Every risk the Implementation Authorization and Sprint Plan named in advance — the orchestration loop's `None`-handling decision, `PersistedFinding`'s design choice, the single wide constructor change, the per-Rule and per-Collector judgment calls — was resolved in the direction each document's own reasoning pointed toward, with no rework or rollback at any phase boundary.

**Risk: low.** The one testing-expectation deviation (Observation 2) is cosmetic — coverage exists, only its delivery mechanism (extended vs. net-new test) differed from the Plan's own anticipation.

---

## 9. Final Release Recommendation

**Sprint 22 is complete.** All six authorized Initiative 3 items implemented exactly as scoped; root workspace, both consumer applications, and the separate Sandbox workspace all verified clean; no item outside the authorized six was touched.

**Recommend:** Chief Architect final approval of this release.

---

## Repository Status

Sprint 22 is complete. Root workspace `cargo fmt --check`, `cargo check --workspace`, and `cargo test` are clean at 269/269 (264 baseline + 5 net-new `modiq-runtime` tests); `console` is clean at 4/4, unchanged; the full workspace is clean at 273/273; Sandbox remains clean at 9/9. No Governance Register item or ADR resulted from this Sprint. The repository is ready for its next engineering objective, not yet scoped by this document.

---

## Repository Timeline

```
Engineering Release 1.6 (Sprint 21)
        ↓
Initiative 3 Architecture Evaluation (committed)
        ↓
Initiative 3 Architectural Resolution (committed)
        ↓
Initiative 3 Implementation Authorization (committed)
        ↓
Sprint 22 Implementation Plan (committed)
        ↓
Sprint 22 — Domain Model Anatomy Extension (969e595)
        ↓
Engineering Release 1.7 — Sprint 22 complete
```
